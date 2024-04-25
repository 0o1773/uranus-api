use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Stats {
    kill: i32,
    death: i32,
    assist: i32,
    kd: f64,
    kast: f64,
    combatScore: i32,
    averageCombatScore: f32,
    averageDamageRound: f32,
    shootRate: ShootRate
}

impl Stats {
    pub fn new(kill: i32, death: i32, assist: i32, kd: f64, kast: f64, combat_score: i32, average_combat_score: f32, average_damage_round: f32, shoot_rate: ShootRate) -> Self {
        Self {
            kill,
            death,
            assist,
            kd,
            kast,
            combatScore: combat_score,
            averageCombatScore: average_combat_score,
            averageDamageRound: average_damage_round,
            shootRate: shoot_rate
        }
    }

    pub fn get_kill(&self) -> &i32 { &self.kill }
    pub fn get_death(&self) -> &i32 { &self.death }
    pub fn get_assist(&self) -> &i32 { &self.assist }
    pub fn get_kd(&self) -> &f64 { &self.kd }
    pub fn get_kast(&self) -> &f64 { &self.kast }
    pub fn get_combat_score(&self) -> &i32 { &self.combatScore }
    pub fn get_average_combat_score(&self) -> &f32 { &self.averageCombatScore }
    pub fn get_average_damage_round(&self) -> &f32 { &self.averageDamageRound }
    pub fn get_shoot_rate(&self) -> &ShootRate { &self.shootRate }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CareerResults {
    wins: i32,
    losses: i32,
    draw: i32,
    winRate: f64
}

impl CareerResults {
    pub fn new(wins: i32, losses: i32, draw: i32, win_rate: f64) -> Self {
        Self {
            wins,
            losses,
            draw,
            winRate: win_rate
        }
    }

    pub fn get_wins(&self) -> &i32 { &self.wins }
    pub fn get_losses(&self) -> &i32 { &self.losses }
    pub fn get_draw(&self) -> &i32 { &self.draw }
    pub fn get_win_rate(&self) -> &f64 { &self.winRate }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RecentStats {
    stats: Stats,
    careerStats: CareerResults,
}

impl RecentStats {
    pub fn new(stats: Stats, career_stats: CareerResults) -> Self {
        Self {
            stats,
            careerStats: career_stats
        }
    }

    pub fn get_stats(&self) -> &Stats { &self.stats }
    pub fn get_career_stats(&self) -> &CareerResults { &self.careerStats }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShootRate {
    head: f64,
    body: f64,
    leg: f64
}

impl ShootRate {
    pub fn new(head: f64, body: f64, leg: f64) -> Self {
        Self {
            head,
            body,
            leg
        }
    }

    pub fn get_head(&self) -> &f64 { &self.head }
    pub fn get_body(&self) -> &f64 { &self.body }
    pub fn get_leg(&self) -> &f64 { &self.leg }
}