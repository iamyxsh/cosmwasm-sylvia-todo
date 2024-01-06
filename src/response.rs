use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::contract::Todo;

#[cw_serde]
pub struct TodoResponse {
    pub todo: Todo,
}

#[cw_serde]
pub struct TodosResponse {
    pub todos: Vec<Todo>,
}

#[cw_serde]
pub struct OwnerResponse {
    pub owner: Addr,
}
