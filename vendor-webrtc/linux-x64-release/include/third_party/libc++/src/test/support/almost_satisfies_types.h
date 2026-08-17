//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef ALMOST_SATISFIES_TYPES_H
#define ALMOST_SATISFIES_TYPES_H

#include <functional>
#include <iterator>
#include <ranges>

#include "test_iterators.h"

template <class T, class U = sentinel_wrapper<T>>
class UncheckedRange {
public:
  T begin();
  U end();
};

static_assert(std::ranges::contiguous_range<UncheckedRange<int*, int*>>);

// almost an input_iterator
template <class T>
class InputIteratorNotDerivedFromGeneric {
public:
  using difference_type = long;
  using value_type = T;
  using iterator_category = void;

  InputIteratorNotDerivedFromGeneric& operator++();
  void operator++(int);
  const T& operator*() const;
};

using InputIteratorNotDerivedFrom = InputIteratorNotDerivedFromGeneric<int>;

template <class T>
using InputRangeNotDerivedFromGeneric = UncheckedRange<InputIteratorNotDerivedFromGeneric<T>>;
using InputRangeNotDerivedFrom = UncheckedRange<InputIteratorNotDerivedFrom>;

static_assert(std::input_or_output_iterator<InputIteratorNotDerivedFrom>);
static_assert(std::indirectly_readable<InputIteratorNotDerivedFrom>);
static_assert(!std::input_iterator<InputIteratorNotDerivedFrom>);
static_assert(!std::ranges::input_range<InputRangeNotDerivedFrom>);

class InputIteratorNotIndirectlyReadable {
public:
  using difference_type = long;
  using iterator_category = std::input_iterator_tag;

  InputIteratorNotIndirectlyReadable& operator++();
  void operator++(int);
  const int& operator*() const;
};

using InputRangeNotIndirectlyReadable = UncheckedRange<InputIteratorNotIndirectlyReadable>;

static_assert(std::input_or_output_iterator<InputIteratorNotIndirectlyReadable>);
static_assert(!std::indirectly_readable<InputIteratorNotIndirectlyReadable>);
static_assert(!std::input_iterator<InputIteratorNotIndirectlyReadable>);
static_assert(!std::ranges::input_range<InputRangeNotIndirectlyReadable>);

class InputIteratorNotInputOrOutputIterator {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::input_iterator_tag;

  int& operator++();
  void operator++(int);
  const int& operator*() const;
};

using InputRangeNotInputOrOutputIterator = UncheckedRange<InputIteratorNotInputOrOutputIterator>;

static_assert(!std::input_or_output_iterator<InputIteratorNotInputOrOutputIterator>);
static_assert(std::indirectly_readable<InputIteratorNotInputOrOutputIterator>);
static_assert(!std::input_iterator<InputIteratorNotInputOrOutputIterator>);
static_assert(!std::ranges::input_range<InputRangeNotInputOrOutputIterator>);

// almost an indirect_unary_predicate
class IndirectUnaryPredicateNotCopyConstructible {
public:
  IndirectUnaryPredicateNotCopyConstructible(const IndirectUnaryPredicateNotCopyConstructible&) = delete;
  bool operator()(int) const;
};

static_assert(std::predicate<IndirectUnaryPredicateNotCopyConstructible, int&>);
static_assert(!std::indirect_unary_predicate<IndirectUnaryPredicateNotCopyConstructible, int*>);

class IndirectUnaryPredicateNotPredicate {
public:
  bool operator()(int&&) const;
};

static_assert(!std::predicate<IndirectUnaryPredicateNotPredicate, int&>);
static_assert(!std::indirect_unary_predicate<IndirectUnaryPredicateNotPredicate, int*>);

// almost a sentinel_for cpp20_input_iterator
class SentinelForNotSemiregular {
public:
  SentinelForNotSemiregular() = delete;
  using difference_type = long;
  SentinelForNotSemiregular& operator++();
  void operator++(int);
  const int& operator*() const;
  friend bool operator==(const SentinelForNotSemiregular&, const cpp20_input_iterator<int*>&);
};

using InputRangeNotSentinelSemiregular = UncheckedRange<cpp20_input_iterator<int*>, SentinelForNotSemiregular>;
using OutputRangeNotSentinelSemiregular = UncheckedRange<cpp20_output_iterator<int*>, SentinelForNotSemiregular>;

