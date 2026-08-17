// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_INTERNAL_H

#include <openssl/crypto.h>
#include <openssl/ex_data.h>
#include <openssl/stack.h>
#include <openssl/thread.h>

#include <assert.h>
#include <stdlib.h>
#include <string.h>

#if defined(BORINGSSL_CONSTANT_TIME_VALIDATION)
#include <valgrind/memcheck.h>
#endif

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
#include <stdlib.h>
#endif

#if defined(OPENSSL_THREADS) && \
    (!defined(OPENSSL_WINDOWS) || defined(__MINGW32__))
#include <pthread.h>
#define OPENSSL_PTHREADS
#endif

#if defined(OPENSSL_THREADS) && !defined(OPENSSL_PTHREADS) && \
    defined(OPENSSL_WINDOWS)
#define OPENSSL_WINDOWS_THREADS
#endif

#if defined(OPENSSL_THREADS)
#include <atomic>
#endif

#if defined(OPENSSL_WINDOWS_THREADS)
#include <windows.h>
#endif

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_STATIC_ARMCAP) && \
    (defined(OPENSSL_X86) || defined(OPENSSL_X86_64) ||            \
     defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64))
// x86, x86_64, and the ARMs need to record the result of a cpuid/getauxval call
// for the asm to work correctly, unless compiled without asm code.
#define NEED_CPUID

// OPENSSL_cpuid_setup initializes the platform-specific feature cache. This
// function should not be called directly. Call |OPENSSL_init_cpuid| instead.
void OPENSSL_cpuid_setup(void);

// OPENSSL_init_cpuid initializes the platform-specific feature cache, if
// needed. This function is idempotent and may be called concurrently.
void OPENSSL_init_cpuid(void);
#else
inline void OPENSSL_init_cpuid(void) {}
#endif

#if (defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)) && \
    !defined(OPENSSL_STATIC_ARMCAP)
// OPENSSL_get_armcap_pointer_for_test returns a pointer to |OPENSSL_armcap_P|
// for unit tests. Any modifications to the value must be made before any other
// function call in BoringSSL.
OPENSSL_EXPORT uint32_t *OPENSSL_get_armcap_pointer_for_test(void);
#endif


// On non-MSVC 64-bit targets, we expect __uint128_t support. This includes
// clang-cl, which defines both __clang__ and _MSC_VER.
#if (!defined(_MSC_VER) || defined(__clang__)) && defined(OPENSSL_64_BIT)
#define BORINGSSL_HAS_UINT128
typedef __int128_t int128_t;
typedef __uint128_t uint128_t;

// __uint128_t division depends on intrinsics in the compiler runtime. Those
// intrinsics are missing in clang-cl (https://crbug.com/787617) and nanolibc.
// These may be bugs in the toolchain definition, but just disable it for now.
// EDK2's toolchain is missing __udivti3 (b/339380897) so cannot support
// 128-bit division currently.
#if !defined(_MSC_VER) && !defined(OPENSSL_NANOLIBC) && \
    !defined(__EDK2_BORINGSSL__)
#define BORINGSSL_CAN_DIVIDE_UINT128
#endif
#endif

#define OPENSSL_ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))

// GCC-like compilers indicate SSE2 with |__SSE2__|. MSVC leaves the caller to
// know that x86_64 has SSE2, and uses _M_IX86_FP to indicate SSE2 on x86.
// https://learn.microsoft.com/en-us/cpp/preprocessor/predefined-macros?view=msvc-170
#if defined(__SSE2__) || defined(_M_AMD64) || defined(_M_X64) || \
    (defined(_M_IX86_FP) && _M_IX86_FP >= 2)
#define OPENSSL_SSE2
#endif

#if defined(OPENSSL_X86) && !defined(OPENSSL_NO_ASM) && !defined(OPENSSL_SSE2)
#error \
    "x86 assembly requires SSE2. Build with -msse2 (recommended), or disable assembly optimizations with -DOPENSSL_NO_ASM."
#endif

// For convenience in testing the fallback code, we allow disabling SSE2
// intrinsics via |OPENSSL_NO_SSE2_FOR_TESTING|. We require SSE2 on x86 and
// x86_64, so we would otherwise need to test such code on a non-x86 platform.
//
// This does not remove the above requirement for SSE2 support with assembly
// optimizations. It only disables some intrinsics-based optimizations so that
// we can test the fallback code on CI.
#if defined(OPENSSL_SSE2) && defined(OPENSSL_NO_SSE2_FOR_TESTING)
#undef OPENSSL_SSE2
#endif

#if defined(__GNUC__) || defined(__clang__)
#define OPENSSL_ATTR_CONST __attribute__((const))
#else
#define OPENSSL_ATTR_CONST
#endif

#if defined(BORINGSSL_MALLOC_FAILURE_TESTING)
// OPENSSL_reset_malloc_counter_for_testing, when malloc testing is enabled,
// resets the internal malloc counter, to simulate further malloc failures. This
// should be called in between independent tests, at a point where failure from
// a previous test will not impact subsequent ones.
OPENSSL_EXPORT void OPENSSL_reset_malloc_counter_for_testing(void);

// OPENSSL_disable_malloc_failures_for_testing, when malloc testing is enabled,
// disables simulated malloc failures. Calls to |OPENSSL_malloc| will not
// increment the malloc counter or synthesize failures. This may be used to skip
// simulating malloc failures in some region of code.
OPENSSL_EXPORT void OPENSSL_disable_malloc_failures_for_testing(void);

// OPENSSL_enable_malloc_failures_for_testing, when malloc testing is enabled,
// re-enables simulated malloc failures.
OPENSSL_EXPORT void OPENSSL_enable_malloc_failures_for_testing(void);
#else
inline void OPENSSL_reset_malloc_counter_for_testing(void) {}
inline void OPENSSL_disable_malloc_failures_for_testing(void) {}
inline void OPENSSL_enable_malloc_failures_for_testing(void) {}
#endif

#if defined(__has_builtin)
#define OPENSSL_HAS_BUILTIN(x) __has_builtin(x)
#else
#define OPENSSL_HAS_BUILTIN(x) 0
#endif


// Pointer utility functions.

// buffers_alias returns one if |a| and |b| alias and zero otherwise.
static inline int buffers_alias(const void *a, size_t a_bytes, const void *b,
                                size_t b_bytes) {
  // Cast |a| and |b| to integers. In C, pointer comparisons between unrelated
  // objects are undefined whereas pointer to integer conversions are merely
  // implementation-defined. We assume the implementation defined it in a sane
  // way.
  uintptr_t a_u = (uintptr_t)a;
  uintptr_t b_u = (uintptr_t)b;
  return a_u + a_bytes > b_u && b_u + b_bytes > a_u;
}

// align_pointer returns |ptr|, advanced to |alignment|. |alignment| must be a
// power of two, and |ptr| must have at least |alignment - 1| bytes of scratch
// space.
static inline void *align_pointer(void *ptr, size_t alignment) {
  // |alignment| must be a power of two.
  assert(alignment != 0 && (alignment & (alignment - 1)) == 0);
  // Instead of aligning |ptr| as a |uintptr_t| and casting back, compute the
  // offset and advance in pointer space. C guarantees that casting from pointer
  // to |uintptr_t| and back gives the same pointer, but general
  // integer-to-pointer conversions are implementation-defined. GCC does define
  // it in the useful way, but this makes fewer assumptions.
  uintptr_t offset = (0u - (uintptr_t)ptr) & (alignment - 1);
  ptr = (char *)ptr + offset;
  assert(((uintptr_t)ptr & (alignment - 1)) == 0);
  return ptr;
}


