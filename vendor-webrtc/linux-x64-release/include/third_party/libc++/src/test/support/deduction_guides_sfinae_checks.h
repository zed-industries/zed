//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_DEDUCTION_GUIDES_SFINAE_CHECKS_H
#define TEST_SUPPORT_DEDUCTION_GUIDES_SFINAE_CHECKS_H

#include <cstddef>
#include <functional>
#include <initializer_list>
#include <iterator>
#include <memory>
#include <type_traits>
#include <utility>
#include <vector>

#include "test_macros.h"
#if TEST_STD_VER >= 23
#include "almost_satisfies_types.h"
#endif

#if TEST_STD_VER >= 23

template <class T>
struct RangeT {
  T* begin();
  T* end();
};
static_assert(std::ranges::input_range<RangeT<int>>);

template <class T>
using BadRangeT = InputRangeNotDerivedFromGeneric<T>;
static_assert(std::ranges::range<BadRangeT<int>>);
static_assert(!std::ranges::input_range<BadRangeT<int>>);

#endif

// `SFINAEs_away` template variable checks whether the template arguments for
// a given template class `Instantiated` can be deduced from the given
// constructor parameter types `CtrArgs` using CTAD.

template<template<typename ...> class Instantiated, class ...CtrArgs,
    class = decltype(Instantiated(std::declval<CtrArgs>()...))>
std::false_type SFINAEs_away_impl(int);

template<template<typename ...> class Instantiated, class ...CtrArgs>
std::true_type SFINAEs_away_impl(...);

template<template<typename ...> class Instantiated, class ...CtrArgs>
constexpr bool SFINAEs_away =
    decltype(SFINAEs_away_impl<Instantiated, CtrArgs...>(0))::value;

struct Empty {};

// For sequence containers the deduction guides should be SFINAE'd away when
// given:
// - "bad" input iterators (that is, a type not qualifying as an input
//   iterator);
// - a bad allocator;
// - a range not satisfying the `input_range` concept.
template<template<typename ...> class Container, typename InstantiatedContainer, typename BadAlloc = Empty>
constexpr void SequenceContainerDeductionGuidesSfinaeAway() {
  using T = typename InstantiatedContainer::value_type;
  using Alloc = std::allocator<T>;
  using Iter = T*;

  // Note: the only requirement in the Standard is that integral types cannot be
  // considered input iterators; however, this doesn't work for sequence
  // containers because they have constructors of the form `(size_type count,
  // const value_type& value)`. These constructors would be used when passing
  // two integral types and would deduce `value_type` to be an integral type.
#ifdef _LIBCPP_VERSION
  using OutputIter = std::insert_iterator<InstantiatedContainer>;
#endif // _LIBCPP_VERSION

  // (iter, iter)
  //
  // Cannot deduce from (BAD_iter, BAD_iter)
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter>);

  // (iter, iter, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, alloc)
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter, Alloc>);
  // Cannot deduce from (iter, iter, BAD_alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, BadAlloc>);

  // (alloc)
  //
  // Cannot deduce from (alloc)
  static_assert(SFINAEs_away<Container, Alloc>);

#if TEST_STD_VER >= 23
  using BadRange = BadRangeT<T>;

  // (from_range, range)
  //
  // Cannot deduce from (BAD_range)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange>);

  // (from_range, range, alloc)
  //
  // Cannot deduce from (range, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, RangeT<int>, BadAlloc>);
#endif
}

// Deduction guides should be SFINAE'd away when given:
// - a "bad" allocator (that is, a type not qualifying as an allocator);
// - an allocator instead of a container;
// - an allocator and a container that uses a different allocator;
// - a range not satisfying the `input_range` concept.
template<template<typename ...> class Container, typename InstantiatedContainer>
constexpr void ContainerAdaptorDeductionGuidesSfinaeAway() {
  using T = typename InstantiatedContainer::value_type;
  using Alloc [[maybe_unused]] = std::allocator<T>;
  using Iter = T*;

  using BadIter [[maybe_unused]] = int;
  using BadAlloc = Empty;

  // (container) -- no constraints.

  // (container, alloc)
  //
  // Cannot deduce from (container, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::vector<T>, BadAlloc>);

  // (iter, iter)
  //
  // Cannot deduce from (BAD_iter, BAD_iter)
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, BadIter, BadIter>);

