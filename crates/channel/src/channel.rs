mod channel_store;

pub mod channel_buffer;
use std::sync::Arc;

pub use channel_store::*;
use client::Client;

#[cfg(test)]
mod channel_store_tests;

pub fn init(client: &Arc<Client>) {
    channel_buffer::init(client);
}
