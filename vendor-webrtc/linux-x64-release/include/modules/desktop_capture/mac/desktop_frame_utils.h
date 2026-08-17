/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_UTILS_H_
#define MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_UTILS_H_

#include <memory>

#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/mac/desktop_frame_cgimage.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

std::unique_ptr<DesktopFrame> RTC_EXPORT
CreateDesktopFrameFromCGImage(webrtc::ScopedCFTypeRef<CGImageRef> cg_image);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_FRAME_UTILS_H_
