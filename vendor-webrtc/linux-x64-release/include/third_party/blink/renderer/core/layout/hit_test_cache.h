// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CACHE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/hit_test_location.h"
#include "third_party/blink/renderer/core/layout/hit_test_result.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// This object implements a cache for storing successful hit tests to DOM nodes
// in the visible viewport. The cache is cleared on dom modifications,
// scrolling, CSS style modifications.
//
// Multiple hit tests can occur when processing events. Typically the DOM
// doesn't change when each event is processed so in order to decrease the time
// spent processing the events a hit cache is useful. For example a GestureTap
// event will generate a series of simulated mouse events (move, down, up,
// click) with the same co-ordinates and ideally we'd like to do the hit test
// once and use the result for the targetting of each event.
//
// Some of the related design, motivation can be found in:
// https://docs.google.com/document/d/1b0NYAD4S9BJIpHGa4JD2HLmW28f2rUh1jlqrgpU3zVU/
//

// A cache size of 2 is used because it is relatively cheap to store;
// and the ping-pong behaviour of some of the HitTestRequest flags during
// Mouse/Touch/Pointer events can generate increased cache misses with
// size of 1.
#define HIT_TEST_CACHE_SIZE (2)

struct HitTestCacheEntry {
  DISALLOW_NEW();

  void Trace(Visitor*) const;
  HitTestLocation location;
  HitTestResult result;

  void CacheValues(const HitTestCacheEntry&);
};

class CORE_EXPORT HitTestCache final : public GarbageCollected<HitTestCache> {
 public:
  HitTestCache() : update_index_(0), dom_tree_version_(0) {}
  HitTestCache(const HitTestCache&) = delete;
  HitTestCache& operator=(const HitTestCache&) = delete;

  // Check the cache for a possible hit and update |result| if
  // hit encountered; returning true. Otherwise false.
  bool LookupCachedResult(const HitTestLocation&,
                          HitTestResult&,
                          uint64_t dom_tree_version);

  void Clear();

  // Adds a HitTestResult to the cache.
  void AddCachedResult(const HitTestLocation&,
                       const HitTestResult&,
                       uint64_t dom_tree_version);

  void Trace(Visitor*) const;

 private:
  unsigned update_index_;

  HeapVector<HitTestCacheEntry, HIT_TEST_CACHE_SIZE> items_;
  uint64_t dom_tree_version_;
};

}  // namespace blink

WTF_ALLOW_CLEAR_UNUSED_SLOTS_WITH_MEM_FUNCTIONS(blink::HitTestCacheEntry)

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_HIT_TEST_CACHE_H_