// Constant-time utility functions.
//
// The following methods return a bitmask of all ones (0xff...f) for true and 0
// for false. This is useful for choosing a value based on the result of a
// conditional in constant time. For example,
//
// if (a < b) {
//   c = a;
// } else {
//   c = b;
// }
//
// can be written as
//
// crypto_word_t lt = constant_time_lt_w(a, b);
// c = constant_time_select_w(lt, a, b);

// crypto_word_t is the type that most constant-time functions use. Ideally we
// would like it to be |size_t|, but NaCl builds in 64-bit mode with 32-bit
// pointers, which means that |size_t| can be 32 bits when |BN_ULONG| is 64
// bits. Since we want to be able to do constant-time operations on a
// |BN_ULONG|, |crypto_word_t| is defined as an unsigned value with the native
// word length.
#if defined(OPENSSL_64_BIT)
typedef uint64_t crypto_word_t;
#elif defined(OPENSSL_32_BIT)
typedef uint32_t crypto_word_t;
#else
#error "Must define either OPENSSL_32_BIT or OPENSSL_64_BIT"
#endif

#define CONSTTIME_TRUE_W ~((crypto_word_t)0)
#define CONSTTIME_FALSE_W ((crypto_word_t)0)
#define CONSTTIME_TRUE_8 ((uint8_t)0xff)
#define CONSTTIME_FALSE_8 ((uint8_t)0)

// value_barrier_w returns |a|, but prevents GCC and Clang from reasoning about
// the returned value. This is used to mitigate compilers undoing constant-time
// code, until we can express our requirements directly in the language.
//
// Note the compiler is aware that |value_barrier_w| has no side effects and
// always has the same output for a given input. This allows it to eliminate
// dead code, move computations across loops, and vectorize.
static inline crypto_word_t value_barrier_w(crypto_word_t a) {
#if defined(__GNUC__) || defined(__clang__)
  __asm__("" : "+r"(a) : /* no inputs */);
#endif
  return a;
}

// value_barrier_u32 behaves like |value_barrier_w| but takes a |uint32_t|.
static inline uint32_t value_barrier_u32(uint32_t a) {
#if defined(__GNUC__) || defined(__clang__)
  __asm__("" : "+r"(a) : /* no inputs */);
#endif
  return a;
}

// value_barrier_u64 behaves like |value_barrier_w| but takes a |uint64_t|.
static inline uint64_t value_barrier_u64(uint64_t a) {
#if defined(__GNUC__) || defined(__clang__)
  __asm__("" : "+r"(a) : /* no inputs */);
#endif
  return a;
}

// |value_barrier_u8| could be defined as above, but compilers other than
// clang seem to still materialize 0x00..00MM instead of reusing 0x??..??MM.

// constant_time_msb_w returns the given value with the MSB copied to all the
// other bits.
static inline crypto_word_t constant_time_msb_w(crypto_word_t a) {
  return 0u - (a >> (sizeof(a) * 8 - 1));
}

// constant_time_lt_w returns 0xff..f if a < b and 0 otherwise.
static inline crypto_word_t constant_time_lt_w(crypto_word_t a,
                                               crypto_word_t b) {
  // Consider the two cases of the problem:
  //   msb(a) == msb(b): a < b iff the MSB of a - b is set.
  //   msb(a) != msb(b): a < b iff the MSB of b is set.
  //
  // If msb(a) == msb(b) then the following evaluates as:
  //   msb(a^((a^b)|((a-b)^a))) ==
  //   msb(a^((a-b) ^ a))       ==   (because msb(a^b) == 0)
  //   msb(a^a^(a-b))           ==   (rearranging)
  //   msb(a-b)                      (because ∀x. x^x == 0)
  //
  // Else, if msb(a) != msb(b) then the following evaluates as:
  //   msb(a^((a^b)|((a-b)^a))) ==
  //   msb(a^(𝟙 | ((a-b)^a)))   ==   (because msb(a^b) == 1 and 𝟙
  //                                  represents a value s.t. msb(𝟙) = 1)
  //   msb(a^𝟙)                 ==   (because ORing with 1 results in 1)
  //   msb(b)
  //
  //
  // Here is an SMT-LIB verification of this formula:
  //
  // (define-fun lt ((a (_ BitVec 32)) (b (_ BitVec 32))) (_ BitVec 32)
  //   (bvxor a (bvor (bvxor a b) (bvxor (bvsub a b) a)))
  // )
  //
  // (declare-fun a () (_ BitVec 32))
  // (declare-fun b () (_ BitVec 32))
  //
  // (assert (not (= (= #x00000001 (bvlshr (lt a b) #x0000001f)) (bvult a b))))
  // (check-sat)
  // (get-model)
  return constant_time_msb_w(a ^ ((a ^ b) | ((a - b) ^ a)));
}

// constant_time_lt_8 acts like |constant_time_lt_w| but returns an 8-bit
// mask.
static inline uint8_t constant_time_lt_8(crypto_word_t a, crypto_word_t b) {
  return (uint8_t)(constant_time_lt_w(a, b));
}

// constant_time_ge_w returns 0xff..f if a >= b and 0 otherwise.
static inline crypto_word_t constant_time_ge_w(crypto_word_t a,
                                               crypto_word_t b) {
  return ~constant_time_lt_w(a, b);
}

// constant_time_ge_8 acts like |constant_time_ge_w| but returns an 8-bit
// mask.
static inline uint8_t constant_time_ge_8(crypto_word_t a, crypto_word_t b) {
  return (uint8_t)(constant_time_ge_w(a, b));
}

// constant_time_is_zero returns 0xff..f if a == 0 and 0 otherwise.
static inline crypto_word_t constant_time_is_zero_w(crypto_word_t a) {
  // Here is an SMT-LIB verification of this formula:
  //
  // (define-fun is_zero ((a (_ BitVec 32))) (_ BitVec 32)
  //   (bvand (bvnot a) (bvsub a #x00000001))
  // )
  //
  // (declare-fun a () (_ BitVec 32))
  //
  // (assert (not (= (= #x00000001 (bvlshr (is_zero a) #x0000001f)) (= a
  // #x00000000)))) (check-sat) (get-model)
  return constant_time_msb_w(~a & (a - 1));
}

// constant_time_is_zero_8 acts like |constant_time_is_zero_w| but returns an
// 8-bit mask.
static inline uint8_t constant_time_is_zero_8(crypto_word_t a) {
  return (uint8_t)(constant_time_is_zero_w(a));
}

// constant_time_eq_w returns 0xff..f if a == b and 0 otherwise.
static inline crypto_word_t constant_time_eq_w(crypto_word_t a,
                                               crypto_word_t b) {
  return constant_time_is_zero_w(a ^ b);
}

// constant_time_eq_8 acts like |constant_time_eq_w| but returns an 8-bit
// mask.
static inline uint8_t constant_time_eq_8(crypto_word_t a, crypto_word_t b) {
  return (uint8_t)(constant_time_eq_w(a, b));
}

// constant_time_eq_int acts like |constant_time_eq_w| but works on int
// values.
static inline crypto_word_t constant_time_eq_int(int a, int b) {
  return constant_time_eq_w((crypto_word_t)(a), (crypto_word_t)(b));
}

