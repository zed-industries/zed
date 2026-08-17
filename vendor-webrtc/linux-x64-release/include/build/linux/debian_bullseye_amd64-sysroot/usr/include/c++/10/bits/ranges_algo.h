// Core algorithmic facilities -*- C++ -*-

// Copyright (C) 2020 Free Software Foundation, Inc.
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

/** @file bits/ranges_algo.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{algorithm}
 */

#ifndef _RANGES_ALGO_H
#define _RANGES_ALGO_H 1

#if __cplusplus > 201703L

#include <bits/ranges_algobase.h>
#include <bits/uniform_int_dist.h> // concept uniform_random_bit_generator

#if __cpp_lib_concepts
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace ranges
{
  namespace __detail
  {
    template<typename _Comp, typename _Proj>
      constexpr auto
      __make_comp_proj(_Comp& __comp, _Proj& __proj)
      {
	return [&] (auto&& __lhs, auto&& __rhs) -> bool {
	  using _TL = decltype(__lhs);
	  using _TR = decltype(__rhs);
	  return std::__invoke(__comp,
			       std::__invoke(__proj, std::forward<_TL>(__lhs)),
			       std::__invoke(__proj, std::forward<_TR>(__rhs)));
	};
      }

    template<typename _Pred, typename _Proj>
      constexpr auto
      __make_pred_proj(_Pred& __pred, _Proj& __proj)
      {
	return [&] <typename _Tp> (_Tp&& __arg) -> bool {
	  return std::__invoke(__pred,
			       std::__invoke(__proj, std::forward<_Tp>(__arg)));
	};
      }
  } // namespace __detail

  struct __all_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!(bool)std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __all_of_fn all_of{};

  struct __any_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return true;
	return false;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __any_of_fn any_of{};

  struct __none_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __none_of_fn none_of{};

  template<typename _Iter, typename _Fp>
    struct in_fun_result
    {
      [[no_unique_address]] _Iter in;
      [[no_unique_address]] _Fp fun;

      template<typename _Iter2, typename _F2p>
	requires convertible_to<const _Iter&, _Iter2>
	  && convertible_to<const _Fp&, _F2p>
	constexpr
	operator in_fun_result<_Iter2, _F2p>() const &
	{ return {in, fun}; }

      template<typename _Iter2, typename _F2p>
	requires convertible_to<_Iter, _Iter2> && convertible_to<_Fp, _F2p>
	constexpr
	operator in_fun_result<_Iter2, _F2p>() &&
	{ return {std::move(in), std::move(fun)}; }
    };

  template<typename _Iter, typename _Fp>
    using for_each_result = in_fun_result<_Iter, _Fp>;

  struct __for_each_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirectly_unary_invocable<projected<_Iter, _Proj>> _Fun>
      constexpr for_each_result<_Iter, _Fun>
      operator()(_Iter __first, _Sent __last, _Fun __f, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  std::__invoke(__f, std::__invoke(__proj, *__first));
	return { std::move(__first), std::move(__f) };
      }

    template<input_range _Range, typename _Proj = identity,
	     indirectly_unary_invocable<projected<iterator_t<_Range>, _Proj>>
	       _Fun>
      constexpr for_each_result<borrowed_iterator_t<_Range>, _Fun>
      operator()(_Range&& __r, _Fun __f, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__f), std::move(__proj));
      }
  };

  inline constexpr __for_each_fn for_each{};

  template<typename _Iter, typename _Fp>
    using for_each_n_result = in_fun_result<_Iter, _Fp>;

  struct __for_each_n_fn
  {
    template<input_iterator _Iter, typename _Proj = identity,
	     indirectly_unary_invocable<projected<_Iter, _Proj>> _Fun>
      constexpr for_each_n_result<_Iter, _Fun>
      operator()(_Iter __first, iter_difference_t<_Iter> __n,
		 _Fun __f, _Proj __proj = {}) const
      {
	if constexpr (random_access_iterator<_Iter>)
	  {
	    if (__n <= 0)
	      return {std::move(__first), std::move(__f)};
	    auto __last = __first + __n;
	    return ranges::for_each(std::move(__first), std::move(__last),
				    std::move(__f), std::move(__proj));
	  }
	else
	  {
	    while (__n-- > 0)
	      {
		std::__invoke(__f, std::__invoke(__proj, *__first));
		++__first;
	      }
	    return {std::move(__first), std::move(__f)};
	  }
      }
  };

  inline constexpr __for_each_n_fn for_each_n{};

  struct __find_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent, typename _Tp,
	     typename _Proj = identity>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>, const _Tp*>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	while (__first != __last
	    && !(std::__invoke(__proj, *__first) == __value))
	  ++__first;
	return __first;
      }

    template<input_range _Range, typename _Tp, typename _Proj = identity>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<iterator_t<_Range>, _Proj>,
					 const _Tp*>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__proj));
      }
  };

  inline constexpr __find_fn find{};

  struct __find_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	while (__first != __last
	    && !(bool)std::__invoke(__pred, std::__invoke(__proj, *__first)))
	  ++__first;
	return __first;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __find_if_fn find_if{};

  struct __find_if_not_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	while (__first != __last
	    && (bool)std::__invoke(__pred, std::__invoke(__proj, *__first)))
	  ++__first;
	return __first;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __find_if_not_fn find_if_not{};

  struct __find_first_of_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr _Iter1
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	for (; __first1 != __last1; ++__first1)
	  for (auto __iter = __first2; __iter != __last2; ++__iter)
	    if (std::__invoke(__pred,
			      std::__invoke(__proj1, *__first1),
			      std::__invoke(__proj2, *__iter)))
	      return __first1;
	return __first1;
      }

    template<input_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr borrowed_iterator_t<_Range1>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __find_first_of_fn find_first_of{};

  struct __count_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>,
					 const _Tp*>
      constexpr iter_difference_t<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = 0;
	for (; __first != __last; ++__first)
	  if (std::__invoke(__proj, *__first) == __value)
	    ++__n;
	return __n;
      }

    template<input_range _Range, typename _Tp, typename _Proj = identity>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<iterator_t<_Range>, _Proj>,
					 const _Tp*>
      constexpr range_difference_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__proj));
      }
  };

  inline constexpr __count_fn count{};

  struct __count_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr iter_difference_t<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = 0;
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    ++__n;
	return __n;
      }

    template<input_range _Range,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr range_difference_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __count_if_fn count_if{};

  template<typename _Iter1, typename _Iter2>
    struct in_in_result
    {
      [[no_unique_address]] _Iter1 in1;
      [[no_unique_address]] _Iter2 in2;

      template<typename _IIter1, typename _IIter2>
	requires convertible_to<const _Iter1&, _IIter1>
	  && convertible_to<const _Iter2&, _IIter2>
	constexpr
	operator in_in_result<_IIter1, _IIter2>() const &
	{ return {in1, in2}; }

      template<typename _IIter1, typename _IIter2>
	requires convertible_to<_Iter1, _IIter1>
	  && convertible_to<_Iter2, _IIter2>
	constexpr
	operator in_in_result<_IIter1, _IIter2>() &&
	{ return {std::move(in1), std::move(in2)}; }
    };

  template<typename _Iter1, typename _Iter2>
    using mismatch_result = in_in_result<_Iter1, _Iter2>;

  struct __mismatch_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr mismatch_result<_Iter1, _Iter2>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2
	       && (bool)std::__invoke(__pred,
				      std::__invoke(__proj1, *__first1),
				      std::__invoke(__proj2, *__first2)))
	{
	  ++__first1;
	  ++__first2;
	}
	return { std::move(__first1), std::move(__first2) };
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr mismatch_result<iterator_t<_Range1>, iterator_t<_Range2>>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __mismatch_fn mismatch{};

  struct __search_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr subrange<_Iter1>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if (__first1 == __last1 || __first2 == __last2)
	  return {__first1, __first1};

	for (;;)
	  {
	    for (;;)
	      {
		if (__first1 == __last1)
		  return {__first1, __first1};
		if (std::__invoke(__pred,
				  std::__invoke(__proj1, *__first1),
				  std::__invoke(__proj2, *__first2)))
		  break;
		++__first1;
	      }
	    auto __cur1 = __first1;
	    auto __cur2 = __first2;
	    for (;;)
	      {
		if (++__cur2 == __last2)
		  return {__first1, ++__cur1};
		if (++__cur1 == __last1)
		  return {__cur1, __cur1};
		if (!(bool)std::__invoke(__pred,
					 std::__invoke(__proj1, *__cur1),
					 std::__invoke(__proj2, *__cur2)))
		  {
		    ++__first1;
		    break;
		  }
	      }
	  }
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr borrowed_subrange_t<_Range1>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __search_fn search{};

  struct __search_n_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent, typename _Tp,
	     typename _Pred = ranges::equal_to, typename _Proj = identity>
      requires indirectly_comparable<_Iter, const _Tp*, _Pred, _Proj>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, iter_difference_t<_Iter> __count,
		 const _Tp& __value, _Pred __pred = {}, _Proj __proj = {}) const
      {
	if (__count <= 0)
	  return {__first, __first};

	auto __value_comp = [&] <typename _Rp> (_Rp&& __arg) {
	    return std::__invoke(__pred, std::forward<_Rp>(__arg), __value);
	};
	if (__count == 1)
	  {
	    __first = ranges::find_if(std::move(__first), __last,
				      std::move(__value_comp),
				      std::move(__proj));
	    if (__first == __last)
	      return {__first, __first};
	    else
	      {
		auto __end = __first;
		return {__first, ++__end};
	      }
	  }

	if constexpr (sized_sentinel_for<_Sent, _Iter>
		      && random_access_iterator<_Iter>)
	  {
	    auto __tail_size = __last - __first;
	    auto __remainder = __count;

	    while (__remainder <= __tail_size)
	      {
		__first += __remainder;
		__tail_size -= __remainder;
		auto __backtrack = __first;
		while (__value_comp(std::__invoke(__proj, *--__backtrack)))
		  {
		    if (--__remainder == 0)
		      return {__first - __count, __first};
		  }
		__remainder = __count + 1 - (__first - __backtrack);
	      }
	    auto __i = __first + __tail_size;
	    return {__i, __i};
	  }
	else
	  {
	    __first = ranges::find_if(__first, __last, __value_comp, __proj);
	    while (__first != __last)
	      {
		auto __n = __count;
		auto __i = __first;
		++__i;
		while (__i != __last && __n != 1
		       && __value_comp(std::__invoke(__proj, *__i)))
		  {
		    ++__i;
		    --__n;
		  }
		if (__n == 1)
		  return {__first, __i};
		if (__i == __last)
		  return {__i, __i};
		__first = ranges::find_if(++__i, __last, __value_comp, __proj);
	      }
	    return {__first, __first};
	  }
      }

    template<forward_range _Range, typename _Tp,
	     typename _Pred = ranges::equal_to, typename _Proj = identity>
      requires indirectly_comparable<iterator_t<_Range>, const _Tp*,
				     _Pred, _Proj>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, range_difference_t<_Range> __count,
	       const _Tp& __value, _Pred __pred = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__count), __value,
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __search_n_fn search_n{};

  struct __find_end_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr subrange<_Iter1>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if constexpr (bidirectional_iterator<_Iter1>
		      && bidirectional_iterator<_Iter2>)
	  {
	    auto __i1 = ranges::next(__first1, __last1);
	    auto __i2 = ranges::next(__first2, __last2);
	    auto __rresult
	      = ranges::search(reverse_iterator<_Iter1>{__i1},
			       reverse_iterator<_Iter1>{__first1},
			       reverse_iterator<_Iter2>{__i2},
			       reverse_iterator<_Iter2>{__first2},
			       std::move(__pred),
			       std::move(__proj1), std::move(__proj2));
	    auto __result_first = ranges::end(__rresult).base();
	    auto __result_last = ranges::begin(__rresult).base();
	    if (__result_last == __first1)
	      return {__i1, __i1};
	    else
	      return {__result_first, __result_last};
	  }
	else
	  {
	    auto __i = ranges::next(__first1, __last1);
	    if (__first2 == __last2)
	      return {__i, __i};

	    auto __result_begin = __i;
	    auto __result_end = __i;
	    for (;;)
	      {
		auto __new_range = ranges::search(__first1, __last1,
						  __first2, __last2,
						  __pred, __proj1, __proj2);
		auto __new_result_begin = ranges::begin(__new_range);
		auto __new_result_end = ranges::end(__new_range);
		if (__new_result_begin == __last1)
		  return {__result_begin, __result_end};
		else
		  {
		    __result_begin = __new_result_begin;
		    __result_end = __new_result_end;
		    __first1 = __result_begin;
		    ++__first1;
		  }
	      }
	  }
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr borrowed_subrange_t<_Range1>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __find_end_fn find_end{};

  struct __adjacent_find_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_binary_predicate<projected<_Iter, _Proj>,
				       projected<_Iter, _Proj>> _Pred
	       = ranges::equal_to>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;
	auto __next = __first;
	for (; ++__next != __last; __first = __next)
	  {
	    if (std::__invoke(__pred,
			      std::__invoke(__proj, *__first),
			      std::__invoke(__proj, *__next)))
	      return __first;
	  }
	return __next;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_binary_predicate<
	       projected<iterator_t<_Range>, _Proj>,
	       projected<iterator_t<_Range>, _Proj>> _Pred = ranges::equal_to>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Pred __pred = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __adjacent_find_fn adjacent_find{};

  struct __is_permutation_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_equivalence_relation<projected<_Iter1, _Proj1>,
					   projected<_Iter2, _Proj2>> _Pred
	       = ranges::equal_to>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	constexpr bool __sized_iters
	  = (sized_sentinel_for<_Sent1, _Iter1>
	     && sized_sentinel_for<_Sent2, _Iter2>);
	if constexpr (__sized_iters)
	  {
	    auto __d1 = ranges::distance(__first1, __last1);
	    auto __d2 = ranges::distance(__first2, __last2);
	    if (__d1 != __d2)
	      return false;
	  }

	// Efficiently compare identical prefixes:  O(N) if sequences
	// have the same elements in the same order.
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2)
	  if (!(bool)std::__invoke(__pred,
				   std::__invoke(__proj1, *__first1),
				   std::__invoke(__proj2, *__first2)))
	      break;

	if constexpr (__sized_iters)
	  {
	    if (__first1 == __last1)
	      return true;
	  }
	else
	  {
	    auto __d1 = ranges::distance(__first1, __last1);
	    auto __d2 = ranges::distance(__first2, __last2);
	    if (__d1 == 0 && __d2 == 0)
	      return true;
	    if (__d1 != __d2)
	      return false;
	  }

	for (auto __scan = __first1; __scan != __last1; ++__scan)
	  {
	    auto __proj_scan = std::__invoke(__proj1, *__scan);
	    auto __comp_scan = [&] <typename _Tp> (_Tp&& __arg) {
	      return std::__invoke(__pred, __proj_scan,
				   std::forward<_Tp>(__arg));
	    };
	    if (__scan != ranges::find_if(__first1, __scan,
					  __comp_scan, __proj1))
	      continue; // We've seen this one before.

	    auto __matches = ranges::count_if(__first2, __last2,
					      __comp_scan, __proj2);
	    if (__matches == 0
		|| ranges::count_if(__scan, __last1,
				    __comp_scan, __proj1) != __matches)
	      return false;
	  }
	return true;
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range1>, _Proj1>,
	       projected<iterator_t<_Range2>, _Proj2>> _Pred = ranges::equal_to>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __is_permutation_fn is_permutation{};

  template<typename _Iter, typename _Out>
    using copy_if_result = in_out_result<_Iter, _Out>;

  struct __copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
      constexpr copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __copy_if_fn copy_if{};

  template<typename _Iter1, typename _Iter2>
    using swap_ranges_result = in_in_result<_Iter1, _Iter2>;

  struct __swap_ranges_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2>
      requires indirectly_swappable<_Iter1, _Iter2>
      constexpr swap_ranges_result<_Iter1, _Iter2>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2) const
      {
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2)
	  ranges::iter_swap(__first1, __first2);
	return {std::move(__first1), std::move(__first2)};
      }

    template<input_range _Range1, input_range _Range2>
      requires indirectly_swappable<iterator_t<_Range1>, iterator_t<_Range2>>
      constexpr swap_ranges_result<borrowed_iterator_t<_Range1>,
				   borrowed_iterator_t<_Range2>>
      operator()(_Range1&& __r1, _Range2&& __r2) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2));
      }
  };

  inline constexpr __swap_ranges_fn swap_ranges{};

  template<typename _Iter, typename _Out>
    using unary_transform_result = in_out_result<_Iter, _Out>;

  template<typename _Iter1, typename _Iter2, typename _Out>
    struct in_in_out_result
    {
      [[no_unique_address]] _Iter1 in1;
      [[no_unique_address]] _Iter2 in2;
      [[no_unique_address]] _Out  out;

      template<typename _IIter1, typename _IIter2, typename _OOut>
	requires convertible_to<const _Iter1&, _IIter1>
	  && convertible_to<const _Iter2&, _IIter2>
	  && convertible_to<const _Out&, _OOut>
	constexpr
	operator in_in_out_result<_IIter1, _IIter2, _OOut>() const &
	{ return {in1, in2, out}; }

      template<typename _IIter1, typename _IIter2, typename _OOut>
	requires convertible_to<_Iter1, _IIter1>
	  && convertible_to<_Iter2, _IIter2>
	  && convertible_to<_Out, _OOut>
	constexpr
	operator in_in_out_result<_IIter1, _IIter2, _OOut>() &&
	{ return {std::move(in1), std::move(in2), std::move(out)}; }
    };

  template<typename _Iter1, typename _Iter2, typename _Out>
    using binary_transform_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __transform_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out,
	     copy_constructible _Fp, typename _Proj = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<_Iter, _Proj>>>
      constexpr unary_transform_result<_Iter, _Out>
      operator()(_Iter __first1, _Sent __last1, _Out __result,
		 _Fp __op, _Proj __proj = {}) const
      {
	for (; __first1 != __last1; ++__first1, (void)++__result)
	  *__result = std::__invoke(__op, std::__invoke(__proj, *__first1));
	return {std::move(__first1), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     copy_constructible _Fp, typename _Proj = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<iterator_t<_Range>, _Proj>>>
      constexpr unary_transform_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result, _Fp __op, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__op), std::move(__proj));
      }

    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, copy_constructible _Fp,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<_Iter1, _Proj1>,
				     projected<_Iter2, _Proj2>>>
      constexpr binary_transform_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Fp __binary_op,
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2, ++__result)
	  *__result = std::__invoke(__binary_op,
				    std::__invoke(__proj1, *__first1),
				    std::__invoke(__proj2, *__first2));
	return {std::move(__first1), std::move(__first2), std::move(__result)};
      }

    template<input_range _Range1, input_range _Range2,
	     weakly_incrementable _Out, copy_constructible _Fp,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<iterator_t<_Range1>, _Proj1>,
				     projected<iterator_t<_Range2>, _Proj2>>>
      constexpr binary_transform_result<borrowed_iterator_t<_Range1>,
					borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result, _Fp __binary_op,
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__binary_op),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __transform_fn transform{};

  struct __replace_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp1, typename _Tp2, typename _Proj = identity>
      requires indirectly_writable<_Iter, const _Tp2&>
	&& indirect_binary_predicate<ranges::equal_to, projected<_Iter, _Proj>,
				     const _Tp1*>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__proj, *__first) == __old_value)
	    *__first = __new_value;
	return __first;
      }

    template<input_range _Range,
	     typename _Tp1, typename _Tp2, typename _Proj = identity>
      requires indirectly_writable<iterator_t<_Range>, const _Tp2&>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp1*>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __old_value, __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_fn replace{};

  struct __replace_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_writable<_Iter, const _Tp&>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    *__first = __new_value;
	return std::move(__first);
      }

    template<input_range _Range, typename _Tp, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_writable<iterator_t<_Range>, const _Tp&>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_if_fn replace_if{};

  template<typename _Iter, typename _Out>
    using replace_copy_result = in_out_result<_Iter, _Out>;

  struct __replace_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp1, typename _Tp2, output_iterator<const _Tp2&> _Out,
	     typename _Proj = identity>
      requires indirectly_copyable<_Iter, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<_Iter, _Proj>, const _Tp1*>
      constexpr replace_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first, (void)++__result)
	  if (std::__invoke(__proj, *__first) == __old_value)
	    *__result = __new_value;
	  else
	    *__result = *__first;
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, typename _Tp1, typename _Tp2,
	     output_iterator<const _Tp2&> _Out, typename _Proj = identity>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp1*>
      constexpr replace_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), __old_value,
		       __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_copy_fn replace_copy{};

  template<typename _Iter, typename _Out>
    using replace_copy_if_result = in_out_result<_Iter, _Out>;

  struct __replace_copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, output_iterator<const _Tp&> _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
      constexpr replace_copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first, (void)++__result)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    *__result = __new_value;
	  else
	    *__result = *__first;
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range,
	     typename _Tp, output_iterator<const _Tp&> _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr replace_copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), std::move(__pred),
		       __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_copy_if_fn replace_copy_if{};

  struct __generate_n_fn
  {
    template<input_or_output_iterator _Out, copy_constructible _Fp>
      requires invocable<_Fp&>
	&& indirectly_writable<_Out, invoke_result_t<_Fp&>>
      constexpr _Out
      operator()(_Out __first, iter_difference_t<_Out> __n, _Fp __gen) const
      {
	for (; __n > 0; --__n, (void)++__first)
	  *__first = std::__invoke(__gen);
	return __first;
      }
  };

  inline constexpr __generate_n_fn generate_n{};

  struct __generate_fn
  {
    template<input_or_output_iterator _Out, sentinel_for<_Out> _Sent,
	     copy_constructible _Fp>
      requires invocable<_Fp&>
	&& indirectly_writable<_Out, invoke_result_t<_Fp&>>
      constexpr _Out
      operator()(_Out __first, _Sent __last, _Fp __gen) const
      {
	for (; __first != __last; ++__first)
	  *__first = std::__invoke(__gen);
	return __first;
      }

    template<typename _Range, copy_constructible _Fp>
      requires invocable<_Fp&> && output_range<_Range, invoke_result_t<_Fp&>>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Fp __gen) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__gen));
      }
  };

  inline constexpr __generate_fn generate{};

  struct __remove_if_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	__first = ranges::find_if(__first, __last, __pred, __proj);
	if (__first == __last)
	  return {__first, __first};

	auto __result = __first;
	++__first;
	for (; __first != __last; ++__first)
	  if (!std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = std::move(*__first);
	      ++__result;
	    }

	return {__result, __first};
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __remove_if_fn remove_if{};

  struct __remove_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>,
					 const _Tp*>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	auto __pred = [&] (auto&& __arg) {
	  return std::forward<decltype(__arg)>(__arg) == __value;
	};
	return ranges::remove_if(__first, __last,
				 std::move(__pred), std::move(__proj));
      }

    template<forward_range _Range, typename _Tp, typename _Proj = identity>
      requires permutable<iterator_t<_Range>>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp*>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__proj));
      }
  };

  inline constexpr __remove_fn remove{};

  template<typename _Iter, typename _Out>
    using remove_copy_if_result = in_out_result<_Iter, _Out>;

  struct __remove_copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
      constexpr remove_copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr remove_copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __remove_copy_if_fn remove_copy_if{};

  template<typename _Iter, typename _Out>
    using remove_copy_result = in_out_result<_Iter, _Out>;

  struct __remove_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Tp, typename _Proj = identity>
      requires indirectly_copyable<_Iter, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<_Iter, _Proj>,
				     const _Tp*>
      constexpr remove_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!(std::__invoke(__proj, *__first) == __value))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Tp, typename _Proj = identity>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp*>
      constexpr remove_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), __value, std::move(__proj));
      }
  };

  inline constexpr __remove_copy_fn remove_copy{};

  struct __unique_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<_Iter, _Proj>> _Comp = ranges::equal_to>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	__first = ranges::adjacent_find(__first, __last, __comp, __proj);
	if (__first == __last)
	  return {__first, __first};

	auto __dest = __first;
	++__first;
	while (++__first != __last)
	  if (!std::__invoke(__comp,
			     std::__invoke(__proj, *__dest),
			     std::__invoke(__proj, *__first)))
	    *++__dest = std::move(*__first);
	return {++__dest, __first};
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range>, _Proj>> _Comp = ranges::equal_to>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __unique_fn unique{};

  template<typename _Iter, typename _Out>
    using unique_copy_result = in_out_result<_Iter, _Out>;

  struct __unique_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<_Iter, _Proj>> _Comp = ranges::equal_to>
      requires indirectly_copyable<_Iter, _Out>
	&& (forward_iterator<_Iter>
	    || (input_iterator<_Out>
		&& same_as<iter_value_t<_Iter>, iter_value_t<_Out>>)
	    || indirectly_copyable_storable<_Iter, _Out>)
      constexpr unique_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), std::move(__result)};

	// TODO: perform a closer comparison with reference implementations
	if constexpr (forward_iterator<_Iter>)
	  {
	    auto __next = __first;
	    *__result = *__next;
	    while (++__next != __last)
	      if (!std::__invoke(__comp,
				 std::__invoke(__proj, *__first),
				 std::__invoke(__proj, *__next)))
		{
		  __first = __next;
		  *++__result = *__first;
		}
	    return {__next, std::move(++__result)};
	  }
	else if constexpr (input_iterator<_Out>
			   && same_as<iter_value_t<_Iter>, iter_value_t<_Out>>)
	  {
	    *__result = *__first;
	    while (++__first != __last)
	      if (!std::__invoke(__comp,
				 std::__invoke(__proj, *__result),
				 std::__invoke(__proj, *__first)))
		  *++__result = *__first;
	    return {std::move(__first), std::move(++__result)};
	  }
	else // indirectly_copyable_storable<_Iter, _Out>
	  {
	    auto __value = *__first;
	    *__result = __value;
	    while (++__first != __last)
	      {
		if (!(bool)std::__invoke(__comp,
					 std::__invoke(__proj, *__first),
					 std::__invoke(__proj, __value)))
		  {
		    __value = *__first;
		    *++__result = __value;
		  }
	      }
	    return {std::move(__first), std::move(++__result)};
	  }
      }

    template<input_range _Range,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range>, _Proj>> _Comp = ranges::equal_to>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& (forward_iterator<iterator_t<_Range>>
	    || (input_iterator<_Out>
		&& same_as<range_value_t<_Range>, iter_value_t<_Out>>)
	    || indirectly_copyable_storable<iterator_t<_Range>, _Out>)
      constexpr unique_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __unique_copy_fn unique_copy{};

  struct __reverse_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent>
      requires permutable<_Iter>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last) const
      {
	auto __i = ranges::next(__first, __last);
	auto __tail = __i;

	if constexpr (random_access_iterator<_Iter>)
	  {
	    if (__first != __last)
	      {
		--__tail;
		while (__first < __tail)
		  {
		    ranges::iter_swap(__first, __tail);
		    ++__first;
		    --__tail;
		  }
	      }
	    return __i;
	  }
	else
	  {
	    for (;;)
	      if (__first == __tail || __first == --__tail)
		break;
	      else
		{
		  ranges::iter_swap(__first, __tail);
		  ++__first;
		}
	    return __i;
	  }
      }

    template<bidirectional_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r));
      }
  };

  inline constexpr __reverse_fn reverse{};

  template<typename _Iter, typename _Out>
    using reverse_copy_result = in_out_result<_Iter, _Out>;

  struct __reverse_copy_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr reverse_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result) const
      {
	auto __i = ranges::next(__first, __last);
	auto __tail = __i;
	while (__first != __tail)
	  {
	    --__tail;
	    *__result = *__tail;
	    ++__result;
	  }
	return {__i, __result};
      }

    template<bidirectional_range _Range, weakly_incrementable _Out>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr reverse_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __reverse_copy_fn reverse_copy{};

  struct __rotate_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Iter __middle, _Sent __last) const
      {
	auto __lasti = ranges::next(__first, __last);
	if (__first == __middle)
	  return {__lasti, __lasti};
	if (__last == __middle)
	  return {std::move(__first), std::move(__lasti)};

	if constexpr (random_access_iterator<_Iter>)
	  {
	    auto __n = __lasti - __first;
	    auto __k = __middle - __first;

	    if (__k == __n - __k)
	      {
		ranges::swap_ranges(__first, __middle, __middle, __middle + __k);
		return {std::move(__middle), std::move(__lasti)};
	      }

	    auto __p = __first;
	    auto __ret = __first + (__lasti - __middle);

	    for (;;)
	      {
		if (__k < __n - __k)
		  {
		    // TODO: is_pod is deprecated, but this condition is
		    // consistent with the STL implementation.
		    if constexpr (__is_pod(iter_value_t<_Iter>))
		      if (__k == 1)
			{
			  auto __t = std::move(*__p);
			  ranges::move(__p + 1, __p + __n, __p);
			  *(__p + __n - 1) = std::move(__t);
			  return {std::move(__ret), std::move(__lasti)};
			}
		    auto __q = __p + __k;
		    for (decltype(__n) __i = 0; __i < __n - __k; ++ __i)
		      {
			ranges::iter_swap(__p, __q);
			++__p;
			++__q;
		      }
		    __n %= __k;
		    if (__n == 0)
		      return {std::move(__ret), std::move(__lasti)};
		    ranges::swap(__n, __k);
		    __k = __n - __k;
		  }
		else
		  {
		    __k = __n - __k;
		    // TODO: is_pod is deprecated, but this condition is
		    // consistent with the STL implementation.
		    if constexpr (__is_pod(iter_value_t<_Iter>))
		      if (__k == 1)
			{
			  auto __t = std::move(*(__p + __n - 1));
			  ranges::move_backward(__p, __p + __n - 1, __p + __n);
			  *__p = std::move(__t);
			  return {std::move(__ret), std::move(__lasti)};
			}
		    auto __q = __p + __n;
		    __p = __q - __k;
		    for (decltype(__n) __i = 0; __i < __n - __k; ++ __i)
		      {
			--__p;
			--__q;
			ranges::iter_swap(__p, __q);
		      }
		    __n %= __k;
		    if (__n == 0)
		      return {std::move(__ret), std::move(__lasti)};
		    std::swap(__n, __k);
		  }
	      }
	  }
	else if constexpr (bidirectional_iterator<_Iter>)
	  {
	    auto __tail = __lasti;

	    ranges::reverse(__first, __middle);
	    ranges::reverse(__middle, __tail);

	    while (__first != __middle && __middle != __tail)
	      {
		ranges::iter_swap(__first, --__tail);
		++__first;
	      }

	    if (__first == __middle)
	      {
		ranges::reverse(__middle, __tail);
		return {std::move(__tail), std::move(__lasti)};
	      }
	    else
	      {
		ranges::reverse(__first, __middle);
		return {std::move(__first), std::move(__lasti)};
	      }
	  }
	else
	  {
	    auto __first2 = __middle;
	    do
	      {
		ranges::iter_swap(__first, __first2);
		++__first;
		++__first2;
		if (__first == __middle)
		  __middle = __first2;
	      } while (__first2 != __last);

	    auto __ret = __first;

	    __first2 = __middle;

	    while (__first2 != __last)
	      {
		ranges::iter_swap(__first, __first2);
		++__first;
		++__first2;
		if (__first == __middle)
		  __middle = __first2;
		else if (__first2 == __last)
		  __first2 = __middle;
	      }
	    return {std::move(__ret), std::move(__lasti)};
	  }
      }

    template<forward_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r));
      }
  };

  inline constexpr __rotate_fn rotate{};

  template<typename _Iter, typename _Out>
    using rotate_copy_result = in_out_result<_Iter, _Out>;

  struct __rotate_copy_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr rotate_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Out __result) const
      {
	auto __copy1 = ranges::copy(__middle,
				    std::move(__last),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first),
				    std::move(__middle),
				    std::move(__copy1.out));
	return { std::move(__copy1.in), std::move(__copy2.out) };
      }

    template<forward_range _Range, weakly_incrementable _Out>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr rotate_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, iterator_t<_Range> __middle, _Out __result) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r), std::move(__result));
      }
  };

  inline constexpr __rotate_copy_fn rotate_copy{};

  struct __sample_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Gen>
      requires (forward_iterator<_Iter> || random_access_iterator<_Out>)
	&& indirectly_copyable<_Iter, _Out>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Out
      operator()(_Iter __first, _Sent __last, _Out __out,
		 iter_difference_t<_Iter> __n, _Gen&& __g) const
      {
	if constexpr (forward_iterator<_Iter>)
	  {
	    // FIXME: Forwarding to std::sample here requires computing __lasti
	    // which may take linear time.
	    auto __lasti = ranges::next(__first, __last);
	    return std::sample(std::move(__first), std::move(__lasti),
			       std::move(__out), __n, std::forward<_Gen>(__g));
	  }
	else
	  {
	    using __distrib_type
	      = uniform_int_distribution<iter_difference_t<_Iter>>;
	    using __param_type = typename __distrib_type::param_type;
	    __distrib_type __d{};
	    iter_difference_t<_Iter> __sample_sz = 0;
	    while (__first != __last && __sample_sz != __n)
	      {
		__out[__sample_sz++] = *__first;
		++__first;
	      }
	    for (auto __pop_sz = __sample_sz; __first != __last;
		++__first, (void) ++__pop_sz)
	      {
		const auto __k = __d(__g, __param_type{0, __pop_sz});
		if (__k < __n)
		  __out[__k] = *__first;
	      }
	    return __out + __sample_sz;
	  }
      }

    template<input_range _Range, weakly_incrementable _Out, typename _Gen>
      requires (forward_range<_Range> || random_access_iterator<_Out>)
	&& indirectly_copyable<iterator_t<_Range>, _Out>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Out
      operator()(_Range&& __r, _Out __out,
		 range_difference_t<_Range> __n, _Gen&& __g) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__out), __n,
		       std::forward<_Gen>(__g));
      }
  };

  inline constexpr __sample_fn sample{};

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  struct __shuffle_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Gen>
      requires permutable<_Iter>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Iter
      operator()(_Iter __first, _Sent __last, _Gen&& __g) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::shuffle(std::move(__first), __lasti, std::forward<_Gen>(__g));
	return __lasti;
      }

    template<random_access_range _Range, typename _Gen>
      requires permutable<iterator_t<_Range>>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Gen&& __g) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::forward<_Gen>(__g));
      }
  };

  inline constexpr __shuffle_fn shuffle{};
