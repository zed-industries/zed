//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SUPPORT_CHARCONV_TEST_HELPERS_H
#define SUPPORT_CHARCONV_TEST_HELPERS_H

#include <algorithm>
#include <cassert>
#include <cerrno>
#include <charconv>
#include <cstddef>
#include <limits>
#include <numeric>
#include <string.h>
#include <stdlib.h>
#include <type_traits>

#include "test_macros.h"

#if TEST_STD_VER < 11
#error This file requires C++11
#endif

using std::false_type;
using std::true_type;

template <typename To, typename From>
constexpr auto
is_non_narrowing(From a) -> decltype(To{a}, true_type())
{
    return {};
}

template <typename To>
constexpr auto
is_non_narrowing(...) -> false_type
{
    return {};
}

template <typename X, typename T>
constexpr bool
_fits_in(T, true_type /* non-narrowing*/, ...)
{
    return true;
}

template <typename X, typename T, typename xl = std::numeric_limits<X>>
constexpr bool
_fits_in(T v, false_type, true_type /* T signed*/, true_type /* X signed */)
{
    return xl::lowest() <= v && v <= (xl::max)();
}

template <typename X, typename T, typename xl = std::numeric_limits<X>>
constexpr bool
_fits_in(T v, false_type, true_type /* T signed */, false_type /* X unsigned*/)
{
    return 0 <= v && typename std::make_unsigned<T>::type(v) <= (xl::max)();
}

template <typename X, typename T, typename xl = std::numeric_limits<X>>
constexpr bool
_fits_in(T v, false_type, false_type /* T unsigned */, ...)
{
    return v <= typename std::make_unsigned<X>::type((xl::max)());
}

template <typename X, typename T>
constexpr bool
fits_in(T v)
{
  return _fits_in<X>(v, is_non_narrowing<X>(v), std::is_signed<T>(), std::is_signed<X>());
}

template <typename X>
struct to_chars_test_base
{
    template <typename T, std::size_t N, typename... Ts>
    TEST_CONSTEXPR_CXX23 void test(T v, char const (&expect)[N], Ts... args)
    {
        std::to_chars_result r;

        constexpr std::size_t len = N - 1;
        static_assert(len > 0, "expected output won't be empty");

        if (!fits_in<X>(v))
            return;

        r = std::to_chars(buf, buf + len - 1, X(v), args...);
        assert(r.ptr == buf + len - 1);
        assert(r.ec == std::errc::value_too_large);

        r = std::to_chars(buf, buf + sizeof(buf), X(v), args...);
        assert(r.ptr == buf + len);
        assert(r.ec == std::errc{});
        assert(std::equal(buf, buf + len, expect));
    }

    template <typename... Ts>
    TEST_CONSTEXPR_CXX23 void test_value(X v, Ts... args)
    {
        std::to_chars_result r;

        // Poison the buffer for testing whether a successful std::to_chars
        // doesn't modify data beyond r.ptr. Use unsigned values to avoid
        // overflowing char when it's signed.
        std::iota(buf, buf + sizeof(buf), static_cast<unsigned char>(1));
        r = std::to_chars(buf, buf + sizeof(buf), v, args...);
        assert(r.ec == std::errc{});
        for (std::size_t i = r.ptr - buf; i < sizeof(buf); ++i)
            assert(static_cast<unsigned char>(buf[i]) == i + 1);
        *r.ptr = '\0';

#ifndef TEST_HAS_NO_INT128
        if (sizeof(X) == sizeof(__int128_t)) {
            auto a = fromchars128_impl(buf, r.ptr, args...);
            assert(v == a);
        } else
#endif
        {
            auto a = fromchars_impl(buf, r.ptr, args...);
            assert(v == a);
        }

        auto ep = r.ptr - 1;
        r = std::to_chars(buf, ep, v, args...);
        assert(r.ptr == ep);
        assert(r.ec == std::errc::value_too_large);
    }

private:
    static TEST_CONSTEXPR_CXX23 long long fromchars_impl(char const* p, char const* ep, int base, true_type)
    {
        char* last;
        long long r;
        if (TEST_IS_CONSTANT_EVALUATED)
          last = const_cast<char*>(std::from_chars(p, ep, r, base).ptr);
        else
          r = strtoll(p, &last, base);
        assert(last == ep);

        return r;
    }

