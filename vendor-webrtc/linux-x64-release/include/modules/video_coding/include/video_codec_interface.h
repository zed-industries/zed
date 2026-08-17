/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INTERFACE_H_
#define MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INTERFACE_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <type_traits>
#include <variant>

#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video/video_codec_type.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/video_encoder.h"
#include "common_video/frame_instrumentation_data.h"
#include "common_video/generic_frame_descriptor/generic_frame_info.h"
#include "modules/video_coding/codecs/h264/include/h264_globals.h"
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Note: If any pointers are added to this struct, it must be fitted
// with a copy-constructor. See below.
// Hack alert - the code assumes that thisstruct is memset when constructed.
struct CodecSpecificInfoVP8 {
  bool nonReference;
  uint8_t temporalIdx;
  bool layerSync;
  int8_t keyIdx;  // Negative value to skip keyIdx.

  // Used to generate the list of dependency frames.
  // `referencedBuffers` and `updatedBuffers` contain buffer IDs.
  // Note that the buffer IDs here have a one-to-one mapping with the actual
  // codec buffers, but the exact mapping (i.e. whether 0 refers to Last,
  // to Golden or to Arf) is not pre-determined.
  // More references may be specified than are strictly necessary, but not less.
  // TODO(bugs.webrtc.org/10242): Remove `useExplicitDependencies` once all
  // encoder-wrappers are updated.
  bool useExplicitDependencies;
  static constexpr size_t kBuffersCount = 3;
  size_t referencedBuffers[kBuffersCount];
  size_t referencedBuffersCount;
  size_t updatedBuffers[kBuffersCount];
  size_t updatedBuffersCount;
};
static_assert(std::is_trivial_v<CodecSpecificInfoVP8> &&
                  std::is_standard_layout_v<CodecSpecificInfoVP8>,
              "");

// Hack alert - the code assumes that thisstruct is memset when constructed.
struct CodecSpecificInfoVP9 {
  bool first_frame_in_picture;  // First frame, increment picture_id.
  bool inter_pic_predicted;     // This layer frame is dependent on previously
                                // coded frame(s).
  bool flexible_mode;
  bool ss_data_available;
  bool non_ref_for_inter_layer_pred;

  uint8_t temporal_idx;
  bool temporal_up_switch;
  bool inter_layer_predicted;  // Frame is dependent on directly lower spatial
                               // layer frame.
  uint8_t gof_idx;

  // SS data.
  size_t num_spatial_layers;  // Always populated.
  size_t first_active_layer;
  bool spatial_layer_resolution_present;
  uint16_t width[kMaxVp9NumberOfSpatialLayers];
  uint16_t height[kMaxVp9NumberOfSpatialLayers];
  GofInfoVP9 gof;

  // Frame reference data.
  uint8_t num_ref_pics;
  uint8_t p_diff[kMaxVp9RefPics];
};
static_assert(std::is_trivial_v<CodecSpecificInfoVP9> &&
                  std::is_standard_layout_v<CodecSpecificInfoVP9>,
              "");

// Hack alert - the code assumes that thisstruct is memset when constructed.
struct CodecSpecificInfoH264 {
  H264PacketizationMode packetization_mode;
  uint8_t temporal_idx;
  bool base_layer_sync;
  bool idr_frame;
};
static_assert(std::is_trivial_v<CodecSpecificInfoH264> &&
                  std::is_standard_layout_v<CodecSpecificInfoH264>,
              "");

union CodecSpecificInfoUnion {
  CodecSpecificInfoVP8 VP8;
  CodecSpecificInfoVP9 VP9;
  CodecSpecificInfoH264 H264;
};
static_assert(std::is_trivial_v<CodecSpecificInfoUnion> &&
                  std::is_standard_layout_v<CodecSpecificInfoUnion>,
              "");

// Note: if any pointers are added to this struct or its sub-structs, it
// must be fitted with a copy-constructor. This is because it is copied
// in the copy-constructor of VCMEncodedFrame.
struct RTC_EXPORT CodecSpecificInfo {
  CodecSpecificInfo();
  CodecSpecificInfo(const CodecSpecificInfo&);
  ~CodecSpecificInfo();

  VideoCodecType codecType;
  CodecSpecificInfoUnion codecSpecific;
  bool end_of_picture = true;
  std::optional<GenericFrameInfo> generic_frame_info;
  std::optional<FrameDependencyStructure> template_structure;
  std::optional<ScalabilityMode> scalability_mode;

  // Required for automatic corruption detection.
  std::optional<
      std::variant<FrameInstrumentationSyncData, FrameInstrumentationData>>
      frame_instrumentation_data;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_INCLUDE_VIDEO_CODEC_INTERFACE_H_
