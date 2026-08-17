// -*- C++ -*- compatibility header.

// Copyright (C) 2002-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file stdlib.h
 *  This is a Standard C++ Library header.
 */

#if !defined __cplusplus || defined _GLIBCXX_INCLUDE_NEXT_C_HEADERS
# include_next <stdlib.h>
#else

#ifndef _GLIBCXX_STDLIB_H
#define _GLIBCXX_STDLIB_H 1

# include <cstdlib>

using std::abort;
using std::atexit;
using std::exit;
#if __cplusplus >= 201103L
# ifdef _GLIBCXX_HAVE_AT_QUICK_EXIT
  using std::at_quick_exit;
# endif
# ifdef _GLIBCXX_HAVE_QUICK_EXIT
  using std::quick_exit;
# endif
#endif

#if _GLIBCXX_HOSTED
using std::div_t;
using std::ldiv_t;

using std::abs;
using std::atof;
using std::atoi;
using std::atol;
using std::bsearch;
using std::calloc;
using std::div;
using std::free;
using std::getenv;
using std::labs;
using std::ldiv;
using std::malloc;
#ifdef _GLIBCXX_HAVE_MBSTATE_T
using std::mblen;
using std::mbstowcs;
using std::mbtowc;
#endif // _GLIBCXX_HAVE_MBSTATE_T
using std::qsort;
using std::rand;
using std::realloc;
using std::srand;
using std::strtod;
using std::strtol;
using std::strtoul;
using std::system;
#ifdef _GLIBCXX_USE_WCHAR_T
using std::wcstombs;
using std::wctomb;
#endif // _GLIBCXX_USE_WCHAR_T
#endif

#endif // _GLIBCXX_STDLIB_H
#endif // __cplusplus
