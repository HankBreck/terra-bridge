use std::any::type_name;

use cosmwasm_std::{Addr, StdError, StdResult, Storage, CanonicalAddr};
use cw_storage_plus::{Map, Item};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// TODO:
// * Whitelist of projects that can cross

/// Storage for the history of a tokens bridging activity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeHistory {
    /// true if the token has been successfully bridged to SN
    pub is_bridged: bool,
    /// true if the token has been released from the bridge
    pub is_released: bool,
    /// id of bridged token
    pub token_id: String,
    /// the network the external collection is on
    pub network_id: String,
    /// the Terra address involved in the bridge tx
    pub source_address: String,
    /// the address of the Terra collection
    pub source_collection: Option<String>,
    /// the address of the SN collection
    pub destination_collection: Option<String>,
    /// the Terra block of the tx
    pub block_height: u64,
    /// the time (in seconds since 01/01/1970) of tx
    pub block_time: u64,
}

/*
 *
 * Contract State
 *
 */

/// Vector of admins' raw addresses
pub const ADMINS: Item<Vec<CanonicalAddr>> = Item::new("admins");
/// Vector of operators' raw addresses
pub const OPERS: Item<Vec<CanonicalAddr>> = Item::new("operators");
/// Mapping of a Secret Network contract's raw address to a local chain's contract raw address
pub const S_TO_C_MAP: Map<CanonicalAddr, CanonicalAddr> = Map::new("s_to_c");
/// Mapping of (contract_address, token_id) -> BridgeHistory
/// Current state of the bridged token
pub const HISTORY: Map<(&Addr, &str), BridgeHistory> = Map::new("bridge_history");


/*
 *
 *  Taken from Stashh's bridge escrow contract
 *
 */

 /// network and address of an external collection
#[derive(Serialize, Deserialize)]
pub struct ExternInfo {
    /// network the collection is on
    pub network: String,
    /// address of the collection
    pub address: String,
}

/// Returns StdResult<()> resulting from saving an item to storage
///
/// # Arguments
///
/// * `storage` - a mutable reference to the storage this item should go to
/// * `key` - a byte slice representing the key to access the stored item
/// * `value` - a reference to the item to store
pub fn save<T: Serialize>(storage: &mut dyn Storage, key: &[u8], value: &T) -> StdResult<()> {
    storage.set(
        key,
        &bincode2::serialize(value).map_err(|e| StdError::serialize_err(type_name::<T>(), e))?,
    );
    Ok(())
}

/// Returns StdResult<T> from retrieving the item with the specified key.  Returns a
/// StdError::NotFound if there is no item with that key
///
/// # Arguments
///
/// * `storage` - a reference to the storage this item is in
/// * `key` - a byte slice representing the key that accesses the stored item
pub fn load<T: DeserializeOwned>(storage: &dyn Storage, key: &[u8]) -> StdResult<T> {
    bincode2::deserialize(
        &storage
            .get(key)
            .ok_or_else(|| StdError::not_found(type_name::<T>()))?,
    )
    .map_err(|e| StdError::parse_err(type_name::<T>(), e))
}
