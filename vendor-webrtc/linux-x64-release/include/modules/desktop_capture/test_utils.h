/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_TEST_UTILS_H_
#define MODULES_DESKTOP_CAPTURE_TEST_UTILS_H_

#include "modules/desktop_capture/desktop_frame.h"

namespace webrtc {

// Clears a DesktopFrame `frame` by setting its data() into 0.
void ClearDesktopFrame(DesktopFrame* frame);

// Compares size() and data() of two DesktopFrames `left` and `right`.
bool DesktopFrameDataEquals(const DesktopFrame& left,
                            const DesktopFrame& right);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_TEST_UTILS_H_
