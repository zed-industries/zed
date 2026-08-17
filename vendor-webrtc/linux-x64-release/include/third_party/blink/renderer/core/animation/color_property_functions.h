// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COLOR_PROPERTY_FUNCTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COLOR_PROPERTY_FUNCTIONS_H_

#include <optional>

#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/style_color.h"

namespace blink {

class ComputedStyle;
class ComputedStyleBuilder;
class CSSProperty;

class OptionalStyleColor {
  DISALLOW_NEW();

 public:
  explicit OptionalStyleColor(const StyleColor& value)
      : has_value_(true), value_(value) {}
  OptionalStyleColor() = default;

  bool has_value() const { return has_value_; }

  const StyleColor& value() const {
    DCHECK(has_value_);
    return value_;
  }

  bool operator==(const OptionalStyleColor& other) const {
    return has_value_ == other.has_value_ && value_ == other.value_;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(value_); }

 private:
  bool has_value_ = false;
  StyleColor value_;
};

class ColorPropertyFunctions {
 public:
  static OptionalStyleColor GetInitialColor(const CSSProperty&,
                                            const ComputedStyle& initial_style);
  template <typename ComputedStyleOrBuilder>
  static OptionalStyleColor GetUnvisitedColor(const CSSProperty&,
                                              const ComputedStyleOrBuilder&);
  template <typename ComputedStyleOrBuilder>
  static OptionalStyleColor GetVisitedColor(const CSSProperty&,
                                            const ComputedStyleOrBuilder&);
  static void SetUnvisitedColor(const CSSProperty&,
                                ComputedStyleBuilder&,
                                const Color&);
  static void SetVisitedColor(const CSSProperty&,
                              ComputedStyleBuilder&,
                              const Color&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_COLOR_PROPERTY_FUNCTIONS_H_
