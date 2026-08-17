// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_QUAD_LENGTH_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_QUAD_LENGTH_VALUE_H_

#include "third_party/blink/renderer/platform/geometry/length.h"

namespace blink {

struct QuadLengthValue {
  DISALLOW_NEW();

  QuadLengthValue() {}

  explicit QuadLengthValue(Length length)
      : top(length), right(length), bottom(length), left(length) {}

  QuadLengthValue(const QuadLengthValue& other)
      : top(other.top),
        right(other.right),
        bottom(other.bottom),
        left(other.left) {}

  bool operator==(const QuadLengthValue& other) const {
    return top == other.top && right == other.right && bottom == other.bottom &&
           left == other.left;
  }

  bool operator!=(const QuadLengthValue& other) const {
    return !(*this == other);
  }

  Length top;
  Length right;
  Length bottom;
  Length left;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_QUAD_LENGTH_VALUE_H_
