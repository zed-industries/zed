/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TAG_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TAG_COLLECTION_H_

#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/tag_collection.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

// Collection that limits to a particular tag and whose rootNode is in an
// HTMLDocument.
class HTMLTagCollection final : public TagCollection {
 public:
  HTMLTagCollection(ContainerNode& root_node,
                    const AtomicString& qualified_name);
  HTMLTagCollection(ContainerNode& root_node,
                    CollectionType type,
                    const AtomicString& qualified_name);

  bool ElementMatches(const Element&) const;

 private:
  AtomicString lowered_qualified_name_;
};

template <>
struct DowncastTraits<HTMLTagCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kHTMLTagCollectionType;
  }
};

inline bool HTMLTagCollection::ElementMatches(
    const Element& test_element) const {
  if (qualified_name_ == g_star_atom)
    return true;
  if (test_element.IsHTMLElement())
    return lowered_qualified_name_ == test_element.TagQName().ToString();
  return qualified_name_ == test_element.TagQName().ToString();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_TAG_COLLECTION_H_
