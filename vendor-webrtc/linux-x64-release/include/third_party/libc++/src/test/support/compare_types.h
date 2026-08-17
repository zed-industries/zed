//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_COMPARE_TYPES_H
#define TEST_SUPPORT_COMPARE_TYPES_H

#include <compare>
#include <concepts>
#include <type_traits>

// `noexcept` specifiers deliberately imperfect since not all programmers bother to put the
// specifiers on their overloads.

struct equality_comparable_with_ec1;
struct no_neq;

struct cxx20_member_eq {
  bool operator==(cxx20_member_eq const&) const = default;
};

struct cxx20_friend_eq {
  friend bool operator==(cxx20_friend_eq const&, cxx20_friend_eq const&) = default;
};

struct member_three_way_comparable {
  auto operator<=>(member_three_way_comparable const&) const = default;
};

struct friend_three_way_comparable {
  friend auto operator<=>(friend_three_way_comparable const&, friend_three_way_comparable const&) = default;
};

struct explicit_operators {
  friend bool operator==(explicit_operators, explicit_operators) noexcept;
  friend bool operator!=(explicit_operators, explicit_operators) noexcept;
  friend bool operator<(explicit_operators, explicit_operators) noexcept;
  friend bool operator>(explicit_operators, explicit_operators) noexcept;
  friend bool operator<=(explicit_operators, explicit_operators) noexcept;
  friend bool operator>=(explicit_operators, explicit_operators) noexcept;

  friend bool operator==(explicit_operators const&, equality_comparable_with_ec1 const&) noexcept;
  friend bool operator==(equality_comparable_with_ec1 const&, explicit_operators const&) noexcept;
  friend bool operator!=(explicit_operators const&, equality_comparable_with_ec1 const&) noexcept;
  friend bool operator!=(equality_comparable_with_ec1 const&, explicit_operators const&) noexcept;
};

struct different_return_types {
  bool operator==(different_return_types) const noexcept;
  char operator!=(different_return_types) const noexcept;
  short operator<(different_return_types) const noexcept;
  int operator>(different_return_types) const noexcept;
  long operator<=(different_return_types) const noexcept;
  long long operator>=(different_return_types) const noexcept;

  friend signed char operator==(explicit_operators, different_return_types);
  friend unsigned char operator==(different_return_types, explicit_operators);
  friend float operator!=(explicit_operators, different_return_types);
  friend double operator!=(different_return_types, explicit_operators);

  operator explicit_operators() const;
};

struct boolean {
  operator bool() const noexcept;
};

struct one_member_one_friend {
  friend boolean operator==(one_member_one_friend, one_member_one_friend) noexcept;
  boolean operator!=(one_member_one_friend) const noexcept;

  operator explicit_operators() const noexcept;
  operator different_return_types() const noexcept;
};

struct equality_comparable_with_ec1 {
  bool operator==(equality_comparable_with_ec1) const noexcept;
  bool operator!=(equality_comparable_with_ec1) const noexcept;
  operator explicit_operators() const noexcept;
};

struct no_eq {
  friend bool operator==(no_eq, no_eq) = delete;
  friend bool operator!=(no_eq, no_eq) noexcept;
  friend bool operator<(no_eq, no_eq) noexcept;
  friend bool operator>(no_eq, no_eq) noexcept;
  friend bool operator>=(no_eq, no_eq) noexcept;
  friend bool operator<=(no_eq, no_eq) noexcept;
};

struct no_neq {
  friend bool operator==(no_neq, no_neq) noexcept;
  friend bool operator!=(no_neq, no_neq) = delete;
  friend bool operator<(no_eq, no_eq) noexcept;
  friend bool operator>(no_eq, no_eq) noexcept;
  friend bool operator>=(no_eq, no_eq) noexcept;
  friend bool operator<=(no_eq, no_eq) noexcept;
};

