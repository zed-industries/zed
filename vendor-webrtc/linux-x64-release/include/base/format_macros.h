// Copyright 2009 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FORMAT_MACROS_H_
#define BASE_FORMAT_MACROS_H_

// This file defines the format macros for some integer types.

// To print a 64-bit value in a portable way:
//   int64_t value;
//   printf("xyz:%" PRId64, value);
// The "d" in the macro corresponds to %d; you can also use PRIu64 etc.
//
// For wide strings, prepend "Wide" to the macro:
//   int64_t value;
//   StringPrintf(L"xyz: %" WidePRId64, value);
//
// To print a size_t value in a portable way:
//   size_t size;
//   printf("xyz: %" PRIuS, size);
// The "u" in the macro corresponds to %u, and S is for "size".

#include <stddef.h>
#include <stdint.h>

#include "build/build_config.h"

#if (BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)) && \
    (defined(_INTTYPES_H) || defined(_INTTYPES_H_)) && !defined(PRId64)
#error "inttypes.h has already been included before this header file, but "
#error "without __STDC_FORMAT_MACROS defined."
#endif

#if (BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)) && \
    !defined(__STDC_FORMAT_MACROS)
#define __STDC_FORMAT_MACROS
#endif

#include <inttypes.h>

#if !defined(PRIuS)
#define PRIuS "zu"
#endif

// The size of NSInteger and NSUInteger varies between 32-bit and 64-bit
// architectures and Apple does not provides standard format macros and
// recommends casting. This has many drawbacks, so instead define macros
// for formatting those types.
#if BUILDFLAG(IS_APPLE)
#if defined(ARCH_CPU_64_BITS)
#if !defined(PRIdNS)
#define PRIdNS "ld"
#endif
#if !defined(PRIuNS)
#define PRIuNS "lu"
#endif
#if !defined(PRIxNS)
#define PRIxNS "lx"
#endif
#else  // defined(ARCH_CPU_64_BITS)
#if !defined(PRIdNS)
#define PRIdNS "d"
#endif
#if !defined(PRIuNS)
#define PRIuNS "u"
#endif
#if !defined(PRIxNS)
#define PRIxNS "x"
#endif
#endif
#endif  // BUILDFLAG(IS_APPLE)

#endif  // BASE_FORMAT_MACROS_H_
