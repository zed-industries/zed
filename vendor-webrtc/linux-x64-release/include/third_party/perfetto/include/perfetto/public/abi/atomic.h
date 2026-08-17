/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_ATOMIC_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_ATOMIC_H_

// Problem: C++11 and C11 use a different syntax for atomics and the C11 syntax
// is not supported in C++11.
//
// This header bridges the gap.
//
// This assumes that C++11 atomics are binary compatible with C11 atomics. While
// this is technically not required by the standards, reasonable compilers
// appear to guarantee this.

#ifdef __cplusplus
#include <atomic>
#else
#include <stdatomic.h>
#endif

#ifdef __cplusplus
#define PERFETTO_ATOMIC(TYPE) std::atomic<TYPE>
#else
#define PERFETTO_ATOMIC(TYPE) _Atomic(TYPE)
#endif

#ifdef __cplusplus
#define PERFETTO_ATOMIC_LOAD std::atomic_load
#define PERFETTO_ATOMIC_LOAD_EXPLICIT std::atomic_load_explicit
#define PERFETTO_ATOMIC_STORE std::atomic_store
#define PERFETTO_ATOMIC_STORE_EXPLICIT std::atomic_store_explicit

#define PERFETTO_MEMORY_ORDER_ACQ_REL std::memory_order_acq_rel
#define PERFETTO_MEMORY_ORDER_ACQUIRE std::memory_order_acquire
#define PERFETTO_MEMORY_ORDER_CONSUME std::memory_order_consume
#define PERFETTO_MEMORY_ORDER_RELAXED std::memory_order_relaxed
#define PERFETTO_MEMORY_ORDER_RELEASE std::memory_order_release
#define PERFETTO_MEMORY_ORDER_SEQ_CST std::memory_order_seq_cst
#else
#define PERFETTO_ATOMIC_LOAD atomic_load
#define PERFETTO_ATOMIC_LOAD_EXPLICIT atomic_load_explicit
#define PERFETTO_ATOMIC_STORE atomic_store
#define PERFETTO_ATOMIC_STORE_EXPLICIT atomic_store_explicit

#define PERFETTO_MEMORY_ORDER_ACQ_REL memory_order_acq_rel
#define PERFETTO_MEMORY_ORDER_ACQUIRE memory_order_acquire
#define PERFETTO_MEMORY_ORDER_CONSUME memory_order_consume
#define PERFETTO_MEMORY_ORDER_RELAXED memory_order_relaxed
#define PERFETTO_MEMORY_ORDER_RELEASE memory_order_release
#define PERFETTO_MEMORY_ORDER_SEQ_CST memory_order_seq_cst
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_ATOMIC_H_
