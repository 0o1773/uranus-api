use tonic::{async_trait, Request, Response, Status};

use accounts::{
    AccountDto,
    accounts_service_server::AccountsService,
    DiscordDto,
    RequestFromId,
    RequestFromToken,
    RiotAccountDto,
    UserDto,
};
use val::{AgentDto, CareerResultsDto, MapDto, MatchDto, MatchesDto, MatchInfoDto, MatchPlayerDto, PlayerInfoDto, RecentStatsDto, RequestMatch, RequestMatches, RoundDto, RoundResultDto, ShootRateDto, StatsDto, TierInfoDto};
use val::valorant_service_server::ValorantService;

use crate::application::traits::account::AccountApplications;
use crate::application::traits::r#match::MatchApplications;
use crate::dependency_injection::{AccServ, AssetServ, MatchServ};
use crate::domain::structs::errors::{AccountRepositoryError, DomainError};

pub mod accounts {
    tonic::include_proto!("uranus.accounts");
}

pub mod val {
    tonic::include_proto!("uranus.valorant");
}

pub struct AccountsHandler {
    account_service: AccServ,
}

impl AccountsHandler {
    pub fn new(account_service: AccServ) -> Self {
        Self {
            account_service,
        }
    }
}

#[async_trait]
impl AccountsService for AccountsHandler {
    async fn get_account_from_id(&self, request: Request<RequestFromId>) -> Result<Response<AccountDto>, Status> {
        let id = request.into_inner().id;
        match self.account_service.fetch_account_by_user_id(id.as_str()).await {
            Ok(account) => {
                Ok(
                    Response::new(
                        AccountDto {
                            user: Some(
                                UserDto {
                                    id: account.get_user().get_id().into(),
                                    email: account.get_user().get_email().clone(),
                                }
                            ),
                            discord: Some(
                                DiscordDto {
                                    id: account.get_discord().get_id().into(),
                                }
                            ),
                            riot: Some(
                                RiotAccountDto {
                                    id: account.get_riot().get_puuid().into(),
                                    name: account.get_riot().get_game_name().into(),
                                    tag: account.get_riot().get_tag_line().into(),
                                }
                            )
                        }
                    )
                )
            }
            Err(err) => {
                if err == AccountRepositoryError::AccountNotFound {
                    Err(Status::not_found("Not Found"))
                } else {
                    Err(Status::internal("Internal Server Error"))
                }
            }
        }
    }

    async fn get_account_from_puuid(&self, request: Request<RequestFromId>) -> Result<Response<AccountDto>, Status> {
        let puuid = request.into_inner().id;
        match self.account_service.fetch_account_by_puuid(puuid.as_str()).await {
            Ok(account) => {
                Ok(
                    Response::new(
                        AccountDto {
                            user: Some(
                                UserDto {
                                    id: account.get_user().get_id().into(),
                                    email: account.get_user().get_email().clone(),
                                }
                            ),
                            discord: Some(
                                DiscordDto {
                                    id: account.get_discord().get_id().into(),
                                }
                            ),
                            riot: Some(
                                RiotAccountDto {
                                    id: account.get_riot().get_puuid().into(),
                                    name: account.get_riot().get_game_name().into(),
                                    tag: account.get_riot().get_tag_line().into(),
                                }
                            )
                        }
                    )
                )
            }
            Err(e) => {
                if e == AccountRepositoryError::AccountNotFound {
                    Err(Status::not_found("Not Found"))
                } else {
                    Err(Status::internal("Internal Server Error"))
                }
            }
        }
    }

    async fn get_account_from_discord_id(&self, request: Request<RequestFromId>) -> Result<Response<AccountDto>, Status> {
        let discord_id = request.into_inner().id;
        match self.account_service.fetch_account_by_discord_id(discord_id.as_str()).await {
            Ok(account) => {
                Ok(
                    Response::new(
                        AccountDto {
                            user: Some(
                                UserDto {
                                    id: account.get_user().get_id().into(),
                                    email: account.get_user().get_email().clone(),
                                }
                            ),
                            discord: Some(
                                DiscordDto {
                                    id: account.get_discord().get_id().into(),
                                }
                            ),
                            riot: Some(
                                RiotAccountDto {
                                    id: account.get_riot().get_puuid().into(),
                                    name: account.get_riot().get_game_name().into(),
                                    tag: account.get_riot().get_tag_line().into(),
                                }
                            )
                        }
                    )
                )
            }
            Err(e) => {
                if e == AccountRepositoryError::AccountNotFound {
                    Err(Status::not_found("Not Found"))
                } else {
                    Err(Status::internal("Internal Server Error"))
                }
            }
        }
    }

