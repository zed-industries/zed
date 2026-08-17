// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_SUM_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_SUM_VALUE_H_

#include "third_party/blink/renderer/core/css/css_primitive_value.h"

#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

// CSSNumericSumValue represents the sum of one or more "terms".
// A term is a number with a set of units. e.g.
// 1px/s + 5m^2 - 1Hz is a sum value with three terms.
struct CSSNumericSumValue {
  // A UnitMap maps units to exponents. e.g. the term
  // 1m/s^2 would have a unit map of { m: 1, s: -2 }.
  // UnitMaps must not contain entries with a zero value.
  using UnitMap = WTF::HashMap<CSSPrimitiveValue::UnitType, int>;

  // A term is a number and a unit map e.g. 1px is represented as
  // (1, { px: 1 })
  struct Term {
    double value;
    UnitMap units;

    Term(double value, UnitMap units) : value(value), units(std::move(units)) {}
  };

  Vector<Term> terms;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_SUM_VALUE_H_
