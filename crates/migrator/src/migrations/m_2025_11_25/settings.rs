use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_nested_settings;

pub fn remove_context_server_source(value: &mut Value) -> Result<()> {
    migrate_nested_settings(value, migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(context_servers) = obj.get_mut("context_servers") else {
        return Ok(());
    };

    let Some(servers) = context_servers.as_object_mut() else {
        return Ok(());
    };

    for (_, server) in servers.iter_mut() {
        if let Some(server_obj) = server.as_object_mut() {
            server_obj.remove("source");
        }
    }

    Ok(())
}
