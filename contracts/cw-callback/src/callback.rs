use cosmwasm_std::{Binary, Coin, Empty, Response, StdResult, SubMsg, WasmMsg};

pub const REPLY_ID_MSG: u64 = 1;

pub fn callback(
    contract_addr: String,
    contract_msg: Binary,
    contract_funds: Vec<Coin>,
    callback_addr: String,
    callback_msg: Binary,
    callback_funds: Vec<Coin>,
) -> StdResult<Response> {
    let msg = SubMsg::<Empty>::new(WasmMsg::Execute {
        contract_addr,
        msg: contract_msg,
        funds: contract_funds,
    });
    let callback_msg = SubMsg::<Empty>::new(WasmMsg::Execute {
        contract_addr: callback_addr,
        msg: callback_msg,
        funds: callback_funds,
    });
    Ok(Response::default().add_submessages([msg, callback_msg]))
}
