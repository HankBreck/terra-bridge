use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    msg::{
        InstantiateMsg, ExecuteMsg
    }, 
    execute::{
        try_update_super_user, try_release_nft, try_receive_nft
    }, error::ContractError
};

// add entrypoint
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // instantiate the contract with all admins, operators, and empty tokens vector
    
    Ok(Response::default())
}

// add entrypoint
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmins { 
            add, 
            remove 
        } => try_update_super_user(deps, env, info, true, add, remove),

        ExecuteMsg::UpdateOperators { 
            add, 
            remove 
        } => try_update_super_user(deps, env, info, false, add, remove),

        ExecuteMsg::ReleaseNft { 
            recipient, 
            contract_address, 
            token_id 
        } => try_release_nft(deps, env, info, recipient, contract_address, token_id),

        ExecuteMsg::ReceiveNft(receive_msg) => try_receive_nft(deps, env, info, receive_msg),
    }
}

// add entrypoint for CW721Receive
// source: https://github.com/CosmWasm/cw-nfts/blob/main/contracts/cw721-base/src/execute.rs#L154
