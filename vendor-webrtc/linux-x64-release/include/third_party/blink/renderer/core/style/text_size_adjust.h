// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_SIZE_ADJUST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_SIZE_ADJUST_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Value for text-size-adjust, see: https://drafts.csswg.org/css-size-adjust
class TextSizeAdjust {
  DISALLOW_NEW();

 public:
  TextSizeAdjust(float adjustment) : adjustment_(adjustment) {}

  // Negative values are invalid so we use them internally to signify 'auto'.
  static TextSizeAdjust AdjustAuto() { return TextSizeAdjust(-1); }
  // An adjustment of 'none' is equivalent to 100%.
  static TextSizeAdjust AdjustNone() { return TextSizeAdjust(1); }

  bool IsAuto() const { return adjustment_ < 0.f; }

  float Multiplier() const {
    // If the adjustment is 'auto', no multiplier is available.
    DCHECK(!IsAuto());
    return adjustment_;
  }

  bool operator==(const TextSizeAdjust& o) const {
    return adjustment_ == o.adjustment_;
  }

  bool operator!=(const TextSizeAdjust& o) const { return !(*this == o); }

 private:
  // Percent adjustment, without units (i.e., 10% is .1 and not 10). Negative
  // values indicate 'auto' adjustment.
  float adjustment_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_TEXT_SIZE_ADJUST_H_
