/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COUNTER_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COUNTER_VALUE_H_

#include "third_party/blink/renderer/core/css/css_custom_ident_value.h"
#include "third_party/blink/renderer/core/css/css_identifier_value.h"
#include "third_party/blink/renderer/core/css/css_string_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

namespace cssvalue {

class CSSCounterValue : public CSSValue {
 public:
  CSSCounterValue(const CSSCustomIdentValue* identifier,
                  const CSSCustomIdentValue* list_style,
                  const CSSStringValue* separator)
      : CSSValue(kCounterClass),
        identifier_(identifier),
        list_style_(list_style),
        separator_(separator) {
    // There's no way to define a counter() function value where the identifiers
    // are associated with different tree scopes.
    DCHECK_EQ(identifier->IsScopedValue(), list_style->IsScopedValue());
    DCHECK_EQ(identifier->GetTreeScope(), list_style->GetTreeScope());
    needs_tree_scope_population_ = !list_style->IsScopedValue();
  }

  const String& Identifier() const { return identifier_->Value(); }
  const AtomicString& ListStyle() const { return list_style_->Value(); }
  const String& Separator() const { return separator_->Value(); }
  const TreeScope* GetTreeScope() const { return list_style_->GetTreeScope(); }

  bool Equals(const CSSCounterValue& other) const {
    return Identifier() == other.Identifier() &&
           ListStyle() == other.ListStyle() &&
           Separator() == other.Separator() &&
           IsScopedValue() == other.IsScopedValue() &&
           GetTreeScope() == other.GetTreeScope();
  }

  const CSSCounterValue& PopulateWithTreeScope(const TreeScope*) const;

  String CustomCSSText() const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CSSCustomIdentValue> identifier_;  // string
  Member<const CSSCustomIdentValue> list_style_;  // ident
  Member<const CSSStringValue> separator_;        // string
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSCounterValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsCounterValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_COUNTER_VALUE_H_
