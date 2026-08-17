//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LIBCPP_TEST_STD_ITERATORS_ITERATOR_REQUIREMENTS_ITERATOR_CONCEPTS_INCREMENTABLE_H
#define LIBCPP_TEST_STD_ITERATORS_ITERATOR_REQUIREMENTS_ITERATOR_CONCEPTS_INCREMENTABLE_H

struct postfix_increment_returns_void {
  using difference_type = int;
  postfix_increment_returns_void& operator++();
  void operator++(int);
};

struct postfix_increment_returns_copy {
  using difference_type = int;
  postfix_increment_returns_copy& operator++();
  postfix_increment_returns_copy operator++(int);
};

struct has_integral_minus {
  has_integral_minus& operator++();
  has_integral_minus operator++(int);

  long operator-(has_integral_minus) const;
};

struct has_distinct_difference_type_and_minus {
  using difference_type = short;

  has_distinct_difference_type_and_minus& operator++();
  has_distinct_difference_type_and_minus operator++(int);

  long operator-(has_distinct_difference_type_and_minus) const;
};

struct missing_difference_type {
  missing_difference_type& operator++();
  void operator++(int);
};

struct floating_difference_type {
  using difference_type = float;

  floating_difference_type& operator++();
  void operator++(int);
};

struct non_const_minus {
  non_const_minus& operator++();
  non_const_minus operator++(int);

  long operator-(non_const_minus);
};

struct non_integral_minus {
  non_integral_minus& operator++();
  non_integral_minus operator++(int);

  void operator-(non_integral_minus);
};

struct bad_difference_type_good_minus {
  using difference_type = float;

  bad_difference_type_good_minus& operator++();
  void operator++(int);

  int operator-(bad_difference_type_good_minus) const;
};

struct not_default_initializable {
  using difference_type = int;
  not_default_initializable() = delete;

  not_default_initializable& operator++();
  void operator++(int);
};

struct not_movable {
  using difference_type = int;

  not_movable() = default;
  not_movable(not_movable&&) = delete;

  not_movable& operator++();
  void operator++(int);
};

struct preinc_not_declared {
  using difference_type = int;

  void operator++(int);
};

struct postinc_not_declared {
  using difference_type = int;

  postinc_not_declared& operator++();
};

struct incrementable_with_difference_type {
  using difference_type = int;

  incrementable_with_difference_type& operator++();
  incrementable_with_difference_type operator++(int);

  bool operator==(incrementable_with_difference_type const&) const;
};

struct incrementable_without_difference_type {
  incrementable_without_difference_type& operator++();
  incrementable_without_difference_type operator++(int);

  bool operator==(incrementable_without_difference_type const&) const;

  int operator-(incrementable_without_difference_type) const;
};

struct difference_type_and_void_minus {
  using difference_type = int;

  difference_type_and_void_minus& operator++();
  difference_type_and_void_minus operator++(int);

  bool operator==(difference_type_and_void_minus const&) const;

  void operator-(difference_type_and_void_minus) const;
};

struct noncopyable_with_difference_type {
  using difference_type = int;

  noncopyable_with_difference_type() = default;
  noncopyable_with_difference_type(noncopyable_with_difference_type&&) = default;
  noncopyable_with_difference_type(noncopyable_with_difference_type const&) = delete;

  noncopyable_with_difference_type& operator=(noncopyable_with_difference_type&&) = default;
  noncopyable_with_difference_type& operator=(noncopyable_with_difference_type const&) = delete;

  noncopyable_with_difference_type& operator++();
  noncopyable_with_difference_type operator++(int);

  bool operator==(noncopyable_with_difference_type const&) const;
};

struct noncopyable_without_difference_type {
  noncopyable_without_difference_type() = default;
  noncopyable_without_difference_type(noncopyable_without_difference_type&&) = default;
  noncopyable_without_difference_type(noncopyable_without_difference_type const&) = delete;

  noncopyable_without_difference_type& operator=(noncopyable_without_difference_type&&) = default;
  noncopyable_without_difference_type& operator=(noncopyable_without_difference_type const&) = delete;

  noncopyable_without_difference_type& operator++();
  noncopyable_without_difference_type operator++(int);

  int operator-(noncopyable_without_difference_type const&) const;

  bool operator==(noncopyable_without_difference_type const&) const;
};

struct noncopyable_with_difference_type_and_minus {
  using difference_type = int;

  noncopyable_with_difference_type_and_minus() = default;
  noncopyable_with_difference_type_and_minus(noncopyable_with_difference_type_and_minus&&) = default;
  noncopyable_with_difference_type_and_minus(noncopyable_with_difference_type_and_minus const&) = delete;

  noncopyable_with_difference_type_and_minus& operator=(noncopyable_with_difference_type_and_minus&&) = default;
  noncopyable_with_difference_type_and_minus& operator=(noncopyable_with_difference_type_and_minus const&) = delete;

  noncopyable_with_difference_type_and_minus& operator++();
  noncopyable_with_difference_type_and_minus operator++(int);

  int operator-(noncopyable_with_difference_type_and_minus const&) const;

  bool operator==(noncopyable_with_difference_type_and_minus const&) const;
};

#endif // #define LIBCPP_TEST_STD_ITERATORS_ITERATOR_REQUIREMENTS_ITERATOR_CONCEPTS_INCREMENTABLE_H
