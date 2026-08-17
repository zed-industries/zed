/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_DEVICE_MOCK_AUDIO_DEVICE_BUFFER_H_
#define MODULES_AUDIO_DEVICE_MOCK_AUDIO_DEVICE_BUFFER_H_

#include <optional>

#include "modules/audio_device/audio_device_buffer.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioDeviceBuffer : public AudioDeviceBuffer {
 public:
  using AudioDeviceBuffer::AudioDeviceBuffer;
  virtual ~MockAudioDeviceBuffer() {}
  MOCK_METHOD(int32_t, RequestPlayoutData, (size_t nSamples), (override));
  MOCK_METHOD(int32_t, GetPlayoutData, (void* audioBuffer), (override));
  MOCK_METHOD(int32_t,
              SetRecordedBuffer,
              (const void* audioBuffer,
               size_t nSamples,
               std::optional<int64_t> capture_time_ns),
              (override));
  MOCK_METHOD(void, SetVQEData, (int playDelayMS, int recDelayMS), (override));
  MOCK_METHOD(int32_t, DeliverRecordedData, (), (override));
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_DEVICE_MOCK_AUDIO_DEVICE_BUFFER_H_
