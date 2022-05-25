use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};
use terra_bridge::msg::{
    AdminsResponse, CollectionMappingResponse, ExecuteMsg, HistoryResponse, InstantiateMsg,
    MigrateMsg, OperatorsResponse, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    // Export schema for core contract messages
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    // export_schema(&schema_for!(MigrateMsg), &out_dir);

    // Export schema for query response messages
    export_schema_with_title(&schema_for!(AdminsResponse), &out_dir, "AdminsResponse");
    export_schema_with_title(
        &schema_for!(OperatorsResponse),
        &out_dir,
        "OperatorsResponse",
    );
    export_schema_with_title(
        &schema_for!(CollectionMappingResponse),
        &out_dir,
        "CollectionMappingResponse",
    );
    export_schema_with_title(&schema_for!(HistoryResponse), &out_dir, "HistoryResponse");
}
