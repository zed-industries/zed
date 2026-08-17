//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_SIZED_ALLOCATOR_H
#define TEST_SUPPORT_SIZED_ALLOCATOR_H

#include <cstddef>
#include <limits>
#include <memory>
#include <new>

#include "test_macros.h"

// Allocator with a provided size_type and difference_type, used to test corner cases
// like arithmetic on Allocator::size_type in generic code.
template <typename T, typename Size = std::size_t, typename Difference = std::ptrdiff_t>
class sized_allocator {
  template <typename U, typename Sz, typename Diff>
  friend class sized_allocator;

public:
  using value_type                  = T;
  using size_type                   = Size;
  using difference_type             = Difference;
  using propagate_on_container_swap = std::true_type;

  TEST_CONSTEXPR_CXX20 explicit sized_allocator(int d = 0) : data_(d) {}

  template <typename U, typename Sz, typename Diff>
  TEST_CONSTEXPR_CXX20 sized_allocator(const sized_allocator<U, Sz, Diff>& a) TEST_NOEXCEPT : data_(a.data_) {}

  TEST_CONSTEXPR_CXX20 T* allocate(size_type n) {
    if (n > max_size())
      TEST_THROW(std::bad_array_new_length());
    return std::allocator<T>().allocate(static_cast<std::size_t>(n));
  }

  TEST_CONSTEXPR_CXX20 void deallocate(T* p, size_type n) TEST_NOEXCEPT {
    std::allocator<T>().deallocate(p, static_cast<std::size_t>(n));
  }

  TEST_CONSTEXPR size_type max_size() const TEST_NOEXCEPT {
    return std::numeric_limits<size_type>::max() / sizeof(value_type);
  }

private:
  int data_;

  TEST_CONSTEXPR friend bool operator==(const sized_allocator& a, const sized_allocator& b) {
    return a.data_ == b.data_;
  }
  TEST_CONSTEXPR friend bool operator!=(const sized_allocator& a, const sized_allocator& b) {
    return a.data_ != b.data_;
  }
};

#endif // TEST_SUPPORT_SIZED_ALLOCATOR_H
