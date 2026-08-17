/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_FAKE_FRAME_ENCRYPTOR_H_
#define API_TEST_FAKE_FRAME_ENCRYPTOR_H_

#include <stddef.h>
#include <stdint.h>

#include "api/array_view.h"
#include "api/crypto/frame_encryptor_interface.h"
#include "api/media_types.h"
#include "rtc_base/ref_counted_object.h"

namespace webrtc {

// The FakeFrameEncryptor is a TEST ONLY fake implementation of the
// FrameEncryptorInterface. It is constructed with a simple single digit key and
// a fixed postfix byte. This is just to validate that the core code works
// as expected.
class FakeFrameEncryptor : public RefCountedObject<FrameEncryptorInterface> {
 public:
  // Provide a key (0,255) and some postfix byte (0,255).
  explicit FakeFrameEncryptor(uint8_t fake_key = 0xAA,
                              uint8_t postfix_byte = 255);
  // Simply xors each payload with the provided fake key and adds the postfix
  // bit to the end. This will always fail if fail_encryption_ is set to true.
  int Encrypt(webrtc::MediaType media_type,
              uint32_t ssrc,
              ArrayView<const uint8_t> additional_data,
              ArrayView<const uint8_t> frame,
              ArrayView<uint8_t> encrypted_frame,
              size_t* bytes_written) override;
  // Always returns 1 more than the size of the frame.
  size_t GetMaxCiphertextByteSize(webrtc::MediaType media_type,
                                  size_t frame_size) override;
  // Sets the fake key to use during encryption.
  void SetFakeKey(uint8_t fake_key);
  // Returns the fake key used during encryption.
  uint8_t GetFakeKey() const;
  // Set the postfix byte to use.
  void SetPostfixByte(uint8_t expected_postfix_byte);
  // Return a postfix byte added to each outgoing payload.
  uint8_t GetPostfixByte() const;
  // Force all encryptions to fail.
  void SetFailEncryption(bool fail_encryption);

  enum class FakeEncryptionStatus : int {
    OK = 0,
    FORCED_FAILURE = 1,
  };

 private:
  uint8_t fake_key_ = 0;
  uint8_t postfix_byte_ = 0;
  bool fail_encryption_ = false;
};

}  // namespace webrtc

#endif  // API_TEST_FAKE_FRAME_ENCRYPTOR_H_
