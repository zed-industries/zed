/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2011, 2012 Apple Inc. All
 * rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_COLLECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_COLLECTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/live_node_list_base.h"
#include "third_party/blink/renderer/core/html/collection_items_cache.h"
#include "third_party/blink/renderer/core/html/collection_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// A simple iterator based on an index number in an HTMLCollection.
// This doesn't work if the HTMLCollection is updated during iteration.
template <class CollectionType, class NodeType>
class HTMLCollectionIterator {
  STACK_ALLOCATED();

 public:
  explicit HTMLCollectionIterator(const CollectionType* collection)
      : collection_(collection) {}
  NodeType* operator*() { return collection_->item(index_); }

  void operator++() {
    if (index_ < collection_->length())
      ++index_;
  }

  bool operator!=(const HTMLCollectionIterator& other) const {
    return collection_ != other.collection_ || index_ != other.index_;
  }

  static HTMLCollectionIterator CreateEnd(const CollectionType* collection) {
    HTMLCollectionIterator iterator(collection);
    iterator.index_ = collection->length();
    return iterator;
  }

 private:
  const CollectionType* collection_;
  unsigned index_ = 0;
};

// blink::HTMLCollection implements HTMLCollection IDL interface.
class CORE_EXPORT HTMLCollection : public ScriptWrappable,
                                   public LiveNodeListBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum ItemAfterOverrideType {
    kOverridesItemAfter,
    kDoesNotOverrideItemAfter,
  };

  HTMLCollection(ContainerNode& base,
                 CollectionType,
                 ItemAfterOverrideType = kDoesNotOverrideItemAfter);
  ~HTMLCollection() override;
  void InvalidateCache(Document* old_document = nullptr) const override;
  void InvalidateCacheForAttribute(const QualifiedName*) const;

  // DOM API
  unsigned length() const;
  Element* item(unsigned offset) const;
  virtual Element* namedItem(const AtomicString& name) const;
  bool NamedPropertyQuery(const AtomicString&, ExceptionState&);
  void NamedPropertyEnumerator(Vector<String>& names, ExceptionState&);

  // Non-DOM API
  void NamedItems(const AtomicString& name, HeapVector<Member<Element>>&) const;
  bool HasNamedItems(const AtomicString& name) const;
  bool IsEmpty() const { return collection_items_cache_.IsEmpty(*this); }
  bool HasExactlyOneItem() const {
    return collection_items_cache_.HasExactlyOneNode(*this);
  }
  bool ElementMatches(const Element&) const;

  // CollectionIndexCache API.
  bool CanTraverseBackward() const { return !OverridesItemAfter(); }
  Element* TraverseToFirst() const;
  Element* TraverseToLast() const;
  Element* TraverseForwardToOffset(unsigned offset,
                                   Element& current_element,
                                   unsigned& current_offset) const;
  Element* TraverseBackwardToOffset(unsigned offset,
                                    Element& current_element,
                                    unsigned& current_offset) const;

  using Iterator = HTMLCollectionIterator<HTMLCollection, Element>;
  Iterator begin() const { return Iterator(this); }
  Iterator end() const { return Iterator::CreateEnd(this); }

  void Trace(Visitor*) const override;

  bool NamedItemsEmpty() const {
    UpdateIdNameCache();
    return !GetNamedItemCache().empty();
  }

 protected:
  class NamedItemCache final : public GarbageCollected<NamedItemCache> {
   public:
    NamedItemCache();

    const GCedHeapVector<Member<Element>>* GetElementsById(
        const AtomicString& id) const {
      auto it = id_cache_.find(id);
      if (it == id_cache_.end())
        return nullptr;
      return it->value.Get();
    }
    const GCedHeapVector<Member<Element>>* GetElementsByName(
        const AtomicString& name) const {
      auto it = name_cache_.find(name);
      if (it == name_cache_.end())
        return nullptr;
      return it->value.Get();
    }
    void AddElementWithId(const AtomicString& id, Element* element) {
      AddElementToMap(id_cache_, id, element);
    }
    void AddElementWithName(const AtomicString& name, Element* element) {
      AddElementToMap(name_cache_, name, element);
    }

    void Trace(Visitor* visitor) const {
      visitor->Trace(id_cache_);
      visitor->Trace(name_cache_);
    }

    bool empty() const { return id_cache_.empty() && name_cache_.empty(); }

   private:
    typedef HeapHashMap<AtomicString, Member<GCedHeapVector<Member<Element>>>>
        StringToElementsMap;
    static void AddElementToMap(StringToElementsMap& map,
                                const AtomicString& key,
                                Element* element) {
      GCedHeapVector<Member<Element>>* vector =
          map.insert(key,
                     MakeGarbageCollected<GCedHeapVector<Member<Element>>>())
              .stored_value->value.Get();
      vector->push_back(element);
    }

    StringToElementsMap id_cache_;
    StringToElementsMap name_cache_;
  };

  bool OverridesItemAfter() const { return overrides_item_after_; }
  virtual Element* VirtualItemAfter(Element*) const;
  bool ShouldOnlyIncludeDirectChildren() const {
    return should_only_include_direct_children_;
  }
  virtual void SupportedPropertyNames(Vector<String>& names);

  virtual void UpdateIdNameCache() const;
  bool HasValidIdNameCache() const { return named_item_cache_; }

  void SetNamedItemCache(NamedItemCache* cache) const {
    DCHECK(!named_item_cache_);
    // Do not repeat registration for the same invalidation type.
    if (InvalidationType() != kInvalidateOnIdNameAttrChange)
      GetDocument().RegisterNodeListWithIdNameCache(this);
    named_item_cache_ = cache;
  }

  NamedItemCache& GetNamedItemCache() const {
    DCHECK(named_item_cache_);
    return *named_item_cache_;
  }

 private:
  void InvalidateIdNameCacheMaps(Document* old_document = nullptr) const {
    if (!HasValidIdNameCache())
      return;

    // Make sure we decrement the NodeListWithIdNameCache count from
    // the old document instead of the new one in the case the collection
    // is moved to a new document.
    UnregisterIdNameCacheFromDocument(old_document ? *old_document
                                                   : GetDocument());

    named_item_cache_.Clear();
  }

  void UnregisterIdNameCacheFromDocument(Document& document) const {
    DCHECK(HasValidIdNameCache());
    // Do not repeat unregistration for the same invalidation type.
    if (InvalidationType() != kInvalidateOnIdNameAttrChange)
      document.UnregisterNodeListWithIdNameCache(this);
  }

  const unsigned overrides_item_after_ : 1;
  const unsigned should_only_include_direct_children_ : 1;
  mutable Member<NamedItemCache> named_item_cache_;
  mutable CollectionItemsCache<HTMLCollection, Element> collection_items_cache_;
};

template <>
struct DowncastTraits<HTMLCollection> {
  static bool AllowFrom(const LiveNodeListBase& collection) {
    return IsHTMLCollectionType(collection.GetType());
  }
};

DISABLE_CFI_PERF
inline void HTMLCollection::InvalidateCacheForAttribute(
    const QualifiedName* attr_name) const {
  if (!attr_name ||
      ShouldInvalidateTypeOnAttributeChange(InvalidationType(), *attr_name))
    InvalidateCache();
  else if (*attr_name == html_names::kIdAttr ||
           *attr_name == html_names::kNameAttr)
    InvalidateIdNameCacheMaps();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_COLLECTION_H_
