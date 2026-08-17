//===-- Implementation header for qsort utilities ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDLIB_QUICK_SORT_H
#define LLVM_LIBC_SRC_STDLIB_QUICK_SORT_H

#include "src/__support/CPP/bit.h"
#include "src/__support/CPP/cstddef.h"
#include "src/__support/macros/config.h"
#include "src/stdlib/qsort_pivot.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// Branchless Lomuto partition based on the implementation by Lukas
// Bergdoll and Orson Peters
// https://github.com/Voultapher/sort-research-rs/blob/main/writeup/lomcyc_partition/text.md.
// Simplified to avoid having to stack allocate.
template <typename A, typename F>
LIBC_INLINE size_t partition_lomuto_branchless(const A &array,
                                               const void *pivot,
                                               const F &is_less) {
  const size_t array_len = array.len();

  size_t left = 0;
  size_t right = 0;

  while (right < array_len) {
    const bool right_is_lt = is_less(array.get(right), pivot);
    array.swap(left, right);
    left += static_cast<size_t>(right_is_lt);
    right += 1;
  }

  return left;
}

// Optimized for large types that are expensive to move. Not optimized
// for integers. It's possible to use a cyclic permutation here for
// large types as done in ipnsort but the advantages of this are limited
// as `is_less` is a small wrapper around a call to a function pointer
// and won't incur much binary-size overhead. The other reason to use
// cyclic permutation is to have more efficient swapping, but we don't
// know the element size so this isn't applicable here either.
template <typename A, typename F>
LIBC_INLINE size_t partition_hoare_branchy(const A &array, const void *pivot,
                                           const F &is_less) {
  const size_t array_len = array.len();

  size_t left = 0;
  size_t right = array_len;

  while (true) {
    while (left < right && is_less(array.get(left), pivot))
      ++left;

    while (true) {
      --right;
      if (left >= right || is_less(array.get(right), pivot)) {
        break;
      }
    }

    if (left >= right)
      break;

    array.swap(left, right);
    ++left;
  }

  return left;
}

template <typename A, typename F>
LIBC_INLINE size_t partition(const A &array, size_t pivot_index,
                             const F &is_less) {
  // Place the pivot at the beginning of the array.
  if (pivot_index != 0) {
    array.swap(0, pivot_index);
  }

  const A array_without_pivot = array.make_array(1, array.len() - 1);
  const void *pivot = array.get(0);

  size_t num_lt;
  if constexpr (A::has_fixed_size()) {
    // Branchless Lomuto avoid branch misprediction penalties, but
    // it also swaps more often which is only faster if the swap is a fast
    // constant operation.
    num_lt = partition_lomuto_branchless(array_without_pivot, pivot, is_less);
  } else {
    num_lt = partition_hoare_branchy(array_without_pivot, pivot, is_less);
  }

  // Place the pivot between the two partitions.
  array.swap(0, num_lt);

  return num_lt;
}

template <typename A, typename F>
LIBC_INLINE void quick_sort_impl(A &array, const void *ancestor_pivot,
                                 size_t limit, const F &is_less) {
  while (true) {
    const size_t array_len = array.len();
    if (array_len <= 1)
      return;

    // If too many bad pivot choices were made, simply fall back to
    // heapsort in order to guarantee `O(N x log(N))` worst-case.
    if (limit == 0) {
      heap_sort(array, is_less);
      return;
    }

    limit -= 1;

    const size_t pivot_index = choose_pivot(array, is_less);

    // If the chosen pivot is equal to the predecessor, then it's the smallest
    // element in the slice. Partition the slice into elements equal to and
    // elements greater than the pivot. This case is usually hit when the slice
    // contains many duplicate elements.
    if (ancestor_pivot) {
      if (!is_less(ancestor_pivot, array.get(pivot_index))) {
        const size_t num_lt =
            partition(array, pivot_index,
                      [is_less](const void *a, const void *b) -> bool {
                        return !is_less(b, a);
                      });

        // Continue sorting elements greater than the pivot. We know that
        // `num_lt` cont
        array.reset_bounds(num_lt + 1, array.len() - (num_lt + 1));
        ancestor_pivot = nullptr;
        continue;
      }
    }

    size_t split_index = partition(array, pivot_index, is_less);

    if (array_len == 2)
      // The partition operation sorts the two element array.
      return;

    // Split the array into `left`, `pivot`, and `right`.
    A left = array.make_array(0, split_index);
    const void *pivot = array.get(split_index);
    const size_t right_start = split_index + 1;
    A right = array.make_array(right_start, array.len() - right_start);

    // Recurse into the left side. We have a fixed recursion limit,
    // testing shows no real benefit for recursing into the shorter
    // side.
    quick_sort_impl(left, ancestor_pivot, limit, is_less);

    // Continue with the right side.
    array = right;
    ancestor_pivot = pivot;
  }
}

constexpr size_t ilog2(size_t n) {
  return static_cast<size_t>(cpp::bit_width(n)) - 1;
}

template <typename A, typename F>
LIBC_INLINE void quick_sort(A &array, const F &is_less) {
  const void *ancestor_pivot = nullptr;
  // Limit the number of imbalanced partitions to `2 * floor(log2(len))`.
  // The binary OR by one is used to eliminate the zero-check in the logarithm.
  const size_t limit = 2 * ilog2((array.len() | 1));
  quick_sort_impl(array, ancestor_pivot, limit, is_less);
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_QUICK_SORT_H
