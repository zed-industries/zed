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

/** @file parallel/base.h
 *  @brief Sequential helper functions.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_BASE_H
#define _GLIBCXX_PARALLEL_BASE_H 1

#include <bits/c++config.h>
#include <bits/stl_function.h>
#include <omp.h>
#include <parallel/features.h>
#include <parallel/basic_iterator.h>
#include <parallel/parallel.h>

// Parallel mode namespaces.

/**
 * @namespace std::__parallel
 * @brief GNU parallel code, replaces standard behavior with parallel behavior.
 */
namespace std _GLIBCXX_VISIBILITY(default) 
{ 
  namespace __parallel { } 
}

/**
 * @namespace __gnu_parallel
 * @brief GNU parallel code for public use.
 */
namespace __gnu_parallel
{
  // Import all the parallel versions of components in namespace std.
  using namespace std::__parallel;
}

/**
 * @namespace __gnu_sequential
 * @brief GNU sequential classes for public use.
 */
namespace __gnu_sequential 
{ 
  // Import whatever is the serial version.
#ifdef _GLIBCXX_PARALLEL
  using namespace std::_GLIBCXX_STD_A;
#else
  using namespace std;
#endif   
}


namespace __gnu_parallel
{
  // NB: Including this file cannot produce (unresolved) symbols from
  // the OpenMP runtime unless the parallel mode is actually invoked
  // and active, which imples that the OpenMP runtime is actually
  // going to be linked in.
  inline _ThreadIndex
  __get_max_threads() 
  { 
    _ThreadIndex __i = omp_get_max_threads();
    return __i > 1 ? __i : 1; 
  }


  inline bool 
  __is_parallel(const _Parallelism __p) { return __p != sequential; }


  /** @brief Calculates the rounded-down logarithm of @c __n for base 2.
   *  @param __n Argument.
   *  @return Returns 0 for any argument <1.
   */
  template<typename _Size>
    inline _Size
    __rd_log2(_Size __n)
    {
      _Size __k;
      for (__k = 0; __n > 1; __n >>= 1)
        ++__k;
      return __k;
    }

  /** @brief Encode two integers into one gnu_parallel::_CASable.
   *  @param __a First integer, to be encoded in the most-significant @c
   *  _CASable_bits/2 bits.
   *  @param __b Second integer, to be encoded in the least-significant
   *  @c _CASable_bits/2 bits.
   *  @return value encoding @c __a and @c __b.
   *  @see __decode2
   */
  inline _CASable
  __encode2(int __a, int __b)     //must all be non-negative, actually
  {
    return (((_CASable)__a) << (_CASable_bits / 2)) | (((_CASable)__b) << 0);
  }

  /** @brief Decode two integers from one gnu_parallel::_CASable.
   *  @param __x __gnu_parallel::_CASable to decode integers from.
   *  @param __a First integer, to be decoded from the most-significant
   *  @c _CASable_bits/2 bits of @c __x.
   *  @param __b Second integer, to be encoded in the least-significant
   *  @c _CASable_bits/2 bits of @c __x.
   *  @see __encode2
   */
  inline void
  __decode2(_CASable __x, int& __a, int& __b)
  {
    __a = (int)((__x >> (_CASable_bits / 2)) & _CASable_mask);
    __b = (int)((__x >>               0 ) & _CASable_mask);
  }

  //needed for parallel "numeric", even if "algorithm" not included

  /** @brief Equivalent to std::min. */
  template<typename _Tp>
    inline const _Tp&
    min(const _Tp& __a, const _Tp& __b)
    { return (__a < __b) ? __a : __b; }

  /** @brief Equivalent to std::max. */
  template<typename _Tp>
    inline const _Tp&
    max(const _Tp& __a, const _Tp& __b)
    { return (__a > __b) ? __a : __b; }

  /** @brief Constructs predicate for equality from strict weak
   *  ordering predicate
   */
  template<typename _T1, typename _T2, typename _Compare>
    class _EqualFromLess : public std::binary_function<_T1, _T2, bool>
    {
    private:
      _Compare& _M_comp;

