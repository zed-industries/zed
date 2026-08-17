/*
 * Common code for checksum implementations
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

#ifndef CHKSUM_COMMON_H
#define CHKSUM_COMMON_H

#if __BYTE_ORDER__ != __ORDER_LITTLE_ENDIAN__
#error Only little endian supported
#endif

#include <limits.h>
#include <stdbool.h>
#include <stdint.h>
#include <string.h>

/* Assertions must be explicitly enabled */
#if WANT_ASSERT
#undef NDEBUG
#include <assert.h>
#define Assert(exp) assert(exp)
#else
#define Assert(exp) (void) (exp)
#endif

#ifdef __GNUC__
#define likely(x)     __builtin_expect(!!(x), 1)
#define unlikely(x)   __builtin_expect(!!(x), 0)
#define may_alias     __attribute__((__may_alias__))
#define always_inline __attribute__((always_inline))
#ifdef __clang__
#define no_unroll_loops
#else
#define no_unroll_loops  __attribute__((optimize("no-unroll-loops")))
#endif
#define bswap16(x)    __builtin_bswap16((x))
#else
#define likely(x)     (x)
#define unlikely(x)   (x)
#define may_alias
#define always_inline
#define no_unroll_loops
#define bswap16(x)    ((uint8_t)((x) >> 8) | ((uint8_t)(x) << 8))
#endif

#define ALL_ONES ~UINT64_C(0)

static inline
uint64_t load64(const void *ptr)
{
    /* GCC will optimise this to a normal load instruction */
    uint64_t v;
    memcpy(&v, ptr, sizeof v);
    return v;
}

static inline
uint32_t load32(const void *ptr)
{
    /* GCC will optimise this to a normal load instruction */
    uint32_t v;
    memcpy(&v, ptr, sizeof v);
    return v;
}

static inline
uint16_t load16(const void *ptr)
{
    /* GCC will optimise this to a normal load instruction */
    uint16_t v;
    memcpy(&v, ptr, sizeof v);
    return v;
}

/* slurp_small() is for small buffers, don't waste cycles on alignment */
no_unroll_loops
always_inline
static inline uint64_t
slurp_small(const void *ptr, uint32_t nbytes)
{
    const unsigned char *cptr = ptr;
    uint64_t sum = 0;
    while (nbytes >= 4)
    {
	sum += load32(cptr);
	cptr += 4;
	nbytes -= 4;
    }
    if (nbytes & 2)
    {
	sum += load16(cptr);
	cptr += 2;
    }
    if (nbytes & 1)
    {
	sum += (uint8_t) *cptr;
    }
    return sum;
}

static inline const void *
align_ptr(const void *ptr, size_t bytes)
{
    return (void *) ((uintptr_t) ptr & -(uintptr_t) bytes);
}

always_inline
static inline uint16_t
fold_and_swap(uint64_t sum, bool swap)
{
    /* Fold 64-bit sum to 32 bits */
    sum = (sum & 0xffffffff) + (sum >> 32);
    sum = (sum & 0xffffffff) + (sum >> 32);
    Assert(sum == (uint32_t) sum);

    /* Fold 32-bit sum to 16 bits */
    sum = (sum & 0xffff) + (sum >> 16);
    sum = (sum & 0xffff) + (sum >> 16);
    Assert(sum == (uint16_t) sum);

    if (unlikely(swap)) /* Odd base pointer is unexpected */
    {
	sum = bswap16(sum);
    }

    return (uint16_t) sum;
}

#endif
