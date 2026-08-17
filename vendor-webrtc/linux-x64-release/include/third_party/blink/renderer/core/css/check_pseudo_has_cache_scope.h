// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_CACHE_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_CACHE_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/check_pseudo_has_argument_context.h"
#include "third_party/blink/renderer/core/css/check_pseudo_has_fast_reject_filter.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class CSSSelector;
class Document;
class CheckPseudoHasArgumentContext;

// To determine whether a :has() pseudo class matches an element or not, we need
// to check the :has() argument selector on the descendants, next siblings or
// next sibling descendants. While checking the :has() argument selector in
// reversed DOM tree traversal order, we can get the :has() pseudo class
// checking result on the elements in the subtree. By caching these results, we
// can prevent unnecessary :has() pseudo class checking operations. (Please
// refer the comments of CheckPseudoHasArgumentTraversalIterator)
//
// Caching the results on all elements in the subtree is a very memory consuming
// approach. To prevent the large and inefficient cache memory consumption,
// ElementCheckPseudoHasResultMap stores following flags for an element.
//
// - flag1 (Checked) : Indicates that the :has() pseudo class was already
//     checked on the element.
//
// - flag2 (Matched) : Indicates that the :has() pseudo class was already
//     checked on the element and it matched.
//
// - flag3 (AllDescendantsOrNextSiblingsChecked) : Indicates that all the
//     not-cached descendant elements (or all the not-cached next sibling
//     elements) of the element were already checked as not-matched.
//     When the :has() argument checking traversal is stopped, this flag is set
//     on the stopped element and the next sibling element of its ancestors to
//     mark already traversed subtree.
//
// - flag4 (SomeChildrenChecked) : Indicates that some children of the element
//     were already checked. This flag is set on the parent of the
//     kCheckPseudoHasResultAllDescendantsOrNextSiblingsChecked element.
//     If the parent of an not-cached element has this flag set, we can
//     determine whether the element is 'already checked as not-matched' or
//     'not yet checked' by checking the AllDescendantsOrNextSiblingsChecked
//     flag of its previous sibling elements.
//
// Example)  subject.match(':has(.a)')
//  - DOM
//      <div id=subject>
//        <div id=d1>
//          <div id=d11></div>
//        </div>
//        <div id=d2>
//          <div id=d21></div>
//          <div id=d22 class=a>
//            <div id=d221></div>
//          </div>
//          <div id=d23></div>
//        </div>
//        <div id=d3>
//          <div id=d31></div>
//        </div>
//        <div id=d4></div>
//      </div>
//
//  - Cache
//      |    id    |  flag1  |  flag2  |  flag3  |  flag4  | actual state |
//      | -------- | ------- | ------- | ------- | ------- | ------------ |
//      |  subject |    1    |    1    |    0    |    1    |    matched   |
//      |    d1    |    -    |    -    |    -    |    -    |  not checked |
//      |    d11   |    -    |    -    |    -    |    -    |  not checked |
//      |    d2    |    1    |    1    |    0    |    1    |    matched   |
//      |    d21   |    -    |    -    |    -    |    -    |  not checked |
//      |    d22   |    1    |    0    |    1    |    0    |  not matched |
//      |    d221  |    -    |    -    |    -    |    -    |  not matched |
//      |    d23   |    -    |    -    |    -    |    -    |  not matched |
//      |    d3    |    1    |    0    |    1    |    0    |  not matched |
//      |    d31   |    -    |    -    |    -    |    -    |  not matched |
//      |    d4    |    -    |    -    |    -    |    -    |  not matched |
//
//  - How to check elements that are not in the cache.
//    - d1 :   1. Check parent(subject). Parent is 'SomeChildrenChecked'.
//             2. Traverse to previous siblings to find an element with the
//                flag3 (AllDescendantsOrNextSiblingsChecked).
//             >> not checked because no previous sibling with the flag set.
//    - d11 :  1. Check parent(d1). Parent is not cached.
//             2. Traverse to the parent(p1).
//             3. Check parent(subject). Parent is 'SomeChildrenChecked'.
//             4. Traverse to previous siblings to find an element with the
//                flag3 (AllDescendantsOrNextSiblingsChecked).
//             >> not checked because no previous sibling with the flag set.
//    - d21 :  1. Check parent(d2). Parent is 'SomeChildrenChecked'.
//             2. Traverse to previous siblings to find an element with the
//                flag3 (AllDescendantsOrNextSiblingsChecked).
//             >> not checked because no previous sibling with the flag set.
//    - d221 : 1. Check parent(d2).
//                Parent is 'AllDescendantsOrNextSiblingsChecked'.
//             >> not matched
//    - d23 :  1. Check parent(d2). Parent is 'SomeChildrenChecked'.
//             2. Traverse to previous siblings to find an element with the
//                flag3 (AllDescendantsOrNextSiblingsChecked).
//             >> not matched because d22 is
//                'AllDescendantsOrNextSiblingsChecked'.
//    - d31 :  1. Check parent(d3).
//                Parent is 'AllDescendantsOrNextSiblingsChecked'.
//             >> not matched
//    - d4 :   1. Check parent(subject). Parent is 'SomeChildrenChecked'.
//             2. Traverse to previous siblings to find an element with the
//                flag3 (AllDescendantsOrNextSiblingsChecked).
//             >> not matched because d3 is
//                'AllDescendantsOrNextSiblingsChecked'.
//
// Please refer the check_pseudo_has_cache_scope_context_test.cc for other
// cases.
using CheckPseudoHasResult = uint8_t;
constexpr CheckPseudoHasResult kCheckPseudoHasResultNotCached = 0;
constexpr CheckPseudoHasResult kCheckPseudoHasResultChecked = 1 << 0;
constexpr CheckPseudoHasResult kCheckPseudoHasResultMatched = 1 << 1;
constexpr CheckPseudoHasResult
    kCheckPseudoHasResultAllDescendantsOrNextSiblingsChecked = 1 << 2;
