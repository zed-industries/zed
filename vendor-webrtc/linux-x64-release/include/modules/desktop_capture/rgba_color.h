/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_RGBA_COLOR_H_
#define MODULES_DESKTOP_CAPTURE_RGBA_COLOR_H_

#include <stdint.h>

#include "modules/desktop_capture/desktop_frame.h"

namespace webrtc {

// A four-byte structure to store a color in BGRA format. This structure also
// provides functions to be created from uint8_t array, say,
// DesktopFrame::data(). It always uses BGRA order for internal storage to match
// DesktopFrame::data().
struct RgbaColor final {
  // Creates a color with BGRA channels.
  RgbaColor(uint8_t blue, uint8_t green, uint8_t red, uint8_t alpha);

  // Creates a color with BGR channels, and set alpha channel to 255 (opaque).
  RgbaColor(uint8_t blue, uint8_t green, uint8_t red);

  // Creates a color from four-byte in BGRA order, i.e. DesktopFrame::data().
  explicit RgbaColor(const uint8_t* bgra);

  // Creates a color from BGRA channels in a uint format. Consumers should make
  // sure the memory order of the uint32_t is always BGRA from left to right, no
  // matter the system endian. This function creates an equivalent RgbaColor
  // instance from the ToUInt32() result of another RgbaColor instance.
  explicit RgbaColor(uint32_t bgra);

  // Returns true if `this` and `right` is the same color.
  bool operator==(const RgbaColor& right) const;

  // Returns true if `this` and `right` are different colors.
  bool operator!=(const RgbaColor& right) const;

  uint32_t ToUInt32() const;

  uint8_t blue;
  uint8_t green;
  uint8_t red;
  uint8_t alpha;
};
static_assert(
    DesktopFrame::kBytesPerPixel == sizeof(RgbaColor),
    "A pixel in DesktopFrame should be safe to be represented by a RgbaColor");

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_RGBA_COLOR_H_
