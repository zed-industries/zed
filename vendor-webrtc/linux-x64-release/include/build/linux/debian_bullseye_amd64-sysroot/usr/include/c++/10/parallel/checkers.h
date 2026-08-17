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

/** @file parallel/checkers.h
 *  @brief Routines for checking the correctness of algorithm results.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_CHECKERS_H
#define _GLIBCXX_PARALLEL_CHECKERS_H 1

#include <cstdio>
#include <bits/stl_algobase.h>
#include <bits/stl_function.h>

namespace __gnu_parallel
{
  /**
   * @brief Check whether @c [__begin, @c __end) is sorted according
   * to @c __comp.
   * @param __begin Begin iterator of sequence.
   * @param __end End iterator of sequence.
   * @param __comp Comparator.
   * @return @c true if sorted, @c false otherwise.
   */
  template<typename _IIter, typename _Compare>
    bool
    __is_sorted(_IIter __begin, _IIter __end, _Compare __comp)
    {
      if (__begin == __end)
        return true;

      _IIter __current(__begin), __recent(__begin);

      for (__current++; __current != __end; __current++)
        {
          if (__comp(*__current, *__recent))
            {
              return false;
            }
          __recent = __current;
        }

      return true;
    }
}

#endif /* _GLIBCXX_PARALLEL_CHECKERS_H */
