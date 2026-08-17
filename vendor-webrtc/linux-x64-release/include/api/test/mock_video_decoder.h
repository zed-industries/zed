/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_VIDEO_DECODER_H_
#define API_TEST_MOCK_VIDEO_DECODER_H_

#include <cstdint>
#include <optional>

#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video_codecs/video_decoder.h"
#include "test/gmock.h"

namespace webrtc {

using testing::_;
using testing::Invoke;

class MockDecodedImageCallback : public DecodedImageCallback {
 public:
  MOCK_METHOD(int32_t,
              Decoded,
              (VideoFrame & decoded_image),  // NOLINT
              (override));
  MOCK_METHOD(int32_t,
              Decoded,
              (VideoFrame & decoded_image,  // NOLINT
               int64_t decode_time_ms),
              (override));
  MOCK_METHOD(void,
              Decoded,
              (VideoFrame & decoded_image,  // NOLINT
               std::optional<int32_t> decode_time_ms,
               std::optional<uint8_t> qp),
              (override));
};

class MockVideoDecoder : public VideoDecoder {
 public:
  MockVideoDecoder() {
    // Make `Configure` succeed by default, so that individual tests that
    // verify other methods wouldn't need to stub `Configure`.
    ON_CALL(*this, Configure).WillByDefault(testing::Return(true));

    // TODO(bugs.webrtc.org/15444): Remove once all tests have been migrated to
    // expecting calls Decode without a missing_frames param.
    ON_CALL(*this, Decode(_, _))
        .WillByDefault(Invoke([this](const EncodedImage& input_image,
                                     int64_t render_time_ms) {
          return Decode(input_image, /*missing_frames=*/false, render_time_ms);
        }));
  }

  ~MockVideoDecoder() override { Destruct(); }

  MOCK_METHOD(bool, Configure, (const Settings& settings), (override));
  MOCK_METHOD(int32_t,
              Decode,
              (const EncodedImage& input_image, int64_t render_time_ms),
              (override));
  MOCK_METHOD(int32_t,
              Decode,
              (const EncodedImage& input_image,
               bool missing_frames,
               int64_t render_time_ms));
  MOCK_METHOD(int32_t,
              RegisterDecodeCompleteCallback,
              (DecodedImageCallback * callback),
              (override));
  MOCK_METHOD(int32_t, Release, (), (override));

  // Special utility method that allows a test to monitor/verify when
  // destruction of the decoder instance occurs.
  MOCK_METHOD(void, Destruct, (), ());
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_VIDEO_DECODER_H_
