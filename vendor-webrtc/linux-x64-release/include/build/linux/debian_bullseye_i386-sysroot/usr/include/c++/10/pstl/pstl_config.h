// -*- C++ -*-
//===-- pstl_config.h -----------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _PSTL_CONFIG_H
#define _PSTL_CONFIG_H

// The version is XYYZ, where X is major, YY is minor, and Z is patch (i.e. X.YY.Z)
#define _PSTL_VERSION 9000
#define _PSTL_VERSION_MAJOR (_PSTL_VERSION / 1000)
#define _PSTL_VERSION_MINOR ((_PSTL_VERSION % 1000) / 10)
#define _PSTL_VERSION_PATCH (_PSTL_VERSION % 10)

#if !defined(_PSTL_PAR_BACKEND_SERIAL) && !defined(_PSTL_PAR_BACKEND_TBB)
#    error "The parallel backend is neither serial nor TBB"
#endif

// Check the user-defined macro for warnings
#if defined(PSTL_USAGE_WARNINGS)
#    undef _PSTL_USAGE_WARNINGS
#    define _PSTL_USAGE_WARNINGS PSTL_USAGE_WARNINGS
// Check the internal macro for warnings
#elif !defined(_PSTL_USAGE_WARNINGS)
#    define _PSTL_USAGE_WARNINGS 0
#endif

// Portability "#pragma" definition
#ifdef _MSC_VER
#    define _PSTL_PRAGMA(x) __pragma(x)
#else
#    define _PSTL_PRAGMA(x) _Pragma(#    x)
#endif

#define _PSTL_STRING_AUX(x) #x
#define _PSTL_STRING(x) _PSTL_STRING_AUX(x)
#define _PSTL_STRING_CONCAT(x, y) x #y

// note that when ICC or Clang is in use, _PSTL_GCC_VERSION might not fully match
// the actual GCC version on the system.
#define _PSTL_GCC_VERSION (__GNUC__ * 10000 + __GNUC_MINOR__ * 100 + __GNUC_PATCHLEVEL__)

#if __clang__
// according to clang documentation, version can be vendor specific
#    define _PSTL_CLANG_VERSION (__clang_major__ * 10000 + __clang_minor__ * 100 + __clang_patchlevel__)
#endif

// Enable SIMD for compilers that support OpenMP 4.0
#if (_OPENMP >= 201307) || (__INTEL_COMPILER >= 1600) || (!defined(__INTEL_COMPILER) && _PSTL_GCC_VERSION >= 40900)
#    define _PSTL_PRAGMA_SIMD _PSTL_PRAGMA(omp simd)
#    define _PSTL_PRAGMA_DECLARE_SIMD _PSTL_PRAGMA(omp declare simd)
#    define _PSTL_PRAGMA_SIMD_REDUCTION(PRM) _PSTL_PRAGMA(omp simd reduction(PRM))
#elif !defined(_MSC_VER) //#pragma simd
#    define _PSTL_PRAGMA_SIMD _PSTL_PRAGMA(simd)
#    define _PSTL_PRAGMA_DECLARE_SIMD
#    define _PSTL_PRAGMA_SIMD_REDUCTION(PRM) _PSTL_PRAGMA(simd reduction(PRM))
#else //no simd
#    define _PSTL_PRAGMA_SIMD
#    define _PSTL_PRAGMA_DECLARE_SIMD
#    define _PSTL_PRAGMA_SIMD_REDUCTION(PRM)
#endif //Enable SIMD

#if (__INTEL_COMPILER)
#    define _PSTL_PRAGMA_FORCEINLINE _PSTL_PRAGMA(forceinline)
#else
#    define _PSTL_PRAGMA_FORCEINLINE
#endif

#if (__INTEL_COMPILER >= 1900) || (_PSTL_GCC_VERSION >= 100000)
#    define _PSTL_PRAGMA_SIMD_SCAN(PRM) _PSTL_PRAGMA(omp simd reduction(inscan, PRM))
#    define _PSTL_PRAGMA_SIMD_INCLUSIVE_SCAN(PRM) _PSTL_PRAGMA(omp scan inclusive(PRM))
#    define _PSTL_PRAGMA_SIMD_EXCLUSIVE_SCAN(PRM) _PSTL_PRAGMA(omp scan exclusive(PRM))
#else
#    define _PSTL_PRAGMA_SIMD_SCAN(PRM)
#    define _PSTL_PRAGMA_SIMD_INCLUSIVE_SCAN(PRM)
#    define _PSTL_PRAGMA_SIMD_EXCLUSIVE_SCAN(PRM)
#endif

// Should be defined to 1 for environments with a vendor implementation of C++17 execution policies
#define _PSTL_CPP17_EXECUTION_POLICIES_PRESENT (_MSC_VER >= 1912)

#define _PSTL_CPP14_2RANGE_MISMATCH_EQUAL_PRESENT                                                                      \
    (_MSC_VER >= 1900 || __cplusplus >= 201300L || __cpp_lib_robust_nonmodifying_seq_ops == 201304)
