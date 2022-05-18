use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;


pub fn execute_update_admins(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    add: Option<Vec<String>>,
    remove: Option<Vec<String>>,
) {
    // Check if sender is an admin
    
    // Validate every address, fail if any invalid

    // Perform update via map
}

pub fn execute_update_operators(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    add: Option<Vec<String>>,
    remove: Option<Vec<String>>,
) {
    // Check if sender is an admin
    
    // Validate every address, fail if any invalid

    // Perform update via map
}

pub fn execute_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    // Check if sender is an operator
    
    // Validate the recipient address

    // Call new the new contract to transfer ownership

    Ok(Response::new()
        // .add_submessage()
        .add_attribute("action", "transfer_nft")
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient)
        .add_attribute("contract_address", contract_address)
        .add_attribute("token_id", token_id))
}
