/* ----------------------------------------------------------------------------
Copyright (c) 2019-2024 Microsoft Research, Daan Leijen
This is free software; you can redistribute it and/or modify it under the
terms of the MIT license. A copy of the license can be found in the file
"LICENSE" at the root of this distribution.
-----------------------------------------------------------------------------*/

/* ----------------------------------------------------------------------------
  Bit operation, and platform dependent definition (MI_INTPTR_SIZE etc)
---------------------------------------------------------------------------- */

#pragma once
#ifndef MI_BITS_H
#define MI_BITS_H


// ------------------------------------------------------
// Size of a pointer.
// We assume that `sizeof(void*)==sizeof(intptr_t)`
// and it holds for all platforms we know of.
//
// However, the C standard only requires that:
//  p == (void*)((intptr_t)p))
// but we also need:
//  i == (intptr_t)((void*)i)
// or otherwise one might define an intptr_t type that is larger than a pointer...
// ------------------------------------------------------

#if INTPTR_MAX > INT64_MAX
# define MI_INTPTR_SHIFT (4)  // assume 128-bit  (as on arm CHERI for example)
#elif INTPTR_MAX == INT64_MAX
# define MI_INTPTR_SHIFT (3)
#elif INTPTR_MAX == INT32_MAX
# define MI_INTPTR_SHIFT (2)
#else
#error platform pointers must be 32, 64, or 128 bits
#endif

#if (INTPTR_MAX) > LONG_MAX
# define MI_PU(x)  x##ULL
#else
# define MI_PU(x)  x##UL
#endif

#if SIZE_MAX == UINT64_MAX
# define MI_SIZE_SHIFT (3)
typedef int64_t  mi_ssize_t;
#elif SIZE_MAX == UINT32_MAX
# define MI_SIZE_SHIFT (2)
typedef int32_t  mi_ssize_t;
#else
#error platform objects must be 32 or 64 bits in size
#endif

#if (SIZE_MAX/2) > LONG_MAX
# define MI_ZU(x)  x##ULL
#else
# define MI_ZU(x)  x##UL
#endif

#define MI_INTPTR_SIZE  (1<<MI_INTPTR_SHIFT)
#define MI_INTPTR_BITS  (MI_INTPTR_SIZE*8)

#define MI_SIZE_SIZE  (1<<MI_SIZE_SHIFT)
#define MI_SIZE_BITS  (MI_SIZE_SIZE*8)

#define MI_KiB     (MI_ZU(1024))
#define MI_MiB     (MI_KiB*MI_KiB)
#define MI_GiB     (MI_MiB*MI_KiB)


/* --------------------------------------------------------------------------------
  Architecture
-------------------------------------------------------------------------------- */

#if defined(__aarch64__) || defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)  // consider arm64ec as arm64
#define MI_ARCH_ARM64     1
#elif defined(__amd64__) || defined(__amd64) || defined(__x86_64__) || defined(__x86_64) || defined(_M_X64) || defined(_M_AMD64)
#define MI_ARCH_X64       1
#elif defined(__i386__) || defined(__i386) || defined(_M_IX86) || defined(_X86_) || defined(__X86__)
#define MI_ARCH_X86       1
#elif defined(__arm__) || defined(_ARM) || defined(_M_ARM)  || defined(_M_ARMT) || defined(__arm)
#define MI_ARCH_ARM32     1
#elif defined(__riscv) || defined(_M_RISCV)
#define MI_ARCH_RISCV     1
#if (LONG_MAX == INT32_MAX)
#define MI_ARCH_RISCV32   1
#else
#define MI_ARCH_RISCV64   1
#endif
#endif

#if (MI_ARCH_X86 || MI_ARCH_X64)
#include <immintrin.h>
#elif MI_ARCH_ARM64 && MI_OPT_SIMD
#include <arm_neon.h>
#endif
#if defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
#include <intrin.h>
#endif

#if MI_ARCH_X64 && defined(__AVX2__) && !defined(__BMI2__) // msvc
#define __BMI2__  1
#endif
#if MI_ARCH_X64 && (defined(__AVX2__) || defined(__BMI2__)) && !defined(__BMI1__) // msvc
#define __BMI1__  1
#endif

// Define big endian if needed
// #define MI_BIG_ENDIAN  1

