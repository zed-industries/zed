/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DECODER_DATABASE_H_
#define MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DECODER_DATABASE_H_

#include <string>

#include "api/environment/environment_factory.h"
#include "modules/audio_coding/neteq/decoder_database.h"
#include "test/gmock.h"

namespace webrtc {

class MockDecoderDatabase : public DecoderDatabase {
 public:
  MockDecoderDatabase()
      : DecoderDatabase(CreateEnvironment(),
                        /*decoder_factory=*/nullptr,
                        /*codec_pair_id=*/std::nullopt) {}
  ~MockDecoderDatabase() override { Die(); }
  MOCK_METHOD(void, Die, ());
  MOCK_METHOD(bool, Empty, (), (const, override));
  MOCK_METHOD(int, Size, (), (const, override));
  MOCK_METHOD(int,
              RegisterPayload,
              (int rtp_payload_type, const SdpAudioFormat& audio_format),
              (override));
  MOCK_METHOD(int, Remove, (uint8_t rtp_payload_type), (override));
  MOCK_METHOD(void, RemoveAll, (), (override));
  MOCK_METHOD(const DecoderInfo*,
              GetDecoderInfo,
              (uint8_t rtp_payload_type),
              (const, override));
  MOCK_METHOD(int,
              SetActiveDecoder,
              (uint8_t rtp_payload_type, bool* new_decoder),
              (override));
  MOCK_METHOD(AudioDecoder*, GetActiveDecoder, (), (const, override));
  MOCK_METHOD(int, SetActiveCngDecoder, (uint8_t rtp_payload_type), (override));
  MOCK_METHOD(ComfortNoiseDecoder*, GetActiveCngDecoder, (), (const, override));
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_MOCK_MOCK_DECODER_DATABASE_H_
