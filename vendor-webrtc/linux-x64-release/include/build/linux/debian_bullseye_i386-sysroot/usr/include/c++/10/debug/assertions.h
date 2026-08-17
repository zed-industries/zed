// Debugging support implementation -*- C++ -*-

// Copyright (C) 2003-2020 Free Software Foundation, Inc.
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

/** @file debug/assertions.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_ASSERTIONS_H
#define _GLIBCXX_DEBUG_ASSERTIONS_H 1

#ifndef _GLIBCXX_DEBUG

# define _GLIBCXX_DEBUG_ASSERT(_Condition)
# define _GLIBCXX_DEBUG_PEDASSERT(_Condition)
# define _GLIBCXX_DEBUG_ONLY(_Statement)

#endif

#ifndef _GLIBCXX_ASSERTIONS
# define __glibcxx_requires_non_empty_range(_First,_Last)
# define __glibcxx_requires_nonempty()
# define __glibcxx_requires_subscript(_N)
#else

// Verify that [_First, _Last) forms a non-empty iterator range.
# define __glibcxx_requires_non_empty_range(_First,_Last)	\
  __glibcxx_assert(__builtin_expect(_First != _Last, true))
# define __glibcxx_requires_subscript(_N)	\
  __glibcxx_assert(__builtin_expect(_N < this->size(), true))
// Verify that the container is nonempty
# define __glibcxx_requires_nonempty()		\
  __glibcxx_assert(__builtin_expect(!this->empty(), true))
#endif

#ifdef _GLIBCXX_DEBUG
# define _GLIBCXX_DEBUG_ASSERT(_Condition) __glibcxx_assert(_Condition)

# ifdef _GLIBCXX_DEBUG_PEDANTIC
#  define _GLIBCXX_DEBUG_PEDASSERT(_Condition) _GLIBCXX_DEBUG_ASSERT(_Condition)
# else
#  define _GLIBCXX_DEBUG_PEDASSERT(_Condition)
# endif

# define _GLIBCXX_DEBUG_ONLY(_Statement) _Statement
#endif

#endif // _GLIBCXX_DEBUG_ASSERTIONS
