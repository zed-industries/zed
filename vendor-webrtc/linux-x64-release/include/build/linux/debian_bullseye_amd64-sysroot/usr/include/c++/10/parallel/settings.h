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

/** @file parallel/settings.h
 *  @brief Runtime settings and tuning parameters, heuristics to decide
 *  whether to use parallelized algorithms.
 *
 *  This file is a GNU parallel extension to the Standard C++ Library.
 *
 *  @section parallelization_decision Deciding whether to run an algorithm in parallel.
 *
 *  There are several ways the user can switch on and off the parallel
 *  execution of an algorithm, both at compile- and run-time.
 *
 *  Only sequential execution can be forced at compile-time.  This
 *  reduces code size and protects code parts that have
 *  non-thread-safe side effects.
 *
 *  Ultimately, forcing parallel execution at compile-time makes
 *  sense.  Often, the sequential algorithm implementation is used as
 *  a subroutine, so no reduction in code size can be achieved.  Also,
 *  the machine the program is run on might have only one processor
 *  core, so to avoid overhead, the algorithm is executed
 *  sequentially.
 *
 *  To force sequential execution of an algorithm ultimately at
 *  compile-time, the user must add the tag
*  gnu_parallel::sequential_tag() to the end of the parameter list,
 *  e. g.
 *
 *  \code
 *  std::sort(__v.begin(), __v.end(), __gnu_parallel::sequential_tag());
 *  \endcode
 *
 *  This is compatible with all overloaded algorithm variants.  No
 *  additional code will be instantiated, at all.  The same holds for
 *  most algorithm calls with iterators not providing random access.
 *
 *  If the algorithm call is not forced to be executed sequentially
 *  at compile-time, the decision is made at run-time.
 *  The global variable __gnu_parallel::_Settings::algorithm_strategy
 *  is checked. It is a tristate variable corresponding to:
 *    - a. force_sequential, meaning the sequential algorithm is executed.
 *    - b. force_parallel, meaning the parallel algorithm is executed.
 *    - c. heuristic
 *
 *  For heuristic, the parallel algorithm implementation is called
 *  only if the input size is sufficiently large.  For most
 *  algorithms, the input size is the (combined) length of the input
 *  sequence(__s).  The threshold can be set by the user, individually
 *  for each algorithm.  The according variables are called
 *  gnu_parallel::_Settings::[algorithm]_minimal_n .
 *
 *  For some of the algorithms, there are even more tuning options,
 *  e. g. the ability to choose from multiple algorithm variants.  See
 *  below for details.
 */

// Written by Johannes Singler and Felix Putze.

#ifndef _GLIBCXX_PARALLEL_SETTINGS_H
#define _GLIBCXX_PARALLEL_SETTINGS_H 1

#include <parallel/types.h>

/** 
  * @brief Determine at compile(?)-time if the parallel variant of an
  * algorithm should be called.
  * @param __c A condition that is convertible to bool that is overruled by
  * __gnu_parallel::_Settings::algorithm_strategy. Usually a decision
  * based on the input size.
  */
#define _GLIBCXX_PARALLEL_CONDITION(__c) \
  (__gnu_parallel::_Settings::get().algorithm_strategy \
    != __gnu_parallel::force_sequential \
  && ((__gnu_parallel::__get_max_threads() > 1 && (__c)) \
     || __gnu_parallel::_Settings::get().algorithm_strategy \
        == __gnu_parallel::force_parallel))

/*
inline bool
parallel_condition(bool __c)
{
  bool ret = false;
  const _Settings& __s = _Settings::get();
  if (__s.algorithm_strategy != force_seqential)
    {
      if (__s.algorithm_strategy == force_parallel)
        ret = true;
      else
        ret = __get_max_threads() > 1 && __c;
    }
  return ret;
}
*/

namespace __gnu_parallel
{
  /// class _Settings
  /// Run-time settings for the parallel mode including all tunable parameters.
  struct _Settings
  {
    _AlgorithmStrategy          algorithm_strategy;
    
    _SortAlgorithm              sort_algorithm;
    _PartialSumAlgorithm        partial_sum_algorithm;
    _MultiwayMergeAlgorithm     multiway_merge_algorithm;
    _FindAlgorithm              find_algorithm;

    _SplittingAlgorithm         sort_splitting;
    _SplittingAlgorithm         merge_splitting;
    _SplittingAlgorithm         multiway_merge_splitting;

    // Per-algorithm settings.

    /// Minimal input size for accumulate.
    _SequenceIndex              accumulate_minimal_n;

    /// Minimal input size for adjacent_difference.
    unsigned int                adjacent_difference_minimal_n;

    /// Minimal input size for count and count_if.
    _SequenceIndex              count_minimal_n;

    /// Minimal input size for fill.
    _SequenceIndex              fill_minimal_n;

    /// Block size increase factor for find.
    double                      find_increasing_factor;

    /// Initial block size for find.
    _SequenceIndex              find_initial_block_size;

    /// Maximal block size for find.
    _SequenceIndex              find_maximum_block_size;

    /// Start with looking for this many elements sequentially, for find.
    _SequenceIndex              find_sequential_search_size;

    /// Minimal input size for for_each.
    _SequenceIndex              for_each_minimal_n;

    /// Minimal input size for generate.
    _SequenceIndex              generate_minimal_n;

    /// Minimal input size for max_element.
    _SequenceIndex              max_element_minimal_n;

    /// Minimal input size for merge.
    _SequenceIndex              merge_minimal_n;

    /// Oversampling factor for merge.
    unsigned int                merge_oversampling;

    /// Minimal input size for min_element.
    _SequenceIndex              min_element_minimal_n;

