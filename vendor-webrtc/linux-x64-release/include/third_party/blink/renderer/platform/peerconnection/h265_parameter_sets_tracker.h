// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_H265_PARAMETER_SETS_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_H265_PARAMETER_SETS_TRACKER_H_

#include <cstddef>
#include <cstdint>
#include <memory>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/hash_functions.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/webrtc/api/array_view.h"
#include "third_party/webrtc/common_video/h265/h265_bitstream_parser.h"
#include "third_party/webrtc/modules/video_coding/include/video_codec_interface.h"

namespace blink {

// This is used on H.265 sender side to ensure we are always sending
// bitstream that has parameter set NALUs enclosed into the H.265 IRAP frames.
// Unlike H.264, the tracker is not intended to be used by receiver side
// for attempt to fix received bitstream. H.265 receiver must always issue key
// frame request if parameter set is not part of IRAP picture.
// For more details, refer to the "sprop-sps, sprop-pps, sprop-vps, sprop-sei:"
// section of
// https://datatracker.ietf.org/doc/html/draft-ietf-avtcore-hevc-webrtc-01
//
// Parameter sets supported by this tracker include VPS(video parameter set),
// SPS(sequence parameter set) and PPS(picture parameter set). They are defined
// in section 7.3.2.1, 7.3.2.2 and 7.3.2.3 of ITU H.265: High efficiency video
// coding (https://www.itu.int/rec/T-REC-H.265, version 09/23)
class PLATFORM_EXPORT H265ParameterSetsTracker {
 public:
  enum class PacketAction : uint8_t {
    kInsert = 0,
    kRequestKeyframe,
    kPassThrough,
  };
  struct FixedBitstream {
    PacketAction action;
    webrtc::scoped_refptr<webrtc::EncodedImageBuffer> bitstream;
  };

  H265ParameterSetsTracker();
  virtual ~H265ParameterSetsTracker();

  // Keeps track of incoming bitstream and insert VPS/SPS/PPS before the VCL
  // layer NALUs when needed.
  // Once VPS/SPS/PPS is detected in the bitstream, it will be recorded, and
  // if an IRAP picture is passed in without associated VPS/SPS/PPS in the
  // bitstream, will return the fixed bitstream with action set to kInsert; If
  // the incoming bitstream already contains necessary parameter sets, or
  // incoming bitstream does not contain IRAP pictures, the returned
  // FixedBistream's |bitstream member| is not set, and |action| will be set to
  // kPassThrough; If the incoming bitstream needs to be fixed but corresponding
  // parameter set is not found, the returned FixedBitstream will get |action|
  // set to kRequestkeyframe, and its |bitstream| member will not be set.
  virtual FixedBitstream MaybeFixBitstream(
      webrtc::ArrayView<const uint8_t> bitstream);

 private:
  // Stores PPS payload and the active SPS ID.
  struct PpsData {
    PpsData();
    PpsData(PpsData&& rhs);
    PpsData& operator=(PpsData&& rhs);
    ~PpsData();

    // The value of sps_seq_parameter_set_id for the active SPS.
    uint32_t sps_id = 0;
    // Payload size.
    size_t size = 0;
    std::unique_ptr<uint8_t[]> payload;
  };

  // Stores SPS payload and the active VPS ID.
  struct SpsData {
    SpsData();
    SpsData(SpsData&& rhs);
    SpsData& operator=(SpsData&& rhs);
    ~SpsData();

    // The value of the vps_video_parameter_set_id of the active VPS.
    uint32_t vps_id = 0;
    // Payload size.
    size_t size = 0;
    std::unique_ptr<uint8_t[]> payload;
  };

  // Stores VPS payload.
  struct VpsData {
    VpsData();
    VpsData(VpsData&& rhs);
    VpsData& operator=(VpsData&& rhs);
    ~VpsData();

    // Payload size.
    size_t size = 0;
    std::unique_ptr<uint8_t[]> payload;
  };

  webrtc::H265BitstreamParser parser_;
  // Map from vps_video_parameter_set_id to the VPS payload associated with this
  // ID.
  WTF::HashMap<uint32_t,
               std::unique_ptr<PpsData>,
               IntWithZeroKeyHashTraits<uint32_t>>
      pps_data_;
  // Map from sps_video_parameter_set_id to the SPS payload associated with this
  // ID.
  WTF::HashMap<uint32_t,
               std::unique_ptr<SpsData>,
               IntWithZeroKeyHashTraits<uint32_t>>
      sps_data_;
  // Map from pps_pic_parameter_set_id to the PPS payload associated with this
  // ID.
  WTF::HashMap<uint32_t,
               std::unique_ptr<VpsData>,
               IntWithZeroKeyHashTraits<uint32_t>>
      vps_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_H265_PARAMETER_SETS_TRACKER_H_