#endif

  struct __push_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::push_heap(__first, __lasti,
		       __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __push_heap_fn push_heap{};

  struct __pop_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::pop_heap(__first, __lasti,
		      __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __pop_heap_fn pop_heap{};

  struct __make_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::make_heap(__first, __lasti,
		       __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __make_heap_fn make_heap{};

  struct __sort_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::sort_heap(__first, __lasti,
		       __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __sort_heap_fn sort_heap{};

  struct __is_heap_until_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = ranges::distance(__first, __last);
	iter_difference_t<_Iter> __parent = 0, __child = 1;
	for (; __child < __n; ++__child)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *(__first + __parent)),
			    std::__invoke(__proj, *(__first + __child))))
	    return __first + __child;
	  else if ((__child & 1) == 0)
	    ++__parent;

	return __first + __n;
      }

    template<random_access_range _Range,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_heap_until_fn is_heap_until{};

  struct __is_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (__last
		== ranges::is_heap_until(__first, __last,
					 std::move(__comp),
					 std::move(__proj)));
      }

    template<random_access_range _Range,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_heap_fn is_heap{};

  struct __sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::sort(std::move(__first), __lasti,
		  __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __sort_fn sort{};

  struct __stable_sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::stable_sort(std::move(__first), __lasti,
			 __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __stable_sort_fn stable_sort{};

  struct __partial_sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __middle)
	  return ranges::next(__first, __last);

	ranges::make_heap(__first, __middle, __comp, __proj);
	auto __i = __middle;
	for (; __i != __last; ++__i)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__i),
			    std::__invoke(__proj, *__first)))
	    {
	      ranges::pop_heap(__first, __middle, __comp, __proj);
	      ranges::iter_swap(__middle-1, __i);
	      ranges::push_heap(__first, __middle, __comp, __proj);
	    }
	ranges::sort_heap(__first, __middle, __comp, __proj);

	return __i;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __partial_sort_fn partial_sort{};

  template<typename _Iter, typename _Out>
    using partial_sort_copy_result = in_out_result<_Iter, _Out>;

  struct __partial_sort_copy_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     random_access_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_copyable<_Iter1, _Iter2>
	&& sortable<_Iter2, _Comp, _Proj2>
	&& indirect_strict_weak_order<_Comp,
				      projected<_Iter1, _Proj1>,
				      projected<_Iter2, _Proj2>>
      constexpr partial_sort_copy_result<_Iter1, _Iter2>
      operator()(_Iter1 __first, _Sent1 __last,
		 _Iter2 __result_first, _Sent2 __result_last,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if (__result_first == __result_last)
	  {
	    // TODO: Eliminating the variable __lasti triggers an ICE.
	    auto __lasti = ranges::next(std::move(__first),
					std::move(__last));
	    return {std::move(__lasti), std::move(__result_first)};
	  }

	auto __result_real_last = __result_first;
	while (__first != __last && __result_real_last != __result_last)
	  {
	    *__result_real_last = *__first;
	    ++__result_real_last;
	    ++__first;
	  }

	ranges::make_heap(__result_first, __result_real_last, __comp, __proj2);
	for (; __first != __last; ++__first)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first),
			    std::__invoke(__proj2, *__result_first)))
	    {
	      ranges::pop_heap(__result_first, __result_real_last,
			       __comp, __proj2);
	      *(__result_real_last-1) = *__first;
	      ranges::push_heap(__result_first, __result_real_last,
				__comp, __proj2);
	    }
	ranges::sort_heap(__result_first, __result_real_last, __comp, __proj2);

	return {std::move(__first), std::move(__result_real_last)};
      }

    template<input_range _Range1, random_access_range _Range2,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_copyable<iterator_t<_Range1>, iterator_t<_Range2>>
	&& sortable<iterator_t<_Range2>, _Comp, _Proj2>
	&& indirect_strict_weak_order<_Comp,
				      projected<iterator_t<_Range1>, _Proj1>,
				      projected<iterator_t<_Range2>, _Proj2>>
      constexpr partial_sort_copy_result<borrowed_iterator_t<_Range1>,
					 borrowed_iterator_t<_Range2>>
      operator()(_Range1&& __r, _Range2&& __out, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       ranges::begin(__out), ranges::end(__out),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __partial_sort_copy_fn partial_sort_copy{};

  struct __is_sorted_until_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __next = __first;
	for (++__next; __next != __last; __first = __next, (void)++__next)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__next),
			    std::__invoke(__proj, *__first)))
	    return __next;
	return __next;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_sorted_until_fn is_sorted_until{};

  struct __is_sorted_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return true;

	auto __next = __first;
	for (++__next; __next != __last; __first = __next, (void)++__next)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__next),
			    std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_sorted_fn is_sorted{};

  struct __nth_element_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Iter __nth, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::nth_element(std::move(__first), std::move(__nth), __lasti,
			 __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __nth,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__nth),
		       ranges::end(__r), std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __nth_element_fn nth_element{};

  struct __lower_bound_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp, std::__invoke(__proj, *__middle), __value))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else
	      __len = __half;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __lower_bound_fn lower_bound{};

  struct __upper_bound_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp, __value, std::__invoke(__proj, *__middle)))
	      __len = __half;
	    else
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	  }
	return __first;
      }

    template<forward_range _Range, typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __upper_bound_fn upper_bound{};

  struct __equal_range_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__middle),
			      __value))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else if (std::__invoke(__comp,
				   __value,
				   std::__invoke(__proj, *__middle)))
	      __len = __half;
	    else
	      {
		auto __left
		  = ranges::lower_bound(__first, __middle,
					__value, __comp, __proj);
		ranges::advance(__first, __len);
		auto __right
		  = ranges::upper_bound(++__middle, __first,
					__value, __comp, __proj);
		return {__left, __right};
	      }
	  }
	return {__first, __first};
      }

    template<forward_range _Range,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, const _Tp& __value,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __equal_range_fn equal_range{};

  struct __binary_search_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __i = ranges::lower_bound(__first, __last, __value, __comp, __proj);
	if (__i == __last)
	  return false;
	return !(bool)std::__invoke(__comp, __value,
				    std::__invoke(__proj, *__i));
      }

    template<forward_range _Range,
	     typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, const _Tp& __value, _Comp __comp = {},
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __binary_search_fn binary_search{};

  struct __is_partitioned_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	__first = ranges::find_if_not(std::move(__first), __last,
				      __pred, __proj);
	if (__first == __last)
	  return true;
	++__first;
	return ranges::none_of(std::move(__first), std::move(__last),
			       std::move(__pred), std::move(__proj));
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __is_partitioned_fn is_partitioned{};

  struct __partition_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	if constexpr (bidirectional_iterator<_Iter>)
	  {
	    auto __lasti = ranges::next(__first, __last);
	    auto __tail = __lasti;
	    for (;;)
	      {
		for (;;)
		  if (__first == __tail)
		    return {std::move(__first), std::move(__lasti)};
		  else if (std::__invoke(__pred,
					 std::__invoke(__proj, *__first)))
		    ++__first;
		  else
		    break;
		--__tail;
		for (;;)
		  if (__first == __tail)
		    return {std::move(__first), std::move(__lasti)};
		  else if (!(bool)std::__invoke(__pred,
						std::__invoke(__proj, *__tail)))
		    --__tail;
		  else
		    break;
		ranges::iter_swap(__first, __tail);
		++__first;
	      }
	  }
	else
	  {
	    if (__first == __last)
	      return {std::move(__first), std::move(__first)};

	    while (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	      if (++__first == __last)
		return {std::move(__first), std::move(__first)};

	    auto __next = __first;
	    while (++__next != __last)
	      if (std::__invoke(__pred, std::__invoke(__proj, *__next)))
		{
		  ranges::iter_swap(__first, __next);
		  ++__first;
		}

	    return {std::move(__first), std::move(__next)};
	  }
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_fn partition{};

  struct __stable_partition_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires permutable<_Iter>
      subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	auto __middle
	  = std::stable_partition(std::move(__first), __lasti,
				  __detail::__make_pred_proj(__pred, __proj));
	return {std::move(__middle), std::move(__lasti)};
      }

    template<bidirectional_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __stable_partition_fn stable_partition{};

  template<typename _Iter, typename _Out1, typename _Out2>
    struct in_out_out_result
    {
      [[no_unique_address]] _Iter  in;
      [[no_unique_address]] _Out1 out1;
      [[no_unique_address]] _Out2 out2;

      template<typename _IIter, typename _OOut1, typename _OOut2>
	requires convertible_to<const _Iter&, _IIter>
	  && convertible_to<const _Out1&, _OOut1>
	  && convertible_to<const _Out2&, _OOut2>
	constexpr
	operator in_out_out_result<_IIter, _OOut1, _OOut2>() const &
	{ return {in, out1, out2}; }

      template<typename _IIter, typename _OOut1, typename _OOut2>
	requires convertible_to<_Iter, _IIter>
	  && convertible_to<_Out1, _OOut1>
	  && convertible_to<_Out2, _OOut2>
	constexpr
	operator in_out_out_result<_IIter, _OOut1, _OOut2>() &&
	{ return {std::move(in), std::move(out1), std::move(out2)}; }
    };

  template<typename _Iter, typename _Out1, typename _Out2>
    using partition_copy_result = in_out_out_result<_Iter, _Out1, _Out2>;

  struct __partition_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out1, weakly_incrementable _O2,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out1>
	&& indirectly_copyable<_Iter, _O2>
      constexpr partition_copy_result<_Iter, _Out1, _O2>
      operator()(_Iter __first, _Sent __last,
		 _Out1 __out_true, _O2 __out_false,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__out_true = *__first;
	      ++__out_true;
	    }
	  else
	    {
	      *__out_false = *__first;
	      ++__out_false;
	    }

	return {std::move(__first),
		std::move(__out_true), std::move(__out_false)};
      }

    template<input_range _Range, weakly_incrementable _Out1,
	     weakly_incrementable _O2,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out1>
	&& indirectly_copyable<iterator_t<_Range>, _O2>
      constexpr partition_copy_result<borrowed_iterator_t<_Range>, _Out1, _O2>
      operator()(_Range&& __r, _Out1 out_true, _O2 out_false,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(out_true), std::move(out_false),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_copy_fn partition_copy{};

  struct __partition_point_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__pred, std::__invoke(__proj, *__middle)))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else
	      __len = __half;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_point_fn partition_point{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using merge_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __merge_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr merge_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj2, *__first2),
			      std::__invoke(__proj1, *__first1)))
	      {
		*__result = *__first2;
		++__first2;
	      }
	    else
	      {
		*__result = *__first1;
		++__first1;
	      }
	    ++__result;
	  }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return { std::move(__copy1.in), std::move(__copy2.in),
		 std::move(__copy2.out) };
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr merge_result<borrowed_iterator_t<_Range1>,
			     borrowed_iterator_t<_Range2>,
			     _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __merge_fn merge{};

  struct __inplace_merge_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      _Iter
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __lasti = ranges::next(__first, __last);
	std::inplace_merge(std::move(__first), std::move(__middle), __lasti,
			   __detail::__make_comp_proj(__comp, __proj));
	return __lasti;
      }

    template<bidirectional_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __inplace_merge_fn inplace_merge{};

  struct __includes_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<_Iter1, _Proj1>,
					projected<_Iter2, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj2, *__first2),
			    std::__invoke(__proj1, *__first1)))
	    return false;
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj1, *__first1),
				 std::__invoke(__proj2, *__first2)))
	    ++__first1;
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }

	return __first2 == __last2;
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range1>, _Proj1>,
					projected<iterator_t<_Range2>, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __includes_fn includes{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_union_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_union_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_union_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj1, *__first1),
			      std::__invoke(__proj2, *__first2)))
	      {
		*__result = *__first1;
		++__first1;
	      }
	    else if (std::__invoke(__comp,
				   std::__invoke(__proj2, *__first2),
				   std::__invoke(__proj1, *__first1)))
	      {
		*__result = *__first2;
		++__first2;
	      }
	    else
	      {
		*__result = *__first1;
		++__first1;
		++__first2;
	      }
	    ++__result;
	  }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return {std::move(__copy1.in), std::move(__copy2.in),
		std::move(__copy2.out)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_union_result<borrowed_iterator_t<_Range1>,
				 borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_union_fn set_union{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_intersection_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_intersection_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_intersection_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    ++__first1;
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    ++__first2;
	  else
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__first2;
	      ++__result;
	    }
	// TODO: Eliminating these variables triggers an ICE.
	auto __last1i = ranges::next(std::move(__first1), std::move(__last1));
	auto __last2i = ranges::next(std::move(__first2), std::move(__last2));
	return {std::move(__last1i), std::move(__last2i), std::move(__result)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_intersection_result<borrowed_iterator_t<_Range1>,
					borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_intersection_fn set_intersection{};

  template<typename _Iter, typename _Out>
    using set_difference_result = in_out_result<_Iter, _Out>;

  struct __set_difference_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_difference_result<_Iter1, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__result;
	    }
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    ++__first2;
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }
	return ranges::copy(std::move(__first1), std::move(__last1),
			    std::move(__result));
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_difference_result<borrowed_iterator_t<_Range1>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_difference_fn set_difference{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_symmetric_difference_result
      = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_symmetric_difference_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_symmetric_difference_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__result;
	    }
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    {
	      *__result = *__first2;
	      ++__first2;
	      ++__result;
	    }
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return {std::move(__copy1.in), std::move(__copy2.in),
		std::move(__copy2.out)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_symmetric_difference_result<borrowed_iterator_t<_Range1>,
						borrowed_iterator_t<_Range2>,
						_Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_symmetric_difference_fn set_symmetric_difference{};

  struct __min_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr const _Tp&
      operator()(const _Tp& __a, const _Tp& __b,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (std::__invoke(std::move(__comp),
			  std::__invoke(__proj, __b),
			  std::__invoke(__proj, __a)))
	  return __b;
	else
	  return __a;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      requires indirectly_copyable_storable<iterator_t<_Range>,
					    range_value_t<_Range>*>
      constexpr range_value_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __first = ranges::begin(__r);
	auto __last = ranges::end(__r);
	__glibcxx_assert(__first != __last);
	auto __result = *__first;
	while (++__first != __last)
	  {
	    auto __tmp = *__first;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, __tmp),
			      std::__invoke(__proj, __result)))
	      __result = std::move(__tmp);
	  }
	return __result;
      }

    template<copyable _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr _Tp
      operator()(initializer_list<_Tp> __r,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::subrange(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __min_fn min{};

  struct __max_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr const _Tp&
      operator()(const _Tp& __a, const _Tp& __b,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (std::__invoke(std::move(__comp),
			  std::__invoke(__proj, __a),
			  std::__invoke(__proj, __b)))
	  return __b;
	else
	  return __a;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      requires indirectly_copyable_storable<iterator_t<_Range>,
					    range_value_t<_Range>*>
      constexpr range_value_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __first = ranges::begin(__r);
	auto __last = ranges::end(__r);
	__glibcxx_assert(__first != __last);
	auto __result = *__first;
	while (++__first != __last)
	  {
	    auto __tmp = *__first;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, __result),
			      std::__invoke(__proj, __tmp)))
	      __result = std::move(__tmp);
	  }
	return __result;
      }

    template<copyable _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr _Tp
      operator()(initializer_list<_Tp> __r,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::subrange(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __max_fn max{};

  struct __clamp_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>> _Comp
	       = ranges::less>
      constexpr const _Tp&
      operator()(const _Tp& __val, const _Tp& __lo, const _Tp& __hi,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	__glibcxx_assert(!(std::__invoke(__comp,
					 std::__invoke(__proj, __hi),
					 std::__invoke(__proj, __lo))));
	auto&& __proj_val = std::__invoke(__proj, __val);
	if (std::__invoke(__comp, __proj_val, std::__invoke(__proj, __lo)))
	  return __lo;
	else if (std::__invoke(__comp, std::__invoke(__proj, __hi), __proj_val))
	  return __hi;
	else
	  return __val;
      }
  };

  inline constexpr __clamp_fn clamp{};

  template<typename _Tp>
    struct min_max_result
    {
      [[no_unique_address]] _Tp min;
      [[no_unique_address]] _Tp max;

      template<typename _Tp2>
	requires convertible_to<const _Tp&, _Tp2>
	constexpr
	operator min_max_result<_Tp2>() const &
	{ return {min, max}; }

      template<typename _Tp2>
	requires convertible_to<_Tp, _Tp2>
	constexpr
	operator min_max_result<_Tp2>() &&
	{ return {std::move(min), std::move(max)}; }
    };

  template<typename _Tp>
    using minmax_result = min_max_result<_Tp>;

  struct __minmax_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_result<const _Tp&>
      operator()(const _Tp& __a, const _Tp& __b,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (std::__invoke(std::move(__comp),
			  std::__invoke(__proj, __b),
			  std::__invoke(__proj, __a)))
	  return {__b, __a};
	else
	  return {__a, __b};
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      requires indirectly_copyable_storable<iterator_t<_Range>,
      range_value_t<_Range>*>
      constexpr minmax_result<range_value_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __first = ranges::begin(__r);
	auto __last = ranges::end(__r);
	__glibcxx_assert(__first != __last);
	minmax_result<range_value_t<_Range>> __result = {*__first, *__first};
	while (++__first != __last)
	  {
	    auto __tmp = *__first;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, __tmp),
			      std::__invoke(__proj, __result.min)))
	      __result.min = std::move(__tmp);
	    if (!(bool)std::__invoke(__comp,
				     std::__invoke(__proj, __tmp),
				     std::__invoke(__proj, __result.max)))
	      __result.max = std::move(__tmp);
	  }
	return __result;
      }

    template<copyable _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_result<_Tp>
      operator()(initializer_list<_Tp> __r,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::subrange(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __minmax_fn minmax{};

  struct __min_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __i = __first;
	while (++__i != __last)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__i),
			      std::__invoke(__proj, *__first)))
	      __first = __i;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __min_element_fn min_element{};

  struct __max_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __i = __first;
	while (++__i != __last)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__first),
			      std::__invoke(__proj, *__i)))
	      __first = __i;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __max_element_fn max_element{};

  template<typename _Iter>
    using minmax_element_result = min_max_result<_Iter>;

  struct __minmax_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_element_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {__first, __first};

	minmax_element_result<_Iter> __result = {__first, __first};
	auto __i = __first;
	while (++__i != __last)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__i),
			      std::__invoke(__proj, *__result.min)))
	      __result.min = __i;
	    if (!(bool)std::__invoke(__comp,
				     std::__invoke(__proj, *__i),
				     std::__invoke(__proj, *__result.max)))
	      __result.max = __i;
	  }
	return __result;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_element_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __minmax_element_fn minmax_element{};

  struct __lexicographical_compare_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<_Iter1, _Proj1>,
					projected<_Iter2, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if constexpr (__detail::__is_normal_iterator<_Iter1>
		      && same_as<_Iter1, _Sent1>)
	  return (*this)(__first1.base(), __last1.base(),
			 std::move(__first2), std::move(__last2),
			 std::move(__comp),
			 std::move(__proj1), std::move(__proj2));
	else if constexpr (__detail::__is_normal_iterator<_Iter2>
			   && same_as<_Iter2, _Sent2>)
	  return (*this)(std::move(__first1), std::move(__last1),
			 __first2.base(), __last2.base(),
			 std::move(__comp),
			 std::move(__proj1), std::move(__proj2));
	else
	  {
	    constexpr bool __sized_iters
	      = (sized_sentinel_for<_Sent1, _Iter1>
		 && sized_sentinel_for<_Sent2, _Iter2>);
	    if constexpr (__sized_iters)
	      {
		using _ValueType1 = iter_value_t<_Iter1>;
		using _ValueType2 = iter_value_t<_Iter2>;
		// This condition is consistent with the one in
		// __lexicographical_compare_aux in <bits/stl_algobase.h>.
		constexpr bool __use_memcmp
		  = (__is_memcmp_ordered_with<_ValueType1, _ValueType2>::__value
		     && __ptr_to_nonvolatile<_Iter1>
		     && __ptr_to_nonvolatile<_Iter2>
		     && (is_same_v<_Comp, ranges::less>
			 || is_same_v<_Comp, ranges::greater>)
		     && is_same_v<_Proj1, identity>
		     && is_same_v<_Proj2, identity>);
		if constexpr (__use_memcmp)
		  {
		    const auto __d1 = __last1 - __first1;
		    const auto __d2 = __last2 - __first2;

		    if (const auto __len = std::min(__d1, __d2))
		      {
			const auto __c
			  = std::__memcmp(__first1, __first2, __len);
			if constexpr (is_same_v<_Comp, ranges::less>)
			  {
			    if (__c < 0)
			      return true;
			    if (__c > 0)
			      return false;
			  }
			else if constexpr (is_same_v<_Comp, ranges::greater>)
			  {
			    if (__c > 0)
			      return true;
			    if (__c < 0)
			      return false;
			  }
		      }
		    return __d1 < __d2;
		  }
	      }

	    for (; __first1 != __last1 && __first2 != __last2;
		 ++__first1, (void) ++__first2)
	      {
		if (std::__invoke(__comp,
				  std::__invoke(__proj1, *__first1),
				  std::__invoke(__proj2, *__first2)))
		  return true;
		if (std::__invoke(__comp,
				  std::__invoke(__proj2, *__first2),
				  std::__invoke(__proj1, *__first1)))
		  return false;
	      }
	    return __first1 == __last1 && __first2 != __last2;
	  }
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range1>, _Proj1>,
					projected<iterator_t<_Range2>, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }

  private:
    template<typename _Iter, typename _Ref = iter_reference_t<_Iter>>
      static constexpr bool __ptr_to_nonvolatile
	= is_pointer_v<_Iter> && !is_volatile_v<remove_reference_t<_Ref>>;
  };

  inline constexpr __lexicographical_compare_fn lexicographical_compare;

  template<typename _Iter>
    struct in_found_result
    {
      [[no_unique_address]] _Iter in;
      bool found;

      template<typename _Iter2>
	requires convertible_to<const _Iter&, _Iter2>
	constexpr
	operator in_found_result<_Iter2>() const &
	{ return {in, found}; }

      template<typename _Iter2>
	requires convertible_to<_Iter, _Iter2>
	constexpr
	operator in_found_result<_Iter2>() &&
	{ return {std::move(in), found}; }
    };

  template<typename _Iter>
    using next_permutation_result = in_found_result<_Iter>;

  struct __next_permutation_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr next_permutation_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), false};

	auto __i = __first;
	++__i;
	if (__i == __last)
	  return {std::move(__i), false};

	auto __lasti = ranges::next(__first, __last);
	__i = __lasti;
	--__i;

	for (;;)
	  {
	    auto __ii = __i;
	    --__i;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__i),
			      std::__invoke(__proj, *__ii)))
	      {
		auto __j = __lasti;
		while (!(bool)std::__invoke(__comp,
					    std::__invoke(__proj, *__i),
					    std::__invoke(__proj, *--__j)))
		  ;
		ranges::iter_swap(__i, __j);
		ranges::reverse(__ii, __last);
		return {std::move(__lasti), true};
	      }
	    if (__i == __first)
	      {
		ranges::reverse(__first, __last);
		return {std::move(__lasti), false};
	      }
	  }
      }

    template<bidirectional_range _Range, typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr next_permutation_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __next_permutation_fn next_permutation{};

  template<typename _Iter>
    using prev_permutation_result = in_found_result<_Iter>;

  struct __prev_permutation_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr prev_permutation_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), false};

	auto __i = __first;
	++__i;
	if (__i == __last)
	  return {std::move(__i), false};

	auto __lasti = ranges::next(__first, __last);
	__i = __lasti;
	--__i;

	for (;;)
	  {
	    auto __ii = __i;
	    --__i;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__ii),
			      std::__invoke(__proj, *__i)))
	      {
		auto __j = __lasti;
		while (!(bool)std::__invoke(__comp,
					    std::__invoke(__proj, *--__j),
					    std::__invoke(__proj, *__i)))
		  ;
		ranges::iter_swap(__i, __j);
		ranges::reverse(__ii, __last);
		return {std::move(__lasti), true};
	      }
	    if (__i == __first)
	      {
		ranges::reverse(__first, __last);
		return {std::move(__lasti), false};
	      }
	  }
      }

    template<bidirectional_range _Range, typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr prev_permutation_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __prev_permutation_fn prev_permutation{};

} // namespace ranges