struct no_lt {
  friend bool operator==(no_lt, no_lt) noexcept;
  friend bool operator!=(no_lt, no_lt) noexcept;
  friend bool operator<(no_lt, no_lt) = delete;
  friend bool operator>(no_lt, no_lt) noexcept;
  friend bool operator>=(no_lt, no_lt) noexcept;
  friend bool operator<=(no_lt, no_lt) noexcept;
};

struct no_gt {
  friend bool operator==(no_gt, no_gt) noexcept;
  friend bool operator!=(no_gt, no_gt) noexcept;
  friend bool operator<(no_gt, no_gt) noexcept;
  friend bool operator>(no_gt, no_gt) = delete;
  friend bool operator>=(no_gt, no_gt) noexcept;
  friend bool operator<=(no_gt, no_gt) noexcept;
};

struct no_le {
  friend bool operator==(no_le, no_le) noexcept;
  friend bool operator!=(no_le, no_le) noexcept;
  friend bool operator<(no_le, no_le) noexcept;
  friend bool operator>(no_le, no_le) noexcept;
  friend bool operator>=(no_le, no_le) = delete;
  friend bool operator<=(no_le, no_le) noexcept;
};

struct no_ge {
  friend bool operator==(no_ge, no_ge) noexcept;
  friend bool operator!=(no_ge, no_ge) noexcept;
  friend bool operator<(no_ge, no_ge) noexcept;
  friend bool operator>(no_ge, no_ge) noexcept;
  friend bool operator>=(no_ge, no_ge) noexcept;
  friend bool operator<=(no_ge, no_ge) = delete;
};

struct wrong_return_type_eq {
  void operator==(wrong_return_type_eq) const noexcept;
  bool operator!=(wrong_return_type_eq) const noexcept;
  bool operator<(wrong_return_type_eq) const noexcept;
  bool operator>(wrong_return_type_eq) const noexcept;
  bool operator>=(wrong_return_type_eq) const noexcept;
  bool operator<=(wrong_return_type_eq) const noexcept;
};

struct wrong_return_type_ne {
  bool operator==(wrong_return_type_ne) const noexcept;
  void operator!=(wrong_return_type_ne) const noexcept;
  bool operator<(wrong_return_type_ne) const noexcept;
  bool operator>(wrong_return_type_ne) const noexcept;
  bool operator>=(wrong_return_type_ne) const noexcept;
  bool operator<=(wrong_return_type_ne) const noexcept;
};

struct wrong_return_type_lt {
  bool operator==(wrong_return_type_lt) const noexcept;
  bool operator!=(wrong_return_type_lt) const noexcept;
  void operator<(wrong_return_type_lt) const noexcept;
  bool operator>(wrong_return_type_lt) const noexcept;
  bool operator>=(wrong_return_type_lt) const noexcept;
  bool operator<=(wrong_return_type_lt) const noexcept;
};

struct wrong_return_type_gt {
  bool operator==(wrong_return_type_gt) const noexcept;
  bool operator!=(wrong_return_type_gt) const noexcept;
  bool operator<(wrong_return_type_gt) const noexcept;
  void operator>(wrong_return_type_gt) const noexcept;
  bool operator>=(wrong_return_type_gt) const noexcept;
  bool operator<=(wrong_return_type_gt) const noexcept;
};

struct wrong_return_type_le {
  bool operator==(wrong_return_type_le) const noexcept;
  bool operator!=(wrong_return_type_le) const noexcept;
  bool operator<(wrong_return_type_le) const noexcept;
  bool operator>(wrong_return_type_le) const noexcept;
  void operator>=(wrong_return_type_le) const noexcept;
  bool operator<=(wrong_return_type_le) const noexcept;
};

struct wrong_return_type_ge {
  bool operator==(wrong_return_type_ge) const noexcept;
  bool operator!=(wrong_return_type_ge) const noexcept;
  bool operator<(wrong_return_type_ge) const noexcept;
  bool operator>(wrong_return_type_ge) const noexcept;
  bool operator>=(wrong_return_type_ge) const noexcept;
  void operator<=(wrong_return_type_ge) const noexcept;
};

