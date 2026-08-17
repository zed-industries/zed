/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_COLOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_COLOR_H_

#include <memory>

#include "base/check.h"
#include "base/memory/values_equivalent.h"
#include "third_party/blink/public/mojom/frame/color_scheme.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace ui {
class ColorProvider;
}  // namespace ui

namespace blink {
class CalculationValue;
class CSSValue;

class CORE_EXPORT StyleColor {
  DISALLOW_NEW();

 public:
  // When color functions contain colors that cannot be resolved until used
  // value time (such as "currentcolor"), we need to store them here and
  // resolve them to individual colors later.
  class UnresolvedColorFunction;
  enum class UnderlyingColorType {
    kColor,
    kColorFunction,
    kCurrentColor,
  };
  struct ColorOrUnresolvedColorFunction {
    DISALLOW_NEW();

   public:
    ColorOrUnresolvedColorFunction() : color(Color::kTransparent) {}
    explicit ColorOrUnresolvedColorFunction(Color color) : color(color) {}
    explicit ColorOrUnresolvedColorFunction(
        const UnresolvedColorFunction* color_function)
        : unresolved_color_function(color_function) {}

    CORE_EXPORT void Trace(Visitor*) const;

    static bool Equals(const ColorOrUnresolvedColorFunction& first,
                       const ColorOrUnresolvedColorFunction& second,
                       UnderlyingColorType type) {
      switch (type) {
        case UnderlyingColorType::kCurrentColor:
          return true;

        case UnderlyingColorType::kColor:
          return first.color == second.color;

        case UnderlyingColorType::kColorFunction:
          return *first.unresolved_color_function ==
                 *second.unresolved_color_function;
      }
    }

    Color color;
    Member<const UnresolvedColorFunction> unresolved_color_function;
  };

  class CORE_EXPORT UnresolvedColorFunction
      : public GarbageCollected<UnresolvedColorFunction> {
   public:
    virtual void Trace(Visitor* vistor) const {}
    virtual CSSValue* ToCSSValue() const = 0;

    // Resolves the function to a Color without making any assumption about
    // where this function might appear in an expression tree. For example,
    // the output value should *not* undergo any conversion of `none` values
    // to zero if said `none` should propagate to the input of a containing
    // function.
    virtual Color Resolve(const Color& current_color) const = 0;

    enum class Type { kColorMix, kRelativeColor };
    Type GetType() const { return type_; }

    bool operator==(const UnresolvedColorFunction& other) const;

   protected:
    explicit UnresolvedColorFunction(Type type) : type_(type) {}
    const Type type_;
  };

  class CORE_EXPORT UnresolvedColorMix : public UnresolvedColorFunction {
   public:
    UnresolvedColorMix(Color::ColorSpace color_interpolation_space,
                       Color::HueInterpolationMethod hue_interpolation_method,
                       const StyleColor& c1,
                       const StyleColor& c2,
                       double percentage,
                       double alpha_multiplier);

    void Trace(Visitor* visitor) const override {
      UnresolvedColorFunction::Trace(visitor);
      visitor->Trace(color1_);
      visitor->Trace(color2_);
    }

    CSSValue* ToCSSValue() const override;

    Color Resolve(const Color& current_color) const override;

    bool operator==(const UnresolvedColorMix& other) const {
      if (color_interpolation_space_ != other.color_interpolation_space_ ||
          hue_interpolation_method_ != other.hue_interpolation_method_ ||
          percentage_ != other.percentage_ ||
          alpha_multiplier_ != other.alpha_multiplier_ ||
          color1_type_ != other.color1_type_ ||
          color2_type_ != other.color2_type_) {
        return false;
      }
      return ColorOrUnresolvedColorFunction::Equals(color1_, other.color1_,
                                                    color1_type_) &&
             ColorOrUnresolvedColorFunction::Equals(color2_, other.color2_,
                                                    color2_type_);
    }

