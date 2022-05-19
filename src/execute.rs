use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, CanonicalAddr, StdResult};
use cw721::Cw721ReceiveMsg;

use crate::{
    error::ContractError, 
    state::{
        ADMINS_KEY, 
        load, 
        save, OPERATORS_KEY,
    }
};
        
        
pub fn try_update_super_user(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    is_admin: bool,
    add_list: Option<Vec<String>>,
    remove_list: Option<Vec<String>>,
) -> Result<Response, ContractError> {
    
    // Local state variables
    let storage_key = if is_admin { ADMINS_KEY } else { OPERATORS_KEY };
    let mut save_it = false;
    
    // Check if sender is an admin
    let admins: Vec<CanonicalAddr> = load(deps.storage, ADMINS_KEY)?;
    let sender_raw = deps.api.addr_canonicalize(&info.sender.to_string())?;
    if !admins.contains(&sender_raw) {
        return Err(ContractError::Unauthorized {});
    }

    // Determine whether to update admins or operators
    let mut source_list = if is_admin { admins } else { load(deps.storage, OPERATORS_KEY)? };

    // Add all add_list addresses from storage
    for addr in add_list.unwrap_or_default().iter() {
        // Validate address and convert to raw address
        let addr_raw = deps.api.addr_canonicalize(addr)?;
        if !source_list.contains(&addr_raw) {
            source_list.push(addr_raw);
            save_it = true;
        }
    };

    // Remove all remove_list addresses from storage
    let original_len = source_list.len();
    let to_remove = remove_list
        .unwrap_or_default()
        .iter()
        .map(|addr| deps.api.addr_canonicalize(addr)) // also validates each address
        .collect::<StdResult<Vec<CanonicalAddr>>>()?;
    source_list.retain(|addr| !to_remove.contains(addr));
    // Only update storage if the list has changed
    if original_len > source_list.len() {
        save_it = true;
    }

    if save_it {
        save(deps.storage, storage_key, &source_list)?;
    }
    
    Ok(Response::default())
}

pub fn try_release_nft(
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

pub fn try_receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    receive_msg: Cw721ReceiveMsg,
) -> Result<Response, ContractError> {


    Ok(Response::default())
}
