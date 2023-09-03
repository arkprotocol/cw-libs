use cosmwasm_std::{to_binary, Empty, Response, StdResult, SubMsg, WasmMsg};
use sylvia::{
    contract,
    types::{ExecCtx, QueryCtx},
};

use crate::{
    callback::Callback,
    contract::{ContractError, CwCallbackContract, ExecMsg},
    state::CallbackMsg,
};

#[contract(module=crate::contract)]
#[messages(crate::callback as Callback)]
impl Callback for CwCallbackContract<'_> {
    type Error = ContractError;

    #[msg(exec)]
    fn callback(
        &self,
        ctx: ExecCtx,
        msg: Box<crate::contract::ExecMsg>,
        callback: CallbackMsg,
    ) -> StdResult<Response> {
        let msg = SubMsg::<Empty>::new(WasmMsg::Execute {
            contract_addr: ctx.env.contract.address.to_string(),
            msg: to_binary(&msg)?,
            funds: vec![],
        });
        let callback_msg = WasmMsg::Execute {
            contract_addr: callback.contract,
            msg: callback.msg,
            funds: callback.funds,
        };
        Ok(Response::default()
            .add_submessage(msg)
            .add_message(callback_msg))
    }

    #[msg(query)]
    fn get_callback(&self, ctx: QueryCtx, sender: String) -> StdResult<Option<ExecMsg>> {
        let callback_msg = self.callbacks.may_load(ctx.deps.storage, sender)?;
        Ok(callback_msg)
    }
}
