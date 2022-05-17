use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::{msg::{InstantiateMsg, ExecuteMsg}, execute::{execute_update_admins, execute_update_operators, execute_transfer}};


pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) {
    // instantiate the contract with all admins, operators, and empty tokens vector
}

pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) {
    // Call execute msgs
    match msg {
        ExecuteMsg::UpdateAdmins { 
            add, 
            remove 
        } => execute_update_admins(deps, env, info, add, remove),

        ExecuteMsg::UpdateOperators { 
            add, 
            remove 
        } => execute_update_operators(deps, env, info, add, remove),

        ExecuteMsg::Transfer { 
            recipient, 
            contract_address, 
            token_id 
        } => execute_transfer(deps, env, info, recipient, contract_address, token_id),
    };

    // Ok(token)
}