static_assert(std::input_or_output_iterator<SentinelForNotSemiregular>);
static_assert(!std::semiregular<SentinelForNotSemiregular>);
static_assert(!std::sentinel_for<SentinelForNotSemiregular, cpp20_input_iterator<int*>>);

// almost a sentinel_for cpp20_input_iterator
class SentinelForNotWeaklyEqualityComparableWith {
public:
  using difference_type = long;
  SentinelForNotWeaklyEqualityComparableWith& operator++();
  void operator++(int);
  const int& operator*() const;
};

using InputRangeNotSentinelEqualityComparableWith =
  UncheckedRange<cpp20_input_iterator<int*>, SentinelForNotWeaklyEqualityComparableWith>;
using OutputRangeNotSentinelEqualityComparableWith =
  UncheckedRange<cpp20_output_iterator<int*>, SentinelForNotWeaklyEqualityComparableWith>;

static_assert(std::input_or_output_iterator<SentinelForNotWeaklyEqualityComparableWith>);
static_assert(std::semiregular<SentinelForNotWeaklyEqualityComparableWith>);
static_assert(!std::sentinel_for<SentinelForNotWeaklyEqualityComparableWith, cpp20_input_iterator<int*>>);

class WeaklyIncrementableNotMovable {
public:
  using difference_type = long;
  WeaklyIncrementableNotMovable& operator++();
  void operator++(int);
  WeaklyIncrementableNotMovable(const WeaklyIncrementableNotMovable&) = delete;
};

static_assert(!std::movable<WeaklyIncrementableNotMovable>);
static_assert(!std::weakly_incrementable<WeaklyIncrementableNotMovable>);

// almost a forward_iterator
class ForwardIteratorNotDerivedFrom {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::input_iterator_tag;

  ForwardIteratorNotDerivedFrom& operator++();
  ForwardIteratorNotDerivedFrom operator++(int);
  const int& operator*() const;
  bool operator==(const ForwardIteratorNotDerivedFrom&) const = default;
};

using ForwardRangeNotDerivedFrom = UncheckedRange<ForwardIteratorNotDerivedFrom>;

static_assert(std::input_iterator<ForwardIteratorNotDerivedFrom>);
static_assert(std::incrementable<ForwardIteratorNotDerivedFrom>);
static_assert(std::sentinel_for<ForwardIteratorNotDerivedFrom, ForwardIteratorNotDerivedFrom>);
static_assert(!std::forward_iterator<ForwardIteratorNotDerivedFrom>);

class ForwardIteratorNotIncrementable {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::forward_iterator_tag;

  ForwardIteratorNotIncrementable& operator++();
  int operator++(int);
  const int& operator*() const;
  bool operator==(const ForwardIteratorNotIncrementable&) const = default;
};

using ForwardRangeNotIncrementable = UncheckedRange<ForwardIteratorNotIncrementable>;

static_assert(std::input_iterator<ForwardIteratorNotIncrementable>);
static_assert(!std::incrementable<ForwardIteratorNotIncrementable>);
static_assert(std::sentinel_for<ForwardIteratorNotIncrementable, ForwardIteratorNotIncrementable>);
static_assert(!std::forward_iterator<ForwardIteratorNotIncrementable>);

using ForwardRangeNotSentinelSemiregular = UncheckedRange<forward_iterator<int*>, SentinelForNotSemiregular>;
using ForwardRangeNotSentinelEqualityComparableWith =
    UncheckedRange<forward_iterator<int*>, SentinelForNotWeaklyEqualityComparableWith>;

class BidirectionalIteratorNotDerivedFrom {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::forward_iterator_tag;

  BidirectionalIteratorNotDerivedFrom& operator++();
  BidirectionalIteratorNotDerivedFrom operator++(int);
  BidirectionalIteratorNotDerivedFrom& operator--();
  BidirectionalIteratorNotDerivedFrom operator--(int);
  int& operator*() const;

  bool operator==(const BidirectionalIteratorNotDerivedFrom&) const = default;
};

using BidirectionalRangeNotDerivedFrom = UncheckedRange<BidirectionalIteratorNotDerivedFrom>;
using BidirectionalRangeNotSentinelSemiregular =
    UncheckedRange<bidirectional_iterator<int*>, SentinelForNotSemiregular>;
