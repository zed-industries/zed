// Concepts and traits for use with iterators -*- C++ -*-

// Copyright (C) 2019-2020 Free Software Foundation, Inc.
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

/** @file bits/iterator_concepts.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iterator}
 */

#ifndef _ITERATOR_CONCEPTS_H
#define _ITERATOR_CONCEPTS_H 1

#pragma GCC system_header

#include <concepts>
#include <bits/ptr_traits.h>	// to_address
#include <bits/range_cmp.h>	// identity, ranges::less

#if __cpp_lib_concepts
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  struct input_iterator_tag;
  struct output_iterator_tag;
  struct forward_iterator_tag;
  struct bidirectional_iterator_tag;
  struct random_access_iterator_tag;
  struct contiguous_iterator_tag;

  template<typename _Iterator>
    struct iterator_traits;

  template<typename _Tp> requires is_object_v<_Tp>
    struct iterator_traits<_Tp*>;

  template<typename _Iterator, typename>
    struct __iterator_traits;

  namespace __detail
  {
    template<typename _Tp>
      using __with_ref = _Tp&;

    template<typename _Tp>
      concept __can_reference = requires { typename __with_ref<_Tp>; };

    template<typename _Tp>
      concept __dereferenceable = requires(_Tp& __t)
	{
	  { *__t } -> __can_reference;
	};
  } // namespace __detail

  template<__detail::__dereferenceable _Tp>
    using iter_reference_t = decltype(*std::declval<_Tp&>());

  namespace ranges
  {
    namespace __cust_imove
    {
      void iter_move();

      template<typename _Tp>
	concept __adl_imove
	  = (std::__detail::__class_or_enum<remove_reference_t<_Tp>>)
	  && requires(_Tp&& __t) { iter_move(static_cast<_Tp&&>(__t)); };

      struct _IMove
      {
      private:
	template<typename _Tp>
	  struct __result
	  { using type = iter_reference_t<_Tp>; };

	template<typename _Tp>
	  requires __adl_imove<_Tp>
	  struct __result<_Tp>
	  { using type = decltype(iter_move(std::declval<_Tp>())); };

	template<typename _Tp>
	  requires (!__adl_imove<_Tp>)
	  && is_lvalue_reference_v<iter_reference_t<_Tp>>
	  struct __result<_Tp>
	  { using type = remove_reference_t<iter_reference_t<_Tp>>&&; };

	template<typename _Tp>
	  static constexpr bool
	  _S_noexcept()
	  {
	    if constexpr (__adl_imove<_Tp>)
	      return noexcept(iter_move(std::declval<_Tp>()));
	    else
	      return noexcept(*std::declval<_Tp>());
	  }

      public:
	// The result type of iter_move(std::declval<_Tp>())
	template<std::__detail::__dereferenceable _Tp>
	  using __type = typename __result<_Tp>::type;

	template<std::__detail::__dereferenceable _Tp>
	  constexpr __type<_Tp>
	  operator()(_Tp&& __e) const
	  noexcept(_S_noexcept<_Tp>())
	  {
	    if constexpr (__adl_imove<_Tp>)
	      return iter_move(static_cast<_Tp&&>(__e));
	    else if constexpr (is_lvalue_reference_v<iter_reference_t<_Tp>>)
	      return static_cast<__type<_Tp>>(*__e);
	    else
	      return *__e;
	  }
      };
    } // namespace __cust_imove

    inline namespace __cust
    {
      inline constexpr __cust_imove::_IMove iter_move{};
    } // inline namespace __cust
  } // namespace ranges

  template<__detail::__dereferenceable _Tp>
    requires requires(_Tp& __t)
    { { ranges::iter_move(__t) } -> __detail::__can_reference; }
    using iter_rvalue_reference_t
      = decltype(ranges::iter_move(std::declval<_Tp&>()));

  template<typename> struct incrementable_traits { };

  template<typename _Tp> requires is_object_v<_Tp>
    struct incrementable_traits<_Tp*>
    { using difference_type = ptrdiff_t; };

  template<typename _Iter>
    struct incrementable_traits<const _Iter>
    : incrementable_traits<_Iter> { };

  template<typename _Tp> requires requires { typename _Tp::difference_type; }
    struct incrementable_traits<_Tp>
    { using difference_type = typename _Tp::difference_type; };

  template<typename _Tp>
    requires (!requires { typename _Tp::difference_type; }
	      && requires(const _Tp& __a, const _Tp& __b)
	      {
		requires (!is_void_v<remove_pointer_t<_Tp>>); // PR c++/78173
		{ __a - __b } -> integral;
	      })
    struct incrementable_traits<_Tp>
    {
      using difference_type
	= make_signed_t<decltype(std::declval<_Tp>() - std::declval<_Tp>())>;
    };