    bool operator!=(const UnresolvedColorMix& other) const {
      return !(*this == other);
    }

   private:
    Color::ColorSpace color_interpolation_space_ = Color::ColorSpace::kNone;
    Color::HueInterpolationMethod hue_interpolation_method_ =
        Color::HueInterpolationMethod::kShorter;
    ColorOrUnresolvedColorFunction color1_;
    ColorOrUnresolvedColorFunction color2_;
    double percentage_ = 0.0;
    double alpha_multiplier_ = 1.0;
    UnderlyingColorType color1_type_ = UnderlyingColorType::kColor;
    UnderlyingColorType color2_type_ = UnderlyingColorType::kColor;
  };

  class CORE_EXPORT UnresolvedRelativeColor : public UnresolvedColorFunction {
   public:
    UnresolvedRelativeColor(const StyleColor& origin_color,
                            Color::ColorSpace color_interpolation_space,
                            const CSSValue& channel0,
                            const CSSValue& channel1,
                            const CSSValue& channel2,
                            const CSSValue* alpha);
    virtual ~UnresolvedRelativeColor() = default;
    void Trace(Visitor* visitor) const override;
    CSSValue* ToCSSValue() const override;
    Color Resolve(const Color& current_color) const override;
    bool operator==(const UnresolvedRelativeColor& other) const;

   private:
    ColorOrUnresolvedColorFunction origin_color_;
    UnderlyingColorType origin_color_type_ = UnderlyingColorType::kColor;

    Color::ColorSpace color_interpolation_space_ = Color::ColorSpace::kNone;

    bool alpha_was_specified_ = false;

    // nullptr on any of these fields represents `none`.
    scoped_refptr<const CalculationValue> channel0_;
    scoped_refptr<const CalculationValue> channel1_;
    scoped_refptr<const CalculationValue> channel2_;
    scoped_refptr<const CalculationValue> alpha_;
  };

  StyleColor() = default;
  explicit StyleColor(Color color)
      : color_keyword_(CSSValueID::kInvalid),
        color_or_unresolved_color_function_(color) {}
  explicit StyleColor(CSSValueID keyword) : color_keyword_(keyword) {}
  explicit StyleColor(const UnresolvedColorFunction* color_function)
      : color_keyword_(CSSValueID::kInvalid),
        color_or_unresolved_color_function_(color_function) {}
  // We need to store the color and keyword for system colors to be able to
  // distinguish system colors from a normal color. System colors won't be
  // overridden by forced colors mode, even if forced-color-adjust is 'auto'.
  StyleColor(Color color, CSSValueID keyword)
      : color_keyword_(keyword), color_or_unresolved_color_function_(color) {}

  void Trace(Visitor* visitor) const {
    visitor->Trace(color_or_unresolved_color_function_);
  }

  static StyleColor CurrentColor() { return StyleColor(); }

  bool IsCurrentColor() const {
    return color_keyword_ == CSSValueID::kCurrentcolor;
  }
  bool IsUnresolvedColorFunction() const {
    return color_or_unresolved_color_function_.unresolved_color_function !=
           nullptr;
  }
  bool DependsOnCurrentColor() const {
    return IsCurrentColor() || IsUnresolvedColorFunction();
  }
  bool IsSystemColorIncludingDeprecated() const {
    return IsSystemColorIncludingDeprecated(color_keyword_);
  }
  bool IsSystemColor() const { return IsSystemColor(color_keyword_); }
  bool IsAbsoluteColor() const {
    return !IsCurrentColor() && !IsUnresolvedColorFunction();
  }
  Color GetColor() const;

  CSSValueID GetColorKeyword() const {
    DCHECK(!IsNumeric());
    return color_keyword_;
  }
  bool HasColorKeyword() const {
    return color_keyword_ != CSSValueID::kInvalid;
  }

