use cosmwasm_std::{CanonicalAddr, DepsMut, Env, MessageInfo, Response, StdResult, Binary, from_binary, Addr};
use cw0::maybe_addr;
use cw_storage_plus::U64Key;

use crate::{
    error::ContractError,
    state::{ADMINS, OPERS, C_TO_S_MAP, BridgeRecord, history, next_history_pk},
};

pub fn try_update_super_user(
    deps: DepsMut,
    info: MessageInfo,
    is_admin: bool,
    add_list: Option<Vec<String>>,
    remove_list: Option<Vec<String>>,
) -> Result<Response, ContractError> {

    // Local state variables
    let mut save_it = false;

    // Check if sender is an admin
    let admins: Vec<CanonicalAddr> = ADMINS.load(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(&info.sender.to_string())?;
    if !admins.contains(&sender_raw) {
        return Err(ContractError::Unauthorized {});
    }

    // Determine whether to update admins or operators
    let mut source_list = if is_admin {
        admins
    } else {
        OPERS.load(deps.storage)?
    };

    // Add all add_list addresses from storage
    for addr in add_list.unwrap_or_default().iter() {
        // Validate address and convert to raw address
        let addr_raw = deps.api.addr_canonicalize(addr)?;
        if !source_list.contains(&addr_raw) {
            source_list.push(addr_raw);
            save_it = true;
        }
    }

    // Remove all remove_list addresses from storage
    let original_len = source_list.len();
    let to_remove = remove_list
        .unwrap_or_default()
        .iter()
        .map(|addr| deps.api.addr_canonicalize(addr)) // also validates each address
        .collect::<StdResult<Vec<CanonicalAddr>>>()?;
    source_list.retain(|addr| !to_remove.contains(addr));
    if original_len > source_list.len() {
        save_it = true;
    }

    // Only update storage source_list changed
    if save_it {
        if is_admin {
            ADMINS.save(deps.storage, &source_list)?;
        } else {
            OPERS.save(deps.storage, &source_list)?;
        }
    }

    let action = format!("update_{}", if is_admin {"admins"} else {"operators"});

    // TODO: Add response attributes
    Ok(Response::default()
        .add_attribute("action", action))
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
    sender: String,
    token_id: String,
) -> Result<Response, ContractError> {
    // Validate NFT sender
    let sender_addr = deps.api.addr_validate(&sender)?;

    // info.sender is the Cosmos contract that sent the NFT
    let sn_coll_addr = C_TO_S_MAP.may_load(deps.storage, info.sender.as_str())?
        .ok_or_else(|| ContractError::UnauthorizedCollection { })?;

    // Save history
    let record = BridgeRecord {
        is_bridged: false,
        token_id: token_id,
        is_released: false,
        source_address: sender_addr,
        source_collection: info.sender,
        destination_collection: sn_coll_addr,
        block_height: env.block.height,
        block_time: env.block.time.seconds(),
    };

    // Load next primary key and save history to storage
    let hist_id = next_history_pk(deps.storage)?;
    history().save(deps.storage, U64Key::new(hist_id), &record)?;
    
    Ok(Response::default()
        .add_attribute("action", "receive_nft")
        .add_attribute("sender", record.source_address)
        .add_attribute("cosmos_collection_addr", record.source_collection)
        .add_attribute("secret_collection_addr", record.destination_collection)
        .add_attribute("history_id", hist_id.to_string()))
}