constexpr CheckPseudoHasResult kCheckPseudoHasResultSomeChildrenChecked = 1
                                                                          << 3;

// The :has() result cache keeps the :has() pseudo class checking result
// regardless of the :has() pseudo class location (whether it is for subject or
// not).
// (e.g. '.a:has(.b) .c', '.a .b:has(.c)', ':is(.a:has(.b) .c) .d', ...)
//
// It stores the checking result of a :has() pseudo class. For example, when we
// have the selector '.a:has(.b) .c', during the selector checking sequence,
// checking result for ':has(.b)' will be inserted into the cache.
//
// To differentiate multiple :has() pseudo classes, the argument selector
// text is selected as a cache key. For example, if we already have the result
// of ':has(.a)' in the cache with cache key '.a', and we have the selectors
// '.b:has(.a) .c' and '.b .c:has(a)' to be checked, then the selector checking
// overhead of those 2 selectors will be similar with the overhead of '.a .c'
// because we can get the result of ':has(.a)' from the cache with the cache
// key '.a'.
//
// The :has() checking result cache uses a 2 dimensional hash map to store the
// result.
// - hashmap[<argument-selector>][<element>] = <result>
//
// ElementCheckPseudoHasResultMap is a hash map that stores the
// :has(<argument-selector>) checking result on each element.
// - hashmap[<element>] = <result>
using ElementCheckPseudoHasResultMap =
    GCedHeapHashMap<Member<const Element>, CheckPseudoHasResult>;
using CheckPseudoHasResultCache =
    HeapHashMap<String, Member<ElementCheckPseudoHasResultMap>>;

// The :has() result cache keeps a bloom filter for rejecting :has() argument
// selector checking.
//
// The element identifier hashes in the bloom filter depend on the relationship
// between the :has() anchor element and the :has() argument subject element.
// The relationship can be categorized by this information in
// CheckPseudoHasArgumentContext.
// - traversal scope
// - adjacent limit
// - depth limit
// (Please refer the comment of CheckPseudoHasArgumentTraversalType)
//
// The CheckPseudoHasFastRejectFilterCache uses a 2 dimensional hash map to
// store the filter.
// - hashmap[<traversal type>][<element>] = <filter>
//
// ElementCheckPseudoHasFastRejectFilterMap is a hash map that stores the
// filter for each element.
// - hashmap[<element>] = <filter>
using ElementCheckPseudoHasFastRejectFilterMap =
    GCedHeapHashMap<Member<const Element>,
                    std::unique_ptr<CheckPseudoHasFastRejectFilter>>;
using CheckPseudoHasFastRejectFilterCache =
    HeapHashMap<CheckPseudoHasArgumentTraversalType,
                Member<ElementCheckPseudoHasFastRejectFilterMap>>;

