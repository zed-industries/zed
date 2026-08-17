// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_REPEAT_COUNT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_REPEAT_COUNT_H_

#include <cmath>
#include <limits>

#include "base/check_op.h"

namespace blink {

// Representation of the value from the 'repeatCount' SMIL attribute.
//
// "Unspecified" is used to indicate that the attribute is not specified.
// "Invalid" is used to indicate that the attribute may have changed and needs
// to be reparsed.
class SMILRepeatCount {
 public:
  static SMILRepeatCount Unspecified() {
    return SMILRepeatCount(std::numeric_limits<double>::quiet_NaN());
  }
  static SMILRepeatCount Indefinite() {
    return SMILRepeatCount(std::numeric_limits<double>::infinity());
  }
  static SMILRepeatCount Numeric(double value) {
    DCHECK(std::isfinite(value));
    DCHECK_GT(value, 0);
    return SMILRepeatCount(value);
  }
  static SMILRepeatCount Invalid() {
    return SMILRepeatCount(-std::numeric_limits<double>::infinity());
  }

  bool IsValid() const {
    return value_ != -std::numeric_limits<double>::infinity();
  }
  bool IsUnspecified() const {
    DCHECK(IsValid());
    return std::isnan(value_);
  }
  bool IsIndefinite() const {
    DCHECK(IsValid());
    return std::isinf(value_);
  }
  double NumericValue() const {
    DCHECK(!IsUnspecified());
    DCHECK(!IsIndefinite());
    DCHECK(IsValid());
    return value_;
  }

 private:
  explicit SMILRepeatCount(double value) : value_(value) {}

  double value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_ANIMATION_SMIL_REPEAT_COUNT_H_
