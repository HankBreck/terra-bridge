use cosmwasm_std::{
    entry_point, Binary, CanonicalAddr, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::{
    error::ContractError,
    execute::{
        try_receive_nft, try_release_nft, try_update_collection_mappings, try_update_pause,
        try_update_super_users,
    },
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    query::{query_admins, query_collection_mappings, query_history, query_operators},
    state::{ADMINS, IS_PAUSED, OPERS},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;

    // Validate each admin address
    let mut includes_sender = false;
    let mut admins_valid = msg
        .admins
        .iter()
        .map(|addr| {
            let addr_valid = deps.api.addr_canonicalize(addr)?;
            if addr_valid == sender_raw {
                includes_sender = true;
            }
            Ok(addr_valid)
        })
        .collect::<StdResult<Vec<CanonicalAddr>>>()?;

    // Add sender to admins if not included
    if !includes_sender {
        admins_valid.push(sender_raw);
    }

    // Validate each operator adderss
    let opers_valid = msg
        .operators
        .iter()
        .map(|addr| deps.api.addr_canonicalize(addr))
        .collect::<StdResult<Vec<CanonicalAddr>>>()?;

    // Initialize the state
    ADMINS.save(deps.storage, &admins_valid)?;
    OPERS.save(deps.storage, &opers_valid)?;
    IS_PAUSED.save(deps.storage, &false)?;

    Ok(Response::default()
        .add_attribute("action", "instantiate")
        .add_attribute("sender", info.sender.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Sender must be admin
        ExecuteMsg::UpdateAdmins { add, remove } => {
            try_update_super_users(deps, info, true, add, remove)
        }

        ExecuteMsg::UpdateOperators { add, remove } => {
            try_update_super_users(deps, info, false, add, remove)
        }

        // Sender must be admin or operator
        ExecuteMsg::UpdatePause { pause, collection } => {
            try_update_pause(deps, info, pause, collection)
        }

        ExecuteMsg::UpdateCollectionMapping { add, remove } => {
            try_update_collection_mappings(deps, info, remove, add)
        }

        ExecuteMsg::ReleaseNft {
            recipient,
            sn_collection,
            sn_address,
            token_id,
            recipient_is_contract,
        } => try_release_nft(
            deps,
            env,
            info,
            sn_collection,
            sn_address,
            recipient,
            token_id,
            recipient_is_contract,
        ),

        // Sender must be a cw721 contract
        ExecuteMsg::ReceiveNft(receive_msg) => try_receive_nft(
            deps,
            env,
            info,
            receive_msg.sender,
            receive_msg.token_id,
            receive_msg.msg,
        ),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Admins {} => query_admins(deps),
        QueryMsg::Operators {} => query_operators(deps),
        QueryMsg::CollectionMappings { source_contracts } => {
            query_collection_mappings(deps, source_contracts)
        }
        QueryMsg::HistoryByToken {
            collection_address,
            token_id,
            start_after,
            limit,
        } => query_history(deps, collection_address, token_id, start_after, limit),
    }
}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::new())
}