// CheckPseudoHasCacheScope is the stack-allocated scoping class for :has()
// pseudo class checking result cache and :has() pseudo class checking fast
// reject filter cache. It also manages checking for recursive :has().
//
// This class has hashmap to hold the checking result and filter, so the
// lifecycle of the caches follow the lifecycle of the CheckPseudoHasCacheScope
// instance. (The hashmap for caching will be created at the construction of a
// CheckPseudoHasCacheScope instance, and removed at the destruction of the
// instance)
//
// void SomeFunction() {
//   CheckPseudoHasCacheScope cache_scope; // A cache will be created here.
//   // Can use the created cache here.
// } // The cache will be deleted here.
//
// The scope instance is allocated in the function-call stack, so the
// allocation can be nested. In this case, nested cache scope should not
// override the previous cache scope for a better cache hit ratio.
//
// void SomeFunction2() {
//   CheckPseudoHasCacheScope cache_scope2;
//   // Use the cache in the cache_scope1.
//   // The cache in the cache_scope2 will not be used.
// }
//
// void SomeFunction1() {
//   CheckPseudoHasCacheScope cache_scope1;
//   // Use the cache in the cache_scope1
//   SomeFunction2();
// }
//
// To make this simple, the first allocated instance on the call stack will
// be held in the Document instance. (The instance registers itself in the
// constructor and deregisters itself in the destructor) This is based on
// the restriction : The CheckPseudoHasCacheScope is allowed to use only in the
// sequences on the blink main thread.
//
// The cached results are valid until the DOM doesn't mutate, so any DOM
// mutations inside the cache scope is not allowed for the consistency.
class CORE_EXPORT CheckPseudoHasCacheScope {
  STACK_ALLOCATED();

 public:
  // If within_selector_checking is false, we are just setting up the cache for
  // later convenience. However, if it is true, we are actually within matching
  // a selector, and document->IsInPseudoHasChecking() will return true while
  // this object is in scope. This is used to make sure we do not run :has()
  // within :has(), which isn't allowed. (It is typically disallowed by parsing,
  // but it can be constructed through nesting.)
  CheckPseudoHasCacheScope(Document* document, bool within_selector_checking);
  ~CheckPseudoHasCacheScope();

  // Context provides getter and setter of the following cache items.
  // - :has() pseudo class checking result in ElementCheckPseudoHasResultMap
  // - :has() pseudo class checking fast reject filter in
  //   ElementCheckPseudoHasFastRejectFilterMap.
  class CORE_EXPORT Context {
    STACK_ALLOCATED();

   public:
    Context() = delete;
    Context(const Document*, const CheckPseudoHasArgumentContext&);

    CheckPseudoHasResult SetMatchedAndGetOldResult(Element* element);

    void SetChecked(Element* element);

    void SetAllTraversedElementsAsChecked(Element* last_traversed_element,
                                          int last_traversed_depth);

    CheckPseudoHasResult GetResult(Element*) const;

    bool AlreadyChecked(Element*) const;

    CheckPseudoHasFastRejectFilter& EnsureFastRejectFilter(Element*,
                                                           bool& is_new_entry);

    inline bool CacheAllowed() const { return cache_allowed_; }

   private:
    friend class CheckPseudoHasCacheScopeContextTest;

    CheckPseudoHasResult SetResultAndGetOld(Element*,
                                            CheckPseudoHasResult result);

    void SetTraversedElementAsChecked(Element* traversed_element,
                                      Element* parent);

    bool HasSiblingsWithAllDescendantsOrNextSiblingsChecked(Element*) const;
    bool HasAncestorsWithAllDescendantsOrNextSiblingsChecked(Element*) const;

    size_t GetResultCacheCountForTesting() const {
      return cache_allowed_ ? result_map_->size() : 0;
    }

    size_t GetFastRejectFilterCacheCountForTesting() const {
      return cache_allowed_ ? fast_reject_filter_map_->size() : 0;
    }

    size_t GetBloomFilterAllocationCountForTesting() const;

    bool cache_allowed_;
    ElementCheckPseudoHasResultMap* result_map_;
    ElementCheckPseudoHasFastRejectFilterMap* fast_reject_filter_map_;
    const CheckPseudoHasArgumentContext& argument_context_;
  };

 private:
  static ElementCheckPseudoHasResultMap&
  GetResultMap(const Document*, const CSSSelector*, const ContainerNode* scope);
  static ElementCheckPseudoHasFastRejectFilterMap& GetFastRejectFilterMap(
      const Document*,
      CheckPseudoHasArgumentTraversalType);

  CheckPseudoHasResultCache& GetResultCache() { return result_cache_; }

  CheckPseudoHasFastRejectFilterCache& GetFastRejectFilterCache() {
    return fast_reject_filter_cache_;
  }

  CheckPseudoHasResultCache result_cache_;
  CheckPseudoHasFastRejectFilterCache fast_reject_filter_cache_;

  Document* document_;
  const bool within_selector_checking_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CHECK_PSEUDO_HAS_CACHE_SCOPE_H_
