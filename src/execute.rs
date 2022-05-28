use cosmwasm_std::{
    from_binary, to_binary, Binary, CanonicalAddr, DepsMut, Env, MessageInfo, Response, StdResult,
    WasmMsg,
};
use cw721::Cw721ExecuteMsg::{SendNft, TransferNft};

use crate::{
    error::ContractError,
    msg::CollectionMapping,
    state::{
        save_history, BridgeRecord, ADMINS, IS_COLL_PAUSED, IS_PAUSED, OPERS, SN_TO_TERRA_MAP,
        TERRA_TO_SN_MAP,
    },
    utils::{check_is_admin, check_is_operator, check_is_paused},
};

/// Allows operators to release NFTs from bridge escrow.
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `info` - additional information about the message sender and attached funds
/// * `is_admin` - `true` to update admins, `false` to update operators
/// * `add_list` - a list of [CollectionMapping] structures to add
/// * `remove_list` - a list of [CollectionMapping] structures to remove
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
    Ok(Response::default().add_attribute("action", action))
}

/// Fetches all admins
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `info` - Extern containing all the contract's external dependencies
/// * `pause` - Extern containing all the contract's external dependencies
/// * `collection` - Extern containing all the contract's external dependencies
pub fn try_update_pause(
    deps: DepsMut,
    info: MessageInfo,
    pause: bool,
    collection: Option<String>,
) -> Result<Response, ContractError> {
    // Verify sender is an admin
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if !check_is_admin(deps.storage, sender_raw)? {
        return Err(ContractError::Unauthorized {});
    }

    // Update state with the new value
    if let Some(coll) = collection {
        // Verify the collection is valid
        let coll_valid = deps.api.addr_validate(&coll)?;
        IS_COLL_PAUSED.save(deps.storage, coll_valid, &pause)?;
    } else {
        IS_PAUSED.save(deps.storage, &pause)?;
    }

    Ok(Response::new()
        .add_attribute("action", "update_pause")
        .add_attribute("new_value", pause.to_string()))
}

/// Updates the collection mappings in storage.
/// All items in `rem_list` are removed before adding items from `add_list`.
/// * Sender must be an admin or operator
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
    rem_list: Option<Vec<CollectionMapping>>,
    add_list: Option<Vec<CollectionMapping>>,
) -> Result<Response, ContractError> {
    // Verify sender is an operator
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if !check_is_operator(deps.storage, sender_raw)? {
        return Err(ContractError::Unauthorized {});
    }

    // Remove items first so we can safely update a key's mapping in one message
    for pair in rem_list.unwrap_or_default() {
        let source = deps.api.addr_validate(&pair.source)?;
        let existing_dest = TERRA_TO_SN_MAP.load(deps.storage, source.to_owned())?;

        // Ensure mapping is valid before removing keys
        if existing_dest != pair.destination {
            return Err(ContractError::MappingNotFound {
                source_addr: source.into_string(),
            });
        }
        TERRA_TO_SN_MAP.remove(deps.storage, source);
        SN_TO_TERRA_MAP.remove(deps.storage, pair.destination);
    }

    // Create new mapping in storage for each CollectionMapping
    for pair in add_list.unwrap_or_default() {
        let source = deps.api.addr_validate(&pair.source)?;
        let dest = pair.destination;
        TERRA_TO_SN_MAP.update(deps.storage, source.to_owned(), |existing| match existing {
            // Do not allow key overwrites
            Some(_) => Err(ContractError::MappingExists {
                source_addr: source.to_owned().into_string(),
            }),
            None => Ok(dest.to_owned()),
        })?;
        SN_TO_TERRA_MAP.update(deps.storage, dest.to_owned(), |existing| match existing {
            // Do not allow key overwrites
            Some(_) => Err(ContractError::MappingExists { source_addr: dest }),
            None => Ok(source),
        })?;
    }

    Ok(Response::default().add_attribute("action", "update_collection_mappings"))
}

