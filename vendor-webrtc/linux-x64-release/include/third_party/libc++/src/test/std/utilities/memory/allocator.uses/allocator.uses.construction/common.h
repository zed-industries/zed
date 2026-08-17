//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "test_allocator.h"

using Alloc = test_allocator<int>;

enum class RefType {
  LValue,
  ConstLValue,
  RValue,
  ConstRValue,
};

struct UsesAllocArgT {
  using allocator_type = Alloc;

  bool allocator_constructed_ = false;
  Alloc a_;
  const Alloc& alloc_ = a_;
  const int* val_ptr_;
  RefType ref_type_;

  constexpr UsesAllocArgT() = default;
  constexpr UsesAllocArgT(std::allocator_arg_t, const Alloc& alloc) : allocator_constructed_(true), alloc_(alloc) {}
  constexpr UsesAllocArgT(std::allocator_arg_t, const Alloc& alloc, int& val)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::LValue) {}
  constexpr UsesAllocArgT(std::allocator_arg_t, const Alloc& alloc, const int& val)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::ConstLValue) {}
  constexpr UsesAllocArgT(std::allocator_arg_t, const Alloc& alloc, int&& val)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::RValue) {}
  constexpr UsesAllocArgT(std::allocator_arg_t, const Alloc& alloc, const int&& val)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::ConstRValue) {}
};

struct UsesAllocLast {
  using allocator_type = Alloc;

  bool allocator_constructed_ = false;
  Alloc a_;
  const Alloc& alloc_ = a_;
  const int* val_ptr_;
  RefType ref_type_;

  constexpr UsesAllocLast() = default;
  constexpr UsesAllocLast(const Alloc& alloc) : allocator_constructed_(true), alloc_(alloc) {}
  constexpr UsesAllocLast(int& val, const Alloc& alloc)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::LValue) {}
  constexpr UsesAllocLast(const int& val, const Alloc& alloc)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::ConstLValue) {}
  constexpr UsesAllocLast(int&& val, const Alloc& alloc)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::RValue) {}
  constexpr UsesAllocLast(const int&& val, const Alloc& alloc)
      : allocator_constructed_(true), alloc_(alloc), val_ptr_(&val), ref_type_(RefType::ConstRValue) {}
};

struct NotAllocatorAware {
  bool allocator_constructed_ = false;

  constexpr NotAllocatorAware() = default;
  constexpr NotAllocatorAware(const Alloc&) : allocator_constructed_(true) {}
  constexpr NotAllocatorAware(const Alloc&, int) : allocator_constructed_(true) {}
};

struct ConvertibleToPair {
  constexpr operator std::pair<int, int>() const { return {1, 2}; }
};
