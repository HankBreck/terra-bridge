use cosmwasm_std::Addr;
use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::BridgeHistory;

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
    // Admin messages
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
    Admins { },

    /// Lists the contract's operators
    Operators { },

    /// Lists the information for a given NFT
    HistoryByToken {
        /// The address of the collection you wish to view
        collection_address: String,
        /// The token_id of the NFT
        token_id: String,
    }
}

/// Shows the contract's admins
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct AdminsResponse {
    /// A list of all contract admins
    admins: Vec<Addr>,
}

/// Shows the contract's operators
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct OperatorsResponse {
    /// A list of all contract operators
    operators: Vec<Addr>,
}

/// Shows all token_ids for a given collection
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct HistoryByTokenResponse {
    /// Information about an NFT from a given collection
    history: BridgeHistory,
}


pub struct MigrateMsg {
    foo: String,
}
