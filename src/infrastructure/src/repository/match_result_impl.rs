use chrono::{DateTime};
use reqwest::Client;
use reqwest::header::HeaderValue;
use sea_orm::{ColIdx, ColumnTrait, DbConn, EntityTrait, ModelTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use domain::objects::error::MatchResultRepositoryError;
use domain::objects::match_result::{MatchResult, ID};
use domain::objects::{account, discord_account, riot_account};
use domain::traits::match_result::MatchResultRepository;
use crate::client::db::entity::{
  account::Entity as AccountEntity,
  riot_account::{
    Entity as RiotAccountEntity,
    Column as RiotAccountColumn
  },
  agent:: {
    Entity as AgentEntity,
    Column as AgentColumn
  },
  map:: {
    Entity as MapEntity,
    Column as MapColumn
  },
  match_result:: {
    Entity as MatchResultEntity,
    ActiveModel as MatchResultModel
  }
};
use crate::dto::valorant::{KillDto, MatchDto};

struct MatchResultRepositoryImpl {
  db: DbConn,
  client: Client
}

impl MatchResultRepositoryImpl {
  pub fn new(db: DbConn, client: Client) -> MatchResultRepositoryImpl {
    MatchResultRepositoryImpl {
      db,
      client
    }
  }
}

impl MatchResultRepository for MatchResultRepositoryImpl {
  async fn create(&self, match_id: String, puuid: String) -> Result<MatchResult, MatchResultRepositoryError> {
    let db = self.db.clone();
    let client = self.client.clone();

    let api_key = std::env::var("RIOT_API_KEY");
    if api_key.is_err() {
      return Err(MatchResultRepositoryError::RiotApiKeyNotFound);
    }

    let mut headers = reqwest::header::HeaderMap::new();
    headers.append("X-Riot-Token", HeaderValue::from_str(api_key.unwrap().as_str()).unwrap());

    let resp = match client.get(format!("https://ap.api.riotgames.com/val/matches/{}", match_id))
      .headers(headers)
      .send()
      .await {
        Ok(resp) => resp,
        Err(_) => return Err(MatchResultRepositoryError::RiotApiError)
      };

    let match_data = match resp.json::<MatchDto>().await {
      Ok(data) => data,
      Err(_) => return Err(MatchResultRepositoryError::RiotApiParseError)
    };

    let player_data = match_data.players.iter().find(|&player| player.puuid == puuid).unwrap();

    let riot_account = match RiotAccountEntity::find()
      .filter(RiotAccountColumn::Puuid.contains(puuid.clone()))
      .one(&db)
      .await {
        Ok(Some(account)) => account,
        Ok(None) => return Err(MatchResultRepositoryError::AccountNotFound),
        Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
    };

    let account = match riot_account.find_related(AccountEntity).one(&db).await {
      Ok(Some(account)) => {
        let id: ID = match account.id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(MatchResultRepositoryError::ParseError)
        };
        let riot_account_id: riot_account::ID = match account.riot_account_id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(MatchResultRepositoryError::ParseError)
        };
        let discord_account_id: discord_account::ID = match account.discord_account_id.try_into() {
          Ok(id) => id,
          Err(_) => return Err(MatchResultRepositoryError::ParseError)
        };

        let account = account::Account::new_account_with_id(
          id,
          riot_account_id,
          discord_account_id
        );
        account
      }
      Ok(None) => return Err(MatchResultRepositoryError::AccountNotFound),
      Err(err) => {
        eprintln!("{:?}", err);
        return Err(MatchResultRepositoryError::DatabaseError);
      }
    };

    let agent = match AgentEntity::find()
      .filter(AgentColumn::AgentId.contains(player_data.agent_id.clone()))
      .one(&db)
      .await {
        Ok(Some(agent)) => agent,
        Ok(None) => return Err(MatchResultRepositoryError::PlayerDataNotFound),
        Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
      };

    let agent_id = match agent.id.try_into() {
      Ok(id) => id,
      Err(_) => return Err(MatchResultRepositoryError::ParseError)
    };

    let map_id = match MapEntity::find()
      .filter(MapColumn::MapId.contains(match_data.match_info.map_id))
      .one(&db)
      .await {
        Ok(Some(map)) => {
          let id: ID = match map.id.try_into() {
            Ok(id) => id,
            Err(_) => return Err(MatchResultRepositoryError::ParseError)
          };
          id
        },
        Ok(None) => return Err(MatchResultRepositoryError::MapNotFound),
        Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
      };

    let mut dmgs = 0;
    let mut hs_count = 0;
    let mut shoots = 0;

    let mut first_blood = 0;
    let mut first_death = 0;

    for r in match_data.rounds {
      let p = match r.player_stats.iter().find(|&player| player.puuid == puuid) {
        Some(player) => Some(player),
        None => None
      };
      if p.is_none() {
        continue;
      }

      let player = p.unwrap();
      for d in &player.damage {
        dmgs += d.damage;
        shoots += d.headshots + d.bodyshots + d.legshots;
        hs_count += d.headshots;
      }

      let mut first_kill: Option<KillDto> = None;

      for s in r.player_stats {
        for kill in s.kills {
          if first_kill.is_none() {
            first_kill = Some(kill);
          } else {
            if first_kill.clone().unwrap().time_since_round_start > kill.time_since_round_start {
              first_kill = Some(kill);
            }
          }
        }
      }

      if first_kill.is_some() && first_kill.clone().unwrap().killer == puuid {
        first_blood += 1;
      } else if first_kill.is_some() && first_kill.clone().unwrap().victim == puuid {
        first_death += 1;
      }
    }
    let hs_rate = hs_count as f32 / shoots as f32;

    let team = match_data.teams.iter().find(|&team| team.team_id == player_data.team_id).unwrap();

    let match_result = MatchResult::new_match_result(match_id, puuid, account.id(), agent_id, map_id, player_data.stats.kills, player_data.stats.deaths, player_data.stats.assists, hs_rate, first_blood, first_death, player_data.stats.score, dmgs, team.rounds_played, team.rounds_won, team.rounds_played - team.rounds_won < team.rounds_won, match_data.match_info.start_time.parse().unwrap());

    let time = DateTime::from_timestamp_millis(match_result.match_start_time()).unwrap().naive_utc();

    let match_result_db_model = MatchResultModel {
      id: Set(match_result.id().to_vec()),
      match_id: Set(match_result.match_id()),
      puuid: Set(match_result.puuid()),
      account_id: Set(match_result.account_id().to_vec()),
      agent_id: Set(match_result.agent_id().to_vec()),
      map_id: Set(match_result.map_id().to_vec()),
      kill: Set(match_result.kill()),
      death: Set(match_result.death()),
      assist: Set(match_result.assist()),
      hs_rate: Set(match_result.hs_rate()),
      first_blood: Set(match_result.first_blood()),
      first_death: Set(match_result.first_death()),
      combat_score: Set(match_result.combat_score()),
      damage: Set(match_result.damage()),
      played_rounds: Set(match_result.played_rounds()),
      win_rounds: Set(match_result.win_rounds()),
      win: Set(match_result.win()),
      match_start_time: Set(time),
    };


    match MatchResultEntity::insert(match_result_db_model).exec(&db).await {
      Ok(_) => {
        Ok(match_result)
      },
      Err(_) => Err(MatchResultRepositoryError::DatabaseError)
    }
  }

  async fn find_by_id(&self, id: ID) -> Result<MatchResult, MatchResultRepositoryError> {
    let db = self.db.clone();
    let result = MatchResultEntity::find_by_id(id.to_vec())
      .one(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
    }

    match result {
      Ok(Some(pear)) => {
        let match_result = MatchResult::new_match_result_with_id(
          pear.id.try_into().unwrap(),
          pear.match_id.to_string(),
          pear.puuid.to_string(),
          ID::try_from(pear.account_id.to_vec()).unwrap(),
          ID::try_from(pear.agent_id.to_vec()).unwrap(),
          ID::try_from(pear.map_id.to_vec()).unwrap(),
          pear.kill,
          pear.death,
          pear.assist,
          pear.hs_rate,
          pear.first_blood,
          pear.first_death,
          pear.combat_score,
          pear.damage,
          pear.played_rounds,
          pear.win_rounds,
          pear.win,
          pear.match_start_time.and_utc().timestamp_millis()
        );
        Ok(match_result)
      },
      Ok(None) => Err(MatchResultRepositoryError::NotFound),
      Err(_) => Err(MatchResultRepositoryError::FindError)
    }
  }

  async fn update(&self, match_result: MatchResult) -> Result<MatchResult, MatchResultRepositoryError> {
    let db = self.db.clone();

    let active_model = MatchResultModel {
      id: Set(match_result.id().to_vec()),
      match_id: Set(match_result.match_id()),
      puuid: Set(match_result.puuid()),
      account_id: Set(match_result.account_id().to_vec()),
      agent_id: Set(match_result.agent_id().to_vec()),
      map_id: Set(match_result.map_id().to_vec()),
      kill: Set(match_result.kill()),
      death: Set(match_result.death()),
      assist: Set(match_result.assist()),
      hs_rate: Set(match_result.hs_rate()),
      first_blood: Set(match_result.first_blood()),
      first_death: Set(match_result.first_death()),
      combat_score: Set(match_result.combat_score()),
      damage: Set(match_result.damage()),
      played_rounds: Set(match_result.played_rounds()),
      win_rounds: Set(match_result.win_rounds()),
      win: Set(match_result.win()),
      match_start_time: Set(DateTime::from_timestamp_millis(match_result.match_start_time()).unwrap().naive_utc()),
    };

    match MatchResultEntity::update(active_model).exec(&db).await {
      Ok(_) => Ok(match_result),
      Err(_) => Err(MatchResultRepositoryError::UpdateError)
    }
  }

  async fn delete(&self, id: ID) -> Result<(), MatchResultRepositoryError> {
    let db = self.db.clone();
    let result = MatchResultEntity::delete_by_id(id.to_vec())
      .exec(&db)
      .await;

    match db.close().await {
      Ok(_) => {},
      Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
    }

    match result {
      Ok(_) => Ok(()),
      Err(_) => Err(MatchResultRepositoryError::DeleteError)
    }
  }

  async fn list(&self, account_id: account::ID) -> Result<Vec<MatchResult>, MatchResultRepositoryError> {
    let db = self.db.clone();

    let account = match AccountEntity::find_by_id(account_id.to_vec())
      .one(&db)
      .await {
        Ok(Some(account)) => account,
        Ok(None) => return Err(MatchResultRepositoryError::AccountNotFound),
        Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
    };

    let match_results = match account.find_related(MatchResultEntity).all(&db).await {
      Ok(match_results) => match_results,
      Err(_) => return Err(MatchResultRepositoryError::DatabaseError)
    };

    let mut results = vec![];

    for m in match_results {
      let match_result = MatchResult::new_match_result_with_id(
        m.id.try_into().unwrap(),
        m.match_id.to_string(),
        m.puuid.to_string(),
        ID::try_from(m.account_id.to_vec()).unwrap(),
        ID::try_from(m.agent_id.to_vec()).unwrap(),
        ID::try_from(m.map_id.to_vec()).unwrap(),
        m.kill,
        m.death,
        m.assist,
        m.hs_rate,
        m.first_blood,
        m.first_death,
        m.combat_score,
        m.damage,
        m.played_rounds,
        m.win_rounds,
        m.win,
        m.match_start_time.and_utc().timestamp_millis()
      );
      results.push(match_result);
    };

    Ok(results)
  }
}