struct wrong_return_type {
  void operator==(wrong_return_type) const noexcept;
  void operator!=(wrong_return_type) const noexcept;
  void operator<(wrong_return_type) const noexcept;
  void operator>(wrong_return_type) const noexcept;
  void operator>=(wrong_return_type) const noexcept;
  void operator<=(wrong_return_type_ge) const noexcept;
};

struct cxx20_member_eq_operator_with_deleted_ne {
  bool operator==(cxx20_member_eq_operator_with_deleted_ne const&) const = default;
  bool operator!=(cxx20_member_eq_operator_with_deleted_ne const&) const = delete;
};

struct cxx20_friend_eq_operator_with_deleted_ne {
  friend bool operator==(cxx20_friend_eq_operator_with_deleted_ne const&,
                         cxx20_friend_eq_operator_with_deleted_ne const&) = default;
  friend bool operator!=(cxx20_friend_eq_operator_with_deleted_ne const&,
                         cxx20_friend_eq_operator_with_deleted_ne const&) = delete;
};

struct member_three_way_comparable_with_deleted_eq {
  auto operator<=>(member_three_way_comparable_with_deleted_eq const&) const = default;
  bool operator==(member_three_way_comparable_with_deleted_eq const&) const = delete;
};

struct member_three_way_comparable_with_deleted_ne {
  auto operator<=>(member_three_way_comparable_with_deleted_ne const&) const = default;
  bool operator!=(member_three_way_comparable_with_deleted_ne const&) const = delete;
};

struct friend_three_way_comparable_with_deleted_eq {
  friend auto operator<=>(friend_three_way_comparable_with_deleted_eq const&,
                          friend_three_way_comparable_with_deleted_eq const&) = default;
  friend bool operator==(friend_three_way_comparable_with_deleted_eq const&,
                         friend_three_way_comparable_with_deleted_eq const&) = delete;
};

struct friend_three_way_comparable_with_deleted_ne {
  friend auto operator<=>(friend_three_way_comparable_with_deleted_ne const&,
                          friend_three_way_comparable_with_deleted_ne const&) = default;
  friend bool operator!=(friend_three_way_comparable_with_deleted_ne const&,
                         friend_three_way_comparable_with_deleted_ne const&) = delete;
};

struct one_way_eq {
  bool operator==(one_way_eq const&) const = default;
  friend bool operator==(one_way_eq, explicit_operators);
  friend bool operator==(explicit_operators, one_way_eq) = delete;

  operator explicit_operators() const;
};

struct one_way_ne {
  bool operator==(one_way_ne const&) const = default;
  friend bool operator==(one_way_ne, explicit_operators);
  friend bool operator!=(one_way_ne, explicit_operators) = delete;

  operator explicit_operators() const;
};
static_assert(requires(explicit_operators const x, one_way_ne const y) { x != y; });

struct explicit_bool {
  explicit operator bool() const noexcept;
};

struct totally_ordered_with_others {
  auto operator<=>(totally_ordered_with_others const&) const = default;
};

struct no_lt_not_totally_ordered_with {
  bool operator==(no_lt_not_totally_ordered_with const&) const = default;
  auto operator<=>(no_lt_not_totally_ordered_with const&) const = default;
  operator totally_ordered_with_others() const noexcept;

  bool operator==(totally_ordered_with_others const&) const;
  auto operator<=>(totally_ordered_with_others const&) const;
  auto operator<(totally_ordered_with_others const&) const;
};

struct no_gt_not_totally_ordered_with {
  bool operator==(no_gt_not_totally_ordered_with const&) const = default;
  auto operator<=>(no_gt_not_totally_ordered_with const&) const = default;
  operator totally_ordered_with_others() const noexcept;

  bool operator==(totally_ordered_with_others const&) const;
  auto operator<=>(totally_ordered_with_others const&) const;
  auto operator>(totally_ordered_with_others const&) const;
};

struct no_le_not_totally_ordered_with {
  bool operator==(no_le_not_totally_ordered_with const&) const = default;
  auto operator<=>(no_le_not_totally_ordered_with const&) const = default;
  operator totally_ordered_with_others() const noexcept;

