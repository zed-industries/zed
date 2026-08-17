/*
    Copyright (C) 1999 Lars Knoll (knoll@kde.org)
    Copyright (C) 2006, 2008 Apple Inc. All rights reserved.
    Copyright (C) 2011 Rik Cabanier (cabanier@adobe.com)
    Copyright (C) 2011 Adobe Systems Incorporated. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_H_

#include <cmath>
#include <cstring>
#include <optional>

#include "base/check_op.h"
#include "base/compiler_specific.h"
#include "base/memory/stack_allocated.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/platform/geometry/evaluation_input.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/math_extras.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

struct PixelsAndPercent {
  DISALLOW_NEW();
  explicit PixelsAndPercent(float pixels)
      : pixels(pixels),
        percent(0.0f),
        has_explicit_pixels(true),
        has_explicit_percent(false) {}
  PixelsAndPercent(float pixels,
                   float percent,
                   bool has_explicit_pixels,
                   bool has_explicit_percent)
      : pixels(pixels),
        percent(percent),
        has_explicit_pixels(has_explicit_pixels),
        has_explicit_percent(has_explicit_percent) {}

  PixelsAndPercent& operator+=(const PixelsAndPercent& rhs) {
    pixels += rhs.pixels;
    percent += rhs.percent;
    has_explicit_pixels |= rhs.has_explicit_pixels;
    has_explicit_percent |= rhs.has_explicit_percent;
    return *this;
  }
  friend PixelsAndPercent operator+(PixelsAndPercent lhs,
                                    const PixelsAndPercent& rhs) {
    lhs += rhs;
    return lhs;
  }
  PixelsAndPercent& operator-=(const PixelsAndPercent& rhs) {
    pixels -= rhs.pixels;
    percent -= rhs.percent;
    has_explicit_pixels |= rhs.has_explicit_pixels;
    has_explicit_percent |= rhs.has_explicit_percent;
    return *this;
  }
  PixelsAndPercent& operator*=(float number) {
    pixels *= number;
    percent *= number;
    return *this;
  }

  float pixels;
  float percent;
  bool has_explicit_pixels;
  bool has_explicit_percent;
};

class CalculationValue;
class Length;

PLATFORM_EXPORT extern const Length& g_auto_length;
PLATFORM_EXPORT extern const Length& g_fill_available_length;
PLATFORM_EXPORT extern const Length& g_stretch_length;
PLATFORM_EXPORT extern const Length& g_fit_content_length;
PLATFORM_EXPORT extern const Length& g_max_content_length;
PLATFORM_EXPORT extern const Length& g_min_content_length;
PLATFORM_EXPORT extern const Length& g_min_intrinsic_length;

class PLATFORM_EXPORT Length {
  DISALLOW_NEW();

 public:
  // Initializes global instances.
  static void Initialize();

  enum class ValueRange { kAll, kNonNegative };

  // FIXME: This enum makes it hard to tell in general what values may be
  // appropriate for any given Length.
  enum Type : unsigned char {
    kAuto,
    kPercent,
    kFixed,
    kMinContent,
    kMaxContent,
    kMinIntrinsic,
    kFillAvailable,
    kStretch,
    kFitContent,
    kCalculated,
    kFlex,
    kExtendToZoom,
    kDeviceWidth,
    kDeviceHeight,
    kNone,    // only valid for max-width, max-height, or contain-intrinsic-size
    kContent  // only valid for flex-basis
  };

  Length() : value_(0), type_(kAuto) {}

  explicit Length(Length::Type t) : value_(0), type_(t) {
    DCHECK_NE(t, kCalculated);
  }

  Length(int v, Length::Type t) : value_(v), type_(t) {
    DCHECK_NE(t, kCalculated);
  }

  Length(LayoutUnit v, Length::Type t) : value_(v.ToFloat()), type_(t) {
    DCHECK(std::isfinite(v.ToFloat()));
    DCHECK_NE(t, kCalculated);
  }

  Length(float v, Length::Type t) : value_(v), type_(t) {
    DCHECK(std::isfinite(v));
    DCHECK_NE(t, kCalculated);
  }

  Length(double v, Length::Type t) : type_(t) {
    DCHECK(std::isfinite(v));
    value_ = ClampTo<float>(v);
  }

  explicit Length(scoped_refptr<const CalculationValue>);

  Length(const Length& length) {
    UNSAFE_TODO(memcpy(this, &length, sizeof(Length)));
    if (IsCalculated())
      IncrementCalculatedRef();
  }

  Length& operator=(const Length& length) {
    if (length.IsCalculated())
      length.IncrementCalculatedRef();
    if (IsCalculated())
      DecrementCalculatedRef();
    UNSAFE_TODO(memcpy(this, &length, sizeof(Length)));
    return *this;
  }

  ~Length() {
    if (IsCalculated())
      DecrementCalculatedRef();
  }

  bool operator==(const Length& o) const {
    if (type_ != o.type_ || quirk_ != o.quirk_) {
      return false;
    }
    if (type_ == kCalculated) {
      return IsCalculatedEqual(o);
    } else {
      // For everything that doesn't use value_, it is defined to be zero,
      // so we can compare here unconditionally.
      return value_ == o.value_;
    }
  }
  bool operator!=(const Length& o) const { return !(*this == o); }

  static const Length& Auto() { return g_auto_length; }
  static const Length& FillAvailable() { return g_fill_available_length; }
  static const Length& Stretch() { return g_stretch_length; }
  static const Length& FitContent() { return g_fit_content_length; }
  static const Length& MaxContent() { return g_max_content_length; }
  static const Length& MinContent() { return g_min_content_length; }
  static const Length& MinIntrinsic() { return g_min_intrinsic_length; }

  static Length Content() { return Length(kContent); }
  static Length Fixed() { return Length(kFixed); }
  static Length None() { return Length(kNone); }

  static Length ExtendToZoom() { return Length(kExtendToZoom); }
  static Length DeviceWidth() { return Length(kDeviceWidth); }
  static Length DeviceHeight() { return Length(kDeviceHeight); }

  template <typename NUMBER_TYPE>
  static Length Fixed(NUMBER_TYPE number) {
    return Length(number, kFixed);
  }
  template <typename NUMBER_TYPE>
  static Length Percent(NUMBER_TYPE number) {
    return Length(number, kPercent);
  }
  static Length Flex(float value) { return Length(value, kFlex); }

  int IntValue() const {
    if (IsCalculated()) {
      NOTREACHED();
    }
    DCHECK(!IsNone());
    return static_cast<int>(value_);
  }

  float Pixels() const {
    DCHECK_EQ(GetType(), kFixed);
    return GetFloatValue();
  }
  float Percent() const {
    DCHECK_EQ(GetType(), kPercent);
    return GetFloatValue();
  }
  float Flex() const {
    DCHECK_EQ(GetType(), kFlex);
    return GetFloatValue();
  }

  PixelsAndPercent GetPixelsAndPercent() const;

  const CalculationValue& GetCalculationValue() const;

  // If |this| is calculated, returns the underlying |CalculationValue|. If not,
  // returns a |CalculationValue| constructed from |GetPixelsAndPercent()|. Hits
  // a DCHECK if |this| is not a specified value (e.g., 'auto').
  scoped_refptr<const CalculationValue> AsCalculationValue() const;

  Length::Type GetType() const { return static_cast<Length::Type>(type_); }
  bool Quirk() const { return quirk_; }

  void SetQuirk(bool quirk) { quirk_ = quirk; }

  bool IsNone() const { return GetType() == kNone; }

  // FIXME calc: https://bugs.webkit.org/show_bug.cgi?id=80357. A calculated
  // Length always contains a percentage, and without a maxValue passed to these
  // functions it's impossible to determine the sign or zero-ness. We assume all
  // calc values are positive and non-zero for now.
  bool IsZero() const {
    DCHECK(!IsNone());
    if (IsCalculated())
      return false;

    return !value_;
  }

  // If this is a length in a property that accepts calc-size(), use
  // |HasAuto()|.  If this |Length| is a block-axis size
  // |HasAutoOrContentOrIntrinsic()| is usually a better choice.
  bool IsAuto() const { return GetType() == kAuto; }
  bool IsFixed() const { return GetType() == kFixed; }

  // For the block axis, intrinsic sizes such as `min-content` behave the same
  // as `auto`. https://www.w3.org/TR/css-sizing-3/#valdef-width-min-content
  // This includes content-based sizes in calc-size().
  bool HasAuto() const;
  bool HasContentOrIntrinsic() const;
  bool HasAutoOrContentOrIntrinsic() const;
  // HasPercent and HasPercentOrStretch refer to whether the toplevel value
  // should be treated as a percentage type for web-exposed behavior
  // decisions.  However, a value can still depend on a percentage when
  // HasPercent() is false:  for example, calc-size(any, 20%).
  bool HasPercent() const;
  bool HasPercentOrStretch() const;
  bool HasStretch() const;

  bool HasMinContent() const;
  bool HasMaxContent() const;
  bool HasMinIntrinsic() const { return IsMinIntrinsic(); }
  bool HasFitContent() const;

  // CanConvertToCalculation() is true for any Lengths that are a fixed length,
  // a percent, or a calc() expression.  Note that this *includes* calc-size()
  // expressions that contain sizing keywords, which may not be what you want.
  //
  // Compare to HasOnlyFixedAndPercent.  (The difference is relevant only when
  // sizing keywords may be present.)
  //
  // Note that in some contexts sizing keywords can be converted to
  // calculation expressions, but this function does *not* return true for
  // those cases; the caller is required to convert appropriately.
  bool CanConvertToCalculation() const {
    return GetType() == kFixed || GetType() == kPercent ||
           GetType() == kCalculated;
  }

  // HasOnlyFixedAndPercent() is true for any Lengths that are a fixed length,
  // a percent, or calc() expressions that consist only of those.  (This
  // excludes calc() expressions with calc-size() that depend on sizing
  // keywords.)
  // Compare to CanConvertToCalculation.  (The difference is relevant only
  // when sizing keywords may be present.)
  bool HasOnlyFixedAndPercent() const;

  bool IsCalculated() const { return GetType() == kCalculated; }
  bool IsCalculatedEqual(const Length&) const;

  // These type checking methods should be used with extreme caution;
  // many uses probably want the Has* methods above to work correctly
  // with calc-size().
  bool IsMinContent() const { return GetType() == kMinContent; }
  bool IsMaxContent() const { return GetType() == kMaxContent; }
  bool IsMinIntrinsic() const { return GetType() == kMinIntrinsic; }
  bool IsFillAvailable() const { return GetType() == kFillAvailable; }
  bool IsStretch() const { return GetType() == kStretch; }
  bool IsFitContent() const { return GetType() == kFitContent; }
  bool IsPercent() const { return GetType() == kPercent; }
  // MayHavePercentDependence should be used to decide whether to optimize
  // away computing the value on which percentages depend or optimize away
  // recomputation that results from changes to that value.  It is intended to
  // be used *only* in cases where the implementation could be changed to one
  // that returns true only if there are percentage values somewhere in the
  // expression (that is, one that still returns true for calc-size(any, 30%)
  // for which HasPercent() is false, but is false for calc-size(any, 30px)).
  //
  // We could (if we want) make this exact and remove "May" from the name.
  // But this would require looking into the calculation value like HasPercent
  // does.  However, it needs to be different from HasPercent because of cases
  // where calc-size() erases percentage-ness from the type, like
  // calc-size(any, 20%).
  //
  // For properties that cannot have calc-size in them, we currently use
  // HasPercent() rather than MayHavePercentDependence() since it's a
  // shorter/simpler function name, and the two functions are equivalent in
  // that case.
  bool MayHavePercentDependence() const {
    return GetType() == kPercent || GetType() == kCalculated;
  }
  bool IsFlex() const { return GetType() == kFlex; }
  bool IsExtendToZoom() const { return GetType() == kExtendToZoom; }
  bool IsDeviceWidth() const { return GetType() == kDeviceWidth; }
  bool IsDeviceHeight() const { return GetType() == kDeviceHeight; }

  Length Blend(const Length& from, double progress, ValueRange range) const {
    DCHECK(CanConvertToCalculation());
    DCHECK(from.CanConvertToCalculation());

    if (progress == 0.0)
      return from;

    if (progress == 1.0)
      return *this;

    if (from.GetType() == kCalculated || GetType() == kCalculated)
      return BlendMixedTypes(from, progress, range);

    if (!from.IsZero() && !IsZero() && from.GetType() != GetType())
      return BlendMixedTypes(from, progress, range);

    if (from.IsZero() && IsZero())
      return *this;

    return BlendSameTypes(from, progress, range);
  }

  float NonNanCalculatedValue(float max_value, const EvaluationInput&) const;

  Length SubtractFromOneHundredPercent() const;

  Length Add(const Length& other) const;

  Length Zoom(double factor) const;

  WTF::String ToString() const;

 private:
  float GetFloatValue() const {
    DCHECK(!IsNone());
    DCHECK(!IsCalculated());
    return value_;
  }

  Length BlendMixedTypes(const Length& from, double progress, ValueRange) const;

  Length BlendSameTypes(const Length& from, double progress, ValueRange) const;

  int CalculationHandle() const {
    DCHECK(IsCalculated());
    return calculation_handle_;
  }
  void IncrementCalculatedRef() const;
  void DecrementCalculatedRef() const;

  union {
    // If kType == kCalculated.
    int calculation_handle_;

    // Otherwise. Must be zero if not in use (e.g., for kAuto or kNone).
    float value_;
  };
  bool quirk_ = false;
  unsigned char type_;
};

PLATFORM_EXPORT std::ostream& operator<<(std::ostream&, const Length&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GEOMETRY_LENGTH_H_
