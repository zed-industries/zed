// TR1 stdlib.h -*- C++ -*-

// Copyright (C) 2006-2020 Free Software Foundation, Inc.
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

/** @file tr1/stdlib.h
 *  This is a TR1 C++ Library header. 
 */

#ifndef _GLIBCXX_TR1_STDLIB_H
#define _GLIBCXX_TR1_STDLIB_H 1

#include <tr1/cstdlib>

#if _GLIBCXX_HOSTED

#if _GLIBCXX_USE_C99_STDLIB

using std::tr1::atoll;
using std::tr1::strtoll;
using std::tr1::strtoull;

using std::tr1::abs;
#if !_GLIBCXX_USE_C99_LONG_LONG_DYNAMIC
using std::tr1::div;
#endif

#endif

#endif

#endif // _GLIBCXX_TR1_STDLIB_H