    async fn get_player_info_from_puuid(&self, request: Request<RequestFromId>) -> Result<Response<RiotAccountDto>, Status> {
        let puuid = request.into_inner().id;

        match self.account_service.fetch_player_info_by_puuid(puuid.as_str()).await {
            Ok(p) => Ok(
                Response::new(
                    RiotAccountDto {
                        id: p.get_puuid().into(),
                        name: p.get_game_name().into(),
                        tag: p.get_tag_line().into(),
                    }
                )
            ),
            Err(_) => {
                return Err(Status::internal("Internal Server Error"));
            }
        }
    }


    async fn get_account_from_token(&self, request: Request<RequestFromToken>) -> Result<Response<AccountDto>, Status> {
        todo!()
    } //TODO:実装
    async fn get_player_info_from_token(&self, request: Request<RequestFromToken>) -> Result<Response<RiotAccountDto>, Status> {
        todo!()
    } //TODO:実装
}

pub struct ValHandler {
    asset_service: AssetServ,
    match_service: MatchServ,
}

impl ValHandler {
    pub fn new(asset_service: AssetServ, match_service: MatchServ) -> Self {
        Self {
            asset_service,
            match_service,
        }
    }
}

#[async_trait]
impl ValorantService for ValHandler {
    async fn get_matches(&self, request: Request<RequestMatches>) -> Result<Response<MatchesDto>, Status> {
        let req_data = request.into_inner();
        let puuid = &req_data.player_id;
        let queue = &req_data.queue;

        let max = match &req_data.max {
            Some(max) => max,
            None => &5,
        };

        match self.match_service.fetch_matches(puuid.as_str(), &max, queue.clone()).await {
            Ok(matches) => {
                let mut match_dto = MatchesDto {
                    matches: vec![],
                };
                //TODO: ここを並列処理化
                for m in matches {
                    let match_info = m.get_match_info();

                    let players = m.get_players();
                    let rounds = m.get_rounds();
                    let mut players_dto = vec![];
                    let mut rounds_dto = vec![];

                    for p in players {
                        let player_info = p.get_player_info();
                        let agent_info = p.get_agent();
                        let player_tier = p.get_tier();
                        let player_stats = p.get_stats();
                        let shoot_rate = player_stats.get_shoot_rate();

                        let player_recent_matches = p.get_recent_stats();
                        let player_recent_stats = player_recent_matches.get_stats();
                        let player_recent_career = player_recent_matches.get_career_stats();
                        let recent_shoot_rate = player_recent_stats.get_shoot_rate();
                        players_dto.push(
                            MatchPlayerDto {
                                agent: Some(AgentDto {
                                    id: agent_info.get_id().clone(),
                                    name: agent_info.get_name().clone(),
                                    icon: agent_info.get_icon().clone(),
                                    log_img: agent_info.get_log_img().clone(),
                                    role: agent_info.get_role().clone(),
                                }),
                                player_info: Some(PlayerInfoDto {
                                    id: player_info.get_puuid().clone(),
                                    name: player_info.get_game_name().clone(),
                                    tag: player_info.get_tag_line().clone(),
                                }),
                                tier_info: Some(TierInfoDto {
                                    tier: player_tier.get_tier().clone(),
                                    tier_name: player_tier.get_tier_name().clone(),
                                    division_name: player_tier.get_division_name().clone(),
                                    icon: player_tier.get_icon().clone(),
                                    color: player_tier.get_color().clone(),
                                }),
                                stats: Some(StatsDto {
                                    kill: player_stats.get_kill().clone(),
                                    death: player_stats.get_death().clone(),
                                    assist: player_stats.get_assist().clone(),
                                    kd: player_stats.get_kd().clone() as f32,
                                    shoot_rate: Some(ShootRateDto {
                                        head_shot: shoot_rate.get_head().clone() as f32,
                                        body_shot: shoot_rate.get_body().clone() as f32,
                                        leg_shot: shoot_rate.get_leg().clone() as f32,
                                    }),
                                    combat_score: player_stats.get_combat_score().clone(),
                                    average_combat_score: player_stats.get_average_combat_score().clone() as i32,
                                    average_damage_per_round: player_stats.get_average_damage_round().clone(),
                                }),
                                recent_stats: Some(RecentStatsDto {
                                    stats: Some(StatsDto {
                                        kill: player_recent_stats.get_kill().clone(),
                                        death: player_recent_stats.get_death().clone(),
                                        assist: player_recent_stats.get_assist().clone(),
                                        kd: player_recent_stats.get_kd().clone() as f32,
                                        shoot_rate: Some(ShootRateDto {
                                            head_shot: recent_shoot_rate.get_head().clone() as f32,
                                            body_shot: recent_shoot_rate.get_body().clone() as f32,
                                            leg_shot: recent_shoot_rate.get_leg().clone() as f32,
                                        }),
                                        combat_score: player_recent_stats.get_combat_score().clone(),
                                        average_combat_score: player_recent_stats.get_average_combat_score().clone() as i32,
                                        average_damage_per_round: player_recent_stats.get_average_damage_round().clone()
                                    }),
                                    career_results: Some(CareerResultsDto {
                                        wins: player_recent_career.get_wins().clone(),
                                        losses: player_recent_career.get_losses().clone(),
                                        draw: player_recent_career.get_draw().clone(),
                                        win_rate: player_recent_career.get_win_rate().clone() as f32,
                                    }),
                                }),
                            }
                        )
                    }

                    for r in rounds {
                        let round_result_data = r.get_round_result_data();
                        let round_is_win = r.get_is_win();
                        let round_count = r.get_round_count();
                        rounds_dto.push(RoundDto {
                            round_count: round_count.clone(),
                            is_win: round_is_win.clone(),
                            round_result_data: Some(RoundResultDto {
                                ceremony: round_result_data.get_ceremony().clone(),
                                image: "".to_string(),
                            }),
                        })
                    }

                    match_dto.matches.push(
                        MatchDto {
                            match_info: Some(
                                MatchInfoDto {
                                    queue: match_info.get_queue().clone(),
                                    start_timestamp: match_info.get_start_timestamp().clone(),
                                    map: Some (
                                        MapDto {
                                            id: match_info.get_map().get_id().clone(),
                                            name: match_info.get_map().get_name().clone(),
                                        }
                                    ),
                                    total_round: match_info.get_total_round_count().clone(),
                                    win_round: match_info.get_win_round_count().clone(),
                                    is_win: match_info.get_is_win().clone(),
                                }
                            ),
                            players: players_dto,
                            rounds: rounds_dto,
                        }
                    );
                }
                Ok(Response::new(match_dto))
            }
            Err(e) => {
                if e == DomainError::DbRowNotFound {
                    Err(Status::not_found("Not Found"))
                } else {
                    println!("{:?}", e);
                    Err(Status::internal("Internal Server Error"))
                }
            }
        }
    }

