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

#ifndef GCCVER_STDATOMIC_H_
#define GCCVER_STDATOMIC_H_

#if !defined(__cplusplus)

typedef int atomic_int;
typedef unsigned int atomic_uint;

#define memory_order_relaxed __ATOMIC_RELAXED
#define memory_order_acquire __ATOMIC_ACQUIRE

#define atomic_init(p_a, v)           do { *(p_a) = (v); } while(0)
#define atomic_store(p_a, v)          __atomic_store_n(p_a, v, __ATOMIC_SEQ_CST)
#define atomic_load(p_a)              __atomic_load_n(p_a, __ATOMIC_SEQ_CST)
#define atomic_load_explicit(p_a, mo) __atomic_load_n(p_a, mo)
#define atomic_fetch_add(p_a, inc)    __atomic_fetch_add(p_a, inc, __ATOMIC_SEQ_CST)
#define atomic_fetch_add_explicit(p_a, inc, mo) __atomic_fetch_add(p_a, inc, mo)
#define atomic_fetch_sub(p_a, dec)    __atomic_fetch_sub(p_a, dec, __ATOMIC_SEQ_CST)
#define atomic_exchange(p_a, v)       __atomic_exchange_n(p_a, v, __ATOMIC_SEQ_CST)
#define atomic_fetch_or(p_a, v)       __atomic_fetch_or(p_a, v, __ATOMIC_SEQ_CST)
#define atomic_compare_exchange_strong(p_a, expected, desired) __atomic_compare_exchange_n(p_a, expected, desired, 0, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST)

#endif /* !defined(__cplusplus) */

#endif /* GCCVER_STDATOMIC_H_ */
