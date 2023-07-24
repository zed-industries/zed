mod channels_panel;
mod channels_panel_settings;

pub use channels_panel::*;
use gpui::{AppContext};

use std::sync::Arc;

use client::Client;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    channels_panel::init(cx);
}
