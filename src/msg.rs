use cosmwasm_std::Addr;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::BridgeRecord;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Initial admin addresses
    pub admins: Vec<String>,
    /// Initial operator addresses
    pub operators: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    /*
     * Admin messages
     */

    /// Update the contract's admins
    UpdateAdmins {
        /// The addresses to add
        add: Option<Vec<String>>,
        /// The addresses to remove
        remove: Option<Vec<String>>,
    },

    /// Update the contract's operators
    UpdateOperators {
        /// The addresses to add
        add: Option<Vec<String>>,
        /// The addresses to remove
        remove: Option<Vec<String>>,
    },

    /// Update the collection mappings used for whitelist.
    /// * to update a collections mapping you can remove the old mapping and add a new mapping in the same message
    UpdateCollectionMapping {
        /// List of source -> destination collection mappings that will be added to the contract's state
        add: Option<Vec<CollectionMapping>>,
        /// List of source addresses to remove from the bridge
        /// * nb: this should rarely be used. Removing an collection that is already bridged could be seen as malicious behavior.
        remove: Option<Vec<String>>,
    },

    /// Transfer ownership of NFT to the new owner
    /// * contract_address, token_id is the key for our NFTs
    ReleaseNft {
        /// Address of the recipient
        recipient: String,
        /// The contract address for the NFT
        contract_address: String,
        /// The token_id for the NFT
        token_id: String,
    },

    /// Accept cw721 NFT
    /// * https://docs.cosmwasm.com/cw-plus/0.9.0/cw721/spec/#receiver
    ReceiveNft(Cw721ReceiveMsg),
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Lists the contract's admins
    Admins {},

    /// Lists the contract's operators
    Operators {},

    /// Returns the Secret network address associated with `source_contract` if a mapping exists. 
    CollectionMappings { source_contracts: Vec<String> },

    /// Lists the information for a given NFT
    HistoryByToken {
        /// The address of the collection you wish to view
        collection_address: String,
        /// The token_id of the NFT
        token_id: String,
        /// The last element from the previous query.
        /// Used in pagination.
        start_after: Option<u64>,
        /// The maximum number of records to show.
        /// Used in pagination.
        limit: Option<u8>,
    },
}

/*
 *
 * Util Structs used in messages
 * 
 */

/*
 * Execute Utils
 */

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollectionMapping {
    pub source: String,
    pub destination: String,
}

/*
 * Query Utils
 */

/// Shows the contract's admins
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AdminsResponse {
    /// A list of all contract admins
    pub admins: Vec<Addr>,
}

/// Shows the contract's operators
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OperatorsResponse {
    /// A list of all contract operators
    pub operators: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CollectionMappingResponse {
    pub destinations: Vec<Addr>,
}

/// Shows all token_ids for a given collection
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct HistoryByTokenResponse {
    /// Information about an NFT from a given collection
    pub history: Vec<BridgeRecord>,
}

/// TODO: Test migration
pub struct MigrateMsg {
    pub foo: String,
}
