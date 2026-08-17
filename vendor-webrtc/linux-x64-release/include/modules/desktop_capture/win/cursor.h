/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_CURSOR_H_
#define MODULES_DESKTOP_CAPTURE_WIN_CURSOR_H_

#include <windows.h>

namespace webrtc {

class MouseCursor;

// Converts an HCURSOR into a `MouseCursor` instance.
MouseCursor* CreateMouseCursorFromHCursor(HDC dc, HCURSOR cursor);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_CURSOR_H_
