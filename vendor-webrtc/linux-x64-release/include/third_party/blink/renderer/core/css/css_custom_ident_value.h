// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_IDENT_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_IDENT_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class TreeScope;
class ScopedCSSName;

class CORE_EXPORT CSSCustomIdentValue : public CSSValue {
 public:
  explicit CSSCustomIdentValue(const AtomicString&);
  explicit CSSCustomIdentValue(CSSPropertyID);
  explicit CSSCustomIdentValue(const ScopedCSSName&);

  const TreeScope* GetTreeScope() const { return tree_scope_.Get(); }
  const AtomicString& Value() const {
    DCHECK(!IsKnownPropertyID());
    return string_;
  }
  bool IsKnownPropertyID() const {
    return property_id_ != CSSPropertyID::kInvalid;
  }
  CSSPropertyID ValueAsPropertyID() const {
    DCHECK(IsKnownPropertyID());
    return property_id_;
  }

  String CustomCSSText() const;
  unsigned CustomHash() const;

  const CSSCustomIdentValue& PopulateWithTreeScope(const TreeScope*) const;

  bool Equals(const CSSCustomIdentValue& other) const {
    if (IsKnownPropertyID()) {
      return property_id_ == other.property_id_;
    }
    return IsScopedValue() == other.IsScopedValue() &&
           tree_scope_ == other.tree_scope_ && string_ == other.string_;
  }

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  WeakMember<const TreeScope> tree_scope_;
  AtomicString string_;
  CSSPropertyID property_id_;
};

template <>
struct DowncastTraits<CSSCustomIdentValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsCustomIdentValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CUSTOM_IDENT_VALUE_H_
