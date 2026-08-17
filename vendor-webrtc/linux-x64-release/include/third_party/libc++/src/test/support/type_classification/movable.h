//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_TYPE_CLASSIFICATION_MOVABLE_H
#define TEST_SUPPORT_TYPE_CLASSIFICATION_MOVABLE_H

struct has_const_member {
  int const x;
};

struct has_volatile_member {
  int volatile x;
};

struct has_cv_member {
  int const volatile x;
};

struct has_lvalue_reference_member {
  int& x;
};

struct has_rvalue_reference_member {
  int&& x;
};

struct has_array_member {
  int x[5];
};

struct has_function_ref_member {
  int (&f)();
};

struct cpp03_friendly {
  cpp03_friendly(cpp03_friendly const&);
  cpp03_friendly& operator=(cpp03_friendly const&);
};

struct const_move_ctor {
  const_move_ctor(const_move_ctor const&&);
  const_move_ctor& operator=(const_move_ctor&&);
};

struct volatile_move_ctor {
  volatile_move_ctor(volatile_move_ctor volatile&&);
  volatile_move_ctor& operator=(volatile_move_ctor&&);
};

struct cv_move_ctor {
  cv_move_ctor(cv_move_ctor const volatile&&);
  cv_move_ctor& operator=(cv_move_ctor&&);
};

struct multi_param_move_ctor {
  multi_param_move_ctor(multi_param_move_ctor&&, int = 0);
  multi_param_move_ctor& operator=(multi_param_move_ctor&&);
};

struct not_quite_multi_param_move_ctor {
  not_quite_multi_param_move_ctor(not_quite_multi_param_move_ctor&&, int);
  not_quite_multi_param_move_ctor& operator=(not_quite_multi_param_move_ctor&&);
};

struct copy_with_mutable_parameter {
  copy_with_mutable_parameter(copy_with_mutable_parameter&);
  copy_with_mutable_parameter&
  operator=(copy_with_mutable_parameter&);
};

struct const_move_assignment {
  const_move_assignment& operator=(const_move_assignment&&) const;
};

struct volatile_move_assignment {
  const_move_assignment& operator=(const_move_assignment&&) volatile;
};

struct cv_move_assignment {
  cv_move_assignment& operator=(cv_move_assignment&&) const volatile;
};

struct const_move_assign_and_traditional_move_assign {
  const_move_assign_and_traditional_move_assign&
  operator=(const_move_assign_and_traditional_move_assign&&);
  const_move_assign_and_traditional_move_assign&
  operator=(const_move_assign_and_traditional_move_assign&&) const;
};

struct volatile_move_assign_and_traditional_move_assign {
  volatile_move_assign_and_traditional_move_assign&
  operator=(volatile_move_assign_and_traditional_move_assign&&);
  volatile_move_assign_and_traditional_move_assign&
  operator=(volatile_move_assign_and_traditional_move_assign&&) volatile;
};

struct cv_move_assign_and_traditional_move_assign {
  cv_move_assign_and_traditional_move_assign&
  operator=(cv_move_assign_and_traditional_move_assign&&);
  cv_move_assign_and_traditional_move_assign&
  operator=(cv_move_assign_and_traditional_move_assign&&) const volatile;
};

struct const_move_assign_and_default_ops {
  const_move_assign_and_default_ops(const_move_assign_and_default_ops const&) =
      default;
  const_move_assign_and_default_ops(const_move_assign_and_default_ops&&) =
      default;
  const_move_assign_and_default_ops&
  operator=(const_move_assign_and_default_ops const&) = default;
  const_move_assign_and_default_ops&
  operator=(const_move_assign_and_default_ops&&) = default;
  const_move_assign_and_default_ops&
  operator=(const_move_assign_and_default_ops&&) const;
};

struct volatile_move_assign_and_default_ops {
  volatile_move_assign_and_default_ops(
      volatile_move_assign_and_default_ops const&) = default;
  volatile_move_assign_and_default_ops(volatile_move_assign_and_default_ops&&) =
      default;
  volatile_move_assign_and_default_ops&
  operator=(volatile_move_assign_and_default_ops const&) = default;
  volatile_move_assign_and_default_ops&
  operator=(volatile_move_assign_and_default_ops&&) = default;
  volatile_move_assign_and_default_ops&
  operator=(volatile_move_assign_and_default_ops&&) volatile;
};

struct cv_move_assign_and_default_ops {
  cv_move_assign_and_default_ops(cv_move_assign_and_default_ops const&) =
      default;
  cv_move_assign_and_default_ops(cv_move_assign_and_default_ops&&) = default;
  cv_move_assign_and_default_ops&
  operator=(cv_move_assign_and_default_ops const&) = default;
  cv_move_assign_and_default_ops&
  operator=(cv_move_assign_and_default_ops&&) = default;
  cv_move_assign_and_default_ops&
  operator=(cv_move_assign_and_default_ops&&) const volatile;
};

struct deleted_assignment_from_const_rvalue {
  deleted_assignment_from_const_rvalue(
      deleted_assignment_from_const_rvalue const&);
  deleted_assignment_from_const_rvalue(deleted_assignment_from_const_rvalue&&);
  deleted_assignment_from_const_rvalue&
  operator=(const deleted_assignment_from_const_rvalue&);
  deleted_assignment_from_const_rvalue&
  operator=(deleted_assignment_from_const_rvalue&&);
  deleted_assignment_from_const_rvalue&
  operator=(const deleted_assignment_from_const_rvalue&&) = delete;
};

#endif // TEST_SUPPORT_TYPE_CLASSIFICATION_MOVABLE_H