// maximum virtual address bits in a user-space pointer
#if MI_DEFAULT_VIRTUAL_ADDRESS_BITS > 0 
#define MI_MAX_VABITS     MI_DEFAULT_VIRTUAL_ADDRESS_BITS
#elif   MI_ARCH_X64
#define MI_MAX_VABITS     (47)
#elif MI_INTPTR_SIZE > 4
#define MI_MAX_VABITS     (48)
#else
#define MI_MAX_VABITS     (32)
#endif


// use a flat page-map (or a 2-level one)
#ifndef MI_PAGE_MAP_FLAT
#if MI_MAX_VABITS <= 40 && !MI_SECURE && !defined(__APPLE__) 
#define MI_PAGE_MAP_FLAT  1
#else
#define MI_PAGE_MAP_FLAT  0
#endif
#endif

#if MI_PAGE_MAP_FLAT && MI_SECURE
#error should not use MI_PAGE_MAP_FLAT with a secure build
#endif


/* --------------------------------------------------------------------------------
  Builtin's
-------------------------------------------------------------------------------- */

#if defined(__GNUC__) || defined(__clang__)
#define mi_unlikely(x)     (__builtin_expect(!!(x),false))
#define mi_likely(x)       (__builtin_expect(!!(x),true))
#elif (defined(__cplusplus) && (__cplusplus >= 202002L)) || (defined(_MSVC_LANG) && _MSVC_LANG >= 202002L)
#define mi_unlikely(x)     (x) [[unlikely]]
#define mi_likely(x)       (x) [[likely]]
#else
#define mi_unlikely(x)     (x)
#define mi_likely(x)       (x)
#endif


#ifndef __has_builtin
#define __has_builtin(x)  0
#endif

#define mi_builtin(name)        __builtin_##name
#define mi_has_builtin(name)    __has_builtin(__builtin_##name)

#if (LONG_MAX == INT32_MAX)
#define mi_builtin32(name)       mi_builtin(name##l)
#define mi_has_builtin32(name)   mi_has_builtin(name##l)
#else
#define mi_builtin32(name)       mi_builtin(name)
#define mi_has_builtin32(name)   mi_has_builtin(name)
#endif
#if (LONG_MAX == INT64_MAX)
#define mi_builtin64(name)       mi_builtin(name##l)
#define mi_has_builtin64(name)   mi_has_builtin(name##l)
#else
#define mi_builtin64(name)       mi_builtin(name##ll)
#define mi_has_builtin64(name)   mi_has_builtin(name##ll)
#endif

#if (MI_SIZE_BITS == 32)
#define mi_builtinz(name)        mi_builtin32(name)
#define mi_has_builtinz(name)    mi_has_builtin32(name)
#define mi_msc_builtinz(name)    name
#elif (MI_SIZE_BITS == 64)
#define mi_builtinz(name)        mi_builtin64(name)
#define mi_has_builtinz(name)    mi_has_builtin64(name)
#define mi_msc_builtinz(name)    name##64
#endif

/* --------------------------------------------------------------------------------
  Popcount and count trailing/leading zero's
-------------------------------------------------------------------------------- */

size_t _mi_popcount_generic(size_t x);
extern bool _mi_cpu_has_popcnt;

static inline size_t mi_popcount(size_t x) {
  #if defined(__GNUC__) && (MI_ARCH_X64 || MI_ARCH_X86)
    #if !defined(__BMI1__)
    if mi_unlikely(!_mi_cpu_has_popcnt) { return _mi_popcount_generic(x); }
    #endif
    size_t r;
    __asm ("popcnt\t%1,%0" : "=r"(r) : "r"(x) : "cc");
    return r;
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86)
    #if !defined(__BMI1__)
    if mi_unlikely(!_mi_cpu_has_popcnt) { return _mi_popcount_generic(x); }
    #endif
    return (size_t)mi_msc_builtinz(__popcnt)(x);
  #elif defined(_MSC_VER) && MI_ARCH_ARM64
    return (size_t)mi_msc_builtinz(__popcnt)(x);
  #elif mi_has_builtinz(popcount)
    return mi_builtinz(popcount)(x);
  #else
    #define MI_HAS_FAST_POPCOUNT  0
    return _mi_popcount_generic(x);
  #endif
}

#ifndef MI_HAS_FAST_POPCOUNT
#define MI_HAS_FAST_POPCOUNT 1
#endif



