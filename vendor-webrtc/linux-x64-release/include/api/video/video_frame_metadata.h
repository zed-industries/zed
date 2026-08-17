/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_FRAME_METADATA_H_
#define API_VIDEO_VIDEO_FRAME_METADATA_H_

#include <cstdint>
#include <optional>
#include <variant>
#include <vector>

#include "absl/container/inlined_vector.h"
#include "api/array_view.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_content_type.h"
#include "api/video/video_frame_type.h"
#include "api/video/video_rotation.h"
#include "modules/video_coding/codecs/h264/include/h264_globals.h"
#include "modules/video_coding/codecs/vp8/include/vp8_globals.h"
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

using RTPVideoHeaderCodecSpecifics = std::variant<std::monostate,
                                                  RTPVideoHeaderVP8,
                                                  RTPVideoHeaderVP9,
                                                  RTPVideoHeaderH264>;

// A subset of metadata from the RTP video header, exposed in insertable streams
// API.
class RTC_EXPORT VideoFrameMetadata {
 public:
  VideoFrameMetadata();
  VideoFrameMetadata(const VideoFrameMetadata&) = default;
  VideoFrameMetadata& operator=(const VideoFrameMetadata&) = default;

  VideoFrameType GetFrameType() const;
  void SetFrameType(VideoFrameType frame_type);

  uint16_t GetWidth() const;
  void SetWidth(uint16_t width);

  uint16_t GetHeight() const;
  void SetHeight(uint16_t height);

  VideoRotation GetRotation() const;
  void SetRotation(VideoRotation rotation);

  VideoContentType GetContentType() const;
  void SetContentType(VideoContentType content_type);

  std::optional<int64_t> GetFrameId() const;
  void SetFrameId(std::optional<int64_t> frame_id);

  int GetSpatialIndex() const;
  void SetSpatialIndex(int spatial_index);

  int GetTemporalIndex() const;
  void SetTemporalIndex(int temporal_index);

  ArrayView<const int64_t> GetFrameDependencies() const;
  void SetFrameDependencies(ArrayView<const int64_t> frame_dependencies);

  ArrayView<const DecodeTargetIndication> GetDecodeTargetIndications() const;
  void SetDecodeTargetIndications(
      ArrayView<const DecodeTargetIndication> decode_target_indications);

  bool GetIsLastFrameInPicture() const;
  void SetIsLastFrameInPicture(bool is_last_frame_in_picture);

  uint8_t GetSimulcastIdx() const;
  void SetSimulcastIdx(uint8_t simulcast_idx);

  VideoCodecType GetCodec() const;
  void SetCodec(VideoCodecType codec);

  // Which varient is used depends on the VideoCodecType from GetCodecs().
  const RTPVideoHeaderCodecSpecifics& GetRTPVideoHeaderCodecSpecifics() const;
  void SetRTPVideoHeaderCodecSpecifics(
      RTPVideoHeaderCodecSpecifics codec_specifics);

  uint32_t GetSsrc() const;
  void SetSsrc(uint32_t ssrc);
  std::vector<uint32_t> GetCsrcs() const;
  void SetCsrcs(std::vector<uint32_t> csrcs);

  friend bool operator==(const VideoFrameMetadata& lhs,
                         const VideoFrameMetadata& rhs);
  friend bool operator!=(const VideoFrameMetadata& lhs,
                         const VideoFrameMetadata& rhs);

 private:
  VideoFrameType frame_type_ = VideoFrameType::kEmptyFrame;
  int16_t width_ = 0;
  int16_t height_ = 0;
  VideoRotation rotation_ = VideoRotation::kVideoRotation_0;
  VideoContentType content_type_ = VideoContentType::UNSPECIFIED;

  // Corresponding to GenericDescriptorInfo.
  std::optional<int64_t> frame_id_;
  int spatial_index_ = 0;
  int temporal_index_ = 0;
  absl::InlinedVector<int64_t, 5> frame_dependencies_;
  absl::InlinedVector<DecodeTargetIndication, 10> decode_target_indications_;

  bool is_last_frame_in_picture_ = true;
  uint8_t simulcast_idx_ = 0;
  VideoCodecType codec_ = VideoCodecType::kVideoCodecGeneric;
  RTPVideoHeaderCodecSpecifics codec_specifics_;

  // RTP info.
  uint32_t ssrc_ = 0u;
  std::vector<uint32_t> csrcs_;
};
}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_FRAME_METADATA_H_
