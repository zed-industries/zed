/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_H26X_PACKET_BUFFER_H_
#define MODULES_VIDEO_CODING_H26X_PACKET_BUFFER_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "modules/video_coding/packet_buffer.h"

namespace webrtc {

class H26xPacketBuffer {
 public:
  // The H26xPacketBuffer does the same job as the PacketBuffer but for H264 and
  // H265 only. To make it fit in with surronding code the PacketBuffer
  // input/output classes are used.
  using Packet = video_coding::PacketBuffer::Packet;
  using InsertResult = video_coding::PacketBuffer::InsertResult;

  // |h264_idr_only_keyframes_allowed| is ignored if H.265 is used.
  explicit H26xPacketBuffer(bool h264_idr_only_keyframes_allowed);

  ABSL_MUST_USE_RESULT InsertResult
  InsertPacket(std::unique_ptr<Packet> packet);
  ABSL_MUST_USE_RESULT InsertResult InsertPadding(uint16_t unwrapped_seq_num);

  // Out of band supplied codec parameters for H.264.
  void SetSpropParameterSets(const std::string& sprop_parameter_sets);

 private:
  // Stores PPS payload and the active SPS ID.
  struct PpsInfo {
    PpsInfo() = default;
    PpsInfo(PpsInfo&& rhs) = default;
    PpsInfo& operator=(PpsInfo&& rhs) = default;
    ~PpsInfo() = default;

    // The value of sps_seq_parameter_set_id for the active SPS.
    uint32_t sps_id = 0;
    // Payload size.
    size_t size = 0;
    std::unique_ptr<uint8_t[]> payload;
  };

  // Stores SPS payload and picture size.
  struct SpsInfo {
    SpsInfo() = default;
    SpsInfo(SpsInfo&& rhs) = default;
    SpsInfo& operator=(SpsInfo&& rhs) = default;
    ~SpsInfo() = default;

    // The width and height of decoded pictures.
    int width = -1;
    int height = -1;
    // Payload size.
    size_t size = 0;
    std::unique_ptr<uint8_t[]> payload;
  };

  static constexpr int kBufferSize = 2048;
  static constexpr int kNumTrackedSequences = 5;

  std::unique_ptr<Packet>& GetPacket(int64_t unwrapped_seq_num);
  bool BeginningOfStream(const Packet& packet) const;
  InsertResult FindFrames(int64_t unwrapped_seq_num);
  bool MaybeAssembleFrame(int64_t start_seq_num_unwrapped,
                          int64_t end_sequence_number_unwrapped,
                          InsertResult& result);
  // Store SPS and PPS nalus. They will be used later when an IDR frame is
  // received without SPS/PPS.
  void InsertSpsPpsNalus(const std::vector<uint8_t>& sps,
                         const std::vector<uint8_t>& pps);
  // Insert start code and paramter sets for H.264 payload, also update header
  // if parameter sets are inserted. Return false if required SPS or PPS is not
  // found.
  bool FixH264Packet(Packet& packet);

  // Indicates whether IDR frames without SPS and PPS are allowed.
  const bool h264_idr_only_keyframes_allowed_;
  std::array<std::unique_ptr<Packet>, kBufferSize> buffer_;
  std::array<int64_t, kNumTrackedSequences> last_continuous_in_sequence_;
  int64_t last_continuous_in_sequence_index_ = 0;

  // Map from pps_pic_parameter_set_id to the PPS payload associated with this
  // ID.
  std::map<int, PpsInfo> pps_data_;
  // Map from sps_video_parameter_set_id to the SPS payload associated with this
  // ID.
  std::map<int, SpsInfo> sps_data_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_H26X_PACKET_BUFFER_H_
