use std::any::type_name;

use cosmwasm_std::{Addr, CanonicalAddr, StdError, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex, U64Key};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

// TODO:
// * Whitelist of projects that can cross

/// Storage for the history of a tokens bridging activity
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BridgeRecord {
    /// true if the token has been successfully bridged to SN
    pub is_bridged: bool,
    /// true if the token has been released from the bridge
    pub is_released: bool,
    /// id of bridged token
    pub token_id: String,
    /// the Terra address that initiated the SendMsg request
    pub source_address: Addr,
    /// the address of the Terra collection
    pub source_collection: Addr,
    /// the address of the SN collection
    pub destination_collection: Addr,
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

pub const DEFAULT_LIMIT: u8 = 15;
pub const MAX_LIMIT: u8 = 30;

// Keys
/// Key for HISTORY_PK
pub const HISTORY_PK_NAMESPACE: &str = "history_pk";

// Storage
/// Vector of admins' raw addresses
pub const ADMINS: Item<Vec<CanonicalAddr>> = Item::new("admins");
/// Vector of operators' raw addresses
pub const OPERS: Item<Vec<CanonicalAddr>> = Item::new("operators");
/// Mapping of a Secret Network contract's raw address (as bytes)
/// to a local chain's contract raw address (as bytes)
pub const C_TO_S_MAP: Map<&str, Addr> = Map::new("c_to_s");
/// counter to track the primary key for the history IndexedMap
pub const HISTORY_PK: Item<u64> = Item::new(HISTORY_PK_NAMESPACE);

pub struct BridgeIndexes<'a> {
    pub coll_token_id: MultiIndex<'a, (Addr, String, U64Key), BridgeRecord>,
}

impl<'a> IndexList<BridgeRecord> for BridgeIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<BridgeRecord>> + '_> {
        let v: Vec<&dyn Index<BridgeRecord>> = vec![&self.coll_token_id];
        Box::new(v.into_iter())
    }
}

/// Mapping of history_id -> BridgeRecord
/// * Indexed by (source_collection, token_id, history_id)
pub fn history<'a>() -> IndexedMap<'a, U64Key, BridgeRecord, BridgeIndexes<'a>> {
    let indexes = BridgeIndexes {
        coll_token_id: MultiIndex::new(
            |rec: &BridgeRecord, pk| (rec.source_address.clone(), rec.token_id.clone(), pk.into()),
            HISTORY_PK_NAMESPACE,
            "bridge__coll_token_id",
        ),
    };
    IndexedMap::new(HISTORY_PK_NAMESPACE, indexes)
}

pub fn next_history_pk(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = HISTORY_PK.load(store)? + 1;
    HISTORY_PK.save(store, &id)?;
    Ok(id)
}

/*
 *
 *  Taken from Stashh's SN bridge escrow contract
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
