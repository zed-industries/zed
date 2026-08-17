#![cfg(all(target_arch = "wasm32", target_os = "wasi", feature = "resolve-async"))]

use futures::executor::block_on;
use jsonschema::{async_options, AsyncRetrieve, Uri};
use serde_json::{json, Value};
use std::collections::HashMap;

struct MapAsyncRetriever {
    schemas: HashMap<String, Value>,
}

impl MapAsyncRetriever {
    fn new(entries: impl IntoIterator<Item = (String, Value)>) -> Self {
        Self {
            schemas: entries.into_iter().collect(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl AsyncRetrieve for MapAsyncRetriever {
    async fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        self.schemas
            .get(uri.as_str())
            .cloned()
            .ok_or_else(|| "schema not found".into())
    }
}

#[test]
fn async_retriever_resolves_refs_on_wasi() {
    block_on(async {
        let schema = json!({
            "$ref": "https://example.com/user.json"
        });

        let retriever = MapAsyncRetriever::new([(
            "https://example.com/user.json".to_string(),
            json!({
                "$id": "https://example.com/user.json",
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "age": { "type": "integer", "minimum": 0 }
                },
                "required": ["name"]
            }),
        )]);

        let validator = async_options()
            .with_retriever(retriever)
            .build(&schema)
            .await
            .expect("validator builds on wasm32-wasi");

        assert!(validator.is_valid(&json!({"name": "Ferris", "age": 13})));
        assert!(!validator.is_valid(&json!({"name": 42})));
    });
}
