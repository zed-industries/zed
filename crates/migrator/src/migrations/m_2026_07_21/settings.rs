use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn remove_mcp_registry_source(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(settings: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(context_servers) = settings
        .get_mut("context_servers")
        .and_then(Value::as_object_mut)
    else {
        return Ok(());
    };

    for server in context_servers.values_mut() {
        let Some(registry) = server
            .as_object_mut()
            .and_then(|server| server.get_mut("registry"))
            .and_then(Value::as_object_mut)
        else {
            continue;
        };
        registry.remove("source");
    }

    Ok(())
}
