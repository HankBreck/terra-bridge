use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, CanonicalAddr};

use crate::{
    error::ContractError,
    execute::{try_receive_nft, try_release_nft, try_update_super_user},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    query::{query_admins, query_history, query_operators}, state::{HISTORY_PK, ADMINS, OPERS},
};

// version info for migration info
const CONTRACT_NAME: &str = "terra-bridge";
const CONTRAC_VERSION: &str = env!("CARGO");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;

    // Validate each admin address
    let mut includes_sender = false;
    let mut admins_valid = msg.admins
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
    let opers_valid = msg.operators
        .iter()
        .map(|addr| deps.api.addr_canonicalize(addr))
        .collect::<StdResult<Vec<CanonicalAddr>>>()?;
    
    // Initialize the state
    HISTORY_PK.save(deps.storage, &0u64)?;
    ADMINS.save(deps.storage, &admins_valid)?;
    OPERS.save(deps.storage, &opers_valid)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmins { add, remove } => {
            try_update_super_user(deps, info, true, add, remove)
        }

        ExecuteMsg::UpdateOperators { add, remove } => {
            try_update_super_user(deps, info, false, add, remove)
        }

        ExecuteMsg::ReleaseNft {
            recipient,
            contract_address,
            token_id,
        } => try_release_nft(deps, env, info, recipient, contract_address, token_id),

        ExecuteMsg::ReceiveNft(receive_msg) => {
            try_receive_nft(deps, env, info, receive_msg.sender, receive_msg.token_id)
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admins {} => query_admins(deps, env),
        QueryMsg::Operators {} => query_operators(deps, env),
        QueryMsg::HistoryByToken {
            collection_address,
            token_id,
            start_after,
            limit,
        } => query_history(deps, collection_address, token_id, start_after, limit),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
