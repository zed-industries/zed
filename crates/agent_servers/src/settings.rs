use std::path::PathBuf;

use anyhow::Result;
use collections::HashMap;
use gpui::{App, SharedString};
use project::agent_server_store::AgentServerCommand;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsKey, SettingsSources, SettingsUi};

pub fn init(cx: &mut App) {
    AllAgentServersSettings::register(cx);
}
