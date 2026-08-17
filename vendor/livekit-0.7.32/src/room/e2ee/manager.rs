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

use std::{collections::HashMap, sync::Arc};

use libwebrtc::{
    native::frame_cryptor::{
        DataPacketCryptor, EncryptedPacket, EncryptionAlgorithm, EncryptionState, FrameCryptor,
    },
    rtp_receiver::RtpReceiver,
    rtp_sender::RtpSender,
};
use parking_lot::Mutex;

use super::{key_provider::KeyProvider, EncryptionType};
use crate::{
    e2ee::E2eeOptions,
    id::{ParticipantIdentity, TrackSid},
    participant::{LocalParticipant, RemoteParticipant},
    prelude::{LocalTrack, LocalTrackPublication, RemoteTrack, RemoteTrackPublication},
    rtc_engine::lk_runtime::LkRuntime,
};

type StateChangedHandler = Box<dyn Fn(ParticipantIdentity, EncryptionState) + Send>;

struct ManagerInner {
    options: Option<E2eeOptions>, // If Some, it means the e2ee was initialized
    enabled: bool,                // Used to enable/disable e2ee
    dc_encryption_enabled: bool,
    frame_cryptors: HashMap<(ParticipantIdentity, TrackSid), FrameCryptor>,
    data_packet_cryptor: Option<DataPacketCryptor>,
}

#[derive(Clone)]
pub struct E2eeManager {
    inner: Arc<Mutex<ManagerInner>>,
    state_changed: Arc<Mutex<Option<StateChangedHandler>>>,
}

impl E2eeManager {
    /// E2eeOptions is an optional parameter. We may support to reconfigure e2ee after connect in
    /// the future.
    pub(crate) fn new(options: Option<E2eeOptions>, with_dc_encryption: bool) -> Self {
        // Create DataPacketCryptor whenever E2EE options are available
        // This allows for decryption even if we're not encrypting our own data
        let data_packet_cryptor = options.as_ref().map(|opts| {
            DataPacketCryptor::new(EncryptionAlgorithm::AesGcm, opts.key_provider.handle.clone())
        });

        Self {
            inner: Arc::new(Mutex::new(ManagerInner {
                enabled: options.is_some(), // Enabled by default if options is provided
                dc_encryption_enabled: options.is_some() && with_dc_encryption,
                options,
                frame_cryptors: HashMap::new(),
                data_packet_cryptor,
            })),
            state_changed: Default::default(),
        }
    }

    pub(crate) fn cleanup(&self) {
        let mut inner = self.inner.lock();
        for cryptor in inner.frame_cryptors.values() {
            cryptor.set_enabled(false);
        }
        inner.frame_cryptors.clear();
    }

    /// Register to e2ee state changes
    /// Used by the room to dispatch the event to the room dispatcher
    pub(crate) fn on_state_changed(
        &self,
        handler: impl Fn(ParticipantIdentity, EncryptionState) + Send + 'static,
    ) {
        *self.state_changed.lock() = Some(Box::new(handler));
    }

    pub(crate) fn initialized(&self) -> bool {
        self.inner.lock().options.is_some()
    }

    /// Called by the room
    pub(crate) fn on_track_subscribed(
        &self,
        track: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    ) {
        if !self.initialized() {
            return;
        }

        if publication.encryption_type() == EncryptionType::None {
            return;
        }

        let identity = participant.identity();
        let receiver = track.transceiver().unwrap().receiver();
        let frame_cryptor = self.setup_rtp_receiver(&identity, receiver);
        self.setup_cryptor(&frame_cryptor);

        let mut inner = self.inner.lock();
        inner.frame_cryptors.insert((identity, publication.sid()), frame_cryptor.clone());
    }

    /// Called by the room
    pub(crate) fn on_local_track_published(
        &self,
        track: LocalTrack,
        publication: LocalTrackPublication,
        participant: LocalParticipant,
    ) {
        if !self.initialized() {
            return;
        }

        if publication.encryption_type() == EncryptionType::None {
            return;
        }

        let identity = participant.identity();
        let sender = track.transceiver().unwrap().sender();
        let frame_cryptor = self.setup_rtp_sender(&identity, sender);
        self.setup_cryptor(&frame_cryptor);

        let mut inner = self.inner.lock();
        inner.frame_cryptors.insert((identity, publication.sid()), frame_cryptor.clone());
    }

    fn setup_cryptor(&self, frame_cryptor: &FrameCryptor) {
        let state_changed = self.state_changed.clone();
        frame_cryptor.on_state_change(Some(Box::new(move |participant_identity, state| {
            if let Some(state_changed) = state_changed.lock().as_ref() {
                state_changed(participant_identity.into(), state);
            }
        })));
    }

