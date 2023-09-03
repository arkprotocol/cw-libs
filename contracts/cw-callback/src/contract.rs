use cosmwasm_std::{Response, StdError, StdResult};
use cw_storage_plus::Map;
use sylvia::{contract, entry_points, types::InstantiateCtx};
use thiserror::Error;

pub struct CwCallbackContract<'a> {
    /// Map of sender's last callback.
    pub(crate) callbacks: Map<'a, String, ExecMsg>,
}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
}

#[entry_points]
#[contract]
#[error(ContractError)]
#[messages(crate::callback as Callback)]
impl CwCallbackContract<'_> {
    pub const fn new() -> Self {
        Self {
            callbacks: Map::new("callbacks"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }
}
