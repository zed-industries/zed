/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_RTP_VP9_REF_FINDER_H_
#define MODULES_VIDEO_CODING_RTP_VP9_REF_FINDER_H_

#include <array>
#include <cstdint>
#include <deque>
#include <map>
#include <memory>
#include <set>

#include "modules/rtp_rtcp/source/frame_object.h"
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "modules/video_coding/rtp_frame_reference_finder.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"
#include "rtc_base/numerics/sequence_number_util.h"

namespace webrtc {

class RtpVp9RefFinder {
 public:
  RtpVp9RefFinder() = default;

  RtpFrameReferenceFinder::ReturnVector ManageFrame(
      std::unique_ptr<RtpFrameObject> frame);
  void ClearTo(uint16_t seq_num);

 private:
  static constexpr int kFrameIdLength = 1 << 15;
  static constexpr int kMaxGofSaved = 50;
  static constexpr int kMaxLayerInfo = 50;
  static constexpr int kMaxNotYetReceivedFrames = 100;
  static constexpr int kMaxStashedFrames = 100;
  static constexpr int kMaxTemporalLayers = 5;

  enum FrameDecision { kStash, kHandOff, kDrop };

  struct GofInfo {
    GofInfo(GofInfoVP9* gof, uint16_t last_picture_id)
        : gof(gof), last_picture_id(last_picture_id) {}
    GofInfoVP9* gof;
    uint16_t last_picture_id;
  };

  struct UnwrappedTl0Frame {
    int64_t unwrapped_tl0;
    std::unique_ptr<RtpFrameObject> frame;
  };

  FrameDecision ManageFrameFlexible(RtpFrameObject* frame,
                                    const RTPVideoHeaderVP9& vp9_header);
  FrameDecision ManageFrameGof(RtpFrameObject* frame,
                               const RTPVideoHeaderVP9& vp9_header,
                               int64_t unwrapped_tl0);
  void RetryStashedFrames(RtpFrameReferenceFinder::ReturnVector& res);

  bool MissingRequiredFrameVp9(uint16_t picture_id, const GofInfo& info);

  void FrameReceivedVp9(uint16_t picture_id, GofInfo* info);
  bool UpSwitchInIntervalVp9(uint16_t picture_id,
                             uint8_t temporal_idx,
                             uint16_t pid_ref);

  void FlattenFrameIdAndRefs(RtpFrameObject* frame, bool inter_layer_predicted);

  // Frames that have been fully received but didn't have all the information
  // needed to determine their references.
  std::deque<UnwrappedTl0Frame> stashed_frames_;

  // Where the current scalability structure is in the
  // `scalability_structures_` array.
  uint8_t current_ss_idx_ = 0;

  // Holds received scalability structures.
  std::array<GofInfoVP9, kMaxGofSaved> scalability_structures_;

  // Holds the the Gof information for a given unwrapped TL0 picture index.
  std::map<int64_t, GofInfo> gof_info_;

  // Keep track of which picture id and which temporal layer that had the
  // up switch flag set.
  std::map<uint16_t, uint8_t, DescendingSeqNumComp<uint16_t, kFrameIdLength>>
      up_switch_;

  // For every temporal layer, keep a set of which frames that are missing.
  std::array<std::set<uint16_t, DescendingSeqNumComp<uint16_t, kFrameIdLength>>,
             kMaxTemporalLayers>
      missing_frames_for_layer_;

  // Unwrapper used to unwrap VP8/VP9 streams which have their picture id
  // specified.
  SeqNumUnwrapper<uint16_t, kFrameIdLength> unwrapper_;

  SeqNumUnwrapper<uint8_t> tl0_unwrapper_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_RTP_VP9_REF_FINDER_H_
