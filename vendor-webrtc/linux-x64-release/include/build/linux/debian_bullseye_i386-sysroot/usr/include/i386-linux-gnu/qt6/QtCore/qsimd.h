// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2022 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSIMD_H
#define QSIMD_H

#include <QtCore/qglobal.h>

/*
 * qconfig.h defines the QT_COMPILER_SUPPORTS_XXX macros.
 * They mean the compiler supports the necessary flags and the headers
 * for the x86 and ARM intrinsics.
 *
 * Supported instruction set extensions are:
 *   Flag      | Arch
 *  neon       | ARM
 *  mips_dsp   | mips
 *  mips_dspr2 | mips
 *  sse2       | x86
 *  sse4_1     | x86
 *  sse4_2     | x86
 *  avx        | x86
 *
 * Code can use the following constructs to determine compiler support & status:
 * - #if QT_COMPILER_USES(XXX) (e.g: #if QT_COMPILER_USES(neon) or QT_COMPILER_USES(sse4_1)
 *   If this test passes, then the compiler is already generating code using the
 *   given instruction set. The intrinsics for those instructions are
 *   #included and can be used without restriction or runtime check.
 *
 * Code that requires runtime detection and different code paths at runtime is
 * currently not supported here, have a look at qsimd_p.h for support.
 */

#define QT_COMPILER_USES(feature) (1/QT_COMPILER_USES_##feature == 1)

#if defined(Q_PROCESSOR_ARM) && defined(__ARM_NEON) || defined(__ARM_NEON__)
#  include <arm_neon.h>
#  define QT_COMPILER_USES_neon 1
#else
#  define QT_COMPILER_USES_neon -1
#endif

#if defined(Q_PROCESSOR_MIPS) && (defined(__MIPS_DSP__) || (defined(__mips_dsp) && defined(Q_PROCESSOR_MIPS_32)))
#  define QT_COMPILER_USES_mips_dsp 1
#else
#  define QT_COMPILER_USES_mips_dsp -1
#endif

#if defined(Q_PROCESSOR_MIPS) && (defined(__MIPS_DSPR2__) || (defined(__mips_dspr2) && defined(Q_PROCESSOR_MIPS_32)))
#  define QT_COMPILER_USES_mips_dspr2 1
#else
#  define QT_COMPILER_USES_mips_dspr2 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(Q_CC_MSVC)
// MSVC doesn't define __SSE2__, so do it ourselves
#  if (defined(_M_X64) || _M_IX86_FP >= 2)
#    define __SSE__ 1
#    define __SSE2__ 1
#  endif
#  if (defined(_M_AVX) || defined(__AVX__))
// Visual Studio defines __AVX__ when /arch:AVX is passed, but not the earlier macros
// See: https://msdn.microsoft.com/en-us/library/b0084kay.aspx
#    define __SSE3__                        1
#    define __SSSE3__                       1
#    define __SSE4_1__                      1
#    define __SSE4_2__                      1
#    define __POPCNT__                      1
#    ifndef __AVX__
#      define __AVX__                       1
#    endif
#  endif
#  ifdef __SSE2__
#    define QT_VECTORCALL __vectorcall
#  endif
#  ifdef __AVX2__
// MSVC defines __AVX2__ with /arch:AVX2
#    define __F16C__                        1
#    define __RDRND__                       1
#    define __FMA__                         1
#    define __BMI__                         1
#    define __BMI2__                        1
#    define __MOVBE__                       1
#    define __LZCNT__                       1
#  endif
// Starting with /arch:AVX512, MSVC defines all the macros
#endif

#if defined(Q_PROCESSOR_X86) && defined(__SSE2__)
#  include <immintrin.h>
#  define QT_COMPILER_USES_sse2 1
#else
#  define QT_COMPILER_USES_sse2 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(__SSE3__)
#  define QT_COMPILER_USES_sse3 1
#else
#  define QT_COMPILER_USES_sse3 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(__SSSE3__)
#  define QT_COMPILER_USES_ssse3 1
#else
#  define QT_COMPILER_USES_ssse3 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(__SSE4_1__)
#  define QT_COMPILER_USES_sse4_1 1
#else
#  define QT_COMPILER_USES_sse4_1 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(__SSE4_2__)
#  define QT_COMPILER_USES_sse4_2 1
#else
#  define QT_COMPILER_USES_sse4_2 -1
#endif

#if defined(Q_PROCESSOR_X86) && defined(__AVX__)
#  define QT_COMPILER_USES_avx 1
#else
#  define QT_COMPILER_USES_avx -1
#endif

#ifndef QT_VECTORCALL
#define QT_VECTORCALL
#endif

QT_BEGIN_NAMESPACE
QT_END_NAMESPACE

#endif // QSIMD_H
