use cosmwasm_std::{to_binary, Addr, Binary, CanonicalAddr, Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::{
    error::ContractError,
    msg::{
        AdminsResponse, BridgeRecordResponse, CollectionMappingResponse, HistoryResponse,
        OperatorsResponse,
    },
    state::{ADMINS, DEFAULT_LIMIT, HISTORY, MAX_LIMIT, OPERS, TERRA_TO_SN_MAP},
};

/*
 *
 * Query Functions
 *
 */

/// Fetches all admins
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
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
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
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

/// Fetches the destination addresses that correspond to the `source_contracts`
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `source_contracts` - List of Terra collection addresses
pub fn query_collection_mappings(
    deps: Deps,
    source_contracts: Vec<String>,
) -> Result<Binary, ContractError> {
    let destinations = source_contracts
        .iter()
        .map(|addr| {
            let addr = deps.api.addr_validate(addr)?;
            let destination = TERRA_TO_SN_MAP
                .may_load(deps.storage, addr.clone())?
                .ok_or(ContractError::MappingNotFound {
                    source_addr: addr.into_string(),
                })?;
            Ok(destination)
        })
        .collect::<Result<Vec<String>, ContractError>>()?;

    Ok(to_binary(&CollectionMappingResponse { destinations })?)
}

/// Fetches the history for a single token
///
/// # Arguments
///
/// * `deps` - Extern containing all the contract's external dependencies
/// * `collection_address` - The Terra collection's address
/// * `token_id` - ID of the token to fetch the history for
/// * `start_after` - The last index of the last token receieved in the previous query. Used in pagination.
/// * `limit` - The maximum number of records to fetch. Used in pagination
pub fn query_history(
    deps: Deps,
    collection_address: String,
    token_id: String,
    start_after: Option<u64>,
    limit: Option<u8>,
) -> Result<Binary, ContractError> {
    let source_addr = deps.api.addr_validate(&collection_address)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::Exclusive(s.to_be_bytes().into()));

    // Fetch history from storage
    let history = HISTORY
        .prefix((source_addr, token_id))
        .range(deps.storage, start, None, Order::Descending)
        .take(limit)
        // Separate the record from the key
        .map(|pair| Ok(pair?.1.into()))
        .collect::<Result<Vec<BridgeRecordResponse>, ContractError>>()?;

    Ok(to_binary(&HistoryResponse { history })?)
}
