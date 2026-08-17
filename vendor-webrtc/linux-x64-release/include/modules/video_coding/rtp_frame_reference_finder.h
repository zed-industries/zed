/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_RTP_FRAME_REFERENCE_FINDER_H_
#define MODULES_VIDEO_CODING_RTP_FRAME_REFERENCE_FINDER_H_

#include <cstdint>
#include <memory>

#include "absl/container/inlined_vector.h"
#include "modules/rtp_rtcp/source/frame_object.h"

namespace webrtc {
namespace internal {
class RtpFrameReferenceFinderImpl;
}  // namespace internal

class RtpFrameReferenceFinder {
 public:
  using ReturnVector = absl::InlinedVector<std::unique_ptr<RtpFrameObject>, 3>;

  RtpFrameReferenceFinder();
  explicit RtpFrameReferenceFinder(int64_t picture_id_offset);
  ~RtpFrameReferenceFinder();

  // The RtpFrameReferenceFinder will hold onto the frame until:
  //  - the required information to determine its references has been received,
  //    in which case it (and possibly other) frames are returned, or
  //  - There are too many stashed frames (determined by `kMaxStashedFrames`),
  //    in which case it gets dropped, or
  //  - It gets cleared by ClearTo, in which case its dropped.
  //  - The frame is old, in which case it also gets dropped.
  ReturnVector ManageFrame(std::unique_ptr<RtpFrameObject> frame);

  // Notifies that padding has been received, which the reference finder
  // might need to calculate the references of a frame.
  ReturnVector PaddingReceived(uint16_t seq_num);

  // Clear all stashed frames that include packets older than `seq_num`.
  void ClearTo(uint16_t seq_num);

 private:
  void AddPictureIdOffset(ReturnVector& frames);

  // How far frames have been cleared out of the buffer by RTP sequence number.
  // A frame will be cleared if it contains a packet with a sequence number
  // older than `cleared_to_seq_num_`.
  int cleared_to_seq_num_ = -1;
  const int64_t picture_id_offset_;
  std::unique_ptr<internal::RtpFrameReferenceFinderImpl> impl_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_RTP_FRAME_REFERENCE_FINDER_H_
