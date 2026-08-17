// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_ORIENTATION_ENUM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_ORIENTATION_ENUM_H_

namespace blink {

// This enum intentionally matches the orientation values from the EXIF spec.
// See JEITA CP-3451, page 18. http://www.exif.org/Exif2-2.PDF
// These values are persisted to logs. Entries should not be renumbered and
// numeric values should never be reused.
enum class ImageOrientationEnum : int8_t {
  // "TopLeft" means that the 0 row starts at the Top, the 0 column starts at
  // the Left.
  kOriginTopLeft = 1,      // default
  kOriginTopRight = 2,     // mirror along y-axis
  kOriginBottomRight = 3,  // 180 degree rotation
  kOriginBottomLeft = 4,   // mirror along the x-axis
  kOriginLeftTop = 5,      // mirror along x-axis + 270 degree CW rotation
  kOriginRightTop = 6,     // 90 degree CW rotation
  kOriginRightBottom = 7,  // mirror along x-axis + 90 degree CW rotation
  kOriginLeftBottom = 8,   // 270 degree CW rotation
  // All other values are "reserved" as of EXIF 2.2
  kDefault = kOriginTopLeft,
  kMinValue = kOriginTopLeft,
  kMaxValue = kOriginLeftBottom,
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_ORIENTATION_ENUM_H_
