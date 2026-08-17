/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_ACM2_ACM_SEND_TEST_H_
#define MODULES_AUDIO_CODING_ACM2_ACM_SEND_TEST_H_

#include <memory>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/audio/audio_frame.h"
#include "api/environment/environment.h"
#include "modules/audio_coding/include/audio_coding_module.h"
#include "modules/audio_coding/neteq/tools/packet_source.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
class AudioEncoder;

namespace test {
class InputAudioFile;
class Packet;

class AcmSendTestOldApi : public AudioPacketizationCallback,
                          public PacketSource {
 public:
  AcmSendTestOldApi(InputAudioFile* audio_source,
                    int source_rate_hz,
                    int test_duration_ms);
  ~AcmSendTestOldApi() override;

  AcmSendTestOldApi(const AcmSendTestOldApi&) = delete;
  AcmSendTestOldApi& operator=(const AcmSendTestOldApi&) = delete;

  // Registers the send codec. Returns true on success, false otherwise.
  bool RegisterCodec(absl::string_view payload_name,
                     int sampling_freq_hz,
                     int channels,
                     int payload_type,
                     int frame_size_samples);

  // Registers an external send codec.
  void RegisterExternalCodec(
      std::unique_ptr<AudioEncoder> external_speech_encoder);

  // Inherited from PacketSource.
  std::unique_ptr<Packet> NextPacket() override;

  // Inherited from AudioPacketizationCallback.
  int32_t SendData(AudioFrameType frame_type,
                   uint8_t payload_type,
                   uint32_t timestamp,
                   const uint8_t* payload_data,
                   size_t payload_len_bytes,
                   int64_t absolute_capture_timestamp_ms) override;

  AudioCodingModule* acm() { return acm_.get(); }

 private:
  static const int kBlockSizeMs = 10;

  // Creates a Packet object from the last packet produced by ACM (and received
  // through the SendData method as a callback).
  std::unique_ptr<Packet> CreatePacket();

  SimulatedClock clock_;
  const Environment env_;
  std::unique_ptr<AudioCodingModule> acm_;
  InputAudioFile* audio_source_;
  int source_rate_hz_;
  const size_t input_block_size_samples_;
  AudioFrame input_frame_;
  bool codec_registered_;
  int test_duration_ms_;
  // The following member variables are set whenever SendData() is called.
  AudioFrameType frame_type_;
  int payload_type_;
  uint32_t timestamp_;
  uint16_t sequence_number_;
  std::vector<uint8_t> last_payload_vec_;
  bool data_to_send_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_ACM2_ACM_SEND_TEST_H_
