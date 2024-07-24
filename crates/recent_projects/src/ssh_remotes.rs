use anyhow::Result;
use gpui::AppContext;
use remote::SshConnectionOptions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Deserialize)]
pub struct SshSettings {
    pub ssh_connections: Vec<SshConnection>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SshConnection {
    pub host: String,
    pub username: Option<String>,
    pub port: Option<u16>,
    pub projects: Vec<SshProject>,
}
impl From<SshConnection> for SshConnectionOptions {
    fn from(val: SshConnection) -> Self {
        SshConnectionOptions {
            host: val.host,
            username: val.username,
            port: val.port,
            password: None,
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct SshProject {
    pub paths: Vec<String>,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct RemoteSettingsContent {
    pub ssh_connections: Option<Vec<SshConnection>>,
}

impl Settings for SshSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = RemoteSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        sources.json_merge()
    }
}