size_t _mi_clz_generic(size_t x);
size_t _mi_ctz_generic(size_t x);

static inline size_t mi_ctz(size_t x) {
  #if defined(__GNUC__) && MI_ARCH_X64 && defined(__BMI1__)  
    size_t r;
    __asm ("tzcnt\t%1, %0" : "=r"(r) : "r"(x) : "cc");
    return r;
  #elif defined(__GNUC__) && MI_ARCH_X64
    // tzcnt is interpreted as bsf if BMI1 is not supported (pre-haswell)
    // if the argument is zero:
    // - tzcnt: sets carry-flag, and returns MI_SIZE_BITS
    // - bsf  : sets zero-flag, and leaves the destination _unmodified_ (on both AMD and Intel now, see <https://github.com/llvm/llvm-project/pull/102885>)
    // so we always initialize r to MI_SIZE_BITS to work correctly on all cpu's without branching
    size_t r = MI_SIZE_BITS;
    __asm ("tzcnt\t%1, %0" : "+r"(r) : "r"(x) : "cc");  // use '+r' to keep the assignment to r in case this becomes bsf on older cpu's
    return r;
  #elif mi_has_builtinz(ctz)
    return (x!=0 ? (size_t)mi_builtinz(ctz)(x) : MI_SIZE_BITS);
  #elif defined(_MSC_VER) && MI_ARCH_X64 && defined(__BMI1__)
    return (x!=0 ? _tzcnt_u64(x) : MI_SIZE_BITS);  // ensure it still works on non-BMI1 cpu's as well
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
    if (x==0) return MI_SIZE_BITS; // test explicitly for `x==0` to avoid codegen bug (issue #1071)    
    unsigned long idx; mi_msc_builtinz(_BitScanForward)(&idx, x);
    return (size_t)idx;
  #elif defined(__GNUC__) && MI_ARCH_X86
    size_t r = MI_SIZE_BITS;
    __asm ("bsf\t%1, %0" : "+r"(r) : "r"(x) : "cc");
    return r;
  #elif MI_HAS_FAST_POPCOUNT
    return (x!=0 ? (mi_popcount(x^(x-1))-1) : MI_SIZE_BITS);
  #else
    #define MI_HAS_FAST_BITSCAN  0
    return (x!=0 ? _mi_ctz_generic(x) : MI_SIZE_BITS);
  #endif
}

static inline size_t mi_clz(size_t x) {
  // we don't optimize anymore to lzcnt as there are still non BMI1 cpu's around (like Intel Celeron, see issue #1016)
  // on pre-haswell cpu's lzcnt gets executed as bsr which is not equivalent (at it returns the bit position)
  #if defined(__GNUC__) && MI_ARCH_X64 && defined(__BMI1__) // on x64 lzcnt is defined for 0
    size_t r;
    __asm ("lzcnt\t%1, %0" : "=r"(r) : "r"(x) : "cc");
    return r;
  #elif mi_has_builtinz(clz)
    return (x!=0 ? (size_t)mi_builtinz(clz)(x) : MI_SIZE_BITS);
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
    if (x==0) return MI_SIZE_BITS; // test explicitly for `x==0` to avoid codegen bug (issue #1071)    
    unsigned long idx; mi_msc_builtinz(_BitScanReverse)(&idx, x);
    return (MI_SIZE_BITS - 1 - (size_t)idx);    
  #elif defined(__GNUC__) && (MI_ARCH_X64 || MI_ARCH_X86)
    if (x==0) return MI_SIZE_BITS;
    size_t r;
    __asm ("bsr\t%1, %0" : "=r"(r) : "r"(x) : "cc");
    return (MI_SIZE_BITS - 1 - r);
  #else
    #define MI_HAS_FAST_BITSCAN  0
    return (x!=0 ? _mi_clz_generic(x) : MI_SIZE_BITS);
  #endif
}

#ifndef MI_HAS_FAST_BITSCAN
#define MI_HAS_FAST_BITSCAN 1
#endif

/* --------------------------------------------------------------------------------
  find trailing/leading zero  (bit scan forward/reverse)
-------------------------------------------------------------------------------- */