  // Resolves this StyleColor to an output color in its final form per spec.
  Color Resolve(const Color& current_color,
                mojom::blink::ColorScheme color_scheme,
                bool* is_current_color = nullptr) const;

  // Resolve and override the resolved color's alpha channel as specified by
  // |alpha|.
  Color ResolveWithAlpha(Color current_color,
                         mojom::blink::ColorScheme color_scheme,
                         int alpha,
                         bool* is_current_color = nullptr) const;

  // Re-resolve the current system color keyword. This is needed in cases such
  // as forced colors mode because initial values for some internal forced
  // colors properties are system colors so we need to re-resolve them to ensure
  // they pick up the correct color on theme change.
  StyleColor ResolveSystemColor(mojom::blink::ColorScheme color_scheme,
                                const ui::ColorProvider* color_provider,
                                bool is_in_web_app_scope) const;

  const CSSValue* ToCSSValue() const;

  bool IsNumeric() const {
    return EffectiveColorKeyword() == CSSValueID::kInvalid;
  }

  static Color ColorFromKeyword(CSSValueID,
                                mojom::blink::ColorScheme color_scheme,
                                const ui::ColorProvider* color_provider,
                                bool is_in_web_app_scope);
  static bool IsColorKeyword(CSSValueID);
  static bool IsSystemColorIncludingDeprecated(CSSValueID);
  static bool IsSystemColor(CSSValueID);

  inline bool operator==(const StyleColor& other) const {
    if (color_keyword_ != other.color_keyword_) {
      return false;
    }

    if (IsUnresolvedColorFunction() || other.IsUnresolvedColorFunction()) {
      return base::ValuesEquivalent(
          color_or_unresolved_color_function_.unresolved_color_function,
          other.color_or_unresolved_color_function_.unresolved_color_function);
    }

    return color_or_unresolved_color_function_.color ==
           other.color_or_unresolved_color_function_.color;
  }

  inline bool operator!=(const StyleColor& other) const {
    return !(*this == other);
  }

 protected:
  CSSValueID color_keyword_ = CSSValueID::kCurrentcolor;
  ColorOrUnresolvedColorFunction color_or_unresolved_color_function_;

 private:
  // An UnresolvedColorFunction may be directly accessed immediately after it's
  // been built at computed-value time, so that sub-expressions can be resolved
  // in context. However, once an UnresolvedColorFunction has been embedded
  // into a StyleColor, we should only resolve the entire expression tree at
  // once, which is accomplished by calling StyleColor::Resolve().
  // Thus, only test/debug code and other instances of StyleColor, which need to
  // determine equality, should reach for an UnresolvedColorFunction once it's
  // been embedded into a StyleColor.
  friend CORE_EXPORT std::ostream& operator<<(std::ostream& stream,
                                              const StyleColor& color);
  const UnresolvedColorFunction& GetUnresolvedColorFunction() const {
    DCHECK(IsUnresolvedColorFunction());
    return *color_or_unresolved_color_function_.unresolved_color_function;
  }

  CSSValueID EffectiveColorKeyword() const;
};

template <>
struct DowncastTraits<StyleColor::UnresolvedColorMix> {
  static bool AllowFrom(const StyleColor::UnresolvedColorFunction& value) {
    return value.GetType() ==
           StyleColor::UnresolvedColorFunction::Type::kColorMix;
  }
};

template <>
struct DowncastTraits<StyleColor::UnresolvedRelativeColor> {
  static bool AllowFrom(const StyleColor::UnresolvedColorFunction& value) {
    return value.GetType() ==
           StyleColor::UnresolvedColorFunction::Type::kRelativeColor;
  }
};

// For debugging only.
CORE_EXPORT std::ostream& operator<<(std::ostream& stream,
                                     const StyleColor& color);
CORE_EXPORT std::ostream& operator<<(
    std::ostream& stream,
    const StyleColor::UnresolvedColorFunction& unresolved_color_function);

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::StyleColor)
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_STYLE_COLOR_H_
