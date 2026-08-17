//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef RANGES_RANGE_UTILITY_RANGE_UTILITY_CONV_CONTAINER_H
#define RANGES_RANGE_UTILITY_RANGE_UTILITY_CONV_CONTAINER_H

#include <algorithm>
#include <cstddef>

enum class CtrChoice { Invalid, DefaultCtrAndInsert, BeginEndPair, FromRangeT, DirectCtr };

enum class InserterChoice { Invalid, Insert, Emplace, PushBack, EmplaceBack };

// Allows checking that `ranges::to` correctly follows the order of priority of different constructors -- e.g., if
// 3 constructors are available, the `from_range_t` constructor is chosen in favor of the constructor taking two
// iterators, etc.
template <class ElementType, CtrChoice Rank, InserterChoice Inserter = InserterChoice::Insert, bool CanReserve = false>
struct Container {
  CtrChoice ctr_choice           = CtrChoice::Invalid;
  InserterChoice inserter_choice = InserterChoice::Invalid;
  bool called_reserve            = false;

  int extra_arg1  = 0;
  char extra_arg2 = 0;

  using value_type              = ElementType;
  static constexpr int Capacity = 8;
  int size_                     = 0;
  ElementType buffer_[Capacity] = {};

  // Case 1 -- construct directly from the range.

  constexpr explicit Container(std::ranges::input_range auto&& in)
    requires(Rank >= CtrChoice::DirectCtr)
      : ctr_choice(CtrChoice::DirectCtr), size_(static_cast<int>(std::ranges::size(in))) {
    std::ranges::copy(in, begin());
  }

  // Check that `ranges::to` can also pass extra parameters.
  constexpr explicit Container(std::ranges::input_range auto&& in, int arg1, char arg2)
    requires(Rank >= CtrChoice::DirectCtr)
      : Container(in) {
    extra_arg1 = arg1;
    extra_arg2 = arg2;
  }

  // Case 2 -- use `from_range_t` constructor.

  constexpr Container(std::from_range_t, std::ranges::input_range auto&& in)
    requires(Rank >= CtrChoice::FromRangeT)
      : ctr_choice(CtrChoice::FromRangeT), size_(static_cast<int>(std::ranges::size(in))) {
    std::ranges::copy(in, begin());
  }

  constexpr Container(std::from_range_t, std::ranges::input_range auto&& in, int arg1, char arg2)
    requires(Rank >= CtrChoice::FromRangeT)
      : Container(std::from_range, in) {
    extra_arg1 = arg1;
    extra_arg2 = arg2;
  }

  // Case 3 -- use begin-end pair.

  template <class Iter>
  constexpr Container(Iter b, Iter e)
    requires(Rank >= CtrChoice::BeginEndPair)
      : ctr_choice(CtrChoice::BeginEndPair), size_(static_cast<int>(e - b)) {
    std::ranges::copy(b, e, begin());
  }

  template <class Iter>
  constexpr Container(Iter b, Iter e, int arg1, char arg2)
    requires(Rank >= CtrChoice::BeginEndPair)
      : Container(b, e) {
    extra_arg1 = arg1;
    extra_arg2 = arg2;
  }

  // Case 4 -- default-construct and insert, reserving the size if possible.

  constexpr Container()
    requires(Rank >= CtrChoice::DefaultCtrAndInsert)
      : ctr_choice(CtrChoice::DefaultCtrAndInsert) {}

  constexpr Container(int arg1, char arg2)
    requires(Rank >= CtrChoice::DefaultCtrAndInsert)
      : ctr_choice(CtrChoice::DefaultCtrAndInsert), extra_arg1(arg1), extra_arg2(arg2) {}

  constexpr ElementType* begin() { return buffer_; }
  constexpr ElementType* end() { return buffer_ + size_; }
  constexpr std::size_t size() const { return size_; }

  template <class T>
  constexpr void emplace_back(T val)
    requires(Inserter >= InserterChoice::EmplaceBack)
  {
    inserter_choice = InserterChoice::EmplaceBack;
    __push_back_impl(val);
  }

  template <class T>
  constexpr void push_back(T val)
    requires(Inserter >= InserterChoice::PushBack)
  {
    inserter_choice = InserterChoice::PushBack;
    __push_back_impl(val);
  }

  template <class T>
  constexpr void __push_back_impl(T val) {
    buffer_[size_] = val;
    ++size_;
  }

  template <class T>
  constexpr ElementType* emplace(ElementType* where, T val)
    requires(Inserter >= InserterChoice::Emplace)
  {
    inserter_choice = InserterChoice::Emplace;
    return __insert_impl(where, val);
  }

  template <class T>
  constexpr ElementType* insert(ElementType* where, T val)
    requires(Inserter >= InserterChoice::Insert)
  {
    inserter_choice = InserterChoice::Insert;
    return __insert_impl(where, val);
  }

  template <class T>
  constexpr ElementType* __insert_impl(ElementType* where, T val) {
    assert(size() + 1 <= Capacity);
    std::shift_right(where, end(), 1);
    *where = val;
    ++size_;
    return where;
  }

  constexpr void reserve(size_t)
    requires CanReserve
  {
    called_reserve = true;
  }

  constexpr std::size_t capacity() const
    requires CanReserve
  {
    return Capacity;
  }

  constexpr std::size_t max_size() const
    requires CanReserve
  {
    return Capacity;
  }

  friend constexpr bool operator==(const Container&, const Container&) = default;
};

#endif // RANGES_RANGE_UTILITY_RANGE_UTILITY_CONV_CONTAINER_H
