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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_name.h"
#include "third_party/blink/renderer/core/css/css_property_names.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/properties/css_property.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT CSSPropertyValue {
  DISALLOW_NEW();

 public:
  CSSPropertyValue(const CSSPropertyName& name,
                   const CSSValue& value,
                   bool important = false,
                   bool is_set_from_shorthand = false,
                   int index_in_shorthands_vector = 0,
                   bool implicit = false)
      : property_id_(static_cast<unsigned>(name.Id())),
        is_set_from_shorthand_(is_set_from_shorthand),
        index_in_shorthands_vector_(index_in_shorthands_vector),
        important_(important),
        implicit_(implicit),
        value_(value, decltype(value_)::AtomicInitializerTag{}) {
    if (name.IsCustomProperty()) {
      custom_name_ = name.ToAtomicString();
    }
  }

  CSSPropertyValue(const CSSPropertyValue& other)
      : custom_name_(other.custom_name_),
        property_id_(other.property_id_),
        is_set_from_shorthand_(other.is_set_from_shorthand_),
        index_in_shorthands_vector_(other.index_in_shorthands_vector_),
        important_(other.important_),
        implicit_(other.implicit_),
        value_(other.value_.Get(), decltype(value_)::AtomicInitializerTag{}) {}
  CSSPropertyValue& operator=(const CSSPropertyValue& other) = default;

  CSSPropertyID PropertyID() const {
    return ConvertToCSSPropertyID(property_id_);
  }
  const AtomicString& CustomPropertyName() const {
    DCHECK_EQ(PropertyID(), CSSPropertyID::kVariable);
    return custom_name_;
  }
  bool IsSetFromShorthand() const { return is_set_from_shorthand_; }
  CSSPropertyID ShorthandID() const;
  bool IsImportant() const { return important_; }
  void SetImportant() { important_ = true; }
  bool IsImplicit() const { return implicit_; }
  bool IsAffectedByAll() const {
    return PropertyID() != CSSPropertyID::kVariable &&
           CSSProperty::Get(PropertyID()).IsAffectedByAll();
  }
  CSSPropertyName Name() const;

  const CSSValue& Value() const { return *value_; }

  bool operator==(const CSSPropertyValue& other) const;

  void Trace(Visitor* visitor) const { visitor->Trace(value_); }

 private:
  AtomicString custom_name_;
  unsigned property_id_ : kCSSPropertyIDBitLength;
  unsigned is_set_from_shorthand_ : 1;
  // If this property was set as part of an ambiguous shorthand, gives the index
  // in the shorthands vector.
  unsigned index_in_shorthands_vector_ : 2;
  unsigned important_ : 1;
  // Whether or not the property was set implicitly as the result of a
  // shorthand.
  unsigned implicit_ : 1;
  // 17 free bits here.
  Member<const CSSValue> value_;
};

}  // namespace blink

namespace WTF {
template <>
struct VectorTraits<blink::CSSPropertyValue>
    : VectorTraitsBase<blink::CSSPropertyValue> {
  static const bool kCanInitializeWithMemset = true;
  static const bool kCanClearUnusedSlotsWithMemset = true;
  static const bool kCanMoveWithMemcpy = true;
  static const bool kCanTraceConcurrently = true;
};
}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_PROPERTY_VALUE_H_
