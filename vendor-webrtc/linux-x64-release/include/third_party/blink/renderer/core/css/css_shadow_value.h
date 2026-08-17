/*
 * (C) 1999-2003 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHADOW_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHADOW_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class CSSIdentifierValue;
class CSSPrimitiveValue;

// Used for text-shadow and box-shadow
class CORE_EXPORT CSSShadowValue : public CSSValue {
 public:
  CSSShadowValue(CSSPrimitiveValue* x,
                 CSSPrimitiveValue* y,
                 CSSPrimitiveValue* blur,
                 CSSPrimitiveValue* spread,
                 CSSIdentifierValue* style,
                 const CSSValue* color);

  WTF::String CustomCSSText() const;

  bool Equals(const CSSShadowValue&) const;

  Member<CSSPrimitiveValue> x;
  Member<CSSPrimitiveValue> y;
  Member<CSSPrimitiveValue> blur;
  Member<CSSPrimitiveValue> spread;
  Member<CSSIdentifierValue> style;
  Member<const CSSValue> color;

  void TraceAfterDispatch(blink::Visitor*) const;
};

template <>
struct DowncastTraits<CSSShadowValue> {
  static bool AllowFrom(const CSSValue& value) { return value.IsShadowValue(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_SHADOW_VALUE_H_
