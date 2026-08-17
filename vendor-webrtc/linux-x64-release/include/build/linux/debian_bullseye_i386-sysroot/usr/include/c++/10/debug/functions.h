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

/** @file debug/functions.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_FUNCTIONS_H
#define _GLIBCXX_DEBUG_FUNCTIONS_H 1

#include <bits/stl_function.h>	// for less

#if __cplusplus >= 201103L
# include <bits/stl_iterator.h>	// for __miter_base
# include <type_traits>		// for is_lvalue_reference and conditional.
#endif

#include <debug/helper_functions.h>
#include <debug/formatter.h>

namespace __gnu_debug
{
  template<typename _Sequence>
    struct _Insert_range_from_self_is_safe
    { enum { __value = 0 }; };

  template<typename _Sequence>
    struct _Is_contiguous_sequence : std::__false_type { };

  /* Checks that [first, last) is a valid range, and then returns
   * __first. This routine is useful when we can't use a separate
   * assertion statement because, e.g., we are in a constructor.
  */
  template<typename _InputIterator>
    inline _InputIterator
    __check_valid_range(const _InputIterator& __first,
			const _InputIterator& __last,
			const char* __file,
			unsigned int __line,
			const char* __function)
    {
      __glibcxx_check_valid_range_at(__first, __last,
				     __file, __line, __function);
      return __first;
    }

  /* Handle the case where __other is a pointer to _Sequence::value_type. */
  template<typename _Iterator, typename _Sequence, typename _Category>
    inline bool
    __foreign_iterator_aux4(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	const typename _Sequence::value_type* __other)
    {
      typedef const typename _Sequence::value_type* _PointerType;
      typedef std::less<_PointerType> _Less;
#if __cplusplus >= 201103L
      constexpr _Less __l{};
#else
      const _Less __l = _Less();
#endif
      const _Sequence* __seq = __it._M_get_sequence();
      const _PointerType __begin = std::__addressof(*__seq->_M_base().begin());
      const _PointerType __end = std::__addressof(*(__seq->_M_base().end()-1));

      // Check whether __other points within the contiguous storage.
      return __l(__other, __begin) || __l(__end, __other);
    }

  /* Fallback overload for when we can't tell, assume it is valid. */
  template<typename _Iterator, typename _Sequence, typename _Category>
    inline bool
    __foreign_iterator_aux4(
	const _Safe_iterator<_Iterator, _Sequence, _Category>&, ...)
    { return true; }

  /* Handle sequences with contiguous storage */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _InputIterator>
    inline bool
    __foreign_iterator_aux3(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	const _InputIterator& __other, const _InputIterator& __other_end,
	std::__true_type)
    {
      if (__other == __other_end)
	return true;  // inserting nothing is safe even if not foreign iters
      if (__it._M_get_sequence()->empty())
	return true;  // can't be self-inserting if self is empty
      return __foreign_iterator_aux4(__it, std::__addressof(*__other));
    }

  /* Handle non-contiguous containers, assume it is valid. */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _InputIterator>
    inline bool
    __foreign_iterator_aux3(
	const _Safe_iterator<_Iterator, _Sequence, _Category>&,
	const _InputIterator&, const _InputIterator&,
	std::__false_type)
    { return true; }

  /** Handle debug iterators from the same type of container. */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _OtherIterator>
    inline bool
    __foreign_iterator_aux2(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	const _Safe_iterator<_OtherIterator, _Sequence, _Category>& __other,
	const _Safe_iterator<_OtherIterator, _Sequence, _Category>&)
    { return __it._M_get_sequence() != __other._M_get_sequence(); }

  /** Handle debug iterators from different types of container. */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _OtherIterator, typename _OtherSequence,
	   typename _OtherCategory>
    inline bool
    __foreign_iterator_aux2(
	const _Safe_iterator<_Iterator, _Sequence, _Category>&,
	const _Safe_iterator<_OtherIterator, _OtherSequence,
			     _OtherCategory>&,
	const _Safe_iterator<_OtherIterator, _OtherSequence,
			     _OtherCategory>&)
    { return true; }

  /* Handle non-debug iterators. */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _InputIterator>
    inline bool
    __foreign_iterator_aux2(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	const _InputIterator& __other,
	const _InputIterator& __other_end)
    {
#if __cplusplus < 201103L
      typedef _Is_contiguous_sequence<_Sequence> __tag;
#else
      using __lvalref = std::is_lvalue_reference<
	typename std::iterator_traits<_InputIterator>::reference>;
      using __contiguous = _Is_contiguous_sequence<_Sequence>;
      using __tag = typename std::conditional<__lvalref::value, __contiguous,
					      std::__false_type>::type;
#endif
      return __foreign_iterator_aux3(__it, __other, __other_end, __tag());
    }

  /* Handle the case where we aren't really inserting a range after all */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _Integral>
    inline bool
    __foreign_iterator_aux(
	const _Safe_iterator<_Iterator, _Sequence, _Category>&,
	_Integral, _Integral, std::__true_type)
    { return true; }

  /* Handle all iterators. */
  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _InputIterator>
    inline bool
    __foreign_iterator_aux(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	_InputIterator __other, _InputIterator __other_end,
	std::__false_type)
    {
      return _Insert_range_from_self_is_safe<_Sequence>::__value
	|| __foreign_iterator_aux2(__it, std::__miter_base(__other),
				   std::__miter_base(__other_end));
    }

  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _InputIterator>
    inline bool
    __foreign_iterator(
	const _Safe_iterator<_Iterator, _Sequence, _Category>& __it,
	_InputIterator __other, _InputIterator __other_end)
    {
      typedef typename std::__is_integer<_InputIterator>::__type _Integral;
      return __foreign_iterator_aux(__it, __other, __other_end, _Integral());
    }

  // Can't check if an input iterator sequence is sorted, because we
  // can't step through the sequence.
  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_aux(const _InputIterator&, const _InputIterator&,
                       std::input_iterator_tag)
    { return true; }

  // Can verify if a forward iterator sequence is in fact sorted using
  // std::__is_sorted
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_aux(_ForwardIterator __first, _ForwardIterator __last,
                       std::forward_iterator_tag)
    {
      if (__first == __last)
        return true;

      _ForwardIterator __next = __first;
      for (++__next; __next != __last; __first = __next, (void)++__next)
        if (*__next < *__first)
          return false;

      return true;
    }

  // Can't check if an input iterator sequence is sorted, because we can't step
  // through the sequence.
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_aux(const _InputIterator&, const _InputIterator&,
                       _Predicate, std::input_iterator_tag)
    { return true; }

  // Can verify if a forward iterator sequence is in fact sorted using
  // std::__is_sorted
  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_aux(_ForwardIterator __first, _ForwardIterator __last,
                       _Predicate __pred, std::forward_iterator_tag)
    {
      if (__first == __last)
        return true;

      _ForwardIterator __next = __first;
      for (++__next; __next != __last; __first = __next, (void)++__next)
        if (__pred(*__next, *__first))
          return false;

      return true;
    }

  // Determine if a sequence is sorted.
  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted(const _InputIterator& __first, const _InputIterator& __last)
    {
      return __check_sorted_aux(__first, __last,
				std::__iterator_category(__first));
    }

  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted(const _InputIterator& __first, const _InputIterator& __last,
                   _Predicate __pred)
    {
      return __check_sorted_aux(__first, __last, __pred,
				std::__iterator_category(__first));
    }

  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set_aux(const _InputIterator& __first,
			   const _InputIterator& __last,
			   std::__true_type)
    { return __check_sorted(__first, __last); }

  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set_aux(const _InputIterator&,
			   const _InputIterator&,
			   std::__false_type)
    { return true; }

  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set_aux(const _InputIterator& __first,
			   const _InputIterator& __last,
			   _Predicate __pred, std::__true_type)
    { return __check_sorted(__first, __last, __pred); }

  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set_aux(const _InputIterator&,
			   const _InputIterator&, _Predicate,
			   std::__false_type)
    { return true; }

  // ... special variant used in std::merge, std::includes, std::set_*.
  template<typename _InputIterator1, typename _InputIterator2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set(const _InputIterator1& __first,
		       const _InputIterator1& __last,
		       const _InputIterator2&)
    {
      typedef typename std::iterator_traits<_InputIterator1>::value_type
	_ValueType1;
      typedef typename std::iterator_traits<_InputIterator2>::value_type
	_ValueType2;

      typedef typename std::__are_same<_ValueType1, _ValueType2>::__type
	_SameType;
      return __check_sorted_set_aux(__first, __last, _SameType());
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_sorted_set(const _InputIterator1& __first,
		       const _InputIterator1& __last,
		       const _InputIterator2&, _Predicate __pred)
    {
      typedef typename std::iterator_traits<_InputIterator1>::value_type
	_ValueType1;
      typedef typename std::iterator_traits<_InputIterator2>::value_type
	_ValueType2;

      typedef typename std::__are_same<_ValueType1, _ValueType2>::__type
	_SameType;
      return __check_sorted_set_aux(__first, __last, __pred, _SameType());
   }

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 270. Binary search requirements overly strict
  // Determine if a sequence is partitioned w.r.t. this element.
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_partitioned_lower(_ForwardIterator __first,
			      _ForwardIterator __last, const _Tp& __value)
    {
      while (__first != __last && *__first < __value)
	++__first;
      if (__first != __last)
	{
	  ++__first;
	  while (__first != __last && !(*__first < __value))
	    ++__first;
	}
      return __first == __last;
    }

  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_partitioned_upper(_ForwardIterator __first,
			      _ForwardIterator __last, const _Tp& __value)
    {
      while (__first != __last && !(__value < *__first))
	++__first;
      if (__first != __last)
	{
	  ++__first;
	  while (__first != __last && __value < *__first)
	    ++__first;
	}
      return __first == __last;
    }

  // Determine if a sequence is partitioned w.r.t. this element.
  template<typename _ForwardIterator, typename _Tp, typename _Pred>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_partitioned_lower(_ForwardIterator __first,
			      _ForwardIterator __last, const _Tp& __value,
			      _Pred __pred)
    {
      while (__first != __last && bool(__pred(*__first, __value)))
	++__first;
      if (__first != __last)
	{
	  ++__first;
	  while (__first != __last && !bool(__pred(*__first, __value)))
	    ++__first;
	}
      return __first == __last;
    }

  template<typename _ForwardIterator, typename _Tp, typename _Pred>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __check_partitioned_upper(_ForwardIterator __first,
			      _ForwardIterator __last, const _Tp& __value,
			      _Pred __pred)
    {
      while (__first != __last && !bool(__pred(__value, *__first)))
	++__first;
      if (__first != __last)
	{
	  ++__first;
	  while (__first != __last && bool(__pred(__value, *__first)))
	    ++__first;
	}
      return __first == __last;
    }

#if __cplusplus >= 201103L
  struct _Irreflexive_checker
  {
    template<typename _It>
      static typename std::iterator_traits<_It>::reference
      __ref();

    template<typename _It,
	     typename = decltype(__ref<_It>() < __ref<_It>())>
      _GLIBCXX20_CONSTEXPR
      static bool
      _S_is_valid(_It __it)
      { return !(*__it < *__it); }

    // Fallback method if operator doesn't exist.
    template<typename... _Args>
      _GLIBCXX20_CONSTEXPR
      static bool
      _S_is_valid(_Args...)
      { return true; }

    template<typename _It, typename _Pred, typename
	= decltype(std::declval<_Pred>()(__ref<_It>(), __ref<_It>()))>
      _GLIBCXX20_CONSTEXPR
      static bool
      _S_is_valid_pred(_It __it, _Pred __pred)
      { return !__pred(*__it, *__it); }

    // Fallback method if predicate can't be invoked.
    template<typename... _Args>
      _GLIBCXX20_CONSTEXPR
      static bool
      _S_is_valid_pred(_Args...)
      { return true; }
  };

  template<typename _Iterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_irreflexive(_Iterator __it)
    { return _Irreflexive_checker::_S_is_valid(__it); }

  template<typename _Iterator, typename _Pred>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_irreflexive_pred(_Iterator __it, _Pred __pred)
    { return _Irreflexive_checker::_S_is_valid_pred(__it, __pred); }
#endif

} // namespace __gnu_debug

#endif
