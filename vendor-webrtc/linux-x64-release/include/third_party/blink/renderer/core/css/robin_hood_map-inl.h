// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// Inline definitions for robin_hood_map.h.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_INL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_INL_H_

#include "third_party/blink/renderer/core/css/robin_hood_map.h"

#include <algorithm>

#include "base/notreached.h"

namespace blink {

template <class Key, class Value>
typename RobinHoodMap<Key, Value>::Bucket*
RobinHoodMap<Key, Value>::InsertInternal(
    RobinHoodMap<Key, Value>::Bucket to_insert) {
  Bucket* bucket = FindBucket(to_insert.key);
  Bucket* ret = nullptr;
  ptrdiff_t distance = 0;
  while (!bucket->key.IsNull()) {
    // Robin Hood hashing: A technique for reducing the maximum distances
    // from the home bucket (which, in our case, means we need to rehash
    // less often). When we want to insert an element A into a bucket
    // that is already occupied by element B, we check whether A or B
    // is furthest away from their respective home buckets. If it's B,
    // we just keep on moving down, but if it's A, it's better to insert A
    // in that bucket, and then rather continue the insertion process with B
    // (i.e., we swap A and B).
    ptrdiff_t other_distance = bucket - FindBucket(bucket->key);
    if (distance > other_distance) {
      using std::swap;
      if (ret == nullptr) {
        ret = bucket;
      }
      swap(to_insert, *bucket);
      distance = other_distance;
    }
    UNSAFE_TODO(++bucket);
    ++distance;
    if (static_cast<unsigned>(distance) >= kPossibleBucketsPerKey) {
      // Insertion failed. Stick it in the spare bucket at the very bottom,
      // so that we don't lose it, but the caller will need to rehash.
      DCHECK(buckets_[num_buckets_ + kPossibleBucketsPerKey - 1].key.IsNull());
      buckets_[num_buckets_ + kPossibleBucketsPerKey - 1] = to_insert;
      return nullptr;
    }
  }
  *bucket = std::move(to_insert);
  if (ret == nullptr) {
    ret = bucket;
  }
  return ret;
}

template <class Key, class Value>
typename RobinHoodMap<Key, Value>::Bucket* RobinHoodMap<Key, Value>::Insert(
    const Key& key) {
  unsigned hash = key.Hash();
  pre_filter_ |= 1ULL << (hash & 63);
  pre_filter_ |= 1ULL << ((hash >> 6) & 63);

  Bucket* bucket = InsertInternal({key, {}});
  if (bucket != nullptr) {
    // Normal, happy path.
    return bucket;
  }

  return InsertWithRehashing(key);
}

template <class Key, class Value>
RobinHoodMap<Key, Value> RobinHoodMap<Key, Value>::Grow() {
  double new_size = num_buckets_ * kGrowthFactor;
  CHECK_LE(new_size + kPossibleBucketsPerKey,
           std::numeric_limits<unsigned>::max())
      << "This should never happen with 24-bit hashes";

  RobinHoodMap new_ht(new_size);
  for (RobinHoodMap::Bucket& bucket : *this) {
    if (bucket.key.IsNull()) {
      continue;
    }
    if (new_ht.InsertInternal(std::move(bucket)) == nullptr) {
      // Insertion failed, so try increasing recursively.
      new_ht = new_ht.Grow();
    }
  }
  new_ht.pre_filter_ = pre_filter_;
  return new_ht;
}

template <class Key, class Value>
typename RobinHoodMap<Key, Value>::Bucket*
RobinHoodMap<Key, Value>::InsertWithRehashing(const Key& key) {
  // There was no room for the element in the regular hash table.
  // It's still there, just in a special bucket that Find() won't see,
  // so we don't need to re-insert it; but we do need to rehash.
  // Before that, though, we'll check if rehashing would actually help;
  // it would not if we already have kPossibleBucketsPerKey elements
  // with the exact same hash value (i.e., someone is mounting an
  // attack on the hash table). Due to our existing bounded-probe-length
  // invariant, we know exactly what buckets they must be in,
  // so we can check that very quickly.
  {
    Bucket* bucket = FindBucket(key);
    bool rehashing_would_help = false;
    for (unsigned i = 0; i < kPossibleBucketsPerKey;
         ++i, UNSAFE_TODO(++bucket)) {
      if (bucket->key.Hash() != key.Hash()) {
        rehashing_would_help = true;
        break;
      }
    }
    if (!rehashing_would_help) {
      // Remove the element from the sentinel bucket (we know it must
      // be the one we tried to insert, since we already checked that
      // the ones we skipped over have the same hash and thus
      // the same distance).
      // This leaves the hash table back into a consistent state.
      Bucket* sentinel = &buckets_[num_buckets_ + kPossibleBucketsPerKey - 1];
      DCHECK_EQ(sentinel->key, key);
      sentinel->key = AtomicString();
      return nullptr;
    }
  }

  // No room, so try to increase the size of the hash table.
  // Note that the element is there, just in a special bucket that
  // Find() won't see, so we don't need to re-insert it;
  // but we do need to rehash.
  *this = Grow();

  // Find out where the element ended up (it's hard to keep track of where
  // everything moved during the rehashing).
  Bucket* bucket = FindBucket(key);
  for (unsigned i = 0; i < kPossibleBucketsPerKey; ++i, UNSAFE_TODO(++bucket)) {
    if (bucket->key == key) {
      return bucket;
    }
  }
  NOTREACHED();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_INL_H_
