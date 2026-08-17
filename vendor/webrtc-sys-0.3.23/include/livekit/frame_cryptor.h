/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <stdint.h>

#include <memory>
#include <string>
#include <vector>

#include "api/crypto/frame_crypto_transformer.h"
#include "api/scoped_refptr.h"
#include "livekit/peer_connection.h"
#include "livekit/peer_connection_factory.h"
#include "livekit/rtp_receiver.h"
#include "livekit/rtp_sender.h"
#include "livekit/webrtc.h"
#include "rtc_base/synchronization/mutex.h"
#include "rust/cxx.h"

namespace livekit_ffi {

struct KeyProviderOptions;
struct EncryptedPacket;
enum class Algorithm : ::std::int32_t;
class RtcFrameCryptorObserverWrapper;
class NativeFrameCryptorObserver;

/// Shared secret key for frame encryption.
class KeyProvider {
 public:
  KeyProvider(KeyProviderOptions options);
  ~KeyProvider() {}

  bool set_shared_key(int32_t index, rust::Vec<::std::uint8_t> key) const {
    std::vector<uint8_t> key_vec;
    std::copy(key.begin(), key.end(), std::back_inserter(key_vec));
    return impl_->SetSharedKey(index, key_vec);
  }

  rust::Vec<::std::uint8_t> ratchet_shared_key(int32_t key_index) const {
    rust::Vec<uint8_t> vec;
    auto data = impl_->RatchetSharedKey(key_index);
    if (data.empty()) {
      throw std::runtime_error("ratchet_shared_key failed");
    }

    std::move(data.begin(), data.end(), std::back_inserter(vec));
    return vec;
  }

  rust::Vec<::std::uint8_t> get_shared_key(int32_t key_index) const {
    rust::Vec<uint8_t> vec;
    auto data = impl_->ExportSharedKey(key_index);
    if (data.empty()) {
      throw std::runtime_error("get_shared_key failed");
    }

    std::move(data.begin(), data.end(), std::back_inserter(vec));
    return vec;
  }

  /// Set the key at the given index.
  bool set_key(const ::rust::String participant_id,
               int32_t index,
               rust::Vec<::std::uint8_t> key) const {
    std::vector<uint8_t> key_vec;
    std::copy(key.begin(), key.end(), std::back_inserter(key_vec));
    return impl_->SetKey(
        std::string(participant_id.data(), participant_id.size()), index,
        key_vec);
  }

  rust::Vec<::std::uint8_t> ratchet_key(const ::rust::String participant_id,
                                        int32_t key_index) const {
    rust::Vec<uint8_t> vec;
    auto data = impl_->RatchetKey(
        std::string(participant_id.data(), participant_id.size()), key_index);
    if (data.empty()) {
      throw std::runtime_error("ratchet_key failed");
    }

    std::move(data.begin(), data.end(), std::back_inserter(vec));
    return vec;
  }

  rust::Vec<::std::uint8_t> get_key(const ::rust::String participant_id,
                                    int32_t key_index) const {
    rust::Vec<uint8_t> vec;
    auto data = impl_->ExportKey(
        std::string(participant_id.data(), participant_id.size()), key_index);
    if (data.empty()) {
      throw std::runtime_error("get_key failed");
    }

    std::move(data.begin(), data.end(), std::back_inserter(vec));
    return vec;
  }

  void set_sif_trailer(rust::Vec<::std::uint8_t> trailer) const {
    std::vector<uint8_t> trailer_vec;
    std::copy(trailer.begin(), trailer.end(), std::back_inserter(trailer_vec));
    impl_->SetSifTrailer(trailer_vec);
  }

  webrtc::scoped_refptr<webrtc::KeyProvider> rtc_key_provider() { return impl_; }

