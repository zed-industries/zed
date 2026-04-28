mod bencode;
mod client;
mod discovery;
mod nrepl_sessions_ui;
mod nrepl_settings;
mod nrepl_store;

use std::sync::Arc;

use gpui::App;
use project::Fs;
use settings::Settings;

pub use crate::client::{NreplClient, RequestStream};
pub use crate::discovery::{DiscoveredPort, discover_port, discover_port_in};
pub use crate::nrepl_sessions_ui::{Connect, Disconnect, NreplSessionsPage, Sessions};
pub use crate::nrepl_settings::NreplSettings;
pub use crate::nrepl_store::{ConnectTarget, ConnectionState, NreplConnection, NreplStore};

pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    NreplSettings::register(cx);
    NreplStore::init(fs, cx);
    nrepl_sessions_ui::init(cx);
}
