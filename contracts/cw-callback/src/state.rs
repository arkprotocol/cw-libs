use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin};

#[cw_serde]
pub struct CallbackMsg {
    pub contract: String,
    pub msg: Binary,
    pub funds: Vec<Coin>,
}
