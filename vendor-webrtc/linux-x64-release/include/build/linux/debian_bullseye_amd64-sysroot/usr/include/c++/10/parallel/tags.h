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

/**
 * @file parallel/tags.h
 * @brief Tags for compile-time selection.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler and Felix Putze.

#ifndef _GLIBCXX_PARALLEL_TAGS_H
#define _GLIBCXX_PARALLEL_TAGS_H 1

#include <omp.h>
#include <parallel/types.h>

namespace __gnu_parallel
{
  /** @brief Forces sequential execution at compile time. */
  struct sequential_tag { };

  /** @brief Recommends parallel execution at compile time,
   *  optionally using a user-specified number of threads. */
  struct parallel_tag
  {
    private:
      _ThreadIndex _M_num_threads;

    public:
      /** @brief Default constructor. Use default number of threads. */
      parallel_tag()
      { _M_num_threads = 0; }

      /** @brief Default constructor. Recommend number of threads to use.
       *  @param __num_threads Desired number of threads. */
      parallel_tag(_ThreadIndex __num_threads)
      { _M_num_threads = __num_threads; }

      /** @brief Find out desired number of threads.
       *  @return Desired number of threads. */
      _ThreadIndex __get_num_threads()
      {
        if(_M_num_threads == 0)
          return omp_get_max_threads();
        else
          return _M_num_threads;
      }

      /** @brief Set the desired number of threads.
       *  @param __num_threads Desired number of threads. */
      void set_num_threads(_ThreadIndex __num_threads)
      { _M_num_threads = __num_threads; }
  };

  /** @brief Recommends parallel execution using the
      default parallel algorithm. */
  struct default_parallel_tag : public parallel_tag
  {
    default_parallel_tag() { }
    default_parallel_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Recommends parallel execution using dynamic
      load-balancing at compile time. */
  struct balanced_tag : public parallel_tag { };

  /** @brief Recommends parallel execution using static
      load-balancing at compile time. */
  struct unbalanced_tag : public parallel_tag { };

  /** @brief Recommends parallel execution using OpenMP dynamic
      load-balancing at compile time. */
  struct omp_loop_tag : public parallel_tag { };

  /** @brief Recommends parallel execution using OpenMP static
      load-balancing at compile time. */
  struct omp_loop_static_tag : public parallel_tag { };


  /** @brief Base class for for std::find() variants. */
  struct find_tag { };


  /** @brief Forces parallel merging
   *  with exact splitting, at compile time. */
  struct exact_tag : public parallel_tag
  {
    exact_tag() { }
    exact_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Forces parallel merging
   *  with exact splitting, at compile time. */
  struct sampling_tag : public parallel_tag
  {
    sampling_tag() { }
    sampling_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };


  /** @brief Forces parallel sorting using multiway mergesort
   *  at compile time. */
  struct multiway_mergesort_tag : public parallel_tag
  {
    multiway_mergesort_tag() { }
    multiway_mergesort_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Forces parallel sorting using multiway mergesort
   *  with exact splitting at compile time. */
  struct multiway_mergesort_exact_tag : public parallel_tag
  {
    multiway_mergesort_exact_tag() { }
    multiway_mergesort_exact_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Forces parallel sorting using multiway mergesort
   *  with splitting by sampling at compile time. */
  struct multiway_mergesort_sampling_tag : public parallel_tag
  {
    multiway_mergesort_sampling_tag() { }
    multiway_mergesort_sampling_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Forces parallel sorting using unbalanced quicksort
   *  at compile time. */
  struct quicksort_tag : public parallel_tag
  {
    quicksort_tag() { }
    quicksort_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };

  /** @brief Forces parallel sorting using balanced quicksort
   *  at compile time. */
  struct balanced_quicksort_tag : public parallel_tag
  {
    balanced_quicksort_tag() { }
    balanced_quicksort_tag(_ThreadIndex __num_threads)
    : parallel_tag(__num_threads) { }
  };


  /** @brief Selects the growing block size variant for std::find().
      @see _GLIBCXX_FIND_GROWING_BLOCKS */
  struct growing_blocks_tag : public find_tag { };

  /** @brief Selects the constant block size variant for std::find().
      @see _GLIBCXX_FIND_CONSTANT_SIZE_BLOCKS */
  struct constant_size_blocks_tag : public find_tag { };

  /** @brief Selects the equal splitting variant for std::find().
      @see _GLIBCXX_FIND_EQUAL_SPLIT */
  struct equal_split_tag : public find_tag { };
}

#endif /* _GLIBCXX_PARALLEL_TAGS_H */