    /// Minimal input size for multiway_merge.
    _SequenceIndex              multiway_merge_minimal_n;

    /// Oversampling factor for multiway_merge.
    int                         multiway_merge_minimal_k;

    /// Oversampling factor for multiway_merge.
    unsigned int                multiway_merge_oversampling;

    /// Minimal input size for nth_element.
    _SequenceIndex              nth_element_minimal_n;

    /// Chunk size for partition.
    _SequenceIndex              partition_chunk_size;

    /// Chunk size for partition, relative to input size.  If > 0.0,
    /// this value overrides partition_chunk_size.
    double                      partition_chunk_share;

    /// Minimal input size for partition.
    _SequenceIndex              partition_minimal_n;

    /// Minimal input size for partial_sort.
    _SequenceIndex              partial_sort_minimal_n;

    /// Ratio for partial_sum. Assume "sum and write result" to be
    /// this factor slower than just "sum".
    float                       partial_sum_dilation;

    /// Minimal input size for partial_sum.
    unsigned int                partial_sum_minimal_n;

    /// Minimal input size for random_shuffle.
    unsigned int                random_shuffle_minimal_n;

    /// Minimal input size for replace and replace_if.
    _SequenceIndex              replace_minimal_n;

    /// Minimal input size for set_difference.
    _SequenceIndex              set_difference_minimal_n;

    /// Minimal input size for set_intersection.
    _SequenceIndex              set_intersection_minimal_n;

    /// Minimal input size for set_symmetric_difference.
    _SequenceIndex              set_symmetric_difference_minimal_n;

    /// Minimal input size for set_union.
    _SequenceIndex              set_union_minimal_n;

    /// Minimal input size for parallel sorting.
    _SequenceIndex              sort_minimal_n;

    /// Oversampling factor for parallel std::sort (MWMS).
    unsigned int                sort_mwms_oversampling;

    /// Such many samples to take to find a good pivot (quicksort).
    unsigned int                sort_qs_num_samples_preset;

    /// Maximal subsequence __length to switch to unbalanced __base case.
    /// Applies to std::sort with dynamically load-balanced quicksort.
    _SequenceIndex              sort_qsb_base_case_maximal_n;

    /// Minimal input size for parallel std::transform.
    _SequenceIndex              transform_minimal_n;

    /// Minimal input size for unique_copy. 
    _SequenceIndex              unique_copy_minimal_n;

    _SequenceIndex              workstealing_chunk_size;

    // Hardware dependent tuning parameters.

    /// size of the L1 cache in bytes (underestimation).
    unsigned long long          L1_cache_size;

    /// size of the L2 cache in bytes (underestimation).
    unsigned long long          L2_cache_size;

    /// size of the Translation Lookaside Buffer (underestimation).
    unsigned int                TLB_size;

    /// Overestimation of cache line size.  Used to avoid false
    /// sharing, i.e. elements of different threads are at least this
    /// amount apart.
    unsigned int                cache_line_size;

    // Statistics.

    /// The number of stolen ranges in load-balanced quicksort.
    _SequenceIndex              qsb_steals;

    /// Minimal input size for search and search_n.
    _SequenceIndex              search_minimal_n;

    /// Block size scale-down factor with respect to current position.
    float                       find_scale_factor;

    /// Get the global settings.
    _GLIBCXX_CONST static const _Settings&
    get() throw();

    /// Set the global settings.
    static void
    set(_Settings&) throw();

    explicit 
    _Settings() :
            algorithm_strategy(heuristic),
            sort_algorithm(MWMS),
            partial_sum_algorithm(LINEAR),
            multiway_merge_algorithm(LOSER_TREE),
            find_algorithm(CONSTANT_SIZE_BLOCKS),
            sort_splitting(EXACT),
            merge_splitting(EXACT),
            multiway_merge_splitting(EXACT),
            accumulate_minimal_n(1000),
            adjacent_difference_minimal_n(1000),
            count_minimal_n(1000),
            fill_minimal_n(1000),
            find_increasing_factor(2.0),
            find_initial_block_size(256),
            find_maximum_block_size(8192),
            find_sequential_search_size(256),
            for_each_minimal_n(1000),
            generate_minimal_n(1000),
            max_element_minimal_n(1000),
            merge_minimal_n(1000),
            merge_oversampling(10),
            min_element_minimal_n(1000),
            multiway_merge_minimal_n(1000),
            multiway_merge_minimal_k(2), multiway_merge_oversampling(10),
            nth_element_minimal_n(1000),
            partition_chunk_size(1000),
            partition_chunk_share(0.0),
            partition_minimal_n(1000),
            partial_sort_minimal_n(1000),
            partial_sum_dilation(1.0f),
            partial_sum_minimal_n(1000),
            random_shuffle_minimal_n(1000),
            replace_minimal_n(1000),
            set_difference_minimal_n(1000),
            set_intersection_minimal_n(1000),
            set_symmetric_difference_minimal_n(1000),
            set_union_minimal_n(1000),
            sort_minimal_n(1000),
            sort_mwms_oversampling(10),
            sort_qs_num_samples_preset(100),
            sort_qsb_base_case_maximal_n(100),
            transform_minimal_n(1000),
            unique_copy_minimal_n(10000),
            workstealing_chunk_size(100),
            L1_cache_size(16 << 10),
            L2_cache_size(256 << 10),
            TLB_size(128),
            cache_line_size(64),
            qsb_steals(0),
            search_minimal_n(1000),
            find_scale_factor(0.01f)
    { }
  };
}

#endif /* _GLIBCXX_PARALLEL_SETTINGS_H */
