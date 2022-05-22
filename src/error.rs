use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Unauthorized collection")]
    UnauthorizedCollection {},

    #[error("Invalid address: {address:?}")]
    InvalidAddress { address: String },
}
