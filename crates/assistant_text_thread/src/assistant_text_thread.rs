#[cfg(test)]
mod assistant_text_thread_tests;
mod text_thread;
mod text_thread_store;

pub use crate::text_thread::*;
pub use crate::text_thread_store::*;

use client::Client;
use gpui::App;
use std::sync::Arc;

pub fn init(client: Arc<Client>, _: &mut App) {
    text_thread_store::init(&client.into());
}
