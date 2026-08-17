// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_CACHE_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/min_max_sizes.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"

namespace blink {

// Implements an N-way LRU cache for min/max sizes.
//
// Some layout algorithms (grid in particular) query the min/max sizes of an
// element multiple times with different initial block-size each time.
//
// These sizes can differ when there is something dependent on that size -
// an element with an aspect-ratio with "height:100%" for example.
//
// This cache is designed to handle these cases.
class MinMaxSizesCache final : public GarbageCollected<MinMaxSizesCache> {
 public:
  // A single layout pass of[1] can add up to 10 entries into this cache due to
  // grid's multi-pass algorithm.
  //
  // [1] perf_tests/layout/grid-with-block-constraints-dependence.html
  static constexpr unsigned kMaxCacheEntries = 8;

  void Trace(Visitor*) const {}

  // NOTE: To keep this struct small we unpack the `MinMaxSizesResult`.
  struct Entry {
    MinMaxSizes sizes;
    LayoutUnit initial_block_size;
    bool depends_on_block_constraints;
  };

  // Given an initial block-size returns a min/max sizes result if one matches.
  std::optional<MinMaxSizesResult> Find(LayoutUnit initial_block_size) {
    DCHECK_NE(initial_block_size, kIndefiniteSize);
    for (auto it = cache_.rbegin(); it != cache_.rend(); ++it) {
      if (it->initial_block_size == initial_block_size) {
        if (it == cache_.rbegin()) {
          return MinMaxSizesResult(it->sizes, it->depends_on_block_constraints);
        }

        // Shift this result to the back of the cache.
        auto copy = *it;
        cache_.EraseAt(
            static_cast<wtf_size_t>(std::distance(it, cache_.rend())) - 1u);
        cache_.push_back(copy);

        return MinMaxSizesResult(copy.sizes, copy.depends_on_block_constraints);
      }
    }
    return std::nullopt;
  }

  // Adds a result to the cache. NOTE: the entry shouldn't already exist.
  void Add(const MinMaxSizes& sizes,
           LayoutUnit initial_block_size,
           bool depends_on_block_constraints) {
#if EXPENSIVE_DCHECKS_ARE_ON()
    // We shouldn't be adding a duplicate key - we should've had a hit instead.
    for (const auto& entry : cache_) {
      DCHECK_NE(entry.initial_block_size, initial_block_size);
    }
#endif

    // Trim the cache if its going to exceed its max entries.
    if (cache_.size() == kMaxCacheEntries) {
      cache_.EraseAt(0);
    }

    cache_.push_back(
        Entry{sizes, initial_block_size, depends_on_block_constraints});
  }

  void Clear() { cache_.resize(0); }

 private:
  Vector<Entry, 2> cache_;
};

}  // namespace blink

WTF_ALLOW_MOVE_INIT_AND_COMPARE_WITH_MEM_FUNCTIONS(
    blink::MinMaxSizesCache::Entry)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MIN_MAX_SIZES_CACHE_H_
