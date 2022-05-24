use cosmwasm_std::{Addr, CanonicalAddr, StdResult, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex, U64Key};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

/*
 * Keys
 */

/// Key for HISTORY_PK
pub const HISTORY_PK_NAMESPACE: &str = "history_pk";

/*
 * Storage 
 */

/// Vector of admins' raw addresses
pub const ADMINS: Item<Vec<CanonicalAddr>> = Item::new("admins");
/// Vector of operators' raw addresses
pub const OPERS: Item<Vec<CanonicalAddr>> = Item::new("operators");
/// Mapping of a Terra contract's address to a Secret Network contract's address
pub const COLLECTION_MAP: Map<Addr, Addr> = Map::new("c_to_s");
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

pub fn next_history_pk(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = HISTORY_PK.load(store)? + 1;
    HISTORY_PK.save(store, &id)?;
    Ok(id)
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
