// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2001 The OpenSSL Project.  All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_CRYPTO_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_INTERNAL_H

#include <openssl/crypto.h>
#include <openssl/ex_data.h>
#include <openssl/service_indicator.h>
#include <openssl/stack.h>
#include <openssl/thread.h>

#include <assert.h>
#include <string.h>

#if defined(BORINGSSL_CONSTANT_TIME_VALIDATION)
#include <valgrind/memcheck.h>
#endif

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
#include <stdlib.h>
#endif

#if !defined(__cplusplus)
#if defined(_MSC_VER)
#define alignas(x) __declspec(align(x))
#define alignof __alignof
#elif !defined(AWS_LC_STDALIGN_AVAILABLE)
#define alignas(x) __attribute__ ((aligned (x)))
#define alignof(x) __alignof__ (x)
#else
#include <stdalign.h>
#endif
#endif

#if defined(OPENSSL_THREADS) && \
    (!defined(OPENSSL_WINDOWS) || \
        (defined(__MINGW32__) && !defined(__clang__)))
#include <pthread.h>
#define OPENSSL_PTHREADS
#endif

#if defined(OPENSSL_THREADS) && !defined(OPENSSL_PTHREADS) && \
    defined(OPENSSL_WINDOWS)
#define OPENSSL_WINDOWS_THREADS
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <windows.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#endif

#include <stdbool.h>

#if defined(__cplusplus)
extern "C" {
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
#if !defined(_MSC_VER) && !defined(OPENSSL_NANOLIBC)
#define BORINGSSL_CAN_DIVIDE_UINT128
#endif
#endif

#define OPENSSL_ARRAY_SIZE(array) (sizeof(array) / sizeof((array)[0]))

// Have a generic fall-through for different versions of C/C++.
#if defined(__cplusplus) && __cplusplus >= 201703L
#define OPENSSL_FALLTHROUGH [[fallthrough]]
#elif defined(__cplusplus) && __cplusplus >= 201103L && defined(__clang__)
#define OPENSSL_FALLTHROUGH [[clang::fallthrough]]
#elif defined(__cplusplus) && __cplusplus >= 201103L && defined(__GNUC__) && \
    __GNUC__ >= 7
#define OPENSSL_FALLTHROUGH [[gnu::fallthrough]]
#elif defined(__GNUC__) && __GNUC__ >= 7 // gcc 7
#define OPENSSL_FALLTHROUGH __attribute__ ((fallthrough))
#elif defined(__clang__)
#if __has_attribute(fallthrough) && __clang_major__ >= 5
// Clang 3.5, at least, complains about "error: declaration does not declare
// anything", possibily because we put a semicolon after this macro in
// practice. Thus limit it to >= Clang 5, which does work.
#define OPENSSL_FALLTHROUGH __attribute__ ((fallthrough))
#else // clang versions that do not support fallthrough.
#define OPENSSL_FALLTHROUGH
#endif
#else // C++11 on gcc 6, and all other cases
#define OPENSSL_FALLTHROUGH
#endif

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
#define OPENSSL_ATTR_PURE __attribute__((pure))
#else
#define OPENSSL_ATTR_PURE
#endif

#if defined(__has_builtin)
#define OPENSSL_HAS_BUILTIN(x) __has_builtin(x)
#else
#define OPENSSL_HAS_BUILTIN(x) 0
#endif


// Pointer utility functions.

// buffers_alias returns one if |a| and |b| alias and zero otherwise.
static inline int buffers_alias(const uint8_t *a, size_t a_len,
                                const uint8_t *b, size_t b_len) {
  // Cast |a| and |b| to integers. In C, pointer comparisons between unrelated
  // objects are undefined whereas pointer to integer conversions are merely
  // implementation-defined. We assume the implementation defined it in a sane
  // way.
  uintptr_t a_u = (uintptr_t)a;
  uintptr_t b_u = (uintptr_t)b;
  return a_u + a_len > b_u && b_u + b_len > a_u;
}

typedef uint8_t stack_align_type;
OPENSSL_STATIC_ASSERT(sizeof(stack_align_type) == 1,
                      stack_align_type_is_not_8_bits_long)

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
  return constant_time_msb_w(a^((a^b)|((a-b)^a)));
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
  // (assert (not (= (= #x00000001 (bvlshr (is_zero a) #x0000001f)) (= a #x00000000))))
  // (check-sat)
  // (get-model)
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
  // Adding barriers to both |mask| and |~mask| breaks the relationship between
  // the two, which makes the compiler stick with bitmasks.
  return (value_barrier_w(mask) & a) | (value_barrier_w(~mask) & b);
}

