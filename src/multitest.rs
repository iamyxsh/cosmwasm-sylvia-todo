use sylvia::multitest::App;

use crate::{contract::sv::multitest_utils::CodeId, errors::ContractError};

const TODO_ITEM: &str = "Complete writting tests.";

#[test]
fn it_instantiates() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let sender = "owner";

    let contract = code_id.instantiate().call(sender).unwrap();

    let owner = contract.owner().unwrap().owner;

    assert_eq!(sender, owner)
}

#[test]
fn it_can_add_todo() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let sender = "owner";

    let contract = code_id.instantiate().call(sender).unwrap();

    contract
        .add_todo(TODO_ITEM.to_string())
        .call(sender)
        .unwrap();

    let todo = contract.get_todo(0).unwrap().todo;

    assert_eq!(todo.item, TODO_ITEM.to_string());
    assert_eq!(todo.completed, false);
}

#[test]
fn it_can_query_todos() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let sender = "owner";

    let contract = code_id.instantiate().call(sender).unwrap();

    contract
        .add_todo(TODO_ITEM.to_string())
        .call(sender)
        .unwrap();

    contract
        .add_todo("Test Todo".to_string())
        .call(sender)
        .unwrap();

    let todos = contract.get_all_todos().unwrap().todos;

    assert_eq!(todos.len(), 2);
}

#[test]
fn it_can_complete_todo() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let sender = "owner";

    let contract = code_id.instantiate().call(sender).unwrap();

    contract
        .add_todo(TODO_ITEM.to_string())
        .call(sender)
        .unwrap();

    contract.complete_todo(0).call(sender).unwrap();

    let todos = contract.get_todo(0).unwrap().todo;
    assert_eq!(todos.completed, true);
}

#[test]
fn it_only_allows_owner() {
    let app = App::default();
    let code_id = CodeId::store_code(&app);

    let sender = "owner";
    let fake_sender = "fake";

    let contract = code_id.instantiate().call(sender).unwrap();

    let err = contract
        .add_todo(TODO_ITEM.to_string())
        .call(fake_sender)
        .unwrap_err();

    assert_eq!(err, ContractError::NotOwner);

    contract
        .add_todo(TODO_ITEM.to_string())
        .call(sender)
        .unwrap();

    let err = contract.complete_todo(0).call(fake_sender).unwrap_err();

    assert_eq!(err, ContractError::NotOwner);
}
