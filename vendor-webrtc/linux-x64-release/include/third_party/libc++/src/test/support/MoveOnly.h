//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef MOVEONLY_H
#define MOVEONLY_H

#include "test_macros.h"

#include <cstddef>
#include <functional>

class MoveOnly
{
    int data_;
public:
    TEST_CONSTEXPR MoveOnly(int data = 1) : data_(data) {}

    MoveOnly(const MoveOnly&) = delete;
    MoveOnly& operator=(const MoveOnly&) = delete;

    TEST_CONSTEXPR_CXX14 MoveOnly(MoveOnly&& x) TEST_NOEXCEPT
        : data_(x.data_) {x.data_ = 0;}
    TEST_CONSTEXPR_CXX14 MoveOnly& operator=(MoveOnly&& x)
        {data_ = x.data_; x.data_ = 0; return *this;}

    TEST_CONSTEXPR int get() const {return data_;}

    friend TEST_CONSTEXPR bool operator==(const MoveOnly& x, const MoveOnly& y)
        { return x.data_ == y.data_; }
    friend TEST_CONSTEXPR bool operator!=(const MoveOnly& x, const MoveOnly& y)
        { return x.data_ != y.data_; }
    friend TEST_CONSTEXPR bool operator< (const MoveOnly& x, const MoveOnly& y)
        { return x.data_ <  y.data_; }
    friend TEST_CONSTEXPR bool operator<=(const MoveOnly& x, const MoveOnly& y)
        { return x.data_ <= y.data_; }
    friend TEST_CONSTEXPR bool operator> (const MoveOnly& x, const MoveOnly& y)
        { return x.data_ >  y.data_; }
    friend TEST_CONSTEXPR bool operator>=(const MoveOnly& x, const MoveOnly& y)
        { return x.data_ >= y.data_; }

#if TEST_STD_VER > 17
    friend constexpr auto operator<=>(const MoveOnly&, const MoveOnly&) = default;
#endif // TEST_STD_VER > 17

    TEST_CONSTEXPR_CXX14 MoveOnly operator+(const MoveOnly& x) const
        { return MoveOnly(data_ + x.data_); }
    TEST_CONSTEXPR_CXX14 MoveOnly operator*(const MoveOnly& x) const
        { return MoveOnly(data_ * x.data_); }

    template<class T>
    friend void operator,(MoveOnly const&, T) = delete;

    template<class T>
    friend void operator,(T, MoveOnly const&) = delete;
};

template <>
struct std::hash<MoveOnly>
{
    typedef MoveOnly argument_type;
    typedef std::size_t result_type;
    TEST_CONSTEXPR std::size_t operator()(const MoveOnly& x) const {return static_cast<size_t>(x.get());}
};

class TrivialMoveOnly {
    int data_;

  public:
    TEST_CONSTEXPR TrivialMoveOnly(int data = 1) : data_(data) {}

    TrivialMoveOnly(const TrivialMoveOnly&)            = delete;
    TrivialMoveOnly& operator=(const TrivialMoveOnly&) = delete;

    TrivialMoveOnly(TrivialMoveOnly&&)            = default;
    TrivialMoveOnly& operator=(TrivialMoveOnly&&) = default;

    TEST_CONSTEXPR int get() const { return data_; }

    friend TEST_CONSTEXPR bool operator==(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ == y.data_;
    }
    friend TEST_CONSTEXPR bool operator!=(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ != y.data_;
    }
    friend TEST_CONSTEXPR bool operator<(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ < y.data_;
    }
    friend TEST_CONSTEXPR bool operator<=(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ <= y.data_;
    }
    friend TEST_CONSTEXPR bool operator>(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ > y.data_;
    }
    friend TEST_CONSTEXPR bool operator>=(const TrivialMoveOnly& x, const TrivialMoveOnly& y) {
      return x.data_ >= y.data_;
    }

#if TEST_STD_VER > 17
    friend constexpr auto operator<=>(const TrivialMoveOnly&, const TrivialMoveOnly&) = default;
#endif // TEST_STD_VER > 17

    TEST_CONSTEXPR_CXX14 TrivialMoveOnly operator+(const TrivialMoveOnly& x) const {
      return TrivialMoveOnly(data_ + x.data_);
    }
    TEST_CONSTEXPR_CXX14 TrivialMoveOnly operator*(const TrivialMoveOnly& x) const {
      return TrivialMoveOnly(data_ * x.data_);
    }

    template<class T>
    friend void operator,(TrivialMoveOnly const&, T) = delete;

    template<class T>
    friend void operator,(T, TrivialMoveOnly const&) = delete;
};

template <>
struct std::hash<TrivialMoveOnly> {
    typedef TrivialMoveOnly argument_type;
    typedef std::size_t result_type;
    TEST_CONSTEXPR std::size_t operator()(const TrivialMoveOnly& x) const { return static_cast<size_t>(x.get()); }
};

#endif // MOVEONLY_H
