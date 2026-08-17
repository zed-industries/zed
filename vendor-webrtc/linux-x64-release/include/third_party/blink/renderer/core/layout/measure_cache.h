// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MEASURE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MEASURE_CACHE_H_

#include <optional>

#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class LayoutResult;
class ConstraintSpace;
class BlockNode;
struct FragmentGeometry;

// Implements an N-way LRU cache for "measure" layout results.
//
// Some layout algorithms (grid in particular) will measure an element multiple
// times with different constraint spaces.
//
// This cache is designed to handle these multiple measure passes.
class MeasureCache final : public GarbageCollected<MeasureCache> {
 public:
  // A single layout pass of[1] can add up to 6 entries into this cache due to
  // grid's multi-pass algorithm.
  //
  // [1] perf_tests/layout/grid-with-block-constraints-dependence.html
  static constexpr unsigned kMaxCacheEntries = 8;

  void Trace(Visitor* visitor) const { visitor->Trace(cache_); }

  // Finds a layout result match. Performs a full size-based cache test,
  // potentially populating `fragment_geometry`.
  const LayoutResult* Find(const BlockNode& node,
                           const ConstraintSpace& new_space,
                           std::optional<FragmentGeometry>* fragment_geometry);

  void Add(const LayoutResult* result);

  void Clear();

  void InvalidateItems();
  void LayoutObjectWillBeDestroyed();
  void SetFragmentChildrenInvalid(const LayoutResult* except);

  const LayoutResult* GetLastForTesting() const;

 private:
  HeapVector<Member<const LayoutResult>, 2> cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_MEASURE_CACHE_H_
