/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCHED_PROPERTIES_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCHED_PROPERTIES_CACHE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/resolver/match_result.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ComputedStyle;
class ComputedStyleBuilder;
class StyleResolverState;

class CORE_EXPORT CachedMatchedProperties final
    : public GarbageCollected<CachedMatchedProperties> {
 public:
  // Caches data of MatchedProperties. See |MatchedPropertiesCache::Cache| for
  // semantics.
  // We use UntracedMember<> here because WeakMember<> would require using a
  // HeapHashSet which is slower to iterate.
  Vector<
      std::pair<UntracedMember<CSSPropertyValueSet>, MatchedProperties::Data>>
      matched_properties;

  struct Entry {
    DISALLOW_NEW();

   public:
    Entry(const ComputedStyle* computed_style_arg,
          const ComputedStyle* parent_computed_style_arg,
          unsigned last_used_arg)
        : computed_style(computed_style_arg),
          parent_computed_style(parent_computed_style_arg),
          last_used(last_used_arg) {}

    // Note that we don't cache the original ComputedStyle instance. It may be
    // further modified. The ComputedStyle in the cache is really just a holder
    // for the substructures and never used as-is.
    Member<const ComputedStyle> computed_style;
    Member<const ComputedStyle> parent_computed_style;
    unsigned last_used;

    void Trace(Visitor* visitor) const {
      visitor->Trace(computed_style);
      visitor->Trace(parent_computed_style);
    }
  };

  HeapVector<Entry, 4> entries;

  CachedMatchedProperties(const ComputedStyle* style,
                          const ComputedStyle* parent_style,
                          const MatchedPropertiesVector&,
                          unsigned clock);

  void Clear();

  void Trace(Visitor* visitor) const { visitor->Trace(entries); }

  bool CorrespondsTo(const MatchedPropertiesVector& lookup_properties) const;
  void RefreshKey(const MatchedPropertiesVector& lookup_properties);
};

class CORE_EXPORT MatchedPropertiesCache {
  DISALLOW_NEW();

 public:
  MatchedPropertiesCache();
  MatchedPropertiesCache(const MatchedPropertiesCache&) = delete;
  MatchedPropertiesCache& operator=(const MatchedPropertiesCache&) = delete;
  ~MatchedPropertiesCache() { DCHECK(cache_.empty()); }

  class CORE_EXPORT Key {
    STACK_ALLOCATED();

   public:
    explicit Key(const MatchResult&);

    bool IsCacheable() const { return result_.IsCacheable(); }

   private:
    friend class MatchedPropertiesCache;
    friend class MatchedPropertiesCacheTestKey;
    friend std::ostream& operator<<(std::ostream&,
                                    MatchedPropertiesCache::Key&);

    Key(const MatchResult&, unsigned hash);

    const MatchResult& result_;
    unsigned hash_;
  };

  const CachedMatchedProperties::Entry* Find(const Key&,
                                             const StyleResolverState&);
  void Add(const Key&, const ComputedStyle*, const ComputedStyle* parent_style);

  void Clear();
  void ClearViewportDependent();

  static bool IsCacheable(const StyleResolverState&);
  static bool IsStyleCacheable(const ComputedStyleBuilder&);

  void Trace(Visitor*) const;

 private:
  // The cache is mapping a hash to a cached entry where the entry is kept as
  // long as *all* properties referred to by the entry are alive. This requires
  // custom weakness which is managed through |CleanMatchedPropertiesCache|.
  //
  // Note that this cache is keyed somewhat funny; the actual key is stored
  // in the value (the first entries of CachedMatchedProperties), while the
  // HashMap key is the hash of these values. This is because it turned out
  // to be hard to make this performant in any other way; HashMap does not deal
  // well with complex keys. Of course, it means we are vulnerable to hash
  // collisions, in that we cannot store more than one different cache entry
  // with the same hash.
  using Cache = HeapHashMap<unsigned,
                            Member<CachedMatchedProperties>,
                            AlreadyHashedTraits>;

  void CleanMatchedPropertiesCache(const LivenessBroker&);

  // Erase all MPC entries where the given predicate returns true,
  // and updates the counter. Removes all keys that have no entries left.
  template <class Predicate>
  void EraseEntriesIf(Predicate&& pred);

  Cache cache_;
  unsigned cache_entries_ = 0;

  // Virtual clock for LRU purposes (last_used in each CachedMatchedProperties).
  // If this wraps (more than four billion lookups or inserts), we will evict
  // the wrong entries, but this only matters for performance, not correctness.
  unsigned clock_ = 0;
};

// For debugging only.
std::ostream& operator<<(std::ostream&, MatchedPropertiesCache::Key&);

}  // namespace blink

namespace WTF {

template <>
struct VectorTraits<blink::CachedMatchedProperties::Entry>
    : VectorTraitsBase<blink::CachedMatchedProperties::Entry> {
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  // HeapVector is evidently not concurrent.
  static constexpr bool kCanTraceConcurrently = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_RESOLVER_MATCHED_PROPERTIES_CACHE_H_
