// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_PLATFORM_SUPPORT_WITH_MOCK_AUDIO_CAPTURE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_PLATFORM_SUPPORT_WITH_MOCK_AUDIO_CAPTURE_SOURCE_H_

#include <string>

#include "base/memory/scoped_refptr.h"
#include "media/base/audio_capturer_source.h"
#include "media/base/audio_parameters.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/public/web/web_local_frame.h"
#include "third_party/blink/renderer/platform/testing/io_task_runner_testing_platform_support.h"

namespace blink {

class MockAudioCapturerSource : public media::AudioCapturerSource {
 public:
  MockAudioCapturerSource() = default;
  MOCK_METHOD2(Initialize,
               void(const media::AudioParameters& params,
                    CaptureCallback* callback));
  MOCK_METHOD0(Start, void());
  MOCK_METHOD0(Stop, void());
  MOCK_METHOD1(SetAutomaticGainControl, void(bool enable));
  void SetVolume(double volume) override {}
  void SetOutputDeviceForAec(const std::string& output_device_id) override {}

 private:
  ~MockAudioCapturerSource() override = default;
};

// Test Platform implementation that overrides the known methods needed
// by the tests, including creation of AudioCapturerSource instances.
class AudioCapturerSourceTestingPlatformSupport
    : public IOTaskRunnerTestingPlatformSupport {
 public:
  AudioCapturerSourceTestingPlatformSupport() = default;

  scoped_refptr<media::AudioCapturerSource> NewAudioCapturerSource(
      WebLocalFrame* web_frame,
      const media::AudioSourceParameters& params) override;
  MockAudioCapturerSource* mock_audio_capturer_source() {
    return mock_audio_capturer_source_.get();
  }

 private:
  scoped_refptr<MockAudioCapturerSource> mock_audio_capturer_source_ =
      base::MakeRefCounted<MockAudioCapturerSource>();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TESTING_PLATFORM_SUPPORT_WITH_MOCK_AUDIO_CAPTURE_SOURCE_H_
