use crate::callback::callback;
use cosmwasm_std::{from_binary, Binary, Coin, Response, StdError, StdResult};
use cw_storage_plus::Map;
use sylvia::{
    contract, entry_points,
    types::{ExecCtx, InstantiateCtx, QueryCtx},
};
use thiserror::Error;

pub struct CustomCallbackContract<'a> {
    /// Map of sender's last callback.
    pub(crate) callbacks: Map<'a, String, ExecMsg>,
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unknown message {0}")]
    UnknownMsg(Binary),

    #[error("Unauthorized")]
    Unauthorized {},
}

#[entry_points]
#[contract]
#[error(ContractError)]
impl CustomCallbackContract<'_> {
    pub const fn new() -> Self {
        Self {
            callbacks: Map::new("callbacks"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[msg(exec)]
    fn callback(
        &self,
        ctx: ExecCtx,
        contract_msg: Binary,
        contract_funds: Vec<Coin>,
        callback_addr: String,
        callback_msg: Binary,
        callback_funds: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        let msg_result: StdResult<ExecMsg> = from_binary(&contract_msg);
        match msg_result {
            Ok(_) => {
                self.callbacks.save(
                    ctx.deps.storage,
                    ctx.info.sender.to_string(),
                    &ExecMsg::Callback {
                        contract_msg: contract_msg.clone(),
                        contract_funds: contract_funds.clone(),
                        callback_addr: callback_addr.clone(),
                        callback_msg: callback_msg.clone(),
                        callback_funds: callback_funds.clone(),
                    },
                )?;
                let res = callback(
                    ctx.env.contract.address.to_string(),
                    contract_msg,
                    contract_funds,
                    callback_addr,
                    callback_msg,
                    callback_funds,
                )?;
                Ok(res)
            }
            Err(_) => {
                return Err(ContractError::UnknownMsg(contract_msg));
            }
        }
    }

    #[msg(exec)]
    fn success_or_fail(&self, _ctx: ExecCtx, success: bool) -> Result<Response, ContractError> {
        match success {
            true => Ok(Response::default()),
            false => Err(ContractError::Unauthorized {}),
        }
    }

    #[msg(query)]
    fn get_callback(&self, ctx: QueryCtx, sender: String) -> StdResult<Option<ExecMsg>> {
        let callback_msg = self.callbacks.may_load(ctx.deps.storage, sender)?;
        Ok(callback_msg)
    }
}