    public:
      _EqualFromLess(_Compare& __comp) : _M_comp(__comp) { }

      bool operator()(const _T1& __a, const _T2& __b)
      { return !_M_comp(__a, __b) && !_M_comp(__b, __a); }
    };


  /** @brief Similar to std::unary_negate,
   *  but giving the argument types explicitly. */
  template<typename _Predicate, typename argument_type>
    class __unary_negate
    : public std::unary_function<argument_type, bool>
    {
    protected:
      _Predicate _M_pred;

    public:
      explicit
      __unary_negate(const _Predicate& __x) : _M_pred(__x) { }

      bool
      operator()(const argument_type& __x)
      { return !_M_pred(__x); }
    };

  /** @brief Similar to std::binder1st,
   *  but giving the argument types explicitly. */
  template<typename _Operation, typename _FirstArgumentType,
	   typename _SecondArgumentType, typename _ResultType>
    class __binder1st
    : public std::unary_function<_SecondArgumentType, _ResultType>
    {
    protected:
      _Operation _M_op;
      _FirstArgumentType _M_value;

    public:
      __binder1st(const _Operation& __x, const _FirstArgumentType& __y)
      : _M_op(__x), _M_value(__y) { }

      _ResultType
      operator()(const _SecondArgumentType& __x)
      { return _M_op(_M_value, __x); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 109.  Missing binders for non-const sequence elements
      _ResultType
      operator()(_SecondArgumentType& __x) const
      { return _M_op(_M_value, __x); }
    };

  /**
   *  @brief Similar to std::binder2nd, but giving the argument types
   *  explicitly.
   */
  template<typename _Operation, typename _FirstArgumentType,
	   typename _SecondArgumentType, typename _ResultType>
    class __binder2nd
    : public std::unary_function<_FirstArgumentType, _ResultType>
    {
    protected:
      _Operation _M_op;
      _SecondArgumentType _M_value;

    public:
      __binder2nd(const _Operation& __x, const _SecondArgumentType& __y)
      : _M_op(__x), _M_value(__y) { }

      _ResultType
      operator()(const _FirstArgumentType& __x) const
      { return _M_op(__x, _M_value); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 109.  Missing binders for non-const sequence elements
      _ResultType
      operator()(_FirstArgumentType& __x)
      { return _M_op(__x, _M_value); }
    };

  /** @brief Similar to std::equal_to, but allows two different types. */
  template<typename _T1, typename _T2>
    struct _EqualTo : std::binary_function<_T1, _T2, bool>
    {
      bool operator()(const _T1& __t1, const _T2& __t2) const
      { return __t1 == __t2; }
    };

  /** @brief Similar to std::less, but allows two different types. */
  template<typename _T1, typename _T2>
    struct _Less : std::binary_function<_T1, _T2, bool>
    {
      bool
      operator()(const _T1& __t1, const _T2& __t2) const
      { return __t1 < __t2; }

      bool
      operator()(const _T2& __t2, const _T1& __t1) const
      { return __t2 < __t1; }
    };

  // Partial specialization for one type. Same as std::less.
  template<typename _Tp>
    struct _Less<_Tp, _Tp>
    : public std::less<_Tp> { };

  /** @brief Similar to std::plus, but allows two different types. */
  template<typename _Tp1, typename _Tp2, typename _Result
	   = __typeof__(*static_cast<_Tp1*>(0)
			+ *static_cast<_Tp2*>(0))>
    struct _Plus : public std::binary_function<_Tp1, _Tp2, _Result>
    {
      _Result
      operator()(const _Tp1& __x, const _Tp2& __y) const
      { return __x + __y; }
    };

  // Partial specialization for one type. Same as std::plus.
  template<typename _Tp>
    struct _Plus<_Tp, _Tp, _Tp>
    : public std::plus<_Tp> { };

  /** @brief Similar to std::multiplies, but allows two different types. */
  template<typename _Tp1, typename _Tp2, typename _Result
	   = __typeof__(*static_cast<_Tp1*>(0)
			* *static_cast<_Tp2*>(0))>
    struct _Multiplies : public std::binary_function<_Tp1, _Tp2, _Result>
    {
      _Result
      operator()(const _Tp1& __x, const _Tp2& __y) const
      { return __x * __y; }
    };

  // Partial specialization for one type. Same as std::multiplies.
  template<typename _Tp>
    struct _Multiplies<_Tp, _Tp, _Tp>
    : public std::multiplies<_Tp> { };

  /** @brief _Iterator associated with __gnu_parallel::_PseudoSequence.
   *  If features the usual random-access iterator functionality.
   *  @param _Tp Sequence _M_value type.
   *  @param _DifferenceTp Sequence difference type.
   */
  template<typename _Tp, typename _DifferenceTp>
    class _PseudoSequenceIterator
    {
    public:
      typedef _DifferenceTp _DifferenceType;

      _PseudoSequenceIterator(const _Tp& __val, _DifferenceType __pos)
      : _M_val(__val), _M_pos(__pos) { }

      // Pre-increment operator.
      _PseudoSequenceIterator&
      operator++()
      {
	++_M_pos;
	return *this;
      }

      // Post-increment operator.
      _PseudoSequenceIterator
      operator++(int)
      { return _PseudoSequenceIterator(_M_pos++); }

      const _Tp&
      operator*() const
      { return _M_val; }

      const _Tp&
      operator[](_DifferenceType) const
      { return _M_val; }

      bool
      operator==(const _PseudoSequenceIterator& __i2)
      { return _M_pos == __i2._M_pos; }

      bool
      operator!=(const _PseudoSequenceIterator& __i2)
      { return _M_pos != __i2._M_pos; }

      _DifferenceType
      operator-(const _PseudoSequenceIterator& __i2)
      { return _M_pos - __i2._M_pos; }

    private:
      const _Tp& _M_val;
      _DifferenceType _M_pos;
    };

  /** @brief Sequence that conceptually consists of multiple copies of
      the same element.
      *  The copies are not stored explicitly, of course.
      *  @param _Tp Sequence _M_value type.
      *  @param _DifferenceTp Sequence difference type.
      */
  template<typename _Tp, typename _DifferenceTp>
    class _PseudoSequence
    {
    public:
      typedef _DifferenceTp _DifferenceType;

      // Better cast down to uint64_t, than up to _DifferenceTp.
      typedef _PseudoSequenceIterator<_Tp, uint64_t> iterator;

      /** @brief Constructor.
       *  @param __val Element of the sequence.
       *  @param __count Number of (virtual) copies.
       */
      _PseudoSequence(const _Tp& __val, _DifferenceType __count)
      : _M_val(__val), _M_count(__count)  { }

      /** @brief Begin iterator. */
      iterator
      begin() const
      { return iterator(_M_val, 0); }

      /** @brief End iterator. */
      iterator
      end() const
      { return iterator(_M_val, _M_count); }

    private:
      const _Tp& _M_val;
      _DifferenceType _M_count;
    };

  /** @brief Compute the median of three referenced elements,
      according to @c __comp.
      *  @param __a First iterator.
      *  @param __b Second iterator.
      *  @param __c Third iterator.
      *  @param __comp Comparator.
      */
  template<typename _RAIter, typename _Compare>
    _RAIter
    __median_of_three_iterators(_RAIter __a, _RAIter __b,
				_RAIter __c, _Compare __comp)
    {
      if (__comp(*__a, *__b))
	if (__comp(*__b, *__c))
	  return __b;
	else
	  if (__comp(*__a, *__c))
	    return __c;
	  else
	    return __a;
      else
	{
	  // Just swap __a and __b.
	  if (__comp(*__a, *__c))
	    return __a;
	  else
	    if (__comp(*__b, *__c))
	      return __c;
	    else
	      return __b;
	}
    }

#if _GLIBCXX_PARALLEL_ASSERTIONS && defined(__glibcxx_assert_impl)
#define _GLIBCXX_PARALLEL_ASSERT(_Condition) __glibcxx_assert_impl(_Condition)
#else
#define _GLIBCXX_PARALLEL_ASSERT(_Condition)
#endif

} //namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_BASE_H */
