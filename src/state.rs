use cosmwasm_std::{Addr, CanonicalAddr, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex, U64Key, PrimaryKey, Prefixer};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/*
 *
 * Type Definitions
 * 
 */

/// Storage for the history of a tokens bridging activity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeRecord {
    /// true if the token has been released from the bridge
    pub is_released: bool,
    /// id of bridged token
    pub token_id: String,
    /// the Terra address that initiated the SendMsg request
    pub source_address: Option<Addr>,
    /// the address of the Terra collection
    pub source_collection: Addr,
    /// the SN address that initiated the SendMsg request
    pub destination_address: Option<String>,
    /// the address of the SN collection
    pub destination_collection: String,
    /// the Terra block of the tx
    pub block_height: u64,
    /// the time (in seconds since 01/01/1970) of tx
    pub block_time: u64,
}

/// (contract_address, token_id, history_id)
pub type HistoryPK = (Addr, String, U64Key);

/*
 *
 * Contract State
 *
 */

pub const DEFAULT_LIMIT: u8 = 15;
pub const MAX_LIMIT: u8 = 30;

/*
 * Storage 
 */

/// Vector of admins' raw addresses
pub const ADMINS: Item<Vec<CanonicalAddr>> = Item::new("admins");
/// Vector of operators' raw addresses
pub const OPERS: Item<Vec<CanonicalAddr>> = Item::new("operators");
/// Mapping of a Terra contract's address to a Secret Network contract's address
pub const TERRA_TO_SN_MAP: Map<Addr, String> = Map::new("t_to_s");
/// Mapping of a Terra contract's address to a Secret Network contract's address
pub const SN_TO_TERRA_MAP: Map<String, Addr> = Map::new("s_to_t");
/// Mapping of a Terra contract and token id to the number of TX records for that pair
pub const HISTORY_COUNT: Map<(Addr, String), u64> = Map::new("history_pk");
/// Mapping of a Terra contract, token id, and TX record id to the BridgeRecord for that TX
pub const HISTORY: Map<HistoryPK, BridgeRecord> = Map::new("history");

pub fn next_history_pk(
    store: &mut dyn Storage,
    source_addr: Addr,
    token_id: String,
) -> StdResult<u64> {
    let key_prefix = (source_addr, token_id);
    let id: u64 = HISTORY_COUNT.load(store, key_prefix.clone()).unwrap_or(0u64) + 1;
    HISTORY_COUNT.save(store, key_prefix, &id)?;
    Ok(id)
}

pub fn save_history (
    store: &mut dyn Storage,
    source_collection: Addr,
    token_id: String,
    record: BridgeRecord,
) -> StdResult<u64> {
    let history_id: u64 = next_history_pk(store, source_collection.to_owned(), token_id.to_owned())?;
    HISTORY.save(store, (source_collection, token_id, history_id.into()), &record)?;
    // Return history_id to be used in wasm attributes
    Ok(history_id)
}
