use std::collections::BTreeSet;

use remote::IrohConnectionOptions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::SharedString;

use crate::remote_connections::SshProject;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct P2pConnection {
    pub ticket: SharedString,
    #[serde(default)]
    pub projects: BTreeSet<SshProject>,
    /// Name to use for this server in UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

impl From<P2pConnection> for IrohConnectionOptions {
    fn from(val: P2pConnection) -> Self {
        IrohConnectionOptions {
            ticket: val.ticket.parse().expect("invalid ticket"),
            port_forwards: Default::default(),
            nickname: val.nickname,
        }
    }
}
