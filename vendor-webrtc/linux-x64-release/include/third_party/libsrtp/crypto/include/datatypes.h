/*
 * datatypes.h
 *
 * data types for bit vectors and finite fields
 *
 * David A. McGrew
 * Cisco Systems, Inc.
 */

/*
 *
 * Copyright (c) 2001-2017, Cisco Systems, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 *
 *   Redistributions in binary form must reproduce the above
 *   copyright notice, this list of conditions and the following
 *   disclaimer in the documentation and/or other materials provided
 *   with the distribution.
 *
 *   Neither the name of the Cisco Systems, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
 * INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef DATATYPES_H
#define DATATYPES_H

#include "integers.h" /* definitions of uint32_t, et cetera   */
#include "alloc.h"

#include <stdarg.h>

#include <stdio.h>
#include <string.h>
#include <time.h>
#ifdef HAVE_NETINET_IN_H
#include <netinet/in.h>
#elif defined HAVE_WINSOCK2_H
#include <winsock2.h>
#else
#error "Platform not recognized"
#endif

#if defined(__SSE2__)
#include <emmintrin.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef union {
    uint8_t v8[16];
    uint16_t v16[8];
    uint32_t v32[4];
    uint64_t v64[2];
} v128_t;

#define MAX_PRINT_STRING_LEN 1024

char *srtp_octet_string_hex_string(const void *str, int length);

char *v128_bit_string(v128_t *x);

char *v128_hex_string(v128_t *x);

void v128_copy_octet_string(v128_t *x, const uint8_t s[16]);

void v128_left_shift(v128_t *x, int shift_index);

/*
 * the following macros define the data manipulation functions
 *
 */

#if defined(__SSE2__)

#define v128_set_to_zero(x)                                                    \
    (_mm_storeu_si128((__m128i *)(x), _mm_setzero_si128()))

#define v128_copy(x, y)                                                        \
    (_mm_storeu_si128((__m128i *)(x), _mm_loadu_si128((const __m128i *)(y))))

#define v128_xor(z, x, y)                                                      \
    (_mm_storeu_si128((__m128i *)(z),                                          \
                      _mm_xor_si128(_mm_loadu_si128((const __m128i *)(x)),     \
                                    _mm_loadu_si128((const __m128i *)(y)))))

#define v128_xor_eq(z, x)                                                      \
    (_mm_storeu_si128((__m128i *)(z),                                          \
                      _mm_xor_si128(_mm_loadu_si128((const __m128i *)(x)),     \
                                    _mm_loadu_si128((const __m128i *)(z)))))

#else /* defined(__SSE2__) */

#define v128_set_to_zero(x)                                                    \
    ((x)->v32[0] = 0, (x)->v32[1] = 0, (x)->v32[2] = 0, (x)->v32[3] = 0)

#define v128_copy(x, y)                                                        \
    ((x)->v32[0] = (y)->v32[0], (x)->v32[1] = (y)->v32[1],                     \
     (x)->v32[2] = (y)->v32[2], (x)->v32[3] = (y)->v32[3])

#define v128_xor(z, x, y)                                                      \
    ((z)->v32[0] = (x)->v32[0] ^ (y)->v32[0],                                  \
     (z)->v32[1] = (x)->v32[1] ^ (y)->v32[1],                                  \
     (z)->v32[2] = (x)->v32[2] ^ (y)->v32[2],                                  \
     (z)->v32[3] = (x)->v32[3] ^ (y)->v32[3])

/* ok for NO_64BIT_MATH if it can compare uint64_t's (even as structures) */
#ifdef NO_64BIT_MATH
#define v128_xor_eq(z, x)                                                      \
    ((z)->v32[0] ^= (x)->v32[0], (z)->v32[1] ^= (x)->v32[1],                   \
     (z)->v32[2] ^= (x)->v32[2], (z)->v32[3] ^= (x)->v32[3])
#else
#define v128_xor_eq(z, x)                                                      \
    ((z)->v64[0] ^= (x)->v64[0], (z)->v64[1] ^= (x)->v64[1])
#endif

#endif /* defined(__SSE2__) */

/* NOTE!  This assumes an odd ordering! */
/* This will not be compatible directly with math on some processors */
/* bit 0 is first 32-bit word, low order bit. in little-endian, that's
   the first byte of the first 32-bit word.  In big-endian, that's
   the 3rd byte of the first 32-bit word */
/* The get/set bit code is used by the replay code ONLY, and it doesn't
   really care which bit is which.  AES does care which bit is which, but
   doesn't use the 128-bit get/set or 128-bit shifts  */

