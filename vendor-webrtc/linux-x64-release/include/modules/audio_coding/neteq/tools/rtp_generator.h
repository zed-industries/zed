/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_GENERATOR_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_GENERATOR_H_

#include "api/rtp_headers.h"

namespace webrtc {
namespace test {

// Class for generating RTP headers.
class RtpGenerator {
 public:
  RtpGenerator(int samples_per_ms,
               uint16_t start_seq_number = 0,
               uint32_t start_timestamp = 0,
               uint32_t start_send_time_ms = 0,
               uint32_t ssrc = 0x12345678)
      : seq_number_(start_seq_number),
        timestamp_(start_timestamp),
        next_send_time_ms_(start_send_time_ms),
        ssrc_(ssrc),
        samples_per_ms_(samples_per_ms),
        drift_factor_(0.0) {}

  virtual ~RtpGenerator() {}

  RtpGenerator(const RtpGenerator&) = delete;
  RtpGenerator& operator=(const RtpGenerator&) = delete;

  // Writes the next RTP header to `rtp_header`, which will be of type
  // `payload_type`. Returns the send time for this packet (in ms). The value of
  // `payload_length_samples` determines the send time for the next packet.
  virtual uint32_t GetRtpHeader(uint8_t payload_type,
                                size_t payload_length_samples,
                                RTPHeader* rtp_header);

  void set_drift_factor(double factor);

 protected:
  uint16_t seq_number_;
  uint32_t timestamp_;
  uint32_t next_send_time_ms_;
  const uint32_t ssrc_;
  const int samples_per_ms_;
  double drift_factor_;
};

class TimestampJumpRtpGenerator : public RtpGenerator {
 public:
  TimestampJumpRtpGenerator(int samples_per_ms,
                            uint16_t start_seq_number,
                            uint32_t start_timestamp,
                            uint32_t jump_from_timestamp,
                            uint32_t jump_to_timestamp)
      : RtpGenerator(samples_per_ms, start_seq_number, start_timestamp),
        jump_from_timestamp_(jump_from_timestamp),
        jump_to_timestamp_(jump_to_timestamp) {}

  TimestampJumpRtpGenerator(const TimestampJumpRtpGenerator&) = delete;
  TimestampJumpRtpGenerator& operator=(const TimestampJumpRtpGenerator&) =
      delete;

  uint32_t GetRtpHeader(uint8_t payload_type,
                        size_t payload_length_samples,
                        RTPHeader* rtp_header) override;

 private:
  uint32_t jump_from_timestamp_;
  uint32_t jump_to_timestamp_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_RTP_GENERATOR_H_
