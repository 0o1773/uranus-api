use reqwest::StatusCode;
use sea_orm::ColIdx;
use tonic::async_trait;
use crate::domain::repository::asset_repository::AssetRepository;
use crate::domain::structs::errors::DomainError;
use crate::domain::structs::r#match::{Match, MatchInfo, MatchPlayer};
use crate::domain::structs::round::{Round, RoundResultData};
use crate::domain::structs::stats::{CareerResults, RecentStats, ShootRate, Stats};
use crate::domain::repository::match_repository::{HistoryRepository, MatchRepository};
use crate::domain::structs::assets::{Agent, TierInfo};
use crate::domain::structs::player_info::PlayerInfo;
use crate::infrastructure::structs::{
    riot::{RiotPlayerRoundStats, RiotRoundResult},
};

use super::structs::riot::{RiotAccount, RiotMatch, RiotMatchList, RiotMatchlistEntry};

const RIOT_API_BASE_URL: &str = "https://ap.api.riotgames.com";

#[derive(Clone)]
pub struct APIMatchRepository<AR> {
    req_client: reqwest::Client,
    asset_repository: AR,
}

pub struct APIHistoryRepository<MR> {
    req_client: reqwest::Client,
    match_repository: MR
}

impl<R> APIMatchRepository<R>
where
    R: AssetRepository
{
    pub fn new(req_client: &reqwest::Client, asset_repository: R) -> Self {
        Self {
            req_client: req_client.clone(),
            asset_repository
        }
    }
    pub fn calc_rate_and_adr(&self, rounds: &Vec<RiotRoundResult>, puuid: &String) -> (ShootRate, f32) {
        let mut head = 0;
        let mut body = 0;
        let mut leg = 0;
        let mut damages = 0;
        let mut shoots = 0;
        let played_rounds = rounds.len();

        for round in rounds {
            let p = round.get_player_stats().iter().filter(|p| p.get_puuid() == puuid).collect::<Vec<&RiotPlayerRoundStats>>();

            if p.len().as_usize() == Some(&0) {
                continue;
            }

            let player = p[0];

            for dmg in player.get_damage() {
                head = head + dmg.get_headshots();
                body = body + dmg.get_bodyshots();
                leg = leg + dmg.get_legshots();
                shoots = shoots + dmg.get_headshots() + dmg.get_bodyshots() + dmg.get_legshots();
                damages = damages + dmg.get_damage();
            }
        }

        (ShootRate::new(
            head as f64 / shoots as f64,
            body as f64 / shoots as f64,
            leg as f64 / shoots as f64
        ),
         damages as f32 / played_rounds as f32
        )
    }
    pub async fn get_player_info(&self, puuid: &String) -> Result<PlayerInfo, DomainError> {
        match self.req_client.get(format!("{}/riot/account/v1/by-puuid/{}", RIOT_API_BASE_URL, puuid)).send().await {
            Ok(res) => {
                match res.json::<RiotAccount>().await {
                    Ok(account) => {
                        Ok(PlayerInfo::new(account.get_puuid().clone(), account.get_game_name().clone(), account.get_tag_line().clone()))
                    },
                    Err(_) => {
                        Err(DomainError::ResponseParseError)
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e);
                Err(DomainError::RequestFailed)
            }
        }
    }
    pub fn fetch_player_recent_stats(&self, puuid: &String) -> RecentStats {
        let stats = Stats::new(0, 0, 0, 0_f64, 0_f64, 0, 0_f32, 0_f32, ShootRate::new(0_f64, 0_f64, 0_f64));
        let career_stats = CareerResults::new(0, 0, 0,0_f64);
        RecentStats::new(stats, career_stats)

        //TODO: ここでDBから取得する
    }
}

impl<MR> APIHistoryRepository<MR>
where
    MR: MatchRepository
{
    pub fn new(req_client: &reqwest::Client, match_repository: MR) -> Self {
        Self {
            req_client: req_client.clone(),
            match_repository
        }
    }
}

#[async_trait]
impl<AR> MatchRepository for APIMatchRepository<AR>
where
    AR: AssetRepository
{
    async fn fetch_match(&self, puuid: &str, match_id: &str) -> Result<Match, DomainError> {
        let client = &self.req_client;

        let match_data = match client.get(format!("{}/val/match/v1/matches/{}", RIOT_API_BASE_URL, match_id)).send().await {
            Ok(res) => match res.json::<RiotMatch>().await {
                Ok(match_data) => match_data,
                Err(e) => {
                    println!("{match_id}");
                    println!("{:?}", e);
                    return Err(DomainError::ResponseParseError);
                }
            },
            Err(e) => {
                println!("{:?}", e);
                return Err(DomainError::RequestFailed);
            }
        };

        let mut players: Vec<MatchPlayer> = vec![];
        let mut rounds: Vec<Round> = vec![];
        let mut user_opt: Option<MatchPlayer> = None;

        

        for player in match_data.players {
            let match_player: MatchPlayer;
            let player_info: PlayerInfo = PlayerInfo::new(player.get_puuid().to_string(), player.get_game_name().to_string(), player.get_tag_line().to_string());
            let tier_info: TierInfo = self.asset_repository.fetch_tier(player.get_competitive_tier()).await;
            let stats: Stats;
            let recent_stats: RecentStats = self.fetch_player_recent_stats(player.get_puuid());


            let agent: Agent = self.asset_repository.fetch_agent(player.get_character_id().as_str()).await;

            let player_stats = player.get_stats();
            let (shoot_rate, adr) = self.calc_rate_and_adr(&match_data.round_results, &puuid.to_string());

            stats = Stats::new(
                *player_stats.get_kills(),
                *player_stats.get_deaths(),
                *player_stats.get_assists(),
                (*player_stats.get_kills() as f64 / *player_stats.get_deaths() as f64 * 100_f64).round() / 100_f64,
                0_f64,
                *player_stats.get_score(),
                (*player_stats.get_score() as f32 / *player_stats.get_rounds_played() as f32).round(),
                adr,
                shoot_rate
            );

            match_player = MatchPlayer::new(player_info, tier_info, stats, recent_stats, agent, player.get_team_id());
            if player.get_puuid() == &puuid.to_string() {
                user_opt = Some(match_player.clone());
            }
            
            players.push(match_player);
        }
        
        let user = match user_opt {
            Some(user) => user,
            None => return Err(DomainError::GetUserFail)
        };
        let user_team_id = user.get_team_id().to_string();

        let map = self.asset_repository.fetch_map(match_data.match_info.get_map_id()).await;

        if let Some(team) = match_data.teams.iter().find(|t| t.get_team_id() == &user_team_id) {
            let match_info = MatchInfo::new(
                match_data.match_info.get_queue_id(),
                team.get_won(),
                match_data.match_info.get_game_start_millis(),
                team.get_rounds_won(),
                team.get_rounds_played(),
                map
            );
            for round in match_data.round_results {
                let is_win = *round.get_winning_team() == user_team_id;
                rounds.push(Round::new(*round.get_round_num(), is_win, RoundResultData::new(&*round.get_round_ceremony())));
            }
            
            Ok(Match::new(match_info, players, rounds))
        } else {
            Err(DomainError::GetTeamFail)
        }
    }
}


#[async_trait]
impl<MR> HistoryRepository for APIHistoryRepository<MR>
    where
        MR: MatchRepository
{
    //TODO:この辺の修理
    async fn fetch_matches(&self, puuid: &str, mut max: &i32, queue: Option<String>) -> Result<Vec<Match>, DomainError> {
        let matches = match self.req_client.get(format!("{}/val/match/v1/matchlists/by-puuid/{}", RIOT_API_BASE_URL, puuid))
            .send().await {
            Ok(res) => {
                if res.status() != StatusCode::OK {
                    println!("error: {:?}", res.status());
                    return Err(DomainError::RequestFailed);
                }
                match res.json::<RiotMatchList>().await {
                    Ok(matches) => matches,
                    Err(e) => {
                        println!("{:?}", e);
                        return Err(DomainError::ResponseParseError);
                    }
                }
            },
            Err(e) => {
                println!("{:?}", e);
                println!("219");
                return Err(DomainError::RequestFailed);
            }
        };
        let mut history = matches.get_history().into_iter().collect::<Vec<&RiotMatchlistEntry>>();

        let mut matches: Vec<Match> = vec![];

        if queue.is_some() {
            history = history.into_iter().filter( move |m| *m.get_queue() == queue.clone().unwrap()).collect();
        }

        if max == &-1 { max = &5 };
        let idx = max.clone() as usize;

        if &history.len() > &idx {
            history = history.as_slice()[0..idx].to_vec();
        }

        for match_history in &history {
            let player_id = puuid.to_string();
            let match_data = match self.match_repository.fetch_match(player_id.as_str(), match_history.get_match_id()).await {
                Ok(match_data) => match_data,
                Err(e) => {
                    println!("{:?}", e);
                    return Err(DomainError::RequestFailed);
                }
            };
            matches.push(match_data);
        };
        Ok(matches)
    }
}