// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::io::{self, Write};

pub mod server;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
struct AnyMethodMessage<T> {
    pub jsonrpc: String,
    pub method: &'static str,
    pub params: Option<T>,
}

pub fn send_message<T: serde::Serialize>(method: &'static str, params: Option<T>) {
    let payload = AnyMethodMessage {
        jsonrpc: "2.0".to_string(),
        method,
        params,
    };
    let message = serde_json::to_string(&payload).unwrap();
    print!(
        "Content-Length: {}\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n{}",
        message.len(),
        message
    );
    let _ = io::stdout().flush();
}
pub fn send_reply<T: serde::Serialize>(id: u32, payload: Option<T>) {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "result": payload,
        "id": id
    });
    let message = serde_json::to_string(&payload).unwrap();
    print!(
        "Content-Length: {}\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n{}",
        message.len(),
        message
    );
    let _ = io::stdout().flush();
}

pub fn send_error(id: Option<u32>, code: i32, message: String) {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "error": { "code": code, "message": message },
        "id": id
    });
    let message = serde_json::to_string(&payload).unwrap();
    print!(
        "Content-Length: {}\r\nContent-Type: application/vscode-jsonrpc; charset=utf-8\r\n\r\n{}",
        message.len(),
        message
    );
    let _ = io::stdout().flush();
}
