use std::sync::Arc;

use collections::HashMap;
use gpui::App;
use schemars::JsonSchema;
use schemars::r#gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

pub fn init(cx: &mut App) {
    ContextServerSettings::register(cx);
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct ServerConfig {
    /// The command to run this context server.
    ///
    /// This will override the command set by an extension.
    pub command: Option<ServerCommand>,
    /// The settings for this context server.
    ///
    /// Consult the documentation for the context server to see what settings
    /// are supported.
    #[schemars(schema_with = "server_config_settings_json_schema")]
    pub settings: Option<serde_json::Value>,
}

fn server_config_settings_json_schema(_generator: &mut SchemaGenerator) -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::Object.into()),
        ..Default::default()
    })
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ServerCommand {
    pub path: String,
    pub args: Vec<String>,
    pub env: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Default, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ContextServerSettings {
    /// Settings for context servers used in the Assistant.
    #[serde(default)]
    pub context_servers: HashMap<Arc<str>, ServerConfig>,
}

impl Settings for ContextServerSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = Self;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::App,
    ) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut Self::FileContent) {
        // TODO: handle "inputs" replacement strings, see perplexity-key in this example:
        // https://code.visualstudio.com/docs/copilot/chat/mcp-servers#_configuration-example
        #[derive(Deserialize)]
        struct VsCodeServerCommand {
            command: String,
            args: Option<Vec<String>>,
            env: Option<HashMap<String, String>>,
            // TODO: support envFile and type
        }
        impl From<VsCodeServerCommand> for ServerCommand {
            fn from(cmd: VsCodeServerCommand) -> Self {
                Self {
                    path: cmd.command,
                    args: cmd.args.unwrap_or_default(),
                    env: cmd.env,
                }
            }

        }
        if let Some(mcp) = vscode.read_value("mcp").and_then(|v| v.as_object()) {
            current.context_servers.extend(mcp.iter().filter_map(|(k, v)| {
                Some((
                    k.clone().into(),
                    ServerConfig {
                        command: Some(
                            serde_json::from_value::<VsCodeServerCommand>(v.clone())
                                .ok()?
                                .into(),
                        ),
                        settings: None,
                    },
                ))
            }));
        }
    }
}
