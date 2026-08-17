// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FIXED_SIZE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FIXED_SIZE_CACHE_H_

// A cache of fixed size, which will automatically evict members
// when there is no room for them. This is a simple 2-way associative
// cache; i.e., every element can go into one out of two neighboring
// slots. An inserted element is always overwriting whatever is in
// slot 1 (unless slot 0 is empty); on a successful lookup,
// it is moved to slot 0. This gives preference to the elements that are
// actually used, and the scheme is simple enough that it's faster than
// using a standard HashMap.
//
// There are no heap allocations after the initial setup. Deletions
// and overwrites (inserting the same key more than once) are not
// supported. Uses the given hash traits, so you should never try to
// insert or search for EmptyValue(). It can hold Oilpan members.

#include <stdint.h>

#include <algorithm>
#include <array>
#include <utility>

#include "base/check.h"
#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"

namespace blink {

template <class Key,
          class Value,
          class KeyTraits = HashTraits<Key>,
          unsigned cache_size = 512>
class FixedSizeCache {
  DISALLOW_NEW();

  static_assert((cache_size & (cache_size - 1)) == 0,
                "cache_size should be a power of two");
  static_assert(cache_size >= 2);

 public:
  FixedSizeCache() {
    cache_.reserve(cache_size);
    for (unsigned i = 0; i < cache_size; ++i) {
      cache_.emplace_back(KeyTraits::EmptyValue(), Value());
    }
  }

  void Trace(Visitor* visitor) const { visitor->Trace(cache_); }

  Value* Find(const Key& key) { return Find(key, KeyTraits::GetHash(key)); }

  // Returns nullptr if not found.
  Value* Find(const Key& key, unsigned hash) {
    DCHECK_NE(KeyTraits::EmptyValue(), key);
    DCHECK_EQ(KeyTraits::GetHash(key), hash);
    unsigned bucket_set = (hash % cache_size) & ~1;
    uint8_t prefilter_hash = GetPrefilterHash(hash);

    // Search, moving to front if we find a match.
    if (UNSAFE_TODO(prefilter_[bucket_set]) == prefilter_hash &&
        cache_[bucket_set].first == key) {
      return &cache_[bucket_set].second;
    }
    if (UNSAFE_TODO(prefilter_[bucket_set + 1]) == prefilter_hash &&
        cache_[bucket_set + 1].first == key) {
      using std::swap;
      swap(UNSAFE_TODO(prefilter_[bucket_set]),
           UNSAFE_TODO(prefilter_[bucket_set + 1]));
      swap(cache_[bucket_set], cache_[bucket_set + 1]);
      return &cache_[bucket_set].second;
    }
    return nullptr;
  }

  Value& Insert(const Key& key, const Value& value) {
    return Insert(key, value, KeyTraits::GetHash(key));
  }

  // Returns a reference to the newly inserted value.
  Value& Insert(const Key& key, const Value& value, unsigned hash) {
    DCHECK_NE(KeyTraits::EmptyValue(), key);
    DCHECK_EQ(KeyTraits::GetHash(key), hash);
    unsigned slot = (hash % cache_size) & ~1;

    // Overwrites are not supported (if so, use Find()
    // and modify the resulting value).
    DCHECK_NE(cache_[slot].first, key);
    DCHECK_NE(cache_[slot + 1].first, key);

    if (UNSAFE_TODO(prefilter_[slot]) != 0) {  // Not empty.
      ++slot;
    }
    UNSAFE_TODO(prefilter_[slot]) = GetPrefilterHash(hash);
    cache_[slot] = std::pair(key, value);
    return cache_[slot].second;
  }

 private:
  uint8_t GetPrefilterHash(unsigned hash) {
    // Use the bits we didn't use for the bucket set.
    return ((hash / cache_size) & 0xff) | 1;
  }

  // Contains some extra bits of the hash (those not used for bucketing),
  // as an extra filter before operator==, which may be expensive.
  // This is especially useful in the case where we keep missing the cache,
  // and don't want to burn the CPU's L1 cache on repeated useless lookups
  // into cache_, especially if Key or Value are large. (This is why it's
  // kept as a separate array.)
  //
  // The lower bit is always set to 1 for a non-empty value.
  std::array<uint8_t, cache_size> prefilter_ = {0};

  HeapVector<std::pair<Key, Value>> cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FIXED_SIZE_CACHE_H_
