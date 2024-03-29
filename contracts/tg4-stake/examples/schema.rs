use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, export_schema_with_title, remove_schemas, schema_for};

pub use tg4::{AdminResponse, MemberListResponse, MemberResponse, TotalPointsResponse};
pub use tg4_stake::msg::{
    ClaimsResponse, ExecuteMsg, InstantiateMsg, MigrateMsg, PreauthResponse, QueryMsg,
    StakedResponse, UnbondingPeriodResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema_with_title(&schema_for!(InstantiateMsg), &out_dir, "InstantiateMsg");
    export_schema_with_title(&schema_for!(ExecuteMsg), &out_dir, "ExecuteMsg");
    export_schema_with_title(&schema_for!(QueryMsg), &out_dir, "QueryMsg");
    export_schema_with_title(&schema_for!(MigrateMsg), &out_dir, "MigrateMsg");
    export_schema(&schema_for!(AdminResponse), &out_dir);
    export_schema(&schema_for!(MemberListResponse), &out_dir);
    export_schema(&schema_for!(MemberResponse), &out_dir);
    export_schema(&schema_for!(TotalPointsResponse), &out_dir);
    export_schema(&schema_for!(ClaimsResponse), &out_dir);
    export_schema(&schema_for!(UnbondingPeriodResponse), &out_dir);
    export_schema(&schema_for!(StakedResponse), &out_dir);
    export_schema(&schema_for!(PreauthResponse), &out_dir);
}
