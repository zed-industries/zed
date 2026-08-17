/* ----------------------------------------------------------------------- *
 *
 *   Copyright 1996-2017 The NASM Authors - All Rights Reserved
 *   See the file AUTHORS included with the NASM distribution for
 *   the specific copyright holders.
 *
 *   Redistribution and use in source and binary forms, with or without
 *   modification, are permitted provided that the following
 *   conditions are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above
 *     copyright notice, this list of conditions and the following
 *     disclaimer in the documentation and/or other materials provided
 *     with the distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND
 *     CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
 *     INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 *     MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 *     DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 *     CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 *     SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 *     NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 *     HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 *     CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 *     OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE,
 *     EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * ----------------------------------------------------------------------- */

#ifndef ILOG2_H
#define ILOG2_H

#include "compiler.h"

#ifdef ILOG2_C                  /* For generating the out-of-line functions */
# ifndef HAVE_MSVC_INLINE
#  undef extern_inline
#  define extern_inline
# endif
# define inline_prototypes
#endif

#ifdef inline_prototypes
extern unsigned int const_func ilog2_32(uint32_t v);
extern unsigned int const_func ilog2_64(uint64_t v);
extern unsigned int const_func ilog2_64(uint64_t vv);
extern int const_func alignlog2_32(uint32_t v);
extern int const_func alignlog2_64(uint64_t v);
#endif

#ifdef extern_inline

#define ROUND(v, a, w)                                  \
    do {                                                \
        if (v & (((UINT32_C(1) << w) - 1) << w)) {      \
            a  += w;                                    \
            v >>= w;                                    \
        }                                               \
    } while (0)


#if defined(HAVE___BUILTIN_CLZ) && INT_MAX == 2147483647

extern_inline unsigned int const_func ilog2_32(uint32_t v)
{
    if (!v)
        return 0;

    return __builtin_clz(v) ^ 31;
}

#elif defined(__GNUC__) && defined(__x86_64__)

extern_inline unsigned int const_func ilog2_32(uint32_t v)
{
    unsigned int n;

    __asm__("bsrl %1,%0"
            : "=r" (n)
            : "rm" (v), "0" (0));
    return n;
}

#elif defined(__GNUC__) && defined(__i386__)

extern_inline unsigned int const_func ilog2_32(uint32_t v)
{
    unsigned int n;

#ifdef __i686__
    __asm__("bsrl %1,%0 ; cmovz %2,%0\n"
            : "=&r" (n)
            : "rm" (v), "r" (0));
#else
    __asm__("bsrl %1,%0 ; jnz 1f ; xorl %0,%0\n"
            "1:"
            : "=&r" (n)
            : "rm" (v));
#endif
     return n;
}

#elif defined(HAVE__BITSCANREVERSE)

extern_inline unsigned int const_func ilog2_32(uint32_t v)
{
    unsigned long ix;
    return _BitScanReverse(&ix, v) ? v : 0;
}

#else

extern_inline unsigned int const_func ilog2_32(uint32_t v)
{
    unsigned int p = 0;

    ROUND(v, p, 16);
    ROUND(v, p,  8);
    ROUND(v, p,  4);
    ROUND(v, p,  2);
    ROUND(v, p,  1);

    return p;
}

#endif

#if defined(HAVE__BUILTIN_CLZLL) && LLONG_MAX == 9223372036854775807LL

extern_inline unsigned int const_func ilog2_64(uint64_t v)
{
    if (!v)
        return 0;

    return __builtin_clzll(v) ^ 63;
}

#elif defined(__GNUC__) && defined(__x86_64__)

extern_inline unsigned int const_func ilog2_64(uint64_t v)
{
    uint64_t n;

    __asm__("bsrq %1,%0"
            : "=r" (n)
            : "rm" (v), "0" (UINT64_C(0)));
    return n;
}

#elif defined(HAVE__BITSCANREVERSE64)

extern_inline unsigned int const_func ilog2_64(uint64_t v)
{
    unsigned long ix;
    return _BitScanReverse64(&ix, v) ? ix : 0;
}

#else

extern_inline unsigned int const_func ilog2_64(uint64_t vv)
{
    unsigned int p = 0;
    uint32_t v;

    v = vv >> 32;
    if (v)
        p += 32;
    else
        v = vv;

    return p + ilog2_32(v);
}

#endif

/*
 * v == 0 ? 0 : is_power2(x) ? ilog2_X(v) : -1
 */
extern_inline int const_func alignlog2_32(uint32_t v)
{
    if (unlikely(v & (v-1)))
        return -1;              /* invalid alignment */

    return ilog2_32(v);
}

extern_inline int const_func alignlog2_64(uint64_t v)
{
    if (unlikely(v & (v-1)))
        return -1;              /* invalid alignment */

    return ilog2_64(v);
}

#undef ROUND

#endif /* extern_inline */

#endif /* ILOG2_H */
