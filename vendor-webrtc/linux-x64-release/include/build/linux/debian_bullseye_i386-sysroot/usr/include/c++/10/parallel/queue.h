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

/** @file parallel/queue.h
 *  @brief Lock-free double-ended queue.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_QUEUE_H
#define _GLIBCXX_PARALLEL_QUEUE_H 1

#include <parallel/types.h>
#include <parallel/base.h>
#include <parallel/compatibility.h>

/** @brief Decide whether to declare certain variable volatile in this file. */
#define _GLIBCXX_VOLATILE volatile

namespace __gnu_parallel
{
  /**@brief Double-ended queue of bounded size, allowing lock-free
   *  atomic access.  push_front() and pop_front() must not be called
   *  concurrently to each other, while pop_back() can be called
   *  concurrently at all times.
   *  @c empty(), @c size(), and @c top() are intentionally not provided.
   *  Calling them would not make sense in a concurrent setting.
   *  @param _Tp Contained element type. */
  template<typename _Tp>
    class _RestrictedBoundedConcurrentQueue
    {
    private:
      /** @brief Array of elements, seen as cyclic buffer. */
      _Tp* _M_base;

      /** @brief Maximal number of elements contained at the same time. */
      _SequenceIndex _M_max_size;

      /** @brief Cyclic __begin and __end pointers contained in one
          atomically changeable value. */
      _GLIBCXX_VOLATILE _CASable _M_borders;

    public:
      /** @brief Constructor. Not to be called concurrent, of course.
       *  @param __max_size Maximal number of elements to be contained. */
      _RestrictedBoundedConcurrentQueue(_SequenceIndex __max_size)
      {
        _M_max_size = __max_size;
        _M_base = new _Tp[__max_size];
        _M_borders = __encode2(0, 0);
#pragma omp flush
      }

      /** @brief Destructor. Not to be called concurrent, of course. */
      ~_RestrictedBoundedConcurrentQueue()
      { delete[] _M_base; }

      /** @brief Pushes one element into the queue at the front end.
       *  Must not be called concurrently with pop_front(). */
      void
      push_front(const _Tp& __t)
      {
        _CASable __former_borders = _M_borders;
        int __former_front, __former_back;
        __decode2(__former_borders, __former_front, __former_back);
        *(_M_base + __former_front % _M_max_size) = __t;
#if _GLIBCXX_PARALLEL_ASSERTIONS
        // Otherwise: front - back > _M_max_size eventually.
        _GLIBCXX_PARALLEL_ASSERT(((__former_front + 1) - __former_back)
                                 <= _M_max_size);
#endif
        __fetch_and_add(&_M_borders, __encode2(1, 0));
      }

      /** @brief Pops one element from the queue at the front end.
       *  Must not be called concurrently with pop_front(). */
      bool
      pop_front(_Tp& __t)
      {
        int __former_front, __former_back;
#pragma omp flush
        __decode2(_M_borders, __former_front, __former_back);
        while (__former_front > __former_back)
          {
            // Chance.
            _CASable __former_borders = __encode2(__former_front,
						  __former_back);
            _CASable __new_borders = __encode2(__former_front - 1,
					       __former_back);
            if (__compare_and_swap(&_M_borders, __former_borders,
				   __new_borders))
              {
                __t = *(_M_base + (__former_front - 1) % _M_max_size);
                return true;
              }
#pragma omp flush
            __decode2(_M_borders, __former_front, __former_back);
          }
        return false;
      }

      /** @brief Pops one element from the queue at the front end.
       *  Must not be called concurrently with pop_front(). */
      bool
      pop_back(_Tp& __t)        //queue behavior
      {
        int __former_front, __former_back;
#pragma omp flush
        __decode2(_M_borders, __former_front, __former_back);
        while (__former_front > __former_back)
          {
            // Chance.
            _CASable __former_borders = __encode2(__former_front,
						  __former_back);
            _CASable __new_borders = __encode2(__former_front,
					       __former_back + 1);
            if (__compare_and_swap(&_M_borders, __former_borders,
				   __new_borders))
              {
                __t = *(_M_base + __former_back % _M_max_size);
                return true;
              }
#pragma omp flush
            __decode2(_M_borders, __former_front, __former_back);
          }
        return false;
      }
  };
}       //namespace __gnu_parallel

#undef _GLIBCXX_VOLATILE

#endif /* _GLIBCXX_PARALLEL_QUEUE_H */
