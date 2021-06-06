use crate::state::SecretContract;
use cosmwasm_std::{HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub accepted_token: SecretContract,
    pub offered_token: SecretContract,
    pub sale_end_time: u64,
    pub viewing_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    ReceiveAcceptedTokenCallback { from: HumanAddr, amount: Uint128 },
    WithdrawFunding { amount: Uint128 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AcceptedTokenAvailable {},
    Config {},
    OfferedTokenAvailable {},
}

// QUERY RESPONSE STRUCTS
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub accepted_token: SecretContract,
    pub admin: HumanAddr,
    pub offered_token: SecretContract,
    pub sale_end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub amount: Uint128,
}
