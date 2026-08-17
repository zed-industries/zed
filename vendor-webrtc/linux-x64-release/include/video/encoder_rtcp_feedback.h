/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef VIDEO_ENCODER_RTCP_FEEDBACK_H_
#define VIDEO_ENCODER_RTCP_FEEDBACK_H_

#include <functional>
#include <vector>

#include "api/environment/environment.h"
#include "api/sequence_checker.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "call/rtp_video_sender_interface.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "rtc_base/system/no_unique_address.h"
#include "video/video_stream_encoder_interface.h"

namespace webrtc {

class VideoStreamEncoderInterface;

// This class passes feedback (such as key frame requests or loss notifications)
// from the RtpRtcp module.
class EncoderRtcpFeedback : public RtcpIntraFrameObserver,
                            public RtcpLossNotificationObserver {
 public:
  EncoderRtcpFeedback(
      const Environment& env,
      bool per_layer_keyframes,
      const std::vector<uint32_t>& ssrcs,
      VideoStreamEncoderInterface* encoder,
      std::function<std::vector<RtpSequenceNumberMap::Info>(
          uint32_t ssrc,
          const std::vector<uint16_t>& seq_nums)> get_packet_infos);
  ~EncoderRtcpFeedback() override = default;

  void OnReceivedIntraFrameRequest(uint32_t ssrc) override;

  // Implements RtcpLossNotificationObserver.
  void OnReceivedLossNotification(uint32_t ssrc,
                                  uint16_t seq_num_of_last_decodable,
                                  uint16_t seq_num_of_last_received,
                                  bool decodability_flag) override;

 private:
  const Environment env_;
  const std::vector<uint32_t> ssrcs_;
  const bool per_layer_keyframes_;
  const std::function<std::vector<RtpSequenceNumberMap::Info>(
      uint32_t ssrc,
      const std::vector<uint16_t>& seq_nums)>
      get_packet_infos_;
  VideoStreamEncoderInterface* const video_stream_encoder_;

  RTC_NO_UNIQUE_ADDRESS SequenceChecker packet_delivery_queue_;
  std::vector<Timestamp> time_last_packet_delivery_queue_
      RTC_GUARDED_BY(packet_delivery_queue_);

  const TimeDelta min_keyframe_send_interval_;
};

}  // namespace webrtc

#endif  // VIDEO_ENCODER_RTCP_FEEDBACK_H_