#define __cpp_lib_shift 201806L
  template<typename _ForwardIterator>
    constexpr _ForwardIterator
    shift_left(_ForwardIterator __first, _ForwardIterator __last,
	       typename iterator_traits<_ForwardIterator>::difference_type __n)
    {
      __glibcxx_assert(__n >= 0);
      if (__n == 0)
	return __last;

      auto __mid = ranges::next(__first, __n, __last);
      if (__mid == __last)
	return __first;
      return std::move(std::move(__mid), std::move(__last), std::move(__first));
    }

  template<typename _ForwardIterator>
    constexpr _ForwardIterator
    shift_right(_ForwardIterator __first, _ForwardIterator __last,
		typename iterator_traits<_ForwardIterator>::difference_type __n)
    {
      __glibcxx_assert(__n >= 0);
      if (__n == 0)
	return __first;

      using _Cat
	= typename iterator_traits<_ForwardIterator>::iterator_category;
      if constexpr (derived_from<_Cat, bidirectional_iterator_tag>)
	{
	  auto __mid = ranges::next(__last, -__n, __first);
	  if (__mid == __first)
	    return __last;

	  return std::move_backward(std::move(__first), std::move(__mid),
				    std::move(__last));
	}
      else
	{
	  auto __result = ranges::next(__first, __n, __last);
	  if (__result == __last)
	    return __last;

	  auto __dest_head = __first, __dest_tail = __result;
	  while (__dest_head != __result)
	    {
	      if (__dest_tail == __last)
		{
		  // If we get here, then we must have
		  //     2*n >= distance(__first, __last)
		  // i.e. we are shifting out at least half of the range.  In
		  // this case we can safely perform the shift with a single
		  // move.
		  std::move(std::move(__first), std::move(__dest_head),
			    std::move(__result));
		  return __result;
		}
	      ++__dest_head;
	      ++__dest_tail;
	    }

	  for (;;)
	    {
	      // At the start of each iteration of this outer loop, the range
	      // [__first, __result) contains those elements that after shifting
	      // the whole range right by __n, should end up in
	      // [__dest_head, __dest_tail) in order.

	      // The below inner loop swaps the elements of [__first, __result)
	      // and [__dest_head, __dest_tail), while simultaneously shifting
	      // the latter range by __n.
	      auto __cursor = __first;
	      while (__cursor != __result)
		{
		  if (__dest_tail == __last)
		    {
		      // At this point the ranges [__first, result) and
		      // [__dest_head, dest_tail) are disjoint, so we can safely
		      // move the remaining elements.
		      __dest_head = std::move(__cursor, __result,
					      std::move(__dest_head));
		      std::move(std::move(__first), std::move(__cursor),
				std::move(__dest_head));
		      return __result;
		    }
		  std::iter_swap(__cursor, __dest_head);
		  ++__dest_head;
		  ++__dest_tail;
		  ++__cursor;
		}
	    }
	}
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // concepts
#endif // C++20
#endif // _RANGES_ALGO_H
