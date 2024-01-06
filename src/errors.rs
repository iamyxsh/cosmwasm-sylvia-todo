use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("No Todo Found with this ID.")]
    TodoNotFound,

    #[error("Todo already completed.")]
    TodoAlreadyCompleted,

    #[error("Caller not an owner.")]
    NotOwner,
}
