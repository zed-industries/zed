/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_TEST_MOCK_AUDIO_SEND_STREAM_H_
#define CALL_TEST_MOCK_AUDIO_SEND_STREAM_H_

#include <memory>

#include "api/audio/audio_frame.h"
#include "api/rtp_sender_interface.h"
#include "call/audio_send_stream.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {

class MockAudioSendStream : public AudioSendStream {
 public:
  MOCK_METHOD(const webrtc::AudioSendStream::Config&,
              GetConfig,
              (),
              (const, override));
  MOCK_METHOD(void,
              Reconfigure,
              (const Config& config, SetParametersCallback callback),
              (override));
  MOCK_METHOD(void, Start, (), (override));
  MOCK_METHOD(void, Stop, (), (override));
  // GMock doesn't like move-only types, such as std::unique_ptr.
  void SendAudioData(std::unique_ptr<webrtc::AudioFrame> audio_frame) override {
    SendAudioDataForMock(audio_frame.get());
  }
  MOCK_METHOD(void, SendAudioDataForMock, (webrtc::AudioFrame*));
  MOCK_METHOD(
      bool,
      SendTelephoneEvent,
      (int payload_type, int payload_frequency, int event, int duration_ms),
      (override));
  MOCK_METHOD(void, SetMuted, (bool muted), (override));
  MOCK_METHOD(Stats, GetStats, (), (const, override));
  MOCK_METHOD(Stats, GetStats, (bool has_remote_tracks), (const, override));
};
}  // namespace test
}  // namespace webrtc

#endif  // CALL_TEST_MOCK_AUDIO_SEND_STREAM_H_
