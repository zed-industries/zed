//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TRANSPARENT_H
#define TRANSPARENT_H

#include "test_macros.h"

#include <functional> // for std::equal_to

// testing transparent
#if TEST_STD_VER > 11

struct transparent_less
{
    template <class T, class U>
    constexpr auto operator()(T&& t, U&& u) const
    noexcept(noexcept(std::forward<T>(t) < std::forward<U>(u)))
    -> decltype      (std::forward<T>(t) < std::forward<U>(u))
        { return      std::forward<T>(t) < std::forward<U>(u); }
    using is_transparent = void;  // correct
};

struct transparent_less_not_referenceable
{
    template <class T, class U>
    constexpr auto operator()(T&& t, U&& u) const
    noexcept(noexcept(std::forward<T>(t) < std::forward<U>(u)))
    -> decltype      (std::forward<T>(t) < std::forward<U>(u))
        { return      std::forward<T>(t) < std::forward<U>(u); }
    using is_transparent = void () const &;  // it's a type; a weird one, but a type
};

struct transparent_less_no_type
{
    template <class T, class U>
    constexpr auto operator()(T&& t, U&& u) const
    noexcept(noexcept(std::forward<T>(t) < std::forward<U>(u)))
    -> decltype      (std::forward<T>(t) < std::forward<U>(u))
        { return      std::forward<T>(t) < std::forward<U>(u); }
private:
//    using is_transparent = void;  // error - should exist
};

struct transparent_less_private
{
    template <class T, class U>
    constexpr auto operator()(T&& t, U&& u) const
    noexcept(noexcept(std::forward<T>(t) < std::forward<U>(u)))
    -> decltype      (std::forward<T>(t) < std::forward<U>(u))
        { return      std::forward<T>(t) < std::forward<U>(u); }
private:
    using is_transparent = void;  // error - should be accessible
};

struct transparent_less_not_a_type
{
    template <class T, class U>
    constexpr auto operator()(T&& t, U&& u) const
    noexcept(noexcept(std::forward<T>(t) < std::forward<U>(u)))
    -> decltype      (std::forward<T>(t) < std::forward<U>(u))
        { return      std::forward<T>(t) < std::forward<U>(u); }

    int is_transparent;  // error - should be a type
};

struct C2Int { // comparable to int
    C2Int() : i_(0) {}
    C2Int(int i): i_(i) {}
    int get () const { return i_; }
private:
    int i_;
    };

bool operator <(int          rhs,   const C2Int& lhs) { return rhs       < lhs.get(); }
bool operator <(const C2Int& rhs,   const C2Int& lhs) { return rhs.get() < lhs.get(); }
bool operator <(const C2Int& rhs,            int lhs) { return rhs.get() < lhs; }

#endif // TEST_STD_VER > 11

#endif // TRANSPARENT_H
