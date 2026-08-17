/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_FRAME_OBJECT_H_
#define MODULES_RTP_RTCP_SOURCE_FRAME_OBJECT_H_

#include <cstdint>
#include <optional>
#include <variant>
#include <vector>

#include "api/rtp_packet_infos.h"
#include "api/scoped_refptr.h"
#include "api/video/color_space.h"
#include "api/video/encoded_frame.h"
#include "api/video/encoded_image.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_content_type.h"
#include "api/video/video_frame_metadata.h"
#include "api/video/video_frame_type.h"
#include "api/video/video_rotation.h"
#include "api/video/video_timing.h"
#include "common_video/frame_instrumentation_data.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"

namespace webrtc {

class RtpFrameObject : public EncodedFrame {
 public:
  RtpFrameObject(uint16_t first_seq_num,
                 uint16_t last_seq_num,
                 bool markerBit,
                 int times_nacked,
                 int64_t first_packet_received_time,
                 int64_t last_packet_received_time,
                 uint32_t rtp_timestamp,
                 int64_t ntp_time_ms,
                 const VideoSendTiming& timing,
                 uint8_t payload_type,
                 VideoCodecType codec,
                 VideoRotation rotation,
                 VideoContentType content_type,
                 const RTPVideoHeader& video_header,
                 const std::optional<webrtc::ColorSpace>& color_space,
                 const std::optional<std::variant<FrameInstrumentationSyncData,
                                                  FrameInstrumentationData>>&
                     frame_instrumentation_data,
                 RtpPacketInfos packet_infos,
                 scoped_refptr<EncodedImageBuffer> image_buffer);

  ~RtpFrameObject() override;
  uint16_t first_seq_num() const;
  uint16_t last_seq_num() const;
  int times_nacked() const;
  VideoFrameType frame_type() const;
  VideoCodecType codec_type() const;
  int64_t ReceivedTime() const override;
  int64_t RenderTime() const override;
  bool delayed_by_retransmission() const override;
  const RTPVideoHeader& GetRtpVideoHeader() const;

  uint8_t* mutable_data() { return image_buffer_->data(); }

  const std::vector<uint32_t>& Csrcs() const { return csrcs_; }

  void SetFirstSeqNum(uint16_t first_seq_num) {
    first_seq_num_ = first_seq_num;
  }
  void SetLastSeqNum(uint16_t last_seq_num) { last_seq_num_ = last_seq_num; }
  void SetHeaderFromMetadata(const VideoFrameMetadata& metadata);

 private:
  // Reference for mutable access.
  scoped_refptr<EncodedImageBuffer> image_buffer_;
  RTPVideoHeader rtp_video_header_;
  VideoCodecType codec_type_;
  uint16_t first_seq_num_;
  uint16_t last_seq_num_;
  int64_t last_packet_received_time_;
  std::vector<uint32_t> csrcs_;

  // Equal to times nacked of the packet with the highet times nacked
  // belonging to this frame.
  int times_nacked_;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_FRAME_OBJECT_H_
