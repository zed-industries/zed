// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUES_H_

#include "third_party/blink/renderer/core/css/cssom/css_unit_value.h"

namespace blink {

class CSSUnitValues {
  STATIC_ONLY(CSSUnitValues);

 public:
  // <length>
  static CSSUnitValue* number(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kNumber);
  }

  static CSSUnitValue* percent(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kPercentage);
  }

  static CSSUnitValue* em(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kEms);
  }

  static CSSUnitValue* ex(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kExs);
  }

  static CSSUnitValue* ch(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kChs);
  }

  static CSSUnitValue* ic(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kIcs);
  }

  static CSSUnitValue* lh(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kLhs);
  }

  static CSSUnitValue* rlh(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRlhs);
  }

  static CSSUnitValue* rem(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRems);
  }

  static CSSUnitValue* cap(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kCaps);
  }

  static CSSUnitValue* rcap(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRcaps);
  }

  static CSSUnitValue* rex(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRexs);
  }

  static CSSUnitValue* rch(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRchs);
  }

  static CSSUnitValue* ric(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRics);
  }

  static CSSUnitValue* vw(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kViewportWidth);
  }

  static CSSUnitValue* vh(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kViewportHeight);
  }

  static CSSUnitValue* vi(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kViewportInlineSize);
  }

  static CSSUnitValue* vb(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kViewportBlockSize);
  }

  static CSSUnitValue* vmin(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kViewportMin);
  }

  static CSSUnitValue* vmax(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kViewportMax);
  }

  static CSSUnitValue* svw(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kSmallViewportWidth);
  }

  static CSSUnitValue* svh(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kSmallViewportHeight);
  }

  static CSSUnitValue* svi(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kSmallViewportInlineSize);
  }

  static CSSUnitValue* svb(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kSmallViewportBlockSize);
  }

  static CSSUnitValue* svmin(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kSmallViewportMin);
  }

  static CSSUnitValue* svmax(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kSmallViewportMax);
  }

  static CSSUnitValue* lvw(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kLargeViewportWidth);
  }

  static CSSUnitValue* lvh(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kLargeViewportHeight);
  }

  static CSSUnitValue* lvi(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kLargeViewportInlineSize);
  }

  static CSSUnitValue* lvb(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kLargeViewportBlockSize);
  }

  static CSSUnitValue* lvmin(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kLargeViewportMin);
  }

  static CSSUnitValue* lvmax(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kLargeViewportMax);
  }

  static CSSUnitValue* dvw(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportWidth);
  }

  static CSSUnitValue* dvh(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportHeight);
  }

  static CSSUnitValue* dvi(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportInlineSize);
  }

  static CSSUnitValue* dvb(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportBlockSize);
  }

  static CSSUnitValue* dvmin(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportMin);
  }

  static CSSUnitValue* dvmax(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDynamicViewportMax);
  }

  static CSSUnitValue* cqw(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kContainerWidth);
  }

  static CSSUnitValue* cqh(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kContainerHeight);
  }

  static CSSUnitValue* cqi(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kContainerInlineSize);
  }

  static CSSUnitValue* cqb(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kContainerBlockSize);
  }

  static CSSUnitValue* cqmin(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kContainerMin);
  }

  static CSSUnitValue* cqmax(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kContainerMax);
  }

  static CSSUnitValue* cm(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kCentimeters);
  }

  static CSSUnitValue* mm(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kMillimeters);
  }

  static CSSUnitValue* Q(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kQuarterMillimeters);
  }

  static CSSUnitValue* in(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kInches);
  }

  static CSSUnitValue* pt(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kPoints);
  }

  static CSSUnitValue* pc(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kPicas);
  }

  static CSSUnitValue* px(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kPixels);
  }

  // <angle>
  static CSSUnitValue* deg(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kDegrees);
  }

  static CSSUnitValue* grad(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kGradians);
  }

  static CSSUnitValue* rad(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kRadians);
  }

  static CSSUnitValue* turn(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kTurns);
  }

  // <time>
  static CSSUnitValue* s(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kSeconds);
  }

  static CSSUnitValue* ms(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kMilliseconds);
  }

  // <frequency>
  static CSSUnitValue* Hz(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kHertz);
  }

  static CSSUnitValue* kHz(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kKilohertz);
  }

  // <resolution>
  static CSSUnitValue* dpi(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kDotsPerInch);
  }

  static CSSUnitValue* dpcm(double value) {
    return CSSUnitValue::Create(
        value, CSSPrimitiveValue::UnitType::kDotsPerCentimeter);
  }

  static CSSUnitValue* x(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kX);
  }

  static CSSUnitValue* dppx(double value) {
    return CSSUnitValue::Create(value,
                                CSSPrimitiveValue::UnitType::kDotsPerPixel);
  }

  // <flex>
  static CSSUnitValue* fr(double value) {
    return CSSUnitValue::Create(value, CSSPrimitiveValue::UnitType::kFlex);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNIT_VALUES_H_