// constant_time_eq_int_8 acts like |constant_time_eq_int| but returns an 8-bit
// mask.
static inline uint8_t constant_time_eq_int_8(int a, int b) {
  return constant_time_eq_8((crypto_word_t)(a), (crypto_word_t)(b));
}

// constant_time_select_w returns (mask & a) | (~mask & b). When |mask| is all
// 1s or all 0s (as returned by the methods above), the select methods return
// either |a| (if |mask| is nonzero) or |b| (if |mask| is zero).
static inline crypto_word_t constant_time_select_w(crypto_word_t mask,
                                                   crypto_word_t a,
                                                   crypto_word_t b) {
  // Clang recognizes this pattern as a select. While it usually transforms it
  // to a cmov, it sometimes further transforms it into a branch, which we do
  // not want.
  //
  // Hiding the value of the mask from the compiler evades this transformation.
  mask = value_barrier_w(mask);
  return (mask & a) | (~mask & b);
}

// constant_time_select_8 acts like |constant_time_select| but operates on
// 8-bit values.
static inline uint8_t constant_time_select_8(crypto_word_t mask, uint8_t a,
                                             uint8_t b) {
  // |mask| is a word instead of |uint8_t| to avoid materializing 0x000..0MM
  // Making both |mask| and its value barrier |uint8_t| would allow the compiler
  // to materialize 0x????..?MM instead, but only clang is that clever.
  // However, vectorization of bitwise operations seems to work better on
  // |uint8_t| than a mix of |uint64_t| and |uint8_t|, so |m| is cast to
  // |uint8_t| after the value barrier but before the bitwise operations.
  uint8_t m = value_barrier_w(mask);
  return (m & a) | (~m & b);
}

// constant_time_select_int acts like |constant_time_select| but operates on
// ints.
static inline int constant_time_select_int(crypto_word_t mask, int a, int b) {
  return (int)(constant_time_select_w(mask, (crypto_word_t)(a),
                                      (crypto_word_t)(b)));
}

// constant_time_conditional_memcpy copies |n| bytes from |src| to |dst| if
// |mask| is 0xff..ff and does nothing if |mask| is 0. The |n|-byte memory
// ranges at |dst| and |src| must not overlap, as when calling |memcpy|.
static inline void constant_time_conditional_memcpy(void *dst, const void *src,
                                                    const size_t n,
                                                    const crypto_word_t mask) {
  assert(!buffers_alias(dst, n, src, n));
  uint8_t *out = (uint8_t *)dst;
  const uint8_t *in = (const uint8_t *)src;
  for (size_t i = 0; i < n; i++) {
    out[i] = constant_time_select_8(mask, in[i], out[i]);
  }
}

// constant_time_conditional_memxor xors |n| bytes from |src| to |dst| if
// |mask| is 0xff..ff and does nothing if |mask| is 0. The |n|-byte memory
// ranges at |dst| and |src| must not overlap, as when calling |memcpy|.
static inline void constant_time_conditional_memxor(void *dst, const void *src,
                                                    size_t n,
                                                    const crypto_word_t mask) {
  assert(!buffers_alias(dst, n, src, n));
  uint8_t *out = (uint8_t *)dst;
  const uint8_t *in = (const uint8_t *)src;
#if defined(__GNUC__) && !defined(__clang__)
  // gcc 13.2.0 doesn't automatically vectorize this loop regardless of barrier
  typedef uint8_t v32u8 __attribute__((vector_size(32), aligned(1), may_alias));
  size_t n_vec = n & ~(size_t)31;
  v32u8 masks = ((uint8_t)mask - (v32u8){});  // broadcast
  for (size_t i = 0; i < n_vec; i += 32) {
    *(v32u8 *)&out[i] ^= masks & *(v32u8 *)&in[i];
  }
  out += n_vec;
  n -= n_vec;
#endif
  for (size_t i = 0; i < n; i++) {
    out[i] ^= value_barrier_w(mask) & in[i];
  }
}

#if defined(BORINGSSL_CONSTANT_TIME_VALIDATION)

// CONSTTIME_SECRET takes a pointer and a number of bytes and marks that region
// of memory as secret. Secret data is tracked as it flows to registers and
// other parts of a memory. If secret data is used as a condition for a branch,
// or as a memory index, it will trigger warnings in valgrind.
#define CONSTTIME_SECRET(ptr, len) VALGRIND_MAKE_MEM_UNDEFINED(ptr, len)

// CONSTTIME_DECLASSIFY takes a pointer and a number of bytes and marks that
// region of memory as public. Public data is not subject to constant-time
// rules.
#define CONSTTIME_DECLASSIFY(ptr, len) VALGRIND_MAKE_MEM_DEFINED(ptr, len)

#else

#define CONSTTIME_SECRET(ptr, len)
#define CONSTTIME_DECLASSIFY(ptr, len)

#endif  // BORINGSSL_CONSTANT_TIME_VALIDATION

static inline crypto_word_t constant_time_declassify_w(crypto_word_t v) {
  // Return |v| through a value barrier to be safe. Valgrind-based constant-time
  // validation is partly to check the compiler has not undone any constant-time
  // work. Any place |BORINGSSL_CONSTANT_TIME_VALIDATION| influences
  // optimizations, this validation is inaccurate.
  //
  // However, by sending pointers through valgrind, we likely inhibit escape
  // analysis. On local variables, particularly booleans, we likely
  // significantly impact optimizations.
  //
  // Thus, to be safe, stick a value barrier, in hopes of comparably inhibiting
  // compiler analysis.
  CONSTTIME_DECLASSIFY(&v, sizeof(v));
  return value_barrier_w(v);
}

static inline int constant_time_declassify_int(int v) {
  static_assert(sizeof(uint32_t) == sizeof(int),
                "int is not the same size as uint32_t");
  // See comment above.
  CONSTTIME_DECLASSIFY(&v, sizeof(v));
  return value_barrier_u32(v);
}

// declassify_assert behaves like |assert| but declassifies the result of
// evaluating |expr|. This allows the assertion to branch on the (presumably
// public) result, but still ensures that values leading up to the computation
// were secret.
#define declassify_assert(expr) assert(constant_time_declassify_int(expr))


// Thread-safe initialisation.

#if !defined(OPENSSL_THREADS)
typedef uint32_t CRYPTO_once_t;
#define CRYPTO_ONCE_INIT 0
#elif defined(OPENSSL_WINDOWS_THREADS)
typedef INIT_ONCE CRYPTO_once_t;
#define CRYPTO_ONCE_INIT INIT_ONCE_STATIC_INIT
#elif defined(OPENSSL_PTHREADS)
typedef pthread_once_t CRYPTO_once_t;
#define CRYPTO_ONCE_INIT PTHREAD_ONCE_INIT
#else
#error "Unknown threading library"
#endif

// CRYPTO_once calls |init| exactly once per process. This is thread-safe: if
// concurrent threads call |CRYPTO_once| with the same |CRYPTO_once_t| argument
// then they will block until |init| completes, but |init| will have only been
// called once.
//
// The |once| argument must be a |CRYPTO_once_t| that has been initialised with
// the value |CRYPTO_ONCE_INIT|.
OPENSSL_EXPORT void CRYPTO_once(CRYPTO_once_t *once, void (*init)(void));


// Atomics.
//
// The following functions provide an API analogous to <stdatomic.h> from C11
// and abstract between a few variations on atomics we need to support.

