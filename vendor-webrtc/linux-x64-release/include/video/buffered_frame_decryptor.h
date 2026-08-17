/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_BUFFERED_FRAME_DECRYPTOR_H_
#define VIDEO_BUFFERED_FRAME_DECRYPTOR_H_

#include <deque>
#include <memory>

#include "api/crypto/crypto_options.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/field_trials_view.h"
#include "modules/rtp_rtcp/source/frame_object.h"

namespace webrtc {

// This callback is provided during the construction of the
// BufferedFrameDecryptor and is called each time a frame is sucessfully
// decrypted by the buffer.
class OnDecryptedFrameCallback {
 public:
  virtual ~OnDecryptedFrameCallback() = default;
  // Called each time a decrypted frame is returned.
  virtual void OnDecryptedFrame(std::unique_ptr<RtpFrameObject> frame) = 0;
};

// This callback is called each time there is a status change in the decryption
// stream. For example going from a none state to a first decryption or going
// frome a decryptable state to a non decryptable state.
class OnDecryptionStatusChangeCallback {
 public:
  virtual ~OnDecryptionStatusChangeCallback() = default;
  // Called each time the decryption stream status changes. This call is
  // blocking so the caller must relinquish the callback quickly. This status
  // must match what is specified in the FrameDecryptorInterface file. Notably
  // 0 must indicate success and any positive integer is a failure.
  virtual void OnDecryptionStatusChange(
      FrameDecryptorInterface::Status status) = 0;
};

// The BufferedFrameDecryptor is responsible for deciding when to pass
// decrypted received frames onto the OnDecryptedFrameCallback. Frames can be
// delayed when frame encryption is enabled but the key hasn't arrived yet. In
// this case we stash about 1 second of encrypted frames instead of dropping
// them to prevent re-requesting the key frame. This optimization is
// particularly important on low bandwidth networks. Note stashing is only ever
// done if we have never sucessfully decrypted a frame before. After the first
// successful decryption payloads will never be stashed.
class BufferedFrameDecryptor final {
 public:
  // Constructs a new BufferedFrameDecryptor that can hold
  explicit BufferedFrameDecryptor(
      OnDecryptedFrameCallback* decrypted_frame_callback,
      OnDecryptionStatusChangeCallback* decryption_status_change_callback,
      const FieldTrialsView& field_trials);

  ~BufferedFrameDecryptor();
  // This object cannot be copied.
  BufferedFrameDecryptor(const BufferedFrameDecryptor&) = delete;
  BufferedFrameDecryptor& operator=(const BufferedFrameDecryptor&) = delete;

  // Sets a new frame decryptor as the decryptor for the buffered frame
  // decryptor. This allows the decryptor to be switched out without resetting
  // the video stream.
  void SetFrameDecryptor(
      scoped_refptr<FrameDecryptorInterface> frame_decryptor);

  // Determines whether the frame should be stashed, dropped or handed off to
  // the OnDecryptedFrameCallback.
  void ManageEncryptedFrame(std::unique_ptr<RtpFrameObject> encrypted_frame);

 private:
  // Represents what should be done with a given frame.
  enum class FrameDecision { kStash, kDecrypted, kDrop };

  // Attempts to decrypt the frame, if it fails and no prior frames have been
  // decrypted it will return kStash. Otherwise fail to decrypts will return
  // kDrop. Successful decryptions will always return kDecrypted.
  FrameDecision DecryptFrame(RtpFrameObject* frame);
  // Retries all the stashed frames this is triggered each time a kDecrypted
  // event occurs.
  void RetryStashedFrames();

  static const size_t kMaxStashedFrames = 24;

  const bool generic_descriptor_auth_experiment_;
  bool first_frame_decrypted_ = false;
  FrameDecryptorInterface::Status last_status_ =
      FrameDecryptorInterface::Status::kUnknown;
  scoped_refptr<FrameDecryptorInterface> frame_decryptor_;
  OnDecryptedFrameCallback* const decrypted_frame_callback_;
  OnDecryptionStatusChangeCallback* const decryption_status_change_callback_;
  std::deque<std::unique_ptr<RtpFrameObject>> stashed_frames_;
};

}  // namespace webrtc

#endif  // VIDEO_BUFFERED_FRAME_DECRYPTOR_H_
