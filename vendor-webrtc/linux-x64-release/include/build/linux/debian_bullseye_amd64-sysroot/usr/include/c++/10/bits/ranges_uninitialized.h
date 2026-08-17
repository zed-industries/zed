// Raw memory manipulators -*- C++ -*-

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

/** @file bits/ranges_uninitialized.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _RANGES_UNINITIALIZED_H
#define _RANGES_UNINITIALIZED_H 1

#if __cplusplus > 201703L
#if __cpp_lib_concepts

#include <bits/ranges_algobase.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace ranges
{
  namespace __detail
  {
    template<typename _Tp>
      constexpr void*
      __voidify(_Tp& __obj) noexcept
      {
	return const_cast<void*>
		 (static_cast<const volatile void*>(std::__addressof(__obj)));
      }

    template<typename _Iter>
      concept __nothrow_input_iterator
	= (input_iterator<_Iter>
	   && is_lvalue_reference_v<iter_reference_t<_Iter>>
	   && same_as<remove_cvref_t<iter_reference_t<_Iter>>,
		      iter_value_t<_Iter>>);

    template<typename _Sent, typename _Iter>
      concept __nothrow_sentinel = sentinel_for<_Sent, _Iter>;

    template<typename _Range>
      concept __nothrow_input_range
	= (range<_Range>
	   && __nothrow_input_iterator<iterator_t<_Range>>
	   && __nothrow_sentinel<sentinel_t<_Range>, iterator_t<_Range>>);

    template<typename _Iter>
      concept __nothrow_forward_iterator
	= (__nothrow_input_iterator<_Iter>
	   && forward_iterator<_Iter>
	   && __nothrow_sentinel<_Iter, _Iter>);

    template<typename _Range>
      concept __nothrow_forward_range
	= (__nothrow_input_range<_Range>
	   && __nothrow_forward_iterator<iterator_t<_Range>>);
  } // namespace __detail

  struct __destroy_fn
  {
    template<__detail::__nothrow_input_iterator _Iter,
	     __detail::__nothrow_sentinel<_Iter> _Sent>
      requires destructible<iter_value_t<_Iter>>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last) const noexcept;

    template<__detail::__nothrow_input_range _Range>
      requires destructible<range_value_t<_Range>>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r) const noexcept;
  };

  inline constexpr __destroy_fn destroy{};

  namespace __detail
  {
    template<typename _Iter>
      requires destructible<iter_value_t<_Iter>>
      struct _DestroyGuard
      {
      private:
	_Iter _M_first;
	const _Iter* _M_cur;

      public:
	explicit
	_DestroyGuard(const _Iter* __iter)
	  : _M_first(*__iter), _M_cur(__iter)
	{ }

	void
	release() noexcept
	{ _M_cur = nullptr; }

	~_DestroyGuard()
	{
	  if (_M_cur != nullptr)
	    ranges::destroy(std::move(_M_first), *_M_cur);
	}
      };

    template<typename _Iter>
      requires destructible<iter_value_t<_Iter>>
	&& is_trivially_destructible_v<iter_value_t<_Iter>>
      struct _DestroyGuard<_Iter>
      {
	explicit
	_DestroyGuard(const _Iter*)
	{ }

	void
	release() noexcept
	{ }
      };
  } // namespace __detail

  struct __uninitialized_default_construct_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter,
	     __detail::__nothrow_sentinel<_Iter> _Sent>
      requires default_initializable<iter_value_t<_Iter>>
      _Iter
      operator()(_Iter __first, _Sent __last) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivially_default_constructible_v<_ValueType>)
	  return ranges::next(__first, __last);
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __first != __last; ++__first)
	      ::new (__detail::__voidify(*__first)) _ValueType;
	    __guard.release();
	    return __first;
	  }
      }

    template<__detail::__nothrow_forward_range _Range>
      requires default_initializable<range_value_t<_Range>>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r));
      }
  };

  inline constexpr __uninitialized_default_construct_fn
    uninitialized_default_construct{};

  struct __uninitialized_default_construct_n_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter>
      requires default_initializable<iter_value_t<_Iter>>
      _Iter
      operator()(_Iter __first, iter_difference_t<_Iter> __n) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivially_default_constructible_v<_ValueType>)
	  return ranges::next(__first, __n);
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __n > 0; ++__first, (void) --__n)
	      ::new (__detail::__voidify(*__first)) _ValueType;
	    __guard.release();
	    return __first;
	  }
      }
  };

  inline constexpr __uninitialized_default_construct_n_fn
    uninitialized_default_construct_n;

  struct __uninitialized_value_construct_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter,
	     __detail::__nothrow_sentinel<_Iter> _Sent>
      requires default_initializable<iter_value_t<_Iter>>
      _Iter
      operator()(_Iter __first, _Sent __last) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivial_v<_ValueType>
		      && is_copy_assignable_v<_ValueType>)
	  return ranges::fill(__first, __last, _ValueType());
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __first != __last; ++__first)
	      ::new (__detail::__voidify(*__first)) _ValueType();
	    __guard.release();
	    return __first;
	  }
      }

    template<__detail::__nothrow_forward_range _Range>
      requires default_initializable<range_value_t<_Range>>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r));
      }
  };

  inline constexpr __uninitialized_value_construct_fn
    uninitialized_value_construct{};

  struct __uninitialized_value_construct_n_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter>
      requires default_initializable<iter_value_t<_Iter>>
      _Iter
      operator()(_Iter __first, iter_difference_t<_Iter> __n) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivial_v<_ValueType>
		      && is_copy_assignable_v<_ValueType>)
	  return ranges::fill_n(__first, __n, _ValueType());
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __n > 0; ++__first, (void) --__n)
	      ::new (__detail::__voidify(*__first)) _ValueType();
	    __guard.release();
	    return __first;
	  }
      }
  };

  inline constexpr __uninitialized_value_construct_n_fn
    uninitialized_value_construct_n;

  template<typename _Iter, typename _Out>
    using uninitialized_copy_result = in_out_result<_Iter, _Out>;

  struct __uninitialized_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _ISent,
	     __detail::__nothrow_forward_iterator _Out,
	     __detail::__nothrow_sentinel<_Out> _OSent>
      requires constructible_from<iter_value_t<_Out>, iter_reference_t<_Iter>>
      uninitialized_copy_result<_Iter, _Out>
      operator()(_Iter __ifirst, _ISent __ilast,
		 _Out __ofirst, _OSent __olast) const
      {
	using _OutType = remove_reference_t<iter_reference_t<_Out>>;
	if constexpr (sized_sentinel_for<_ISent, _Iter>
		      && sized_sentinel_for<_OSent, _Out>
		      && is_trivial_v<_OutType>
		      && is_nothrow_assignable_v<_OutType&,
						 iter_reference_t<_Iter>>)
	  {
	    auto __d1 = __ilast - __ifirst;
	    auto __d2 = __olast - __ofirst;
	    return ranges::copy_n(std::move(__ifirst), std::min(__d1, __d2),
				  __ofirst);
	  }
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__ofirst);
	    for (; __ifirst != __ilast && __ofirst != __olast;
		 ++__ofirst, (void)++__ifirst)
	      ::new (__detail::__voidify(*__ofirst)) _OutType(*__ifirst);
	    __guard.release();
	    return {std::move(__ifirst), __ofirst};
	  }
      }

    template<input_range _IRange, __detail::__nothrow_forward_range _ORange>
      requires constructible_from<range_value_t<_ORange>,
				  range_reference_t<_IRange>>
      uninitialized_copy_result<borrowed_iterator_t<_IRange>,
				borrowed_iterator_t<_ORange>>
      operator()(_IRange&& __inr, _ORange&& __outr) const
      {
	return (*this)(ranges::begin(__inr), ranges::end(__inr),
		       ranges::begin(__outr), ranges::end(__outr));
      }
  };

  inline constexpr __uninitialized_copy_fn uninitialized_copy{};

  template<typename _Iter, typename _Out>
    using uninitialized_copy_n_result = in_out_result<_Iter, _Out>;

  struct __uninitialized_copy_n_fn
  {
    template<input_iterator _Iter, __detail::__nothrow_forward_iterator _Out,
	     __detail::__nothrow_sentinel<_Out> _Sent>
      requires constructible_from<iter_value_t<_Out>, iter_reference_t<_Iter>>
      uninitialized_copy_n_result<_Iter, _Out>
      operator()(_Iter __ifirst, iter_difference_t<_Iter> __n,
		 _Out __ofirst, _Sent __olast) const
      {
	using _OutType = remove_reference_t<iter_reference_t<_Out>>;
	if constexpr (sized_sentinel_for<_Sent, _Out>
		      && is_trivial_v<_OutType>
		      && is_nothrow_assignable_v<_OutType&,
						 iter_reference_t<_Iter>>)
	  {
	    auto __d = __olast - __ofirst;
	    return ranges::copy_n(std::move(__ifirst), std::min(__n, __d),
				  __ofirst);
	  }
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__ofirst);
	    for (; __n > 0 && __ofirst != __olast;
		 ++__ofirst, (void)++__ifirst, (void)--__n)
	      ::new (__detail::__voidify(*__ofirst)) _OutType(*__ifirst);
	    __guard.release();
	    return {std::move(__ifirst), __ofirst};
	  }
      }
  };

  inline constexpr __uninitialized_copy_n_fn uninitialized_copy_n{};

  template<typename _Iter, typename _Out>
    using uninitialized_move_result = in_out_result<_Iter, _Out>;

  struct __uninitialized_move_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _ISent,
	     __detail::__nothrow_forward_iterator _Out,
	     __detail::__nothrow_sentinel<_Out> _OSent>
      requires constructible_from<iter_value_t<_Out>,
				  iter_rvalue_reference_t<_Iter>>
      uninitialized_move_result<_Iter, _Out>
      operator()(_Iter __ifirst, _ISent __ilast,
		 _Out __ofirst, _OSent __olast) const
      {
	using _OutType = remove_reference_t<iter_reference_t<_Out>>;
	if constexpr (sized_sentinel_for<_ISent, _Iter>
		      && sized_sentinel_for<_OSent, _Out>
		      && is_trivial_v<_OutType>
		      && is_nothrow_assignable_v<_OutType&,
						 iter_rvalue_reference_t<_Iter>>)
	  {
	    auto __d1 = __ilast - __ifirst;
	    auto __d2 = __olast - __ofirst;
	    auto [__in, __out]
	      = ranges::copy_n(std::make_move_iterator(std::move(__ifirst)),
			       std::min(__d1, __d2), __ofirst);
	    return {std::move(__in).base(), __out};
	  }
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__ofirst);
	    for (; __ifirst != __ilast && __ofirst != __olast;
		 ++__ofirst, (void)++__ifirst)
	      ::new (__detail::__voidify(*__ofirst))
		    _OutType(ranges::iter_move(__ifirst));
	    __guard.release();
	    return {std::move(__ifirst), __ofirst};
	  }
      }

    template<input_range _IRange, __detail::__nothrow_forward_range _ORange>
      requires constructible_from<range_value_t<_ORange>,
	       range_rvalue_reference_t<_IRange>>
      uninitialized_move_result<borrowed_iterator_t<_IRange>,
				borrowed_iterator_t<_ORange>>
      operator()(_IRange&& __inr, _ORange&& __outr) const
      {
	return (*this)(ranges::begin(__inr), ranges::end(__inr),
		       ranges::begin(__outr), ranges::end(__outr));
      }
  };

  inline constexpr __uninitialized_move_fn uninitialized_move{};

  template<typename _Iter, typename _Out>
    using uninitialized_move_n_result = in_out_result<_Iter, _Out>;

  struct __uninitialized_move_n_fn
  {
    template<input_iterator _Iter, __detail::__nothrow_forward_iterator _Out,
      __detail::__nothrow_sentinel<_Out> _Sent>
	requires constructible_from<iter_value_t<_Out>,
				    iter_rvalue_reference_t<_Iter>>
      uninitialized_move_n_result<_Iter, _Out>
      operator()(_Iter __ifirst, iter_difference_t<_Iter> __n,
		 _Out __ofirst, _Sent __olast) const
      {
	using _OutType = remove_reference_t<iter_reference_t<_Out>>;
	if constexpr (sized_sentinel_for<_Sent, _Out>
		      && is_trivial_v<_OutType>
		      && is_nothrow_assignable_v<_OutType&,
						 iter_rvalue_reference_t<_Iter>>)
	  {
	    auto __d = __olast - __ofirst;
	    auto [__in, __out]
	      = ranges::copy_n(std::make_move_iterator(std::move(__ifirst)),
			       std::min(__n, __d), __ofirst);
	    return {std::move(__in).base(), __out};
	  }
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__ofirst);
	    for (; __n > 0 && __ofirst != __olast;
		 ++__ofirst, (void)++__ifirst, (void)--__n)
	      ::new (__detail::__voidify(*__ofirst))
		    _OutType(ranges::iter_move(__ifirst));
	    __guard.release();
	    return {std::move(__ifirst), __ofirst};
	  }
      }
  };

  inline constexpr __uninitialized_move_n_fn uninitialized_move_n{};

  struct __uninitialized_fill_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter,
	     __detail::__nothrow_sentinel<_Iter> _Sent, typename _Tp>
      requires constructible_from<iter_value_t<_Iter>, const _Tp&>
      _Iter
      operator()(_Iter __first, _Sent __last, const _Tp& __x) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivial_v<_ValueType>
		      && is_nothrow_assignable_v<_ValueType&, const _Tp&>)
	  return ranges::fill(__first, __last, __x);
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __first != __last; ++__first)
	      ::new (__detail::__voidify(*__first)) _ValueType(__x);
	    __guard.release();
	    return __first;
	  }
      }

    template<__detail::__nothrow_forward_range _Range, typename _Tp>
      requires constructible_from<range_value_t<_Range>, const _Tp&>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, const _Tp& __x) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r), __x);
      }
  };

  inline constexpr __uninitialized_fill_fn uninitialized_fill{};

  struct __uninitialized_fill_n_fn
  {
    template<__detail::__nothrow_forward_iterator _Iter, typename _Tp>
      requires constructible_from<iter_value_t<_Iter>, const _Tp&>
      _Iter
      operator()(_Iter __first, iter_difference_t<_Iter> __n,
		 const _Tp& __x) const
      {
	using _ValueType = remove_reference_t<iter_reference_t<_Iter>>;
	if constexpr (is_trivial_v<_ValueType>
		      && is_nothrow_assignable_v<_ValueType&, const _Tp&>)
	  return ranges::fill_n(__first, __n, __x);
	else
	  {
	    auto __guard = __detail::_DestroyGuard(&__first);
	    for (; __n > 0; ++__first, (void)--__n)
	      ::new (__detail::__voidify(*__first)) _ValueType(__x);
	    __guard.release();
	    return __first;
	  }
      }
  };

  inline constexpr __uninitialized_fill_n_fn uninitialized_fill_n{};

  struct __construct_at_fn
  {
    template<typename _Tp, typename... _Args>
      requires requires {
	::new (std::declval<void*>()) _Tp(std::declval<_Args>()...);
      }
      constexpr _Tp*
      operator()(_Tp* __location, _Args&&... __args) const
      noexcept(noexcept(std::construct_at(__location,
					  std::forward<_Args>(__args)...)))
      {
	return std::construct_at(__location,
				 std::forward<_Args>(__args)...);
      }
  };

  inline constexpr __construct_at_fn construct_at{};

  struct __destroy_at_fn
  {
    template<destructible _Tp>
      constexpr void
      operator()(_Tp* __location) const noexcept
      {
	if constexpr (is_array_v<_Tp>)
	  ranges::destroy(ranges::begin(*__location), ranges::end(*__location));
	else
	  __location->~_Tp();
      }
  };

  inline constexpr __destroy_at_fn destroy_at{};

  template<__detail::__nothrow_input_iterator _Iter,
	   __detail::__nothrow_sentinel<_Iter> _Sent>
    requires destructible<iter_value_t<_Iter>>
    constexpr _Iter
    __destroy_fn::operator()(_Iter __first, _Sent __last) const noexcept
    {
      if constexpr (is_trivially_destructible_v<iter_value_t<_Iter>>)
	return ranges::next(std::move(__first), __last);
      else
	{
	  for (; __first != __last; ++__first)
	    ranges::destroy_at(std::__addressof(*__first));
	  return __first;
	}
    }

  template<__detail::__nothrow_input_range _Range>
    requires destructible<range_value_t<_Range>>
    constexpr borrowed_iterator_t<_Range>
    __destroy_fn::operator()(_Range&& __r) const noexcept
    {
      return (*this)(ranges::begin(__r), ranges::end(__r));
    }

  struct __destroy_n_fn
  {
    template<__detail::__nothrow_input_iterator _Iter>
      requires destructible<iter_value_t<_Iter>>
      constexpr _Iter
      operator()(_Iter __first, iter_difference_t<_Iter> __n) const noexcept
      {
	if constexpr (is_trivially_destructible_v<iter_value_t<_Iter>>)
	  return ranges::next(std::move(__first), __n);
	else
	  {
	    for (; __n > 0; ++__first, (void)--__n)
	      ranges::destroy_at(std::__addressof(*__first));
	    return __first;
	  }
      }
  };

  inline constexpr __destroy_n_fn destroy_n{};
}
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // concepts
#endif // C++20
#endif // _RANGES_UNINITIALIZED_H
