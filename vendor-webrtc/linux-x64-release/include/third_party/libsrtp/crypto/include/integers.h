/*
 * integers.h
 *
 * defines integer types (or refers to their definitions)
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

#ifndef INTEGERS_H
#define INTEGERS_H

/* use standard integer definitions, if they're available  */
#ifdef HAVE_STDLIB_H
#include <stdlib.h>
#endif
#ifdef HAVE_STDINT_H
#include <stdint.h>
#endif
#ifdef HAVE_INTTYPES_H
#include <inttypes.h>
#endif
#ifdef HAVE_SYS_TYPES_H
#include <sys/types.h>
#endif
#ifdef HAVE_SYS_INT_TYPES_H
#include <sys/int_types.h> /* this exists on Sun OS */
#endif
#ifdef HAVE_MACHINE_TYPES_H
#include <machine/types.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif

/* Can we do 64 bit integers? */
#if !defined(HAVE_UINT64_T)
#if SIZEOF_UNSIGNED_LONG == 8
typedef unsigned long uint64_t;
#elif SIZEOF_UNSIGNED_LONG_LONG == 8
typedef unsigned long long uint64_t;
#else
#define NO_64BIT_MATH 1
#endif
#endif

/* Reasonable defaults for 32 bit machines - you may need to
 * edit these definitions for your own machine. */
#ifndef HAVE_UINT8_T
typedef unsigned char uint8_t;
#endif
#ifndef HAVE_UINT16_T
typedef unsigned short int uint16_t;
#endif
#ifndef HAVE_UINT32_T
typedef unsigned int uint32_t;
#endif
#ifndef HAVE_INT32_T
typedef int int32_t;
#endif

#if defined(NO_64BIT_MATH) && defined(HAVE_CONFIG_H)
typedef double uint64_t;
/* assert that sizeof(double) == 8 */
extern uint64_t make64(uint32_t high, uint32_t low);
extern uint32_t high32(uint64_t value);
extern uint32_t low32(uint64_t value);
#endif

/* These macros are to load and store 32-bit values from un-aligned
   addresses.  This is required for processors that do not allow unaligned
   loads. */
#ifdef ALIGNMENT_32BIT_REQUIRED
/* Note that if it's in a variable, you can memcpy it */
#ifdef WORDS_BIGENDIAN
#define PUT_32(addr, value)                                                    \
    {                                                                          \
        ((unsigned char *)(addr))[0] = (value >> 24);                          \
        ((unsigned char *)(addr))[1] = (value >> 16) & 0xff;                   \
        ((unsigned char *)(addr))[2] = (value >> 8) & 0xff;                    \
        ((unsigned char *)(addr))[3] = (value)&0xff;                           \
    }
#define GET_32(addr)                                                           \
    ((((unsigned char *)(addr))[0] << 24) |                                    \
     (((unsigned char *)(addr))[1] << 16) |                                    \
     (((unsigned char *)(addr))[2] << 8) | (((unsigned char *)(addr))[3]))
#else
#define PUT_32(addr, value)                                                    \
    {                                                                          \
        ((unsigned char *)(addr))[3] = (value >> 24);                          \
        ((unsigned char *)(addr))[2] = (value >> 16) & 0xff;                   \
        ((unsigned char *)(addr))[1] = (value >> 8) & 0xff;                    \
        ((unsigned char *)(addr))[0] = (value)&0xff;                           \
    }
#define GET_32(addr)                                                           \
    ((((unsigned char *)(addr))[3] << 24) |                                    \
     (((unsigned char *)(addr))[2] << 16) |                                    \
     (((unsigned char *)(addr))[1] << 8) | (((unsigned char *)(addr))[0]))
#endif // WORDS_BIGENDIAN
#else
#define PUT_32(addr, value) *(((uint32_t *) (addr)) = (value)
#define GET_32(addr) (*(((uint32_t *) (addr)))
#endif

#ifdef __cplusplus
}
#endif

#endif /* INTEGERS_H */
