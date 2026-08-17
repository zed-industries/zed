// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP_COMPLEX_H
#define _LIBCPP_COMPLEX_H

/*
    complex.h synopsis

#include <ccomplex>

*/

#if defined(__cplusplus) && __cplusplus < 201103L && defined(_LIBCPP_USE_FROZEN_CXX03_HEADERS)
#  include <__cxx03/complex.h>
#else
#  include <__config>

#  if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#    pragma GCC system_header
#  endif

#  ifdef __cplusplus
#    include <complex>
#  elif __has_include_next(<complex.h>)
#    include_next <complex.h>
#  endif
#endif // defined(__cplusplus) && __cplusplus < 201103L && defined(_LIBCPP_USE_FROZEN_CXX03_HEADERS)

#endif // _LIBCPP_COMPLEX_H
