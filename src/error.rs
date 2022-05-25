use cosmwasm_std::StdError;
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

    #[error("Collection mapping already exists for Terra address {source_addr:?}")]
    MappingExists { source_addr: String },

    #[error("Collection mapping not found for Terra address {source_addr:?}")]
    MappingNotFound { source_addr: String },

    #[error("Bridge is in the paused state. Tokens cannot be transfered in or out.")]
    BridgePaused {}, 
}
