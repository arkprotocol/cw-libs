use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr};
use sylvia::{cw_multi_test::Executor, multitest::App};

use crate::testing::callback_contract::multitest_utils::CodeId;

use super::callback_contract::{ContractError, ExecMsg};

#[cw_serde]
struct UnknownExecuteMsg {}

#[test]
fn instantiate() {
    // setup
    let app = App::default();
    let code_id = CodeId::store_code(&app);
    let owner = "owner";
    let contract = code_id.instantiate().call(owner).unwrap();

    let callback_msg = contract.get_callback(owner.to_string()).unwrap();
    assert_eq!(callback_msg, None);
}

#[test]
fn callback() {
    // setup
    let app = App::default();
    let owner = "owner";
    let custom_callback_contract_code_id =
        crate::testing::callback_contract::multitest_utils::CodeId::store_code(&app);
    let custom_callback_contract = custom_callback_contract_code_id
        .instantiate()
        .call(owner)
        .unwrap();
    let consumer_contract_code_id =
        crate::testing::caller_contract::multitest_utils::CodeId::store_code(&app);
    let consumer_contract = consumer_contract_code_id.instantiate().call(owner).unwrap();

    // call unknown msg
    let error: ContractError = app
        .app_mut()
        .execute_contract(
            Addr::unchecked(owner),
            custom_callback_contract.contract_addr.clone(),
            &ExecMsg::Callback {
                contract_msg: to_binary(&UnknownExecuteMsg {}).unwrap(),
                contract_funds: vec![],
                callback_addr: consumer_contract.contract_addr.to_string(),
                callback_msg: to_binary(&crate::testing::caller_contract::ExecMsg::Foo {}).unwrap(),
                callback_funds: vec![],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        error,
        ContractError::UnknownMsg(to_binary(&UnknownExecuteMsg {}).unwrap())
    );

    // call msg succeeds
    app.app_mut()
        .execute_contract(
            Addr::unchecked(owner),
            custom_callback_contract.contract_addr.clone(),
            &ExecMsg::Callback {
                contract_msg: to_binary(&ExecMsg::SuccessOrFail { success: true }).unwrap(),
                contract_funds: vec![],
                callback_addr: consumer_contract.contract_addr.to_string(),
                callback_msg: to_binary(&crate::testing::caller_contract::ExecMsg::Foo {}).unwrap(),
                callback_funds: vec![],
            },
            &[],
        )
        .unwrap();

    // call msg fails
    let error: ContractError = app
        .app_mut()
        .execute_contract(
            Addr::unchecked(owner),
            custom_callback_contract.contract_addr,
            &ExecMsg::Callback {
                contract_msg: to_binary(&ExecMsg::SuccessOrFail { success: false }).unwrap(),
                contract_funds: vec![],
                callback_addr: consumer_contract.contract_addr.to_string(),
                callback_msg: to_binary(&crate::testing::caller_contract::ExecMsg::Foo {}).unwrap(),
                callback_funds: vec![],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(error, ContractError::Unauthorized {})
}