#if defined __STRICT_ANSI__ && defined __SIZEOF_INT128__
  // __int128 is incrementable even if !integral<__int128>
  template<>
    struct incrementable_traits<__int128>
    { using difference_type = __int128; };

  template<>
    struct incrementable_traits<unsigned __int128>
    { using difference_type = __int128; };
#endif

  namespace __detail
  {
    // An iterator such that iterator_traits<_Iter> names a specialization
    // generated from the primary template.
    template<typename _Iter>
      concept __primary_traits_iter
	= __is_base_of(__iterator_traits<_Iter, void>, iterator_traits<_Iter>);

    template<typename _Iter, typename _Tp>
      struct __iter_traits_impl
      { using type = iterator_traits<_Iter>; };

    template<typename _Iter, typename _Tp>
      requires __primary_traits_iter<_Iter>
      struct __iter_traits_impl<_Iter, _Tp>
      { using type = _Tp; };

    // ITER_TRAITS
    template<typename _Iter, typename _Tp = _Iter>
      using __iter_traits = typename __iter_traits_impl<_Iter, _Tp>::type;

    template<typename _Tp>
      using __iter_diff_t = typename
	__iter_traits<_Tp, incrementable_traits<_Tp>>::difference_type;
  } // namespace __detail

  template<typename _Tp>
    using iter_difference_t = __detail::__iter_diff_t<remove_cvref_t<_Tp>>;

  namespace __detail
  {
    template<typename> struct __cond_value_type { };

    template<typename _Tp> requires is_object_v<_Tp>
      struct __cond_value_type<_Tp>
      { using value_type = remove_cv_t<_Tp>; };
  } // namespace __detail

  template<typename> struct indirectly_readable_traits { };

  template<typename _Tp>
    struct indirectly_readable_traits<_Tp*>
    : __detail::__cond_value_type<_Tp>
    { };

  template<typename _Iter> requires is_array_v<_Iter>
    struct indirectly_readable_traits<_Iter>
    { using value_type = remove_cv_t<remove_extent_t<_Iter>>; };

  template<typename _Iter>
    struct indirectly_readable_traits<const _Iter>
    : indirectly_readable_traits<_Iter>
    { };

  template<typename _Tp> requires requires { typename _Tp::value_type; }
    struct indirectly_readable_traits<_Tp>
    : __detail::__cond_value_type<typename _Tp::value_type>
    { };

  template<typename _Tp> requires requires { typename _Tp::element_type; }
    struct indirectly_readable_traits<_Tp>
    : __detail::__cond_value_type<typename _Tp::element_type>
    { };

  namespace __detail
  {
    template<typename _Tp>
      using __iter_value_t = typename
	__iter_traits<_Tp, indirectly_readable_traits<_Tp>>::value_type;
  } // namespace __detail

  template<typename _Tp>
    using iter_value_t = __detail::__iter_value_t<remove_cvref_t<_Tp>>;

  namespace __detail
  {
    // _GLIBCXX_RESOLVE_LIB_DEFECTS
    // 3420. cpp17-iterator should check [type] looks like an iterator first
    template<typename _Iter>
      concept __cpp17_iterator = requires(_Iter __it)
	{
	  { *__it } -> __can_reference;
	  { ++__it } -> same_as<_Iter&>;
	  { *__it++ } -> __can_reference;
	} && copyable<_Iter>;

    template<typename _Iter>
      concept __cpp17_input_iterator = __cpp17_iterator<_Iter>
	&& equality_comparable<_Iter>
	&& requires(_Iter __it)
	{
	  typename incrementable_traits<_Iter>::difference_type;
	  typename indirectly_readable_traits<_Iter>::value_type;
	  typename common_reference_t<iter_reference_t<_Iter>&&,
		   typename indirectly_readable_traits<_Iter>::value_type&>;
	  typename common_reference_t<decltype(*__it++)&&,
		   typename indirectly_readable_traits<_Iter>::value_type&>;
	  requires signed_integral<
	    typename incrementable_traits<_Iter>::difference_type>;
	};

    template<typename _Iter>
      concept __cpp17_fwd_iterator = __cpp17_input_iterator<_Iter>
	&& constructible_from<_Iter>
	&& is_lvalue_reference_v<iter_reference_t<_Iter>>
	&& same_as<remove_cvref_t<iter_reference_t<_Iter>>,
		   typename indirectly_readable_traits<_Iter>::value_type>
	&& requires(_Iter __it)
	{
	  {  __it++ } -> convertible_to<const _Iter&>;
	  { *__it++ } -> same_as<iter_reference_t<_Iter>>;
	};

    template<typename _Iter>
      concept __cpp17_bidi_iterator = __cpp17_fwd_iterator<_Iter>
	&& requires(_Iter __it)
	{
	  {  --__it } -> same_as<_Iter&>;
	  {  __it-- } -> convertible_to<const _Iter&>;
	  { *__it-- } -> same_as<iter_reference_t<_Iter>>;
	};

    template<typename _Iter>
      concept __cpp17_randacc_iterator = __cpp17_bidi_iterator<_Iter>
	&& totally_ordered<_Iter>
	&& requires(_Iter __it,
		    typename incrementable_traits<_Iter>::difference_type __n)
	{
	  { __it += __n } -> same_as<_Iter&>;
	  { __it -= __n } -> same_as<_Iter&>;
	  { __it +  __n } -> same_as<_Iter>;
	  { __n +  __it } -> same_as<_Iter>;
	  { __it -  __n } -> same_as<_Iter>;
	  { __it -  __it } -> same_as<decltype(__n)>;
	  {  __it[__n]  } -> convertible_to<iter_reference_t<_Iter>>;
	};

    template<typename _Iter>
      concept __iter_with_nested_types = requires {
	typename _Iter::iterator_category;
	typename _Iter::value_type;
	typename _Iter::difference_type;
	typename _Iter::reference;
      };

    template<typename _Iter>
      concept __iter_without_nested_types = !__iter_with_nested_types<_Iter>;

    template<typename _Iter>
      concept __iter_without_category
	= !requires { typename _Iter::iterator_category; };

  } // namespace __detail

  template<typename _Iterator>
    requires __detail::__iter_with_nested_types<_Iterator>
    struct __iterator_traits<_Iterator, void>
    {
    private:
      template<typename _Iter>
	struct __ptr
	{ using type = void; };

      template<typename _Iter> requires requires { typename _Iter::pointer; }
	struct __ptr<_Iter>
	{ using type = typename _Iter::pointer; };

    public:
      using iterator_category = typename _Iterator::iterator_category;
      using value_type	      = typename _Iterator::value_type;
      using difference_type   = typename _Iterator::difference_type;
      using pointer	      = typename __ptr<_Iterator>::type;
      using reference	      = typename _Iterator::reference;
    };

  template<typename _Iterator>
    requires __detail::__iter_without_nested_types<_Iterator>
	      && __detail::__cpp17_input_iterator<_Iterator>
    struct __iterator_traits<_Iterator, void>
    {
    private:
      template<typename _Iter>
	struct __cat
	{ using type = input_iterator_tag; };

      template<typename _Iter>
	requires requires { typename _Iter::iterator_category; }
	struct __cat<_Iter>
	{ using type = typename _Iter::iterator_category; };

      template<typename _Iter>
	requires __detail::__iter_without_category<_Iter>
		  && __detail::__cpp17_randacc_iterator<_Iter>
	struct __cat<_Iter>
	{ using type = random_access_iterator_tag; };

      template<typename _Iter>
	requires __detail::__iter_without_category<_Iter>
		  && __detail::__cpp17_bidi_iterator<_Iter>
	struct __cat<_Iter>
	{ using type = bidirectional_iterator_tag; };

      template<typename _Iter>
	requires __detail::__iter_without_category<_Iter>
		  && __detail::__cpp17_fwd_iterator<_Iter>
	struct __cat<_Iter>
	{ using type = forward_iterator_tag; };

      template<typename _Iter>
	struct __ptr
	{ using type = void; };

      template<typename _Iter> requires requires { typename _Iter::pointer; }
	struct __ptr<_Iter>
	{ using type = typename _Iter::pointer; };

      template<typename _Iter>
	requires (!requires { typename _Iter::pointer; }
	    && requires(_Iter& __it) { __it.operator->(); })
	struct __ptr<_Iter>
	{ using type = decltype(std::declval<_Iter&>().operator->()); };

      template<typename _Iter>
	struct __ref
	{ using type = iter_reference_t<_Iter>; };

      template<typename _Iter> requires requires { typename _Iter::reference; }
	struct __ref<_Iter>
	{ using type = typename _Iter::reference; };

    public:
      using iterator_category = typename __cat<_Iterator>::type;
      using value_type
	= typename indirectly_readable_traits<_Iterator>::value_type;
      using difference_type
	= typename incrementable_traits<_Iterator>::difference_type;
      using pointer	      = typename __ptr<_Iterator>::type;
      using reference	      = typename __ref<_Iterator>::type;
    };

  template<typename _Iterator>
    requires __detail::__iter_without_nested_types<_Iterator>
	      && __detail::__cpp17_iterator<_Iterator>
    struct __iterator_traits<_Iterator, void>
    {
    private:
      template<typename _Iter>
	struct __diff
	{ using type = void; };

      template<typename _Iter>
	requires requires
	{ typename incrementable_traits<_Iter>::difference_type; }
	struct __diff<_Iter>
	{
	  using type = typename incrementable_traits<_Iter>::difference_type;
	};

    public:
      using iterator_category = output_iterator_tag;
      using value_type	      = void;
      using difference_type   = typename __diff<_Iterator>::type;
      using pointer	      = void;
      using reference	      = void;
    };

  namespace __detail
  {
    template<typename _Iter>
      struct __iter_concept_impl;

    // ITER_CONCEPT(I) is ITER_TRAITS(I)::iterator_concept if that is valid.
    template<typename _Iter>
      requires requires { typename __iter_traits<_Iter>::iterator_concept; }
      struct __iter_concept_impl<_Iter>
      { using type = typename __iter_traits<_Iter>::iterator_concept; };

    // Otherwise, ITER_TRAITS(I)::iterator_category if that is valid.
    template<typename _Iter>
      requires (!requires { typename __iter_traits<_Iter>::iterator_concept; }
	  && requires { typename __iter_traits<_Iter>::iterator_category; })
      struct __iter_concept_impl<_Iter>
      { using type = typename __iter_traits<_Iter>::iterator_category; };

    // Otherwise, random_access_tag if iterator_traits<I> is not specialized.
    template<typename _Iter>
      requires (!requires { typename __iter_traits<_Iter>::iterator_concept; }
	  && !requires { typename __iter_traits<_Iter>::iterator_category; }
	  && __primary_traits_iter<_Iter>)
      struct __iter_concept_impl<_Iter>
      { using type = random_access_iterator_tag; };

    // Otherwise, there is no ITER_CONCEPT(I) type.
    template<typename _Iter>
      struct __iter_concept_impl
      { };

    // ITER_CONCEPT
    template<typename _Iter>
      using __iter_concept = typename __iter_concept_impl<_Iter>::type;

  template<typename _In>
    concept __indirectly_readable_impl = requires(const _In __in)
      {
	typename iter_value_t<_In>;
	typename iter_reference_t<_In>;
	typename iter_rvalue_reference_t<_In>;
	{ *__in } -> same_as<iter_reference_t<_In>>;
	{ ranges::iter_move(__in) } -> same_as<iter_rvalue_reference_t<_In>>;
      }
      && common_reference_with<iter_reference_t<_In>&&, iter_value_t<_In>&>
      && common_reference_with<iter_reference_t<_In>&&,
			      iter_rvalue_reference_t<_In>&&>
      && common_reference_with<iter_rvalue_reference_t<_In>&&,
			       const iter_value_t<_In>&>;

  } // namespace __detail

  /// Requirements for types that are readable by applying operator*.
  template<typename _In>
    concept indirectly_readable
      = __detail::__indirectly_readable_impl<remove_cvref_t<_In>>;

  template<indirectly_readable _Tp>
    using iter_common_reference_t
      = common_reference_t<iter_reference_t<_Tp>, iter_value_t<_Tp>&>;

  /// Requirements for writing a value into an iterator's referenced object.
  template<typename _Out, typename _Tp>
    concept indirectly_writable = requires(_Out&& __o, _Tp&& __t)
      {
	*__o = std::forward<_Tp>(__t);
	*std::forward<_Out>(__o) = std::forward<_Tp>(__t);
	const_cast<const iter_reference_t<_Out>&&>(*__o)
	  = std::forward<_Tp>(__t);
	const_cast<const iter_reference_t<_Out>&&>(*std::forward<_Out>(__o))
	  = std::forward<_Tp>(__t);
      };

  namespace ranges::__detail
  {
#if __SIZEOF_INT128__
    using __max_diff_type = __int128;
    using __max_size_type = unsigned __int128;
#else
    using __max_diff_type = long long;
    using __max_size_type = unsigned long long;
#endif

    template<typename _Tp>
      concept __is_integer_like = integral<_Tp>
	|| same_as<_Tp, __max_diff_type> || same_as<_Tp, __max_size_type>;

    template<typename _Tp>
      concept __is_signed_integer_like = signed_integral<_Tp>
	|| same_as<_Tp, __max_diff_type>;

  } // namespace ranges::__detail

  namespace __detail { using ranges::__detail::__is_signed_integer_like; }

  /// Requirements on types that can be incremented with ++.
  template<typename _Iter>
    concept weakly_incrementable = default_initializable<_Iter>
      && movable<_Iter>
      && requires(_Iter __i)
      {
	typename iter_difference_t<_Iter>;
	requires __detail::__is_signed_integer_like<iter_difference_t<_Iter>>;
	{ ++__i } -> same_as<_Iter&>;
	__i++;
      };

  template<typename _Iter>
    concept incrementable = regular<_Iter> && weakly_incrementable<_Iter>
      && requires(_Iter __i) { { __i++ } -> same_as<_Iter>; };

  template<typename _Iter>
    concept input_or_output_iterator
      = requires(_Iter __i) { { *__i } -> __detail::__can_reference; }
	&& weakly_incrementable<_Iter>;

  template<typename _Sent, typename _Iter>
    concept sentinel_for = semiregular<_Sent>
      && input_or_output_iterator<_Iter>
      && __detail::__weakly_eq_cmp_with<_Sent, _Iter>;

  template<typename _Sent, typename _Iter>
    inline constexpr bool disable_sized_sentinel_for = false;

  template<typename _Sent, typename _Iter>
    concept sized_sentinel_for = sentinel_for<_Sent, _Iter>
    && !disable_sized_sentinel_for<remove_cv_t<_Sent>, remove_cv_t<_Iter>>
    && requires(const _Iter& __i, const _Sent& __s)
    {
      { __s - __i } -> same_as<iter_difference_t<_Iter>>;
      { __i - __s } -> same_as<iter_difference_t<_Iter>>;
    };

  template<typename _Iter>
    concept input_iterator = input_or_output_iterator<_Iter>
      && indirectly_readable<_Iter>
      && requires { typename __detail::__iter_concept<_Iter>; }
      && derived_from<__detail::__iter_concept<_Iter>, input_iterator_tag>;

  template<typename _Iter, typename _Tp>
    concept output_iterator = input_or_output_iterator<_Iter>
      && indirectly_writable<_Iter, _Tp>
      && requires(_Iter __i, _Tp&& __t) { *__i++ = std::forward<_Tp>(__t); };

  template<typename _Iter>
    concept forward_iterator = input_iterator<_Iter>
      && derived_from<__detail::__iter_concept<_Iter>, forward_iterator_tag>
      && incrementable<_Iter> && sentinel_for<_Iter, _Iter>;

  template<typename _Iter>
    concept bidirectional_iterator = forward_iterator<_Iter>
      && derived_from<__detail::__iter_concept<_Iter>,
		      bidirectional_iterator_tag>
      && requires(_Iter __i)
      {
	{ --__i } -> same_as<_Iter&>;
	{ __i-- } -> same_as<_Iter>;
      };

  template<typename _Iter>
    concept random_access_iterator = bidirectional_iterator<_Iter>
      && derived_from<__detail::__iter_concept<_Iter>,
		      random_access_iterator_tag>
      && totally_ordered<_Iter> && sized_sentinel_for<_Iter, _Iter>
      && requires(_Iter __i, const _Iter __j,
		  const iter_difference_t<_Iter> __n)
      {
	{ __i += __n } -> same_as<_Iter&>;
	{ __j +  __n } -> same_as<_Iter>;
	{ __n +  __j } -> same_as<_Iter>;
	{ __i -= __n } -> same_as<_Iter&>;
	{ __j -  __n } -> same_as<_Iter>;
	{  __j[__n]  } -> same_as<iter_reference_t<_Iter>>;
      };

  template<typename _Iter>
    concept contiguous_iterator = random_access_iterator<_Iter>
      && derived_from<__detail::__iter_concept<_Iter>, contiguous_iterator_tag>
      && is_lvalue_reference_v<iter_reference_t<_Iter>>
      && same_as<iter_value_t<_Iter>, remove_cvref_t<iter_reference_t<_Iter>>>
      && requires(const _Iter& __i)
      {
	{ std::to_address(__i) }
	  -> same_as<add_pointer_t<iter_reference_t<_Iter>>>;
      };

  // [indirectcallable], indirect callable requirements

  // [indirectcallable.indirectinvocable], indirect callables

  template<typename _Fn, typename _Iter>
    concept indirectly_unary_invocable = indirectly_readable<_Iter>
      && copy_constructible<_Fn> && invocable<_Fn&, iter_value_t<_Iter>&>
      && invocable<_Fn&, iter_reference_t<_Iter>>
      && invocable<_Fn&, iter_common_reference_t<_Iter>>
      && common_reference_with<invoke_result_t<_Fn&, iter_value_t<_Iter>&>,
			       invoke_result_t<_Fn&, iter_reference_t<_Iter>>>;

  template<typename _Fn, typename _Iter>
    concept indirectly_regular_unary_invocable = indirectly_readable<_Iter>
      && copy_constructible<_Fn>
      && regular_invocable<_Fn&, iter_value_t<_Iter>&>
      && regular_invocable<_Fn&, iter_reference_t<_Iter>>
      && regular_invocable<_Fn&, iter_common_reference_t<_Iter>>
      && common_reference_with<invoke_result_t<_Fn&, iter_value_t<_Iter>&>,
			       invoke_result_t<_Fn&, iter_reference_t<_Iter>>>;

  template<typename _Fn, typename _Iter>
    concept indirect_unary_predicate = indirectly_readable<_Iter>
      && copy_constructible<_Fn> && predicate<_Fn&, iter_value_t<_Iter>&>
      && predicate<_Fn&, iter_reference_t<_Iter>>
      && predicate<_Fn&, iter_common_reference_t<_Iter>>;

  template<typename _Fn, typename _I1, typename _I2>
    concept indirect_binary_predicate
      = indirectly_readable<_I1> && indirectly_readable<_I2>
      && copy_constructible<_Fn>
      && predicate<_Fn&, iter_value_t<_I1>&, iter_value_t<_I2>&>
      && predicate<_Fn&, iter_value_t<_I1>&, iter_reference_t<_I2>>
      && predicate<_Fn&, iter_reference_t<_I1>, iter_value_t<_I2>&>
      && predicate<_Fn&, iter_reference_t<_I1>, iter_reference_t<_I2>>
      && predicate<_Fn&, iter_common_reference_t<_I1>,
		   iter_common_reference_t<_I2>>;

  template<typename _Fn, typename _I1, typename _I2 = _I1>
    concept indirect_equivalence_relation
      = indirectly_readable<_I1> && indirectly_readable<_I2>
      && copy_constructible<_Fn>
      && equivalence_relation<_Fn&, iter_value_t<_I1>&, iter_value_t<_I2>&>
      && equivalence_relation<_Fn&, iter_value_t<_I1>&, iter_reference_t<_I2>>
      && equivalence_relation<_Fn&, iter_reference_t<_I1>, iter_value_t<_I2>&>
      && equivalence_relation<_Fn&, iter_reference_t<_I1>,
			      iter_reference_t<_I2>>
      && equivalence_relation<_Fn&, iter_common_reference_t<_I1>,
			      iter_common_reference_t<_I2>>;

  template<typename _Fn, typename _I1, typename _I2 = _I1>
    concept indirect_strict_weak_order
      = indirectly_readable<_I1> && indirectly_readable<_I2>
      && copy_constructible<_Fn>
      && strict_weak_order<_Fn&, iter_value_t<_I1>&, iter_value_t<_I2>&>
      && strict_weak_order<_Fn&, iter_value_t<_I1>&, iter_reference_t<_I2>>
      && strict_weak_order<_Fn&, iter_reference_t<_I1>, iter_value_t<_I2>&>
      && strict_weak_order<_Fn&, iter_reference_t<_I1>, iter_reference_t<_I2>>
      && strict_weak_order<_Fn&, iter_common_reference_t<_I1>,
			   iter_common_reference_t<_I2>>;

  template<typename _Fn, typename... _Is>
    requires (indirectly_readable<_Is> && ...)
      && invocable<_Fn, iter_reference_t<_Is>...>
    using indirect_result_t = invoke_result_t<_Fn, iter_reference_t<_Is>...>;

  /// [projected], projected
  template<indirectly_readable _Iter,
	   indirectly_regular_unary_invocable<_Iter> _Proj>
    struct projected
    {
      using value_type = remove_cvref_t<indirect_result_t<_Proj&, _Iter>>;

      indirect_result_t<_Proj&, _Iter> operator*() const; // not defined
    };

  template<weakly_incrementable _Iter, typename _Proj>
    struct incrementable_traits<projected<_Iter, _Proj>>
    { using difference_type = iter_difference_t<_Iter>; };

  // [alg.req], common algorithm requirements

  /// [alg.req.ind.move], concept `indirectly_movable`

  template<typename _In, typename _Out>
    concept indirectly_movable = indirectly_readable<_In>
      && indirectly_writable<_Out, iter_rvalue_reference_t<_In>>;

  template<typename _In, typename _Out>
    concept indirectly_movable_storable = indirectly_movable<_In, _Out>
      && indirectly_writable<_Out, iter_value_t<_In>>
      && movable<iter_value_t<_In>>
      && constructible_from<iter_value_t<_In>, iter_rvalue_reference_t<_In>>
      && assignable_from<iter_value_t<_In>&, iter_rvalue_reference_t<_In>>;

  /// [alg.req.ind.copy], concept `indirectly_copyable`
  template<typename _In, typename _Out>
    concept indirectly_copyable = indirectly_readable<_In>
      && indirectly_writable<_Out, iter_reference_t<_In>>;

  template<typename _In, typename _Out>
    concept indirectly_copyable_storable = indirectly_copyable<_In, _Out>
      && indirectly_writable<_Out, iter_value_t<_In>&>
      && indirectly_writable<_Out, const iter_value_t<_In>&>
      && indirectly_writable<_Out, iter_value_t<_In>&&>
      && indirectly_writable<_Out, const iter_value_t<_In>&&>
      && copyable<iter_value_t<_In>>
      && constructible_from<iter_value_t<_In>, iter_reference_t<_In>>
      && assignable_from<iter_value_t<_In>&, iter_reference_t<_In>>;

