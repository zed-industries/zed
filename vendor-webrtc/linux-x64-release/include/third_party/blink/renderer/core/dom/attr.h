/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Peter Kelly (pmk@post.com)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/qualified_name.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class CORE_EXPORT Attr final : public Node {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Attr(Element&, const QualifiedName&);
  Attr(Document&, const QualifiedName&, const AtomicString& value);
  ~Attr() override;

  String name() const { return name_.ToString(); }
  bool specified() const { return true; }
  Element* ownerElement() const { return element_.Get(); }

  const AtomicString& value() const;
  void setValue(const AtomicString&, ExceptionState&);

  const QualifiedName GetQualifiedName() const;

  void AttachToElement(Element*, const AtomicString&);
  void DetachFromElementWithValue(const AtomicString&);

  const AtomicString& localName() const { return name_.LocalName(); }
  const AtomicString& namespaceURI() const { return name_.NamespaceURI(); }
  const AtomicString& prefix() const { return name_.Prefix(); }

  void Trace(Visitor*) const override;

 private:
  bool IsElementNode() const =
      delete;  // This will catch anyone doing an unnecessary check.

  String nodeName() const override { return name(); }

  String nodeValue() const override { return value(); }
  void setNodeValue(const String&, ExceptionState&) override;
  void setTextContentForBinding(const V8UnionStringOrTrustedScript* value,
                                ExceptionState& exception_state) override;
  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;

  bool IsAttributeNode() const override { return true; }

  // Attr wraps either an element/name, or a name/value pair (when it's a
  // standalone Node.)
  // Note that name_ is always set, but element_ /
  // standalone_value_or_attached_local_name_ may be null.
  Member<Element> element_;
  QualifiedName name_;
  // Holds the value if it is a standalone Node, or the local name of the
  // attribute it is attached to on an Element. The latter may (letter case)
  // differ from name_'s local name. As these two modes are non-overlapping,
  // use a single field.
  AtomicString standalone_value_or_attached_local_name_;
};

template <>
struct DowncastTraits<Attr> {
  static bool AllowFrom(const Node& node) { return node.IsAttributeNode(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ATTR_H_
