// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_CANVAS_ROTATION_IN_VERTICAL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_CANVAS_ROTATION_IN_VERTICAL_H_

namespace blink {

enum class CanvasRotationInVertical : char {
  kRegular = 0,
  kRotateCanvasUpright = 1,
  kOblique = 2,
  kRotateCanvasUprightOblique = 3,
};

inline bool IsCanvasRotationInVerticalUpright(CanvasRotationInVertical r) {
  return static_cast<char>(r) &
         static_cast<char>(CanvasRotationInVertical::kRotateCanvasUpright);
}

inline bool IsCanvasRotationOblque(CanvasRotationInVertical r) {
  return static_cast<char>(r) &
         static_cast<char>(CanvasRotationInVertical::kOblique);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_CANVAS_ROTATION_IN_VERTICAL_H_
