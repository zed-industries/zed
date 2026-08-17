/*
* Copyright © 2018, VideoLAN and dav1d authors
* All rights reserved.
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice, this
*    list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
*    this list of conditions and the following disclaimer in the documentation
*    and/or other materials provided with the distribution.
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
* ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
* WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
* ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
* (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
* LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
* ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
* (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
* SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

#ifndef MSCVER_STDATOMIC_H_
#define MSCVER_STDATOMIC_H_

#if !defined(__cplusplus) && defined(_MSC_VER)

#pragma warning(push)
#pragma warning(disable:4067)    /* newline for __has_include_next */

#if defined(__clang__) && __has_include_next(<stdatomic.h>)
   /* use the clang stdatomic.h with clang-cl*/
#  include_next <stdatomic.h>
#else /* ! stdatomic.h */

#include <windows.h>

#include "common/attributes.h"

typedef volatile LONG  atomic_int;
typedef volatile ULONG atomic_uint;

typedef enum {
    memory_order_relaxed,
    memory_order_acquire
} msvc_atomic_memory_order;

#define atomic_init(p_a, v)           do { *(p_a) = (v); } while(0)
#define atomic_store(p_a, v)          InterlockedExchange((LONG*)p_a, v)
#define atomic_load(p_a)              InterlockedCompareExchange((LONG*)p_a, 0, 0)
#define atomic_exchange(p_a, v)       InterlockedExchange(p_a, v)
#define atomic_load_explicit(p_a, mo) atomic_load(p_a)

static inline int atomic_compare_exchange_strong_int(LONG *obj, LONG *expected,
                                                     LONG desired)
{
    LONG orig = *expected;
    *expected = InterlockedCompareExchange(obj, desired, orig);
    return *expected == orig;
}
#define atomic_compare_exchange_strong(p_a, expected, desired) atomic_compare_exchange_strong_int((LONG *)p_a, (LONG *)expected, (LONG)desired)

/*
 * TODO use a special call to increment/decrement
 * using InterlockedIncrement/InterlockedDecrement
 */
#define atomic_fetch_add(p_a, inc)    InterlockedExchangeAdd(p_a, inc)
#define atomic_fetch_sub(p_a, dec)    InterlockedExchangeAdd(p_a, -(dec))
#define atomic_fetch_or(p_a, v)       InterlockedOr(p_a, v)
#define atomic_fetch_add_explicit(p_a, inc, mo) atomic_fetch_add(p_a, inc)

#endif /* ! stdatomic.h */

#pragma warning(pop)

#endif /* !defined(__cplusplus) && defined(_MSC_VER) */

#endif /* MSCVER_STDATOMIC_H_ */
