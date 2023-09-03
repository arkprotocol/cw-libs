use cosmwasm_std::{Response, StdError, StdResult};
use sylvia::{
    interface,
    types::{ExecCtx, QueryCtx},
};

use crate::state::CallbackMsg;

#[interface]
pub trait Callback {
    type Error: From<StdError>;

    #[msg(exec)]
    fn callback(
        &self,
        ctx: ExecCtx,
        msg: Box<crate::contract::ExecMsg>,
        callback: CallbackMsg,
    ) -> StdResult<Response>;

    #[msg(query)]
    fn get_callback(
        &self,
        ctx: QueryCtx,
        sender: String,
    ) -> StdResult<Option<crate::contract::ExecMsg>>;
}
