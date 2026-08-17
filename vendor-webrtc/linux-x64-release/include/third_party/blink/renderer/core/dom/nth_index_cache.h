// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// Cache element indices for :nth-child and :nth-last-child selectors,
// and similar for :nth-of-type and :nth-last-of-type.
//
// In order to avoid n^2 for :nth-selectors, we introduce a cache where we
// store the index of every kth child of a parent node P the first time the
// nth-count is queried for one of its children. The number k is given by
// the "spread" constant, currently 3. (The number 3 was chosen after some
// kind of testing, but the details have been lost to the mists of time.)
//
// After the cache has been populated for the children of P, the nth-index
// for an element will be found by walking the siblings from the element
// queried for and leftwards until a cached element/index pair is found.
// So populating the cache for P is O(n). Subsequent lookups are best case
// O(1), worst case O(k).
//
// The cache is created on the stack when we do operations where we know we
// can benefit from having it. Currently, those are querySelector,
// querySelectorAll, and updating style. Also, we need to see at least 32
// children for the given node, which is a rough cutoff for when the cost of
// building the cache is outweighed by the gains of faster queries.
// We are throwing away the cache after each operation to avoid holding on
// to potentially large caches in memory.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NTH_INDEX_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NTH_INDEX_CACHE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/selector_checker.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Document;

// The cache for a given :nth-* selector; maps from each child element of
// a given node (modulo spread; see file comment) to its correct child index.
// The owner needs to key by parent and potentially tag name or selector;
// we receive them to do the actual query, but do not store them.
class CORE_EXPORT NthIndexData final : public GarbageCollected<NthIndexData> {
 public:
  enum SiblingOrder {
    kLightTree,
    // sibling-index() and sibling-count() are based on non-flattened
    // assignedNodes order for slotted elements.
    kFlatTree,
  };

  NthIndexData(ContainerNode&,
               const CSSSelectorList* filter,
               const SelectorChecker* selector_checker,
               const SelectorChecker::SelectorCheckingContext* context,
               SiblingOrder sibling_order);
  NthIndexData(ContainerNode&, const QualifiedName& type);
  NthIndexData(const NthIndexData&) = delete;
  NthIndexData& operator=(const NthIndexData&) = delete;

  // The three last parameters in NthIndex() and NthLastIndex() are for
  // re-checking the selector (if any), since we only store every third
  // matching element. We're not allowed to store them easily in the
  // constructor, since they are marked as STACK_ALLOCATED.
  unsigned NthIndex(Element&,
                    const CSSSelectorList* filter,
                    const SelectorChecker* selector_checker,
                    const SelectorChecker::SelectorCheckingContext* context,
                    SiblingOrder sibling_order) const;
  unsigned NthLastIndex(Element&,
                        const CSSSelectorList* filter,
                        const SelectorChecker* selector_checker,
                        const SelectorChecker::SelectorCheckingContext* context,
                        SiblingOrder sibling_order) const;
  unsigned NthOfTypeIndex(Element&) const;
  unsigned NthLastOfTypeIndex(Element&) const;

  void Trace(Visitor*) const;

 private:
  HeapHashMap<Member<Element>, unsigned> element_index_map_;

  // Number of total elements under the given node, so that we know
  // what to search for when doing nth-last-child.
  // (element_index_map_.size() is not correct, since we do not store the
  // indices for all children.)
  unsigned count_ = 0;
};

// The singleton cache, usually allocated at the stack on-demand.
// Caches for all nodes in the entire document.
//
// This class also has a dual role of RAII on Document; when constructed,
// it sets Document's NthIndexCache member to ourselves (so that NthChildIndex
// etc. can be static, and we don't need to send the cache through to
// selector matching), and when destroyed, unsets that member.
class CORE_EXPORT NthIndexCache final {
  STACK_ALLOCATED();

 public:
  explicit NthIndexCache(Document&);
  NthIndexCache(const NthIndexCache&) = delete;
  NthIndexCache& operator=(const NthIndexCache&) = delete;
  ~NthIndexCache();

  static unsigned NthChildIndex(
      Element& element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context,
      NthIndexData::SiblingOrder sibling_order);
  static unsigned NthLastChildIndex(
      Element& element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context,
      NthIndexData::SiblingOrder sibling_order);
  static unsigned NthOfTypeIndex(Element&);
  static unsigned NthLastOfTypeIndex(Element&);

 private:
  // Key in the top-level cache; identifies the parent and the type of query.
  struct Key : public GarbageCollected<Key> {
    Key(Node* parent_arg,
        const CSSSelectorList* filter_arg,
        NthIndexData::SiblingOrder order)
        : parent(parent_arg), filter(filter_arg), sibling_order(order) {}
    Key(Node* parent_arg, String child_tag_name_arg)
        : parent(parent_arg), child_tag_name(child_tag_name_arg) {}

    Member<Node> parent;
    String child_tag_name;  // Empty if not :nth-of-type.
    // Can be nullptr. Always nullptr if :nth-of-type, which filters on
    // child_tag_name instead.
    Member<const CSSSelectorList> filter;
    // kFlatTree if the index is for a slotted element in the flat tree order.
    // Used for sibling-index() and sibling-count().
    // Indices for :nth-*() selectors are always cached for kLightTree.
    NthIndexData::SiblingOrder sibling_order = NthIndexData::kLightTree;

    void Trace(Visitor* visitor) const;
    unsigned GetHash() const;
    bool operator==(const Key& other) const {
      // NOTE: We compare filter by identity, which makes for potentially
      // (theoretically) less effective caching between different selectors, but
      // is simpler.
      return parent == other.parent && filter == other.filter &&
             child_tag_name == other.child_tag_name &&
             sibling_order == other.sibling_order;
    }
  };

  // Helper needed to make sure Key is compared by value and not by pointer,
  // even though the hash map key is a Member<> (which Oilpan forces us to).
  struct KeyHashTraits : WTF::MemberHashTraits<Key> {
    static unsigned GetHash(const Member<Key>& key) { return key->GetHash(); }
    static bool Equal(const Member<Key>& a, const Member<Key>& b) {
      return *a == *b;
    }
    static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
  };

  // Helper needed to allow calling Find() with a Key instead of Member<Key>
  // (note that find() does not allow this).
  struct KeyHashTranslator {
    STATIC_ONLY(KeyHashTranslator);

    static unsigned GetHash(const Key& key) { return key.GetHash(); }
    static bool Equal(const Member<Key>& a, const Key& b) {
      return a && *a == b;
    }
  };

  static bool MatchesFilter(
      Element* element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context);
  static unsigned UncachedNthChildIndex(
      Element& element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context,
      NthIndexData::SiblingOrder,
      unsigned& sibling_count);
  static unsigned UncachedNthLastChildIndex(
      Element& element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context,
      NthIndexData::SiblingOrder,
      unsigned& sibling_count);
  void CacheNthIndexDataForParent(
      Element& element,
      const CSSSelectorList* filter,
      const SelectorChecker* selector_checker,
      const SelectorChecker::SelectorCheckingContext* context,
      NthIndexData::SiblingOrder sibling_order);
  void CacheNthOfTypeIndexDataForParent(Element&);
  void EnsureCache();

  Document* document_ = nullptr;

  // Effectively maps (parent, optional tag name, child) → index.
  // (The child part of the key is in NthIndexData.)
  GCedHeapHashMap<Member<Key>, Member<NthIndexData>, KeyHashTraits>* cache_ =
      nullptr;

#if DCHECK_IS_ON()
  uint64_t dom_tree_version_;
#endif

  friend class NthIndexData;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NTH_INDEX_CACHE_H_
