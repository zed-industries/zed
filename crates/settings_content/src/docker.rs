use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DockerSettingsContent {
    /// Seconds between automatic status refreshes. Default: 300. A value of
    /// 0 disables autopolling entirely (manual refresh only).
    pub poll_interval_seconds: Option<u64>,
    /// Configured Docker endpoints. Default: []
    pub connections: Option<Vec<DockerConnectionContent>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct DockerConnectionContent {
    /// Unique display name of the endpoint.
    pub name: String,
    /// How to reach the daemon: "local" (default socket) or "ssh".
    pub kind: DockerEndpointKindContent,
    /// For kind = "ssh": the SSH target `user@host` (used as DOCKER_HOST=ssh://user@host).
    pub ssh_host: Option<String>,
    /// When true, destructive actions (stop/restart/remove/compose down) are blocked. Default: false
    pub read_only: Option<bool>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DockerEndpointKindContent {
    Local,
    Ssh,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn docker_connection_content_roundtrips() {
        let json = r#"{"name":"prod","kind":"ssh","ssh_host":"deploy@1.2.3.4","read_only":true}"#;
        let parsed: DockerConnectionContent = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.name, "prod");
        assert!(matches!(parsed.kind, DockerEndpointKindContent::Ssh));
        assert_eq!(parsed.ssh_host.as_deref(), Some("deploy@1.2.3.4"));
        assert_eq!(parsed.read_only, Some(true));
    }
}
