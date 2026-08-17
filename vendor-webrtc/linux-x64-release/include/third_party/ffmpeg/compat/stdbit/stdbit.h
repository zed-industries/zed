/*
 * Copyright (C) 2023 Rémi Denis-Courmont
 *
 * This program is free software; you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2.1 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston MA 02110-1301, USA.
 */

#ifndef __STDC_VERSION_STDBIT_H__
#define __STDC_VERSION_STDBIT_H__ 202311L

#include <stdbool.h>
#include <limits.h> /* CHAR_BIT */

#define __STDC_ENDIAN_LITTLE__ 1234
#define __STDC_ENDIAN_BIG__    4321

#ifdef __BYTE_ORDER__
# if (__BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__)
#  define __STDC_ENDIAN_NATIVE__ __STDC_ENDIAN_LITTLE__
# elif (__BYTE_ORDER__ == __ORDER_BIG_ENDIAN__)
#  define __STDC_ENDIAN_NATIVE__ __STDC_ENDIAN_BIG__
# else
#  define __STDC_ENDIAN_NATIVE__ 3412
# endif
#elif defined(_MSC_VER)
#  define __STDC_ENDIAN_NATIVE__ __STDC_ENDIAN_LITTLE__
#else
# error Not implemented.
#endif

#define __stdbit_generic_type_func(func, value) \
    _Generic (value, \
        unsigned long long: stdc_##func##_ull((unsigned long long)(value)), \
        unsigned long:      stdc_##func##_ul((unsigned long)(value)), \
        unsigned int:       stdc_##func##_ui((unsigned int)(value)), \
        unsigned short:     stdc_##func##_us((unsigned short)(value)), \
        unsigned char:      stdc_##func##_uc((unsigned char)(value)))

#if defined (__GNUC__) || defined (__clang__)
static inline unsigned int stdc_leading_zeros_ull(unsigned long long value)
{
    return value ? __builtin_clzll(value) : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_leading_zeros_ul(unsigned long value)
{
    return value ? __builtin_clzl(value) : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_leading_zeros_ui(unsigned int value)
{
    return value ? __builtin_clz(value) : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_leading_zeros_us(unsigned short value)
{
    return stdc_leading_zeros_ui(value)
           - CHAR_BIT * (sizeof (int) - sizeof (value));
}

static inline unsigned int stdc_leading_zeros_uc(unsigned char value)
{
    return stdc_leading_zeros_ui(value) - (CHAR_BIT * (sizeof (int) - 1));
}
#else
static inline unsigned int __stdc_leading_zeros(unsigned long long value,
                                                unsigned int size)
{
    unsigned int zeros = size * CHAR_BIT;

    while (value != 0) {
        value >>= 1;
        zeros--;
    }

    return zeros;
}

static inline unsigned int stdc_leading_zeros_ull(unsigned long long value)
{
    return __stdc_leading_zeros(value, sizeof (value));
}

static inline unsigned int stdc_leading_zeros_ul(unsigned long value)
{
    return __stdc_leading_zeros(value, sizeof (value));
}

static inline unsigned int stdc_leading_zeros_ui(unsigned int value)
{
    return __stdc_leading_zeros(value, sizeof (value));
}

static inline unsigned int stdc_leading_zeros_us(unsigned short value)
{
    return __stdc_leading_zeros(value, sizeof (value));
}

static inline unsigned int stdc_leading_zeros_uc(unsigned char value)
{
    return __stdc_leading_zeros(value, sizeof (value));
}
#endif

#define stdc_leading_zeros(value) \
        __stdbit_generic_type_func(leading_zeros, value)

static inline unsigned int stdc_leading_ones_ull(unsigned long long value)
{
    return stdc_leading_zeros_ull(~value);
}

static inline unsigned int stdc_leading_ones_ul(unsigned long value)
{
    return stdc_leading_zeros_ul(~value);
}

static inline unsigned int stdc_leading_ones_ui(unsigned int value)
{
    return stdc_leading_zeros_ui(~value);
}

static inline unsigned int stdc_leading_ones_us(unsigned short value)
{
    return stdc_leading_zeros_us(~value);
}

static inline unsigned int stdc_leading_ones_uc(unsigned char value)
{
    return stdc_leading_zeros_uc(~value);
}

#define stdc_leading_ones(value) \
        __stdbit_generic_type_func(leading_ones, value)

#if defined (__GNUC__) || defined (__clang__)
static inline unsigned int stdc_trailing_zeros_ull(unsigned long long value)
{
    return value ? (unsigned int)__builtin_ctzll(value)
                 : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_ul(unsigned long value)
{
    return value ? (unsigned int)__builtin_ctzl(value)
                 : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_ui(unsigned int value)
{
    return value ? (unsigned int)__builtin_ctz(value)
                 : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_us(unsigned short value)
{
    return value ? (unsigned int)__builtin_ctz(value)
                 : (CHAR_BIT * sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_uc(unsigned char value)
{
    return value ? (unsigned int)__builtin_ctz(value)
                 : (CHAR_BIT * sizeof (value));
}
#else
static inline unsigned int __stdc_trailing_zeros(unsigned long long value,
                                                 unsigned int size)
{
    unsigned int zeros = 0;

    if (!value)
        return size * CHAR_BIT;

    while ((value & 1) == 0) {
        value >>= 1;
        zeros++;
    }

    return zeros;
}

static inline unsigned int stdc_trailing_zeros_ull(unsigned long long value)
{
    return __stdc_trailing_zeros(value, sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_ul(unsigned long value)
{
    return __stdc_trailing_zeros(value, sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_ui(unsigned int value)
{
    return __stdc_trailing_zeros(value, sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_us(unsigned short value)
{
    return __stdc_trailing_zeros(value, sizeof (value));
}

static inline unsigned int stdc_trailing_zeros_uc(unsigned char value)
{
    return __stdc_trailing_zeros(value, sizeof (value));
}
#endif

#define stdc_trailing_zeros(value) \
        __stdbit_generic_type_func(trailing_zeros, value)

static inline unsigned int stdc_trailing_ones_ull(unsigned long long value)
{
    return stdc_trailing_zeros_ull(~value);
}

static inline unsigned int stdc_trailing_ones_ul(unsigned long value)
{
    return stdc_trailing_zeros_ul(~value);
}

static inline unsigned int stdc_trailing_ones_ui(unsigned int value)
{
    return stdc_trailing_zeros_ui(~value);
}

static inline unsigned int stdc_trailing_ones_us(unsigned short value)
{
    return stdc_trailing_zeros_us(~value);
}

static inline unsigned int stdc_trailing_ones_uc(unsigned char value)
{
    return stdc_trailing_zeros_uc(~value);
}

#define stdc_trailing_ones(value) \
        __stdbit_generic_type_func(trailing_ones, value)

static inline unsigned int stdc_first_leading_one_ull(unsigned long long value)
{
    return value ? (stdc_leading_zeros_ull(value) + 1) : 0;
}

static inline unsigned int stdc_first_leading_one_ul(unsigned long value)
{
    return value ? (stdc_leading_zeros_ul(value) + 1) : 0;
}

static inline unsigned int stdc_first_leading_one_ui(unsigned int value)
{
    return value ? (stdc_leading_zeros_ui(value) + 1) : 0;
}

static inline unsigned int stdc_first_leading_one_us(unsigned short value)
{
    return value ? (stdc_leading_zeros_us(value) + 1) : 0;
}

static inline unsigned int stdc_first_leading_one_uc(unsigned char value)
{
    return value ? (stdc_leading_zeros_uc(value) + 1) : 0;
}

#define stdc_first_leading_one(value) \
        __stdbit_generic_type_func(first_leading_one, value)

static inline unsigned int stdc_first_leading_zero_ull(unsigned long long value)
{
    return stdc_leading_ones_ull(~value);
}

static inline unsigned int stdc_first_leading_zero_ul(unsigned long value)
{
    return stdc_leading_ones_ul(~value);
}

static inline unsigned int stdc_first_leading_zero_ui(unsigned int value)
{
    return stdc_leading_ones_ui(~value);
}

static inline unsigned int stdc_first_leading_zero_us(unsigned short value)
{
    return stdc_leading_ones_us(~value);
}

static inline unsigned int stdc_first_leading_zero_uc(unsigned char value)
{
    return stdc_leading_ones_uc(~value);
}

#define stdc_first_leading_zero(value) \
        __stdbit_generic_type_func(first_leading_zero, value)

#if defined (__GNUC__) || defined (__clang__)
static inline unsigned int stdc_first_trailing_one_ull(unsigned long long value)
{
    return __builtin_ffsll(value);
}

static inline unsigned int stdc_first_trailing_one_ul(unsigned long value)
{
    return __builtin_ffsl(value);
}

static inline unsigned int stdc_first_trailing_one_ui(unsigned int value)
{
    return __builtin_ffs(value);
}

static inline unsigned int stdc_first_trailing_one_us(unsigned short value)
{
    return __builtin_ffs(value);
}

static inline unsigned int stdc_first_trailing_one_uc(unsigned char value)
{
    return __builtin_ffs(value);
}
#else
static inline unsigned int stdc_first_trailing_one_ull(unsigned long long value)
{
    return value ? (1 + stdc_trailing_zeros_ull(value)) : 0;
}

static inline unsigned int stdc_first_trailing_one_ul(unsigned long value)
{
    return value ? (1 + stdc_trailing_zeros_ul(value)) : 0;
}

static inline unsigned int stdc_first_trailing_one_ui(unsigned int value)
{
    return value ? (1 + stdc_trailing_zeros_ui(value)) : 0;
}

static inline unsigned int stdc_first_trailing_one_us(unsigned short value)
{
    return value ? (1 + stdc_trailing_zeros_us(value)) : 0;
}

static inline unsigned int stdc_first_trailing_one_uc(unsigned char value)
{
    return value ? (1 + stdc_trailing_zeros_uc(value)) : 0;
}
#endif

#define stdc_first_trailing_one(value) \
        __stdbit_generic_type_func(first_trailing_one, value)

static inline unsigned int stdc_first_trailing_zero_ull(unsigned long long value)
{
    return stdc_first_trailing_one_ull(~value);
}

static inline unsigned int stdc_first_trailing_zero_ul(unsigned long value)
{
    return stdc_first_trailing_one_ul(~value);
}

static inline unsigned int stdc_first_trailing_zero_ui(unsigned int value)
{
    return stdc_first_trailing_one_ui(~value);
}

static inline unsigned int stdc_first_trailing_zero_us(unsigned short value)
{
    return stdc_first_trailing_one_us(~value);
}

static inline unsigned int stdc_first_trailing_zero_uc(unsigned char value)
{
    return stdc_first_trailing_one_uc(~value);
}

#define stdc_first_trailing_zero(value) \
        __stdbit_generic_type_func(first_trailing_zero, value)

#if defined (__GNUC__) || defined (__clang__)
static inline unsigned int stdc_count_ones_ull(unsigned long long value)
{
    return __builtin_popcountll(value);
}

static inline unsigned int stdc_count_ones_ul(unsigned long value)
{
    return __builtin_popcountl(value);
}

static inline unsigned int stdc_count_ones_ui(unsigned int value)
{
    return __builtin_popcount(value);
}

static inline unsigned int stdc_count_ones_us(unsigned short value)
{
    return __builtin_popcount(value);
}

static inline unsigned int stdc_count_ones_uc(unsigned char value)
{
    return __builtin_popcount(value);
}
#else
static inline unsigned int __stdc_count_ones(unsigned long long value,
                                             unsigned int size)
{
    unsigned int ones = 0;

    for (unsigned int c = 0; c < (size * CHAR_BIT); c++) {
         ones += value & 1;
         value >>= 1;
    }

    return ones;
}

static inline unsigned int stdc_count_ones_ull(unsigned long long value)
{
    return __stdc_count_ones(value, sizeof (value));
}

static inline unsigned int stdc_count_ones_ul(unsigned long value)
{
    return __stdc_count_ones(value, sizeof (value));
}

static inline unsigned int stdc_count_ones_ui(unsigned int value)
{
    return __stdc_count_ones(value, sizeof (value));
}

static inline unsigned int stdc_count_ones_us(unsigned short value)
{
    return __stdc_count_ones(value, sizeof (value));
}

static inline unsigned int stdc_count_ones_uc(unsigned char value)
{
    return __stdc_count_ones(value, sizeof (value));
}
#endif

#define stdc_count_ones(value) \
        __stdbit_generic_type_func(count_ones, value)

static inline unsigned int stdc_count_zeros_ull(unsigned long long value)
{
    return stdc_count_ones_ull(~value);
}

static inline unsigned int stdc_count_zeros_ul(unsigned long value)
{
    return stdc_count_ones_ul(~value);
}

static inline unsigned int stdc_count_zeros_ui(unsigned int value)
{
    return stdc_count_ones_ui(~value);
}

static inline unsigned int stdc_count_zeros_us(unsigned short value)
{
    return stdc_count_ones_us(~value);
}

static inline unsigned int stdc_count_zeros_uc(unsigned char value)
{
    return stdc_count_ones_uc(~value);
}

#define stdc_count_zeros(value) \
        __stdbit_generic_type_func(count_zeros, value)

static inline bool stdc_has_single_bit_ull(unsigned long long value)
{
    return value && (value & (value - 1)) == 0;
}

static inline bool stdc_has_single_bit_ul(unsigned long value)
{
    return value && (value & (value - 1)) == 0;
}

static inline bool stdc_has_single_bit_ui(unsigned int value)
{
    return value && (value & (value - 1)) == 0;
}

static inline bool stdc_has_single_bit_us(unsigned short value)
{
    return value && (value & (value - 1)) == 0;
}

static inline bool stdc_has_single_bit_uc(unsigned char value)
{
    return value && (value & (value - 1)) == 0;
}

#define stdc_has_single_bit(value) \
        __stdbit_generic_type_func(has_single_bit, value)

static inline unsigned int stdc_bit_width_ull(unsigned long long value)
{
    return (CHAR_BIT * sizeof (value)) - stdc_leading_zeros_ull(value);
}

static inline unsigned int stdc_bit_width_ul(unsigned long value)
{
    return (CHAR_BIT * sizeof (value)) - stdc_leading_zeros_ul(value);
}

static inline unsigned int stdc_bit_width_ui(unsigned int value)
{
    return (CHAR_BIT * sizeof (value)) - stdc_leading_zeros_ui(value);
}

static inline unsigned int stdc_bit_width_us(unsigned short value)
{
    return (CHAR_BIT * sizeof (value)) - stdc_leading_zeros_us(value);
}

static inline unsigned int stdc_bit_width_uc(unsigned char value)
{
    return (CHAR_BIT * sizeof (value)) - stdc_leading_zeros_uc(value);
}

#define stdc_bit_width(value) \
        __stdbit_generic_type_func(bit_width, value)

static inline unsigned long long stdc_bit_floor_ull(unsigned long long value)
{
    return value ? (1ULL << (stdc_bit_width_ull(value) - 1)) : 0ULL;
}

static inline unsigned long stdc_bit_floor_ul(unsigned long value)
{
    return value ? (1UL << (stdc_bit_width_ul(value) - 1)) : 0UL;
}

static inline unsigned int stdc_bit_floor_ui(unsigned int value)
{
    return value ? (1U << (stdc_bit_width_ui(value) - 1)) : 0U;
}

static inline unsigned short stdc_bit_floor_us(unsigned short value)
{
    return value ? (1U << (stdc_bit_width_us(value) - 1)) : 0U;
}

static inline unsigned int stdc_bit_floor_uc(unsigned char value)
{
    return value ? (1U << (stdc_bit_width_uc(value) - 1)) : 0U;
}

#define stdc_bit_floor(value) \
        __stdbit_generic_type_func(bit_floor, value)

/* NOTE: Bit ceiling undefines overflow. */
static inline unsigned long long stdc_bit_ceil_ull(unsigned long long value)
{
    return 1ULL << (value ? stdc_bit_width_ull(value - 1) : 0);
}

static inline unsigned long stdc_bit_ceil_ul(unsigned long value)
{
    return 1UL << (value ? stdc_bit_width_ul(value - 1) : 0);
}

static inline unsigned int stdc_bit_ceil_ui(unsigned int value)
{
    return 1U << (value ? stdc_bit_width_ui(value - 1) : 0);
}

static inline unsigned short stdc_bit_ceil_us(unsigned short value)
{
    return 1U << (value ? stdc_bit_width_us(value - 1) : 0);
}

static inline unsigned int stdc_bit_ceil_uc(unsigned char value)
{
    return 1U << (value ? stdc_bit_width_uc(value - 1) : 0);
}

#define stdc_bit_ceil(value) \
        __stdbit_generic_type_func(bit_ceil, value)

#endif /* __STDC_VERSION_STDBIT_H__ */
