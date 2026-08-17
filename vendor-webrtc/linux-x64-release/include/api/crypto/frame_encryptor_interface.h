/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_CRYPTO_FRAME_ENCRYPTOR_INTERFACE_H_
#define API_CRYPTO_FRAME_ENCRYPTOR_INTERFACE_H_

#include <cstddef>
#include <cstdint>

#include "api/array_view.h"
#include "api/media_types.h"
#include "api/ref_count.h"

namespace webrtc {

// FrameEncryptorInterface allows users to provide a custom encryption
// implementation to encrypt all outgoing audio and video frames. The user must
// also provide a FrameDecryptorInterface to be able to decrypt the frames on
// the receiving device. Note this is an additional layer of encryption in
// addition to the standard SRTP mechanism and is not intended to be used
// without it. Implementations of this interface will have the same lifetime as
// the RTPSenders it is attached to. Additional data may be null.
class FrameEncryptorInterface : public RefCountInterface {
 public:
  ~FrameEncryptorInterface() override {}

  // Attempts to encrypt the provided frame. You may assume the encrypted_frame
  // will match the size returned by GetMaxCiphertextByteSize for a give frame.
  // You may assume that the frames will arrive in order if SRTP is enabled.
  // The ssrc will simply identify which stream the frame is travelling on. You
  // must set bytes_written to the number of bytes you wrote in the
  // encrypted_frame. 0 must be returned if successful all other numbers can be
  // selected by the implementer to represent error codes.
  virtual int Encrypt(webrtc::MediaType media_type,
                      uint32_t ssrc,
                      ArrayView<const uint8_t> additional_data,
                      ArrayView<const uint8_t> frame,
                      ArrayView<uint8_t> encrypted_frame,
                      size_t* bytes_written) = 0;

  // Returns the total required length in bytes for the output of the
  // encryption. This can be larger than the actual number of bytes you need but
  // must never be smaller as it informs the size of the encrypted_frame buffer.
  virtual size_t GetMaxCiphertextByteSize(webrtc::MediaType media_type,
                                          size_t frame_size) = 0;
};

}  // namespace webrtc

#endif  // API_CRYPTO_FRAME_ENCRYPTOR_INTERFACE_H_