  bool operator==(totally_ordered_with_others const&) const;
  auto operator<=>(totally_ordered_with_others const&) const;
  auto operator<=(totally_ordered_with_others const&) const;
};

struct no_ge_not_totally_ordered_with {
  bool operator==(no_ge_not_totally_ordered_with const&) const = default;
  auto operator<=>(no_ge_not_totally_ordered_with const&) const = default;
  operator totally_ordered_with_others() const noexcept;

  bool operator==(totally_ordered_with_others const&) const;
  auto operator<=>(totally_ordered_with_others const&) const;
  auto operator>=(totally_ordered_with_others const&) const;
};

struct partial_ordering_totally_ordered_with {
  auto operator<=>(partial_ordering_totally_ordered_with const&) const noexcept = default;
  std::partial_ordering operator<=>(totally_ordered_with_others const&) const noexcept;

  operator totally_ordered_with_others() const;
};

struct weak_ordering_totally_ordered_with {
  auto operator<=>(weak_ordering_totally_ordered_with const&) const noexcept = default;
  std::weak_ordering operator<=>(totally_ordered_with_others const&) const noexcept;

  operator totally_ordered_with_others() const;
};

struct strong_ordering_totally_ordered_with {
  auto operator<=>(strong_ordering_totally_ordered_with const&) const noexcept = default;
  std::strong_ordering operator<=>(totally_ordered_with_others const&) const noexcept;

  operator totally_ordered_with_others() const;
};

