/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_RTP_FRAME_ID_ONLY_REF_FINDER_H_
#define MODULES_VIDEO_CODING_RTP_FRAME_ID_ONLY_REF_FINDER_H_

#include <cstdint>
#include <memory>

#include "modules/rtp_rtcp/source/frame_object.h"
#include "modules/video_coding/rtp_frame_reference_finder.h"
#include "rtc_base/numerics/sequence_number_unwrapper.h"

namespace webrtc {

class RtpFrameIdOnlyRefFinder {
 public:
  RtpFrameIdOnlyRefFinder() = default;

  RtpFrameReferenceFinder::ReturnVector ManageFrame(
      std::unique_ptr<RtpFrameObject> frame,
      int frame_id);

 private:
  static constexpr int kFrameIdLength = 1 << 15;
  SeqNumUnwrapper<uint16_t, kFrameIdLength> unwrapper_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_RTP_FRAME_ID_ONLY_REF_FINDER_H_
