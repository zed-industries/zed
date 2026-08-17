/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_ECHO_CONTROL_MOCK_H_
#define MODULES_AUDIO_PROCESSING_TEST_ECHO_CONTROL_MOCK_H_

#include "api/audio/echo_control.h"
#include "test/gmock.h"

namespace webrtc {

class AudioBuffer;

class MockEchoControl : public EchoControl {
 public:
  MOCK_METHOD(void, AnalyzeRender, (AudioBuffer * render), (override));
  MOCK_METHOD(void, AnalyzeCapture, (AudioBuffer * capture), (override));
  MOCK_METHOD(void,
              ProcessCapture,
              (AudioBuffer * capture, bool echo_path_change),
              (override));
  MOCK_METHOD(void,
              ProcessCapture,
              (AudioBuffer * capture,
               AudioBuffer* linear_output,
               bool echo_path_change),
              (override));
  MOCK_METHOD(EchoControl::Metrics, GetMetrics, (), (const, override));
  MOCK_METHOD(void, SetAudioBufferDelay, (int delay_ms), (override));
  MOCK_METHOD(void,
              SetCaptureOutputUsage,
              (bool capture_output_used),
              (override));
  MOCK_METHOD(bool, ActiveProcessing, (), (const, override));
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_ECHO_CONTROL_MOCK_H_