namespace ranges
{
  namespace __cust_iswap
  {
    template<typename _It1, typename _It2>
      void iter_swap(_It1, _It2) = delete;

    template<typename _Tp, typename _Up>
      concept __adl_iswap
	= (std::__detail::__class_or_enum<remove_reference_t<_Tp>>
	  || std::__detail::__class_or_enum<remove_reference_t<_Up>>)
	&& requires(_Tp&& __t, _Up&& __u) {
	  iter_swap(static_cast<_Tp&&>(__t), static_cast<_Up&&>(__u));
	};

    template<typename _Xp, typename _Yp>
      constexpr iter_value_t<_Xp>
      __iter_exchange_move(_Xp&& __x, _Yp&& __y)
      noexcept(noexcept(iter_value_t<_Xp>(iter_move(__x)))
	       && noexcept(*__x = iter_move(__y)))
      {
	iter_value_t<_Xp> __old_value(iter_move(__x));
	*__x = iter_move(__y);
	return __old_value;
      }

    struct _IterSwap
    {
    private:
      template<typename _Tp, typename _Up>
	static constexpr bool
	_S_noexcept()
	{
	  if constexpr (__adl_iswap<_Tp, _Up>)
	    return noexcept(iter_swap(std::declval<_Tp>(),
				      std::declval<_Up>()));
	  else if constexpr (indirectly_readable<_Tp>
	      && indirectly_readable<_Up>
	      && swappable_with<iter_reference_t<_Tp>, iter_reference_t<_Up>>)
	    return noexcept(ranges::swap(*std::declval<_Tp>(),
					 *std::declval<_Up>()));
	  else
	    return noexcept(*std::declval<_Tp>()
		= __iter_exchange_move(std::declval<_Up>(),
				       std::declval<_Tp>()));
	}

