// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "__lk-e2e-test")]
use {
    anyhow::{Context, Result},
    common::test_rooms,
    livekit::prelude::PerformRpcData,
    std::time::Duration,
};

mod common;

#[cfg(feature = "__lk-e2e-test")]
#[test_log::test(tokio::test)]
pub async fn test_rpc_invocation() -> Result<()> {
    let mut rooms = test_rooms(2).await?;
    let (caller_room, _) = rooms.pop().unwrap();
    let (callee_room, _) = rooms.pop().unwrap();
    let callee_identity = callee_room.local_participant().identity();

    const METHOD_NAME: &str = "test-method";
    const PAYLOAD: &str = "test-payload";

    callee_room.local_participant().register_rpc_method(METHOD_NAME.to_string(), |data| {
        // Echo caller payload as return value
        Box::pin(async move { Ok(data.payload.to_string()) })
    });

    let perform_data = PerformRpcData {
        method: METHOD_NAME.to_string(),
        destination_identity: callee_identity.to_string(),
        payload: PAYLOAD.to_string(),
        response_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let return_payload = caller_room
        .local_participant()
        .perform_rpc(perform_data)
        .await
        .context("Invocation failed")?;
    assert_eq!(return_payload, PAYLOAD, "Unexpected return value");
    Ok(())
}

#[cfg(feature = "__lk-e2e-test")]
#[test_log::test(tokio::test)]
pub async fn test_rpc_unregistered() -> Result<()> {
    let mut rooms = test_rooms(2).await?;
    let (caller_room, _) = rooms.pop().unwrap();
    let (callee_room, _) = rooms.pop().unwrap();
    let callee_identity = callee_room.local_participant().identity();

    const METHOD_NAME: &str = "unregistered-method";
    const PAYLOAD: &str = "test-payload";

    let perform_data = PerformRpcData {
        method: METHOD_NAME.to_string(),
        destination_identity: callee_identity.to_string(),
        payload: PAYLOAD.to_string(),
        response_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let result = caller_room.local_participant().perform_rpc(perform_data).await;
    assert!(result.is_err(), "Expected error");
    Ok(())
}

#[cfg(feature = "__lk-e2e-test")]
#[test_log::test(tokio::test)]
pub async fn test_rpc_unknown_destination() -> Result<()> {
    let mut rooms = test_rooms(1).await?;
    let (caller_room, _) = rooms.pop().unwrap();

    let perform_data = PerformRpcData {
        method: "unregistered-method".to_string(),
        destination_identity: "unknown-participant".to_string(),
        payload: "test-payload".to_string(),
        response_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let result = caller_room.local_participant().perform_rpc(perform_data).await;
    assert!(result.is_err(), "Expected error");
    Ok(())
}
