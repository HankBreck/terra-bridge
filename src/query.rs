use cosmwasm_std::{to_binary, Addr, Binary, CanonicalAddr, Deps, Env, Order, StdResult};
use cw0::maybe_addr;
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    msg::{AdminsResponse, OperatorsResponse},
    state::{history, BridgeRecord, ADMINS, DEFAULT_LIMIT, MAX_LIMIT, OPERS},
};

/*
 *
 * Query Functions
 *
 */

/// Fetches all admins
/// ADD REAL DOCS
pub fn query_admins(deps: Deps, env: Env) -> StdResult<Binary> {
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
pub fn query_operators(deps: Deps, env: Env) -> StdResult<Binary> {
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
    collection_address: String,
    token_id: String,
    start_after: Option<u64>,
    limit: Option<u8>,
) -> StdResult<Binary> {
    let addr = deps.api.addr_validate(&collection_address)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::Exclusive(s.to_be_bytes().into()));

    // Fetch history from storage
    let history = history()
        .idx
        .coll_token_id
        .prefix((addr, token_id))
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        // Separate the records from the indexes
        .map(|item| item.and_then(|vals| Ok(vals.1)))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&history)
}
