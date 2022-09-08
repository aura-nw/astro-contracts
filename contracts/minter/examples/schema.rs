use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema,export_schema_with_title, remove_schemas, schema_for};

// use minter::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use minter::msg::{ MinterQueryMsg,ExecuteMsg, InstantiateMsg };
use cw721_base::{Extension, MinterResponse, QueryMsg};
use minter::state::Config;
use cw721::{
    AllNftInfoResponse, ContractInfoResponse, NftInfoResponse, NumTokensResponse,
    OperatorsResponse, OwnerOfResponse, TokensResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(Config), &out_dir);
    export_schema_with_title(
        &schema_for!(AllNftInfoResponse<Extension>),
        &out_dir,
        "AllNftInfoResponse",
    );
    export_schema(&schema_for!(OperatorsResponse), &out_dir);
    export_schema(&schema_for!(ContractInfoResponse), &out_dir);
    export_schema(&schema_for!(MinterResponse), &out_dir);
    export_schema_with_title(
        &schema_for!(NftInfoResponse<Extension>),
        &out_dir,
        "NftInfoResponse",
    );
    export_schema(&schema_for!(NumTokensResponse), &out_dir);
    export_schema(&schema_for!(OwnerOfResponse), &out_dir);
    export_schema(&schema_for!(TokensResponse), &out_dir);
    export_schema(&schema_for!(MinterQueryMsg), &out_dir);
}
