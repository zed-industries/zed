//===-- Implementation of heap sort -----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDLIB_HEAP_SORT_H
#define LLVM_LIBC_SRC_STDLIB_HEAP_SORT_H

#include "src/__support/CPP/cstddef.h"
#include "src/stdlib/qsort_data.h"

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// A simple in-place heapsort implementation.
// Follow the implementation in https://en.wikipedia.org/wiki/Heapsort.

template <typename A, typename F>
LIBC_INLINE void heap_sort(const A &array, const F &is_less) {
  size_t end = array.len();
  size_t start = end / 2;

  const auto left_child = [](size_t i) -> size_t { return 2 * i + 1; };

  while (end > 1) {
    if (start > 0) {
      // Select the next unheapified element to sift down.
      --start;
    } else {
      // Extract the max element of the heap, moving a leaf to root to be sifted
      // down.
      --end;
      array.swap(0, end);
    }

    // Sift start down the heap.
    size_t root = start;
    while (left_child(root) < end) {
      size_t child = left_child(root);
      // If there are two children, set child to the greater.
      if ((child + 1 < end) && is_less(array.get(child), array.get(child + 1)))
        ++child;

      // If the root is less than the greater child
      if (!is_less(array.get(root), array.get(child)))
        break;

      // Swap the root with the greater child and continue sifting down.
      array.swap(root, child);
      root = child;
    }
  }
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_HEAP_SORT_H