    static TEST_CONSTEXPR_CXX23 unsigned long long fromchars_impl(char const* p, char const* ep, int base, false_type)
    {
        char* last;
        unsigned long long r;
        if (TEST_IS_CONSTANT_EVALUATED)
          last = const_cast<char*>(std::from_chars(p, ep, r, base).ptr);
        else
          r = strtoull(p, &last, base);
        assert(last == ep);

        return r;
    }
#ifndef TEST_HAS_NO_INT128
    static TEST_CONSTEXPR_CXX23 __int128_t fromchars128_impl(char const* p, char const* ep, int base, true_type)
    {
        if (!TEST_IS_CONSTANT_EVALUATED) {
            char* last;
            __int128_t r = strtoll(p, &last, base);
            if(errno != ERANGE) {
                assert(last == ep);
                return r;
            }
        }

        // When the value doesn't fit in a long long use from_chars. This is
        // not ideal since it does a round-trip test instead if using an
        // external source.
        __int128_t r;
        std::from_chars_result s = std::from_chars(p, ep, r, base);
        assert(s.ec == std::errc{});
        assert(s.ptr == ep);

        return r;
    }

    static TEST_CONSTEXPR_CXX23 __uint128_t fromchars128_impl(char const* p, char const* ep, int base, false_type)
    {
        if (!TEST_IS_CONSTANT_EVALUATED) {
            char* last;
            __uint128_t r = strtoull(p, &last, base);
            if(errno != ERANGE) {
                assert(last == ep);
                return r;
            }
        }

        __uint128_t r;
        std::from_chars_result s = std::from_chars(p, ep, r, base);
        assert(s.ec == std::errc{});
        assert(s.ptr == ep);

        return r;
    }

    static TEST_CONSTEXPR_CXX23 auto fromchars128_impl(char const* p, char const* ep, int base = 10)
    -> decltype(fromchars128_impl(p, ep, base, std::is_signed<X>()))
    {
        return fromchars128_impl(p, ep, base, std::is_signed<X>());
    }

#endif

    static TEST_CONSTEXPR_CXX23 auto fromchars_impl(char const* p, char const* ep, int base = 10)
    -> decltype(fromchars_impl(p, ep, base, std::is_signed<X>()))
    {
        return fromchars_impl(p, ep, base, std::is_signed<X>());
    }

    char buf[150];
};

template <typename X>
struct roundtrip_test_base
{
    template <typename T, typename... Ts>
    TEST_CONSTEXPR_CXX23 void test(T v, Ts... args)
    {
        std::from_chars_result r2;
        std::to_chars_result r;
        X x = 0xc;

        if (fits_in<X>(v))
        {
            r = std::to_chars(buf, buf + sizeof(buf), v, args...);
            assert(r.ec == std::errc{});

            r2 = std::from_chars(buf, r.ptr, x, args...);
            assert(r2.ptr == r.ptr);
            assert(x == X(v));
        }
        else
        {
            r = std::to_chars(buf, buf + sizeof(buf), v, args...);
            assert(r.ec == std::errc{});

            r2 = std::from_chars(buf, r.ptr, x, args...);

            TEST_DIAGNOSTIC_PUSH
            TEST_MSVC_DIAGNOSTIC_IGNORED(4127) // conditional expression is constant

            if (std::is_signed<T>::value && v < 0 && std::is_unsigned<X>::value)
            {
                assert(x == 0xc);
                assert(r2.ptr == buf);
                assert(r2.ec == std::errc::invalid_argument);
            }
            else
            {
                assert(x == 0xc);
                assert(r2.ptr == r.ptr);
                assert(r2.ec == std::errc::result_out_of_range);
            }

            TEST_DIAGNOSTIC_POP
        }
    }

private:
    char buf[150];
};

template <typename... T>
struct type_list
{
};

template <typename L1, typename L2>
struct type_concat;

template <typename... Xs, typename... Ys>
struct type_concat<type_list<Xs...>, type_list<Ys...>>
{
    using type = type_list<Xs..., Ys...>;
};

template <typename L1, typename L2>
using concat_t = typename type_concat<L1, L2>::type;

template <typename L1, typename L2>
constexpr auto concat(L1, L2) -> concat_t<L1, L2>
{
    return {};
}

auto all_signed = type_list<
    char,
    signed char,
    short,
    int,
    long,
    long long
#ifndef TEST_HAS_NO_INT128
    ,
    __int128_t
#endif
    >();
auto all_unsigned = type_list<
    unsigned char,
    unsigned short,
    unsigned int,
    unsigned long,
    unsigned long long
#ifndef TEST_HAS_NO_INT128
    ,
    __uint128_t
#endif
    >();
auto integrals = concat(all_signed, all_unsigned);

auto all_floats = type_list< float, double >(); //TODO: Add long double

template <template <typename> class Fn, typename... Ts>
TEST_CONSTEXPR_CXX23 void
run(type_list<Ts...>)
{
    int ls[sizeof...(Ts)] = {(Fn<Ts>{}(), 0)...};
    (void)ls;
}

#endif // SUPPORT_CHARCONV_TEST_HELPERS_H
