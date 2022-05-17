use cosmwasm_std::{DepsMut, Env, MessageInfo};


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
) {
    // Check if sender is an admin
    
    // Validate every address, fail if any invalid

    // Perform update via map
}
