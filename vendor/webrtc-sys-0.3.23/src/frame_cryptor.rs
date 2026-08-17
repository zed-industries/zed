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

use std::sync::Arc;

use crate::impl_thread_safety;

#[cxx::bridge(namespace = "livekit_ffi")]
pub mod ffi {

    #[derive(Debug)]
    pub struct KeyProviderOptions {
        pub shared_key: bool,
        pub ratchet_window_size: i32,
        pub ratchet_salt: Vec<u8>,
        pub failure_tolerance: i32,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum Algorithm {
        AesGcm = 0,
        AesCbc,
    }

    #[derive(Debug)]
    #[repr(i32)]
    pub enum FrameCryptionState {
        New = 0,
        Ok,
        EncryptionFailed,
        DecryptionFailed,
        MissingKey,
        KeyRatcheted,
        InternalError,
    }

    #[derive(Debug)]
    pub struct EncryptedPacket {
        pub data: Vec<u8>,
        pub iv: Vec<u8>,
        pub key_index: u32,
    }

    unsafe extern "C++" {
        include!("livekit/frame_cryptor.h");

        pub type KeyProvider;

        pub fn new_key_provider(options: KeyProviderOptions) -> SharedPtr<KeyProvider>;

        pub fn set_shared_key(self: &KeyProvider, key_index: i32, key: Vec<u8>) -> bool;

        pub fn ratchet_shared_key(self: &KeyProvider, key_index: i32) -> Result<Vec<u8>>;

        pub fn get_shared_key(self: &KeyProvider, key_index: i32) -> Result<Vec<u8>>;

        pub fn set_sif_trailer(&self, trailer: Vec<u8>);

        pub fn set_key(
            self: &KeyProvider,
            participant_id: String,
            key_index: i32,
            key: Vec<u8>,
        ) -> bool;

        pub fn ratchet_key(
            self: &KeyProvider,
            participant_id: String,
            key_index: i32,
        ) -> Result<Vec<u8>>;

        pub fn get_key(
            self: &KeyProvider,
            participant_id: String,
            key_index: i32,
        ) -> Result<Vec<u8>>;
    }

    unsafe extern "C++" {
        include!("livekit/frame_cryptor.h");
        include!("livekit/rtp_sender.h");
        include!("livekit/rtp_receiver.h");
        include!("livekit/peer_connection_factory.h");

        type RtpSender = crate::rtp_sender::ffi::RtpSender;
        type RtpReceiver = crate::rtp_receiver::ffi::RtpReceiver;
        type PeerConnectionFactory = crate::peer_connection_factory::ffi::PeerConnectionFactory;

        pub type FrameCryptor;

        pub fn new_frame_cryptor_for_rtp_sender(
            peer_factory: SharedPtr<PeerConnectionFactory>,
            participant_id: String,
            algorithm: Algorithm,
            key_provider: SharedPtr<KeyProvider>,
            sender: SharedPtr<RtpSender>,
        ) -> SharedPtr<FrameCryptor>;

        pub fn new_frame_cryptor_for_rtp_receiver(
            peer_factory: SharedPtr<PeerConnectionFactory>,
            participant_id: String,
            algorithm: Algorithm,
            key_provider: SharedPtr<KeyProvider>,
            receiver: SharedPtr<RtpReceiver>,
        ) -> SharedPtr<FrameCryptor>;

        pub fn set_enabled(self: &FrameCryptor, enabled: bool);

        pub fn enabled(self: &FrameCryptor) -> bool;

        pub fn set_key_index(self: &FrameCryptor, index: i32);

        pub fn key_index(self: &FrameCryptor) -> i32;

        pub fn participant_id(self: &FrameCryptor) -> String;

        pub fn register_observer(
            self: &FrameCryptor,
            observer: Box<RtcFrameCryptorObserverWrapper>,
        );

        pub fn unregister_observer(self: &FrameCryptor);
    }