using BidirectionalRangeNotSentinelWeaklyEqualityComparableWith =
    UncheckedRange<bidirectional_iterator<int*>, SentinelForNotWeaklyEqualityComparableWith>;

static_assert(std::forward_iterator<BidirectionalIteratorNotDerivedFrom>);
static_assert(!std::bidirectional_iterator<BidirectionalIteratorNotDerivedFrom>);
static_assert(!std::ranges::bidirectional_range<BidirectionalRangeNotDerivedFrom>);

class BidirectionalIteratorNotDecrementable {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::bidirectional_iterator_tag;

  BidirectionalIteratorNotDecrementable& operator++();
  BidirectionalIteratorNotDecrementable operator++(int);
  int& operator*() const;

  bool operator==(const BidirectionalIteratorNotDecrementable&) const = default;
};

using BidirectionalRangeNotDecrementable = UncheckedRange<BidirectionalIteratorNotDecrementable>;

static_assert(std::forward_iterator<BidirectionalIteratorNotDecrementable>);
static_assert(!std::bidirectional_iterator<BidirectionalIteratorNotDecrementable>);
static_assert(!std::ranges::bidirectional_range<BidirectionalRangeNotDecrementable>);

class PermutableNotForwardIterator {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::input_iterator_tag;

  PermutableNotForwardIterator& operator++();
  void operator++(int);
  int& operator*() const;
};

using PermutableRangeNotForwardIterator = UncheckedRange<PermutableNotForwardIterator>;

static_assert(std::input_iterator<PermutableNotForwardIterator>);
static_assert(!std::forward_iterator<PermutableNotForwardIterator>);
static_assert(!std::permutable<PermutableNotForwardIterator>);

class PermutableNotSwappable {
public:
  class NotSwappable {
    NotSwappable(NotSwappable&&) = delete;
  };

  using difference_type = long;
  using value_type = NotSwappable;
  using iterator_category = std::contiguous_iterator_tag;

  PermutableNotSwappable& operator++();
  PermutableNotSwappable operator++(int);
  NotSwappable& operator*() const;

  bool operator==(const PermutableNotSwappable&) const = default;
};

using PermutableRangeNotSwappable = UncheckedRange<PermutableNotSwappable>;

static_assert(std::input_iterator<PermutableNotSwappable>);
static_assert(std::forward_iterator<PermutableNotSwappable>);
static_assert(!std::permutable<PermutableNotSwappable>);
static_assert(!std::indirectly_swappable<PermutableNotSwappable>);

class OutputIteratorNotInputOrOutputIterator {
public:
  using difference_type = long;
  using value_type = int;
  using iterator_category = std::input_iterator_tag;

  int& operator++();
  void operator++(int);
  int& operator*();
};

using OutputRangeNotInputOrOutputIterator = UncheckedRange<InputIteratorNotInputOrOutputIterator>;

static_assert(!std::input_or_output_iterator<OutputIteratorNotInputOrOutputIterator>);
static_assert(std::indirectly_writable<OutputIteratorNotInputOrOutputIterator, int>);
static_assert(!std::output_iterator<OutputIteratorNotInputOrOutputIterator, int>);
static_assert(!std::ranges::output_range<OutputRangeNotInputOrOutputIterator, int>);

class OutputIteratorNotIndirectlyWritable {
public:
  using difference_type = long;
  using iterator_category = std::input_iterator_tag;

  OutputIteratorNotIndirectlyWritable& operator++();
  void operator++(int);
  const int& operator*() const;
};

using OutputRangeNotIndirectlyWritable = UncheckedRange<OutputIteratorNotIndirectlyWritable>;

static_assert(std::input_or_output_iterator<OutputIteratorNotIndirectlyWritable>);
static_assert(!std::indirectly_writable<OutputIteratorNotIndirectlyWritable, int>);
static_assert(!std::output_iterator<OutputIteratorNotIndirectlyWritable, int>);
static_assert(!std::ranges::output_range<OutputIteratorNotIndirectlyWritable, int>);

class IndirectBinaryPredicateNotIndirectlyReadable {
public:
  using difference_type = long;
  using iterator_category = std::input_iterator_tag;

