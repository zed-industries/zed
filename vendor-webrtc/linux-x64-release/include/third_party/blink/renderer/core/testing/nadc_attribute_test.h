// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NADC_ATTRIBUTE_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NADC_ATTRIBUTE_TEST_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class NADCAttributeTest final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  NADCAttributeTest() = default;
  ~NADCAttributeTest() override = default;

  void setFloat64Value(double value) { value_ = value; }
  double float64Value() const { return value_; }

  void Trace(Visitor* visitor) const override {
    blink::ScriptWrappable::Trace(visitor);
  }

 private:
  double value_;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_NADC_ATTRIBUTE_TEST_H_
