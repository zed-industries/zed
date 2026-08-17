//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_FUZZING_FUZZ_H
#define TEST_LIBCXX_FUZZING_FUZZ_H

#include <cassert>
#include <cstddef>
#include <cstdint>
#include <cstring> // std::strlen
#include <iterator>
#include <type_traits>
#include <utility> // std::swap


// This is a struct we can use to test the stable_XXX algorithms.
// Perform the operation on the key, then check the order of the payload.
struct ByteWithPayload {
  std::uint8_t key;
  std::size_t payload;

  ByteWithPayload(std::uint8_t k) : key(k), payload(0) { }
  ByteWithPayload(std::uint8_t k, std::size_t p) : key(k), payload(p) { }

  friend bool operator==(ByteWithPayload const& x, ByteWithPayload const& y) {
    return x.key == y.key && x.payload == y.payload;
  }

  friend bool operator!=(ByteWithPayload const& x, ByteWithPayload const& y) {
    return !(x == y);
  }

  struct key_less {
    bool operator()(ByteWithPayload const& x, ByteWithPayload const& y) const
    { return x.key < y.key; }
  };

  struct payload_less {
    bool operator()(ByteWithPayload const& x, ByteWithPayload const& y) const
    { return x.payload < y.payload; }
  };

  struct total_less {
    bool operator()(ByteWithPayload const& x, ByteWithPayload const& y) const {
      return x.key == y.key ? x.payload < y.payload : x.key < y.key;
    }
  };

  friend void swap(ByteWithPayload& lhs, ByteWithPayload& rhs) {
      std::swap(lhs.key, rhs.key);
      std::swap(lhs.payload, rhs.payload);
  }
};

// Faster version of std::is_permutation
//
// Builds a set of buckets for each of the key values, and sums all the payloads.
// Not 100% perfect, but _way_ faster.
template <typename Iter1, typename Iter2, typename = typename std::enable_if<
  std::is_same<typename std::iterator_traits<Iter1>::value_type, ByteWithPayload>::value &&
  std::is_same<typename std::iterator_traits<Iter2>::value_type, ByteWithPayload>::value
>::type>
bool fast_is_permutation(Iter1 first1, Iter1 last1, Iter2 first2) {
  std::size_t xBuckets[256]  = {0};
  std::size_t xPayloads[256] = {0};
  std::size_t yBuckets[256]  = {0};
  std::size_t yPayloads[256] = {0};

  for (; first1 != last1; ++first1, ++first2) {
    xBuckets[first1->key]++;
    xPayloads[first1->key] += first1->payload;

    yBuckets[first2->key]++;
    yPayloads[first2->key] += first2->payload;
  }

  for (std::size_t i = 0; i < 256; ++i) {
    if (xBuckets[i] != yBuckets[i])
      return false;
    if (xPayloads[i] != yPayloads[i])
      return false;
  }

  return true;
}

template <typename Iter1, typename Iter2, typename = void, typename = typename std::enable_if<
  std::is_same<typename std::iterator_traits<Iter1>::value_type, std::uint8_t>::value &&
  std::is_same<typename std::iterator_traits<Iter2>::value_type, std::uint8_t>::value
>::type>
bool fast_is_permutation(Iter1 first1, Iter1 last1, Iter2 first2) {
  std::size_t xBuckets[256] = {0};
  std::size_t yBuckets[256] = {0};

  for (; first1 != last1; ++first1, ++first2) {
    xBuckets[*first1]++;
    yBuckets[*first2]++;
  }

  for (std::size_t i = 0; i < 256; ++i)
    if (xBuckets[i] != yBuckets[i])
      return false;

  return true;
}

// When running inside OSS-Fuzz, we link against a fuzzing library that defines
// main() and calls LLVMFuzzerTestOneInput.
//
// Otherwise, when e.g. running the Lit tests, we define main() to run fuzzing
// tests on a few inputs.
#if !defined(LIBCPP_OSS_FUZZ)
extern "C" int LLVMFuzzerTestOneInput(const std::uint8_t*, std::size_t);

int main(int, char**) {
  const char* test_cases[] = {
    "",
    "s",
    "bac",
    "bacasf",
    "lkajseravea",
    "adsfkajdsfjkas;lnc441324513,34535r34525234",
    "b*c",
    "ba?sf",
    "lka*ea",
    "adsf*kas;lnc441[0-9]1r34525234"
  };

  for (const char* tc : test_cases) {
    const std::size_t size = std::strlen(tc);
    const std::uint8_t* data = reinterpret_cast<const std::uint8_t*>(tc);
    int result = LLVMFuzzerTestOneInput(data, size);
    assert(result == 0);
  }

  return 0;
}
#endif // !LIBCPP_OSS_FUZZ

#endif // TEST_LIBCXX_FUZZING_FUZZ_H
