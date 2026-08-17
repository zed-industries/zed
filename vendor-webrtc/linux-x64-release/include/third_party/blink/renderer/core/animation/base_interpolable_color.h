// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_BASE_INTERPOLABLE_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_BASE_INTERPOLABLE_COLOR_H_

#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/animation/interpolable_value.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

class CORE_EXPORT BaseInterpolableColor : public InterpolableValue {
 public:
  virtual void Composite(const BaseInterpolableColor& other,
                         double underlying_fraction) = 0;

  virtual bool HasCurrentColorDependency() const = 0;

  void Trace(Visitor* v) const override { InterpolableValue::Trace(v); }

  bool Equals(const InterpolableValue& other) const final { NOTREACHED(); }

  virtual Color Resolve(const Color& current_color,
                        const Color& active_link_color,
                        const Color& link_color,
                        const Color& text_color,
                        mojom::blink::ColorScheme color_scheme) const = 0;
};

template <>
struct DowncastTraits<BaseInterpolableColor> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsColor() || interpolable_value.IsStyleColor();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_BASE_INTERPOLABLE_COLOR_H_