 private:
  webrtc::scoped_refptr<webrtc::DefaultKeyProviderImpl> impl_;
};

class FrameCryptor {
 public:
  FrameCryptor(std::shared_ptr<RtcRuntime> rtc_runtime,
               const std::string participant_id,
               webrtc::FrameCryptorTransformer::Algorithm algorithm,
               webrtc::scoped_refptr<webrtc::KeyProvider> key_provider,
               webrtc::scoped_refptr<webrtc::RtpSenderInterface> sender);

  FrameCryptor(std::shared_ptr<RtcRuntime> rtc_runtime,
               const std::string participant_id,
               webrtc::FrameCryptorTransformer::Algorithm algorithm,
               webrtc::scoped_refptr<webrtc::KeyProvider> key_provider,
               webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver);
  ~FrameCryptor();

  /// Enable/Disable frame crypto for the sender or receiver.
  void set_enabled(bool enabled) const;

  /// Get the enabled state for the sender or receiver.
  bool enabled() const;

  /// Set the key index for the sender or receiver.
  /// If the key index is not set, the key index will be set to 0.
  void set_key_index(int32_t index) const;

  /// Get the key index for the sender or receiver.
  int32_t key_index() const;

  rust::String participant_id() const { return participant_id_; }

  void register_observer(
      rust::Box<RtcFrameCryptorObserverWrapper> observer) const;

  void unregister_observer() const;

 private:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  const rust::String participant_id_;
  mutable webrtc::Mutex mutex_;
  webrtc::scoped_refptr<webrtc::FrameCryptorTransformer> e2ee_transformer_;
  webrtc::scoped_refptr<webrtc::KeyProvider> key_provider_;
  webrtc::scoped_refptr<webrtc::RtpSenderInterface> sender_;
  webrtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver_;
  mutable webrtc::scoped_refptr<NativeFrameCryptorObserver> observer_;
};

class NativeFrameCryptorObserver
    : public webrtc::FrameCryptorTransformerObserver {
 public:
  NativeFrameCryptorObserver(rust::Box<RtcFrameCryptorObserverWrapper> observer,
                             const FrameCryptor* fc);
  ~NativeFrameCryptorObserver();

  void OnFrameCryptionStateChanged(const std::string participant_id,
                                   webrtc::FrameCryptionState error) override;

 private:
  rust::Box<RtcFrameCryptorObserverWrapper> observer_;
  const FrameCryptor* fc_;
};

class DataPacketCryptor {
 public:
  DataPacketCryptor(webrtc::FrameCryptorTransformer::Algorithm algorithm,
                   webrtc::scoped_refptr<webrtc::KeyProvider> key_provider);

  EncryptedPacket encrypt_data_packet(
      const ::rust::String participant_id,
      uint32_t key_index,
      rust::Vec<::std::uint8_t> data) const;

  rust::Vec<::std::uint8_t> decrypt_data_packet(
      const ::rust::String participant_id,
      const EncryptedPacket& encrypted_packet) const;

 private:
  webrtc::scoped_refptr<webrtc::DataPacketCryptor> data_packet_cryptor_;
};

std::shared_ptr<FrameCryptor> new_frame_cryptor_for_rtp_sender(
    std::shared_ptr<PeerConnectionFactory> peer_factory,
    const ::rust::String participant_id,
    Algorithm algorithm,
    std::shared_ptr<KeyProvider> key_provider,
    std::shared_ptr<RtpSender> sender);

std::shared_ptr<FrameCryptor> new_frame_cryptor_for_rtp_receiver(
    std::shared_ptr<PeerConnectionFactory> peer_factory,
    const ::rust::String participant_id,
    Algorithm algorithm,
    std::shared_ptr<KeyProvider> key_provider,
    std::shared_ptr<RtpReceiver> receiver);

std::shared_ptr<KeyProvider> new_key_provider(KeyProviderOptions options);

std::shared_ptr<DataPacketCryptor> new_data_packet_cryptor(
    Algorithm algorithm,
    std::shared_ptr<KeyProvider> key_provider);

}  // namespace livekit_ffi
