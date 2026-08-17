/*
 * Copyright (C) 2011 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_BASIC_SHAPE_VALUES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_BASIC_SHAPE_VALUES_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/css_primitive_value.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_value_pair.h"
#include "third_party/blink/renderer/platform/geometry/path_types.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
namespace cssvalue {

class CSSBasicShapeCircleValue final : public CSSValue {
 public:
  CSSBasicShapeCircleValue() : CSSValue(kBasicShapeCircleClass) {}

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapeCircleValue&) const;

  CSSValue* CenterX() const { return center_x_.Get(); }
  CSSValue* CenterY() const { return center_y_.Get(); }
  CSSValue* Radius() const { return radius_.Get(); }

  // TODO(sashab): Remove these and pass them as arguments in the constructor.
  void SetCenterX(CSSValue* center_x) { center_x_ = center_x; }
  void SetCenterY(CSSValue* center_y) { center_y_ = center_y; }
  void SetRadius(CSSValue* radius) { radius_ = radius; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<CSSValue> center_x_;
  Member<CSSValue> center_y_;
  Member<CSSValue> radius_;
};

class CSSBasicShapeEllipseValue final : public CSSValue {
 public:
  CSSBasicShapeEllipseValue() : CSSValue(kBasicShapeEllipseClass) {}

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapeEllipseValue&) const;

  CSSValue* CenterX() const { return center_x_.Get(); }
  CSSValue* CenterY() const { return center_y_.Get(); }
  CSSValue* RadiusX() const { return radius_x_.Get(); }
  CSSValue* RadiusY() const { return radius_y_.Get(); }

  // TODO(sashab): Remove these and pass them as arguments in the constructor.
  void SetCenterX(CSSValue* center_x) { center_x_ = center_x; }
  void SetCenterY(CSSValue* center_y) { center_y_ = center_y; }
  void SetRadiusX(CSSValue* radius_x) { radius_x_ = radius_x; }
  void SetRadiusY(CSSValue* radius_y) { radius_y_ = radius_y; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<CSSValue> center_x_;
  Member<CSSValue> center_y_;
  Member<CSSValue> radius_x_;
  Member<CSSValue> radius_y_;
};

class CSSBasicShapePolygonValue final : public CSSValue {
 public:
  CSSBasicShapePolygonValue()
      : CSSValue(kBasicShapePolygonClass), wind_rule_(RULE_NONZERO) {}

  void AppendPoint(CSSPrimitiveValue* x, CSSPrimitiveValue* y) {
    values_.push_back(x);
    values_.push_back(y);
  }

  CSSPrimitiveValue* GetXAt(unsigned i) const {
    return values_.at(i * 2).Get();
  }
  CSSPrimitiveValue* GetYAt(unsigned i) const {
    return values_.at(i * 2 + 1).Get();
  }
  const HeapVector<Member<CSSPrimitiveValue>>& Values() const {
    return values_;
  }

  // TODO(sashab): Remove this and pass it as an argument in the constructor.
  void SetWindRule(WindRule w) { wind_rule_ = w; }
  WindRule GetWindRule() const { return wind_rule_; }

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapePolygonValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  HeapVector<Member<CSSPrimitiveValue>> values_;
  WindRule wind_rule_;
};

class CSSBasicShapeInsetValue final : public CSSValue {
 public:
  CSSBasicShapeInsetValue() : CSSValue(kBasicShapeInsetClass) {}

  CSSValue* Top() const { return top_.Get(); }
  CSSValue* Right() const { return right_.Get(); }
  CSSValue* Bottom() const { return bottom_.Get(); }
  CSSValue* Left() const { return left_.Get(); }

  CSSValuePair* TopLeftRadius() const { return top_left_radius_.Get(); }
  CSSValuePair* TopRightRadius() const { return top_right_radius_.Get(); }
  CSSValuePair* BottomRightRadius() const { return bottom_right_radius_.Get(); }
  CSSValuePair* BottomLeftRadius() const { return bottom_left_radius_.Get(); }

  // TODO(sashab): Remove these and pass them as arguments in the constructor.
  void SetTop(CSSValue* top) { top_ = top; }
  void SetRight(CSSValue* right) { right_ = right; }
  void SetBottom(CSSValue* bottom) { bottom_ = bottom; }
  void SetLeft(CSSValue* left) { left_ = left; }

  void UpdateShapeSize4Values(CSSValue* top,
                              CSSValue* right,
                              CSSValue* bottom,
                              CSSValue* left) {
    SetTop(top);
    SetRight(right);
    SetBottom(bottom);
    SetLeft(left);
  }

  void UpdateShapeSize1Value(CSSValue* value1) {
    UpdateShapeSize4Values(value1, value1, value1, value1);
  }

  void UpdateShapeSize2Values(CSSValue* value1, CSSValue* value2) {
    UpdateShapeSize4Values(value1, value2, value1, value2);
  }

  void UpdateShapeSize3Values(CSSValue* value1,
                              CSSValue* value2,
                              CSSValue* value3) {
    UpdateShapeSize4Values(value1, value2, value3, value2);
  }

  void SetTopLeftRadius(CSSValuePair* radius) { top_left_radius_ = radius; }
  void SetTopRightRadius(CSSValuePair* radius) { top_right_radius_ = radius; }
  void SetBottomRightRadius(CSSValuePair* radius) {
    bottom_right_radius_ = radius;
  }
  void SetBottomLeftRadius(CSSValuePair* radius) {
    bottom_left_radius_ = radius;
  }

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapeInsetValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<CSSValue> top_;
  Member<CSSValue> right_;
  Member<CSSValue> bottom_;
  Member<CSSValue> left_;

  Member<CSSValuePair> top_left_radius_;
  Member<CSSValuePair> top_right_radius_;
  Member<CSSValuePair> bottom_right_radius_;
  Member<CSSValuePair> bottom_left_radius_;
};

class CSSBasicShapeRectValue final : public CSSValue {
 public:
  CSSBasicShapeRectValue(CSSValue* top,
                         CSSValue* right,
                         CSSValue* bottom,
                         CSSValue* left)
      : CSSValue(kBasicShapeRectClass),
        top_(top),
        right_(right),
        bottom_(bottom),
        left_(left) {
    Validate();
  }

  CSSValue* Top() const { return top_.Get(); }
  CSSValue* Right() const { return right_.Get(); }
  CSSValue* Bottom() const { return bottom_.Get(); }
  CSSValue* Left() const { return left_.Get(); }

  CSSValuePair* TopLeftRadius() const { return top_left_radius_.Get(); }
  CSSValuePair* TopRightRadius() const { return top_right_radius_.Get(); }
  CSSValuePair* BottomRightRadius() const { return bottom_right_radius_.Get(); }
  CSSValuePair* BottomLeftRadius() const { return bottom_left_radius_.Get(); }

  void SetTopLeftRadius(CSSValuePair* radius) { top_left_radius_ = radius; }
  void SetTopRightRadius(CSSValuePair* radius) { top_right_radius_ = radius; }
  void SetBottomRightRadius(CSSValuePair* radius) {
    bottom_right_radius_ = radius;
  }
  void SetBottomLeftRadius(CSSValuePair* radius) {
    bottom_left_radius_ = radius;
  }

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapeRectValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  void Validate() const;

  Member<CSSValue> top_;
  Member<CSSValue> right_;
  Member<CSSValue> bottom_;
  Member<CSSValue> left_;

  Member<CSSValuePair> top_left_radius_;
  Member<CSSValuePair> top_right_radius_;
  Member<CSSValuePair> bottom_right_radius_;
  Member<CSSValuePair> bottom_left_radius_;
};

class CSSBasicShapeXYWHValue final : public CSSValue {
 public:
  CSSBasicShapeXYWHValue(CSSPrimitiveValue* x,
                         CSSPrimitiveValue* y,
                         CSSPrimitiveValue* width,
                         CSSPrimitiveValue* height)
      : CSSValue(kBasicShapeXYWHClass),
        x_(x),
        y_(y),
        width_(width),
        height_(height) {
    Validate();
  }

  CSSPrimitiveValue* X() const { return x_.Get(); }
  CSSPrimitiveValue* Y() const { return y_.Get(); }
  CSSPrimitiveValue* Width() const { return width_.Get(); }
  CSSPrimitiveValue* Height() const { return height_.Get(); }

  CSSValuePair* TopLeftRadius() const { return top_left_radius_.Get(); }
  CSSValuePair* TopRightRadius() const { return top_right_radius_.Get(); }
  CSSValuePair* BottomRightRadius() const { return bottom_right_radius_.Get(); }
  CSSValuePair* BottomLeftRadius() const { return bottom_left_radius_.Get(); }

  void SetTopLeftRadius(CSSValuePair* radius) { top_left_radius_ = radius; }
  void SetTopRightRadius(CSSValuePair* radius) { top_right_radius_ = radius; }
  void SetBottomRightRadius(CSSValuePair* radius) {
    bottom_right_radius_ = radius;
  }
  void SetBottomLeftRadius(CSSValuePair* radius) {
    bottom_left_radius_ = radius;
  }

  String CustomCSSText() const;
  bool Equals(const CSSBasicShapeXYWHValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  void Validate() const;

  Member<CSSPrimitiveValue> x_;
  Member<CSSPrimitiveValue> y_;
  Member<CSSPrimitiveValue> width_;
  Member<CSSPrimitiveValue> height_;

  Member<CSSValuePair> top_left_radius_;
  Member<CSSValuePair> top_right_radius_;
  Member<CSSValuePair> bottom_right_radius_;
  Member<CSSValuePair> bottom_left_radius_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSBasicShapeCircleValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapeCircleValue();
  }
};

template <>
struct DowncastTraits<cssvalue::CSSBasicShapeEllipseValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapeEllipseValue();
  }
};

template <>
struct DowncastTraits<cssvalue::CSSBasicShapePolygonValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapePolygonValue();
  }
};

template <>
struct DowncastTraits<cssvalue::CSSBasicShapeInsetValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapeInsetValue();
  }
};

template <>
struct DowncastTraits<cssvalue::CSSBasicShapeRectValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapeRectValue();
  }
};

template <>
struct DowncastTraits<cssvalue::CSSBasicShapeXYWHValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsBasicShapeXYWHValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_BASIC_SHAPE_VALUES_H_
