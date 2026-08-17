/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_FRAME_DECRYPTOR_H_
#define API_TEST_MOCK_FRAME_DECRYPTOR_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "api/array_view.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/media_types.h"
#include "test/gmock.h"

namespace webrtc {

class MockFrameDecryptor : public FrameDecryptorInterface {
 public:
  MOCK_METHOD(Result,
              Decrypt,
              (webrtc::MediaType,
               const std::vector<uint32_t>&,
               webrtc::ArrayView<const uint8_t>,
               webrtc::ArrayView<const uint8_t>,
               webrtc::ArrayView<uint8_t>),
              (override));

  MOCK_METHOD(size_t,
              GetMaxPlaintextByteSize,
              (webrtc::MediaType, size_t encrypted_frame_size),
              (override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_FRAME_DECRYPTOR_H_