#if TEST_STD_VER >= 23
  using BadRange = BadRangeT<T>;

  // (iter, iter, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, alloc)
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, BadIter, BadIter, Alloc>);
  // Cannot deduce from (iter, iter, BAD_alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, BadAlloc>);

  // (from_range, range)
  //
  // Cannot deduce from (BAD_range)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange>);

  // (from_range, range, alloc)
  //
  // Cannot deduce from (range, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, RangeT<int>, BadAlloc>);
#endif
}

// For associative containers the deduction guides should be SFINAE'd away when
// given:
// - "bad" input iterators (that is, a type not qualifying as an input
//   iterator);
// - a bad allocator;
// - an allocator in place of a comparator;
// - a range not satisfying the `input_range` concept.
template<template<typename ...> class Container, typename InstantiatedContainer>
constexpr void AssociativeContainerDeductionGuidesSfinaeAway() {
  using ValueType = typename InstantiatedContainer::value_type;
  using Comp = std::less<int>;
  using Alloc = std::allocator<ValueType>;
  using Iter = ValueType*;
  using InitList = std::initializer_list<ValueType>;

  struct BadAlloc {};
  // The only requirement in the Standard is that integral types cannot be
  // considered input iterators, beyond that it is unspecified.
  using BadIter = int;
#ifdef _LIBCPP_VERSION
  using OutputIter = std::insert_iterator<InstantiatedContainer>;
#endif // _LIBCPP_VERSION
  using AllocAsComp = Alloc;

  // (iter, iter)
  //
  // Cannot deduce from (BAD_iter, BAD_iter)
  static_assert(SFINAEs_away<Container, BadIter, BadIter>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter>);

  // (iter, iter, comp)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, comp)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, Comp>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter, Comp>);

  // (iter, iter, comp, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, comp, alloc)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, Comp, Alloc>);
  LIBCPP_STATIC_ASSERT(
      SFINAEs_away<Container, OutputIter, OutputIter, Comp, Alloc>);
  // Cannot deduce from (iter, iter, ALLOC_as_comp, alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, AllocAsComp, Alloc>);
  // Cannot deduce from (iter, iter, comp, BAD_alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, Comp, BadAlloc>);

  // (iter, iter, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, alloc)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, Alloc>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter, Alloc>);
  // Note: (iter, iter, BAD_alloc) is interpreted as (iter, iter, comp)
  // instead and fails upon instantiation. There is no requirement to SFINAE
  // away bad comparators.

  // (init_list, comp, alloc)
  //
  // Cannot deduce from (init_list, ALLOC_as_comp, alloc)
  static_assert(SFINAEs_away<Container, InitList, AllocAsComp, Alloc>);
  // Cannot deduce from (init_list, comp, BAD_alloc)
  static_assert(SFINAEs_away<Container, InitList, Comp, BadAlloc>);

  // (init_list, alloc)
  //
  // Note: (init_list, BAD_alloc) is interpreted as (init_list, comp) instead
  // and fails upon instantiation. There is no requirement to SFINAE away bad
  // comparators.

#if TEST_STD_VER >= 23
  using Range = RangeT<ValueType>;
  using BadRange = BadRangeT<ValueType>;

  // (from_range, range)
  //
  // Can deduce from (from_range, range)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range>);
  // Cannot deduce from (from_range, BAD_range)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange>);

  // (from_range, range, comp)
  //
  // Can deduce from (from_range, _range, comp)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, Comp>);
  // Cannot deduce from (from_range, BAD_range, comp)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, Comp>);

  // (from_range, range, comp, alloc)
  //
  // Can deduce from (from_range, range, comp, alloc)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, Comp, Alloc>);
  // Cannot deduce from (from_range, BAD_range, comp, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, Comp, Alloc>);
  // Cannot deduce from (from_range, range, comp, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, Comp, BadAlloc>);

  // (from_range, range, alloc)
  //
  // Can deduce from (from_range, range, alloc)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, Alloc>);
  // Cannot deduce from (from_range, BAD_range, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, Alloc>);
  // Note: (from_range, range, BAD_alloc) is interpreted as (from_range, range, comp) instead.
#endif
}

