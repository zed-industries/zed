/*
 * Copyright 2022 LiveKit
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

#ifndef WEBRTC_FRAME_CRYPTOR_TRANSFORMER_H_
#define WEBRTC_FRAME_CRYPTOR_TRANSFORMER_H_

#include <unordered_map>

#include "api/frame_transformer_interface.h"
#include "api/make_ref_counted.h"
#include "api/rtc_error.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/buffer.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread.h"

namespace webrtc {

const size_t DEFAULT_KEYRING_SIZE = 16;
const size_t MAX_KEYRING_SIZE = 255;

class ParticipantKeyHandler;

enum KeyDerivationAlgorithm {
  kPBKDF2 = 0,
  kHKDF,
};

struct KeyProviderOptions {
  bool shared_key;
  std::vector<uint8_t> ratchet_salt;
  std::vector<uint8_t> uncrypted_magic_bytes;
  int ratchet_window_size;
  int failure_tolerance;
  // key ring size should be between 1 and 255
  int key_ring_size;
  bool discard_frame_when_cryptor_not_ready;
  KeyDerivationAlgorithm key_derivation_algorithm;
  KeyProviderOptions()
      : shared_key(false),
        ratchet_window_size(0),
        failure_tolerance(-1),
        key_ring_size(DEFAULT_KEYRING_SIZE),
        discard_frame_when_cryptor_not_ready(false),
        key_derivation_algorithm(kPBKDF2) {}
  KeyProviderOptions(KeyProviderOptions& copy)
      : shared_key(copy.shared_key),
        ratchet_salt(copy.ratchet_salt),
        uncrypted_magic_bytes(copy.uncrypted_magic_bytes),
        ratchet_window_size(copy.ratchet_window_size),
        failure_tolerance(copy.failure_tolerance),
        key_ring_size(copy.key_ring_size),
        key_derivation_algorithm(copy.key_derivation_algorithm) {}
};

class KeyProvider : public webrtc::RefCountInterface {
 public:
  virtual bool SetSharedKey(int key_index, std::vector<uint8_t> key) = 0;

  virtual const webrtc::scoped_refptr<ParticipantKeyHandler> GetSharedKey(
      const std::string participant_id) = 0;

  virtual const std::vector<uint8_t> RatchetSharedKey(int key_index) = 0;

  virtual const std::vector<uint8_t> ExportSharedKey(int key_index) const = 0;

  virtual bool SetKey(const std::string participant_id,
                      int key_index,
                      std::vector<uint8_t> key) = 0;

  virtual const webrtc::scoped_refptr<ParticipantKeyHandler> GetKey(
      const std::string participant_id) const = 0;

  virtual const std::vector<uint8_t> RatchetKey(
      const std::string participant_id,
      int key_index) = 0;

  virtual const std::vector<uint8_t> ExportKey(const std::string participant_id,
                                               int key_index) const = 0;

  virtual void SetSifTrailer(const std::vector<uint8_t> trailer) = 0;

  virtual KeyProviderOptions& options() = 0;

 protected:
  virtual ~KeyProvider() {}
};

class ParticipantKeyHandler : public webrtc::RefCountInterface {
 public:
  struct KeySet : public webrtc::RefCountInterface {
    std::vector<uint8_t> material;
    std::vector<uint8_t> encryption_key;
    KeySet(std::vector<uint8_t> material, std::vector<uint8_t> encryptionKey)
        : material(material), encryption_key(encryptionKey) {}
  };

 public:
  ParticipantKeyHandler(KeyProvider* key_provider)
      : key_provider_(key_provider) {
    int key_ring_size = key_provider_->options().key_ring_size;
    if (key_ring_size <= 0) {
      key_ring_size = DEFAULT_KEYRING_SIZE;
    } else if (key_ring_size > (int)MAX_KEYRING_SIZE) {
      // Keyring size needs to be between 1 and 256
      key_ring_size = MAX_KEYRING_SIZE;
    }
    crypto_key_ring_.resize(key_ring_size);
  }

  virtual ~ParticipantKeyHandler() = default;

  webrtc::scoped_refptr<ParticipantKeyHandler> Clone() {
    auto clone = webrtc::make_ref_counted<ParticipantKeyHandler>(key_provider_);
    clone->crypto_key_ring_ = crypto_key_ring_;
    clone->current_key_index_ = current_key_index_;
    clone->has_valid_key_ = has_valid_key_;
    return clone;
  }

  virtual std::vector<uint8_t> RatchetKey(int key_index) {
    auto key_set = GetKeySet(key_index);
    if (!key_set) {
      return std::vector<uint8_t>();
    }
    auto current_material = key_set->material;
    std::vector<uint8_t> new_material;
    if (DoKeyDerivation(current_material,
                                  key_provider_->options().ratchet_salt, 256,
                                  new_material) != 0) {
      return std::vector<uint8_t>();
    }
    SetKeyFromMaterial(new_material,
                       key_index != -1 ? key_index : current_key_index_);
    SetHasValidKey();
    return new_material;
  }

  virtual webrtc::scoped_refptr<KeySet> GetKeySet(int key_index) {
    webrtc::MutexLock lock(&mutex_);
    return crypto_key_ring_[key_index != -1 ? key_index : current_key_index_];
  }

  virtual void SetKey(std::vector<uint8_t> password, int key_index) {
    SetKeyFromMaterial(password, key_index);
    SetHasValidKey();
  }

  std::vector<uint8_t> RatchetKeyMaterial(
      std::vector<uint8_t> current_material) {
    std::vector<uint8_t> new_material;
    if (DoKeyDerivation(current_material,
                                  key_provider_->options().ratchet_salt, 256,
                                  new_material) != 0) {
      return std::vector<uint8_t>();
    }
    return new_material;
  }

  webrtc::scoped_refptr<KeySet> DeriveKeys(std::vector<uint8_t> password,
                                           std::vector<uint8_t> ratchet_salt,
                                           unsigned int optional_length_bits) {
    std::vector<uint8_t> derived_key;
    if (DoKeyDerivation(password, ratchet_salt, optional_length_bits,
                                  derived_key) == 0) {
      return webrtc::make_ref_counted<KeySet>(password, derived_key);
    }
    return nullptr;
  }

  bool HasValidKey() {
    webrtc::MutexLock lock(&mutex_);
    return has_valid_key_;
  }

  void SetHasValidKey() {
    webrtc::MutexLock lock(&mutex_);
    decryption_failure_count_ = 0;
    has_valid_key_ = true;
  }

  void SetKeyFromMaterial(std::vector<uint8_t> password, int key_index) {
    webrtc::MutexLock lock(&mutex_);
    if (key_index >= 0) {
      current_key_index_ = key_index % crypto_key_ring_.size();
    }
    crypto_key_ring_[current_key_index_] =
        DeriveKeys(password, key_provider_->options().ratchet_salt, 128);
  }

  bool DecryptionFailure() {
    webrtc::MutexLock lock(&mutex_);
    if (key_provider_->options().failure_tolerance < 0) {
      return false;
    }
    decryption_failure_count_ += 1;

    if (decryption_failure_count_ >
        key_provider_->options().failure_tolerance) {
      has_valid_key_ = false;
      return true;
    }
    return false;
  }

private:
  int DoKeyDerivation(const std::vector<uint8_t>& secret,
                      const std::vector<uint8_t>& salt,
                      unsigned int optional_length_bits,
                      std::vector<uint8_t>& derived_key);
 private:
  bool has_valid_key_ = false;
  int decryption_failure_count_ = 0;
  mutable webrtc::Mutex mutex_;
  int current_key_index_ = 0;
  KeyProvider* key_provider_;
  std::vector<rtc::scoped_refptr<KeySet>> crypto_key_ring_;
};

class DefaultKeyProviderImpl : public KeyProvider {
 public:
  DefaultKeyProviderImpl(KeyProviderOptions options) : options_(options) {}
  ~DefaultKeyProviderImpl() override = default;

  /// Set the shared key.
  bool SetSharedKey(int key_index, std::vector<uint8_t> key) override {
    webrtc::MutexLock lock(&mutex_);
    if (options_.shared_key) {
      if (keys_.find("shared") == keys_.end()) {
        keys_["shared"] = webrtc::make_ref_counted<ParticipantKeyHandler>(this);
      }

      auto key_handler = keys_["shared"];
      key_handler->SetKey(key, key_index);

      for (auto& key_pair : keys_) {
        if (key_pair.first != "shared") {
          key_pair.second->SetKey(key, key_index);
        }
      }
      return true;
    }
    return false;
  }

  const std::vector<uint8_t> RatchetSharedKey(int key_index) override {
    webrtc::MutexLock lock(&mutex_);
    auto it = keys_.find("shared");
    if (it == keys_.end()) {
      return std::vector<uint8_t>();
    }
    auto new_key = it->second->RatchetKey(key_index);
    if (options_.shared_key) {
      for (auto& key_pair : keys_) {
        if (key_pair.first != "shared") {
          key_pair.second->SetKey(new_key, key_index);
        }
      }
    }
    return new_key;
  }

  const std::vector<uint8_t> ExportSharedKey(int key_index) const override {
    webrtc::MutexLock lock(&mutex_);
    auto it = keys_.find("shared");
    if (it == keys_.end()) {
      return std::vector<uint8_t>();
    }
    auto key_set = it->second->GetKeySet(key_index);
    if (key_set) {
      return key_set->material;
    }
    return std::vector<uint8_t>();
  }

  const webrtc::scoped_refptr<ParticipantKeyHandler> GetSharedKey(
      const std::string participant_id) override {
    webrtc::MutexLock lock(&mutex_);
    if (options_.shared_key && keys_.find("shared") != keys_.end()) {
      auto shared_key_handler = keys_["shared"];
      if (keys_.find(participant_id) != keys_.end()) {
        return keys_[participant_id];
      } else {
        auto key_handler_clone = shared_key_handler->Clone();
        keys_[participant_id] = key_handler_clone;
        return key_handler_clone;
      }
    }
    return nullptr;
  }

  /// Set the key at the given index.
  bool SetKey(const std::string participant_id,
              int index,
              std::vector<uint8_t> key) override {
    webrtc::MutexLock lock(&mutex_);

    if (keys_.find(participant_id) == keys_.end()) {
      keys_[participant_id] =
          webrtc::make_ref_counted<ParticipantKeyHandler>(this);
    }

    auto key_handler = keys_[participant_id];
    key_handler->SetKey(key, index);
    return true;
  }

  const webrtc::scoped_refptr<ParticipantKeyHandler> GetKey(
      const std::string participant_id) const override {
    webrtc::MutexLock lock(&mutex_);

    if (keys_.find(participant_id) == keys_.end()) {
      return nullptr;
    }

    return keys_.find(participant_id)->second;
  }

  const std::vector<uint8_t> RatchetKey(const std::string participant_id,
                                        int key_index) override {
    auto key_handler = GetKey(participant_id);
    if (key_handler) {
      return key_handler->RatchetKey(key_index);
    }
    return std::vector<uint8_t>();
  }

  const std::vector<uint8_t> ExportKey(const std::string participant_id,
                                       int key_index) const override {
    auto key_handler = GetKey(participant_id);
    if (key_handler) {
      auto key_set = key_handler->GetKeySet(key_index);
      if (key_set) {
        return key_set->material;
      }
    }
    return std::vector<uint8_t>();
  }

  void SetSifTrailer(const std::vector<uint8_t> trailer) override {
    webrtc::MutexLock lock(&mutex_);
    options_.uncrypted_magic_bytes = trailer;
  }

  KeyProviderOptions& options() override { return options_; }

 private:
  mutable webrtc::Mutex mutex_;
  KeyProviderOptions options_;
  std::unordered_map<std::string, webrtc::scoped_refptr<ParticipantKeyHandler>>
      keys_;
};

enum FrameCryptionState {
  kNew = 0,
  kOk,
  kEncryptionFailed,
  kDecryptionFailed,
  kMissingKey,
  kKeyRatcheted,
  kInternalError,
};

class FrameCryptorTransformerObserver : public webrtc::RefCountInterface {
 public:
  virtual void OnFrameCryptionStateChanged(const std::string participant_id,
                                           FrameCryptionState error) = 0;

 protected:
  virtual ~FrameCryptorTransformerObserver() {}
};

class RTC_EXPORT FrameCryptorTransformer
    : public webrtc::RefCountedObject<webrtc::FrameTransformerInterface> {
 public:
  enum class MediaType {
    kAudioFrame = 0,
    kVideoFrame,
  };

  enum class Algorithm {
    kAesGcm = 0,
    kAesCbc,
  };

  explicit FrameCryptorTransformer(
      rtc::Thread* signaling_thread,
      const std::string participant_id,
      MediaType type,
      Algorithm algorithm,
      webrtc::scoped_refptr<KeyProvider> key_provider);
  ~FrameCryptorTransformer();
  virtual void RegisterFrameCryptorTransformerObserver(
      webrtc::scoped_refptr<FrameCryptorTransformerObserver> observer) {
    webrtc::MutexLock lock(&mutex_);
    observer_ = observer;
  }

  virtual void UnRegisterFrameCryptorTransformerObserver() {
    webrtc::MutexLock lock(&mutex_);
    observer_ = nullptr;
  }

  virtual void SetKeyIndex(int index) {
    webrtc::MutexLock lock(&mutex_);
    key_index_ = index;
  }

  virtual int key_index() const { return key_index_; }

  virtual void SetEnabled(bool enabled) {
    webrtc::MutexLock lock(&mutex_);
    enabled_cryption_ = enabled;
  }
  virtual bool enabled() const {
    webrtc::MutexLock lock(&mutex_);
    return enabled_cryption_;
  }
  virtual const std::string participant_id() const { return participant_id_; }

 protected:
  virtual void RegisterTransformedFrameCallback(
      webrtc::scoped_refptr<webrtc::TransformedFrameCallback> callback)
      override {
    webrtc::MutexLock lock(&sink_mutex_);
    sink_callback_ = callback;
  }
  virtual void UnregisterTransformedFrameCallback() override {
    webrtc::MutexLock lock(&sink_mutex_);
    sink_callback_ = nullptr;
  }
  virtual void RegisterTransformedFrameSinkCallback(
      webrtc::scoped_refptr<webrtc::TransformedFrameCallback> callback,
      uint32_t ssrc) override {
    webrtc::MutexLock lock(&sink_mutex_);
    sink_callbacks_[ssrc] = callback;
  }
  virtual void UnregisterTransformedFrameSinkCallback(uint32_t ssrc) override {
    webrtc::MutexLock lock(&sink_mutex_);
    auto it = sink_callbacks_.find(ssrc);
    if (it != sink_callbacks_.end()) {
      sink_callbacks_.erase(it);
    }
  }

  virtual void Transform(
      std::unique_ptr<webrtc::TransformableFrameInterface> frame) override;

 private:
  void encryptFrame(std::unique_ptr<webrtc::TransformableFrameInterface> frame);
  void decryptFrame(std::unique_ptr<webrtc::TransformableFrameInterface> frame);
  void onFrameCryptionStateChanged(FrameCryptionState error);
  rtc::Buffer makeIv(uint32_t ssrc, uint32_t timestamp);
  uint8_t getIvSize();

 private:
  TaskQueueBase* const signaling_thread_;
  std::unique_ptr<rtc::Thread> thread_;
  std::string participant_id_;
  mutable webrtc::Mutex mutex_;
  mutable webrtc::Mutex sink_mutex_;
  bool enabled_cryption_ RTC_GUARDED_BY(mutex_) = false;
  MediaType type_;
  Algorithm algorithm_;
  webrtc::scoped_refptr<webrtc::TransformedFrameCallback> sink_callback_;
  std::map<uint32_t, webrtc::scoped_refptr<webrtc::TransformedFrameCallback>>
      sink_callbacks_;
  int key_index_ = 0;
  std::map<uint32_t, uint32_t> send_counts_;
  webrtc::scoped_refptr<KeyProvider> key_provider_;
  webrtc::scoped_refptr<FrameCryptorTransformerObserver> observer_;
  FrameCryptionState last_enc_error_ = FrameCryptionState::kNew;
  FrameCryptionState last_dec_error_ = FrameCryptionState::kNew;
};

class RTC_EXPORT EncryptedPacket : public webrtc::RefCountInterface {
 public:
  EncryptedPacket() = default;
  EncryptedPacket(std::vector<uint8_t> data,
                  std::vector<uint8_t> iv,
                  uint8_t key_index)
      : data(data), iv(iv), key_index(key_index) {}
  ~EncryptedPacket() = default;

  std::vector<uint8_t> data;
  std::vector<uint8_t> iv;
  uint8_t key_index = 0;
};

class RTC_EXPORT DataPacketCryptor : public webrtc::RefCountInterface {
 public:
  DataPacketCryptor(FrameCryptorTransformer::Algorithm algorithm,
                    webrtc::scoped_refptr<KeyProvider> key_provider);
  ~DataPacketCryptor();

  virtual RTCErrorOr<webrtc::scoped_refptr<EncryptedPacket>> Encrypt(
      const std::string participant_id,
      int key_index,
      const std::vector<uint8_t>& data);

  virtual RTCErrorOr<std::vector<uint8_t>> Decrypt(
      const std::string participant_id,
      const webrtc::scoped_refptr<EncryptedPacket> encryptedPacket);

 private:
  rtc::Buffer makeIv(uint32_t timestamp);

 private:
  FrameCryptorTransformer::Algorithm algorithm_;
  webrtc::scoped_refptr<KeyProvider> key_provider_;
  uint32_t send_count_ = 0;
  mutable webrtc::Mutex mutex_;
};

}  // namespace webrtc

#endif  // WEBRTC_FRAME_CRYPTOR_TRANSFORMER_H_
