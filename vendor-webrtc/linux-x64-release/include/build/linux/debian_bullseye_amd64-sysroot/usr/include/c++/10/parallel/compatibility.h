// -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software
// Foundation; either version 3, or (at your option) any later
// version.

// This library is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file parallel/compatibility.h
 *  @brief Compatibility layer, mostly concerned with atomic operations.
 *
 *  This file is a GNU parallel extension to the Standard C++ Library
 *  and contains implementation details for the library's internal use.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_COMPATIBILITY_H
#define _GLIBCXX_PARALLEL_COMPATIBILITY_H 1

#include <parallel/types.h>
#include <parallel/base.h>

#if !defined(_WIN32) || defined (__CYGWIN__)
#include <sched.h>
#endif

#ifdef __MINGW32__
// Including <windows.h> will drag in all the windows32 names.  Since
// that can cause user code portability problems, we just declare the
// one needed function here.
extern "C"
__attribute((dllimport)) void __attribute__((stdcall)) Sleep (unsigned long);
#endif

namespace __gnu_parallel
{
  template<typename _Tp>
    inline _Tp
    __add_omp(volatile _Tp* __ptr, _Tp __addend)
    {
      int64_t __res;
#pragma omp critical
      {
	__res = *__ptr;
	*(__ptr) += __addend;
      }
      return __res;
    }

  /** @brief Add a value to a variable, atomically.
   *
   *  @param __ptr Pointer to a signed integer.
   *  @param __addend Value to add.
   */
  template<typename _Tp>
    inline _Tp
    __fetch_and_add(volatile _Tp* __ptr, _Tp __addend)
    {
      if (__atomic_always_lock_free(sizeof(_Tp), __ptr))
	return __atomic_fetch_add(__ptr, __addend, __ATOMIC_ACQ_REL);
      return __add_omp(__ptr, __addend);
    }

  template<typename _Tp>
    inline bool
    __cas_omp(volatile _Tp* __ptr, _Tp __comparand, _Tp __replacement)
    {
      bool __res = false;
#pragma omp critical
      {
	if (*__ptr == __comparand)
	  {
	    *__ptr = __replacement;
	    __res = true;
	  }
      }
      return __res;
    }

  /** @brief Compare-and-swap
   *
   * Compare @c *__ptr and @c __comparand. If equal, let @c
   * *__ptr=__replacement and return @c true, return @c false otherwise.
   *
   *  @param __ptr Pointer to signed integer.
   *  @param __comparand Compare value.
   *  @param __replacement Replacement value.
   */
  template<typename _Tp>
    inline bool
    __compare_and_swap(volatile _Tp* __ptr, _Tp __comparand, _Tp __replacement)
    {
      if (__atomic_always_lock_free(sizeof(_Tp), __ptr))
	return __atomic_compare_exchange_n(__ptr, &__comparand, __replacement,
					   false, __ATOMIC_ACQ_REL,
					   __ATOMIC_RELAXED);
      return __cas_omp(__ptr, __comparand, __replacement);
    }

  /** @brief Yield control to another thread, without waiting for
   *  the end of the time slice.
   */
  inline void
  __yield()
  {
#if defined (_WIN32) && !defined (__CYGWIN__)
    Sleep(0);
#else
    sched_yield();
#endif
  }
} // end namespace

#endif /* _GLIBCXX_PARALLEL_COMPATIBILITY_H */
