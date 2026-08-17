/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_COPY_TO_FILE_AUDIO_CAPTURER_H_
#define TEST_TESTSUPPORT_COPY_TO_FILE_AUDIO_CAPTURER_H_

#include <memory>
#include <optional>
#include <string>

#include "api/array_view.h"
#include "common_audio/wav_file.h"
#include "modules/audio_device/include/test_audio_device.h"
#include "rtc_base/buffer.h"

namespace webrtc {
namespace test {

// TestAudioDeviceModule::Capturer that will store audio data, captured by
// delegate to the specified output file. Can be used to create a copy of
// generated audio data to be able then to compare it as a reference with
// audio on the TestAudioDeviceModule::Renderer side.
class CopyToFileAudioCapturer : public TestAudioDeviceModule::Capturer {
 public:
  CopyToFileAudioCapturer(
      std::unique_ptr<TestAudioDeviceModule::Capturer> delegate,
      std::string stream_dump_file_name);
  ~CopyToFileAudioCapturer() override;

  int SamplingFrequency() const override;
  int NumChannels() const override;
  bool Capture(BufferT<int16_t>* buffer) override;

 private:
  std::unique_ptr<TestAudioDeviceModule::Capturer> delegate_;
  std::unique_ptr<WavWriter> wav_writer_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_COPY_TO_FILE_AUDIO_CAPTURER_H_
