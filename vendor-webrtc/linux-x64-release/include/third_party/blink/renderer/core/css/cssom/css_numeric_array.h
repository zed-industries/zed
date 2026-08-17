// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_ARRAY_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_numeric_value.h"

namespace blink {

// See CSSNumericArray.idl for more information about this class.
class CORE_EXPORT CSSNumericArray final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit CSSNumericArray(CSSNumericValueVector values)
      : values_(std::move(values)) {}
  CSSNumericArray(const CSSNumericArray&) = delete;
  CSSNumericArray& operator=(const CSSNumericArray&) = delete;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(values_);
    ScriptWrappable::Trace(visitor);
  }

  unsigned length() const { return values_.size(); }
  CSSNumericValue* AnonymousIndexedGetter(unsigned index) {
    if (index < values_.size()) {
      return values_[index].Get();
    }
    return nullptr;
  }

  const CSSNumericValueVector& Values() const { return values_; }

 private:
  CSSNumericValueVector values_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_NUMERIC_ARRAY_H_
