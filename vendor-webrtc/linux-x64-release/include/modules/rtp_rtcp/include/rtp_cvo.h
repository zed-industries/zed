/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_INCLUDE_RTP_CVO_H_
#define MODULES_RTP_RTCP_INCLUDE_RTP_CVO_H_

#include <cstdint>

#include "api/video/video_rotation.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Please refer to http://www.etsi.org/deliver/etsi_ts/126100_126199/126114/
// 12.07.00_60/ts_126114v120700p.pdf Section 7.4.5. The rotation of a frame is
// the clockwise angle the frames must be rotated in order to display the frames
// correctly if the display is rotated in its natural orientation.
inline uint8_t ConvertVideoRotationToCVOByte(VideoRotation rotation) {
  switch (rotation) {
    case kVideoRotation_0:
      return 0;
    case kVideoRotation_90:
      return 1;
    case kVideoRotation_180:
      return 2;
    case kVideoRotation_270:
      return 3;
  }
  RTC_DCHECK_NOTREACHED();
  return 0;
}

inline VideoRotation ConvertCVOByteToVideoRotation(uint8_t cvo_byte) {
  // CVO byte: |0 0 0 0 C F R R|.
  const uint8_t rotation_bits = cvo_byte & 0x3;
  switch (rotation_bits) {
    case 0:
      return kVideoRotation_0;
    case 1:
      return kVideoRotation_90;
    case 2:
      return kVideoRotation_180;
    case 3:
      return kVideoRotation_270;
    default:
      RTC_DCHECK_NOTREACHED();
      return kVideoRotation_0;
  }
}

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_INCLUDE_RTP_CVO_H_