#if defined(OPENSSL_THREADS)

using CRYPTO_atomic_u32 = std::atomic<uint32_t>;

static_assert(sizeof(CRYPTO_atomic_u32) == sizeof(uint32_t), "");

inline uint32_t CRYPTO_atomic_load_u32(const CRYPTO_atomic_u32 *val) {
  return val->load(std::memory_order_seq_cst);
}

inline bool CRYPTO_atomic_compare_exchange_weak_u32(CRYPTO_atomic_u32 *val,
                                                    uint32_t *expected,
                                                    uint32_t desired) {
  return val->compare_exchange_weak(
      *expected, desired, std::memory_order_seq_cst, std::memory_order_seq_cst);
}

inline void CRYPTO_atomic_store_u32(CRYPTO_atomic_u32 *val, uint32_t desired) {
  val->store(desired, std::memory_order_seq_cst);
}

#else

typedef uint32_t CRYPTO_atomic_u32;

inline uint32_t CRYPTO_atomic_load_u32(CRYPTO_atomic_u32 *val) { return *val; }

inline int CRYPTO_atomic_compare_exchange_weak_u32(CRYPTO_atomic_u32 *val,
                                                   uint32_t *expected,
                                                   uint32_t desired) {
  if (*val != *expected) {
    *expected = *val;
    return 0;
  }
  *val = desired;
  return 1;
}

inline void CRYPTO_atomic_store_u32(CRYPTO_atomic_u32 *val, uint32_t desired) {
  *val = desired;
}

#endif

// See the comment in the |__cplusplus| section above.
static_assert(sizeof(CRYPTO_atomic_u32) == sizeof(uint32_t),
              "CRYPTO_atomic_u32 does not match uint32_t size");
static_assert(alignof(CRYPTO_atomic_u32) == alignof(uint32_t),
              "CRYPTO_atomic_u32 does not match uint32_t alignment");


// Reference counting.

// CRYPTO_REFCOUNT_MAX is the value at which the reference count saturates.
#define CRYPTO_REFCOUNT_MAX 0xffffffff

// CRYPTO_refcount_inc atomically increments the value at |*count| unless the
// value would overflow. It's safe for multiple threads to concurrently call
// this or |CRYPTO_refcount_dec_and_test_zero| on the same
// |CRYPTO_refcount_t|.
OPENSSL_EXPORT void CRYPTO_refcount_inc(CRYPTO_refcount_t *count);

// CRYPTO_refcount_dec_and_test_zero tests the value at |*count|:
//   if it's zero, it crashes the address space.
//   if it's the maximum value, it returns zero.
//   otherwise, it atomically decrements it and returns one iff the resulting
//       value is zero.
//
// It's safe for multiple threads to concurrently call this or
// |CRYPTO_refcount_inc| on the same |CRYPTO_refcount_t|.
OPENSSL_EXPORT int CRYPTO_refcount_dec_and_test_zero(CRYPTO_refcount_t *count);


// Locks.

#if !defined(OPENSSL_THREADS)
typedef struct crypto_mutex_st {
  char padding;  // Empty structs have different sizes in C and C++.
} CRYPTO_MUTEX;
#define CRYPTO_MUTEX_INIT {0}
#elif defined(OPENSSL_WINDOWS_THREADS)
typedef SRWLOCK CRYPTO_MUTEX;
#define CRYPTO_MUTEX_INIT SRWLOCK_INIT
#elif defined(OPENSSL_PTHREADS)
typedef pthread_rwlock_t CRYPTO_MUTEX;
#define CRYPTO_MUTEX_INIT PTHREAD_RWLOCK_INITIALIZER
#else
#error "Unknown threading library"
#endif

// CRYPTO_MUTEX_init initialises |lock|. If |lock| is a static variable, use a
// |CRYPTO_MUTEX_INIT|.
OPENSSL_EXPORT void CRYPTO_MUTEX_init(CRYPTO_MUTEX *lock);

// CRYPTO_MUTEX_lock_read locks |lock| such that other threads may also have a
// read lock, but none may have a write lock.
OPENSSL_EXPORT void CRYPTO_MUTEX_lock_read(CRYPTO_MUTEX *lock);

// CRYPTO_MUTEX_lock_write locks |lock| such that no other thread has any type
// of lock on it.
OPENSSL_EXPORT void CRYPTO_MUTEX_lock_write(CRYPTO_MUTEX *lock);

// CRYPTO_MUTEX_unlock_read unlocks |lock| for reading.
OPENSSL_EXPORT void CRYPTO_MUTEX_unlock_read(CRYPTO_MUTEX *lock);

// CRYPTO_MUTEX_unlock_write unlocks |lock| for writing.
OPENSSL_EXPORT void CRYPTO_MUTEX_unlock_write(CRYPTO_MUTEX *lock);

// CRYPTO_MUTEX_cleanup releases all resources held by |lock|.
OPENSSL_EXPORT void CRYPTO_MUTEX_cleanup(CRYPTO_MUTEX *lock);

#if defined(__cplusplus)
extern "C++" {

BSSL_NAMESPACE_BEGIN

namespace internal {

// MutexLockBase is a RAII helper for CRYPTO_MUTEX locking.
template <void (*LockFunc)(CRYPTO_MUTEX *), void (*ReleaseFunc)(CRYPTO_MUTEX *)>
class MutexLockBase {
 public:
  explicit MutexLockBase(CRYPTO_MUTEX *mu) : mu_(mu) {
    assert(mu_ != nullptr);
    LockFunc(mu_);
  }
  ~MutexLockBase() { ReleaseFunc(mu_); }
  MutexLockBase(const MutexLockBase<LockFunc, ReleaseFunc> &) = delete;
  MutexLockBase &operator=(const MutexLockBase<LockFunc, ReleaseFunc> &) =
      delete;

 private:
  CRYPTO_MUTEX *const mu_;
};

}  // namespace internal

using MutexWriteLock =
    internal::MutexLockBase<CRYPTO_MUTEX_lock_write, CRYPTO_MUTEX_unlock_write>;
using MutexReadLock =
    internal::MutexLockBase<CRYPTO_MUTEX_lock_read, CRYPTO_MUTEX_unlock_read>;

BSSL_NAMESPACE_END

}  // extern "C++"
#endif  // defined(__cplusplus)


// Thread local storage.

// thread_local_data_t enumerates the types of thread-local data that can be
// stored.
typedef enum {
  OPENSSL_THREAD_LOCAL_ERR = 0,
  OPENSSL_THREAD_LOCAL_RAND,
  OPENSSL_THREAD_LOCAL_FIPS_COUNTERS,
  OPENSSL_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE,
  OPENSSL_THREAD_LOCAL_TEST,
  NUM_OPENSSL_THREAD_LOCALS,
} thread_local_data_t;

// thread_local_destructor_t is the type of a destructor function that will be
// called when a thread exits and its thread-local storage needs to be freed.
typedef void (*thread_local_destructor_t)(void *);

// CRYPTO_get_thread_local gets the pointer value that is stored for the
// current thread for the given index, or NULL if none has been set.
OPENSSL_EXPORT void *CRYPTO_get_thread_local(thread_local_data_t value);

