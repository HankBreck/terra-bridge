use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult, entry_point, Deps, Binary, StdError};

use crate::{
    error::ContractError,
    execute::{try_receive_nft, try_release_nft, try_update_super_user},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg}, query::{query_operators, query_admins},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // instantiate the contract with all admins, operators, and empty tokens vector

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

        ExecuteMsg::ReceiveNft(receive_msg) => try_receive_nft(deps, env, info, receive_msg),
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admins { } => query_admins(deps, env),
        QueryMsg::Operators { } => query_operators(deps, env),
        QueryMsg::HistoryByToken { collection_address, token_id } => query_operators(deps, env),

    }
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    env: Env,
    msg: MigrateMsg,
) -> StdResult<Response> {

    Ok(Response::default())
}

// add entrypoint for CW721Receive
// source: https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/execute.rs#L154
