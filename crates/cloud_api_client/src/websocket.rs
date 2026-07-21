use std::pin::Pin;

use anyhow::Result;
use cloud_api_types::websocket_protocol::MessageToClient;
use futures::Stream;
use futures::channel::mpsc::UnboundedSender;
use yawc::frame::{Frame, OpCode};

#[cfg(not(target_family = "wasm"))]
mod native;
#[cfg(target_family = "wasm")]
mod web;

pub type MessageStream = Pin<Box<dyn Stream<Item = Result<MessageToClient>>>>;

fn forward_frame(frame: Frame, message_tx: &UnboundedSender<Result<MessageToClient>>) -> bool {
    match frame.opcode() {
        OpCode::Binary => message_tx
            .unbounded_send(MessageToClient::deserialize(frame.payload()))
            .is_ok(),
        OpCode::Close => false,
        OpCode::Continuation | OpCode::Text | OpCode::Ping | OpCode::Pong => true,
    }
}
