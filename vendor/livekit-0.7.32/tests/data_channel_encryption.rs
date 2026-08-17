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
    crate::common::test_rooms_with_options,
    anyhow::{Ok, Result},
    livekit::{
        e2ee::{
            key_provider::{KeyProvider, KeyProviderOptions},
            EncryptionType,
        },
        DataPacket, E2eeOptions, RoomEvent, RoomOptions,
    },
    std::time::Duration,
    tokio::{time::timeout, try_join},
};

mod common;

#[cfg(feature = "__lk-e2e-test")]
#[tokio::test]
async fn test_data_channel_encryption() -> Result<()> {
    const ITERATIONS: usize = 128;
    const PAYLOAD_SIZE: usize = 4096;

    let key_provider1 =
        KeyProvider::with_shared_key(KeyProviderOptions::default(), "password".as_bytes().to_vec());

    let mut options1 = RoomOptions::default();
    options1.encryption =
        Some(E2eeOptions { key_provider: key_provider1, encryption_type: EncryptionType::Gcm });

    let key_provider2 =
        KeyProvider::with_shared_key(KeyProviderOptions::default(), "password".as_bytes().to_vec());

    let mut options2 = RoomOptions::default();
    options2.encryption =
        Some(E2eeOptions { key_provider: key_provider2, encryption_type: EncryptionType::Gcm });

    let mut rooms = test_rooms_with_options([options1, options2]).await?;

    let (sending_room, _) = rooms.pop().unwrap();
    let (receiving_room, mut receiving_event_rx) = rooms.pop().unwrap();

    sending_room.e2ee_manager().set_enabled(true);
    receiving_room.e2ee_manager().set_enabled(true);

    let send_packets = async move {
        for iteration in 0..ITERATIONS {
            let packet = DataPacket {
                reliable: true,
                // Set all the bytes in the payload equal to the iteration number
                // to verify on the receiver side.
                payload: [iteration as u8; PAYLOAD_SIZE].to_vec(),
                ..Default::default()
            };
            sending_room.local_participant().publish_data(packet).await?;
        }
        Ok(())
    };

    let receive_packets = async move {
        let mut recv_idx = 0;
        while let Some(event) = receiving_event_rx.recv().await {
            let RoomEvent::DataReceived { payload, .. } = event else {
                continue;
            };
            assert!(payload.iter().all(|byte| *byte == recv_idx as u8));
            recv_idx += 1;
            if recv_idx >= ITERATIONS {
                break;
            }
        }
        Ok(())
    };

    timeout(Duration::from_secs(5), async { try_join!(send_packets, receive_packets) }).await??;
    Ok(())
}
