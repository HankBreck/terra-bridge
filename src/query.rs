use cosmwasm_std::{Addr, to_binary, Binary, Deps, Env, CanonicalAddr, StdResult};

use crate::{error::ContractError, msg::{AdminsResponse, OperatorsResponse}, state::{load, ADMINS_KEY, OPERATORS_KEY}};


/*
 * 
 * Query Functions
 * 
 */

/// Fetches all admins
pub fn query_admins(
    deps: Deps,
    env: Env,
) -> StdResult<Binary> {
    let admins: Vec<CanonicalAddr> = load(deps.storage, ADMINS_KEY)?;
    let resp = AdminsResponse {
        admins: admins
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    to_binary(&resp)
}

/// Fetches all operators
pub fn query_operators(
    deps: Deps,
    env: Env,
) -> StdResult<Binary> {
    let operators: Vec<CanonicalAddr> = load(deps.storage, OPERATORS_KEY)?;
    let resp = OperatorsResponse {
        operators: operators
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    to_binary(&resp)
}


/*
 * 
 * Query Responses
 * 
 */

