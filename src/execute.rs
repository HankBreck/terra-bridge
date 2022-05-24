use cosmwasm_std::{CanonicalAddr, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    error::ContractError,
    state::{BridgeRecord, ADMINS, COLLECTION_MAP, OPERS, save_history}, msg::CollectionMapping,
};

pub fn try_update_super_users(
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
    for addr in add_list.unwrap_or_default() {
        // Validate address and convert to raw address
        let addr_raw = deps.api.addr_canonicalize(&addr)?;
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

    let action = format!("update_{}", if is_admin { "admins" } else { "operators" });

    // TODO: Add response attributes
    Ok(Response::default().add_attribute("action", action))
}

/// Updates the collection mappings in storage.
/// All items in `rem_list` are removed before adding items from `add_list`.
/// * Sender must be an admin
/// 
/// # Arguments
/// 
/// * `deps` - a mutable reference to Extern containing all the contract's external dependencies
/// * `info` - additional information about the contract's caller
/// * `add_list` - a list of [CollectionMapping]s to be stored
/// * `rem_list` - a list of Terra contract addresses to be cleared from storage
pub fn try_update_collection_mappings(
    deps: DepsMut,
    info: MessageInfo,
    rem_list: Option<Vec<String>>,
    add_list: Option<Vec<CollectionMapping>>,
) -> Result<Response, ContractError> {
    // Verify sender is an admin
    let admins = ADMINS.load(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if !admins.contains(&sender_raw) {
        return Err(ContractError::Unauthorized { });
    }

    // Remove items first so we can perform safely update a key's mapping in one message
    for addr in rem_list.unwrap_or_default() {
        let source = deps.api.addr_validate(&addr)?;
        COLLECTION_MAP.remove(deps.storage, source);
    }

    // Create new mapping in storage for each CollectionMapping
    for pair in add_list.unwrap_or_default() {
        let source = deps.api.addr_validate(&pair.source)?;
        let dest = deps.api.addr_validate(&pair.destination)?;
        COLLECTION_MAP.update(deps.storage, source.clone(), |existing| match existing {
            // Do not allow key overwrites
            Some(_) => Err(ContractError::MappingExists { source_addr: source.into_string() }),
            None => Ok(dest),
        })?;
    }

    Ok(Response::default().add_attribute("action", "update_collection_mappings"))
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

    // Check whitelist to see if the collection is mapped to Secret
    let sn_coll_addr = COLLECTION_MAP
        .may_load(deps.storage, info.sender.to_owned())?
        .ok_or(ContractError::UnauthorizedCollection { })?;

    // Save history
    let record = BridgeRecord {
        is_bridged: false,
        token_id: token_id.to_owned(),
        is_released: false,
        source_address: sender_addr.to_owned(),
        source_collection: info.sender.to_owned(),
        destination_collection: sn_coll_addr.to_owned(),
        block_height: env.block.height,
        block_time: env.block.time.seconds(),
    };

    // Load next primary key and save history to storage
    // let hist_id = next_history_pk(deps.storage, &sender_addr, &token_id)?;
    // HISTORY.save(deps.storage, (sender_addr.as_str(), &token_id, &hist_id), &record)?;
    let hist_id = save_history(deps.storage, info.sender.to_owned(), token_id, record)?;

    Ok(Response::default()
        .add_attribute("action", "receive_nft")
        .add_attribute("sender", sender_addr)
        .add_attribute("cosmos_collection_addr", info.sender)
        .add_attribute("secret_collection_addr", sn_coll_addr)
        .add_attribute("history_id", hist_id.to_string()))
}
