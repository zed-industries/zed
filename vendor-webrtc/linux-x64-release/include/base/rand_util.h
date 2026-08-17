// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_RAND_UTIL_H_
#define BASE_RAND_UTIL_H_

#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <cmath>
#include <concepts>
#include <string>
#include <type_traits>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "base/gtest_prod_util.h"
#include "base/numerics/clamped_math.h"
#include "base/numerics/safe_conversions.h"
#include "base/time/time.h"
#include "build/build_config.h"

#if !BUILDFLAG(IS_NACL)
#include "third_party/boringssl/src/include/openssl/rand.h"
#endif

namespace memory_simulator {
class MemoryHolder;
}

namespace base {

namespace internal {

#if !BUILDFLAG(IS_NACL)
void ConfigureBoringSSLBackedRandBytesFieldTrial();
#endif

// Returns a random double in range [0, 1). For use in allocator shim to avoid
// infinite recursion. Thread-safe.
BASE_EXPORT double RandDoubleAvoidAllocation();

}  // namespace internal

namespace test {
class InsecureRandomGenerator;
}  // namespace test

// Returns a random number in range [0, UINT64_MAX]. Thread-safe.
BASE_EXPORT uint64_t RandUint64();

// Returns a random number between min and max (inclusive). Thread-safe.
//
// TODO(crbug.com/40283703): Change from fully-closed to half-closed (i.e.
// exclude `max`) to parallel other APIs here.
BASE_EXPORT int RandInt(int min, int max);

// Returns a random number in range [0, range).  Thread-safe.
BASE_EXPORT uint64_t RandGenerator(uint64_t range);

// Returns a random double in range [0, 1). Thread-safe.
BASE_EXPORT double RandDouble();

// Returns a random float in range [0, 1). Thread-safe.
BASE_EXPORT float RandFloat();

// Returns a random bool. Thread-safe.
BASE_EXPORT bool RandBool();

// Returns a random duration in [`start`, `limit`). Thread-safe.
//
// REQUIRES: `start` < `limit`
BASE_EXPORT TimeDelta RandTimeDelta(TimeDelta start, TimeDelta limit);

// Returns a random duration in [`TimeDelta()`, `limit`). Thread-safe.
//
// REQUIRES: `limit.is_positive()`
BASE_EXPORT TimeDelta RandTimeDeltaUpTo(TimeDelta limit);

// Adjusts `value` up or down by a random amount up to `percentage`%, e.g. to
// add noise/jitter. Thread-safe.
//
// More precisely, implements something equivalent to the following pseudocode:
// (1) Computes `max_adjustment = value * percentage / 100` as a double
// (2) If `T` is integral, rounds `max_adjustment`, clamped to what is
//     effectively a 65-bit signed value
// (3) Computes `result` as a random value in the range of
//     [`value - max_adjustment`, `value + max_adjustment`)
// (4) Checks that the `result` is in the valid range of `T` and returns it
//
// REQUIRES: inputs are finite, `percentage` >= 0
template <typename T>
  requires std::floating_point<T>
T RandomizeByPercentage(T value, double percentage) {
  CHECK(!std::isinf(value));
  CHECK(!std::isnan(value));
  CHECK(!std::isinf(percentage));
  CHECK_GE(percentage, 0);
  return checked_cast<T>(value +
                         value * (RandDouble() - 0.5) * 2 * percentage / 100);
}
template <typename T>
  requires std::integral<T>
T RandomizeByPercentage(T value, double percentage) {
  CHECK(!std::isinf(percentage));
  CHECK_GE(percentage, 0);
  // If `T` is signed and `percentage` is sufficiently large, the maximum
  // adjustment may not fit in a `T`. The clamped value described in pseudocode
  // step (2) above will always fit in a `uint64_t`, so do math in `uint64_t`s.
  const uint64_t abs_value = SafeUnsignedAbs(value);
  const uint64_t max_abs_adjustment =
      ClampRound<uint64_t>(abs_value * percentage / 100);
  if (!max_abs_adjustment) {
    return value;
  }
  uint64_t abs_adjustment = RandGenerator(max_abs_adjustment);

  CheckedNumeric<T> checked_value(value);
  // Random sign bit for the adjustment.
  if (RandBool()) {
    // Subtract adjustment.
    //
    // Be careful to "translate" the adjustment to the other side of `value` (by
    // doing the subtraction from `max_abs_adjustment` here) instead of
    // "mirroring" it (as would happen if this were omitted). This avoids bias
    // and preserves the desired half-closed interval property of the result
    // range.
    abs_adjustment = max_abs_adjustment - abs_adjustment;
    checked_value -= abs_adjustment;
  } else {
    checked_value += abs_adjustment;
  }
  return checked_value.ValueOrDie();
}
inline TimeDelta RandomizeByPercentage(TimeDelta value, double percentage) {
  CHECK(!value.is_inf());
  return Microseconds(
      RandomizeByPercentage(value.InMicroseconds(), percentage));
}

// Given input |bits|, convert with maximum precision to a double in
// the range [0, 1). Thread-safe.
BASE_EXPORT double BitsToOpenEndedUnitInterval(uint64_t bits);

// Given input `bits`, convert with maximum precision to a float in the range
// [0, 1). Thread-safe.
BASE_EXPORT float BitsToOpenEndedUnitIntervalF(uint64_t bits);

// Fills `output` with cryptographically secure random data. Thread-safe.
//
// Although implementations are required to use a cryptographically secure
// random number source, code outside of base/ that relies on this should use
// crypto::RandBytes instead to ensure the requirement is easily discoverable.
BASE_EXPORT void RandBytes(span<uint8_t> output);

// Creates a vector of `length` bytes, fills it with random data, and returns
// it. Thread-safe.
//
// Although implementations are required to use a cryptographically secure
// random number source, code outside of base/ that relies on this should use
// crypto::RandBytes instead to ensure the requirement is easily discoverable.
BASE_EXPORT std::vector<uint8_t> RandBytesAsVector(size_t length);

// DEPRECATED. Prefer RandBytesAsVector() above.
// Fills a string of length |length| with random data and returns it.
// Thread-safe.
//
// Note that this is a variation of |RandBytes| with a different return type.
// The returned string is likely not ASCII/UTF-8. Use with care.
//
// Although implementations are required to use a cryptographically secure
// random number source, code outside of base/ that relies on this should use
// crypto::RandBytes instead to ensure the requirement is easily discoverable.
BASE_EXPORT std::string RandBytesAsString(size_t length);

// An STL UniformRandomBitGenerator backed by RandUint64.
class RandomBitGenerator {
 public:
  using result_type = uint64_t;
  static constexpr result_type min() { return 0; }
  static constexpr result_type max() { return UINT64_MAX; }
  result_type operator()() const { return RandUint64(); }