    async fn get_match(&self, request: Request<RequestMatch>) -> Result<Response<MatchDto>, Status> {
        let req_data = request.into_inner();
        let match_data = match self.match_service.fetch_match(&req_data.player_id.as_str(), &req_data.match_id.as_str()).await {
            Ok(m) => m,
            Err(e) => {
                return if e == DomainError::DbRowNotFound {
                    Err(Status::not_found("Not Found"))
                } else {
                    Err(Status::internal("Internal Server Error"))
                }
            }
        };

        let match_info = match_data.get_match_info();

        let players = match_data.get_players();
        let rounds = match_data.get_rounds();
        let mut players_dto = vec![];
        let mut rounds_dto = vec![];

        for p in players {
            let player_info = p.get_player_info();
            let agent_info = p.get_agent();
            let player_tier = p.get_tier();
            let player_stats = p.get_stats();
            let shoot_rate = player_stats.get_shoot_rate();

            let player_recent_matches = p.get_recent_stats();
            let player_recent_stats = player_recent_matches.get_stats();
            let player_recent_career = player_recent_matches.get_career_stats();
            let recent_shoot_rate = player_recent_stats.get_shoot_rate();
            players_dto.push(
                MatchPlayerDto {
                    agent: Some(AgentDto {
                        id: agent_info.get_id().clone(),
                        name: agent_info.get_name().clone(),
                        icon: agent_info.get_icon().clone(),
                        log_img: agent_info.get_log_img().clone(),
                        role: agent_info.get_role().clone(),
                    }),
                    player_info: Some(PlayerInfoDto {
                        id: player_info.get_puuid().clone(),
                        name: player_info.get_game_name().clone(),
                        tag: player_info.get_tag_line().clone(),
                    }),
                    tier_info: Some(TierInfoDto {
                        tier: player_tier.get_tier().clone(),
                        tier_name: player_tier.get_tier_name().clone(),
                        division_name: player_tier.get_division_name().clone(),
                        icon: player_tier.get_icon().clone(),
                        color: player_tier.get_color().clone(),
                    }),
                    stats: Some(StatsDto {
                        kill: player_stats.get_kill().clone(),
                        death: player_stats.get_death().clone(),
                        assist: player_stats.get_assist().clone(),
                        kd: player_stats.get_kd().clone() as f32,
                        shoot_rate: Some(ShootRateDto {
                            head_shot: shoot_rate.get_head().clone() as f32,
                            body_shot: shoot_rate.get_body().clone() as f32,
                            leg_shot: shoot_rate.get_leg().clone() as f32,
                        }),
                        combat_score: player_stats.get_combat_score().clone(),
                        average_combat_score: player_stats.get_average_combat_score().clone() as i32,
                        average_damage_per_round: player_stats.get_average_damage_round().clone(),
                    }),
                    recent_stats: Some(RecentStatsDto {
                        stats: Some(StatsDto {
                            kill: player_recent_stats.get_kill().clone(),
                            death: player_recent_stats.get_death().clone(),
                            assist: player_recent_stats.get_assist().clone(),
                            kd: player_recent_stats.get_kd().clone() as f32,
                            shoot_rate: Some(ShootRateDto {
                                head_shot: recent_shoot_rate.get_head().clone() as f32,
                                body_shot: recent_shoot_rate.get_body().clone() as f32,
                                leg_shot: recent_shoot_rate.get_leg().clone() as f32,
                            }),
                            combat_score: player_recent_stats.get_combat_score().clone(),
                            average_combat_score: player_recent_stats.get_average_combat_score().clone() as i32,
                            average_damage_per_round: player_recent_stats.get_average_damage_round().clone()
                        }),
                        career_results: Some(CareerResultsDto {
                            wins: player_recent_career.get_wins().clone(),
                            losses: player_recent_career.get_losses().clone(),
                            draw: player_recent_career.get_draw().clone(),
                            win_rate: player_recent_career.get_win_rate().clone() as f32,
                        }),
                    }),
                }
            )
        }

        for r in rounds {
            let round_result_data = r.get_round_result_data();
            let round_is_win = r.get_is_win();
            let round_count = r.get_round_count();
            rounds_dto.push(RoundDto {
                round_count: round_count.clone(),
                is_win: round_is_win.clone(),
                round_result_data: Some(RoundResultDto {
                    ceremony: round_result_data.get_ceremony().clone(),
                    image: "".to_string(),
                }),
            })
        }

        Ok(Response::new(MatchDto {
            match_info: Some(
                MatchInfoDto {
                    queue: match_info.get_queue().clone(),
                    start_timestamp: match_info.get_start_timestamp().clone(),
                    map: Some (
                        MapDto {
                            id: match_info.get_map().get_id().clone(),
                            name: match_info.get_map().get_name().clone(),
                        }
                    ),
                    total_round: match_info.get_total_round_count().clone(),
                    win_round: match_info.get_win_round_count().clone(),
                    is_win: match_info.get_is_win().clone(),
                }
            ),
            players: players_dto,
            rounds: rounds_dto,
        }))
    }
}