// CRYPTO_set_thread_local sets a pointer value for the current thread at the
// given index. This function should only be called once per thread for a given
// |index|: rather than update the pointer value itself, update the data that
// is pointed to.
//
// The destructor function will be called when a thread exits to free this
// thread-local data. All calls to |CRYPTO_set_thread_local| with the same
// |index| should have the same |destructor| argument. The destructor may be
// called with a NULL argument if a thread that never set a thread-local
// pointer for |index|, exits. The destructor may be called concurrently with
// different arguments.
//
// This function returns one on success or zero on error. If it returns zero
// then |destructor| has been called with |value| already.
OPENSSL_EXPORT int CRYPTO_set_thread_local(
    thread_local_data_t index, void *value,
    thread_local_destructor_t destructor);


// ex_data

typedef struct crypto_ex_data_func_st CRYPTO_EX_DATA_FUNCS;

// CRYPTO_EX_DATA_CLASS tracks the ex_indices registered for a type which
// supports ex_data. It should defined as a static global within the module
// which defines that type.
typedef struct {
  CRYPTO_MUTEX lock;
  // funcs is a linked list of |CRYPTO_EX_DATA_FUNCS| structures. It may be
  // traversed without serialization only up to |num_funcs|. last points to the
  // final entry of |funcs|, or NULL if empty.
  CRYPTO_EX_DATA_FUNCS *funcs, *last;
  // num_funcs is the number of entries in |funcs|.
  CRYPTO_atomic_u32 num_funcs;
  // num_reserved is one if the ex_data index zero is reserved for legacy
  // |TYPE_get_app_data| functions.
  uint8_t num_reserved;
} CRYPTO_EX_DATA_CLASS;

#define CRYPTO_EX_DATA_CLASS_INIT {CRYPTO_MUTEX_INIT, NULL, NULL, {}, 0}
#define CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA \
  {CRYPTO_MUTEX_INIT, NULL, NULL, {}, 1}

// CRYPTO_get_ex_new_index_ex allocates a new index for |ex_data_class|. Each
// class of object should provide a wrapper function that uses the correct
// |CRYPTO_EX_DATA_CLASS|. It returns the new index on success and -1 on error.
OPENSSL_EXPORT int CRYPTO_get_ex_new_index_ex(
    CRYPTO_EX_DATA_CLASS *ex_data_class, long argl, void *argp,
    CRYPTO_EX_free *free_func);

// CRYPTO_set_ex_data sets an extra data pointer on a given object. Each class
// of object should provide a wrapper function.
OPENSSL_EXPORT int CRYPTO_set_ex_data(CRYPTO_EX_DATA *ad, int index, void *val);

// CRYPTO_get_ex_data returns an extra data pointer for a given object, or NULL
// if no such index exists. Each class of object should provide a wrapper
// function.
OPENSSL_EXPORT void *CRYPTO_get_ex_data(const CRYPTO_EX_DATA *ad, int index);

// CRYPTO_new_ex_data initialises a newly allocated |CRYPTO_EX_DATA|.
OPENSSL_EXPORT void CRYPTO_new_ex_data(CRYPTO_EX_DATA *ad);

// CRYPTO_free_ex_data frees |ad|, which is embedded inside |obj|, which is an
// object of the given class.
OPENSSL_EXPORT void CRYPTO_free_ex_data(CRYPTO_EX_DATA_CLASS *ex_data_class,
                                        void *obj, CRYPTO_EX_DATA *ad);


// Endianness conversions.

#if defined(__GNUC__) && __GNUC__ >= 2
static inline uint16_t CRYPTO_bswap2(uint16_t x) {
  return __builtin_bswap16(x);
}

static inline uint32_t CRYPTO_bswap4(uint32_t x) {
  return __builtin_bswap32(x);
}

static inline uint64_t CRYPTO_bswap8(uint64_t x) {
  return __builtin_bswap64(x);
}
#elif defined(_MSC_VER)
#pragma intrinsic(_byteswap_uint64, _byteswap_ulong, _byteswap_ushort)
static inline uint16_t CRYPTO_bswap2(uint16_t x) { return _byteswap_ushort(x); }

static inline uint32_t CRYPTO_bswap4(uint32_t x) { return _byteswap_ulong(x); }

static inline uint64_t CRYPTO_bswap8(uint64_t x) { return _byteswap_uint64(x); }
#else
static inline uint16_t CRYPTO_bswap2(uint16_t x) { return (x >> 8) | (x << 8); }

static inline uint32_t CRYPTO_bswap4(uint32_t x) {
  x = (x >> 16) | (x << 16);
  x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
  return x;
}

static inline uint64_t CRYPTO_bswap8(uint64_t x) {
  return CRYPTO_bswap4(x >> 32) | (((uint64_t)CRYPTO_bswap4(x)) << 32);
}
#endif


// Language bug workarounds.
//
// Most C standard library functions are undefined if passed NULL, even when the
// corresponding length is zero. This gives them (and, in turn, all functions
// which call them) surprising behavior on empty arrays. Some compilers will
// miscompile code due to this rule. See also
// https://www.imperialviolet.org/2016/06/26/nonnull.html
//
// These wrapper functions behave the same as the corresponding C standard
// functions, but behave as expected when passed NULL if the length is zero.
//
// Note |OPENSSL_memcmp| is a different function from |CRYPTO_memcmp|.

// C++ defines |memchr| as a const-correct overload.
#if defined(__cplusplus)
extern "C++" {

static inline const void *OPENSSL_memchr(const void *s, int c, size_t n) {
  if (n == 0) {
    return NULL;
  }

  return memchr(s, c, n);
}

static inline void *OPENSSL_memchr(void *s, int c, size_t n) {
  if (n == 0) {
    return NULL;
  }

  return memchr(s, c, n);
}

}  // extern "C++"
#else  // __cplusplus

static inline void *OPENSSL_memchr(const void *s, int c, size_t n) {
  if (n == 0) {
    return NULL;
  }

  return memchr(s, c, n);
}

#endif  // __cplusplus

static inline int OPENSSL_memcmp(const void *s1, const void *s2, size_t n) {
  if (n == 0) {
    return 0;
  }

  return memcmp(s1, s2, n);
}

static inline void *OPENSSL_memcpy(void *dst, const void *src, size_t n) {
  if (n == 0) {
    return dst;
  }

  return memcpy(dst, src, n);
}

static inline void *OPENSSL_memmove(void *dst, const void *src, size_t n) {
  if (n == 0) {
    return dst;
  }

  return memmove(dst, src, n);
}

static inline void *OPENSSL_memset(void *dst, int c, size_t n) {
  if (n == 0) {
    return dst;
  }

  return memset(dst, c, n);
}


// Loads and stores.
//
// The following functions load and store sized integers with the specified
// endianness. They use |memcpy|, and so avoid alignment or strict aliasing
// requirements on the input and output pointers.

static inline uint16_t CRYPTO_load_u16_be(const void *in) {
  uint16_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
  return CRYPTO_bswap2(v);
}