  int& operator++();
  void operator++(int);
  const int& operator*() const;
};

using InputRangeIndirectBinaryPredicateNotIndirectlyReadable
     = UncheckedRange<cpp20_input_iterator<int*>, IndirectBinaryPredicateNotIndirectlyReadable>;

static_assert(!std::indirect_binary_predicate<std::ranges::equal_to, IndirectBinaryPredicateNotIndirectlyReadable, int*>);

class RandomAccessIteratorNotDerivedFrom {
  using Self = RandomAccessIteratorNotDerivedFrom;

public:
  using value_type = int;
  using difference_type = long;
  using pointer = int*;
  using reference = int&;
  // Deliberately not using the `std::random_access_iterator_tag` category.
  using iterator_category = std::bidirectional_iterator_tag;

  reference operator*() const;
  reference operator[](difference_type) const;

  Self& operator++();
  Self& operator--();
  Self operator++(int);
  Self operator--(int);

  Self& operator+=(difference_type);
  Self& operator-=(difference_type);
  friend Self operator+(Self, difference_type);
  friend Self operator+(difference_type, Self);
  friend Self operator-(Self, difference_type);
  friend difference_type operator-(Self, Self);

  auto operator<=>(const Self&) const = default;
};

static_assert(std::bidirectional_iterator<RandomAccessIteratorNotDerivedFrom>);
static_assert(!std::random_access_iterator<RandomAccessIteratorNotDerivedFrom>);

using RandomAccessRangeNotDerivedFrom = UncheckedRange<RandomAccessIteratorNotDerivedFrom>;

class RandomAccessIteratorBadIndex {
  using Self = RandomAccessIteratorBadIndex;

public:
  using value_type = int;
  using difference_type = long;
  using pointer = int*;
  using reference = int&;
  using iterator_category = std::random_access_iterator_tag;

  reference operator*() const;
  // Deliberately returning a type different from `reference`.
  const int& operator[](difference_type) const;

  Self& operator++();
  Self& operator--();
  Self operator++(int);
  Self operator--(int);

  Self& operator+=(difference_type);
  Self& operator-=(difference_type);
  friend Self operator+(Self, difference_type);
  friend Self operator+(difference_type, Self);
  friend Self operator-(Self, difference_type);
  friend difference_type operator-(Self, Self);

  auto operator<=>(const Self&) const = default;
};

static_assert(std::bidirectional_iterator<RandomAccessIteratorBadIndex>);
static_assert(!std::random_access_iterator<RandomAccessIteratorBadIndex>);

using RandomAccessRangeBadIndex = UncheckedRange<RandomAccessIteratorBadIndex>;

class RandomAccessIteratorBadDifferenceType {
  using Self = RandomAccessIteratorBadDifferenceType;

public:
  using value_type = int;
  // Deliberately use a non-integer `difference_type`
  using difference_type   = double;
  using pointer           = double*;
  using reference         = double&;
  using iterator_category = std::random_access_iterator_tag;

  reference operator*() const;
  reference operator[](difference_type) const;

  Self& operator++();
  Self& operator--();
  Self operator++(int);
  Self operator--(int);

  Self& operator+=(difference_type);
  Self& operator-=(difference_type);
  friend Self operator+(Self, difference_type);
  friend Self operator+(difference_type, Self);
  friend Self operator-(Self, difference_type);
  friend difference_type operator-(Self, Self);

  auto operator<=>(const Self&) const = default;
};

static_assert(std::regular<RandomAccessIteratorBadDifferenceType>);
static_assert(!std::weakly_incrementable<RandomAccessIteratorBadDifferenceType>);
static_assert(!std::random_access_iterator<RandomAccessIteratorBadDifferenceType>);

template <class Iter>
class ComparatorNotCopyable {
public:
  ComparatorNotCopyable(ComparatorNotCopyable&&) = default;
  ComparatorNotCopyable& operator=(ComparatorNotCopyable&&) = default;
  ComparatorNotCopyable(const ComparatorNotCopyable&) = delete;
  ComparatorNotCopyable& operator=(const ComparatorNotCopyable&) = delete;

  bool operator()(Iter&, Iter&) const;
};

#endif // ALMOST_SATISFIES_TYPES_H