// constant_time_select_8 acts like |constant_time_select| but operates on
// 8-bit values.
static inline uint8_t constant_time_select_8(uint8_t mask, uint8_t a,
                                             uint8_t b) {
  return (uint8_t)(constant_time_select_w(mask, a, b));
}

// constant_time_select_int acts like |constant_time_select| but operates on
// ints.
static inline int constant_time_select_int(crypto_word_t mask, int a, int b) {
  return (int)(constant_time_select_w(mask, (crypto_word_t)(a),
                                      (crypto_word_t)(b)));
}

// constant_time_select_array_w applies |constant_time_select_w| on each
// corresponding pair of elements of a and b.
static inline void constant_time_select_array_w(
        crypto_word_t *c, crypto_word_t *a, crypto_word_t *b,
        crypto_word_t mask, size_t len) {
  for (size_t i = 0; i < len; i++) {
    c[i] = constant_time_select_w(mask, a[i], b[i]);
  }
}

static inline void constant_time_select_array_8(
        uint8_t *c, uint8_t *a, uint8_t *b, uint8_t mask, size_t len) {
  for (size_t i = 0; i < len; i++) {
    c[i] = constant_time_select_8(mask, a[i], b[i]);
  }
}

// constant_time_select_entry_from_table_w selects the idx-th entry from table.
static inline void constant_time_select_entry_from_table_w(
        crypto_word_t *out, crypto_word_t *table,
        size_t idx, size_t num_entries, size_t entry_size) {
  for (size_t i = 0; i < num_entries; i++) {
    crypto_word_t mask = constant_time_eq_w(i, idx);
    constant_time_select_array_w(out, &table[i * entry_size], out, mask, entry_size);
  }
}

