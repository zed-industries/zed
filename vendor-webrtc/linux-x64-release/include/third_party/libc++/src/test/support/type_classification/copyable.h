//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_CLASSIFICATION_COPYABLE_H
#define TEST_SUPPORT_TYPE_CLASSIFICATION_COPYABLE_H

#include <memory>

#include "movable.h"

struct no_copy_constructor {
  no_copy_constructor() = default;

  no_copy_constructor(no_copy_constructor const&) = delete;
  no_copy_constructor(no_copy_constructor&&) = default;
};

struct no_copy_assignment {
  no_copy_assignment() = default;

  no_copy_assignment& operator=(no_copy_assignment const&) = delete;
  no_copy_assignment& operator=(no_copy_assignment&&) = default;
};

struct no_copy_assignment_mutable {
  no_copy_assignment_mutable() = default;

  no_copy_assignment_mutable&
  operator=(no_copy_assignment_mutable const&) = default;
  no_copy_assignment_mutable& operator=(no_copy_assignment_mutable&) = delete;
  no_copy_assignment_mutable& operator=(no_copy_assignment_mutable&&) = default;
};

struct derived_from_noncopyable : std::unique_ptr<int> {};

struct has_noncopyable {
  std::unique_ptr<int> x;
};

struct const_copy_assignment {
  const_copy_assignment() = default;

  const_copy_assignment(const_copy_assignment const&);
  const_copy_assignment(const_copy_assignment&&);

  const_copy_assignment& operator=(const_copy_assignment&&);
  const_copy_assignment const& operator=(const_copy_assignment const&) const;
};

struct volatile_copy_assignment {
  volatile_copy_assignment() = default;

  volatile_copy_assignment(volatile_copy_assignment volatile&);
  volatile_copy_assignment(volatile_copy_assignment volatile&&);

  volatile_copy_assignment& operator=(volatile_copy_assignment&&);
  volatile_copy_assignment volatile&
  operator=(volatile_copy_assignment const&) volatile;
};

struct cv_copy_assignment {
  cv_copy_assignment() = default;

  cv_copy_assignment(cv_copy_assignment const volatile&);
  cv_copy_assignment(cv_copy_assignment const volatile&&);

  cv_copy_assignment const volatile&
  operator=(cv_copy_assignment const volatile&) const volatile;
  cv_copy_assignment const volatile&
  operator=(cv_copy_assignment const volatile&&) const volatile;
};

#endif // TEST_SUPPORT_TYPE_CLASSIFICATION_COPYABLE_H
