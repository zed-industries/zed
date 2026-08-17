// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SUPERELLIPSE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SUPERELLIPSE_H_

#include <cmath>
#include <limits>

#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace blink {

// Represents a superellipse, as defined in
// https://drafts.csswg.org/css-borders-4/#funcdef-superellipse
class Superellipse {
 public:
  static constexpr float kHighCurvatureThreshold = 16;

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-bevel
  static constexpr Superellipse Bevel() { return Superellipse(0); }

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-notch
  static constexpr Superellipse Notch() {
    return Superellipse(std::numeric_limits<double>::lowest());
  }

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-round
  static constexpr Superellipse Round() { return Superellipse(1); }

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-scoop
  static constexpr Superellipse Scoop() { return Superellipse(-1); }

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-squircle
  static constexpr Superellipse Squircle() { return Superellipse(2); }

  // https://drafts.csswg.org/css-borders-4/#valdef-corner-shape-value-square
  static constexpr Superellipse Square() {
    return Superellipse(std::numeric_limits<double>::max());
  }

  // Very high curvatures are counted as straight as there would be no visual
  // effect. "Degenerate" means that the corner should be considered to have a
  // zero-size rather than consider its size and curvature.
  constexpr bool IsDegenerate() const {
    return param_ >= kHighCurvatureThreshold;
  }
  constexpr bool IsFullyConcave() const {
    return param_ <= -kHighCurvatureThreshold;
  }
  constexpr bool IsConvex() const { return param_ >= 0; }

  constexpr explicit Superellipse(double param) : param_(param) {}

  // https://drafts.csswg.org/css-borders-4/#superellipse-param
  constexpr double Parameter() const { return param_; }

  constexpr double Exponent() const {
    return std::pow(2, ClampTo<float>(param_, -kHighCurvatureThreshold,
                                      kHighCurvatureThreshold));
  }

  bool operator==(const Superellipse& other) const = default;

 private:
  double param_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_SUPERELLIPSE_H_
