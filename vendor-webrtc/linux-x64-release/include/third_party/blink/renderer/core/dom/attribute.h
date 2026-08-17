/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Peter Kelly (pmk@post.com)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2008, 2012 Apple Inc. All rights
 * reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_H_

#include "base/containers/span.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {
class Attribute;
}

namespace base {
template <>
inline constexpr bool kCanSafelyConvertToByteSpan<::blink::Attribute> =
    kCanSafelyConvertToByteSpan<::blink::QualifiedName> &&
    kCanSafelyConvertToByteSpan<::WTF::AtomicString>;
}

namespace blink {

// This value is set fairly arbitrarily, to get above what we expect to be
// the maximum number of attributes on a normal element. It is used for
// preallocation in Vectors holding Attributes, e.g. to avoid allocations
// in HTMLAtomicToken (which is short-lived, and thus does not need to worry
// much about extra memory usage).
//
// Many places that use this constant don't actually care directly about
// preallocation, but the value tends to propagate out through APIs.
static constexpr int kAttributePrealloc = 10;

// This is the internal representation of an attribute, consisting of a name and
// value. It is distinct from the web-exposed Attr, which also knows of the
// element to which it attached, if any.
class Attribute {
  DISALLOW_NEW();

 public:
  Attribute(const QualifiedName& name, const AtomicString& value)
      : name_(name), value_(value) {}
  Attribute(QualifiedName&& name, AtomicString&& value)
      : name_(std::move(name)), value_(std::move(value)) {}

  // NOTE: The references returned by these functions are only valid for as long
  // as the Attribute stays in place. For example, calling a function that
  // mutates an Element's internal attribute storage may invalidate them.
  const AtomicString& Value() const { return value_; }
  const AtomicString& Prefix() const { return name_.Prefix(); }
  const AtomicString& LocalName() const { return name_.LocalName(); }
  const AtomicString& NamespaceURI() const { return name_.NamespaceURI(); }

  const QualifiedName& GetName() const { return name_; }

  bool IsEmpty() const { return value_.empty(); }
  bool Matches(const QualifiedName&) const;
  bool MatchesCaseInsensitive(const QualifiedName&) const;

  void SetValue(const AtomicString& value) { value_ = value; }

  // Note: This API is only for HTMLTreeBuilder.  It is not safe to change the
  // name of an attribute once parseAttribute has been called as DOM
  // elements may have placed the Attribute in a hash by name.
  void ParserSetName(const QualifiedName& name) { name_ = name; }

#if defined(COMPILER_MSVC)
  // NOTE: This constructor is not actually implemented, it's just defined so
  // MSVC will let us use a zero-length array of Attributes.
  Attribute();
#endif

  bool operator==(const Attribute& other) const = default;

 private:
  QualifiedName name_;
  AtomicString value_;
};
static_assert(sizeof(Attribute) == sizeof(QualifiedName) + sizeof(AtomicString),
              "AttributeHash() assumes Attribute has no padding");

inline bool Attribute::Matches(const QualifiedName& qualified_name) const {
  return (qualified_name.LocalName() == LocalName()) &&
         (qualified_name.NamespaceURI() == NamespaceURI() ||
          qualified_name.Prefix() == g_star_atom);
}

inline bool Attribute::MatchesCaseInsensitive(
    const QualifiedName& qualified_name) const {
  return qualified_name.LocalNameUpper() == name_.LocalNameUpper() &&
         (qualified_name.NamespaceURI() == NamespaceURI() ||
          qualified_name.Prefix() == g_star_atom);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTRIBUTE_H_
