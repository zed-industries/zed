use anyhow::Result;
use serde_json::Value;

pub fn remove_settings_from_http_context_servers(settings: &mut Value) -> Result<()> {
    if let Some(obj) = settings.as_object_mut() {
        if let Some(context_servers) = obj.get_mut("context_servers") {
            if let Some(servers) = context_servers.as_object_mut() {
                for (_, server) in servers.iter_mut() {
                    if let Some(server_obj) = server.as_object_mut() {
                        if server_obj.contains_key("url") {
                            server_obj.remove("settings");
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
