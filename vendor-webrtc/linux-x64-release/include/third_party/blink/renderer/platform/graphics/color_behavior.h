// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_BEHAVIOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_BEHAVIOR_H_

namespace blink {

enum class ColorBehavior {
  // This specifies to ignore color profiles embedded in images entirely. No
  // transformations will be applied to any pixel data, and no SkImages will be
  // tagged with an SkColorSpace.
  kIgnore,
  // This specifies that images will not be transformed (to the extent
  // possible), but that SkImages will be tagged with the embedded SkColorSpace
  // (or sRGB if there was no embedded color profile).
  kTag,
  // This specifies that images will be transformed to sRGB, and that SkImages
  // will not be tagged with any SkColorSpace.
  kTransformToSRGB,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_COLOR_BEHAVIOR_H_
