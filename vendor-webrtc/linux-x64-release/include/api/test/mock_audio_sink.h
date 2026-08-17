/*
 *  Copyright 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_AUDIO_SINK_H_
#define API_TEST_MOCK_AUDIO_SINK_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "api/media_stream_interface.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioSink : public webrtc::AudioTrackSinkInterface {
 public:
  MOCK_METHOD(void,
              OnData,
              (const void* audio_data,
               int bits_per_sample,
               int sample_rate,
               size_t number_of_channels,
               size_t number_of_frames),
              (override));

  MOCK_METHOD(void,
              OnData,
              (const void* audio_data,
               int bits_per_sample,
               int sample_rate,
               size_t number_of_channels,
               size_t number_of_frames,
               std::optional<int64_t> absolute_capture_timestamp_ms),
              (override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_AUDIO_SINK_H_