  RandomBitGenerator() = default;
  ~RandomBitGenerator() = default;
};

#if !BUILDFLAG(IS_NACL)
class NonAllocatingRandomBitGenerator {
 public:
  using result_type = uint64_t;
  static constexpr result_type min() { return 0; }
  static constexpr result_type max() { return UINT64_MAX; }
  result_type operator()() const {
    uint64_t result;
    RAND_get_system_entropy_for_custom_prng(reinterpret_cast<uint8_t*>(&result),
                                            sizeof(result));
    return result;
  }

  NonAllocatingRandomBitGenerator() = default;
  ~NonAllocatingRandomBitGenerator() = default;
};
#endif

// Shuffles [first, last) randomly. Thread-safe.
template <typename Itr>
void RandomShuffle(Itr first, Itr last) {
  std::shuffle(first, last, RandomBitGenerator());
}

#if BUILDFLAG(IS_POSIX)
BASE_EXPORT int GetUrandomFD();
#endif

class MetricsSubSampler;

// Fast, insecure pseudo-random number generator.
//
// WARNING: This is not the generator you are looking for. This has significant
// caveats:
//   - It is non-cryptographic, so easy to misuse
//   - It is neither fork() nor clone()-safe because both RNG's after the
//     fork/clone will have the same state and produce the same number stream.
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
class BASE_EXPORT InsecureRandomGenerator {
 public:
  // Never use outside testing, not enough entropy.
  void ReseedForTesting(uint64_t seed);

