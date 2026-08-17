// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// IMPORTANT NOTE: deprecated. Use std::atomic instead.
//
// Rationale:
// - Uniformity: most of the code uses std::atomic, and the underlying
//   implementation is the same. Use the STL one.
// - Clearer code: return values from some operations (e.g. CompareAndSwap)
//   differ from the equivalent ones in std::atomic, leading to confusion.
// - Richer semantics: can use actual types, rather than e.g. Atomic32 for a
//   boolean flag, or AtomicWord for T*. Bitwise operations (e.g. fetch_or())
//   are only in std::atomic.
// - Harder to misuse: base::subtle::Atomic32 is just an int, making it possible
//   to accidentally manipulate, not realizing that there are no atomic
//   semantics attached to it. For instance, "Atomic32 a; a++;" is almost
//   certainly incorrect.

// For atomic operations on reference counts, see atomic_refcount.h.
// For atomic operations on sequence numbers, see atomic_sequence_num.h.

// The routines exported by this module are subtle.  If you use them, even if
// you get the code right, it will depend on careful reasoning about atomicity
// and memory ordering; it will be less readable, and harder to maintain.  If
// you plan to use these routines, you should have a good reason, such as solid
// evidence that performance would otherwise suffer, or there being no
// alternative.  You should assume only properties explicitly guaranteed by the
// specifications in this file.  You are almost certainly _not_ writing code
// just for the x86; if you assume x86 semantics, x86 hardware bugs and
// implementations on other archtectures will cause your code to break.  If you
// do not know what you are doing, avoid these routines, and use a Mutex.
//
// It is incorrect to make direct assignments to/from an atomic variable.
// You should use one of the Load or Store routines.  The NoBarrier
// versions are provided when no barriers are needed:
//   NoBarrier_Store()
//   NoBarrier_Load()
// Although there are currently no compiler enforcement, you are encouraged
// to use these.
//

#ifndef BASE_ATOMICOPS_H_
#define BASE_ATOMICOPS_H_

#include <stdint.h>

// Small C++ header which defines implementation specific macros used to
// identify the STL implementation.
// - libc++: captures __config for _LIBCPP_VERSION
// - libstdc++: captures bits/c++config.h for __GLIBCXX__
#include <cstddef>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/span.h"
#include "build/build_config.h"

