//===-- Implementation header for qsort utilities ---------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDLIB_QSORT_PIVOT_H
#define LLVM_LIBC_SRC_STDLIB_QSORT_PIVOT_H

#include "src/__support/macros/attributes.h"

#include <stddef.h> // For size_t

namespace LIBC_NAMESPACE_DECL {
namespace internal {

// Recursively select a pseudomedian if above this threshold.
constexpr size_t PSEUDO_MEDIAN_REC_THRESHOLD = 64;

// Selects a pivot from `array`. Algorithm taken from glidesort by Orson Peters.
//
// This chooses a pivot by sampling an adaptive amount of points, approximating
// the quality of a median of sqrt(n) elements.
template <typename A, typename F>
LIBC_INLINE size_t choose_pivot(const A &array, const F &is_less) {
  const size_t len = array.len();

  if (len < 8) {
    return 0;
  }

  const size_t len_div_8 = len / 8;

  const size_t a = 0;             // [0, floor(n/8))
  const size_t b = len_div_8 * 4; // [4*floor(n/8), 5*floor(n/8))
  const size_t c = len_div_8 * 7; // [7*floor(n/8), 8*floor(n/8))

  if (len < PSEUDO_MEDIAN_REC_THRESHOLD)
    return median3(array, a, b, c, is_less);
  else
    return median3_rec(array, a, b, c, len_div_8, is_less);
}

// Calculates an approximate median of 3 elements from sections a, b, c, or
// recursively from an approximation of each, if they're large enough. By
// dividing the size of each section by 8 when recursing we have logarithmic
// recursion depth and overall sample from f(n) = 3*f(n/8) -> f(n) =
// O(n^(log(3)/log(8))) ~= O(n^0.528) elements.
template <typename A, typename F>
LIBC_INLINE size_t median3_rec(const A &array, size_t a, size_t b, size_t c,
                               size_t n, const F &is_less) {
  if (n * 8 >= PSEUDO_MEDIAN_REC_THRESHOLD) {
    const size_t n8 = n / 8;
    a = median3_rec(array, a, a + (n8 * 4), a + (n8 * 7), n8, is_less);
    b = median3_rec(array, b, b + (n8 * 4), b + (n8 * 7), n8, is_less);
    c = median3_rec(array, c, c + (n8 * 4), c + (n8 * 7), n8, is_less);
  }
  return median3(array, a, b, c, is_less);
}

/// Calculates the median of 3 elements.
template <typename A, typename F>
LIBC_INLINE size_t median3(const A &array, size_t a, size_t b, size_t c,
                           const F &is_less) {
  const void *a_ptr = array.get(a);
  const void *b_ptr = array.get(b);
  const void *c_ptr = array.get(c);

  const bool x = is_less(a_ptr, b_ptr);
  const bool y = is_less(a_ptr, c_ptr);
  if (x == y) {
    // If x=y=0 then b, c <= a. In this case we want to return max(b, c).
    // If x=y=1 then a < b, c. In this case we want to return min(b, c).
    // By toggling the outcome of b < c using XOR x we get this behavior.
    const bool z = is_less(b_ptr, c_ptr);
    return z ^ x ? c : b;
  } else {
    // Either c <= a < b or b <= a < c, thus a is our median.
    return a;
  }
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_QSORT_PIVOT_H
