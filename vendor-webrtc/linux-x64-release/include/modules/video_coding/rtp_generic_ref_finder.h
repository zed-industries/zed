/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_RTP_GENERIC_REF_FINDER_H_
#define MODULES_VIDEO_CODING_RTP_GENERIC_REF_FINDER_H_

#include <memory>

#include "modules/rtp_rtcp/source/frame_object.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/video_coding/rtp_frame_reference_finder.h"

namespace webrtc {

class RtpGenericFrameRefFinder {
 public:
  RtpGenericFrameRefFinder() = default;

  RtpFrameReferenceFinder::ReturnVector ManageFrame(
      std::unique_ptr<RtpFrameObject> frame,
      const RTPVideoHeader::GenericDescriptorInfo& descriptor);
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_RTP_GENERIC_REF_FINDER_H_
