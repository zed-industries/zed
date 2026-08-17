// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_value_id_mappings.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

// CSSIdentifierValue stores CSS value keywords, e.g. 'none', 'auto',
// 'lower-roman'.
// TODO(sashab): Rename this class to CSSKeywordValue once it no longer
// conflicts with CSSOM's CSSKeywordValue class.
class CORE_EXPORT CSSIdentifierValue : public CSSValue {
 public:
  static CSSIdentifierValue* Create(CSSValueID);

  // TODO(sashab): Rename this to createFromPlatformValue().
  template <typename T>
  static CSSIdentifierValue* Create(T value) {
    static_assert(!std::is_same<T, CSSValueID>::value,
                  "Do not call create() with a CSSValueID; call "
                  "createIdentifier() instead");
    return MakeGarbageCollected<CSSIdentifierValue>(value);
  }

  static CSSIdentifierValue* Create(const Length& value) {
    return MakeGarbageCollected<CSSIdentifierValue>(value);
  }

  explicit CSSIdentifierValue(CSSValueID);
  explicit CSSIdentifierValue(CSSValueID, bool was_quirky);

  // TODO(sashab): Remove this function, and update mapping methods to
  // specialize the create() method instead.
  template <typename T>
  CSSIdentifierValue(
      T t)  // Overridden for special cases in css_identifier_value_mappings.h
      : CSSValue(kIdentifierClass), value_id_(PlatformEnumToCSSValueID(t)) {}

  CSSIdentifierValue(const Length&);

  CSSValueID GetValueID() const { return value_id_; }

  String CustomCSSText() const;

  bool Equals(const CSSIdentifierValue& other) const {
    return value_id_ == other.value_id_;
  }
  unsigned CustomHash() const { return static_cast<unsigned>(value_id_); }

  template <typename T>
  inline T ConvertTo() const {  // Overridden for special cases in
                                // css_identifier_value_mappings.h
    return CssValueIDToPlatformEnum<T>(value_id_);
  }

  bool WasQuirky() const { return was_quirky_; }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  CSSValueID value_id_;
};

template <>
struct DowncastTraits<CSSIdentifierValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsIdentifierValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_IDENTIFIER_VALUE_H_
