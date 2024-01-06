use cosmwasm_std::{Addr, DepsMut, Response, StdResult};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sylvia::{
    contract, entry_points,
    types::{ExecCtx, QueryCtx},
};

use crate::errors::ContractError;
use crate::response::{OwnerResponse, TodoResponse, TodosResponse};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Todo {
    pub item: String,
    pub completed: bool,
}

pub struct TodoContract {
    pub(crate) owner: Item<'static, Addr>,
    pub(crate) todos: Item<'static, Vec<Todo>>,
}

#[entry_points]
#[contract]
#[error(ContractError)]
impl TodoContract {
    pub const fn new() -> Self {
        Self {
            owner: Item::new("owner"),
            todos: Item::new("todos"),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(
        &self,
        ctx: (
            sylvia::cw_std::DepsMut,
            sylvia::cw_std::Env,
            sylvia::cw_std::MessageInfo,
        ),
    ) -> StdResult<Response> {
        self.owner.save(ctx.0.storage, &ctx.2.sender)?;
        self.todos.save(ctx.0.storage, &Vec::new())?;
        Ok(Response::default())
    }

    #[msg(query)]
    pub fn owner(&self, ctx: QueryCtx) -> StdResult<OwnerResponse> {
        let owner = self.owner.load(ctx.deps.storage)?;
        Ok(OwnerResponse { owner })
    }

    #[msg(query)]
    pub fn get_all_todos(&self, ctx: QueryCtx) -> StdResult<TodosResponse> {
        let todos = self.todos.load(ctx.deps.storage)?;
        Ok(TodosResponse { todos })
    }

    #[msg(query)]
    pub fn get_todo(&self, ctx: QueryCtx, id: usize) -> Result<TodoResponse, ContractError> {
        let todos = self.todos.load(ctx.deps.storage)?;
        let todo = todos.get(id);
        if todo.is_none() {
            return Err(ContractError::TodoNotFound);
        }

        Ok(TodoResponse {
            todo: todo.unwrap().to_owned(),
        })
    }

    #[msg(exec)]
    pub fn add_todo(&self, ctx: ExecCtx, todo_item: String) -> Result<Response, ContractError> {
        self.check_owner(ctx.info.sender, &ctx.deps)?;
        let todo = Todo {
            item: todo_item,
            completed: false,
        };
        self.todos
            .update(ctx.deps.storage, |mut t| -> StdResult<Vec<Todo>> {
                t.push(todo);
                Ok(t)
            })?;
        Ok(Response::default())
    }

    #[msg(exec)]
    pub fn complete_todo(&self, ctx: ExecCtx, id: usize) -> Result<Response, ContractError> {
        self.check_owner(ctx.info.sender, &ctx.deps)?;
        self.todos.update(
            ctx.deps.storage,
            |mut t| -> Result<Vec<Todo>, ContractError> {
                let todo = t.get_mut(id);
                if todo.is_none() {
                    return Err(ContractError::TodoNotFound);
                }
                let todo = todo.unwrap();
                if todo.completed {
                    return Err(ContractError::TodoNotFound);
                }
                todo.completed = true;
                Ok(t)
            },
        )?;
        Ok(Response::default())
    }

    fn check_owner(&self, sender: Addr, deps: &DepsMut) -> Result<(), ContractError> {
        let owner = self.owner.load(deps.storage).unwrap();
        if sender != owner {
            return Err(ContractError::NotOwner);
        }
        Ok(())
    }
}
