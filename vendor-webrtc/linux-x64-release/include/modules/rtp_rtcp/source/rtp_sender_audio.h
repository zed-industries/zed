/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_SENDER_AUDIO_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_SENDER_AUDIO_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/units/timestamp.h"
#include "modules/audio_coding/include/audio_coding_module_typedefs.h"
#include "modules/rtp_rtcp/source/absolute_capture_time_sender.h"
#include "modules/rtp_rtcp/source/dtmf_queue.h"
#include "modules/rtp_rtcp/source/rtp_sender.h"
#include "rtc_base/one_time_event.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

class RTPSenderAudio {
 public:
  RTPSenderAudio(Clock* clock, RTPSender* rtp_sender);

  RTPSenderAudio() = delete;
  RTPSenderAudio(const RTPSenderAudio&) = delete;
  RTPSenderAudio& operator=(const RTPSenderAudio&) = delete;

  ~RTPSenderAudio();

  int32_t RegisterAudioPayload(absl::string_view payload_name,
                               int8_t payload_type,
                               uint32_t frequency,
                               size_t channels,
                               uint32_t rate);

  struct RtpAudioFrame {
    AudioFrameType type = AudioFrameType::kAudioFrameSpeech;
    ArrayView<const uint8_t> payload;

    // Payload id to write to the payload type field of the rtp packet.
    int payload_id = -1;

    // capture time of the audio frame represented as rtp timestamp.
    uint32_t rtp_timestamp = 0;

    // capture time of the audio frame in the same epoch as `clock->CurrentTime`
    std::optional<Timestamp> capture_time;

    // Audio level in dBov for
    // header-extension-for-audio-level-indication.
    // Valid range is [0,127]. Actual value is negative.
    std::optional<int> audio_level_dbov;

    // Contributing sources list.
    ArrayView<const uint32_t> csrcs;
  };
  bool SendAudio(const RtpAudioFrame& frame);

  // Send a DTMF tone using RFC 2833 (4733)
  int32_t SendTelephoneEvent(uint8_t key, uint16_t time_ms, uint8_t level);

 protected:
  bool SendTelephoneEventPacket(
      bool ended,
      uint32_t dtmf_timestamp,
      uint16_t duration,
      bool marker_bit);  // set on first packet in talk burst

  bool MarkerBit(AudioFrameType frame_type, int8_t payload_type);

 private:
  Clock* const clock_ = nullptr;
  RTPSender* const rtp_sender_ = nullptr;

  Mutex send_audio_mutex_;

  // DTMF.
  bool dtmf_event_is_on_ = false;
  bool dtmf_event_first_packet_sent_ = false;
  int8_t dtmf_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;
  uint32_t dtmf_payload_freq_ RTC_GUARDED_BY(send_audio_mutex_) = 8000;
  uint32_t dtmf_timestamp_ = 0;
  uint32_t dtmf_length_samples_ = 0;
  int64_t dtmf_time_last_sent_ = 0;
  uint32_t dtmf_timestamp_last_sent_ = 0;
  DtmfQueue::Event dtmf_current_event_;
  DtmfQueue dtmf_queue_;

  // VAD detection, used for marker bit.
  bool inband_vad_active_ RTC_GUARDED_BY(send_audio_mutex_) = false;
  int8_t cngnb_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;
  int8_t cngwb_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;
  int8_t cngswb_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;
  int8_t cngfb_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;
  int8_t last_payload_type_ RTC_GUARDED_BY(send_audio_mutex_) = -1;

  OneTimeEvent first_packet_sent_;

  std::optional<int> encoder_rtp_timestamp_frequency_
      RTC_GUARDED_BY(send_audio_mutex_);

  AbsoluteCaptureTimeSender absolute_capture_time_sender_
      RTC_GUARDED_BY(send_audio_mutex_);
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_SENDER_AUDIO_H_
