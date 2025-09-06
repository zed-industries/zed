use std::path::PathBuf;

use anyhow::Result;
use collections::HashMap;
use gpui::{App, SharedString};
use project::agent_server_store::AgentServerCommand;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
