mod context;
mod context_store;
mod patch;

use std::sync::Arc;

use client::Client;
use gpui::AppContext;

pub use crate::context::*;
pub use crate::context_store::*;
pub use crate::patch::*;

pub fn init(client: Arc<Client>, _cx: &mut AppContext) {
    context_store::init(&client.into());
}
