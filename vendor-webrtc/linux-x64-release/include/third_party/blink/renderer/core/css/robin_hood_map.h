// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_H_

#include <memory>
#include <type_traits>

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Since RuleMap is so performance-critical for us (a large part of style
// is looking up rules in RuleMaps, especially since we have one RuleSet per
// stylesheet and one RuleSet has many RuleMaps), we have implemented our own
// hash table, which gives better lookup performance than WTF::HashMap,
// especially on cache-starved CPUs. We pay for this with some extra code
// and slightly more expensive inserts (and we also don't support deletes,
// although that could be added). The key features of our implementation are:
//
//  - Partition bucketing: No divide/modulo required, only a single 32x32
//    multiplication and shift to map the hash value to a bucket.
//    (This technique was popularized by Daniel Lemire.)
//
//  - Supports any table size (not restricted to power-of-two or prime),
//    due to the above.
//
//  - Open addressing with Robin Hood hashing and a bounded number of probes
//    (based on an idea by Malte Skarupte); makes lookup always O(1),
//    accessing at most three (neighboring) cache lines (assuming 16-byte
//    buckets), typically inlined and unrolled by the compiler.
//
//  - Inline data (not node-based); few allocations, no extra cache misses
//    after finding the element.
//
//  - High density due to Robin Hood hashing; small maps have almost 100%
//    load factor, whereas larger ones tend to go towards 60% or so.
//    No rehashing based solely on load factor; only violating the maximum
//    probe length will cause one.
//
//  - Not robust towards adversary cache collisions; if someone deliberately
//    introduces lots of AtomicStrings with the exact same hash value,
//    the insert will fail. (This of course isn't ideal, but it's a direct
//    consequence of the O(1) lookup bound, and is extremely unlikely
//    to happen on non-adversary data. Based on simulations with random
//    strings and 256k inserts, which is the maximum RuleData supports,
//    we estimate the odds of a 9-collision are very roughly 1 in 2e14.
//    Of course, if you lower kPossibleBucketsPerKey to e.g. 4, you'll
//    only need a 5-collision, which is _much_ more likely.)
//
//  - A tiny Bloom filter to quickly filter out most (~80%+) negative queries.
//
// Possible future extensions:
//
//  - Arbitrary keys (currently supports only AtomicString as key).
//
//  - Using a HeapVector instead of a regular array, allowing to store Oilpan
//    objects as values without using Persistent<> (note that WTF::HashMap
//    only supports Oilpan objects using Member<>, not directly).
//
//  - Full STL-like or WTF-like interface: Better iterators, removals, etc.
//
//  - Packed buckets, to avoid extraneous padding and save yet more cache/RAM
//    (depending, of course, on Value).
template <class Key, class Value>
struct RobinHoodMap {
  static_assert(std::is_same_v<Key, AtomicString>,
                "We currently only support AtomicString as key.");

 public:
  // Number of possible different places a key can be put in.
  // In the extreme case, 1 means that each element can only be
  // in one bucket (its home bucket) and any collision would cause
  // an immediate rehash. 8 means that the element can be in its
  // home bucket or any of the following seven ones.
  //
  // Higher values mean higher load factors (less rehashing,
  // less RAM usage) but slower lookups (more comparisons),
  // potentially to the point of no longer having inlined/unrolled finds.
  static constexpr unsigned kPossibleBucketsPerKey = 6;

  // When rehashing due to excessive collisions, how much to attempt
  // growing by in each step (1.3 means 30% increase). Smaller values
  // (closer to 1.0) mean higher load factors (less RAM used) but also
  // more frequent rehashing, reducing (amortized) insertion speed.
  //
  // There probably is some sort of relationship between this variable,
  // kPossibleBucketsPerKey and the load factor, but this is just set
  // empirically.
  static constexpr double kGrowthFactor = 1.3;

  struct Bucket {
    Key key;
    Value value;
  };

  // Constructs a map that can hold no elements; the only thing
  // you can do with it is check IsNull() (which will be true)
  // and Find() (which will return nullptr, since pre_filter_
  // will be empty).
  RobinHoodMap() = default;
  explicit RobinHoodMap(unsigned size)
      : buckets_(new Bucket[size + kPossibleBucketsPerKey]),
        num_buckets_(size) {}

  bool IsNull() const { return buckets_ == nullptr; }

  Bucket* Find(const Key& key) {
    uint32_t hash = key.Hash();
    bool h1 = (pre_filter_ >> (hash & 63)) & 1;
    bool h2 = (pre_filter_ >> ((hash >> 6) & 63)) & 1;
    if (!h1 || !h2) {
      // NOTE: Since the filter will always be empty
      // if buckets_ is nullptr, we don't need an extra
      // test for buckets_ below (nor in the caller);
      // we'll always hit this path.
      return nullptr;
    }

    Bucket* bucket = FindBucket(key);
    for (unsigned i = 0; i < kPossibleBucketsPerKey;
         ++i, UNSAFE_TODO(++bucket)) {
      if (bucket->key == key) {
        return bucket;
      }
    }
    return nullptr;
  }
  const Bucket* Find(const Key& key) const {
    return const_cast<RobinHoodMap*>(this)->Find(key);
  }

  // Inserts the given key, with a default-constructed value.
  // Returns the bucket it was put in, so that you can change
  // the value yourself.
  //
  // This function may cause rehashing; if rehashing cannot fix
  // the collisions, it will return nullptr.
  //
  // If you use this function, you will need to include robin_hood_map-inl.h.
  ALWAYS_INLINE Bucket* Insert(const Key& key);

  // STL-like iterators.
  class iterator {
   public:
    iterator(Bucket* pos, const Bucket* end) : pos_(pos), end_(end) {
      while (pos_ != end_ && pos_->key.IsNull()) {
        UNSAFE_TODO(++pos_);
      }
    }
    Bucket& operator*() const { return *pos_; }
    Bucket* operator->() const { return pos_; }
    iterator& operator++() {
      UNSAFE_TODO(++pos_);
      while (pos_ != end_ && pos_->key.IsNull()) {
        UNSAFE_TODO(++pos_);
      }
      return *this;
    }
    bool operator==(const iterator& other) const { return pos_ == other.pos_; }
    bool operator!=(const iterator& other) const { return pos_ != other.pos_; }

   private:
    Bucket* pos_;
    const Bucket* end_;
  };
  class const_iterator {
   public:
    const_iterator(const Bucket* pos, const Bucket* end)
        : pos_(pos), end_(end) {
      while (pos_ != end_ && pos_->key.IsNull()) {
        UNSAFE_TODO(++pos_);
      }
    }
    const Bucket& operator*() const { return *pos_; }
    const Bucket* operator->() const { return pos_; }
    const_iterator& operator++() {
      UNSAFE_TODO(++pos_);
      while (pos_ != end_ && pos_->key.IsNull()) {
        UNSAFE_TODO(++pos_);
      }
      return *this;
    }
    bool operator==(const const_iterator& other) const {
      return pos_ == other.pos_;
    }
    bool operator!=(const const_iterator& other) const {
      return pos_ != other.pos_;
    }

   private:
    const Bucket* pos_;
    const Bucket* end_;
  };

  iterator begin() { return {buckets_.get(), EndBucket()}; }
  const_iterator begin() const { return {buckets_.get(), EndBucket()}; }

  iterator end() { return {EndBucket(), EndBucket()}; }
  const_iterator end() const { return {EndBucket(), EndBucket()}; }

 private:
  Bucket* EndBucket() {
    return buckets_.get() ? UNSAFE_TODO(buckets_.get() + num_buckets_ +
                                        kPossibleBucketsPerKey)
                          : nullptr;
  }
  const Bucket* EndBucket() const {
    return buckets_.get() ? UNSAFE_TODO(buckets_.get() + num_buckets_ +
                                        kPossibleBucketsPerKey)
                          : nullptr;
  }
  unsigned FindBucketIndex(const Key& key) const {
    // AtomicString has a 24-bit hash, so we treat it as a number in
    // 0.24 fixed-point, multiply it by the number of buckets and truncate.
    // This gives a fair map to [0,N) based on (mostly) the high bits
    // of the hash, with only a multiplication and shift.
    unsigned bucket =
        static_cast<unsigned>(((uint64_t)key.Hash() * num_buckets_) >> 24);
    DCHECK_LT(bucket, num_buckets_);
    return bucket;
  }

  // Lookup. Finds the home bucket of the given key; you will need to
  // check both this and the next (kPossibleBucketsPerKey - 1) buckets
  // to find the element. This can never overflow; see the definition
  // of buckets_ below.
  Bucket* FindBucket(const Key& key) {
    return UNSAFE_TODO(buckets_.get() + FindBucketIndex(key));
  }
  const Bucket* FindBucket(const Key& key) const {
    return UNSAFE_TODO(buckets_.get() + FindBucketIndex(key));
  }

  // Inserts the given key/value, possibly displacing other buckets in the
  // process, returning where the element was inserted. If it fails
  // (i.e., some element needed to have a distance larger than
  // kPossibleBucketsPerKey would allow), it inserts the element into the
  // special last bucket and returns nullptr. If so, you need to call
  // Grow() immediately.
  ALWAYS_INLINE Bucket* InsertInternal(Bucket to_insert);

  // Returns a new map that is kGrowthFactor times as large as the existing one,
  // moves everything in the current map into that one (including
  // anything that may be in the wrong bucket; in particular the special
  // last bucket used by InsertInternal() on failure) and then returns
  // the new map. Note that if rehashing fails, it may call itself recursively,
  // so that the map may end up yet larger. CHECK-fails if the new map would
  // become so large as to overflow num_buckets_.
  RobinHoodMap Grow();

  // Non-inlined helper function for Insert(); calls Grow(), then tracks
  // where the given key ended up and returns its bucket.
  Bucket* InsertWithRehashing(const Key& key);

  // A tiny Bloom filter that is used as a prefilter for Find().
  // This allows us to quickly return nullptr for queries that are not
  // in the map (which is a fairly common case for us), without having to
  // take the cache miss to actually look up in the map. (We'd get even
  // better filter rates with three hashes instead of two, but it seems
  // to be net-negative in performance.)
  //
  // Note that this filter uses the lower bits of the string hash,
  // which are fairly independent from what FindBucketIndex() uses.
  uint64_t pre_filter_ = 0;

  // The buckets, allocated in the usual way. Note that in addition to the
  // requested number of buckets (num_buckets_), we allocate first
  // (kPossibleBucketsPerKey - 1) extra buckets, so that we can overflow even
  // something that has a home bucket of the last regular one, without having to
  // worry about wrapping. Then, we add yet another one, as an emergency spot
  // for InsertInternal() to write an element in if it fails regular insertion.
  // So in all, this contains (num_buckets_ + kPossibleBucketsPerKey) buckets.
  std::unique_ptr<Bucket[]> buckets_;
  unsigned num_buckets_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_ROBIN_HOOD_MAP_H_
