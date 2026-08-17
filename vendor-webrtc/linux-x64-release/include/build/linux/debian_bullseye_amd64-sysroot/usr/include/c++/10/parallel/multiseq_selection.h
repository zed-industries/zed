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

/** @file parallel/multiseq_selection.h
 *  @brief Functions to find elements of a certain global __rank in
 *  multiple sorted sequences.  Also serves for splitting such
 *  sequence sets.
 *
 *  The algorithm description can be found in 
 *
 *  P. J. Varman, S. D. Scheufler, B. R. Iyer, and G. R. Ricard.
 *  Merging Multiple Lists on Hierarchical-Memory Multiprocessors.
 *  Journal of Parallel and Distributed Computing, 12(2):171-177, 1991.
 *
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_MULTISEQ_SELECTION_H
#define _GLIBCXX_PARALLEL_MULTISEQ_SELECTION_H 1

#include <vector>
#include <queue>

#include <bits/stl_algo.h>

namespace __gnu_parallel
{
  /** @brief Compare __a pair of types lexicographically, ascending. */
  template<typename _T1, typename _T2, typename _Compare>
    class _Lexicographic
    : public std::binary_function<std::pair<_T1, _T2>,
				  std::pair<_T1, _T2>, bool>
    {
    private:
      _Compare& _M_comp;

    public:
      _Lexicographic(_Compare& __comp) : _M_comp(__comp) { }

      bool
      operator()(const std::pair<_T1, _T2>& __p1,
                 const std::pair<_T1, _T2>& __p2) const
      {
        if (_M_comp(__p1.first, __p2.first))
          return true;

        if (_M_comp(__p2.first, __p1.first))
          return false;

        // Firsts are equal.
        return __p1.second < __p2.second;
      }
    };

  /** @brief Compare __a pair of types lexicographically, descending. */
  template<typename _T1, typename _T2, typename _Compare>
    class _LexicographicReverse : public std::binary_function<_T1, _T2, bool>
    {
    private:
      _Compare& _M_comp;

    public:
      _LexicographicReverse(_Compare& __comp) : _M_comp(__comp) { }

      bool
      operator()(const std::pair<_T1, _T2>& __p1,
                 const std::pair<_T1, _T2>& __p2) const
      {
        if (_M_comp(__p2.first, __p1.first))
          return true;

        if (_M_comp(__p1.first, __p2.first))
          return false;

        // Firsts are equal.
        return __p2.second < __p1.second;
      }
    };

  /** 
   *  @brief Splits several sorted sequences at a certain global __rank,
   *  resulting in a splitting point for each sequence.
   *  The sequences are passed via a sequence of random-access
   *  iterator pairs, none of the sequences may be empty.  If there
   *  are several equal elements across the split, the ones on the
   *  __left side will be chosen from sequences with smaller number.
   *  @param __begin_seqs Begin of the sequence of iterator pairs.
   *  @param __end_seqs End of the sequence of iterator pairs.
   *  @param __rank The global rank to partition at.
   *  @param __begin_offsets A random-access __sequence __begin where the
   *  __result will be stored in. Each element of the sequence is an
   *  iterator that points to the first element on the greater part of
   *  the respective __sequence.
   *  @param __comp The ordering functor, defaults to std::less<_Tp>. 
   */
  template<typename _RanSeqs, typename _RankType, typename _RankIterator,
            typename _Compare>
    void
    multiseq_partition(_RanSeqs __begin_seqs, _RanSeqs __end_seqs,
                       _RankType __rank,
                       _RankIterator __begin_offsets,
                       _Compare __comp = std::less<
                       typename std::iterator_traits<typename
                       std::iterator_traits<_RanSeqs>::value_type::
                       first_type>::value_type>()) // std::less<_Tp>
    {
      _GLIBCXX_CALL(__end_seqs - __begin_seqs)

      typedef typename std::iterator_traits<_RanSeqs>::value_type::first_type
        _It;
      typedef typename std::iterator_traits<_RanSeqs>::difference_type
        _SeqNumber;
      typedef typename std::iterator_traits<_It>::difference_type
               _DifferenceType;
      typedef typename std::iterator_traits<_It>::value_type _ValueType;

      _Lexicographic<_ValueType, _SeqNumber, _Compare> __lcomp(__comp);
      _LexicographicReverse<_ValueType, _SeqNumber, _Compare> __lrcomp(__comp);

      // Number of sequences, number of elements in total (possibly
      // including padding).
      _DifferenceType __m = std::distance(__begin_seqs, __end_seqs), __nn = 0,
                      __nmax, __n, __r;

      for (_SeqNumber __i = 0; __i < __m; __i++)
        {
          __nn += std::distance(__begin_seqs[__i].first,
                               __begin_seqs[__i].second);
          _GLIBCXX_PARALLEL_ASSERT(
            std::distance(__begin_seqs[__i].first,
                          __begin_seqs[__i].second) > 0);
        }

      if (__rank == __nn)
        {
          for (_SeqNumber __i = 0; __i < __m; __i++)
            __begin_offsets[__i] = __begin_seqs[__i].second; // Very end.
          // Return __m - 1;
          return;
        }

      _GLIBCXX_PARALLEL_ASSERT(__m != 0);
      _GLIBCXX_PARALLEL_ASSERT(__nn != 0);
      _GLIBCXX_PARALLEL_ASSERT(__rank >= 0);
      _GLIBCXX_PARALLEL_ASSERT(__rank < __nn);

      _DifferenceType* __ns = new _DifferenceType[__m];
      _DifferenceType* __a = new _DifferenceType[__m];
      _DifferenceType* __b = new _DifferenceType[__m];
      _DifferenceType __l;

      __ns[0] = std::distance(__begin_seqs[0].first, __begin_seqs[0].second);
      __nmax = __ns[0];
      for (_SeqNumber __i = 0; __i < __m; __i++)
        {
          __ns[__i] = std::distance(__begin_seqs[__i].first,
                                    __begin_seqs[__i].second);
          __nmax = std::max(__nmax, __ns[__i]);
        }

      __r = __rd_log2(__nmax) + 1;

      // Pad all lists to this length, at least as long as any ns[__i],
      // equality iff __nmax = 2^__k - 1.
      __l = (1ULL << __r) - 1;

      for (_SeqNumber __i = 0; __i < __m; __i++)
        {
          __a[__i] = 0;
          __b[__i] = __l;
        }
      __n = __l / 2;

      // Invariants:
      // 0 <= __a[__i] <= __ns[__i], 0 <= __b[__i] <= __l

#define __S(__i) (__begin_seqs[__i].first)

      // Initial partition.
      std::vector<std::pair<_ValueType, _SeqNumber> > __sample;

      for (_SeqNumber __i = 0; __i < __m; __i++)
        if (__n < __ns[__i])    //__sequence long enough
          __sample.push_back(std::make_pair(__S(__i)[__n], __i));
      __gnu_sequential::sort(__sample.begin(), __sample.end(), __lcomp);

      for (_SeqNumber __i = 0; __i < __m; __i++)       //conceptual infinity
        if (__n >= __ns[__i])   //__sequence too short, conceptual infinity
          __sample.push_back(
            std::make_pair(__S(__i)[0] /*__dummy element*/, __i));

      _DifferenceType __localrank = __rank / __l;

      _SeqNumber __j;
      for (__j = 0;
           __j < __localrank && ((__n + 1) <= __ns[__sample[__j].second]);
           ++__j)
        __a[__sample[__j].second] += __n + 1;
      for (; __j < __m; __j++)
        __b[__sample[__j].second] -= __n + 1;
      
      // Further refinement.
      while (__n > 0)
        {
          __n /= 2;

          _SeqNumber __lmax_seq = -1;  // to avoid warning
          const _ValueType* __lmax = 0; // impossible to avoid the warning?
          for (_SeqNumber __i = 0; __i < __m; __i++)
            {
              if (__a[__i] > 0)
                {
                  if (!__lmax)
                    {
                      __lmax = &(__S(__i)[__a[__i] - 1]);
                      __lmax_seq = __i;
                    }
                  else
                    {
                      // Max, favor rear sequences.
                      if (!__comp(__S(__i)[__a[__i] - 1], *__lmax))
                        {
                          __lmax = &(__S(__i)[__a[__i] - 1]);
                          __lmax_seq = __i;
                        }
                    }
                }
            }

          _SeqNumber __i;
          for (__i = 0; __i < __m; __i++)
            {
              _DifferenceType __middle = (__b[__i] + __a[__i]) / 2;
              if (__lmax && __middle < __ns[__i] &&
                  __lcomp(std::make_pair(__S(__i)[__middle], __i),
                        std::make_pair(*__lmax, __lmax_seq)))
                __a[__i] = std::min(__a[__i] + __n + 1, __ns[__i]);
              else
                __b[__i] -= __n + 1;
            }

          _DifferenceType __leftsize = 0;
          for (_SeqNumber __i = 0; __i < __m; __i++)
              __leftsize += __a[__i] / (__n + 1);

          _DifferenceType __skew = __rank / (__n + 1) - __leftsize;

          if (__skew > 0)
            {
              // Move to the left, find smallest.
              std::priority_queue<std::pair<_ValueType, _SeqNumber>,
                std::vector<std::pair<_ValueType, _SeqNumber> >,
                _LexicographicReverse<_ValueType, _SeqNumber, _Compare> >
                __pq(__lrcomp);
              
              for (_SeqNumber __i = 0; __i < __m; __i++)
                if (__b[__i] < __ns[__i])
                  __pq.push(std::make_pair(__S(__i)[__b[__i]], __i));

              for (; __skew != 0 && !__pq.empty(); --__skew)
                {
                  _SeqNumber __source = __pq.top().second;
                  __pq.pop();

                  __a[__source]
                      = std::min(__a[__source] + __n + 1, __ns[__source]);
                  __b[__source] += __n + 1;

                  if (__b[__source] < __ns[__source])
                    __pq.push(
                      std::make_pair(__S(__source)[__b[__source]], __source));
                }
            }
          else if (__skew < 0)
            {
              // Move to the right, find greatest.
              std::priority_queue<std::pair<_ValueType, _SeqNumber>,
                std::vector<std::pair<_ValueType, _SeqNumber> >,
                _Lexicographic<_ValueType, _SeqNumber, _Compare> >
                  __pq(__lcomp);

              for (_SeqNumber __i = 0; __i < __m; __i++)
                if (__a[__i] > 0)
                  __pq.push(std::make_pair(__S(__i)[__a[__i] - 1], __i));

              for (; __skew != 0; ++__skew)
                {
                  _SeqNumber __source = __pq.top().second;
                  __pq.pop();

                  __a[__source] -= __n + 1;
                  __b[__source] -= __n + 1;

                  if (__a[__source] > 0)
                    __pq.push(std::make_pair(
                        __S(__source)[__a[__source] - 1], __source));
                }
            }
        }

      // Postconditions:
      // __a[__i] == __b[__i] in most cases, except when __a[__i] has been
      // clamped because of having reached the boundary

      // Now return the result, calculate the offset.

      // Compare the keys on both edges of the border.

      // Maximum of left edge, minimum of right edge.
      _ValueType* __maxleft = 0;
      _ValueType* __minright = 0;
      for (_SeqNumber __i = 0; __i < __m; __i++)
        {
          if (__a[__i] > 0)
            {
              if (!__maxleft)
                __maxleft = &(__S(__i)[__a[__i] - 1]);
              else
                {
                  // Max, favor rear sequences.
                  if (!__comp(__S(__i)[__a[__i] - 1], *__maxleft))
                    __maxleft = &(__S(__i)[__a[__i] - 1]);
                }
            }
          if (__b[__i] < __ns[__i])
            {
              if (!__minright)
                __minright = &(__S(__i)[__b[__i]]);
              else
                {
                  // Min, favor fore sequences.
                  if (__comp(__S(__i)[__b[__i]], *__minright))
                    __minright = &(__S(__i)[__b[__i]]);
                }
            }
        }

      _SeqNumber __seq = 0;
      for (_SeqNumber __i = 0; __i < __m; __i++)
        __begin_offsets[__i] = __S(__i) + __a[__i];

      delete[] __ns;
      delete[] __a;
      delete[] __b;
    }


  /** 
   *  @brief Selects the element at a certain global __rank from several
   *  sorted sequences.
   *
   *  The sequences are passed via a sequence of random-access
   *  iterator pairs, none of the sequences may be empty.
   *  @param __begin_seqs Begin of the sequence of iterator pairs.
   *  @param __end_seqs End of the sequence of iterator pairs.
   *  @param __rank The global rank to partition at.
   *  @param __offset The rank of the selected element in the global
   *  subsequence of elements equal to the selected element. If the
   *  selected element is unique, this number is 0.
   *  @param __comp The ordering functor, defaults to std::less. 
   */
  template<typename _Tp, typename _RanSeqs, typename _RankType,
           typename _Compare>
    _Tp
    multiseq_selection(_RanSeqs __begin_seqs, _RanSeqs __end_seqs,
                       _RankType __rank,
                       _RankType& __offset, _Compare __comp = std::less<_Tp>())
    {
      _GLIBCXX_CALL(__end_seqs - __begin_seqs)

      typedef typename std::iterator_traits<_RanSeqs>::value_type::first_type
        _It;
      typedef typename std::iterator_traits<_RanSeqs>::difference_type
        _SeqNumber;
      typedef typename std::iterator_traits<_It>::difference_type
        _DifferenceType;

      _Lexicographic<_Tp, _SeqNumber, _Compare> __lcomp(__comp);
      _LexicographicReverse<_Tp, _SeqNumber, _Compare> __lrcomp(__comp);

      // Number of sequences, number of elements in total (possibly
      // including padding).
      _DifferenceType __m = std::distance(__begin_seqs, __end_seqs);
      _DifferenceType __nn = 0;
      _DifferenceType __nmax, __n, __r;

      for (_SeqNumber __i = 0; __i < __m; __i++)
        __nn += std::distance(__begin_seqs[__i].first,
			      __begin_seqs[__i].second);

      if (__m == 0 || __nn == 0 || __rank < 0 || __rank >= __nn)
        {
          // result undefined if there is no data or __rank is outside bounds
          throw std::exception();
        }


      _DifferenceType* __ns = new _DifferenceType[__m];
      _DifferenceType* __a = new _DifferenceType[__m];
      _DifferenceType* __b = new _DifferenceType[__m];
      _DifferenceType __l;

      __ns[0] = std::distance(__begin_seqs[0].first, __begin_seqs[0].second);
      __nmax = __ns[0];
      for (_SeqNumber __i = 0; __i < __m; ++__i)
        {
          __ns[__i] = std::distance(__begin_seqs[__i].first,
                                    __begin_seqs[__i].second);
          __nmax = std::max(__nmax, __ns[__i]);
        }

      __r = __rd_log2(__nmax) + 1;

      // Pad all lists to this length, at least as long as any ns[__i],
      // equality iff __nmax = 2^__k - 1
      __l = __round_up_to_pow2(__r) - 1;

      for (_SeqNumber __i = 0; __i < __m; ++__i)
        {
          __a[__i] = 0;
          __b[__i] = __l;
        }
      __n = __l / 2;

      // Invariants:
      // 0 <= __a[__i] <= __ns[__i], 0 <= __b[__i] <= __l

#define __S(__i) (__begin_seqs[__i].first)

      // Initial partition.
      std::vector<std::pair<_Tp, _SeqNumber> > __sample;

      for (_SeqNumber __i = 0; __i < __m; __i++)
        if (__n < __ns[__i])
          __sample.push_back(std::make_pair(__S(__i)[__n], __i));
      __gnu_sequential::sort(__sample.begin(), __sample.end(),
                             __lcomp, sequential_tag());

      // Conceptual infinity.
      for (_SeqNumber __i = 0; __i < __m; __i++)
        if (__n >= __ns[__i])
          __sample.push_back(
            std::make_pair(__S(__i)[0] /*__dummy element*/, __i));

      _DifferenceType __localrank = __rank / __l;

      _SeqNumber __j;
      for (__j = 0;
           __j < __localrank && ((__n + 1) <= __ns[__sample[__j].second]);
           ++__j)
        __a[__sample[__j].second] += __n + 1;
      for (; __j < __m; ++__j)
        __b[__sample[__j].second] -= __n + 1;

      // Further refinement.
      while (__n > 0)
        {
          __n /= 2;

          const _Tp* __lmax = 0;
          for (_SeqNumber __i = 0; __i < __m; ++__i)
            {
              if (__a[__i] > 0)
                {
                  if (!__lmax)
                    __lmax = &(__S(__i)[__a[__i] - 1]);
                  else
                    {
                      if (__comp(*__lmax, __S(__i)[__a[__i] - 1]))      //max
                        __lmax = &(__S(__i)[__a[__i] - 1]);
                    }
                }
            }

          _SeqNumber __i;
          for (__i = 0; __i < __m; __i++)
            {
              _DifferenceType __middle = (__b[__i] + __a[__i]) / 2;
              if (__lmax && __middle < __ns[__i]
                  && __comp(__S(__i)[__middle], *__lmax))
                __a[__i] = std::min(__a[__i] + __n + 1, __ns[__i]);
              else
                __b[__i] -= __n + 1;
            }

          _DifferenceType __leftsize = 0;
          for (_SeqNumber __i = 0; __i < __m; ++__i)
              __leftsize += __a[__i] / (__n + 1);

          _DifferenceType __skew = __rank / (__n + 1) - __leftsize;

          if (__skew > 0)
            {
              // Move to the left, find smallest.
              std::priority_queue<std::pair<_Tp, _SeqNumber>,
                std::vector<std::pair<_Tp, _SeqNumber> >,
                _LexicographicReverse<_Tp, _SeqNumber, _Compare> >
                  __pq(__lrcomp);

              for (_SeqNumber __i = 0; __i < __m; ++__i)
                if (__b[__i] < __ns[__i])
                  __pq.push(std::make_pair(__S(__i)[__b[__i]], __i));

              for (; __skew != 0 && !__pq.empty(); --__skew)
                {
                  _SeqNumber __source = __pq.top().second;
                  __pq.pop();

                  __a[__source]
                      = std::min(__a[__source] + __n + 1, __ns[__source]);
                  __b[__source] += __n + 1;

                  if (__b[__source] < __ns[__source])
                    __pq.push(
                      std::make_pair(__S(__source)[__b[__source]], __source));
                }
            }
          else if (__skew < 0)
            {
              // Move to the right, find greatest.
              std::priority_queue<std::pair<_Tp, _SeqNumber>,
                std::vector<std::pair<_Tp, _SeqNumber> >,
                _Lexicographic<_Tp, _SeqNumber, _Compare> > __pq(__lcomp);

              for (_SeqNumber __i = 0; __i < __m; ++__i)
                if (__a[__i] > 0)
                  __pq.push(std::make_pair(__S(__i)[__a[__i] - 1], __i));

              for (; __skew != 0; ++__skew)
                {
                  _SeqNumber __source = __pq.top().second;
                  __pq.pop();

                  __a[__source] -= __n + 1;
                  __b[__source] -= __n + 1;

                  if (__a[__source] > 0)
                    __pq.push(std::make_pair(
                        __S(__source)[__a[__source] - 1], __source));
                }
            }
        }

      // Postconditions:
      // __a[__i] == __b[__i] in most cases, except when __a[__i] has been
      // clamped because of having reached the boundary

      // Now return the result, calculate the offset.

      // Compare the keys on both edges of the border.

      // Maximum of left edge, minimum of right edge.
      bool __maxleftset = false, __minrightset = false;

      // Impossible to avoid the warning?
      _Tp __maxleft, __minright;
      for (_SeqNumber __i = 0; __i < __m; ++__i)
        {
          if (__a[__i] > 0)
            {
              if (!__maxleftset)
                {
                  __maxleft = __S(__i)[__a[__i] - 1];
                  __maxleftset = true;
                }
              else
                {
                  // Max.
                  if (__comp(__maxleft, __S(__i)[__a[__i] - 1]))
                    __maxleft = __S(__i)[__a[__i] - 1];
                }
            }
          if (__b[__i] < __ns[__i])
            {
              if (!__minrightset)
                {
                  __minright = __S(__i)[__b[__i]];
                  __minrightset = true;
                }
              else
                {
                  // Min.
                  if (__comp(__S(__i)[__b[__i]], __minright))
                    __minright = __S(__i)[__b[__i]];
                }
            }
      }

      // Minright is the __splitter, in any case.

      if (!__maxleftset || __comp(__minright, __maxleft))
        {
          // Good luck, everything is split unambiguously.
          __offset = 0;
        }
      else
        {
          // We have to calculate an offset.
          __offset = 0;

          for (_SeqNumber __i = 0; __i < __m; ++__i)
            {
              _DifferenceType lb
                = std::lower_bound(__S(__i), __S(__i) + __ns[__i],
                                   __minright,
                                   __comp) - __S(__i);
              __offset += __a[__i] - lb;
            }
        }

      delete[] __ns;
      delete[] __a;
      delete[] __b;

      return __minright;
    }
}

#undef __S

#endif /* _GLIBCXX_PARALLEL_MULTISEQ_SELECTION_H */