static inline void constant_time_select_entry_from_table_8(
        uint8_t *out, uint8_t *table, size_t idx,
        size_t num_entries, size_t entry_size) {
  for (size_t i = 0; i < num_entries; i++) {
    uint8_t mask = (uint8_t)(constant_time_eq_w(i, idx));
    constant_time_select_array_8(out, &table[i * entry_size], out, mask, entry_size);
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
  OPENSSL_STATIC_ASSERT(sizeof(uint32_t) == sizeof(int),
                int_is_not_the_same_size_as_uint32_t);
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


// Reference counting.

#if !defined(__STDC_NO_ATOMICS__) && defined(__STDC_VERSION__) && \
    __STDC_VERSION__ >= 201112L
#include <stdatomic.h>
// CRYPTO_refcount_t is a |uint32_t|
#define AWS_LC_ATOMIC_LOCK_FREE ATOMIC_LONG_LOCK_FREE
#else
#define AWS_LC_ATOMIC_LOCK_FREE 0
#endif

// Automatically enable C11 atomics if implemented and lock free
#if !defined(OPENSSL_C11_ATOMIC) && defined(OPENSSL_THREADS) && \
    AWS_LC_ATOMIC_LOCK_FREE == 2
#define OPENSSL_C11_ATOMIC
#endif

// Older MSVC does not support C11 atomics, so we fallback to the Windows APIs.
// This can be removed once we can rely on
// https://devblogs.microsoft.com/cppblog/c11-atomics-in-visual-studio-2022-version-17-5-preview-2/
#if !defined(OPENSSL_C11_ATOMIC) && defined(OPENSSL_THREADS) && \
    defined(OPENSSL_WINDOWS)
#define OPENSSL_WINDOWS_ATOMIC
#endif

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
//
// Two types of locks are defined: |CRYPTO_MUTEX|, which can be used in
// structures as normal, and |struct CRYPTO_STATIC_MUTEX|, which can be used as
// a global lock. A global lock must be initialised to the value
// |CRYPTO_STATIC_MUTEX_INIT|.
//
// |CRYPTO_MUTEX| can appear in public structures and so is defined in
// thread.h as a structure large enough to fit the real type. The global lock is
// a different type so it may be initialized with platform initializer macros.

#if !defined(OPENSSL_THREADS)
struct CRYPTO_STATIC_MUTEX {
  char padding;  // Empty structs have different sizes in C and C++.
};
#define CRYPTO_STATIC_MUTEX_INIT { 0 }
#elif defined(OPENSSL_WINDOWS_THREADS)
struct CRYPTO_STATIC_MUTEX {
  SRWLOCK lock;
};
#define CRYPTO_STATIC_MUTEX_INIT { SRWLOCK_INIT }
#elif defined(OPENSSL_PTHREADS)
struct CRYPTO_STATIC_MUTEX {
  pthread_rwlock_t lock;
};
#define CRYPTO_STATIC_MUTEX_INIT { PTHREAD_RWLOCK_INITIALIZER }
#else
#error "Unknown threading library"
#endif

// CRYPTO_MUTEX_init initialises |lock|. If |lock| is a static variable, use a
// |CRYPTO_STATIC_MUTEX|.
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

// CRYPTO_STATIC_MUTEX_lock_read locks |lock| such that other threads may also
// have a read lock, but none may have a write lock. The |lock| variable does
// not need to be initialised by any function, but must have been statically
// initialised with |CRYPTO_STATIC_MUTEX_INIT|.
OPENSSL_EXPORT void CRYPTO_STATIC_MUTEX_lock_read(
    struct CRYPTO_STATIC_MUTEX *lock);

// CRYPTO_STATIC_MUTEX_lock_write locks |lock| such that no other thread has
// any type of lock on it.  The |lock| variable does not need to be initialised
// by any function, but must have been statically initialised with
// |CRYPTO_STATIC_MUTEX_INIT|.
OPENSSL_EXPORT void CRYPTO_STATIC_MUTEX_lock_write(
    struct CRYPTO_STATIC_MUTEX *lock);

// CRYPTO_STATIC_MUTEX_unlock_read unlocks |lock| for reading.
OPENSSL_EXPORT void CRYPTO_STATIC_MUTEX_unlock_read(
    struct CRYPTO_STATIC_MUTEX *lock);

// CRYPTO_STATIC_MUTEX_unlock_write unlocks |lock| for writing.
OPENSSL_EXPORT void CRYPTO_STATIC_MUTEX_unlock_write(
    struct CRYPTO_STATIC_MUTEX *lock);

#if !defined(NDEBUG)
// CRYPTO_STATIC_MUTEX_is_write_locked checks whether |lock| has an active write
// lock. If it does, the function returns 1. If it doesn't, it returns 0. Returns -1
// on any other error. Note that due to the concurrent nature of locks, the result
// may be stale by the time it is used.
OPENSSL_EXPORT int CRYPTO_STATIC_MUTEX_is_write_locked(
    struct CRYPTO_STATIC_MUTEX *lock);
#endif

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
  AWSLC_THREAD_LOCAL_FIPS_SERVICE_INDICATOR_STATE,
  OPENSSL_THREAD_LOCAL_TEST,
  OPENSSL_THREAD_LOCAL_PRIVATE_RAND,
  OPENSSL_THREAD_LOCAL_PUBLIC_RAND,
  OPENSSL_THREAD_LOCAL_UBE,
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

DECLARE_STACK_OF(CRYPTO_EX_DATA_FUNCS)

// CRYPTO_EX_DATA_CLASS tracks the ex_indices registered for a type which
// supports ex_data. It should defined as a static global within the module
// which defines that type.
typedef struct {
  struct CRYPTO_STATIC_MUTEX lock;
  STACK_OF(CRYPTO_EX_DATA_FUNCS) *meth;
  // num_reserved is one if the ex_data index zero is reserved for legacy
  // |TYPE_get_app_data| functions.
  uint8_t num_reserved;
} CRYPTO_EX_DATA_CLASS;

#define CRYPTO_EX_DATA_CLASS_INIT {CRYPTO_STATIC_MUTEX_INIT, NULL, 0}
#define CRYPTO_EX_DATA_CLASS_INIT_WITH_APP_DATA \
    {CRYPTO_STATIC_MUTEX_INIT, NULL, 1}

// CRYPTO_get_ex_new_index allocates a new index for |ex_data_class| and writes
// it to |*out_index|. Each class of object should provide a wrapper function
// that uses the correct |CRYPTO_EX_DATA_CLASS|. It returns one on success and
// zero otherwise.
OPENSSL_EXPORT int CRYPTO_get_ex_new_index(CRYPTO_EX_DATA_CLASS *ex_data_class,
                                           int *out_index, long argl,
                                           void *argp,
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

#if defined(AWS_LC_BUILTIN_SWAP_SUPPORTED)
static inline uint16_t CRYPTO_bswap2(uint16_t x) {
  return __builtin_bswap16(x);
}

static inline uint32_t CRYPTO_bswap4(uint32_t x) {
  return __builtin_bswap32(x);
}

static inline uint64_t CRYPTO_bswap8(uint64_t x) {
  return __builtin_bswap64(x);
}
static inline crypto_word_t CRYPTO_bswap_word(crypto_word_t x) {
#if defined(OPENSSL_64_BIT)
  return CRYPTO_bswap8(x);
#else
  return CRYPTO_bswap4(x);
#endif
}


#elif defined(_MSC_VER)
OPENSSL_MSVC_PRAGMA(warning(push, 3))
#include <stdlib.h>
OPENSSL_MSVC_PRAGMA(warning(pop))
#pragma intrinsic(_byteswap_uint64, _byteswap_ulong, _byteswap_ushort)
static inline uint16_t CRYPTO_bswap2(uint16_t x) {
  return _byteswap_ushort(x);
}

static inline uint32_t CRYPTO_bswap4(uint32_t x) {
  return _byteswap_ulong(x);
}

static inline uint64_t CRYPTO_bswap8(uint64_t x) {
  return _byteswap_uint64(x);
}
#else
static inline uint16_t CRYPTO_bswap2(uint16_t x) {
  return (x >> 8) | (x << 8);
}

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
// |OPENSSL_memcmp| performs a constant-time comparison that preserves the
// ordering semantics of |memcmp| (negative, zero, or positive). It iterates
// over the full length regardless of where the first differing byte is, so it
// can be used safely with secret data without leaking timing information.

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

  // C23 makes memchr const-correct, returning const void * when the input is
  // const void *. Some C libraries apply this change even in C11 mode. Cast to
  // match our return type.
  return (void *)memchr(s, c, n);
}

#endif  // __cplusplus

static inline int OPENSSL_memcmp(const void *s1, const void *s2, size_t n) {
  const uint8_t *a = (const uint8_t *)s1;
  const uint8_t *b = (const uint8_t *)s2;
  // Walk from the end so the lowest-index differing byte (the one that decides
  // memcmp's sign) is the last write to |result|. Equal-byte iterations leave
  // |result| unchanged. The loop always runs |n| iterations, so timing depends
  // only on |n|.
  int result = 0;
  for (size_t i = n; i > 0; i--) {
    uint8_t ai = a[i - 1];
    uint8_t bi = b[i - 1];
    crypto_word_t diff_mask =
        ~constant_time_is_zero_w((crypto_word_t)(ai ^ bi));
    result = constant_time_select_int(diff_mask, (int)ai - (int)bi, result);
  }
  return result;
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

static inline uint16_t CRYPTO_load_u16_le(const void *in) {
  uint16_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return CRYPTO_bswap2(v);
#else
  return v;
#endif
}

static inline void CRYPTO_store_u16_le(void *out, uint16_t v) {
#if defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap2(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint16_t CRYPTO_load_u16_be(const void *in) {
  uint16_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return v;
#else
  return CRYPTO_bswap2(v);
#endif
}

static inline void CRYPTO_store_u16_be(void *out, uint16_t v) {
#if !defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap2(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));
}

static inline uint32_t CRYPTO_load_u32_le(const void *in) {
  uint32_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return CRYPTO_bswap4(v);
#else
  return v;
#endif
}

static inline void CRYPTO_store_u32_le(void *out, uint32_t v) {
#if defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap4(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));

}

static inline uint32_t CRYPTO_load_u32_be(const void *in) {
  uint32_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return v;
#else
  return CRYPTO_bswap4(v);
#endif
}

static inline void CRYPTO_store_u32_be(void *out, uint32_t v) {

#if !defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap4(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));

}

static inline uint64_t CRYPTO_load_u64_le(const void *in) {
  uint64_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return CRYPTO_bswap8(v);
#else
  return v;
#endif
}

static inline void CRYPTO_store_u64_le(void *out, uint64_t v) {
#if defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap8(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));

}

static inline uint64_t CRYPTO_load_u64_be(const void *ptr) {
  uint64_t ret;
  OPENSSL_memcpy(&ret, ptr, sizeof(ret));
#if defined(OPENSSL_BIG_ENDIAN)
  return ret;
#else
  return CRYPTO_bswap8(ret);
#endif
}

static inline void CRYPTO_store_u64_be(void *out, uint64_t v) {
#if defined(OPENSSL_BIG_ENDIAN)
#else
  v = CRYPTO_bswap8(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));

}

static inline crypto_word_t CRYPTO_load_word_le(const void *in) {

  crypto_word_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return CRYPTO_bswap_word(v);
#else
  return v;
#endif
}

static inline void CRYPTO_store_word_le(void *out, crypto_word_t v) {


#if defined(OPENSSL_BIG_ENDIAN)
  v = CRYPTO_bswap_word(v);
#endif
  OPENSSL_memcpy(out, &v, sizeof(v));

}

static inline crypto_word_t CRYPTO_load_word_be(const void *in) {
  crypto_word_t v;
  OPENSSL_memcpy(&v, in, sizeof(v));
#if defined(OPENSSL_BIG_ENDIAN)
  return v;
#else
#if defined(OPENSSL_64_BIT)
  assert(sizeof(v) == 8);
  return CRYPTO_bswap8(v);
#else
  assert(sizeof(v) == 4);
  return CRYPTO_bswap4(v);
#endif
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


// Arithmetic functions.

// The most efficient versions of these functions on GCC and Clang depend on C11
// |_Generic|. If we ever need to call these from C++, we'll need to add a
// variant that uses C++ overloads instead.
#if !defined(__cplusplus)

// CRYPTO_addc_* returns |x + y + carry|, and sets |*out_carry| to the carry
// bit. |carry| must be zero or one.
#if OPENSSL_HAS_BUILTIN(__builtin_addc)

#define CRYPTO_GENERIC_ADDC(x, y, carry, out_carry) \
  (_Generic((x),                                    \
      unsigned: __builtin_addc,                     \
      unsigned long: __builtin_addcl,               \
      unsigned long long: __builtin_addcll))((x), (y), (carry), (out_carry))

static inline uint32_t CRYPTO_addc_u32(uint32_t x, uint32_t y, uint32_t carry,
                                       uint32_t *out_carry) {
  declassify_assert(carry <= 1);
  return CRYPTO_GENERIC_ADDC(x, y, carry, out_carry);
}

static inline uint64_t CRYPTO_addc_u64(uint64_t x, uint64_t y, uint64_t carry,
                                       uint64_t *out_carry) {
  declassify_assert(carry <= 1);
  return CRYPTO_GENERIC_ADDC(x, y, carry, out_carry);
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

#define CRYPTO_GENERIC_SUBC(x, y, borrow, out_borrow) \
  (_Generic((x),                                      \
      unsigned: __builtin_subc,                       \
      unsigned long: __builtin_subcl,                 \
      unsigned long long: __builtin_subcll))((x), (y), (borrow), (out_borrow))

static inline uint32_t CRYPTO_subc_u32(uint32_t x, uint32_t y, uint32_t borrow,
                                       uint32_t *out_borrow) {
  declassify_assert(borrow <= 1);
  return CRYPTO_GENERIC_SUBC(x, y, borrow, out_borrow);
}

static inline uint64_t CRYPTO_subc_u64(uint64_t x, uint64_t y, uint64_t borrow,
                                       uint64_t *out_borrow) {
  declassify_assert(borrow <= 1);
  return CRYPTO_GENERIC_SUBC(x, y, borrow, out_borrow);
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

#endif  // !__cplusplus


// FIPS functions.

#if defined(BORINGSSL_FIPS)

// AWS_LC_FIPS_failure is called when a FIPS power-on or continuous test
// fails. The behavior depends on how AWS-LC is built:
// - When AWS-LC is not in FIPS mode it prints |message| to |stderr|.
// - If AWS-LC is built with FIPS it prints |message| to |stderr| and prevents
//     any further cryptographic operations by the current process.
// - If AWS-LC is built with FIPS, AWSLC_FIPS_FAILURE_CALLBACK, and the
//      application does not define the AWS_LC_fips_failure_callback function
//      the normal behavior FIPS behavior is used.
// - If AWS-LC is built with FIPS, AWSLC_FIPS_FAILURE_CALLBACK, and the
//      application defines the AWS_LC_fips_failure_callback function that
//      function is called with |message|.
#if defined(AWSLC_FIPS_FAILURE_CALLBACK)
void AWS_LC_FIPS_failure(const char* message);
#else
#if defined(_MSC_VER)
__declspec(noreturn) void AWS_LC_FIPS_failure(const char* message);
#else
void AWS_LC_FIPS_failure(const char* message) __attribute__((noreturn));
#endif
#endif
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

// boringssl_ensure_ml_kem_self_test checks whether the ML-KEM self-test
// has been run in this address space. If not, it runs it and crashes the
// address space if unsuccessful.
void boringssl_ensure_ml_kem_self_test(void);

// boringssl_ensure_ml_dsa_self_test checks whether the ML-DSA self-test
// has been run in this address space. If not, it runs it and crashes the
// address space if unsuccessful.
void boringssl_ensure_ml_dsa_self_test(void);

// boringssl_ensure_eddsa_self_test checks whether the EdDSA self-test
// has been run in this address space. If not, it runs it and crashes the
// address space if unsuccessful.
void boringssl_ensure_eddsa_self_test(void);

// boringssl_ensure_hasheddsa_self_test checks whether the HashEdDSA self-test
// has been run in this address space. If not, it runs it and crashes the
// address space if unsuccessful.
void boringssl_ensure_hasheddsa_self_test(void);

#else

// Outside of FIPS mode, the lazy tests are no-ops.

OPENSSL_INLINE void boringssl_ensure_rsa_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_ecc_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_ffdh_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_ml_kem_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_ml_dsa_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_eddsa_self_test(void) {}
OPENSSL_INLINE void boringssl_ensure_hasheddsa_self_test(void) {}

// Outside of FIPS mode AWS_LC_FIPS_failure simply logs the message to stderr
void AWS_LC_FIPS_failure(const char* message);

#endif  // FIPS

// boringssl_self_test_sha256 performs a SHA-256 KAT
int boringssl_self_test_sha256(void);

  // boringssl_self_test_hmac_sha256 performs an HMAC-SHA-256 KAT
int boringssl_self_test_hmac_sha256(void);

#if defined(BORINGSSL_FIPS_BREAK_TESTS)
OPENSSL_INLINE int boringssl_fips_break_test(const char *test) {
  const char *const value = getenv("BORINGSSL_FIPS_BREAK_TEST");
  return value != NULL && strcmp(value, test) == 0;
}
#else
OPENSSL_INLINE int boringssl_fips_break_test(const char *test) {
  return 0;
}
#endif  // BORINGSSL_FIPS_BREAK_TESTS

#if defined(BORINGSSL_DISPATCH_TEST)
// Runtime CPU dispatch testing support

// BORINGSSL_function_hit is an array of flags. The following functions will
// set these flags if BORINGSSL_DISPATCH_TEST is defined.
// On x86 and x86_64:
//   0: aes_hw_ctr32_encrypt_blocks
//   1: aes_hw_encrypt
//   2: aesni_gcm_encrypt
//   3: aes_hw_set_encrypt_key
//   4: vpaes_encrypt
//   5: vpaes_set_encrypt_key
//   6: sha256_block_data_order_shaext
//   7: aes_gcm_encrypt_avx512
//   8: RSAZ_mod_exp_avx512_x2
//   9: sha3_keccak_f1600
//  10: sha3_keccak4_f1600_alt
// On AARCH64:
//   0: aes_hw_ctr32_encrypt_blocks
//   1: aes_hw_encrypt
//   2: aes_gcm_enc_kernel
//   3: aes_hw_set_encrypt_key
//   4: vpaes_encrypt
//   5: vpaes_set_encrypt_key
//   6: sha256_block_armv8
//   7: aesv8_gcm_8x_enc_128
//   8: sha512_block_armv8
//   9: KeccakF1600_hw
//  10: sha3_keccak_f1600
//  11: sha3_keccak_f1600_alt
//  12: sha3_keccak2_f1600
//  13: sha3_keccak4_f1600_alt
//  14: sha3_keccak4_f1600_alt2
extern uint8_t BORINGSSL_function_hit[15];
#endif  // BORINGSSL_DISPATCH_TEST

#if !defined(AWSLC_FIPS) && !defined(BORINGSSL_SHARED_LIBRARY)
// This function is defined in |bcm.c|, see the comment therein for explanation.
void dummy_func_for_constructor(void);
#endif
// OPENSSL_vasprintf_internal is just like |vasprintf(3)|. If |system_malloc| is
// 0, memory will be allocated with |OPENSSL_malloc| and must be freed with
// |OPENSSL_free|. Otherwise the system |malloc| function is used and the memory
// must be freed with the system |free| function.
OPENSSL_EXPORT int OPENSSL_vasprintf_internal(char **str, const char *format,
                                              va_list args, int system_malloc)
    OPENSSL_PRINTF_FORMAT_FUNC(2, 0);


// Experimental safety macros inspired by s2n-tls.

// If |cond| is false |action| is invoked, otherwise nothing happens.
#define AWS_LC_ENSURE(cond, action) \
    do {                           \
        if (!(cond)) {             \
            action;                \
        }                          \
    } while (0)

#define AWS_LC_ERROR 0
#define AWS_LC_SUCCESS 1

// GUARD_PTR checks |ptr|: if it is NULL it adds |ERR_R_PASSED_NULL_PARAMETER|
// to the error queue and returns 0, if it is not NULL nothing happens.
//
// NOTE: this macro should only be used with functions that return 0 (for error)
// and 1 (for success).
#define GUARD_PTR(ptr) AWS_LC_ENSURE((ptr) != NULL, OPENSSL_PUT_ERROR(CRYPTO, ERR_R_PASSED_NULL_PARAMETER); \
                                      return AWS_LC_ERROR)

// GUARD_PTR_ABORT checks |ptr|: if it is NULL it calls abort() and does nothing
// otherwise.
#define GUARD_PTR_ABORT(ptr) AWS_LC_ENSURE((ptr) != NULL, abort())

#if defined(NDEBUG)
#define AWSLC_ASSERT(x) (void) (x)
#else
#define AWSLC_ASSERT(x) AWS_LC_ENSURE(x, abort())
#endif

#define AWSLC_ABORT_IF_NOT_ONE(x) AWS_LC_ENSURE(1 == (x), abort())

// Windows doesn't really support weak symbols as of May 2019, and Clang on
// Windows will emit strong symbols instead. See
// https://bugs.llvm.org/show_bug.cgi?id=37598
#if defined(__ELF__) && defined(__GNUC__)
#define WEAK_SYMBOL_FUNC(rettype, name, args) \
rettype name args __attribute__((weak));
#else
#define WEAK_SYMBOL_FUNC(rettype, name, args) static rettype(*name) args = NULL;
#endif


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_INTERNAL_H
