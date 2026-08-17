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

/** @file bits/ranges_algobase.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{algorithm}
 */

#ifndef _RANGES_ALGOBASE_H
#define _RANGES_ALGOBASE_H 1

#if __cplusplus > 201703L

#include <compare>
#include <iterator>
// #include <bits/range_concepts.h>
#include <ranges>
#include <bits/invoke.h>
#include <bits/cpp_type_traits.h> // __is_byte

#if __cpp_lib_concepts
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace ranges
{
  namespace __detail
  {
    template<typename _Tp>
      constexpr inline bool __is_normal_iterator = false;

    template<typename _Iterator, typename _Container>
      constexpr inline bool
	__is_normal_iterator<__gnu_cxx::__normal_iterator<_Iterator,
							  _Container>> = true;

    template<typename _Tp>
      constexpr inline bool __is_reverse_iterator = false;

    template<typename _Iterator>
      constexpr inline bool
	__is_reverse_iterator<reverse_iterator<_Iterator>> = true;

    template<typename _Tp>
      constexpr inline bool __is_move_iterator = false;

    template<typename _Iterator>
      constexpr inline bool
	__is_move_iterator<move_iterator<_Iterator>> = true;
  } // namespace __detail

  struct __equal_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	// TODO: implement more specializations to at least have parity with
	// std::equal.
	if constexpr (__detail::__is_normal_iterator<_Iter1>
		      && same_as<_Iter1, _Sent1>)
	  return (*this)(__first1.base(), __last1.base(),
			 std::move(__first2), std::move(__last2),
			 std::move(__pred),
			 std::move(__proj1), std::move(__proj2));
	else if constexpr (__detail::__is_normal_iterator<_Iter2>
			   && same_as<_Iter2, _Sent2>)
	  return (*this)(std::move(__first1), std::move(__last1),
			 __first2.base(), __last2.base(),
			 std::move(__pred),
			 std::move(__proj1), std::move(__proj2));
	else if constexpr (sized_sentinel_for<_Sent1, _Iter1>
			   && sized_sentinel_for<_Sent2, _Iter2>)
	  {
	    auto __d1 = ranges::distance(__first1, __last1);
	    auto __d2 = ranges::distance(__first2, __last2);
	    if (__d1 != __d2)
	      return false;

	    using _ValueType1 = iter_value_t<_Iter1>;
	    using _ValueType2 = iter_value_t<_Iter2>;
	    constexpr bool __use_memcmp
	      = ((is_integral_v<_ValueType1> || is_pointer_v<_ValueType1>)
		 && __memcmpable<_Iter1, _Iter2>::__value
		 && is_same_v<_Pred, ranges::equal_to>
		 && is_same_v<_Proj1, identity>
		 && is_same_v<_Proj2, identity>);
	    if constexpr (__use_memcmp)
	      {
		if (const size_t __len = (__last1 - __first1))
		  return !std::__memcmp(__first1, __first2, __len);
		return true;
	      }
	    else
	      {
		for (; __first1 != __last1; ++__first1, (void)++__first2)
		  if (!(bool)std::__invoke(__pred,
					   std::__invoke(__proj1, *__first1),
					   std::__invoke(__proj2, *__first2)))
		    return false;
		return true;
	      }
	  }
	else
	  {
	    for (; __first1 != __last1 && __first2 != __last2;
		 ++__first1, (void)++__first2)
	      if (!(bool)std::__invoke(__pred,
				       std::__invoke(__proj1, *__first1),
				       std::__invoke(__proj2, *__first2)))
		return false;
	    return __first1 == __last1 && __first2 == __last2;
	  }
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
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

  inline constexpr __equal_fn equal{};

  template<typename _Iter, typename _Out>
    struct in_out_result
    {
      [[no_unique_address]] _Iter in;
      [[no_unique_address]] _Out out;

      template<typename _Iter2, typename _Out2>
	requires convertible_to<const _Iter&, _Iter2>
	  && convertible_to<const _Out&, _Out2>
	constexpr
	operator in_out_result<_Iter2, _Out2>() const &
	{ return {in, out}; }

      template<typename _Iter2, typename _Out2>
	requires convertible_to<_Iter, _Iter2>
	  && convertible_to<_Out, _Out2>
	constexpr
	operator in_out_result<_Iter2, _Out2>() &&
	{ return {std::move(in), std::move(out)}; }
    };

  template<typename _Iter, typename _Out>
    using copy_result = in_out_result<_Iter, _Out>;

  template<typename _Iter, typename _Out>
    using move_result = in_out_result<_Iter, _Out>;

  template<typename _Iter1, typename _Iter2>
    using move_backward_result = in_out_result<_Iter1, _Iter2>;

  template<typename _Iter1, typename _Iter2>
    using copy_backward_result = in_out_result<_Iter1, _Iter2>;

  template<bool _IsMove,
	   bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	   bidirectional_iterator _Out>
    requires (_IsMove
	      ? indirectly_movable<_Iter, _Out>
	      : indirectly_copyable<_Iter, _Out>)
    constexpr conditional_t<_IsMove,
			    move_backward_result<_Iter, _Out>,
			    copy_backward_result<_Iter, _Out>>
    __copy_or_move_backward(_Iter __first, _Sent __last, _Out __result);

  template<bool _IsMove,
	   input_iterator _Iter, sentinel_for<_Iter> _Sent,
	   weakly_incrementable _Out>
    requires (_IsMove
	      ? indirectly_movable<_Iter, _Out>
	      : indirectly_copyable<_Iter, _Out>)
    constexpr conditional_t<_IsMove,
			    move_result<_Iter, _Out>,
			    copy_result<_Iter, _Out>>
    __copy_or_move(_Iter __first, _Sent __last, _Out __result)
    {
      // TODO: implement more specializations to be at least on par with
      // std::copy/std::move.
      using __detail::__is_move_iterator;
      using __detail::__is_reverse_iterator;
      using __detail::__is_normal_iterator;
      if constexpr (__is_move_iterator<_Iter> && same_as<_Iter, _Sent>)
	{
	  auto [__in, __out]
	    = ranges::__copy_or_move<true>(std::move(__first).base(),
					   std::move(__last).base(),
					   std::move(__result));
	  return {move_iterator{std::move(__in)}, std::move(__out)};
	}
      else if constexpr (__is_reverse_iterator<_Iter> && same_as<_Iter, _Sent>
			 && __is_reverse_iterator<_Out>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move_backward<_IsMove>(std::move(__last).base(),
						       std::move(__first).base(),
						       std::move(__result).base());
	  return {reverse_iterator{std::move(__in)},
		  reverse_iterator{std::move(__out)}};
	}
      else if constexpr (__is_normal_iterator<_Iter> && same_as<_Iter, _Sent>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move<_IsMove>(__first.base(), __last.base(),
					      __result);
	  return {decltype(__first){__in}, std::move(__out)};
	}
      else if constexpr (__is_normal_iterator<_Out>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move<_IsMove>(__first, __last, __result.base());
	  return {std::move(__in), decltype(__result){__out}};
	}
      else if constexpr (sized_sentinel_for<_Sent, _Iter>)
	{
#ifdef __cpp_lib_is_constant_evaluated
	  if (!std::is_constant_evaluated())
#endif
	    {
	      if constexpr (__memcpyable<_Iter, _Out>::__value)
		{
		  using _ValueTypeI = iter_value_t<_Iter>;
		  static_assert(_IsMove
		      ? is_move_assignable_v<_ValueTypeI>
		      : is_copy_assignable_v<_ValueTypeI>);
		  auto __num = __last - __first;
		  if (__num)
		    __builtin_memmove(__result, __first,
			sizeof(_ValueTypeI) * __num);
		  return {__first + __num, __result + __num};
		}
	    }

	  for (auto __n = __last - __first; __n > 0; --__n)
	    {
	      if constexpr (_IsMove)
		*__result = std::move(*__first);
	      else
		*__result = *__first;
	      ++__first;
	      ++__result;
	    }
	  return {std::move(__first), std::move(__result)};
	}
      else
	{
	  while (__first != __last)
	    {
	      if constexpr (_IsMove)
		*__result = std::move(*__first);
	      else
		*__result = *__first;
	      ++__first;
	      ++__result;
	    }
	  return {std::move(__first), std::move(__result)};
	}
    }

  struct __copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result) const
      {
	return ranges::__copy_or_move<false>(std::move(__first),
					     std::move(__last),
					     std::move(__result));
      }

    template<input_range _Range, weakly_incrementable _Out>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __copy_fn copy{};

  struct __move_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_movable<_Iter, _Out>
      constexpr move_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result) const
      {
	return ranges::__copy_or_move<true>(std::move(__first),
					    std::move(__last),
					    std::move(__result));
      }

    template<input_range _Range, weakly_incrementable _Out>
      requires indirectly_movable<iterator_t<_Range>, _Out>
      constexpr move_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __move_fn move{};

  template<bool _IsMove,
	   bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	   bidirectional_iterator _Out>
    requires (_IsMove
	      ? indirectly_movable<_Iter, _Out>
	      : indirectly_copyable<_Iter, _Out>)
    constexpr conditional_t<_IsMove,
			    move_backward_result<_Iter, _Out>,
			    copy_backward_result<_Iter, _Out>>
    __copy_or_move_backward(_Iter __first, _Sent __last, _Out __result)
    {
      // TODO: implement more specializations to be at least on par with
      // std::copy_backward/std::move_backward.
      using __detail::__is_reverse_iterator;
      using __detail::__is_normal_iterator;
      if constexpr (__is_reverse_iterator<_Iter> && same_as<_Iter, _Sent>
		    && __is_reverse_iterator<_Out>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move<_IsMove>(std::move(__last).base(),
					      std::move(__first).base(),
					      std::move(__result).base());
	  return {reverse_iterator{std::move(__in)},
		  reverse_iterator{std::move(__out)}};
	}
      else if constexpr (__is_normal_iterator<_Iter> && same_as<_Iter, _Sent>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move_backward<_IsMove>(__first.base(),
						       __last.base(),
						       std::move(__result));
	  return {decltype(__first){__in}, std::move(__out)};
	}
      else if constexpr (__is_normal_iterator<_Out>)
	{
	  auto [__in,__out]
	    = ranges::__copy_or_move_backward<_IsMove>(std::move(__first),
						       std::move(__last),
						       __result.base());
	  return {std::move(__in), decltype(__result){__out}};
	}
      else if constexpr (sized_sentinel_for<_Sent, _Iter>)
	{
#ifdef __cpp_lib_is_constant_evaluated
	  if (!std::is_constant_evaluated())
#endif
	    {
	      if constexpr (__memcpyable<_Out, _Iter>::__value)
		{
		  using _ValueTypeI = iter_value_t<_Iter>;
		  static_assert(_IsMove
		      ? is_move_assignable_v<_ValueTypeI>
		      : is_copy_assignable_v<_ValueTypeI>);
		  auto __num = __last - __first;
		  if (__num)
		    __builtin_memmove(__result - __num, __first,
				      sizeof(_ValueTypeI) * __num);
		  return {__first + __num, __result - __num};
		}
	    }

	  auto __lasti = ranges::next(__first, __last);
	  auto __tail = __lasti;

	  for (auto __n = __last - __first; __n > 0; --__n)
	    {
	      --__tail;
	      --__result;
	      if constexpr (_IsMove)
		*__result = std::move(*__tail);
	      else
		*__result = *__tail;
	    }
	  return {std::move(__lasti), std::move(__result)};
	}
      else
	{
	  auto __lasti = ranges::next(__first, __last);
	  auto __tail = __lasti;

	  while (__first != __tail)
	    {
	      --__tail;
	      --__result;
	      if constexpr (_IsMove)
		*__result = std::move(*__tail);
	      else
		*__result = *__tail;
	    }
	  return {std::move(__lasti), std::move(__result)};
	}
    }

  struct __copy_backward_fn
  {
    template<bidirectional_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     bidirectional_iterator _Iter2>
      requires indirectly_copyable<_Iter1, _Iter2>
      constexpr copy_backward_result<_Iter1, _Iter2>
      operator()(_Iter1 __first, _Sent1 __last, _Iter2 __result) const
      {
	return ranges::__copy_or_move_backward<false>(std::move(__first),
						      std::move(__last),
						      std::move(__result));
      }

    template<bidirectional_range _Range, bidirectional_iterator _Iter>
      requires indirectly_copyable<iterator_t<_Range>, _Iter>
      constexpr copy_backward_result<borrowed_iterator_t<_Range>, _Iter>
      operator()(_Range&& __r, _Iter __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __copy_backward_fn copy_backward{};

  struct __move_backward_fn
  {
    template<bidirectional_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     bidirectional_iterator _Iter2>
      requires indirectly_movable<_Iter1, _Iter2>
      constexpr move_backward_result<_Iter1, _Iter2>
      operator()(_Iter1 __first, _Sent1 __last, _Iter2 __result) const
      {
	return ranges::__copy_or_move_backward<true>(std::move(__first),
						     std::move(__last),
						     std::move(__result));
      }

    template<bidirectional_range _Range, bidirectional_iterator _Iter>
      requires indirectly_movable<iterator_t<_Range>, _Iter>
      constexpr move_backward_result<borrowed_iterator_t<_Range>, _Iter>
      operator()(_Range&& __r, _Iter __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __move_backward_fn move_backward{};

  template<typename _Iter, typename _Out>
    using copy_n_result = in_out_result<_Iter, _Out>;

  struct __copy_n_fn
  {
    template<input_iterator _Iter, weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr copy_n_result<_Iter, _Out>
      operator()(_Iter __first, iter_difference_t<_Iter> __n,
		 _Out __result) const
      {
	if constexpr (random_access_iterator<_Iter>)
	  return ranges::copy(__first, __first + __n, std::move(__result));
	else
	  {
	    for (; __n > 0; --__n, (void)++__result, (void)++__first)
	      *__result = *__first;
	    return {std::move(__first), std::move(__result)};
	  }
      }
  };

  inline constexpr __copy_n_fn copy_n{};

  struct __fill_n_fn
  {
    template<typename _Tp, output_iterator<const _Tp&> _Out>
      constexpr _Out
      operator()(_Out __first, iter_difference_t<_Out> __n,
		 const _Tp& __value) const
      {
	// TODO: implement more specializations to be at least on par with
	// std::fill_n
	if (__n <= 0)
	  return __first;

	// TODO: Generalize this optimization to contiguous iterators.
	if constexpr (is_pointer_v<_Out>
		      // Note that __is_byte already implies !is_volatile.
		      && __is_byte<remove_pointer_t<_Out>>::__value
		      && integral<_Tp>)
	  {
	    __builtin_memset(__first, static_cast<unsigned char>(__value), __n);
	    return __first + __n;
	  }
	else if constexpr (is_scalar_v<_Tp>)
	  {
	    const auto __tmp = __value;
	    for (; __n > 0; --__n, (void)++__first)
	      *__first = __tmp;
	    return __first;
	  }
	else
	  {
	    for (; __n > 0; --__n, (void)++__first)
	      *__first = __value;
	    return __first;
	  }
      }
  };

  inline constexpr __fill_n_fn fill_n{};

  struct __fill_fn
  {
    template<typename _Tp,
	     output_iterator<const _Tp&> _Out, sentinel_for<_Out> _Sent>
      constexpr _Out
      operator()(_Out __first, _Sent __last, const _Tp& __value) const
      {
	// TODO: implement more specializations to be at least on par with
	// std::fill
	if constexpr (sized_sentinel_for<_Sent, _Out>)
	  {
	    const auto __len = __last - __first;
	    return ranges::fill_n(__first, __len, __value);
	  }
	else if constexpr (is_scalar_v<_Tp>)
	  {
	    const auto __tmp = __value;
	    for (; __first != __last; ++__first)
	      *__first = __tmp;
	    return __first;
	  }
	else
	  {
	    for (; __first != __last; ++__first)
	      *__first = __value;
	    return __first;
	  }
      }

    template<typename _Tp, output_range<const _Tp&> _Range>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, const _Tp& __value) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r), __value);
      }
  };

  inline constexpr __fill_fn fill{};
}
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // concepts
#endif // C++20
#endif // _RANGES_ALGOBASE_H