/// Allows operators to release NFTs from bridge escrow.
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `env` - Env of the contract's environment
/// * `info` - additional information about the message sender and attached funds
/// * `sn_coll_addr` - the SN collection's address
/// * `sn_sender` - the SN address that bridged the NFT
/// * `coll_addr` - the Terra collection's address
/// * `recipient` - the Terra address receiving the bridged NFTs
/// * `token_id` - id of the token being bridged
#[allow(clippy::too_many_arguments)]
pub fn try_release_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sn_coll_addr: String,
    sn_sender: String,
    recipient: String,
    token_id: String,
    recipient_is_contract: bool,
) -> Result<Response, ContractError> {
    // Check if sender is an operator or admin
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if !check_is_operator(deps.storage, sender_raw)? {
        return Err(ContractError::Unauthorized {});
    }

    let recipient_valid = deps.api.addr_validate(&recipient)?;
    let terra_collection = SN_TO_TERRA_MAP.load(deps.storage, sn_coll_addr.to_owned())?;

    // Check if the bridge is paused
    let is_paused = check_is_paused(deps.storage, terra_collection.to_owned())?;
    if is_paused {
        return Err(ContractError::BridgePaused {});
    }

    // Create & save history
    let record = BridgeRecord {
        is_enter: false,
        token_id: token_id.to_owned(),
        source_address: Some(recipient_valid),
        source_collection: terra_collection.to_owned(),
        destination_address: Some(sn_sender.to_owned()),
        destination_collection: sn_coll_addr.to_owned(),
        block_height: env.block.height,
        block_time: env.block.time.seconds(),
    };
    let history_id = save_history(
        deps.storage,
        terra_collection.to_owned(),
        token_id.to_owned(),
        record,
    )?;

    // Create the "fire and forget" message to transfer ownership
    let msg = if recipient_is_contract {
        SendNft {
            contract: recipient.to_owned(),
            token_id: token_id.to_owned(),
            msg: Binary::from(vec![]),
        }
    } else {
        TransferNft {
            recipient: recipient.to_owned(),
            token_id: token_id.to_owned(),
        }
    };

    let send = WasmMsg::Execute {
        contract_addr: terra_collection.to_owned().into(),
        msg: to_binary(&msg)?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(send)
        .add_attribute("action", "transfer_nft")
        .add_attribute("secret_sender", sn_sender)
        .add_attribute("recipient", recipient)
        .add_attribute("terra_collection", terra_collection)
        .add_attribute("secret_collection", sn_coll_addr)
        .add_attribute("token_id", token_id)
        .add_attribute("history_id", history_id.to_string()))
}

/// Allows operators to release NFTs from bridge escrow.
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `env` - Env of the contract's environment
/// * `info` - additional information about the message sender and attached funds
/// * `sender` - the Terra address bridging the NFT (from Cw721ReceiveMsg)
/// * `token_id` - id of the token being bridged
pub fn try_receive_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    sender: String,
    token_id: String,
    msg: Binary,
) -> Result<Response, ContractError> {
    // Check if the bridge is paused
    let is_paused = check_is_paused(deps.storage, info.sender.to_owned())?;
    if is_paused {
        return Err(ContractError::BridgePaused {});
    }

    // Validate NFT sender
    let sender_addr = deps.api.addr_validate(&sender)?;

    // Check whitelist to see if the collection is mapped to Secret
    let sn_coll_addr = TERRA_TO_SN_MAP
        .may_load(deps.storage, info.sender.to_owned())?
        .ok_or(ContractError::UnauthorizedCollection {})?;
    let sn_sender: String = from_binary(&msg)?;

    // Save history
    let record = BridgeRecord {
        token_id: token_id.to_owned(),
        is_enter: true,
        source_address: Some(sender_addr.to_owned()),
        source_collection: info.sender.to_owned(),
        destination_address: Some(sn_sender.to_owned()),
        destination_collection: sn_coll_addr.to_owned(),
        block_height: env.block.height,
        block_time: env.block.time.seconds(),
    };

    // Load next primary key and save history to storage
    let hist_id = save_history(deps.storage, info.sender.to_owned(), token_id, record)?;

    Ok(Response::default()
        .add_attribute("action", "receive_nft")
        .add_attribute("terra_sender", sender_addr)
        .add_attribute("secret_sender", sn_sender)
        .add_attribute("terra_collection_addr", info.sender)
        .add_attribute("secret_collection_addr", sn_coll_addr)
        .add_attribute("history_id", hist_id.to_string()))
}
