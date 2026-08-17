// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_RAND_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_RAND_UTIL_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc {
class RandomGenerator;

namespace internal {
class LightweightQuarantineBranch;
}
}  // namespace partition_alloc

namespace partition_alloc::internal::base {

// Returns a random number in range [0, UINT64_MAX]. Thread-safe.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) uint64_t RandUint64();

// Returns a random number in range [0, range).  Thread-safe.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
uint64_t RandGenerator(uint64_t range);

// Fills |output_length| bytes of |output| with random data. Thread-safe.
//
// Although implementations are required to use a cryptographically secure
// random number source, code outside of base/ that relies on this should use
// crypto::RandBytes instead to ensure the requirement is easily discoverable.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
void RandBytes(void* output, size_t output_length);

// Fast, insecure pseudo-random number generator.
//
// WARNING: This is not the generator you are looking for. This has significant
// caveats:
//   - It is non-cryptographic, so easy to miuse
//   - It is neither fork() nor clone()-safe.
//   - Synchronization is up to the client.
//
// Always prefer base::Rand*() above, unless you have a use case where its
// overhead is too high, or system calls are disallowed.
//
// Performance: As of 2021, rough overhead on Linux on a desktop machine of
// base::RandUint64() is ~800ns per call (it performs a system call). On Windows
// it is lower. On the same machine, this generator's cost is ~2ns per call,
// regardless of platform.
//
// This is different from |Rand*()| above as it is guaranteed to never make a
// system call to generate a new number, except to seed it.  This should *never*
// be used for cryptographic applications, and is not thread-safe.
//
// It is seeded using base::RandUint64() in the constructor, meaning that it
// doesn't need to be seeded. It can be re-seeded though, with
// ReseedForTesting(). Its period is long enough that it should not need to be
// re-seeded during use.
//
// Uses the XorShift128+ generator under the hood.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) InsecureRandomGenerator {
 public:
  // Never use outside testing, not enough entropy.
  void ReseedForTesting(uint64_t seed);

  uint32_t RandUint32();
  uint64_t RandUint64();

  static InsecureRandomGenerator ConstructForTesting() {
    return InsecureRandomGenerator();
  }

 private:
  InsecureRandomGenerator();
  // State.
  uint64_t a_ = 0, b_ = 0;

  // Before adding a new friend class, make sure that the overhead of
  // base::Rand*() is too high, using something more representative than a
  // microbenchmark.
  //
  // PartitionAlloc allocations should not take more than 40-50ns per
  // malloc()/free() pair, otherwise high-level benchmarks regress, and does not
  // need a secure PRNG, as it's used for ASLR and zeroing some allocations at
  // free() time.
  friend class ::partition_alloc::RandomGenerator;
  friend class ::partition_alloc::internal::LightweightQuarantineBranch;
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_RAND_UTIL_H_
