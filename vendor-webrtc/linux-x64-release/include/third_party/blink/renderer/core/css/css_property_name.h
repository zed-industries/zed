// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_NAME_H_

#include <optional>

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace blink {

class ExecutionContext;

// This class may be used to represent the name of any valid CSS property,
// including custom properties.
class CORE_EXPORT CSSPropertyName {
  DISALLOW_NEW();

 public:
  explicit CSSPropertyName(CSSPropertyID property_id)
      : value_(static_cast<int>(property_id)) {
    DCHECK_NE(Id(), CSSPropertyID::kInvalid);
    DCHECK_NE(Id(), CSSPropertyID::kVariable);
  }

  explicit CSSPropertyName(const AtomicString& custom_property_name)
      : value_(static_cast<int>(CSSPropertyID::kVariable)),
        custom_property_name_(custom_property_name) {
    DCHECK(!custom_property_name.IsNull());
  }

  static std::optional<CSSPropertyName> From(
      const ExecutionContext* execution_context,
      const String& value) {
    const CSSPropertyID property_id = CssPropertyID(execution_context, value);
    if (property_id == CSSPropertyID::kInvalid) {
      return std::nullopt;
    }
    if (property_id == CSSPropertyID::kVariable) {
      return std::make_optional(CSSPropertyName(AtomicString(value)));
    }
    return std::make_optional(CSSPropertyName(property_id));
  }

  bool operator==(const CSSPropertyName&) const;
  bool operator!=(const CSSPropertyName& other) const {
    return !(*this == other);
  }

  CSSPropertyID Id() const {
    DCHECK(!IsEmptyValue() && !IsDeletedValue());
    return static_cast<CSSPropertyID>(value_);
  }

  bool IsCustomProperty() const { return Id() == CSSPropertyID::kVariable; }

  const AtomicString& ToAtomicString() const;

 private:
  // For HashTraits::EmptyValue().
  static constexpr int kEmptyValue = -1;
  // For HashTraits::ConstructDeletedValue(...).
  static constexpr int kDeletedValue = -2;

  explicit CSSPropertyName(int value) : value_(value) {
    DCHECK(value == kEmptyValue || value == kDeletedValue);
  }

  unsigned GetHash() const;
  bool IsEmptyValue() const { return value_ == kEmptyValue; }
  bool IsDeletedValue() const { return value_ == kDeletedValue; }

  // The value_ field is either a CSSPropertyID, kEmptyValue, or
  // kDeletedValue.
  int value_;
  AtomicString custom_property_name_;

  friend class CSSPropertyNameTest;
  friend struct ::WTF::HashTraits<blink::CSSPropertyName>;
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::CSSPropertyName>
    : SimpleClassHashTraits<blink::CSSPropertyName> {
  static unsigned GetHash(const blink::CSSPropertyName& name) {
    return name.GetHash();
  }

  using CSSPropertyName = blink::CSSPropertyName;
  static const bool kEmptyValueIsZero = false;
  static void ConstructDeletedValue(CSSPropertyName& slot) {
    new (NotNullTag::kNotNull, &slot)
        CSSPropertyName(CSSPropertyName::kDeletedValue);
  }
  static bool IsDeletedValue(const CSSPropertyName& value) {
    return value.IsDeletedValue();
  }
  static bool IsEmptyValue(const CSSPropertyName& value) {
    return value.IsEmptyValue();
  }
  static blink::CSSPropertyName EmptyValue() {
    return blink::CSSPropertyName(CSSPropertyName::kEmptyValue);
  }
};

}  // namespace WTF

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(blink::CSSPropertyName)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_NAME_H_