#define v128_get_bit(x, bit) (((((x)->v32[(bit) >> 5]) >> ((bit)&31)) & 1))

#define v128_set_bit(x, bit)                                                   \
    ((((x)->v32[(bit) >> 5]) |= ((uint32_t)1 << ((bit)&31))))

#define v128_clear_bit(x, bit)                                                 \
    ((((x)->v32[(bit) >> 5]) &= ~((uint32_t)1 << ((bit)&31))))

/*
 * srtp_octet_string_is_eq(a, b, len) returns 1 if the length len strings
 * a and b are not equal. It returns 0 otherwise. The running time of the
 * comparison depends only on len, making this safe to use for (e.g.)
 * verifying authentication tags.
 */

int srtp_octet_string_is_eq(const uint8_t *a, const uint8_t *b, int len);

/*
 * A portable way to zero out memory as recommended by
 * https://cryptocoding.net/index.php/Coding_rules#Clean_memory_of_secret_data
 * This is used to zero memory when OPENSSL_cleanse() is not available.
 */
void srtp_cleanse(void *s, size_t len);

/*
 * Functions as a wrapper that delegates to either srtp_cleanse() or
 * OPENSSL_cleanse() if available to zero memory.
 */
void octet_string_set_to_zero(void *s, size_t len);

#if defined(HAVE_CONFIG_H)

/*
 * Convert big endian integers to CPU byte order.
 */
#ifdef WORDS_BIGENDIAN
/* Nothing to do. */
#define be32_to_cpu(x) (x)
#define be64_to_cpu(x) (x)
#elif defined(HAVE_BYTESWAP_H)
/* We have (hopefully) optimized versions in byteswap.h */
#include <byteswap.h>
#define be32_to_cpu(x) bswap_32((x))
#define be64_to_cpu(x) bswap_64((x))
#elif defined(__APPLE__)
// Mac OS X / Darwin features
#include <libkern/OSByteOrder.h>
#define be32_to_cpu(x) OSSwapInt32(x)
#define be64_to_cpu(x) OSSwapInt64(x)
#else /* WORDS_BIGENDIAN */

#if defined(__GNUC__)
/* Fall back. */
static inline uint32_t be32_to_cpu(uint32_t v)
{
    return __builtin_bswap32(v);
}
#else /* HAVE_X86 */
#ifdef HAVE_NETINET_IN_H
#include <netinet/in.h>
#elif defined HAVE_WINSOCK2_H
#include <winsock2.h>
#endif /* HAVE_NETINET_IN_H */
#define be32_to_cpu(x) ntohl((x))
#endif /* HAVE_X86 */

static inline uint64_t be64_to_cpu(uint64_t v)
{
#if defined(__GNUC__)
    v = __builtin_bswap64(v);
#elif defined(NO_64BIT_MATH)
    /* use the make64 functions to do 64-bit math */
    v = make64(htonl(low32(v)), htonl(high32(v)));
#else  /* NO_64BIT_MATH */
    /* use the native 64-bit math */
    v = (uint64_t)((be32_to_cpu((uint32_t)(v >> 32))) |
                   (((uint64_t)be32_to_cpu((uint32_t)v)) << 32));
#endif /* NO_64BIT_MATH */
    return v;
}

#endif /* WORDS_BIGENDIAN */

#endif /* HAVE_CONFIG_H */

/*
 * functions manipulating bitvector_t
 *
 * A bitvector_t consists of an array of words and an integer
 * representing the number of significant bits stored in the array.
 * The bits are packed as follows: the least significant bit is that
 * of word[0], while the most significant bit is the nth most
 * significant bit of word[m], where length = bits_per_word * m + n.
 *
 */

#define bits_per_word 32
#define bytes_per_word 4

typedef struct {
    uint32_t length;
    uint32_t *word;
} bitvector_t;

#define bitvector_get_bit(v, bit_index)                                        \
    (((((v)->word[((bit_index) >> 5)]) >> ((bit_index)&31)) & 1))

#define bitvector_set_bit(v, bit_index)                                        \
    ((((v)->word[((bit_index) >> 5)] |= ((uint32_t)1 << ((bit_index)&31)))))

#define bitvector_get_length(v) (((v)->length))

int bitvector_alloc(bitvector_t *v, unsigned long length);

void bitvector_dealloc(bitvector_t *v);

void bitvector_set_to_zero(bitvector_t *x);

void bitvector_left_shift(bitvector_t *x, int index);

#ifdef __cplusplus
}
#endif

#endif /* DATATYPES_H */
