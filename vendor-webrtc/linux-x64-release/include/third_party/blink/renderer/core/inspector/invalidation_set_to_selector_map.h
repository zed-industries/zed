// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INVALIDATION_SET_TO_SELECTOR_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INVALIDATION_SET_TO_SELECTOR_MAP_H_

#include "third_party/blink/renderer/core/css/active_style_sheets.h"
#include "third_party/blink/renderer/core/css/style_rule.h"
#include "third_party/blink/renderer/core/inspector/style_rule_to_style_sheet_contents_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class InvalidationSet;
class RuleFeatureSet;
class StyleEngine;
class StyleRule;

// Implements a back-mapping from InvalidationSet entries to the selectors that
// placed them there, for use in diagnostic traces.
// Only active while the appropriate tracing configuration is enabled.
class CORE_EXPORT InvalidationSetToSelectorMap final
    : public GarbageCollected<InvalidationSetToSelectorMap> {
 public:
  // A small helper to bundle together a StyleRule plus an index into its
  // selector list.
  class CORE_EXPORT IndexedSelector final
      : public GarbageCollected<IndexedSelector> {
   public:
    IndexedSelector(StyleRule* style_rule, unsigned selector_index);
    void Trace(Visitor*) const;
    StyleRule* GetStyleRule() const;
    unsigned GetSelectorIndex() const;
    String GetSelectorText() const;
    const StyleSheetContents* GetStyleSheetContents() const;

   private:
    friend struct WTF::HashTraits<
        blink::InvalidationSetToSelectorMap::IndexedSelector>;
    Member<StyleRule> style_rule_;
    unsigned selector_index_;
  };
  using IndexedSelectorList = GCedHeapHashSet<Member<IndexedSelector>>;

  enum class SelectorFeatureType {
    kUnknown,
    kClass,
    kId,
    kTagName,
    kAttribute,
    kWholeSubtree
  };

  // Instantiates a new mapping if a diagnostic tracing session with the
  // appropriate configuration has started, or deletes an existing mapping if
  // tracing is no longer enabled.
  static void StartOrStopTrackingIfNeeded(const TreeScope& tree_scope,
                                          const StyleEngine& style_engine);

  // Returns true if a mapping is active and tracking invalidations.
  // This is primarily intended for tests. Product code generally should not
  // need to call this; the other static entry points will check this state and
  // immediately return if tracking is not active.
  static bool IsTracking();

  // Call at the start and end of indexing rules within a StyleSheetContents.
  static void BeginStyleSheetContents(const StyleSheetContents* contents);
  static void EndStyleSheetContents();

  // Helper object for a Begin/EndStylesheet pair.
  class StyleSheetContentsScope {
   public:
    explicit StyleSheetContentsScope(const StyleSheetContents* contents);
    ~StyleSheetContentsScope();
  };

  // Call at the start and end of indexing features for a given selector.
  static void BeginSelector(StyleRule* style_rule, unsigned selector_index);
  static void EndSelector();

  // Helper object for a Begin/EndSelector pair.
  class SelectorScope {
   public:
    SelectorScope(StyleRule* style_rule, unsigned selector_index);
    ~SelectorScope();
  };

  // Call for each feature recorded to an invalidation set.
  static void RecordInvalidationSetEntry(
      const InvalidationSet* invalidation_set,
      SelectorFeatureType type,
      const AtomicString& value);

  // Call at the start and end of an invalidation set combine operation.
  static void BeginInvalidationSetCombine(const InvalidationSet* target,
                                          const InvalidationSet* source);
  static void EndInvalidationSetCombine();

  // Helper object for a Begin/EndInvalidationSetCombine pair.
  class CombineScope {
   public:
    CombineScope(const InvalidationSet* target, const InvalidationSet* source);
    ~CombineScope();
  };

  // Call when an invalidation set is no longer in use, for example when it is
  // being destroyed.
  static void RemoveEntriesForInvalidationSet(
      const InvalidationSet* invalidation_set);

  // Given an invalidation set and a selector feature representing an entry in
  // that invalidation set, returns a list of selectors that contributed to that
  // entry existing in that invalidation set.
  static const IndexedSelectorList* Lookup(
      const InvalidationSet* invalidation_set,
      SelectorFeatureType type,
      const AtomicString& value);

  // Given a StyleRule, attempt to look up the containing StyleSheetContents.
  static const StyleSheetContents* LookupStyleSheetContentsForRule(
      const StyleRule* style_rule);

  InvalidationSetToSelectorMap();
  void Trace(Visitor*) const;

 protected:
  friend class InvalidationSetToSelectorMapTest;
  static Persistent<InvalidationSetToSelectorMap>& GetInstanceReference();

  void RevisitActiveStyleSheets(
      const ActiveStyleSheetVector& active_style_sheets,
      const StyleEngine& style_engine);
  void RevisitStylesheetOnce(const StyleEngine* style_engine,
                             StyleSheetContents* contents,
                             const RuleFeatureSet* features);

 private:
  // The back-map is stored in two levels: first from an invalidation set
  // pointer to a map of entries, then from each entry to a list of selectors.
  // We don't retain a strong pointer to the InvalidationSet because we don't
  // need it for any purpose other than as a lookup key, and because extra refs
  // on InvalidationSets may cause copy-on-writes that diverge from untraced
  // execution.
  using InvalidationSetEntry = std::pair<SelectorFeatureType, AtomicString>;
  using InvalidationSetEntryMap =
      GCedHeapHashMap<InvalidationSetEntry, Member<IndexedSelectorList>>;
  using InvalidationSetMap =
      GCedHeapHashMap<const InvalidationSet*, Member<InvalidationSetEntryMap>>;

  // Holds the back-map described above.
  Member<InvalidationSetMap> invalidation_set_map_;

  // Holds the set of stylesheets that have been revisited for indexing into
  // the back-map.
  HeapHashSet<Member<const StyleSheetContents>> revisited_style_sheets_;

  // Used during back-map construction.
  // Holds the stylesheet currently being analyzed.
  Member<const StyleSheetContents> current_style_sheet_contents_;

  // Used during back-map construction.
  // Holds the selector currently being analyzed.
  Member<IndexedSelector> current_selector_;

  // Used during back-map construction.
  // Tracks how deeply we've recursed on InvalidationSet::Combine() operations.
  unsigned combine_recursion_depth_ = 0;

  // Holds a table for mapping rules back to sheets on behalf of inner class
  // IndexedSelector.
  Member<StyleRuleToStyleSheetContentsMap> style_rule_to_sheet_map_;
};

}  // namespace blink

