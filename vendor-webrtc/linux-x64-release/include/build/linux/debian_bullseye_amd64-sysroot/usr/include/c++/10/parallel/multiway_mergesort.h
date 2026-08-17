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

/** @file parallel/multiway_mergesort.h
 *  @brief Parallel multiway merge sort.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_MULTIWAY_MERGESORT_H
#define _GLIBCXX_PARALLEL_MULTIWAY_MERGESORT_H 1

#include <vector>

#include <parallel/basic_iterator.h>
#include <bits/stl_algo.h>
#include <parallel/parallel.h>
#include <parallel/multiway_merge.h>

namespace __gnu_parallel
{
  /** @brief Subsequence description. */
  template<typename _DifferenceTp>
    struct _Piece
    {
      typedef _DifferenceTp _DifferenceType;

      /** @brief Begin of subsequence. */
      _DifferenceType _M_begin;

      /** @brief End of subsequence. */
      _DifferenceType _M_end;
    };

  /** @brief Data accessed by all threads.
   *
   *  PMWMS = parallel multiway mergesort */
  template<typename _RAIter>
    struct _PMWMSSortingData
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      /** @brief Number of threads involved. */
      _ThreadIndex _M_num_threads;

      /** @brief Input __begin. */
      _RAIter _M_source;

      /** @brief Start indices, per thread. */
      _DifferenceType* _M_starts;

      /** @brief Storage in which to sort. */
      _ValueType** _M_temporary;

      /** @brief Samples. */
      _ValueType* _M_samples;

      /** @brief Offsets to add to the found positions. */
      _DifferenceType* _M_offsets;

      /** @brief Pieces of data to merge @c [thread][__sequence] */
      std::vector<_Piece<_DifferenceType> >* _M_pieces;
  };

  /**
   *  @brief Select _M_samples from a sequence.
   *  @param __sd Pointer to algorithm data. _Result will be placed in
   *  @c __sd->_M_samples.
   *  @param __num_samples Number of _M_samples to select.
   */
  template<typename _RAIter, typename _DifferenceTp>
    void
    __determine_samples(_PMWMSSortingData<_RAIter>* __sd,
			_DifferenceTp __num_samples)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef _DifferenceTp _DifferenceType;

      _ThreadIndex __iam = omp_get_thread_num();

      _DifferenceType* __es = new _DifferenceType[__num_samples + 2];

      __equally_split(__sd->_M_starts[__iam + 1] - __sd->_M_starts[__iam], 
		      __num_samples + 1, __es);

      for (_DifferenceType __i = 0; __i < __num_samples; ++__i)
	::new(&(__sd->_M_samples[__iam * __num_samples + __i]))
	    _ValueType(__sd->_M_source[__sd->_M_starts[__iam]
				       + __es[__i + 1]]);

      delete[] __es;
    }

  /** @brief Split consistently. */
  template<bool __exact, typename _RAIter,
	   typename _Compare, typename _SortingPlacesIterator>
    struct _SplitConsistently
    { };

  /** @brief Split by exact splitting. */
  template<typename _RAIter, typename _Compare,
	   typename _SortingPlacesIterator>
    struct _SplitConsistently<true, _RAIter, _Compare, _SortingPlacesIterator>
    {
      void
      operator()(const _ThreadIndex __iam,
		 _PMWMSSortingData<_RAIter>* __sd,
		 _Compare& __comp,
		 const typename
		 std::iterator_traits<_RAIter>::difference_type
		 __num_samples) const
      {
#       pragma omp barrier

	std::vector<std::pair<_SortingPlacesIterator,
	                      _SortingPlacesIterator> >
	  __seqs(__sd->_M_num_threads);
	for (_ThreadIndex __s = 0; __s < __sd->_M_num_threads; __s++)
	  __seqs[__s] = std::make_pair(__sd->_M_temporary[__s],
				       __sd->_M_temporary[__s]
				       + (__sd->_M_starts[__s + 1]
					  - __sd->_M_starts[__s]));

	std::vector<_SortingPlacesIterator> __offsets(__sd->_M_num_threads);

	// if not last thread
	if (__iam < __sd->_M_num_threads - 1)
	  multiseq_partition(__seqs.begin(), __seqs.end(),
			     __sd->_M_starts[__iam + 1], __offsets.begin(),
			     __comp);

	for (_ThreadIndex __seq = 0; __seq < __sd->_M_num_threads; __seq++)
	  {
	    // for each sequence
	    if (__iam < (__sd->_M_num_threads - 1))
	      __sd->_M_pieces[__iam][__seq]._M_end
		= __offsets[__seq] - __seqs[__seq].first;
	    else
	      // very end of this sequence
	      __sd->_M_pieces[__iam][__seq]._M_end =
		__sd->_M_starts[__seq + 1] - __sd->_M_starts[__seq];
	  }

#       pragma omp barrier

	for (_ThreadIndex __seq = 0; __seq < __sd->_M_num_threads; __seq++)
	  {
	    // For each sequence.
	    if (__iam > 0)
	      __sd->_M_pieces[__iam][__seq]._M_begin =
		__sd->_M_pieces[__iam - 1][__seq]._M_end;
	    else
	      // Absolute beginning.
	      __sd->_M_pieces[__iam][__seq]._M_begin = 0;
	  }
      }
  };

  /** @brief Split by sampling. */ 
  template<typename _RAIter, typename _Compare,
	   typename _SortingPlacesIterator>
    struct _SplitConsistently<false, _RAIter, _Compare, _SortingPlacesIterator>
    {
      void
      operator()(const _ThreadIndex __iam,
		 _PMWMSSortingData<_RAIter>* __sd,
		 _Compare& __comp,
		 const typename
		 std::iterator_traits<_RAIter>::difference_type
		 __num_samples) const
      {
	typedef std::iterator_traits<_RAIter> _TraitsType;
	typedef typename _TraitsType::value_type _ValueType;
	typedef typename _TraitsType::difference_type _DifferenceType;

	__determine_samples(__sd, __num_samples);

#       pragma omp barrier

#       pragma omp single
	__gnu_sequential::sort(__sd->_M_samples,
			       __sd->_M_samples
			       + (__num_samples * __sd->_M_num_threads),
			       __comp);

#       pragma omp barrier

	for (_ThreadIndex __s = 0; __s < __sd->_M_num_threads; ++__s)
	  {
	    // For each sequence.
	    if (__num_samples * __iam > 0)
	      __sd->_M_pieces[__iam][__s]._M_begin =
                std::lower_bound(__sd->_M_temporary[__s],
				 __sd->_M_temporary[__s]
				 + (__sd->_M_starts[__s + 1]
				    - __sd->_M_starts[__s]),
				 __sd->_M_samples[__num_samples * __iam],
				 __comp)
                - __sd->_M_temporary[__s];
	    else
	      // Absolute beginning.
	      __sd->_M_pieces[__iam][__s]._M_begin = 0;

	    if ((__num_samples * (__iam + 1)) <
		(__num_samples * __sd->_M_num_threads))
	      __sd->_M_pieces[__iam][__s]._M_end =
                std::lower_bound(__sd->_M_temporary[__s],
				 __sd->_M_temporary[__s]
				 + (__sd->_M_starts[__s + 1]
				    - __sd->_M_starts[__s]),
				 __sd->_M_samples[__num_samples * (__iam + 1)],
				 __comp)
                - __sd->_M_temporary[__s];
	    else
	      // Absolute end.
	      __sd->_M_pieces[__iam][__s]._M_end = (__sd->_M_starts[__s + 1]
						    - __sd->_M_starts[__s]);
	  }
      }
  };
  
  template<bool __stable, typename _RAIter, typename _Compare>
    struct __possibly_stable_sort
    { };

  template<typename _RAIter, typename _Compare>
    struct __possibly_stable_sort<true, _RAIter, _Compare>
    {
      void operator()(const _RAIter& __begin,
		      const _RAIter& __end, _Compare& __comp) const
      { __gnu_sequential::stable_sort(__begin, __end, __comp); }
    };

  template<typename _RAIter, typename _Compare>
    struct __possibly_stable_sort<false, _RAIter, _Compare>
    {
      void operator()(const _RAIter __begin,
		      const _RAIter __end, _Compare& __comp) const
      { __gnu_sequential::sort(__begin, __end, __comp); }
    };

  template<bool __stable, typename _Seq_RAIter,
	   typename _RAIter, typename _Compare,
	   typename _DiffType>
    struct __possibly_stable_multiway_merge
    { };

  template<typename _Seq_RAIter, typename _RAIter,
	   typename _Compare, typename _DiffType>
    struct __possibly_stable_multiway_merge<true, _Seq_RAIter,
					    _RAIter, _Compare, _DiffType>
    {
      void operator()(const _Seq_RAIter& __seqs_begin,
		      const _Seq_RAIter& __seqs_end,
		      const _RAIter& __target,
		      _Compare& __comp,
		      _DiffType __length_am) const
      { stable_multiway_merge(__seqs_begin, __seqs_end, __target,
			      __length_am, __comp, sequential_tag()); }
    };

  template<typename _Seq_RAIter, typename _RAIter,
	   typename _Compare, typename _DiffType>
    struct __possibly_stable_multiway_merge<false, _Seq_RAIter,
					    _RAIter, _Compare, _DiffType>
    {
      void operator()(const _Seq_RAIter& __seqs_begin,
                      const _Seq_RAIter& __seqs_end,
                      const _RAIter& __target,
                      _Compare& __comp,
                      _DiffType __length_am) const
      { multiway_merge(__seqs_begin, __seqs_end, __target, __length_am,
		       __comp, sequential_tag()); }
    };

  /** @brief PMWMS code executed by each thread.
   *  @param __sd Pointer to algorithm data.
   *  @param __comp Comparator.
   */
  template<bool __stable, bool __exact, typename _RAIter,
	   typename _Compare>
    void
    parallel_sort_mwms_pu(_PMWMSSortingData<_RAIter>* __sd,
			  _Compare& __comp)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _ThreadIndex __iam = omp_get_thread_num();

      // Length of this thread's chunk, before merging.
      _DifferenceType __length_local =
	__sd->_M_starts[__iam + 1] - __sd->_M_starts[__iam];

      // Sort in temporary storage, leave space for sentinel.

      typedef _ValueType* _SortingPlacesIterator;

      __sd->_M_temporary[__iam] =
        static_cast<_ValueType*>(::operator new(sizeof(_ValueType)
						* (__length_local + 1)));

      // Copy there.
      std::uninitialized_copy(__sd->_M_source + __sd->_M_starts[__iam],
			      __sd->_M_source + __sd->_M_starts[__iam]
			      + __length_local,
			      __sd->_M_temporary[__iam]);

      __possibly_stable_sort<__stable, _SortingPlacesIterator, _Compare>()
        (__sd->_M_temporary[__iam],
	 __sd->_M_temporary[__iam] + __length_local,
         __comp);

      // Invariant: locally sorted subsequence in sd->_M_temporary[__iam],
      // __sd->_M_temporary[__iam] + __length_local.

      // No barrier here: Synchronization is done by the splitting routine.

      _DifferenceType __num_samples =
        _Settings::get().sort_mwms_oversampling * __sd->_M_num_threads - 1;
      _SplitConsistently<__exact, _RAIter, _Compare, _SortingPlacesIterator>()
        (__iam, __sd, __comp, __num_samples);

      // Offset from __target __begin, __length after merging.
      _DifferenceType __offset = 0, __length_am = 0;
      for (_ThreadIndex __s = 0; __s < __sd->_M_num_threads; __s++)
	{
	  __length_am += (__sd->_M_pieces[__iam][__s]._M_end
			  - __sd->_M_pieces[__iam][__s]._M_begin);
	  __offset += __sd->_M_pieces[__iam][__s]._M_begin;
	}

      typedef std::vector<
        std::pair<_SortingPlacesIterator, _SortingPlacesIterator> >
        _SeqVector;
      _SeqVector __seqs(__sd->_M_num_threads);

      for (_ThreadIndex __s = 0; __s < __sd->_M_num_threads; ++__s)
	{
	  __seqs[__s] =
	    std::make_pair(__sd->_M_temporary[__s]
			   + __sd->_M_pieces[__iam][__s]._M_begin,
			   __sd->_M_temporary[__s]
			   + __sd->_M_pieces[__iam][__s]._M_end);
	}

      __possibly_stable_multiway_merge<
        __stable, typename _SeqVector::iterator,
	_RAIter, _Compare, _DifferenceType>()(__seqs.begin(), __seqs.end(),
				     __sd->_M_source + __offset, __comp,
				     __length_am);