    /// Called by the room
    pub(crate) fn on_local_track_unpublished(
        &self,
        publication: LocalTrackPublication,
        participant: LocalParticipant,
    ) {
        self.remove_frame_cryptor(participant.identity(), publication.sid());
    }

    /// Called by the room
    pub(crate) fn on_track_unsubscribed(
        &self,
        _: RemoteTrack,
        publication: RemoteTrackPublication,
        participant: RemoteParticipant,
    ) {
        self.remove_frame_cryptor(participant.identity(), publication.sid());
    }

    pub fn frame_cryptors(&self) -> HashMap<(ParticipantIdentity, TrackSid), FrameCryptor> {
        self.inner.lock().frame_cryptors.clone()
    }

    pub fn enabled(&self) -> bool {
        self.inner.lock().enabled && self.initialized()
    }

    pub fn is_dc_encryption_enabled(&self) -> bool {
        self.inner.lock().dc_encryption_enabled
    }

    pub fn set_enabled(&self, enabled: bool) {
        let inner = self.inner.lock();
        if inner.enabled == enabled {
            return;
        }

        for (_, cryptor) in inner.frame_cryptors.iter() {
            cryptor.set_enabled(enabled);
        }
    }

    pub fn key_provider(&self) -> Option<KeyProvider> {
        let inner = self.inner.lock();
        inner.options.as_ref().map(|opts| opts.key_provider.clone())
    }

    pub fn encryption_type(&self) -> EncryptionType {
        let inner = self.inner.lock();
        inner.options.as_ref().map(|opts| opts.encryption_type).unwrap_or(EncryptionType::None)
    }

    fn setup_rtp_sender(
        &self,
        participant_identity: &ParticipantIdentity,
        sender: RtpSender,
    ) -> FrameCryptor {
        let inner = self.inner.lock();
        let options = inner.options.as_ref().unwrap();

        let frame_cryptor = FrameCryptor::new_for_rtp_sender(
            LkRuntime::instance().pc_factory(),
            participant_identity.to_string(),
            EncryptionAlgorithm::AesGcm,
            options.key_provider.handle.clone(),
            sender,
        );
        frame_cryptor.set_enabled(inner.enabled);
        frame_cryptor
    }

    fn setup_rtp_receiver(
        &self,
        participant_identity: &ParticipantIdentity,
        receiver: RtpReceiver,
    ) -> FrameCryptor {
        let inner = self.inner.lock();
        let options = inner.options.as_ref().unwrap();

        let frame_cryptor = FrameCryptor::new_for_rtp_receiver(
            LkRuntime::instance().pc_factory(),
            participant_identity.to_string(),
            EncryptionAlgorithm::AesGcm,
            options.key_provider.handle.clone(),
            receiver,
        );
        frame_cryptor.set_enabled(inner.enabled);
        frame_cryptor
    }

    fn remove_frame_cryptor(&self, participant_identity: ParticipantIdentity, track_sid: TrackSid) {
        log::debug!("removing frame cryptor for {}", participant_identity);

        let mut inner = self.inner.lock();
        inner.frame_cryptors.remove(&(participant_identity, track_sid));
    }

    /// Decrypt data received from a data channel
    pub fn handle_encrypted_data(
        &self,
        data: &[u8],
        iv: &[u8],
        participant_identity: &str,
        key_index: u32,
    ) -> Option<Vec<u8>> {
        let inner = self.inner.lock();

        let data_packet_cryptor = inner.data_packet_cryptor.as_ref()?;

        let encrypted_packet = EncryptedPacket { data: data.to_vec(), iv: iv.to_vec(), key_index };

        match data_packet_cryptor.decrypt(participant_identity, &encrypted_packet) {
            Ok(decrypted_data) => Some(decrypted_data),
            Err(e) => {
                log::warn!("handle_encrypted_data error: {}", e);
                None
            }
        }
    }

    /// Encrypt data for transmission over a data channel
    pub fn encrypt_data(
        &self,
        data: &[u8],
        participant_identity: &str,
        key_index: u32,
    ) -> Result<EncryptedPacket, Box<dyn std::error::Error>> {
        let inner = self.inner.lock();

        let data_packet_cryptor =
            inner.data_packet_cryptor.as_ref().ok_or("DataPacketCryptor is not initialized")?;

        data_packet_cryptor.encrypt(participant_identity, key_index, data)
    }
}