  uint32_t RandUint32() const;
  uint64_t RandUint64() const;
  // In [0, 1).
  double RandDouble() const;

 private:
  InsecureRandomGenerator();
  // State. These are mutable to allow Rand* functions to be declared as const.
  // This, in turn, enables use of `MetricsSubSampler` in const contexts.
  mutable uint64_t a_ = 0, b_ = 0;

  // Before adding a new friend class, make sure that the overhead of
  // base::Rand*() is too high, using something more representative than a
  // microbenchmark.

  // Uses the generator to fill memory pages with random content to make them
  // hard to compress, in a simulation tool not bundled with Chrome. CPU
  // overhead must be minimized to correctly measure memory effects.
  friend class memory_simulator::MemoryHolder;
  // Uses the generator to sub-sample metrics.
  friend class MetricsSubSampler;
  // test::InsecureRandomGenerator can be used for testing.
  friend class test::InsecureRandomGenerator;

  FRIEND_TEST_ALL_PREFIXES(RandUtilTest,
                           InsecureRandomGeneratorProducesBothValuesOfAllBits);
  FRIEND_TEST_ALL_PREFIXES(RandUtilTest, InsecureRandomGeneratorChiSquared);
  FRIEND_TEST_ALL_PREFIXES(RandUtilTest, InsecureRandomGeneratorRandDouble);
  FRIEND_TEST_ALL_PREFIXES(RandUtilPerfTest, InsecureRandomRandUint64);
};

// Fast class to randomly sub-sample metrics that are logged in high frequency
// code.
//
// WARNING: This uses InsecureRandomGenerator so all the caveats there apply.
// In particular if a MetricsSubSampler object exists when fork()/clone() is
// called, calls to ShouldSample() on both sides of the fork will return the
// same values, possibly introducing metric bias.
class BASE_EXPORT MetricsSubSampler {
 public:
  MetricsSubSampler();
  bool ShouldSample(double probability) const;

  void Reseed();

  // Make any call to ShouldSample for any instance of MetricsSubSampler
  // return true for testing. Cannot be used in conjunction with
  // ScopedNeverSampleForTesting.
  class BASE_EXPORT ScopedAlwaysSampleForTesting {
   public:
    ScopedAlwaysSampleForTesting();
    ~ScopedAlwaysSampleForTesting();
  };

  // Make any call to ShouldSample for any instance of MetricsSubSampler
  // return false for testing. Cannot be used in conjunction with
  // ScopedAlwaysSampleForTesting.
  class BASE_EXPORT ScopedNeverSampleForTesting {
   public:
    ScopedNeverSampleForTesting();
    ~ScopedNeverSampleForTesting();
  };

 private:
  InsecureRandomGenerator generator_;
};

// Returns true with `probability` using a pseudo-random number generator (or
// always/never returns true if a `ScopedAlwaysSampleForTesting` or
// `ScopedNeverSampleForTesting` is in scope). Valid values for `probability`
// are in range [0, 1].
//
// This function is intended for sub-sampled metric recording only. Do not use
// it for any other purpose, especially where cryptographic randomness is
// required.
//
// Uses a thread local MetricsSubSampler.
BASE_EXPORT bool ShouldRecordSubsampledMetric(double probability);

// Reseeds the MetricsSubsampler used by ShouldRecordSubsampledMetric. Used
// after forking a zygote to avoid having multiple processes sharing initial
// RNG state.
BASE_EXPORT void ReseedSharedMetricsSubsampler();

}  // namespace base

#endif  // BASE_RAND_UTIL_H_
