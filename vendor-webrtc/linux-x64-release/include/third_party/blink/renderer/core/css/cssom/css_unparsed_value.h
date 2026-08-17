// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNPARSED_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNPARSED_VALUE_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_union_cssvariablereferencevalue_string.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/bindings/v8_binding.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSUnparsedDeclarationValue;
class CSSVariableData;

class CORE_EXPORT CSSUnparsedValue final : public CSSStyleValue {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static CSSUnparsedValue* Create(
      const HeapVector<Member<V8CSSUnparsedSegment>>& tokens) {
    return MakeGarbageCollected<CSSUnparsedValue>(tokens);
  }

  // Blink-internal constructor
  static CSSUnparsedValue* Create() {
    return Create(HeapVector<Member<V8CSSUnparsedSegment>>());
  }
  static CSSUnparsedValue* FromCSSValue(const CSSUnparsedDeclarationValue&);
  static CSSUnparsedValue* FromCSSVariableData(const CSSVariableData&);
  static CSSUnparsedValue* FromString(const String& string) {
    HeapVector<Member<V8CSSUnparsedSegment>> tokens;
    tokens.push_back(MakeGarbageCollected<V8CSSUnparsedSegment>(string));
    return Create(tokens);
  }

  explicit CSSUnparsedValue(
      const HeapVector<Member<V8CSSUnparsedSegment>>& tokens)
      : tokens_(tokens) {}
  CSSUnparsedValue(const CSSUnparsedValue&) = delete;
  CSSUnparsedValue& operator=(const CSSUnparsedValue&) = delete;

  const CSSValue* ToCSSValue() const override;

  StyleValueType GetType() const override { return kUnparsedType; }

  V8CSSUnparsedSegment* AnonymousIndexedGetter(
      uint32_t index,
      ExceptionState& exception_state) const;
  IndexedPropertySetterResult AnonymousIndexedSetter(
      uint32_t index,
      V8CSSUnparsedSegment* segment,
      ExceptionState& exception_state);

  wtf_size_t length() const { return tokens_.size(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(tokens_);
    CSSStyleValue::Trace(visitor);
  }

  // Unlike CSSStyleValue::toString(), this returns tokens without
  // substituting variables. There are extra /**/ inserted between
  // every token to ensure there are no ambiguities, which is fine
  // because this value is never presented directly to the user
  // (ToCSSValue() will parse to a token range and then re-serialize
  // using extra /**/ only where needed).
  String ToUnparsedString() const;

 private:
  // Return 'false' if there is a cycle in the serialization.
  bool AppendUnparsedString(
      StringBuilder&,
      HeapHashSet<Member<const CSSUnparsedValue>>& values_on_stack) const;

  HeapVector<Member<V8CSSUnparsedSegment>> tokens_;

  FRIEND_TEST_ALL_PREFIXES(CSSUnparsedDeclarationValueTest, MixedList);
};

template <>
struct DowncastTraits<CSSUnparsedValue> {
  static bool AllowFrom(const CSSStyleValue& value) {
    return value.GetType() == CSSStyleValue::StyleValueType::kUnparsedType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_UNPARSED_VALUE_H_