// Bit scan forward: find the least significant bit that is set (i.e. count trailing zero's)
// return false if `x==0` (with `*idx` undefined) and true otherwise,
// with the `idx` is set to the bit index (`0 <= *idx < MI_BFIELD_BITS`).
static inline bool mi_bsf(size_t x, size_t* idx) {
  #if defined(__GNUC__) && MI_ARCH_X64 && defined(__BMI1__) && (!defined(__clang_major__) || __clang_major__ >= 9)
    // on x64 the carry flag is set on zero which gives better codegen
    bool is_zero;
    __asm ( "tzcnt\t%2, %1" : "=@ccc"(is_zero), "=r"(*idx) : "r"(x) : "cc" );
    return !is_zero;
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
    if (x==0) return false; // test explicitly for `x==0` to avoid codegen bug (issue #1071)    
    unsigned long i; mi_msc_builtinz(_BitScanForward)(&i, x);
    *idx = (size_t)i;
    return true;
  #else
    return (x!=0 ? (*idx = mi_ctz(x), true) : false);
  #endif
}

// Bit scan reverse: find the most significant bit that is set
// return false if `x==0` (with `*idx` undefined) and true otherwise,
// with the `idx` is set to the bit index (`0 <= *idx < MI_BFIELD_BITS`).
static inline bool mi_bsr(size_t x, size_t* idx) {
  #if defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
    if (x==0) return false; // test explicitly for `x==0` to avoid codegen bug (issue #1071)    
    unsigned long i; mi_msc_builtinz(_BitScanReverse)(&i, x);
    *idx = (size_t)i;
    return true;    
  #else
    return (x!=0 ? (*idx = MI_SIZE_BITS - 1 - mi_clz(x), true) : false);
  #endif
}


/* --------------------------------------------------------------------------------
  rotate
-------------------------------------------------------------------------------- */

static inline size_t mi_rotr(size_t x, size_t r) {
  #if (mi_has_builtin(rotateright64) && MI_SIZE_BITS==64)
    return mi_builtin(rotateright64)(x,r);
  #elif (mi_has_builtin(rotateright32) && MI_SIZE_BITS==32)
    return mi_builtin(rotateright32)(x,r);
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_ARM64)
    return _rotr64(x, (int)r);
  #elif defined(_MSC_VER) && (MI_ARCH_X86 || MI_ARCH_ARM32)
    return _lrotr(x,(int)r);
  #else
    // The term `(-rshift)&(BITS-1)` is written instead of `BITS - rshift` to
    // avoid UB when `rshift==0`. See <https://blog.regehr.org/archives/1063>
    const unsigned int rshift = (unsigned int)(r) & (MI_SIZE_BITS-1);
    return ((x >> rshift) | (x << ((-rshift) & (MI_SIZE_BITS-1))));
  #endif
}

static inline size_t mi_rotl(size_t x, size_t r) {
  #if (mi_has_builtin(rotateleft64) && MI_SIZE_BITS==64)
    return mi_builtin(rotateleft64)(x,r);
  #elif (mi_has_builtin(rotateleft32) && MI_SIZE_BITS==32)
    return mi_builtin(rotateleft32)(x,r);
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_ARM64)
    return _rotl64(x, (int)r);
  #elif defined(_MSC_VER) && (MI_ARCH_X86 || MI_ARCH_ARM32)
    return _lrotl(x, (int)r);
  #else
    // The term `(-rshift)&(BITS-1)` is written instead of `BITS - rshift` to
    // avoid UB when `rshift==0`. See <https://blog.regehr.org/archives/1063>
    const unsigned int rshift = (unsigned int)(r) & (MI_SIZE_BITS-1);
    return ((x << rshift) | (x >> ((-rshift) & (MI_SIZE_BITS-1))));
  #endif
}

static inline uint32_t mi_rotl32(uint32_t x, uint32_t r) {
  #if mi_has_builtin(rotateleft32)
    return mi_builtin(rotateleft32)(x,r);
  #elif defined(_MSC_VER) && (MI_ARCH_X64 || MI_ARCH_X86 || MI_ARCH_ARM64 || MI_ARCH_ARM32)
    return _lrotl(x, (int)r);
  #else
    // The term `(-rshift)&(BITS-1)` is written instead of `BITS - rshift` to
    // avoid UB when `rshift==0`. See <https://blog.regehr.org/archives/1063>
    const unsigned int rshift = (unsigned int)(r) & 31;
    return ((x << rshift) | (x >> ((-rshift) & 31)));
  #endif
}


#endif // MI_BITS_H