#     pragma omp barrier

      for (_DifferenceType __i = 0; __i < __length_local; ++__i)
	__sd->_M_temporary[__iam][__i].~_ValueType();
      ::operator delete(__sd->_M_temporary[__iam]);
    }

  /** @brief PMWMS main call.
   *  @param __begin Begin iterator of sequence.
   *  @param __end End iterator of sequence.
   *  @param __comp Comparator.
   *  @param __num_threads Number of threads to use.
   */
  template<bool __stable, bool __exact, typename _RAIter,
           typename _Compare>
    void
    parallel_sort_mwms(_RAIter __begin, _RAIter __end,
		       _Compare __comp,
		       _ThreadIndex __num_threads)
    {
      _GLIBCXX_CALL(__end - __begin)

      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __n = __end - __begin;

      if (__n <= 1)
	return;

      // at least one element per thread
      if (__num_threads > __n)
	__num_threads = static_cast<_ThreadIndex>(__n);

      // shared variables
      _PMWMSSortingData<_RAIter> __sd;
      _DifferenceType* __starts;
      _DifferenceType __size;

#     pragma omp parallel num_threads(__num_threads)
      {
        __num_threads = omp_get_num_threads(); //no more threads than requested

#       pragma omp single
	{
	  __sd._M_num_threads = __num_threads;
	  __sd._M_source = __begin;
	  
	  __sd._M_temporary = new _ValueType*[__num_threads];

	  if (!__exact)
	    {
	      __size =
		(_Settings::get().sort_mwms_oversampling * __num_threads - 1)
		* __num_threads;
	      __sd._M_samples = static_cast<_ValueType*>
		(::operator new(__size * sizeof(_ValueType)));
	    }
	  else
	    __sd._M_samples = 0;

	  __sd._M_offsets = new _DifferenceType[__num_threads - 1];
	  __sd._M_pieces
	    = new std::vector<_Piece<_DifferenceType> >[__num_threads];
	  for (_ThreadIndex __s = 0; __s < __num_threads; ++__s)
	    __sd._M_pieces[__s].resize(__num_threads);
	  __starts = __sd._M_starts = new _DifferenceType[__num_threads + 1];

	  _DifferenceType __chunk_length = __n / __num_threads;
	  _DifferenceType __split = __n % __num_threads;
	  _DifferenceType __pos = 0;
	  for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	    {
	      __starts[__i] = __pos;
	      __pos += ((__i < __split)
			? (__chunk_length + 1) : __chunk_length);
	    }
	  __starts[__num_threads] = __pos;
	} //single

        // Now sort in parallel.
        parallel_sort_mwms_pu<__stable, __exact>(&__sd, __comp);
      } //parallel

      delete[] __starts;
      delete[] __sd._M_temporary;

      if (!__exact)
	{
	  for (_DifferenceType __i = 0; __i < __size; ++__i)
	    __sd._M_samples[__i].~_ValueType();
	  ::operator delete(__sd._M_samples);
	}

      delete[] __sd._M_offsets;
      delete[] __sd._M_pieces;
    }

} //namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_MULTIWAY_MERGESORT_H */
