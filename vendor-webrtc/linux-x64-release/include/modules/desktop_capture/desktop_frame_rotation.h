/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_ROTATION_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_ROTATION_H_

#include "modules/desktop_capture/desktop_frame.h"
#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

// Represents the rotation of a DesktopFrame.
enum class Rotation {
  CLOCK_WISE_0,
  CLOCK_WISE_90,
  CLOCK_WISE_180,
  CLOCK_WISE_270,
};

// Rotates input DesktopFrame `source`, copies pixel in an unrotated rectangle
// `source_rect` into the target rectangle of another DesktopFrame `target`.
// Target rectangle here is the rotated `source_rect` plus `target_offset`.
// `rotation` specifies `source` to `target` rotation. `source_rect` is in
// `source` coordinate. `target_offset` is in `target` coordinate.
// This function triggers check failure if `source` does not cover the
// `source_rect`, or `target` does not cover the rotated `rect`.
void RotateDesktopFrame(const DesktopFrame& source,
                        const DesktopRect& source_rect,
                        const Rotation& rotation,
                        const DesktopVector& target_offset,
                        DesktopFrame* target);

// Returns a reverse rotation of `rotation`.
Rotation ReverseRotation(Rotation rotation);

// Returns a rotated DesktopSize of `size`.
DesktopSize RotateSize(DesktopSize size, Rotation rotation);

// Returns a rotated DesktopRect of `rect`. The `size` represents the size of
// the DesktopFrame which `rect` belongs in.
DesktopRect RotateRect(DesktopRect rect, DesktopSize size, Rotation rotation);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_FRAME_ROTATION_H_
