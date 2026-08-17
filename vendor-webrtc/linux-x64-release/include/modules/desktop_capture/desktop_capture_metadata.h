/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METADATA_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METADATA_H_

#if defined(WEBRTC_USE_GIO)
#include "modules/portal/xdg_session_details.h"
#endif  // defined(WEBRTC_USE_GIO)

namespace webrtc {

// Container for the metadata associated with a desktop capturer.
struct DesktopCaptureMetadata {
#if defined(WEBRTC_USE_GIO)
  // Details about the XDG desktop session handle (used by wayland
  // implementation in remoting)
  xdg_portal::SessionDetails session_details;
#endif  // defined(WEBRTC_USE_GIO)
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METADATA_H_
