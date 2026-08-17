//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCXX_TEST_LIBCXX_CONTAINERS_SEQUENCES_VECTOR_FILL_TO_CAPACITY_H
#define LIBCXX_TEST_LIBCXX_CONTAINERS_SEQUENCES_VECTOR_FILL_TO_CAPACITY_H

#include <vector>

template <typename T, typename A>
void fill_to_capacity(std::vector<T, A>& vec) {
  // Fill the given vector up to its capacity. Our bounded iterators are currently unable to catch an out-of-bounds
  // access that goes beyond the container's logical storage (above the size) but is still within its physical storage
  // (below the capacity) due to iterator stability guarantees. Filling a vector makes this distinction go away.
  while (vec.size() < vec.capacity()) {
    vec.push_back(T());
  }
}

#endif // LIBCXX_TEST_LIBCXX_CONTAINERS_SEQUENCES_VECTOR_FILL_TO_CAPACITY_H