struct eq_returns_explicit_bool {
  friend explicit_bool operator==(eq_returns_explicit_bool, eq_returns_explicit_bool);
  friend bool operator!=(eq_returns_explicit_bool, eq_returns_explicit_bool);
  friend bool operator<(eq_returns_explicit_bool, eq_returns_explicit_bool);
  friend bool operator>(eq_returns_explicit_bool, eq_returns_explicit_bool);
  friend bool operator<=(eq_returns_explicit_bool, eq_returns_explicit_bool);
  friend bool operator>=(eq_returns_explicit_bool, eq_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend explicit_bool operator==(eq_returns_explicit_bool, totally_ordered_with_others);
  friend explicit_bool operator==(totally_ordered_with_others, eq_returns_explicit_bool);
  friend bool operator!=(eq_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(totally_ordered_with_others, eq_returns_explicit_bool);
  friend bool operator<(eq_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, eq_returns_explicit_bool);
  friend bool operator>(eq_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, eq_returns_explicit_bool);
  friend bool operator<=(eq_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<=(totally_ordered_with_others, eq_returns_explicit_bool);
  friend bool operator>=(eq_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>=(totally_ordered_with_others, eq_returns_explicit_bool);
};

struct ne_returns_explicit_bool {
  friend bool operator==(ne_returns_explicit_bool, ne_returns_explicit_bool);
  friend explicit_bool operator!=(ne_returns_explicit_bool, ne_returns_explicit_bool);
  friend bool operator<(ne_returns_explicit_bool, ne_returns_explicit_bool);
  friend bool operator>(ne_returns_explicit_bool, ne_returns_explicit_bool);
  friend bool operator<=(ne_returns_explicit_bool, ne_returns_explicit_bool);
  friend bool operator>=(ne_returns_explicit_bool, ne_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend bool operator==(ne_returns_explicit_bool, totally_ordered_with_others);
  friend explicit_bool operator!=(ne_returns_explicit_bool, totally_ordered_with_others);
  friend explicit_bool operator!=(totally_ordered_with_others, ne_returns_explicit_bool);
  friend bool operator<(ne_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, ne_returns_explicit_bool);
  friend bool operator>(ne_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, ne_returns_explicit_bool);
  friend bool operator<=(ne_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<=(totally_ordered_with_others, ne_returns_explicit_bool);
  friend bool operator>=(ne_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>=(totally_ordered_with_others, ne_returns_explicit_bool);
};

struct lt_returns_explicit_bool {
  friend bool operator==(lt_returns_explicit_bool, lt_returns_explicit_bool);
  friend bool operator!=(lt_returns_explicit_bool, lt_returns_explicit_bool);
  friend explicit_bool operator<(lt_returns_explicit_bool, lt_returns_explicit_bool);
  friend bool operator>(lt_returns_explicit_bool, lt_returns_explicit_bool);
  friend bool operator<=(lt_returns_explicit_bool, lt_returns_explicit_bool);
  friend bool operator>=(lt_returns_explicit_bool, lt_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend bool operator==(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(totally_ordered_with_others, lt_returns_explicit_bool);
  friend explicit_bool operator<(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, lt_returns_explicit_bool);
  friend bool operator>(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, lt_returns_explicit_bool);
  friend bool operator<=(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<=(totally_ordered_with_others, lt_returns_explicit_bool);
  friend bool operator>=(lt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>=(totally_ordered_with_others, lt_returns_explicit_bool);
};

struct gt_returns_explicit_bool {
  friend bool operator==(gt_returns_explicit_bool, gt_returns_explicit_bool);
  friend bool operator!=(gt_returns_explicit_bool, gt_returns_explicit_bool);
  friend bool operator<(gt_returns_explicit_bool, gt_returns_explicit_bool);
  friend explicit_bool operator>(gt_returns_explicit_bool, gt_returns_explicit_bool);
  friend bool operator<=(gt_returns_explicit_bool, gt_returns_explicit_bool);
  friend bool operator>=(gt_returns_explicit_bool, gt_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend bool operator==(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(totally_ordered_with_others, gt_returns_explicit_bool);
  friend bool operator<(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, gt_returns_explicit_bool);
  friend explicit_bool operator>(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, gt_returns_explicit_bool);
  friend bool operator<=(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<=(totally_ordered_with_others, gt_returns_explicit_bool);
  friend bool operator>=(gt_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>=(totally_ordered_with_others, gt_returns_explicit_bool);
};

struct le_returns_explicit_bool {
  friend bool operator==(le_returns_explicit_bool, le_returns_explicit_bool);
  friend bool operator!=(le_returns_explicit_bool, le_returns_explicit_bool);
  friend bool operator<(le_returns_explicit_bool, le_returns_explicit_bool);
  friend bool operator>(le_returns_explicit_bool, le_returns_explicit_bool);
  friend explicit_bool operator<=(le_returns_explicit_bool, le_returns_explicit_bool);
  friend bool operator>=(le_returns_explicit_bool, le_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend bool operator==(le_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(le_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(totally_ordered_with_others, le_returns_explicit_bool);
  friend bool operator<(le_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, le_returns_explicit_bool);
  friend bool operator>(le_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, le_returns_explicit_bool);
  friend bool operator<=(le_returns_explicit_bool, totally_ordered_with_others);
  friend explicit_bool operator<=(totally_ordered_with_others, le_returns_explicit_bool);
  friend bool operator>=(le_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>=(totally_ordered_with_others, le_returns_explicit_bool);
};

struct ge_returns_explicit_bool {
  friend bool operator==(ge_returns_explicit_bool, ge_returns_explicit_bool);
  friend bool operator!=(ge_returns_explicit_bool, ge_returns_explicit_bool);
  friend bool operator<(ge_returns_explicit_bool, ge_returns_explicit_bool);
  friend bool operator>(ge_returns_explicit_bool, ge_returns_explicit_bool);
  friend bool operator<=(ge_returns_explicit_bool, ge_returns_explicit_bool);
  friend explicit_bool operator>=(ge_returns_explicit_bool, ge_returns_explicit_bool);

  operator totally_ordered_with_others() const;

  friend bool operator==(ge_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(ge_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator!=(totally_ordered_with_others, ge_returns_explicit_bool);
  friend bool operator<(ge_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<(totally_ordered_with_others, ge_returns_explicit_bool);
  friend bool operator>(ge_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator>(totally_ordered_with_others, ge_returns_explicit_bool);
  friend bool operator<=(ge_returns_explicit_bool, totally_ordered_with_others);
  friend bool operator<=(totally_ordered_with_others, ge_returns_explicit_bool);
  friend bool operator>=(ge_returns_explicit_bool, totally_ordered_with_others);
  friend explicit_bool operator>=(totally_ordered_with_others, ge_returns_explicit_bool);
};

struct returns_true_type {
  friend std::true_type operator==(returns_true_type, returns_true_type);
  friend std::true_type operator!=(returns_true_type, returns_true_type);
  friend std::true_type operator<(returns_true_type, returns_true_type);
  friend std::true_type operator>(returns_true_type, returns_true_type);
  friend std::true_type operator<=(returns_true_type, returns_true_type);
  friend std::true_type operator>=(returns_true_type, returns_true_type);

  operator totally_ordered_with_others() const;

  friend std::true_type operator==(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator==(totally_ordered_with_others, returns_true_type);
  friend std::true_type operator!=(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator!=(totally_ordered_with_others, returns_true_type);
  friend std::true_type operator<(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator<(totally_ordered_with_others, returns_true_type);
  friend std::true_type operator>(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator>(totally_ordered_with_others, returns_true_type);
  friend std::true_type operator<=(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator<=(totally_ordered_with_others, returns_true_type);
  friend std::true_type operator>=(returns_true_type, totally_ordered_with_others);
  friend std::true_type operator>=(totally_ordered_with_others, returns_true_type);
};

struct returns_int_ptr {
  friend int* operator==(returns_int_ptr, returns_int_ptr);
  friend int* operator!=(returns_int_ptr, returns_int_ptr);
  friend int* operator<(returns_int_ptr, returns_int_ptr);
  friend int* operator>(returns_int_ptr, returns_int_ptr);
  friend int* operator<=(returns_int_ptr, returns_int_ptr);
  friend int* operator>=(returns_int_ptr, returns_int_ptr);

  operator totally_ordered_with_others() const;

  friend int* operator==(returns_int_ptr, totally_ordered_with_others);
  friend int* operator==(totally_ordered_with_others, returns_int_ptr);
  friend int* operator!=(returns_int_ptr, totally_ordered_with_others);
  friend int* operator!=(totally_ordered_with_others, returns_int_ptr);
  friend int* operator<(returns_int_ptr, totally_ordered_with_others);
  friend int* operator<(totally_ordered_with_others, returns_int_ptr);
  friend int* operator>(returns_int_ptr, totally_ordered_with_others);
  friend int* operator>(totally_ordered_with_others, returns_int_ptr);
  friend int* operator<=(returns_int_ptr, totally_ordered_with_others);
  friend int* operator<=(totally_ordered_with_others, returns_int_ptr);
  friend int* operator>=(returns_int_ptr, totally_ordered_with_others);
  friend int* operator>=(totally_ordered_with_others, returns_int_ptr);
};

struct ForwardingTestObject {
  constexpr bool operator<(ForwardingTestObject&&) && { return true; }
  constexpr bool operator<(const ForwardingTestObject&) const& { return false; }

  constexpr bool operator==(ForwardingTestObject&&) && { return true; }
  constexpr bool operator==(const ForwardingTestObject&) const& { return false; }

  constexpr bool operator!=(ForwardingTestObject&&) && { return true; }
  constexpr bool operator!=(const ForwardingTestObject&) const& { return false; }

  constexpr bool operator<=(ForwardingTestObject&&) && { return true; }
  constexpr bool operator<=(const ForwardingTestObject&) const& { return false; }

  constexpr bool operator>(ForwardingTestObject&&) && { return true; }
  constexpr bool operator>(const ForwardingTestObject&) const& { return false; }

  constexpr bool operator>=(ForwardingTestObject&&) && { return true; }
  constexpr bool operator>=(const ForwardingTestObject&) const& { return false; }
};

#endif // TEST_SUPPORT_COMPARE_TYPES_H
