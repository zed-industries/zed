/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2001 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Nokia Corporation and/or its subsidiary(-ies)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TAG_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TAG_COLLECTION_H_

#include "third_party/blink/renderer/core/html/html_collection.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Collection that limits to a particular tag.
class TagCollection : public HTMLCollection {
 public:
  TagCollection(ContainerNode& root_node,
                CollectionType,
                const AtomicString& qualified_name);
  ~TagCollection() override;

  bool ElementMatches(const Element&) const;

 protected:
  AtomicString qualified_name_;
};

class TagCollectionNS : public HTMLCollection {
 public:
  TagCollectionNS(ContainerNode& root_node,
                  CollectionType,
                  const AtomicString& namespace_uri,
                  const AtomicString& local_name);
  ~TagCollectionNS() override;

  bool ElementMatches(const Element&) const;

 private:
  AtomicString namespace_uri_;
  AtomicString local_name_;
};

template <>
struct DowncastTraits<TagCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kTagCollectionType;
  }
};

template <>
struct DowncastTraits<TagCollectionNS> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return collection.GetType() == kTagCollectionNSType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TAG_COLLECTION_H_
