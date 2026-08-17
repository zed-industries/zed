/*
 *  Copyright (c) 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_TEST_MOCK_AUDIO_RECEIVE_STREAM_H_
#define CALL_TEST_MOCK_AUDIO_RECEIVE_STREAM_H_

#include <cstdint>
#include <map>
#include <vector>

#include "api/audio/audio_frame.h"
#include "api/audio/audio_mixer.h"
#include "api/audio_codecs/audio_format.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/rtp_headers.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"
#include "call/audio_receive_stream.h"
#include "test/gmock.h"

namespace webrtc {
namespace test {

class MockAudioReceiveStream : public AudioReceiveStreamInterface,
                               public AudioMixer::Source {
 public:
  MOCK_METHOD(uint32_t, remote_ssrc, (), (const override));
  MOCK_METHOD(void, Start, (), (override));
  MOCK_METHOD(void, Stop, (), (override));
  MOCK_METHOD(bool, IsRunning, (), (const override));
  MOCK_METHOD(void,
              SetDepacketizerToDecoderFrameTransformer,
              (scoped_refptr<FrameTransformerInterface>),
              (override));
  MOCK_METHOD(void,
              SetDecoderMap,
              ((std::map<int, SdpAudioFormat>)),
              (override));
  MOCK_METHOD(void, SetNackHistory, (int), (override));
  MOCK_METHOD(void, SetRtcpMode, (RtcpMode), (override));
  MOCK_METHOD(void, SetNonSenderRttMeasurement, (bool), (override));
  MOCK_METHOD(void,
              SetFrameDecryptor,
              (scoped_refptr<FrameDecryptorInterface>),
              (override));

  MOCK_METHOD(webrtc::AudioReceiveStreamInterface::Stats,
              GetStats,
              (bool),
              (const override));
  MOCK_METHOD(void, SetSink, (webrtc::AudioSinkInterface*), (override));
  MOCK_METHOD(void, SetGain, (float), (override));
  MOCK_METHOD(bool, SetBaseMinimumPlayoutDelayMs, (int), (override));
  MOCK_METHOD(int, GetBaseMinimumPlayoutDelayMs, (), (const override));
  MOCK_METHOD(std::vector<webrtc::RtpSource>, GetSources, (), (const override));

  // TODO (b/397376626): Create a MockAudioMixerSource, and instead
  // have a member variable here.
  AudioMixer::Source* source() override { return this; }

  MOCK_METHOD(AudioFrameInfo,
              GetAudioFrameWithInfo,
              (int, AudioFrame*),
              (override));
  MOCK_METHOD(int, Ssrc, (), (const override));
  MOCK_METHOD(int, PreferredSampleRate, (), (const override));
};

}  // namespace test
}  // namespace webrtc

#endif  // CALL_TEST_MOCK_AUDIO_RECEIVE_STREAM_H_
