// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_STYLE_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_STYLE_COLOR_H_

#include "third_party/blink/renderer/core/animation/base_interpolable_color.h"
#include "third_party/blink/renderer/core/animation/interpolable_color.h"
#include "third_party/blink/renderer/core/css/style_color.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

/**
 * InterpolableStyleColor is used with interpolating to or from an unresolved
 * color-mix.
 */
class CORE_EXPORT InterpolableStyleColor : public BaseInterpolableColor {
 public:
  InterpolableStyleColor() = default;
  explicit InterpolableStyleColor(StyleColor style_color)
      : style_color_(style_color) {}

  explicit InterpolableStyleColor(
      const BaseInterpolableColor* interpolable_color)
      : from_color_(interpolable_color) {}

  virtual ~InterpolableStyleColor() = default;

  static InterpolableStyleColor* Create(StyleColor style_color) {
    return MakeGarbageCollected<InterpolableStyleColor>(style_color);
  }

  static InterpolableStyleColor* Create(
      const BaseInterpolableColor* interpolable_color) {
    return MakeGarbageCollected<InterpolableStyleColor>(interpolable_color);
  }

  static InterpolableStyleColor* Create(const InterpolableValue& value) {
    const auto& color = To<BaseInterpolableColor>(value);
    return Create(&color);
  }

  void Trace(Visitor* v) const override {
    BaseInterpolableColor::Trace(v);
    v->Trace(style_color_);
    v->Trace(from_color_);
    v->Trace(to_color_);
  }

  bool IsStyleColor() const final { return true; }

  bool HasCurrentColorDependency() const override { return true; }

  void Composite(const BaseInterpolableColor& other, double fraction) final;

  Color Resolve(const Color& current_color,
                const Color& active_link_color,
                const Color& link_color,
                const Color& text_color,
                mojom::blink::ColorScheme color_scheme) const override;

  void Interpolate(const InterpolableValue& to,
                   const double progress,
                   InterpolableValue& result) const final;

  static void Interpolate(const InterpolableValue& from,
                          const InterpolableValue& to,
                          double progress,
                          InterpolableValue& result);

  void Scale(double scale) override;
  void Add(const InterpolableValue& other) override;

  InterpolableValue* RawClone() const override {
    InterpolableStyleColor* clone =
        MakeGarbageCollected<InterpolableStyleColor>(style_color_);
    clone->from_color_ = from_color_;
    clone->to_color_ = to_color_;
    clone->fraction_ = fraction_;
    return clone;
  }

  InterpolableValue* RawCloneAndZero() const override {
    return MakeGarbageCollected<InterpolableStyleColor>();
  }
  void AssertCanInterpolateWith(const InterpolableValue& other) const override {
    DCHECK(other.IsStyleColor());
  }

  const StyleColor& style_color() const { return style_color_; }

  enum class BlendOp { kBase, kInterpolate, kComposite };

 private:
  StyleColor style_color_;
  Member<const BaseInterpolableColor> from_color_;
  Member<const BaseInterpolableColor> to_color_;
  // Fraction indicates progress when interpolating or underlying amount when
  // compositing.
  double fraction_ = 0;
  BlendOp blend_op_ = BlendOp::kBase;
};

template <>
struct DowncastTraits<InterpolableStyleColor> {
  static bool AllowFrom(const InterpolableValue& interpolable_value) {
    return interpolable_value.IsStyleColor();
  }
  static bool AllowFrom(const BaseInterpolableColor& base) {
    return base.IsStyleColor();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANIMATION_INTERPOLABLE_STYLE_COLOR_H_
