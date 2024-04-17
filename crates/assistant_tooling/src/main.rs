use mod;

use schemars::{schema::RootSchema, schema_for, JsonSchema};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CodebaseQuery {
    pub query: String,
}

fn main() {
    let schema = schema_for!(CodebaseQuery);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
