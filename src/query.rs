use cosmwasm_std::{to_binary, Addr, Binary, CanonicalAddr, Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::{
    msg::{AdminsResponse, HistoryResponse, OperatorsResponse, CollectionMappingResponse, BridgeRecordResponse},
    state::{history, ADMINS, DEFAULT_LIMIT, MAX_LIMIT, OPERS, COLLECTION_MAP, BridgeRecord}, error::ContractError,
};

/*
 *
 * Query Functions
 *
 */

/// Fetches all admins
/// ADD REAL DOCS
pub fn query_admins(deps: Deps) -> Result<Binary, ContractError> {
    let admins: Vec<CanonicalAddr> = ADMINS.load(deps.storage)?;
    let resp = AdminsResponse {
        admins: admins
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    Ok(to_binary(&resp)?)
}

/// Fetches all operators
/// ADD REAL DOCS
pub fn query_operators(deps: Deps) -> Result<Binary, ContractError> {
    let operators: Vec<CanonicalAddr> = OPERS.load(deps.storage)?;
    let resp = OperatorsResponse {
        operators: operators
            .iter()
            .map(|addr| deps.api.addr_humanize(addr))
            .collect::<StdResult<Vec<Addr>>>()?,
    };
    Ok(to_binary(&resp)?)
}

pub fn query_collection_mappings(
    deps: Deps,
    source_contracts: Vec<String>,
) -> Result<Binary, ContractError> {
    let destinations = source_contracts
        .iter()
        .map(|addr| {
            let addr = deps.api.addr_validate(addr)?;
            let destination = COLLECTION_MAP.may_load(deps.storage, addr.clone())?
                .ok_or(ContractError::MappingNotFound { source_addr: addr.into_string() })?;
            Ok(destination)
        })
        .collect::<Result<Vec<Addr>, ContractError>>()?;

    Ok(to_binary(&CollectionMappingResponse { destinations })?)
}

/// Fetches the history for a single token
/// ADD REAL DOCS
pub fn query_history(
    deps: Deps,
    collection_address: String,
    token_id: String,
    start_after: Option<u64>,
    limit: Option<u8>,
) -> Result<Binary, ContractError> {
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
        .map(|item| item.map(|vals| vals.1.into()))
        .collect::<StdResult<Vec<BridgeRecordResponse>>>()?;

    Ok(to_binary(&HistoryResponse { history })?)
}