#define _PSTL_CPP14_MAKE_REVERSE_ITERATOR_PRESENT                                                                      \
    (_MSC_VER >= 1900 || __cplusplus >= 201402L || __cpp_lib_make_reverse_iterator == 201402)
#define _PSTL_CPP14_INTEGER_SEQUENCE_PRESENT (_MSC_VER >= 1900 || __cplusplus >= 201402L)
#define _PSTL_CPP14_VARIABLE_TEMPLATES_PRESENT                                                                         \
    (!__INTEL_COMPILER || __INTEL_COMPILER >= 1700) && (_MSC_FULL_VER >= 190023918 || __cplusplus >= 201402L)

#define _PSTL_EARLYEXIT_PRESENT (__INTEL_COMPILER >= 1800)
#define _PSTL_MONOTONIC_PRESENT (__INTEL_COMPILER >= 1800)

#if (__INTEL_COMPILER >= 1900 || !defined(__INTEL_COMPILER) && _PSTL_GCC_VERSION >= 40900 || _OPENMP >= 201307)
#    define _PSTL_UDR_PRESENT 1
#else
#    define _PSTL_UDR_PRESENT 0
#endif

#if ((__INTEL_COMPILER >= 1900 && __INTEL_COMPILER_BUILD_DATE >= 20180626) || _PSTL_GCC_VERSION >= 100000)
#    define _PSTL_UDS_PRESENT 1
#else
#    define _PSTL_UDS_PRESENT 0
#endif

#if _PSTL_EARLYEXIT_PRESENT
#    define _PSTL_PRAGMA_SIMD_EARLYEXIT _PSTL_PRAGMA(omp simd early_exit)
#else
#    define _PSTL_PRAGMA_SIMD_EARLYEXIT
#endif

#if _PSTL_MONOTONIC_PRESENT
#    define _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(PRM) _PSTL_PRAGMA(omp ordered simd monotonic(PRM))
#    define _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC_2ARGS(PRM1, PRM2) _PSTL_PRAGMA(omp ordered simd monotonic(PRM1, PRM2))
#else
#    define _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC(PRM)
#    define _PSTL_PRAGMA_SIMD_ORDERED_MONOTONIC_2ARGS(PRM1, PRM2)
#endif

// Declaration of reduction functor, where
// NAME - the name of the functor
// OP - type of the callable object with the reduction operation
// omp_in - refers to the local partial result
// omp_out - refers to the final value of the combiner operator
// omp_priv - refers to the private copy of the initial value
// omp_orig - refers to the original variable to be reduced
#define _PSTL_PRAGMA_DECLARE_REDUCTION(NAME, OP)                                                                       \
    _PSTL_PRAGMA(omp declare reduction(NAME:OP : omp_out(omp_in)) initializer(omp_priv = omp_orig))

#if (__INTEL_COMPILER >= 1600)
#    define _PSTL_PRAGMA_VECTOR_UNALIGNED _PSTL_PRAGMA(vector unaligned)
#else
#    define _PSTL_PRAGMA_VECTOR_UNALIGNED
#endif

// Check the user-defined macro to use non-temporal stores
#if defined(PSTL_USE_NONTEMPORAL_STORES) && (__INTEL_COMPILER >= 1600)
#    define _PSTL_USE_NONTEMPORAL_STORES_IF_ALLOWED _PSTL_PRAGMA(vector nontemporal)
#else
#    define _PSTL_USE_NONTEMPORAL_STORES_IF_ALLOWED
#endif

#if _MSC_VER || __INTEL_COMPILER //the preprocessors don't type a message location
#    define _PSTL_PRAGMA_LOCATION __FILE__ ":" _PSTL_STRING(__LINE__) ": [Parallel STL message]: "
#else
#    define _PSTL_PRAGMA_LOCATION " [Parallel STL message]: "
#endif

#define _PSTL_PRAGMA_MESSAGE_IMPL(x) _PSTL_PRAGMA(message(_PSTL_STRING_CONCAT(_PSTL_PRAGMA_LOCATION, x)))

#if _PSTL_USAGE_WARNINGS
#    define _PSTL_PRAGMA_MESSAGE(x) _PSTL_PRAGMA_MESSAGE_IMPL(x)
#    define _PSTL_PRAGMA_MESSAGE_POLICIES(x) _PSTL_PRAGMA_MESSAGE_IMPL(x)
#else
#    define _PSTL_PRAGMA_MESSAGE(x)
#    define _PSTL_PRAGMA_MESSAGE_POLICIES(x)
#endif

// broken macros
#define _PSTL_CPP11_STD_ROTATE_BROKEN ((__GLIBCXX__ && __GLIBCXX__ < 20150716) || (_MSC_VER && _MSC_VER < 1800))

#define _PSTL_ICC_18_OMP_SIMD_BROKEN (__INTEL_COMPILER == 1800)

#endif /* _PSTL_CONFIG_H */