// For unordered containers the deduction guides should be SFINAE'd away when
// given:
// - "bad" input iterators (that is, a type not qualifying as an input
//   iterator);
// - a bad allocator;
// - a bad hash functor (an integral type in place of a hash);
// - an allocator in place of a hash functor;
// - an allocator in place of a predicate;
// - a range not satisfying the `input_range` concept.
template<template<typename ...> class Container, typename InstantiatedContainer>
constexpr void UnorderedContainerDeductionGuidesSfinaeAway() {
  using ValueType = typename InstantiatedContainer::value_type;
  using Pred = std::equal_to<int>;
  using Hash = std::hash<int>;
  using Alloc = std::allocator<ValueType>;
  using Iter = ValueType*;
  using InitList = std::initializer_list<ValueType>;

  using BadHash = short;
  struct BadAlloc {};
  // The only requirement in the Standard is that integral types cannot be
  // considered input iterators, beyond that it is unspecified.
  using BadIter = int;
#ifdef _LIBCPP_VERSION
  using OutputIter = std::insert_iterator<InstantiatedContainer>;
#endif // _LIBCPP_VERSION
  using AllocAsHash = Alloc;
  using AllocAsPred = Alloc;

  // (iter, iter)
  //
  // Cannot deduce from (BAD_iter, BAD_iter)
  static_assert(SFINAEs_away<Container, BadIter, BadIter>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter>);

  // (iter, iter, buckets)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, std::size_t>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter, std::size_t>);

  // (iter, iter, buckets, hash)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets, hash)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, std::size_t, Hash>);
  LIBCPP_STATIC_ASSERT(
      SFINAEs_away<Container, OutputIter, OutputIter, std::size_t, Hash>);
  // Cannot deduce from (iter, iter, buckets, BAD_hash)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, BadHash>);
  // Note: (iter, iter, buckets, ALLOC_as_hash) is allowed -- it just calls
  // (iter, iter, buckets, alloc)

  // (iter, iter, buckets, hash, pred)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets, hash, pred)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, std::size_t, Hash, Pred>);
  LIBCPP_STATIC_ASSERT(
      SFINAEs_away<Container, OutputIter, OutputIter, std::size_t, Hash, Pred>);
  // Cannot deduce from (iter, iter, buckets, BAD_hash, pred)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, BadHash, Pred>);
  // Cannot deduce from (iter, iter, buckets, ALLOC_as_hash, pred)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, AllocAsHash, Pred>);
  // Note: (iter, iter, buckets, hash, ALLOC_as_pred) is allowed -- it just
  // calls (iter, iter, buckets, hash, alloc)

  // (iter, iter, buckets, hash, pred, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets, hash, pred, alloc)
  static_assert(
      SFINAEs_away<Container, BadIter, BadIter, std::size_t, Hash, Pred, Alloc>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter,
      std::size_t, Hash, Pred, Alloc>);
  // Cannot deduce from (iter, iter, buckets, BAD_hash, pred, alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, BadHash, Pred, Alloc>);
  // Cannot deduce from (iter, iter, buckets, ALLOC_as_hash, pred, alloc)
  static_assert(
      SFINAEs_away<Container, Iter, Iter, std::size_t, AllocAsHash, Pred, Alloc>);
  // Cannot deduce from (iter, iter, buckets, hash, ALLOC_as_pred, alloc)
  static_assert(
      SFINAEs_away<Container, Iter, Iter, std::size_t, Hash, AllocAsPred, Alloc>);
  // Cannot deduce from (iter, iter, buckets, hash, pred, BAD_alloc)
  static_assert(
      SFINAEs_away<Container, Iter, Iter, std::size_t, Hash, Pred, BadAlloc>);

  // (iter, iter, buckets, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets, alloc)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, std::size_t, Alloc>);
  LIBCPP_STATIC_ASSERT(
      SFINAEs_away<Container, OutputIter, OutputIter, std::size_t, Alloc>);
  // Note: (iter, iter, buckets, BAD_alloc) is interpreted as (iter, iter,
  // buckets, hash), which is valid because the only requirement for the hash
  // parameter is that it's not integral.

  // (iter, iter, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, alloc)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, Alloc>);
  LIBCPP_STATIC_ASSERT(SFINAEs_away<Container, OutputIter, OutputIter, Alloc>);
  // Cannot deduce from (iter, iter, BAD_alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, BadAlloc>);

  // (iter, iter, buckets, hash, alloc)
  //
  // Cannot deduce from (BAD_iter, BAD_iter, buckets, hash, alloc)
  static_assert(SFINAEs_away<Container, BadIter, BadIter, std::size_t, Hash, Alloc>);
  LIBCPP_STATIC_ASSERT(
      SFINAEs_away<Container, OutputIter, OutputIter, std::size_t, Hash, Alloc>);
  // Cannot deduce from (iter, iter, buckets, BAD_hash, alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, BadHash, Alloc>);
  // Cannot deduce from (iter, iter, buckets, ALLOC_as_hash, alloc)
  static_assert(SFINAEs_away<Container, Iter, Iter, std::size_t, AllocAsHash, Alloc>);
  // Note: (iter, iter, buckets, hash, BAD_alloc) is interpreted as (iter, iter,
  // buckets, hash, pred), which is valid because there are no requirements for
  // the predicate.

  // (init_list, buckets, hash)
  //
  // Cannot deduce from (init_list, buckets, BAD_hash)
  static_assert(SFINAEs_away<Container, InitList, std::size_t, BadHash>);
  // Note: (init_list, buckets, ALLOC_as_hash) is interpreted as (init_list,
  // buckets, alloc), which is valid.

  // (init_list, buckets, hash, pred)
  //
  // Cannot deduce from (init_list, buckets, BAD_hash, pred)
  static_assert(SFINAEs_away<Container, InitList, std::size_t, BadHash, Pred>);
  // Cannot deduce from (init_list, buckets, ALLOC_as_hash, pred)
  static_assert(SFINAEs_away<Container, InitList, std::size_t, AllocAsHash, Pred>);
  // Note: (init_list, buckets, hash, ALLOC_as_pred) is interpreted as
  // (init_list, buckets, hash, alloc), which is valid.

  // (init_list, buckets, hash, pred, alloc)
  //
  // Cannot deduce from (init_list, buckets, BAD_hash, pred, alloc)
  static_assert(
      SFINAEs_away<Container, InitList, std::size_t, BadHash, Pred, Alloc>);
  // Cannot deduce from (init_list, buckets, ALLOC_as_hash, pred, alloc)
  static_assert(
      SFINAEs_away<Container, InitList, std::size_t, AllocAsHash, Pred, Alloc>);
  // Cannot deduce from (init_list, buckets, hash, ALLOC_as_pred, alloc)
  static_assert(
      SFINAEs_away<Container, InitList, std::size_t, Hash, AllocAsPred, Alloc>);
  // Cannot deduce from (init_list, buckets, hash, pred, BAD_alloc)
  static_assert(
      SFINAEs_away<Container, InitList, std::size_t, Hash, Pred, BadAlloc>);

  // (init_list, buckets, alloc)
  //
  // Note: (init_list, buckets, BAD_alloc) is interpreted as (init_list,
  // buckets, hash), which is valid because the only requirement for the hash
  // parameter is that it's not integral.

  // (init_list, buckets, hash, alloc)
  //
  // Cannot deduce from (init_list, buckets, BAD_hash, alloc)
  static_assert(SFINAEs_away<Container, InitList, std::size_t, BadHash, Alloc>);
  // Cannot deduce from (init_list, buckets, ALLOC_as_hash, alloc)
  static_assert(SFINAEs_away<Container, InitList, std::size_t, AllocAsHash, Alloc>);

  // (init_list, alloc)
  //
  // Cannot deduce from (init_list, BAD_alloc)
  static_assert(SFINAEs_away<Container, InitList, BadAlloc>);

#if TEST_STD_VER >= 23
  using Range = RangeT<ValueType>;
  using BadRange = BadRangeT<ValueType>;

  // (from_range, range)
  //
  // Can deduce from (from_range, range)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range>);
  // Cannot deduce from (from_range, BAD_range)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange>);

  // (from_range, range, buckets)
  //
  // Can deduce from (from_range, range, buckets)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t>);
  // Cannot deduce from (from_range, BAD_range, buckets)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t>);

  // (from_range, range, buckets, hash)
  //
  // Can deduce from (from_range, range, buckets, hash)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Hash>);
  // Cannot deduce from (from_range, BAD_range, buckets, hash)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t, Hash>);
  // Cannot deduce from (from_range, range, buckets, BAD_hash)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, BadHash>);

  // (from_range, range, buckets, hash, pred)
  //
  // Can deduce from (from_range, range, buckets, hash, pred)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Hash, Pred>);
  // Cannot deduce from (from_range, BAD_range, buckets, hash, pred)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t, Hash, Pred>);
  // Cannot deduce from (from_range, range, buckets, BAD_hash, pred)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, BadHash, Pred>);

  // (from_range, range, buckets, hash, pred, alloc)
  //
  // Can deduce from (from_range, range, buckets, hash, pred, alloc)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Hash, Pred, Alloc>);
  // Cannot deduce from (from_range, BAD_range, buckets, hash, pred, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t, Hash, Pred, Alloc>);
  // Cannot deduce from (from_range, range, buckets, BAD_hash, pred, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, BadHash, Pred, Alloc>);
  // Cannot deduce from (from_range, range, buckets, hash, pred, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Hash, Pred, BadAlloc>);

  // (from_range, range, buckets, alloc)
  //
  // Can deduce from (from_range, range, buckets, alloc)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Alloc>);
  // Cannot deduce from (from_range, BAD_range, buckets, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t, Alloc>);
  // Note: (from_range, range, buckets, BAD_alloc) is interpreted as (from_range, range, buckets, hash), which is valid
  // because the only requirement for the hash parameter is that it's not integral.

  // (from_range, range, alloc)
  //
  // Can deduce from (from_range, range, alloc)
  // TODO(LWG 2713): uncomment this test once the constructor is added.
  // static_assert(!SFINAEs_away<Container, std::from_range_t, Range, Alloc>);
  // Cannot deduce from (from_range, BAD_range, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, Alloc>);
  // Cannot deduce from (from_range, range, BAD_alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, BadAlloc>);

  // (from_range, range, buckets, hash, alloc)
  //
  // Can deduce from (from_range, range, buckets, hash, alloc)
  static_assert(!SFINAEs_away<Container, std::from_range_t, Range, std::size_t, Hash, Alloc>);
  // Cannot deduce from (from_range, BAD_range, buckets, hash, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, BadRange, std::size_t, Hash, Alloc>);
  // Cannot deduce from (from_range, range, buckets, BAD_hash, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, BadHash, Alloc>);
  // Cannot deduce from (from_range, range, buckets, ALLOC_as_hash, alloc)
  static_assert(SFINAEs_away<Container, std::from_range_t, Range, std::size_t, AllocAsHash, Alloc>);
  // Cannot deduce from (from_range, range, buckets, hash, BAD_alloc)
  // Note: (from_range, range, buckets, hash, BAD_alloc) is interpreted as (from_range, range, buckets, hash, pred),
  // which is valid because the only requirement for the predicate parameter is that it does not resemble an allocator.
#endif
}

#endif // TEST_SUPPORT_DEDUCTION_GUIDES_SFINAE_CHECKS_H
