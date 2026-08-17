/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_MOCK_AUDIO_ENCODER_H_
#define TEST_MOCK_AUDIO_ENCODER_H_

#include <string>

#include "api/array_view.h"
#include "api/audio_codecs/audio_encoder.h"
#include "test/gmock.h"

namespace webrtc {

class MockAudioEncoder : public AudioEncoder {
 public:
  MockAudioEncoder();
  ~MockAudioEncoder();
  MOCK_METHOD(int, SampleRateHz, (), (const, override));
  MOCK_METHOD(size_t, NumChannels, (), (const, override));
  MOCK_METHOD(int, RtpTimestampRateHz, (), (const, override));
  MOCK_METHOD(size_t, Num10MsFramesInNextPacket, (), (const, override));
  MOCK_METHOD(size_t, Max10MsFramesInAPacket, (), (const, override));
  MOCK_METHOD(int, GetTargetBitrate, (), (const, override));
  MOCK_METHOD((std::optional<std::pair<TimeDelta, TimeDelta>>),
              GetFrameLengthRange,
              (),
              (const, override));
  MOCK_METHOD((std::optional<std::pair<DataRate, DataRate>>),
              GetBitrateRange,
              (),
              (const, override));

  MOCK_METHOD(void, Reset, (), (override));
  MOCK_METHOD(bool, SetFec, (bool enable), (override));
  MOCK_METHOD(bool, SetDtx, (bool enable), (override));
  MOCK_METHOD(bool, SetApplication, (Application application), (override));
  MOCK_METHOD(void, SetMaxPlaybackRate, (int frequency_hz), (override));
  MOCK_METHOD(void,
              OnReceivedUplinkBandwidth,
              (int target_audio_bitrate_bps,
               std::optional<int64_t> probing_interval_ms),
              (override));
  MOCK_METHOD(void,
              OnReceivedUplinkPacketLossFraction,
              (float uplink_packet_loss_fraction),
              (override));
  MOCK_METHOD(void,
              OnReceivedOverhead,
              (size_t overhead_bytes_per_packet),
              (override));

  MOCK_METHOD(bool,
              EnableAudioNetworkAdaptor,
              (const std::string& config_string, RtcEventLog*),
              (override));

  // Note, we explicitly chose not to create a mock for the Encode method.
  MOCK_METHOD(EncodedInfo,
              EncodeImpl,
              (uint32_t timestamp,
               webrtc::ArrayView<const int16_t> audio,
               webrtc::Buffer*),
              (override));

  class FakeEncoding {
   public:
    // Creates a functor that will return `info` and adjust the webrtc::Buffer
    // given as input to it, so it is info.encoded_bytes larger.
    explicit FakeEncoding(const AudioEncoder::EncodedInfo& info);

    // Shorthand version of the constructor above, for when only setting
    // encoded_bytes in the EncodedInfo object matters.
    explicit FakeEncoding(size_t encoded_bytes);

    AudioEncoder::EncodedInfo operator()(uint32_t timestamp,
                                         ArrayView<const int16_t> audio,
                                         Buffer* encoded);

   private:
    AudioEncoder::EncodedInfo info_;
  };

  class CopyEncoding {
   public:
    ~CopyEncoding();

    // Creates a functor that will return `info` and append the data in the
    // payload to the buffer given as input to it. Up to info.encoded_bytes are
    // appended - make sure the payload is big enough!  Since it uses an
    // ArrayView, it _does not_ copy the payload. Make sure it doesn't fall out
    // of scope!
    CopyEncoding(AudioEncoder::EncodedInfo info,
                 ArrayView<const uint8_t> payload);

    // Shorthand version of the constructor above, for when you wish to append
    // the whole payload and do not care about any EncodedInfo attribute other
    // than encoded_bytes.
    explicit CopyEncoding(ArrayView<const uint8_t> payload);

    AudioEncoder::EncodedInfo operator()(uint32_t timestamp,
                                         ArrayView<const int16_t> audio,
                                         Buffer* encoded);

   private:
    AudioEncoder::EncodedInfo info_;
    ArrayView<const uint8_t> payload_;
  };
};

}  // namespace webrtc

#endif  // TEST_MOCK_AUDIO_ENCODER_H_