    public:
      template<typename _Tp, typename _Up>
	requires __adl_iswap<_Tp, _Up>
	|| (indirectly_readable<remove_reference_t<_Tp>>
	    && indirectly_readable<remove_reference_t<_Up>>
	    && swappable_with<iter_reference_t<_Tp>, iter_reference_t<_Up>>)
	|| (indirectly_movable_storable<_Tp, _Up>
	    && indirectly_movable_storable<_Up, _Tp>)
	constexpr void
	operator()(_Tp&& __e1, _Up&& __e2) const
	noexcept(_S_noexcept<_Tp, _Up>())
	{
	  if constexpr (__adl_iswap<_Tp, _Up>)
	    iter_swap(static_cast<_Tp&&>(__e1), static_cast<_Up&&>(__e2));
	  else if constexpr (indirectly_readable<_Tp>
	      && indirectly_readable<_Up>
	      && swappable_with<iter_reference_t<_Tp>, iter_reference_t<_Up>>)
	    ranges::swap(*__e1, *__e2);
	  else
	    *__e1 = __iter_exchange_move(__e2, __e1);
	}
    };
  } // namespace __cust_iswap

  inline namespace __cust
  {
    inline constexpr __cust_iswap::_IterSwap iter_swap{};
  } // inline namespace __cust

} // namespace ranges

  /// [alg.req.ind.swap], concept `indirectly_swappable`
  template<typename _I1, typename _I2 = _I1>
    concept indirectly_swappable
      = indirectly_readable<_I1> && indirectly_readable<_I2>
      && requires(const _I1 __i1, const _I2 __i2)
      {
	ranges::iter_swap(__i1, __i1);
	ranges::iter_swap(__i2, __i2);
	ranges::iter_swap(__i1, __i2);
	ranges::iter_swap(__i2, __i1);
      };

  /// [alg.req.ind.cmp], concept `indirectly_comparable`
  template<typename _I1, typename _I2, typename _Rel, typename _P1 = identity,
	   typename _P2 = identity>
    concept indirectly_comparable
      = indirect_binary_predicate<_Rel, projected<_I1, _P1>,
				  projected<_I2, _P2>>;

  /// [alg.req.permutable], concept `permutable`
  template<typename _Iter>
    concept permutable = forward_iterator<_Iter>
      && indirectly_movable_storable<_Iter, _Iter>
      && indirectly_swappable<_Iter, _Iter>;

  /// [alg.req.mergeable], concept `mergeable`
  template<typename _I1, typename _I2, typename _Out,
	   typename _Rel = ranges::less, typename _P1 = identity,
	   typename _P2 = identity>
    concept mergeable = input_iterator<_I1> && input_iterator<_I2>
      && weakly_incrementable<_Out> && indirectly_copyable<_I1, _Out>
      && indirectly_copyable<_I2, _Out>
      && indirect_strict_weak_order<_Rel, projected<_I1, _P1>,
				    projected<_I2, _P2>>;

  /// [alg.req.sortable], concept `sortable`
  template<typename _Iter, typename _Rel = ranges::less,
	   typename _Proj = identity>
    concept sortable = permutable<_Iter>
      && indirect_strict_weak_order<_Rel, projected<_Iter, _Proj>>;

  struct unreachable_sentinel_t
  {
    template<weakly_incrementable _It>
      friend constexpr bool
      operator==(unreachable_sentinel_t, const _It&) noexcept
      { return false; }
  };

  inline constexpr unreachable_sentinel_t unreachable_sentinel{};

  struct default_sentinel_t { };
  inline constexpr default_sentinel_t default_sentinel{};

  namespace __detail
  {
    template<typename _Tp>
      constexpr decay_t<_Tp>
      __decay_copy(_Tp&& __t)
      noexcept(is_nothrow_convertible_v<_Tp, decay_t<_Tp>>)
      { return std::forward<_Tp>(__t); }

    template<typename _Tp>
      concept __member_begin = requires(_Tp& __t)
	{
	  { __detail::__decay_copy(__t.begin()) } -> input_or_output_iterator;
	};

    void begin(auto&) = delete;
    void begin(const auto&) = delete;

    template<typename _Tp>
      concept __adl_begin = __class_or_enum<remove_reference_t<_Tp>>
	&& requires(_Tp& __t)
	{
	  { __detail::__decay_copy(begin(__t)) } -> input_or_output_iterator;
	};

    // Simplified version of std::ranges::begin that only supports lvalues,
    // for use by __range_iter_t below.
    template<typename _Tp>
      requires is_array_v<_Tp> || __member_begin<_Tp&> || __adl_begin<_Tp&>
      auto
      __ranges_begin(_Tp& __t)
      {
	if constexpr (is_array_v<_Tp>)
	  {
	    static_assert(sizeof(remove_all_extents_t<_Tp>) != 0,
			  "not array of incomplete type");
	    return __t + 0;
	  }
	else if constexpr (__member_begin<_Tp&>)
	  return __t.begin();
	else
	  return begin(__t);
      }

    // Implementation of std::ranges::iterator_t, without using ranges::begin.
    template<typename _Tp>
      using __range_iter_t
	= decltype(__detail::__ranges_begin(std::declval<_Tp&>()));

  } // namespace __detail

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // C++20 library concepts
#endif // _ITERATOR_CONCEPTS_H