namespace base {
namespace subtle {

typedef int32_t Atomic32;
#ifdef ARCH_CPU_64_BITS
// We need to be able to go between Atomic64 and AtomicWord implicitly.  This
// means Atomic64 and AtomicWord should be the same type on 64-bit.
#if defined(__ILP32__) || BUILDFLAG(IS_NACL)
// NaCl's intptr_t is not actually 64-bits on 64-bit!
// http://code.google.com/p/nativeclient/issues/detail?id=1162
typedef int64_t Atomic64;
#else
typedef intptr_t Atomic64;
#endif
#endif

// Use AtomicWord for a machine-sized pointer.  It will use the Atomic32 or
// Atomic64 routines below, depending on your architecture.
typedef intptr_t AtomicWord;

// Atomically execute:
//      result = *ptr;
//      if (*ptr == old_value)
//        *ptr = new_value;
//      return result;
//
// I.e., replace "*ptr" with "new_value" if "*ptr" used to be "old_value".
// Always return the old value of "*ptr"
//
// This routine implies no memory barriers.
Atomic32 NoBarrier_CompareAndSwap(volatile Atomic32* ptr,
                                  Atomic32 old_value,
                                  Atomic32 new_value);

// Atomically store new_value into *ptr, returning the previous value held in
// *ptr.  This routine implies no memory barriers.
Atomic32 NoBarrier_AtomicExchange(volatile Atomic32* ptr, Atomic32 new_value);

// Atomically increment *ptr by "increment".  Returns the new value of
// *ptr with the increment applied.  This routine implies no memory barriers.
Atomic32 NoBarrier_AtomicIncrement(volatile Atomic32* ptr, Atomic32 increment);

Atomic32 Barrier_AtomicIncrement(volatile Atomic32* ptr, Atomic32 increment);

// These following lower-level operations are typically useful only to people
// implementing higher-level synchronization operations like spinlocks,
// mutexes, and condition-variables.  They combine CompareAndSwap(), a load, or
// a store with appropriate memory-ordering instructions.  "Acquire" operations
// ensure that no later memory access can be reordered ahead of the operation.
// "Release" operations ensure that no previous memory access can be reordered
// after the operation.  "Barrier" operations have both "Acquire" and "Release"
// semantics.
Atomic32 Acquire_CompareAndSwap(volatile Atomic32* ptr,
                                Atomic32 old_value,
                                Atomic32 new_value);
Atomic32 Release_CompareAndSwap(volatile Atomic32* ptr,
                                Atomic32 old_value,
                                Atomic32 new_value);

void NoBarrier_Store(volatile Atomic32* ptr, Atomic32 value);
void Release_Store(volatile Atomic32* ptr, Atomic32 value);

Atomic32 NoBarrier_Load(volatile const Atomic32* ptr);
Atomic32 Acquire_Load(volatile const Atomic32* ptr);

// 64-bit atomic operations (only available on 64-bit processors).
#ifdef ARCH_CPU_64_BITS
Atomic64 NoBarrier_CompareAndSwap(volatile Atomic64* ptr,
                                  Atomic64 old_value,
                                  Atomic64 new_value);
Atomic64 NoBarrier_AtomicExchange(volatile Atomic64* ptr, Atomic64 new_value);
Atomic64 NoBarrier_AtomicIncrement(volatile Atomic64* ptr, Atomic64 increment);
Atomic64 Barrier_AtomicIncrement(volatile Atomic64* ptr, Atomic64 increment);

Atomic64 Acquire_CompareAndSwap(volatile Atomic64* ptr,
                                Atomic64 old_value,
                                Atomic64 new_value);
Atomic64 Release_CompareAndSwap(volatile Atomic64* ptr,
                                Atomic64 old_value,
                                Atomic64 new_value);
void Release_Store(volatile Atomic64* ptr, Atomic64 value);
Atomic64 NoBarrier_Load(volatile const Atomic64* ptr);
Atomic64 Acquire_Load(volatile const Atomic64* ptr);
#endif  // ARCH_CPU_64_BITS

// Copies non-overlapping spans of the same size. Writes are done using C++
// atomics with `std::memory_order_relaxed`.
//
// This is an analogue of `WTF::AtomicWriteMemcpy` and it should be used
// for copying data into buffers that are accessible from another
// thread while the copy is being done. The buffer will appear inconsistent,
// but it won't trigger C++ UB and won't upset TSAN. The end of copy needs to
// be signaled through a synchronization mechanism like fence, after
// which the `dst` buffer will be observed as consistent.
//
// Notable example is a buffer owned by `SharedArrayBuffer`.
// While the copy is being done, JS and WASM code can access the `dst` buffer
// on a different thread. The data observed by JS may not be consistent
// from application point of view (which is always the case with
// `SharedArrayBuffer`).
//
// Reads from the `src` buffer are not atomic and `src` access
// should be synchronized via other means.
// More info: crbug.com/340606792
BASE_EXPORT void RelaxedAtomicWriteMemcpy(base::span<uint8_t> dst,
                                          base::span<const uint8_t> src);

}  // namespace subtle
}  // namespace base

#include "base/atomicops_internals_portable.h"

// On some platforms we need additional declarations to make
// AtomicWord compatible with our other Atomic* types.
#if BUILDFLAG(IS_APPLE) || BUILDFLAG(IS_OPENBSD)
#include "base/atomicops_internals_atomicword_compat.h"
#endif

#endif  // BASE_ATOMICOPS_H_
