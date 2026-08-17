//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_LIBCXX_UTILITIES_SMARTPTR_ADAPT_TYPES_H
#define TEST_LIBCXX_UTILITIES_SMARTPTR_ADAPT_TYPES_H

#include <type_traits>
#include <memory>

#include "test_macros.h"

// Custom deleters.

template <typename T>
struct MoveOnlyDeleter {
  MoveOnlyDeleter()                                  = default;
  MoveOnlyDeleter(const MoveOnlyDeleter&)            = delete;
  MoveOnlyDeleter& operator=(const MoveOnlyDeleter&) = delete;
  MoveOnlyDeleter(MoveOnlyDeleter&&) : wasMoveInitilized{true} {}
  MoveOnlyDeleter& operator=(MoveOnlyDeleter&&) = default;

  void operator()(T* p) const { delete p; }

  bool wasMoveInitilized = false;
};

// Custom pointer types.

template <typename T>
struct ConstructiblePtr {
  using pointer = T*;
  std::unique_ptr<T> ptr;

  ConstructiblePtr() = default;
  explicit ConstructiblePtr(T* p) : ptr{p} {}

  auto operator==(T val) { return *ptr == val; }

  auto* get() const { return ptr.get(); }

  void release() { ptr.release(); }
};

LIBCPP_STATIC_ASSERT(std::is_same_v<std::__pointer_of_t< ConstructiblePtr<int>>, int* >);
static_assert(std::is_constructible_v< ConstructiblePtr<int>, int* >);

struct ResetArg {};

template <typename T>
struct ResettablePtr {
  using element_type = T;
  std::unique_ptr<T> ptr;

  explicit ResettablePtr(T* p) : ptr{p} {}

  auto operator*() const { return *ptr; }

  auto operator==(T val) { return *ptr == val; }

  void reset() { ptr.reset(); }
  void reset(T* p, ResetArg) { ptr.reset(p); }

  auto* get() const { return ptr.get(); }

  void release() { ptr.release(); }
};

LIBCPP_STATIC_ASSERT(std::is_same_v<std::__pointer_of_t< ResettablePtr<int>>, int* >);
static_assert(std::is_constructible_v< ResettablePtr<int>, int* >);

template <typename T>
struct NonConstructiblePtr : public ResettablePtr<T> {
  NonConstructiblePtr() : NonConstructiblePtr::ResettablePtr(nullptr) {};

  void reset(T* p) { ResettablePtr<T>::ptr.reset(p); }
};

LIBCPP_STATIC_ASSERT(std::is_same_v<std::__pointer_of_t< NonConstructiblePtr<int>>, int* >);
static_assert(!std::is_constructible_v< NonConstructiblePtr<int>, int* >);

#endif // TEST_LIBCXX_UTILITIES_SMARTPTR_ADAPT_TYPES_H
