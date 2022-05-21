use cosmwasm_std::{Addr, to_binary, Binary, Deps, Env, CanonicalAddr, StdResult};
use cw0::maybe_addr;

use crate::{error::ContractError, msg::{AdminsResponse, OperatorsResponse}, state::{load, OPERS, ADMINS, HISTORY, BridgeHistory}};


/*
 * 
 * Query Functions
 * 
 */

/// Fetches all admins
/// ADD REAL DOCS
pub fn query_admins(
    deps: Deps,
    env: Env,
) -> StdResult<Binary> {
    let admins: Vec<CanonicalAddr> = ADMINS.load(deps.storage)?;
    let resp = AdminsResponse {
        admins: admins
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    to_binary(&resp)
}

/// Fetches all operators
/// ADD REAL DOCS
pub fn query_operators(
    deps: Deps,
    env: Env,
) -> StdResult<Binary> {
    let operators: Vec<CanonicalAddr> = OPERS.load(deps.storage)?;
    let resp = OperatorsResponse {
        operators: operators
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    to_binary(&resp)
}

/// Fetches the history for a single token
/// ADD REAL DOCS
pub fn query_history(
    deps: Deps,
    env: Env,
    collection_address: &str, 
    token_id: &str,
) -> StdResult<Binary> {
    let addr = deps.api.addr_validate(collection_address)?;
    let history: BridgeHistory = HISTORY.load(deps.storage, (&addr, token_id))?;
    to_binary(&history)
}
