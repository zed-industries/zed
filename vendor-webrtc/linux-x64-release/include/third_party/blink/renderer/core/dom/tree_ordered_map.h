/*
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_MAP_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_impl.h"

namespace blink {

class Element;
class HTMLSlotElement;
class TreeScope;

// TreeOrderedMap is a map from keys to |Element|s, which allows multiple values
// per key, maintained in tree order per key. Tree walks are avoided when
// possible by retaining a cached, ordered array of matching nodes. Adding or
// removing an element for a given key often clears the cache, forcing a tree
// walk upon the next access.
class CORE_EXPORT TreeOrderedMap : public GarbageCollected<TreeOrderedMap> {
 public:
  TreeOrderedMap();

  void Add(const AtomicString&, Element&);
  void Remove(const AtomicString&, Element&);

  bool Contains(const AtomicString&) const;
  bool ContainsMultiple(const AtomicString&) const;
  // concrete instantiations of the get<>() method template
  Element* GetElementById(const AtomicString&, const TreeScope&) const;
  const HeapVector<Member<Element>>& GetAllElementsById(const AtomicString&,
                                                        const TreeScope&) const;
  Element* GetElementByMapName(const AtomicString&, const TreeScope&) const;
  HTMLSlotElement* GetSlotByName(const AtomicString&, const TreeScope&) const;
  // Don't use this unless the caller can know the internal state of
  // TreeOrderedMap exactly.
  Element* GetCachedFirstElementWithoutAccessingNodeTree(const AtomicString&);

  void Trace(Visitor*) const;

#if DCHECK_IS_ON()
  // While removing a ContainerNode, ID lookups won't be precise should the tree
  // have elements with duplicate IDs contained in the element being removed.
  // Rare trees, but ID lookups may legitimately fail across such removals;
  // this scope object informs TreeOrderedMaps about the transitory state of the
  // underlying tree.
  class CORE_EXPORT RemoveScope {
    STACK_ALLOCATED();

   public:
    RemoveScope();
    ~RemoveScope();
  };
#else
  class CORE_EXPORT RemoveScope {
    STACK_ALLOCATED();

   public:
    RemoveScope() {}
    ~RemoveScope() {}
  };
#endif

 private:
  template <bool keyMatches(const AtomicString&, const Element&)>
  Element* Get(const AtomicString&, const TreeScope&) const;

  class MapEntry : public GarbageCollected<MapEntry> {
   public:
    explicit MapEntry(Element& first_element)
        : element(first_element), count(1) {}

    void Trace(Visitor*) const;

    Member<Element> element;
    unsigned count;
    HeapVector<Member<Element>> ordered_list;
  };

  using Map = HeapHashMap<AtomicString, Member<MapEntry>>;

  mutable Map map_;
};

inline bool TreeOrderedMap::Contains(const AtomicString& id) const {
  return map_.Contains(id);
}

inline bool TreeOrderedMap::ContainsMultiple(const AtomicString& id) const {
  Map::const_iterator it = map_.find(id);
  return it != map_.end() && it->value->count > 1;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TREE_ORDERED_MAP_H_