    unsafe extern "C++" {
        include!("livekit/frame_cryptor.h");

        pub type DataPacketCryptor;

        pub fn new_data_packet_cryptor(
            algorithm: Algorithm,
            key_provider: SharedPtr<KeyProvider>,
        ) -> SharedPtr<DataPacketCryptor>;

        pub fn encrypt_data_packet(
            self: &DataPacketCryptor,
            participant_id: String,
            key_index: u32,
            data: Vec<u8>,
        ) -> Result<EncryptedPacket>;

        pub fn decrypt_data_packet(
            self: &DataPacketCryptor,
            participant_id: String,
            encrypted_packet: &EncryptedPacket,
        ) -> Result<Vec<u8>>;
    }

    extern "Rust" {
        type RtcFrameCryptorObserverWrapper;

        fn on_frame_cryption_state_change(
            self: &RtcFrameCryptorObserverWrapper,
            participant_id: String,
            state: FrameCryptionState,
        );
    }
} // namespace livekit_ffi

impl_thread_safety!(ffi::FrameCryptor, Send + Sync);
impl_thread_safety!(ffi::KeyProvider, Send + Sync);
impl_thread_safety!(ffi::DataPacketCryptor, Send + Sync);

use ffi::FrameCryptionState;

// Re-export the EncryptedPacket for convenience
pub use ffi::EncryptedPacket;

pub trait RtcFrameCryptorObserver: Send + Sync {
    fn on_frame_cryption_state_change(&self, participant_id: String, state: FrameCryptionState);
}

pub struct RtcFrameCryptorObserverWrapper {
    observer: Arc<dyn RtcFrameCryptorObserver>,
}

impl RtcFrameCryptorObserverWrapper {
    pub fn new(observer: Arc<dyn RtcFrameCryptorObserver>) -> Self {
        Self { observer }
    }

    fn on_frame_cryption_state_change(
        self: &RtcFrameCryptorObserverWrapper,
        participant_id: String,
        state: FrameCryptionState,
    ) {
        self.observer.on_frame_cryption_state_change(participant_id, state);
    }
}

/// High-level Rust wrapper for data packet cryptor functionality
pub struct DataPacketCryptor {
    inner: cxx::SharedPtr<ffi::DataPacketCryptor>,
}

impl DataPacketCryptor {
    /// Create a new data packet cryptor with the specified algorithm and key provider
    pub fn new(algorithm: ffi::Algorithm, key_provider: cxx::SharedPtr<ffi::KeyProvider>) -> Self {
        Self { inner: ffi::new_data_packet_cryptor(algorithm, key_provider) }
    }

    /// Encrypt data for a specific participant
    pub fn encrypt(
        &self,
        participant_id: &str,
        key_index: u32,
        data: &[u8],
    ) -> Result<ffi::EncryptedPacket, Box<dyn std::error::Error>> {
        let data_vec: Vec<u8> = data.to_vec();
        match self.inner.encrypt_data_packet(participant_id.to_string(), key_index, data_vec) {
            Ok(packet) => Ok(packet),
            Err(e) => Err(format!("Encryption failed: {}", e).into()),
        }
    }

    /// Decrypt an encrypted packet for a specific participant
    pub fn decrypt(
        &self,
        participant_id: &str,
        encrypted_packet: &ffi::EncryptedPacket,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        match self.inner.decrypt_data_packet(participant_id.to_string(), encrypted_packet) {
            Ok(data) => Ok(data.into_iter().collect()),
            Err(e) => Err(format!("Decryption failed: {}", e).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_packet_cryptor_creation() {
        let options = ffi::KeyProviderOptions {
            shared_key: true,
            ratchet_window_size: 16,
            ratchet_salt: vec![],
            failure_tolerance: -1,
        };

        let key_provider = ffi::new_key_provider(options);
        let _cryptor = DataPacketCryptor::new(ffi::Algorithm::AesGcm, key_provider);
    }
}
