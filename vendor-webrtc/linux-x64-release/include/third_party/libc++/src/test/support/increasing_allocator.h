//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_INCREASING_ALLOCATOR_H
#define TEST_SUPPORT_INCREASING_ALLOCATOR_H

#include <cstddef>
#include <limits>
#include <memory>

#include "test_macros.h"

// The increasing_allocator is a custom allocator that enforces an increasing minimum allocation size,
// ensuring that it allocates an increasing amount of memory, possibly exceeding the requested amount.
// This unique behavior is particularly useful for testing the shrink_to_fit functionality in std::vector,
// vector<bool>, and std::basic_string, ensuring that shrink_to_fit does not increase the capacity of
// the allocated memory.

template <typename T>
struct increasing_allocator {
  using value_type         = T;
  std::size_t min_elements = 1000;
  increasing_allocator()   = default;

  template <typename U>
  TEST_CONSTEXPR_CXX20 increasing_allocator(const increasing_allocator<U>& other) TEST_NOEXCEPT
      : min_elements(other.min_elements) {}

#if TEST_STD_VER >= 23
  TEST_CONSTEXPR_CXX23 std::allocation_result<T*> allocate_at_least(std::size_t n) {
    if (n < min_elements)
      n = min_elements;
    min_elements += 1000;
    return std::allocator<T>{}.allocate_at_least(n);
  }
#endif // TEST_STD_VER >= 23

  TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) { return std::allocator<T>().allocate(n); }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t n) TEST_NOEXCEPT { std::allocator<T>().deallocate(p, n); }
};

template <typename T, typename U>
TEST_CONSTEXPR_CXX20 bool operator==(increasing_allocator<T>, increasing_allocator<U>) TEST_NOEXCEPT {
  return true;
}

template <std::size_t MinAllocSize, typename T>
class min_size_allocator {
public:
  using value_type     = T;
  min_size_allocator() = default;

  template <typename U>
  TEST_CONSTEXPR_CXX20 min_size_allocator(const min_size_allocator<MinAllocSize, U>&) TEST_NOEXCEPT {}

  TEST_NODISCARD TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) {
    if (n < MinAllocSize)
      n = MinAllocSize;
    return static_cast<T*>(::operator new(n * sizeof(T)));
  }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t) TEST_NOEXCEPT { ::operator delete(static_cast<void*>(p)); }

  template <typename U>
  struct rebind {
    using other = min_size_allocator<MinAllocSize, U>;
  };
};

template <std::size_t MinAllocSize, typename T, typename U>
TEST_CONSTEXPR_CXX20 bool
operator==(const min_size_allocator<MinAllocSize, T>&, const min_size_allocator<MinAllocSize, U>&) {
  return true;
}

template <std::size_t MinAllocSize, typename T, typename U>
TEST_CONSTEXPR_CXX20 bool
operator!=(const min_size_allocator<MinAllocSize, T>&, const min_size_allocator<MinAllocSize, U>&) {
  return false;
}

template <typename T>
class pow2_allocator {
public:
  using value_type = T;
  pow2_allocator() = default;

  template <typename U>
  TEST_CONSTEXPR_CXX20 pow2_allocator(const pow2_allocator<U>&) TEST_NOEXCEPT {}

  TEST_NODISCARD TEST_CONSTEXPR_CXX20 T* allocate(std::size_t n) {
    return static_cast<T*>(::operator new(next_power_of_two(n) * sizeof(T)));
  }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, std::size_t) TEST_NOEXCEPT { ::operator delete(static_cast<void*>(p)); }

private:
  TEST_CONSTEXPR_CXX20 std::size_t next_power_of_two(std::size_t n) const {
    if ((n & (n - 1)) == 0)
      return n;
    for (std::size_t shift = 1; shift < std::numeric_limits<std::size_t>::digits; shift <<= 1) {
      n |= n >> shift;
    }
    return n + 1;
  }
};

template <typename T, typename U>
TEST_CONSTEXPR_CXX20 bool operator==(const pow2_allocator<T>&, const pow2_allocator<U>&) {
  return true;
}

template <typename T, typename U>
TEST_CONSTEXPR_CXX20 bool operator!=(const pow2_allocator<T>&, const pow2_allocator<U>&) {
  return false;
}

#endif // TEST_SUPPORT_INCREASING_ALLOCATOR_H
