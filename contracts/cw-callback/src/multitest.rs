use crate::{callback_impl::test_utils::Callback, state::CallbackMsg};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Response, StdResult};
use sylvia::{
    contract,
    cw_multi_test::Executor,
    multitest::App,
    types::{ExecCtx, InstantiateCtx},
};

use crate::contract::multitest_utils::CodeId;

#[test]
fn instantiate() {
    // setup
    let app = App::default();
    let code_id = CodeId::store_code(&app);
    let owner = "owner";
    let contract = code_id.instantiate().call(owner).unwrap();

    let callback_msg = contract
        .callback_proxy()
        .get_callback(owner.to_string())
        .unwrap();
    assert_eq!(callback_msg, None);
}

pub struct CallbackContract {}

#[contract]
impl CallbackContract {
    pub const fn new() -> Self {
        Self {}
    }

    #[msg(instantiate)]
    pub fn instantiate(&self, _ctx: InstantiateCtx) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[msg(exec)]
    pub fn foo(&self, _ctx: ExecCtx) -> StdResult<Response> {
        Ok(Response::default())
    }
}

#[cw_serde]
struct UnknownExecuteMsg {}

#[cw_serde]
struct CustomCallback {
    msg: Box<UnknownExecuteMsg>,
    callback: CallbackMsg,
}

#[test]
fn callback() {
    // setup
    let app = App::default();
    let code_id = CodeId::store_code(&app);
    let owner = "owner";
    let contract = code_id.instantiate().call(owner).unwrap();
    let callback_contract_code_id = crate::multitest::multitest_utils::CodeId::store_code(&app);
    let callback_contract = callback_contract_code_id.instantiate().call(owner).unwrap();

    app.app_mut()
        .execute_contract(
            Addr::unchecked(owner),
            contract.contract_addr,
            &CustomCallback {
                msg: Box::new(UnknownExecuteMsg {}),
                callback: CallbackMsg {
                    contract: callback_contract.contract_addr.to_string(),
                    msg: to_binary(&crate::multitest::ExecMsg::Foo {}).unwrap(),
                    funds: vec![],
                },
            },
            &[],
        )
        .unwrap();
}
