/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_MOCK_AUDIO_DECODER_H_
#define TEST_MOCK_AUDIO_DECODER_H_

#include "api/audio_codecs/audio_decoder.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioDecoder : public AudioDecoder {
 public:
  MockAudioDecoder();
  ~MockAudioDecoder();
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(int,
              DecodeInternal,
              (const uint8_t*, size_t, int, int16_t*, SpeechType*),
              (override));
  MOCK_METHOD(bool, HasDecodePlc, (), (const, override));
  MOCK_METHOD(size_t, DecodePlc, (size_t, int16_t*), (override));
  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(int, ErrorCode, (), (override));
  MOCK_METHOD(int, PacketDuration, (const uint8_t*, size_t), (const, override));
  MOCK_METHOD(size_t, Channels, (), (const, override));
  MOCK_METHOD(int, SampleRateHz, (), (const, override));
};

}  // namespace webrtc
#endif  // TEST_MOCK_AUDIO_DECODER_H_
