/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_CROPPED_DESKTOP_FRAME_H_
#define MODULES_DESKTOP_CAPTURE_CROPPED_DESKTOP_FRAME_H_

#include <memory>

#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Creates a DesktopFrame to contain only the area of `rect` in the original
// `frame`.
// `frame` should not be nullptr. `rect` is in `frame` coordinate, i.e.
// `frame`->top_left() does not impact the area of `rect`.
// Returns nullptr frame if `rect` is not contained by the bounds of `frame`.
std::unique_ptr<DesktopFrame> RTC_EXPORT
CreateCroppedDesktopFrame(std::unique_ptr<DesktopFrame> frame,
                          const DesktopRect& rect);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_CROPPED_DESKTOP_FRAME_H_
