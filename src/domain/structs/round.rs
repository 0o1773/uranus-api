use serde::{Deserialize, Serialize};
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Round {
    roundCount: i32,
    isWin: bool,
    roundResultData: RoundResultData,
}
impl Round {
    pub fn new(round_count: i32, is_win: bool, round_result_data: RoundResultData) -> Self {
        Self {
            roundCount: round_count,
            isWin: is_win,
            roundResultData: round_result_data
        }
    }
    pub fn get_round_count(&self) -> &i32 {
        &self.roundCount
    }
    pub fn get_is_win(&self) -> &bool {
        &self.isWin
    }
    pub fn get_round_result_data(&self) -> &RoundResultData {
        &self.roundResultData
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RoundResultData {
    ceremony: String,
}
impl RoundResultData {
    pub fn new(ceremony: &str) -> Self {
        Self { ceremony: ceremony.to_string() }
    }
    pub fn get_ceremony(&self) -> &String {
        &self.ceremony
    }
}