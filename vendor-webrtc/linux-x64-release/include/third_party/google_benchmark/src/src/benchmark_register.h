#ifndef BENCHMARK_REGISTER_H
#define BENCHMARK_REGISTER_H

#include <algorithm>
#include <limits>
#include <vector>

#include "check.h"

namespace benchmark {
namespace internal {

// Append the powers of 'mult' in the closed interval [lo, hi].
// Returns iterator to the start of the inserted range.
template <typename T>
typename std::vector<T>::iterator AddPowers(std::vector<T>* dst, T lo, T hi,
                                            int mult) {
  BM_CHECK_GE(lo, 0);
  BM_CHECK_GE(hi, lo);
  BM_CHECK_GE(mult, 2);

  const size_t start_offset = dst->size();

  static const T kmax = std::numeric_limits<T>::max();

  // Space out the values in multiples of "mult"
  for (T i = static_cast<T>(1); i <= hi; i = static_cast<T>(i * mult)) {
    if (i >= lo) {
      dst->push_back(i);
    }
    // Break the loop here since multiplying by
    // 'mult' would move outside of the range of T
    if (i > kmax / mult) break;
  }

  return dst->begin() + static_cast<int>(start_offset);
}

template <typename T>
void AddNegatedPowers(std::vector<T>* dst, T lo, T hi, int mult) {
  // We negate lo and hi so we require that they cannot be equal to 'min'.
  BM_CHECK_GT(lo, std::numeric_limits<T>::min());
  BM_CHECK_GT(hi, std::numeric_limits<T>::min());
  BM_CHECK_GE(hi, lo);
  BM_CHECK_LE(hi, 0);

  // Add positive powers, then negate and reverse.
  // Casts necessary since small integers get promoted
  // to 'int' when negating.
  const auto lo_complement = static_cast<T>(-lo);
  const auto hi_complement = static_cast<T>(-hi);

  const auto it = AddPowers(dst, hi_complement, lo_complement, mult);

  std::for_each(it, dst->end(), [](T& t) { t = static_cast<T>(t * -1); });
  std::reverse(it, dst->end());
}

template <typename T>
void AddRange(std::vector<T>* dst, T lo, T hi, int mult) {
  static_assert(std::is_integral<T>::value && std::is_signed<T>::value,
                "Args type must be a signed integer");

  BM_CHECK_GE(hi, lo);
  BM_CHECK_GE(mult, 2);

  // Add "lo"
  dst->push_back(lo);

  // Handle lo == hi as a special case, so we then know
  // lo < hi and so it is safe to add 1 to lo and subtract 1
  // from hi without falling outside of the range of T.
  if (lo == hi) return;

  // Ensure that lo_inner <= hi_inner below.
  if (lo + 1 == hi) {
    dst->push_back(hi);
    return;
  }

  // Add all powers of 'mult' in the range [lo+1, hi-1] (inclusive).
  const auto lo_inner = static_cast<T>(lo + 1);
  const auto hi_inner = static_cast<T>(hi - 1);

  // Insert negative values
  if (lo_inner < 0) {
    AddNegatedPowers(dst, lo_inner, std::min(hi_inner, T{-1}), mult);
  }

  // Treat 0 as a special case (see discussion on #762).
  if (lo < 0 && hi >= 0) {
    dst->push_back(0);
  }

  // Insert positive values
  if (hi_inner > 0) {
    AddPowers(dst, std::max(lo_inner, T{1}), hi_inner, mult);
  }

  // Add "hi" (if different from last value).
  if (hi != dst->back()) {
    dst->push_back(hi);
  }
}

}  // namespace internal
}  // namespace benchmark

#endif  // BENCHMARK_REGISTER_H
