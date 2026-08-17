/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_RTP_VIDEO_FRAME_ASSEMBLER_H_
#define API_VIDEO_RTP_VIDEO_FRAME_ASSEMBLER_H_

#include <cstdint>
#include <memory>
#include <utility>

#include "absl/container/inlined_vector.h"
#include "api/video/encoded_frame.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"

namespace webrtc {
// The RtpVideoFrameAssembler takes RtpPacketReceived and assembles them into
// complete frames. A frame is considered complete when all packets of the frame
// has been received, the bitstream data has successfully extracted, an ID has
// been assigned, and all dependencies are known. Frame IDs are strictly
// monotonic in decode order, dependencies are expressed as frame IDs.
class RtpVideoFrameAssembler {
 public:
  // The RtpVideoFrameAssembler should return "RTP frames", but for now there
  // is no good class for this purpose. For now return an EncodedFrame bundled
  // with some minimal RTP information.
  class AssembledFrame {
   public:
    AssembledFrame(uint16_t rtp_seq_num_start,
                   uint16_t rtp_seq_num_end,
                   std::unique_ptr<EncodedFrame> frame)
        : rtp_seq_num_start_(rtp_seq_num_start),
          rtp_seq_num_end_(rtp_seq_num_end),
          frame_(std::move(frame)) {}

    uint16_t RtpSeqNumStart() const { return rtp_seq_num_start_; }
    uint16_t RtpSeqNumEnd() const { return rtp_seq_num_end_; }
    std::unique_ptr<EncodedFrame> ExtractFrame() { return std::move(frame_); }

   private:
    uint16_t rtp_seq_num_start_;
    uint16_t rtp_seq_num_end_;
    std::unique_ptr<EncodedFrame> frame_;
  };

  // FrameVector is just a vector-like type of std::unique_ptr<EncodedFrame>.
  // The vector type may change without notice.
  using FrameVector = absl::InlinedVector<AssembledFrame, 3>;
  enum PayloadFormat { kRaw, kH264, kVp8, kVp9, kAv1, kGeneric, kH265 };

  explicit RtpVideoFrameAssembler(PayloadFormat payload_format);
  RtpVideoFrameAssembler(const RtpVideoFrameAssembler& other) = delete;
  RtpVideoFrameAssembler& operator=(const RtpVideoFrameAssembler& other) =
      delete;
  ~RtpVideoFrameAssembler();

  // Typically when a packet is inserted zero or one frame is completed. In the
  // case of RTP packets being inserted out of order then sometime multiple
  // frames could be completed from a single packet, hence the 'FrameVector'
  // return type.
  FrameVector InsertPacket(const RtpPacketReceived& packet);

 private:
  class Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace webrtc

#endif  // API_VIDEO_RTP_VIDEO_FRAME_ASSEMBLER_H_