namespace WTF {

// These two HashTraits specializations are needed so that
// HeapHashSet<Member<IndexedSelector>> will do value-wise comparisons instead
// of pointer-wise comparisons.
template <>
struct HashTraits<blink::InvalidationSetToSelectorMap::IndexedSelector>
    : TwoFieldsHashTraits<
          blink::InvalidationSetToSelectorMap::IndexedSelector,
          &blink::InvalidationSetToSelectorMap::IndexedSelector::style_rule_,
          &blink::InvalidationSetToSelectorMap::IndexedSelector::
              selector_index_> {};

template <>
struct HashTraits<
    blink::Member<blink::InvalidationSetToSelectorMap::IndexedSelector>>
    : MemberHashTraits<blink::InvalidationSetToSelectorMap::IndexedSelector> {
  using IndexedSelector = blink::InvalidationSetToSelectorMap::IndexedSelector;
  static unsigned GetHash(const blink::Member<IndexedSelector>& key) {
    return WTF::GetHash(*key);
  }
  static bool Equal(const blink::Member<IndexedSelector>& a,
                    const blink::Member<IndexedSelector>& b) {
    return HashTraits<IndexedSelector>::Equal(*a, *b);
  }

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INVALIDATION_SET_TO_SELECTOR_MAP_H_
