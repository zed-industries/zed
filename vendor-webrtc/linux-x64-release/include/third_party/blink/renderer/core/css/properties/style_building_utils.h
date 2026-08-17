// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_STYLE_BUILDING_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_STYLE_BUILDING_UTILS_H_

#include "third_party/blink/renderer/core/style/border_image_length.h"
#include "third_party/blink/renderer/core/style/border_image_length_box.h"
#include "third_party/blink/renderer/platform/geometry/length.h"
#include "third_party/blink/renderer/platform/geometry/length_box.h"

namespace blink {
namespace style_building_utils {

inline bool BorderImageLengthMatchesAllSides(
    const BorderImageLengthBox& border_image_length_box,
    const BorderImageLength& border_image_length) {
  return (border_image_length_box.Left() == border_image_length &&
          border_image_length_box.Right() == border_image_length &&
          border_image_length_box.Top() == border_image_length &&
          border_image_length_box.Bottom() == border_image_length);
}
inline bool LengthMatchesAllSides(const LengthBox& length_box,
                                  const Length& length) {
  return (length_box.Left() == length && length_box.Right() == length &&
          length_box.Top() == length && length_box.Bottom() == length);
}

}  // namespace style_building_utils
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_STYLE_BUILDING_UTILS_H_