static inline void CRYPTO_store_u16_be(void *out, uint16_t v) {
  v = CRYPTO_bswap2(v);
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint32_t CRYPTO_load_u32_le(const void *in) {
  uint32_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
  return v;
}

static inline void CRYPTO_store_u32_le(void *out, uint32_t v) {
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint32_t CRYPTO_load_u32_be(const void *in) {
  uint32_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
  return CRYPTO_bswap4(v);
}

static inline void CRYPTO_store_u32_be(void *out, uint32_t v) {
  v = CRYPTO_bswap4(v);
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint64_t CRYPTO_load_u64_le(const void *in) {
  uint64_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
  return v;
}

static inline void CRYPTO_store_u64_le(void *out, uint64_t v) {
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint64_t CRYPTO_load_u64_be(const void *ptr) {
  uint64_t ret;
  OPENSSL_memcpy(&ret, ptr, sizeof(ret));
  return CRYPTO_bswap8(ret);
}

static inline void CRYPTO_store_u64_be(void *out, uint64_t v) {
  v = CRYPTO_bswap8(v);
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline crypto_word_t CRYPTO_load_word_le(const void *in) {
  crypto_word_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
  return v;
}

static inline void CRYPTO_store_word_le(void *out, crypto_word_t v) {
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline crypto_word_t CRYPTO_load_word_be(const void *in) {
  crypto_word_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_64_BIT)
  static_assert(sizeof(v) == 8, "crypto_word_t has unexpected size");
  return CRYPTO_bswap8(v);
#else
  static_assert(sizeof(v) == 4, "crypto_word_t has unexpected size");
  return CRYPTO_bswap4(v);
#endif
}


// Bit rotation functions.
//
// Note these functions use |(-shift) & 31|, etc., because shifting by the bit
// width is undefined. Both Clang and GCC recognize this pattern as a rotation,
// but MSVC does not. Instead, we call MSVC's built-in functions.

static inline uint32_t CRYPTO_rotl_u32(uint32_t value, int shift) {
#if defined(_MSC_VER)
  return _rotl(value, shift);
#else
  return (value << shift) | (value >> ((-shift) & 31));
#endif
}

static inline uint32_t CRYPTO_rotr_u32(uint32_t value, int shift) {
#if defined(_MSC_VER)
  return _rotr(value, shift);
#else
  return (value >> shift) | (value << ((-shift) & 31));
#endif
}

static inline uint64_t CRYPTO_rotl_u64(uint64_t value, int shift) {
#if defined(_MSC_VER)
  return _rotl64(value, shift);
#else
  return (value << shift) | (value >> ((-shift) & 63));
#endif
}

static inline uint64_t CRYPTO_rotr_u64(uint64_t value, int shift) {
#if defined(_MSC_VER)
  return _rotr64(value, shift);
#else
  return (value >> shift) | (value << ((-shift) & 63));
#endif
}


// FIPS functions.

#if defined(BORINGSSL_FIPS)

// BORINGSSL_FIPS_abort is called when a FIPS power-on or continuous test
// fails. It prevents any further cryptographic operations by the current
// process.
void BORINGSSL_FIPS_abort(void) __attribute__((noreturn));

// boringssl_self_test_startup runs all startup self tests and returns one on
// success or zero on error. Startup self tests do not include lazy tests.
// Call |BORINGSSL_self_test| to run every self test.
int boringssl_self_test_startup(void);

// boringssl_ensure_rsa_self_test checks whether the RSA self-test has been run
// in this address space. If not, it runs it and crashes the address space if
// unsuccessful.
void boringssl_ensure_rsa_self_test(void);

// boringssl_ensure_ecc_self_test checks whether the ECDSA and ECDH self-test
// has been run in this address space. If not, it runs it and crashes the
// address space if unsuccessful.
void boringssl_ensure_ecc_self_test(void);

// boringssl_ensure_ffdh_self_test checks whether the FFDH self-test has been
// run in this address space. If not, it runs it and crashes the address space
// if unsuccessful.
void boringssl_ensure_ffdh_self_test(void);

#else

// Outside of FIPS mode, the lazy tests are no-ops.

inline void boringssl_ensure_rsa_self_test(void) {}
inline void boringssl_ensure_ecc_self_test(void) {}
inline void boringssl_ensure_ffdh_self_test(void) {}

#endif  // FIPS

// BORINGSSL_check_test memcmp's two values of equal length. It returns 1 on
// success and, on failure, it prints an error message that includes the
// hexdumps the two values and returns 0.
int BORINGSSL_check_test(const void *expected, const void *actual,
                         size_t expected_len, const char *name);

// boringssl_self_test_sha256 performs a SHA-256 KAT.
int boringssl_self_test_sha256(void);

// boringssl_self_test_sha512 performs a SHA-512 KAT.
int boringssl_self_test_sha512(void);

// boringssl_self_test_hmac_sha256 performs an HMAC-SHA-256 KAT.
int boringssl_self_test_hmac_sha256(void);

// boringssl_self_test_mlkem performs the ML-KEM KATs.
OPENSSL_EXPORT int boringssl_self_test_mlkem(void);

// boringssl_self_test_mldsa performs the ML-DSA KATs.
OPENSSL_EXPORT int boringssl_self_test_mldsa(void);

// boringssl_self_test_slhdsa performs the SLH-DSA KATs.
OPENSSL_EXPORT int boringssl_self_test_slhdsa(void);

#if defined(BORINGSSL_FIPS_COUNTERS)
void boringssl_fips_inc_counter(enum fips_counter_t counter);
#else
inline void boringssl_fips_inc_counter(enum fips_counter_t counter) {}
#endif

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
inline int boringssl_fips_break_test(const char *test) {
  const char *const value = getenv("BORINGSSL_FIPS_BREAK_TEST");
  return value != NULL && strcmp(value, test) == 0;
}
#else
inline int boringssl_fips_break_test(const char *test) { return 0; }
#endif  // BORINGSSL_FIPS_BREAK_TESTS


// Runtime CPU feature support

#if defined(OPENSSL_X86) || defined(OPENSSL_X86_64)
// OPENSSL_ia32cap_P contains the Intel CPUID bits when running on an x86 or
// x86-64 system.
//
//   Index 0:
//     EDX for CPUID where EAX = 1
//     Bit 30 is used to indicate an Intel CPU
//   Index 1:
//     ECX for CPUID where EAX = 1
//   Index 2:
//     EBX for CPUID where EAX = 7, ECX = 0
//     Bit 14 (for removed feature MPX) is used to indicate a preference for ymm
//       registers over zmm even when zmm registers are supported
//   Index 3:
//     ECX for CPUID where EAX = 7, ECX = 0
//
// Note: the CPUID bits are pre-adjusted for the OSXSAVE bit and the XMM, YMM,
// and AVX512 bits in XCR0, so it is not necessary to check those. (WARNING: See
// caveats in cpu_intel.c.)
//
// This symbol should only be accessed with |OPENSSL_get_ia32cap|.
extern uint32_t OPENSSL_ia32cap_P[4];

// OPENSSL_get_ia32cap initializes the library if needed and returns the |idx|th
// entry of |OPENSSL_ia32cap_P|. It is marked as a const function so duplicate
// calls can be merged by the compiler, at least when indices match.
OPENSSL_ATTR_CONST uint32_t OPENSSL_get_ia32cap(int idx);

// See Intel manual, volume 2A, table 3-11.

inline int CRYPTO_is_intel_cpu(void) {
  // The reserved bit 30 is used to indicate an Intel CPU.
  return (OPENSSL_get_ia32cap(0) & (1u << 30)) != 0;
}

// See Intel manual, volume 2A, table 3-10.

inline int CRYPTO_is_PCLMUL_capable(void) {
#if defined(__PCLMUL__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 1)) != 0;
#endif
}

inline int CRYPTO_is_SSSE3_capable(void) {
#if defined(__SSSE3__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 9)) != 0;
#endif
}

inline int CRYPTO_is_SSE4_1_capable(void) {
#if defined(__SSE4_1__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 19)) != 0;
#endif
}

inline int CRYPTO_is_MOVBE_capable(void) {
#if defined(__MOVBE__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 22)) != 0;
#endif
}

inline int CRYPTO_is_AESNI_capable(void) {
#if defined(__AES__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 25)) != 0;
#endif
}

// We intentionally avoid defining a |CRYPTO_is_XSAVE_capable| function. See
// |CRYPTO_cpu_perf_is_like_silvermont|.

inline int CRYPTO_is_AVX_capable(void) {
#if defined(__AVX__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(1) & (1u << 28)) != 0;
#endif
}

inline int CRYPTO_is_RDRAND_capable(void) {
  // We intentionally do not check |__RDRND__| here. On some AMD processors, we
  // will act as if the hardware is RDRAND-incapable, even it actually supports
  // it. See cpu_intel.c.
  return (OPENSSL_get_ia32cap(1) & (1u << 30)) != 0;
}

// See Intel manual, volume 2A, table 3-8.

inline int CRYPTO_is_BMI1_capable(void) {
#if defined(__BMI__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 3)) != 0;
#endif
}

inline int CRYPTO_is_AVX2_capable(void) {
#if defined(__AVX2__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 5)) != 0;
#endif
}

inline int CRYPTO_is_BMI2_capable(void) {
#if defined(__BMI2__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 8)) != 0;
#endif
}

inline int CRYPTO_is_ADX_capable(void) {
#if defined(__ADX__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 19)) != 0;
#endif
}

// SHA-1 and SHA-256 are defined as a single extension.
inline int CRYPTO_is_x86_SHA_capable(void) {
#if defined(__SHA__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 29)) != 0;
#endif
}

// CRYPTO_cpu_perf_is_like_silvermont returns one if, based on a heuristic, the
// CPU has Silvermont-like performance characteristics. It is often faster to
// run different codepaths on these CPUs than the available instructions would
// otherwise select. See chacha-x86_64.pl.
//
// Bonnell, Silvermont's predecessor in the Atom lineup, will also be matched by
// this. Goldmont (Silvermont's successor in the Atom lineup) added XSAVE so it
// isn't matched by this. Various sources indicate AMD first implemented MOVBE
// and XSAVE at the same time in Jaguar, so it seems like AMD chips will not be
// matched by this. That seems to be the case for other x86(-64) CPUs.
inline int CRYPTO_cpu_perf_is_like_silvermont(void) {
  // WARNING: This MUST NOT be used to guard the execution of the XSAVE
  // instruction. This is the "hardware supports XSAVE" bit, not the OSXSAVE bit
  // that indicates whether we can safely execute XSAVE. This bit may be set
  // even when XSAVE is disabled (by the operating system). See how the users of
  // this bit use it.
  //
  // Historically, the XSAVE bit was artificially cleared on Knights Landing
  // and Knights Mill chips, but as Intel has removed all support from GCC,
  // LLVM, and SDE, we assume they are no longer worth special-casing.
  int hardware_supports_xsave = (OPENSSL_get_ia32cap(1) & (1u << 26)) != 0;
  return !hardware_supports_xsave && CRYPTO_is_MOVBE_capable();
}

inline int CRYPTO_is_AVX512BW_capable(void) {
#if defined(__AVX512BW__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 30)) != 0;
#endif
}

inline int CRYPTO_is_AVX512VL_capable(void) {
#if defined(__AVX512VL__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(2) & (1u << 31)) != 0;
#endif
}

// CRYPTO_cpu_avoid_zmm_registers returns 1 if zmm registers (512-bit vectors)
// should not be used even if the CPU supports them.
//
// Note that this reuses the bit for the removed MPX feature.
inline int CRYPTO_cpu_avoid_zmm_registers(void) {
  return (OPENSSL_get_ia32cap(2) & (1u << 14)) != 0;
}

inline int CRYPTO_is_VAES_capable(void) {
#if defined(__VAES__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(3) & (1u << 9)) != 0;
#endif
}

inline int CRYPTO_is_VPCLMULQDQ_capable(void) {
#if defined(__VPCLMULQDQ__)
  return 1;
#else
  return (OPENSSL_get_ia32cap(3) & (1u << 10)) != 0;
#endif
}

#endif  // OPENSSL_X86 || OPENSSL_X86_64

#if defined(OPENSSL_ARM) || defined(OPENSSL_AARCH64)

// ARMV7_NEON indicates support for NEON.
#define ARMV7_NEON (1 << 0)

// ARMV8_AES indicates support for hardware AES instructions.
#define ARMV8_AES (1 << 2)

// ARMV8_SHA1 indicates support for hardware SHA-1 instructions.
#define ARMV8_SHA1 (1 << 3)

// ARMV8_SHA256 indicates support for hardware SHA-256 instructions.
#define ARMV8_SHA256 (1 << 4)

// ARMV8_PMULL indicates support for carryless multiplication.
#define ARMV8_PMULL (1 << 5)

// ARMV8_SHA512 indicates support for hardware SHA-512 instructions.
#define ARMV8_SHA512 (1 << 6)

// OPENSSL_armcap_P contains ARM CPU capabilities as a bitmask of the above
// constants. This should only be accessed with |OPENSSL_get_armcap|.
extern uint32_t OPENSSL_armcap_P;

// OPENSSL_get_armcap initializes the library if needed and returns ARM CPU
// capabilities. It is marked as a const function so duplicate calls can be
// merged by the compiler.
OPENSSL_ATTR_CONST uint32_t OPENSSL_get_armcap(void);

// Normalize some older feature flags to their modern ACLE values.
// https://developer.arm.com/architectures/system-architectures/software-standards/acle
#if defined(__ARM_NEON__) && !defined(__ARM_NEON)
#define __ARM_NEON 1
#endif
#if defined(__ARM_FEATURE_CRYPTO)
#if !defined(__ARM_FEATURE_AES)
#define __ARM_FEATURE_AES 1
#endif
#if !defined(__ARM_FEATURE_SHA2)
#define __ARM_FEATURE_SHA2 1
#endif
#endif

// CRYPTO_is_NEON_capable returns true if the current CPU has a NEON unit. If
// this is known statically, it is a constant inline function.
inline int CRYPTO_is_NEON_capable(void) {
#if defined(OPENSSL_STATIC_ARMCAP_NEON) || defined(__ARM_NEON)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV7_NEON) != 0;
#endif
}

inline int CRYPTO_is_ARMv8_AES_capable(void) {
#if defined(OPENSSL_STATIC_ARMCAP_AES) || defined(__ARM_FEATURE_AES)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV8_AES) != 0;
#endif
}

inline int CRYPTO_is_ARMv8_PMULL_capable(void) {
#if defined(OPENSSL_STATIC_ARMCAP_PMULL) || defined(__ARM_FEATURE_AES)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV8_PMULL) != 0;
#endif
}

inline int CRYPTO_is_ARMv8_SHA1_capable(void) {
  // SHA-1 and SHA-2 (only) share |__ARM_FEATURE_SHA2| but otherwise
  // are dealt with independently.
#if defined(OPENSSL_STATIC_ARMCAP_SHA1) || defined(__ARM_FEATURE_SHA2)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV8_SHA1) != 0;
#endif
}

inline int CRYPTO_is_ARMv8_SHA256_capable(void) {
  // SHA-1 and SHA-2 (only) share |__ARM_FEATURE_SHA2| but otherwise
  // are dealt with independently.
#if defined(OPENSSL_STATIC_ARMCAP_SHA256) || defined(__ARM_FEATURE_SHA2)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV8_SHA256) != 0;
#endif
}

inline int CRYPTO_is_ARMv8_SHA512_capable(void) {
  // There is no |OPENSSL_STATIC_ARMCAP_SHA512|.
#if defined(__ARM_FEATURE_SHA512)
  return 1;
#elif defined(OPENSSL_STATIC_ARMCAP)
  return 0;
#else
  return (OPENSSL_get_armcap() & ARMV8_SHA512) != 0;
#endif
}

#endif  // OPENSSL_ARM || OPENSSL_AARCH64


#if defined(BORINGSSL_DISPATCH_TEST)
// Runtime CPU dispatch testing support

// BORINGSSL_function_hit is an array of flags. The following functions will
// set these flags if BORINGSSL_DISPATCH_TEST is defined.
//   0: aes_hw_ctr32_encrypt_blocks
//   1: aes_hw_encrypt
//   2: aesni_gcm_encrypt
//   3: aes_hw_set_encrypt_key
//   4: vpaes_encrypt
//   5: vpaes_set_encrypt_key
//   6: aes_gcm_enc_update_vaes_avx2
//   7: aes_gcm_enc_update_vaes_avx512
extern uint8_t BORINGSSL_function_hit[8];
#endif  // BORINGSSL_DISPATCH_TEST


// OPENSSL_vasprintf_internal is just like |vasprintf(3)|. If |system_malloc| is
// 0, memory will be allocated with |OPENSSL_malloc| and must be freed with
// |OPENSSL_free|. Otherwise the system |malloc| function is used and the memory
// must be freed with the system |free| function.
OPENSSL_EXPORT int OPENSSL_vasprintf_internal(char **str, const char *format,
                                              va_list args, int system_malloc)
    OPENSSL_PRINTF_FORMAT_FUNC(2, 0);


// Fuzzer mode.

#if defined(FUZZING_BUILD_MODE_UNSAFE_FOR_PRODUCTION)
// CRYPTO_fuzzer_mode_enabled returns whether fuzzer mode is enabled. See
// |CRYPTO_set_fuzzer_mode|. In non-fuzzer builds, this function statically
// returns zero so the codepaths will be deleted by the optimizer.
int CRYPTO_fuzzer_mode_enabled(void);
#else
inline int CRYPTO_fuzzer_mode_enabled(void) { return 0; }
#endif


#if defined(__cplusplus)
}  // extern C
#endif

// Arithmetic functions.

// CRYPTO_addc_* returns |x + y + carry|, and sets |*out_carry| to the carry
// bit. |carry| must be zero or one.
#if OPENSSL_HAS_BUILTIN(__builtin_addc)

inline unsigned int CRYPTO_addc_impl(unsigned int x, unsigned int y,
                                     unsigned int carry,
                                     unsigned int *out_carry) {
  return __builtin_addc(x, y, carry, out_carry);
}

inline unsigned long CRYPTO_addc_impl(unsigned long x, unsigned long y,
                                      unsigned long carry,
                                      unsigned long *out_carry) {
  return __builtin_addcl(x, y, carry, out_carry);
}

inline unsigned long long CRYPTO_addc_impl(unsigned long long x,
                                           unsigned long long y,
                                           unsigned long long carry,
                                           unsigned long long *out_carry) {
  return __builtin_addcll(x, y, carry, out_carry);
}

inline uint32_t CRYPTO_addc_u32(uint32_t x, uint32_t y, uint32_t carry,
                                uint32_t *out_carry) {
  return CRYPTO_addc_impl(x, y, carry, out_carry);
}

inline uint64_t CRYPTO_addc_u64(uint64_t x, uint64_t y, uint64_t carry,
                                uint64_t *out_carry) {
  return CRYPTO_addc_impl(x, y, carry, out_carry);
}

#else

static inline uint32_t CRYPTO_addc_u32(uint32_t x, uint32_t y, uint32_t carry,
                                       uint32_t *out_carry) {
  declassify_assert(carry <= 1);
  uint64_t ret = carry;
  ret += (uint64_t)x + y;
  *out_carry = (uint32_t)(ret >> 32);
  return (uint32_t)ret;
}

static inline uint64_t CRYPTO_addc_u64(uint64_t x, uint64_t y, uint64_t carry,
                                       uint64_t *out_carry) {
  declassify_assert(carry <= 1);
#if defined(BORINGSSL_HAS_UINT128)
  uint128_t ret = carry;
  ret += (uint128_t)x + y;
  *out_carry = (uint64_t)(ret >> 64);
  return (uint64_t)ret;
#else
  x += carry;
  carry = x < carry;
  uint64_t ret = x + y;
  carry += ret < x;
  *out_carry = carry;
  return ret;
#endif
}
#endif


// CRYPTO_subc_* returns |x - y - borrow|, and sets |*out_borrow| to the borrow
// bit. |borrow| must be zero or one.
#if OPENSSL_HAS_BUILTIN(__builtin_subc)

inline unsigned int CRYPTO_subc_impl(unsigned int x, unsigned int y,
                                     unsigned int borrow,
                                     unsigned int *out_borrow) {
  return __builtin_subc(x, y, borrow, out_borrow);
}

inline unsigned long CRYPTO_subc_impl(unsigned long x, unsigned long y,
                                      unsigned long borrow,
                                      unsigned long *out_borrow) {
  return __builtin_subcl(x, y, borrow, out_borrow);
}

inline unsigned long long CRYPTO_subc_impl(unsigned long long x,
                                           unsigned long long y,
                                           unsigned long long borrow,
                                           unsigned long long *out_borrow) {
  return __builtin_subcll(x, y, borrow, out_borrow);
}

inline uint32_t CRYPTO_subc_u32(uint32_t x, uint32_t y, uint32_t borrow,
                                uint32_t *out_borrow) {
  return CRYPTO_subc_impl(x, y, borrow, out_borrow);
}

inline uint64_t CRYPTO_subc_u64(uint64_t x, uint64_t y, uint64_t borrow,
                                uint64_t *out_borrow) {
  return CRYPTO_subc_impl(x, y, borrow, out_borrow);
}

#else

static inline uint32_t CRYPTO_subc_u32(uint32_t x, uint32_t y, uint32_t borrow,
                                       uint32_t *out_borrow) {
  declassify_assert(borrow <= 1);
  uint32_t ret = x - y - borrow;
  *out_borrow = (x < y) | ((x == y) & borrow);
  return ret;
}

static inline uint64_t CRYPTO_subc_u64(uint64_t x, uint64_t y, uint64_t borrow,
                                       uint64_t *out_borrow) {
  declassify_assert(borrow <= 1);
  uint64_t ret = x - y - borrow;
  *out_borrow = (x < y) | ((x == y) & borrow);
  return ret;
}
#endif

#if defined(OPENSSL_64_BIT)
#define CRYPTO_addc_w CRYPTO_addc_u64
#define CRYPTO_subc_w CRYPTO_subc_u64
#else
#define CRYPTO_addc_w CRYPTO_addc_u32
#define CRYPTO_subc_w CRYPTO_subc_u32
#endif


#endif  // OPENSSL_HEADER_CRYPTO_INTERNAL_H
