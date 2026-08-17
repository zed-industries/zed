/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006 Apple Computer, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_PAIR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_PAIR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CORE_EXPORT CSSValuePair : public CSSValue {
 public:
  enum IdenticalValuesPolicy { kDropIdenticalValues, kKeepIdenticalValues };

  CSSValuePair(const CSSValue* first,
               const CSSValue* second,
               IdenticalValuesPolicy identical_values_policy)
      : CSSValue(kValuePairClass),
        first_(first),
        second_(second),
        identical_values_policy_(identical_values_policy) {
    DCHECK(first_);
    DCHECK(second_);
  }

  const CSSValue& First() const { return *first_; }
  const CSSValue& Second() const { return *second_; }

  bool KeepIdenticalValues() const {
    return identical_values_policy_ == kKeepIdenticalValues;
  }

  String CustomCSSText() const {
    String first = first_->CssText();
    String second = second_->CssText();
    if (identical_values_policy_ == kDropIdenticalValues && first == second) {
      return first;
    }
    return first + ' ' + second;
  }

  bool Equals(const CSSValuePair& other) const {
    return base::ValuesEquivalent(first_, other.first_) &&
           base::ValuesEquivalent(second_, other.second_) &&
           identical_values_policy_ == other.identical_values_policy_;
  }
  unsigned CustomHash() const {
    return WTF::HashInts(identical_values_policy_,
                         WTF::HashInts(first_->Hash(), second_->Hash()));
  }

  void TraceAfterDispatch(blink::Visitor*) const;

 protected:
  CSSValuePair(ClassType class_type,
               const CSSValue* first,
               const CSSValue* second)
      : CSSValue(class_type),
        first_(first),
        second_(second),
        identical_values_policy_(kKeepIdenticalValues) {
    DCHECK(first_);
    DCHECK(second_);
  }

 private:
  Member<const CSSValue> first_;
  Member<const CSSValue> second_;
  IdenticalValuesPolicy identical_values_policy_;
};

template <>
struct DowncastTraits<CSSValuePair> {
  static bool AllowFrom(const CSSValue& value) { return value.IsValuePair(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_VALUE_PAIR_H_
