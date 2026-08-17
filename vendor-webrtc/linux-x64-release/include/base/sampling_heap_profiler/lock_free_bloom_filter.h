// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SAMPLING_HEAP_PROFILER_LOCK_FREE_BLOOM_FILTER_H_
#define BASE_SAMPLING_HEAP_PROFILER_LOCK_FREE_BLOOM_FILTER_H_

#include <stddef.h>
#include <stdint.h>

#include <atomic>

#include "base/base_export.h"

namespace base {

// A fixed-size Bloom filter for keeping track of a set of pointers, that can be
// read and written from multiple threads at the same time. The number of bits
// is fixed so that all bits can be held in an atomic integer.
//
// The implementation was inspired by
// components/optimization_guide/core/bloom_filter.h, modified to be thread-safe
// and take void* arguments.
//
// A Bloom filter can determine precisely that a pointer is NOT in the set, but
// hash collisions make it impossible to be certain that a given pointer IS in
// the set. (That is, MaybeContains(ptr) can return false positives but never
// false negatives.) It's intended to be used in front of LockFreeAddressHashSet
// to optimize the common case of looking up a pointer that's not in the hash
// set, as follows:
//
//  To add a key:
//    bloom_filter.Add(ptr);
//    hash_set.Insert(ptr);
//
//  To look up a key:
//    if (bloom_filter.MaybeContains(ptr)) {
//      if (hash_set.Contains(ptr)) {
//        ... Do something.
//      } else {
//        ... False positive. Do nothing.
//      }
//    } else {
//      ... Not in bloom_filter, so not in hash_set. Do nothing.
//    }
//
// This class only guarantees that accessing the Bloom filter is thread-safe.
// The caller is responsible for ensuring that accessing both the filter and
// LockFreeAddressHashSet from multiple threads gives consistent results.
class BASE_EXPORT LockFreeBloomFilter {
 public:
  // An integer to hold the bits that are set in the filter.
  //
  // The estimated false positive rate is approximately (1-e^(-kn/m))^k, where
  // k is the number of hash functions (bits per key), m is the number of bits
  // in the filter, and n is the number of keys. (See
  // https://en.wikipedia.org/wiki/Bloom_filter#Probability_of_false_positives.)
  //
  // Since this implementation uses a fixed size, m is hardcoded to 64. This
  // gives estimated false positives of:
  //
  // k=2, n=5: 2.1%
  // k=2, n=10: 7.2%
  // k=2, n=20: 21.6%
  // k=2, n=40: 50.9%
  // k=2, n=80: 84.3%
  // k=2, n=100: 91.4%
  //
  // k=3, n=5: 0.9%
  // k=3, n=10: 5.2%
  // k=3, n=20: 22.5%
  // k=3, n=40: 60.7%
  // k=3, n=80: 93.1%
  // k=3, n=100: 97.3%
  //
  // k=4, n=5: 0.5%
  // k=4, n=10: 4.7%
  // k=4, n=20: 25.9%
  // k=4, n=40: 71.0%
  // k=4, n=80: 97.3%
  // k=4, n=100: 99.2%
  //
  // Please update this table if the size of BitStorage changes. This can be
  // used to estimate the optimal number of hash functions (k) for the expected
  // number of keys that will be added.
  using BitStorage = uint64_t;

  // Atomic wrapper for the bits.
  using AtomicBitStorage = std::atomic<BitStorage>;
  static_assert(AtomicBitStorage::is_always_lock_free);

  // Maximum number of bits in the filter.
  static constexpr size_t kMaxBits = 8 * sizeof(BitStorage);

  // Constructs a Bloom filter of `kMaxBits` size with zero-ed data and using
  // `num_hash_functions` per entry.
  explicit LockFreeBloomFilter(size_t num_hash_functions);

  LockFreeBloomFilter(const LockFreeBloomFilter&) = delete;
  LockFreeBloomFilter& operator=(const LockFreeBloomFilter&) = delete;

  ~LockFreeBloomFilter();

  // Returns whether `ptr` may have been added as a key in this Bloom filter. If
  // this returns false, `ptr` is definitely not in the filter. Otherwise `ptr`
  // may or may not be in the filter, since Bloom filters inherently have false
  // positives. (See the table of estimated false positive rates above.)
  bool MaybeContains(void* ptr) const;

  // Adds `ptr` as a key in this Bloom filter. After this call
  // MaybeContains(ptr) will always return true.
  void Add(void* ptr);

  // Returns the bit array data of this Bloom filter as an integer.
  BitStorage GetBitsForTesting() const;

  // Sets the bits to a fixed value for testing.
  void SetBitsForTesting(const BitStorage& bits);

  // If called with `true`, hashing a key with hash function N will shift the
  // key N bits to the right, allowing the test to precisely control how keys
  // are hashed. Calling this with `false` will restore the default hash
  // functions.
  void SetFakeHashFunctionsForTesting(bool use_fake_hash_functions) {
    use_fake_hash_functions_ = use_fake_hash_functions;
  }

 private:
  // Number of bits to set for each added key.
  const size_t num_hash_functions_;

  // Bit data for the filter.
  // Accessed with std::memory_order_relaxed since the class doesn't synchronize
  // access to pointed-to memory. Instead pointers passed to Add() are treated
  // as opaque keys.
  AtomicBitStorage bits_ = 0;

  bool use_fake_hash_functions_ = false;
};

}  // namespace base

#endif  // BASE_SAMPLING_HEAP_PROFILER_LOCK_FREE_BLOOM_FILTER_H_
