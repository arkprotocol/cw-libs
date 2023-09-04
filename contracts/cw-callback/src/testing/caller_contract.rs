use cosmwasm_std::{Response, StdResult};
use sylvia::{
    contract,
    types::{ExecCtx, InstantiateCtx},
};

pub struct CallerContract {}

#[contract]
impl CallerContract {
    pub const fn new() -> Self {
        Self {}
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[msg(exec)]
    pub fn foo(&self, _ctx: ExecCtx) -> StdResult<Response> {
        println!(">>>>>foo");
        Ok(Response::default())
    }
}
