// Iterators -*- C++ -*-

// Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

/*
 *
 * Copyright (c) 1994
 * Hewlett-Packard Company
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Hewlett-Packard Company makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 *
 * Copyright (c) 1996-1998
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 */

/** @file bits/stl_iterator.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iterator}
 *
 *  This file implements reverse_iterator, back_insert_iterator,
 *  front_insert_iterator, insert_iterator, __normal_iterator, and their
 *  supporting functions and overloaded operators.
 */

#ifndef _STL_ITERATOR_H
#define _STL_ITERATOR_H 1

#include <bits/cpp_type_traits.h>
#include <ext/type_traits.h>
#include <bits/move.h>
#include <bits/ptr_traits.h>

#if __cplusplus >= 201103L
# include <type_traits>
#endif

#if __cplusplus > 201703L
# define __cpp_lib_array_constexpr 201811L
# define __cpp_lib_constexpr_iterator 201811L
#elif __cplusplus == 201703L
# define __cpp_lib_array_constexpr 201803L
#endif

#if __cplusplus > 201703L
# include <compare>
# include <new>
# include <bits/iterator_concepts.h>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup iterators
   * @{
   */

#if __cplusplus > 201703L && __cpp_lib_concepts
  namespace __detail
  {
    // Weaken iterator_category _Cat to _Limit if it is derived from that,
    // otherwise use _Otherwise.
    template<typename _Cat, typename _Limit, typename _Otherwise = _Cat>
      using __clamp_iter_cat
	= conditional_t<derived_from<_Cat, _Limit>, _Limit, _Otherwise>;
  }
#endif

  // 24.4.1 Reverse iterators
  /**
   *  Bidirectional and random access iterators have corresponding reverse
   *  %iterator adaptors that iterate through the data structure in the
   *  opposite direction.  They have the same signatures as the corresponding
   *  iterators.  The fundamental relation between a reverse %iterator and its
   *  corresponding %iterator @c i is established by the identity:
   *  @code
   *      &*(reverse_iterator(i)) == &*(i - 1)
   *  @endcode
   *
   *  <em>This mapping is dictated by the fact that while there is always a
   *  pointer past the end of an array, there might not be a valid pointer
   *  before the beginning of an array.</em> [24.4.1]/1,2
   *
   *  Reverse iterators can be tricky and surprising at first.  Their
   *  semantics make sense, however, and the trickiness is a side effect of
   *  the requirement that the iterators must be safe.
  */
  template<typename _Iterator>
    class reverse_iterator
    : public iterator<typename iterator_traits<_Iterator>::iterator_category,
		      typename iterator_traits<_Iterator>::value_type,
		      typename iterator_traits<_Iterator>::difference_type,
		      typename iterator_traits<_Iterator>::pointer,
                      typename iterator_traits<_Iterator>::reference>
    {
    protected:
      _Iterator current;

      typedef iterator_traits<_Iterator>		__traits_type;

    public:
      typedef _Iterator					iterator_type;
      typedef typename __traits_type::difference_type	difference_type;
      typedef typename __traits_type::pointer		pointer;
      typedef typename __traits_type::reference		reference;

#if __cplusplus > 201703L && __cpp_lib_concepts
      using iterator_concept
	= conditional_t<random_access_iterator<_Iterator>,
			random_access_iterator_tag,
			bidirectional_iterator_tag>;
      using iterator_category
	= __detail::__clamp_iter_cat<typename __traits_type::iterator_category,
				     random_access_iterator_tag>;
#endif

      /**
       *  The default constructor value-initializes member @p current.
       *  If it is a pointer, that means it is zero-initialized.
      */
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 235 No specification of default ctor for reverse_iterator
      // 1012. reverse_iterator default ctor should value initialize
      _GLIBCXX17_CONSTEXPR
      reverse_iterator() : current() { }

      /**
       *  This %iterator will move in the opposite direction that @p x does.
      */
      explicit _GLIBCXX17_CONSTEXPR
      reverse_iterator(iterator_type __x) : current(__x) { }

      /**
       *  The copy constructor is normal.
      */
      _GLIBCXX17_CONSTEXPR
      reverse_iterator(const reverse_iterator& __x)
      : current(__x.current) { }

#if __cplusplus >= 201103L
      reverse_iterator& operator=(const reverse_iterator&) = default;
#endif

      /**
       *  A %reverse_iterator across other types can be copied if the
       *  underlying %iterator can be converted to the type of @c current.
      */
      template<typename _Iter>
	_GLIBCXX17_CONSTEXPR
        reverse_iterator(const reverse_iterator<_Iter>& __x)
	: current(__x.base()) { }

      /**
       *  @return  @c current, the %iterator used for underlying work.
      */
      _GLIBCXX17_CONSTEXPR iterator_type
      base() const
      { return current; }

      /**
       *  @return  A reference to the value at @c --current
       *
       *  This requires that @c --current is dereferenceable.
       *
       *  @warning This implementation requires that for an iterator of the
       *           underlying iterator type, @c x, a reference obtained by
       *           @c *x remains valid after @c x has been modified or
       *           destroyed. This is a bug: http://gcc.gnu.org/PR51823
      */
      _GLIBCXX17_CONSTEXPR reference
      operator*() const
      {
	_Iterator __tmp = current;
	return *--__tmp;
      }

      /**
       *  @return  A pointer to the value at @c --current
       *
       *  This requires that @c --current is dereferenceable.
      */
      _GLIBCXX17_CONSTEXPR pointer
      operator->() const
#if __cplusplus > 201703L && __cpp_concepts >= 201907L
      requires is_pointer_v<_Iterator>
	|| requires(const _Iterator __i) { __i.operator->(); }
#endif
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 1052. operator-> should also support smart pointers
	_Iterator __tmp = current;
	--__tmp;
	return _S_to_pointer(__tmp);
      }

      /**
       *  @return  @c *this
       *
       *  Decrements the underlying iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator&
      operator++()
      {
	--current;
	return *this;
      }

      /**
       *  @return  The original value of @c *this
       *
       *  Decrements the underlying iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator
      operator++(int)
      {
	reverse_iterator __tmp = *this;
	--current;
	return __tmp;
      }

      /**
       *  @return  @c *this
       *
       *  Increments the underlying iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator&
      operator--()
      {
	++current;
	return *this;
      }

      /**
       *  @return  A reverse_iterator with the previous value of @c *this
       *
       *  Increments the underlying iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator
      operator--(int)
      {
	reverse_iterator __tmp = *this;
	++current;
	return __tmp;
      }

      /**
       *  @return  A reverse_iterator that refers to @c current - @a __n
       *
       *  The underlying iterator must be a Random Access Iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator
      operator+(difference_type __n) const
      { return reverse_iterator(current - __n); }

      /**
       *  @return  *this
       *
       *  Moves the underlying iterator backwards @a __n steps.
       *  The underlying iterator must be a Random Access Iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator&
      operator+=(difference_type __n)
      {
	current -= __n;
	return *this;
      }

      /**
       *  @return  A reverse_iterator that refers to @c current - @a __n
       *
       *  The underlying iterator must be a Random Access Iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator
      operator-(difference_type __n) const
      { return reverse_iterator(current + __n); }

      /**
       *  @return  *this
       *
       *  Moves the underlying iterator forwards @a __n steps.
       *  The underlying iterator must be a Random Access Iterator.
      */
      _GLIBCXX17_CONSTEXPR reverse_iterator&
      operator-=(difference_type __n)
      {
	current += __n;
	return *this;
      }

      /**
       *  @return  The value at @c current - @a __n - 1
       *
       *  The underlying iterator must be a Random Access Iterator.
      */
      _GLIBCXX17_CONSTEXPR reference
      operator[](difference_type __n) const
      { return *(*this + __n); }

#if __cplusplus > 201703L && __cpp_lib_concepts
      friend constexpr iter_rvalue_reference_t<_Iterator>
      iter_move(const reverse_iterator& __i)
      noexcept(is_nothrow_copy_constructible_v<_Iterator>
	       && noexcept(ranges::iter_move(--std::declval<_Iterator&>())))
      {
	auto __tmp = __i.base();
	return ranges::iter_move(--__tmp);
      }

      template<indirectly_swappable<_Iterator> _Iter2>
	friend constexpr void
	iter_swap(const reverse_iterator& __x,
		  const reverse_iterator<_Iter2>& __y)
	noexcept(is_nothrow_copy_constructible_v<_Iterator>
		 && is_nothrow_copy_constructible_v<_Iter2>
		 && noexcept(ranges::iter_swap(--std::declval<_Iterator&>(),
					       --std::declval<_Iter2&>())))
	{
	  auto __xtmp = __x.base();
	  auto __ytmp = __y.base();
	  ranges::iter_swap(--__xtmp, --__ytmp);
	}
#endif

    private:
      template<typename _Tp>
	static _GLIBCXX17_CONSTEXPR _Tp*
	_S_to_pointer(_Tp* __p)
        { return __p; }

      template<typename _Tp>
	static _GLIBCXX17_CONSTEXPR pointer
	_S_to_pointer(_Tp __t)
        { return __t.operator->(); }
    };

  //@{
  /**
   *  @param  __x  A %reverse_iterator.
   *  @param  __y  A %reverse_iterator.
   *  @return  A simple bool.
   *
   *  Reverse iterators forward comparisons to their underlying base()
   *  iterators.
   *
  */
#if __cplusplus <= 201703L || ! defined __cpp_lib_concepts
  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator==(const reverse_iterator<_Iterator>& __x,
	       const reverse_iterator<_Iterator>& __y)
    { return __x.base() == __y.base(); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<(const reverse_iterator<_Iterator>& __x,
	      const reverse_iterator<_Iterator>& __y)
    { return __y.base() < __x.base(); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator!=(const reverse_iterator<_Iterator>& __x,
	       const reverse_iterator<_Iterator>& __y)
    { return !(__x == __y); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>(const reverse_iterator<_Iterator>& __x,
	      const reverse_iterator<_Iterator>& __y)
    { return __y < __x; }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<=(const reverse_iterator<_Iterator>& __x,
	       const reverse_iterator<_Iterator>& __y)
    { return !(__y < __x); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>=(const reverse_iterator<_Iterator>& __x,
	       const reverse_iterator<_Iterator>& __y)
    { return !(__x < __y); }

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // DR 280. Comparison of reverse_iterator to const reverse_iterator.
  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator==(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    { return __x.base() == __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    { return __y.base() < __x.base(); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator!=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    { return !(__x == __y); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    { return __y < __x; }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    { return !(__y < __x); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    { return !(__x < __y); }
#else // C++20
  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator==(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() == __y.base() } -> convertible_to<bool>; }
    { return __x.base() == __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator!=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() != __y.base() } -> convertible_to<bool>; }
    { return __x.base() != __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator<(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() > __y.base() } -> convertible_to<bool>; }
    { return __x.base() > __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator>(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() < __y.base() } -> convertible_to<bool>; }
    { return __x.base() < __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator<=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() >= __y.base() } -> convertible_to<bool>; }
    { return __x.base() >= __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    constexpr bool
    operator>=(const reverse_iterator<_IteratorL>& __x,
	       const reverse_iterator<_IteratorR>& __y)
    requires requires { { __x.base() <= __y.base() } -> convertible_to<bool>; }
    { return __x.base() <= __y.base(); }

  template<typename _IteratorL,
	   three_way_comparable_with<_IteratorL> _IteratorR>
    constexpr compare_three_way_result_t<_IteratorL, _IteratorR>
    operator<=>(const reverse_iterator<_IteratorL>& __x,
		const reverse_iterator<_IteratorR>& __y)
    { return __y.base() <=> __x.base(); }
#endif // C++20
  //@}

#if __cplusplus < 201103L
  template<typename _Iterator>
    inline typename reverse_iterator<_Iterator>::difference_type
    operator-(const reverse_iterator<_Iterator>& __x,
	      const reverse_iterator<_Iterator>& __y)
    { return __y.base() - __x.base(); }

  template<typename _IteratorL, typename _IteratorR>
    inline typename reverse_iterator<_IteratorL>::difference_type
    operator-(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    { return __y.base() - __x.base(); }
#else
  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // DR 685. reverse_iterator/move_iterator difference has invalid signatures
  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR auto
    operator-(const reverse_iterator<_IteratorL>& __x,
	      const reverse_iterator<_IteratorR>& __y)
    -> decltype(__y.base() - __x.base())
    { return __y.base() - __x.base(); }
#endif

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR reverse_iterator<_Iterator>
    operator+(typename reverse_iterator<_Iterator>::difference_type __n,
	      const reverse_iterator<_Iterator>& __x)
    { return reverse_iterator<_Iterator>(__x.base() - __n); }

#if __cplusplus >= 201103L
  // Same as C++14 make_reverse_iterator but used in C++11 mode too.
  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR reverse_iterator<_Iterator>
    __make_reverse_iterator(_Iterator __i)
    { return reverse_iterator<_Iterator>(__i); }

# if __cplusplus >= 201402L
#  define __cpp_lib_make_reverse_iterator 201402

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // DR 2285. make_reverse_iterator
  /// Generator function for reverse_iterator.
  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR reverse_iterator<_Iterator>
    make_reverse_iterator(_Iterator __i)
    { return reverse_iterator<_Iterator>(__i); }

#  if __cplusplus > 201703L && defined __cpp_lib_concepts
  template<typename _Iterator1, typename _Iterator2>
    requires (!sized_sentinel_for<_Iterator1, _Iterator2>)
    inline constexpr bool
    disable_sized_sentinel_for<reverse_iterator<_Iterator1>,
			       reverse_iterator<_Iterator2>> = true;
#  endif // C++20
# endif // C++14

  template<typename _Iterator>
    _GLIBCXX20_CONSTEXPR
    auto
    __niter_base(reverse_iterator<_Iterator> __it)
    -> decltype(__make_reverse_iterator(__niter_base(__it.base())))
    { return __make_reverse_iterator(__niter_base(__it.base())); }

  template<typename _Iterator>
    struct __is_move_iterator<reverse_iterator<_Iterator> >
      : __is_move_iterator<_Iterator>
    { };

  template<typename _Iterator>
    _GLIBCXX20_CONSTEXPR
    auto
    __miter_base(reverse_iterator<_Iterator> __it)
    -> decltype(__make_reverse_iterator(__miter_base(__it.base())))
    { return __make_reverse_iterator(__miter_base(__it.base())); }
#endif // C++11

  // 24.4.2.2.1 back_insert_iterator
  /**
   *  @brief  Turns assignment into insertion.
   *
   *  These are output iterators, constructed from a container-of-T.
   *  Assigning a T to the iterator appends it to the container using
   *  push_back.
   *
   *  Tip:  Using the back_inserter function to create these iterators can
   *  save typing.
  */
  template<typename _Container>
    class back_insert_iterator
    : public iterator<output_iterator_tag, void, void, void, void>
    {
    protected:
      _Container* container;

    public:
      /// A nested typedef for the type of whatever container you used.
      typedef _Container          container_type;
#if __cplusplus > 201703L
      using difference_type = ptrdiff_t;

      constexpr back_insert_iterator() noexcept : container(nullptr) { }
#endif

      /// The only way to create this %iterator is with a container.
      explicit _GLIBCXX20_CONSTEXPR
      back_insert_iterator(_Container& __x)
      : container(std::__addressof(__x)) { }

      /**
       *  @param  __value  An instance of whatever type
       *                 container_type::const_reference is; presumably a
       *                 reference-to-const T for container<T>.
       *  @return  This %iterator, for chained operations.
       *
       *  This kind of %iterator doesn't really have a @a position in the
       *  container (you can think of the position as being permanently at
       *  the end, if you like).  Assigning a value to the %iterator will
       *  always append the value to the end of the container.
      */
#if __cplusplus < 201103L
      back_insert_iterator&
      operator=(typename _Container::const_reference __value)
      {
	container->push_back(__value);
	return *this;
      }
#else
      _GLIBCXX20_CONSTEXPR
      back_insert_iterator&
      operator=(const typename _Container::value_type& __value)
      {
	container->push_back(__value);
	return *this;
      }

      _GLIBCXX20_CONSTEXPR
      back_insert_iterator&
      operator=(typename _Container::value_type&& __value)
      {
	container->push_back(std::move(__value));
	return *this;
      }
#endif

      /// Simply returns *this.
      _GLIBCXX20_CONSTEXPR
      back_insert_iterator&
      operator*()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      back_insert_iterator&
      operator++()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      back_insert_iterator
      operator++(int)
      { return *this; }
    };

  /**
   *  @param  __x  A container of arbitrary type.
   *  @return  An instance of back_insert_iterator working on @p __x.
   *
   *  This wrapper function helps in creating back_insert_iterator instances.
   *  Typing the name of the %iterator requires knowing the precise full
   *  type of the container, which can be tedious and impedes generic
   *  programming.  Using this function lets you take advantage of automatic
   *  template parameter deduction, making the compiler match the correct
   *  types for you.
  */
  template<typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline back_insert_iterator<_Container>
    back_inserter(_Container& __x)
    { return back_insert_iterator<_Container>(__x); }

  /**
   *  @brief  Turns assignment into insertion.
   *
   *  These are output iterators, constructed from a container-of-T.
   *  Assigning a T to the iterator prepends it to the container using
   *  push_front.
   *
   *  Tip:  Using the front_inserter function to create these iterators can
   *  save typing.
  */
  template<typename _Container>
    class front_insert_iterator
    : public iterator<output_iterator_tag, void, void, void, void>
    {
    protected:
      _Container* container;

    public:
      /// A nested typedef for the type of whatever container you used.
      typedef _Container          container_type;
#if __cplusplus > 201703L
      using difference_type = ptrdiff_t;

      constexpr front_insert_iterator() noexcept : container(nullptr) { }
#endif

      /// The only way to create this %iterator is with a container.
      explicit _GLIBCXX20_CONSTEXPR
      front_insert_iterator(_Container& __x)
      : container(std::__addressof(__x)) { }

      /**
       *  @param  __value  An instance of whatever type
       *                 container_type::const_reference is; presumably a
       *                 reference-to-const T for container<T>.
       *  @return  This %iterator, for chained operations.
       *
       *  This kind of %iterator doesn't really have a @a position in the
       *  container (you can think of the position as being permanently at
       *  the front, if you like).  Assigning a value to the %iterator will
       *  always prepend the value to the front of the container.
      */
#if __cplusplus < 201103L
      front_insert_iterator&
      operator=(typename _Container::const_reference __value)
      {
	container->push_front(__value);
	return *this;
      }
#else
      _GLIBCXX20_CONSTEXPR
      front_insert_iterator&
      operator=(const typename _Container::value_type& __value)
      {
	container->push_front(__value);
	return *this;
      }

      _GLIBCXX20_CONSTEXPR
      front_insert_iterator&
      operator=(typename _Container::value_type&& __value)
      {
	container->push_front(std::move(__value));
	return *this;
      }
#endif

      /// Simply returns *this.
      _GLIBCXX20_CONSTEXPR
      front_insert_iterator&
      operator*()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      front_insert_iterator&
      operator++()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      front_insert_iterator
      operator++(int)
      { return *this; }
    };

  /**
   *  @param  __x  A container of arbitrary type.
   *  @return  An instance of front_insert_iterator working on @p x.
   *
   *  This wrapper function helps in creating front_insert_iterator instances.
   *  Typing the name of the %iterator requires knowing the precise full
   *  type of the container, which can be tedious and impedes generic
   *  programming.  Using this function lets you take advantage of automatic
   *  template parameter deduction, making the compiler match the correct
   *  types for you.
  */
  template<typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline front_insert_iterator<_Container>
    front_inserter(_Container& __x)
    { return front_insert_iterator<_Container>(__x); }

  /**
   *  @brief  Turns assignment into insertion.
   *
   *  These are output iterators, constructed from a container-of-T.
   *  Assigning a T to the iterator inserts it in the container at the
   *  %iterator's position, rather than overwriting the value at that
   *  position.
   *
   *  (Sequences will actually insert a @e copy of the value before the
   *  %iterator's position.)
   *
   *  Tip:  Using the inserter function to create these iterators can
   *  save typing.
  */
  template<typename _Container>
    class insert_iterator
    : public iterator<output_iterator_tag, void, void, void, void>
    {
#if __cplusplus > 201703L && defined __cpp_lib_concepts
      using _Iter = std::__detail::__range_iter_t<_Container>;

    protected:
      _Container* container = nullptr;
      _Iter iter = _Iter();
#else
      typedef typename _Container::iterator		_Iter;

    protected:
      _Container* container;
      _Iter iter;
#endif

    public:
      /// A nested typedef for the type of whatever container you used.
      typedef _Container          container_type;

#if __cplusplus > 201703L && defined __cpp_lib_concepts
      using difference_type = ptrdiff_t;

      insert_iterator() = default;
#endif

      /**
       *  The only way to create this %iterator is with a container and an
       *  initial position (a normal %iterator into the container).
      */
      _GLIBCXX20_CONSTEXPR
      insert_iterator(_Container& __x, _Iter __i)
      : container(std::__addressof(__x)), iter(__i) {}

      /**
       *  @param  __value  An instance of whatever type
       *                 container_type::const_reference is; presumably a
       *                 reference-to-const T for container<T>.
       *  @return  This %iterator, for chained operations.
       *
       *  This kind of %iterator maintains its own position in the
       *  container.  Assigning a value to the %iterator will insert the
       *  value into the container at the place before the %iterator.
       *
       *  The position is maintained such that subsequent assignments will
       *  insert values immediately after one another.  For example,
       *  @code
       *     // vector v contains A and Z
       *
       *     insert_iterator i (v, ++v.begin());
       *     i = 1;
       *     i = 2;
       *     i = 3;
       *
       *     // vector v contains A, 1, 2, 3, and Z
       *  @endcode
      */
#if __cplusplus < 201103L
      insert_iterator&
      operator=(typename _Container::const_reference __value)
      {
	iter = container->insert(iter, __value);
	++iter;
	return *this;
      }
#else
      _GLIBCXX20_CONSTEXPR
      insert_iterator&
      operator=(const typename _Container::value_type& __value)
      {
	iter = container->insert(iter, __value);
	++iter;
	return *this;
      }

      _GLIBCXX20_CONSTEXPR
      insert_iterator&
      operator=(typename _Container::value_type&& __value)
      {
	iter = container->insert(iter, std::move(__value));
	++iter;
	return *this;
      }
#endif

      /// Simply returns *this.
      _GLIBCXX20_CONSTEXPR
      insert_iterator&
      operator*()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      insert_iterator&
      operator++()
      { return *this; }

      /// Simply returns *this.  (This %iterator does not @a move.)
      _GLIBCXX20_CONSTEXPR
      insert_iterator&
      operator++(int)
      { return *this; }
    };

  /**
   *  @param __x  A container of arbitrary type.
   *  @param __i  An iterator into the container.
   *  @return  An instance of insert_iterator working on @p __x.
   *
   *  This wrapper function helps in creating insert_iterator instances.
   *  Typing the name of the %iterator requires knowing the precise full
   *  type of the container, which can be tedious and impedes generic
   *  programming.  Using this function lets you take advantage of automatic
   *  template parameter deduction, making the compiler match the correct
   *  types for you.
  */
#if __cplusplus > 201703L && defined __cpp_lib_concepts
  template<typename _Container>
    constexpr insert_iterator<_Container>
    inserter(_Container& __x, std::__detail::__range_iter_t<_Container> __i)
    { return insert_iterator<_Container>(__x, __i); }
#else
  template<typename _Container, typename _Iterator>
    inline insert_iterator<_Container>
    inserter(_Container& __x, _Iterator __i)
    {
      return insert_iterator<_Container>(__x,
					 typename _Container::iterator(__i));
    }
#endif

  // @} group iterators

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // This iterator adapter is @a normal in the sense that it does not
  // change the semantics of any of the operators of its iterator
  // parameter.  Its primary purpose is to convert an iterator that is
  // not a class, e.g. a pointer, into an iterator that is a class.
  // The _Container parameter exists solely so that different containers
  // using this template can instantiate different types, even if the
  // _Iterator parameter is the same.
  template<typename _Iterator, typename _Container>
    class __normal_iterator
    {
    protected:
      _Iterator _M_current;

      typedef std::iterator_traits<_Iterator>		__traits_type;

    public:
      typedef _Iterator					iterator_type;
      typedef typename __traits_type::iterator_category iterator_category;
      typedef typename __traits_type::value_type  	value_type;
      typedef typename __traits_type::difference_type 	difference_type;
      typedef typename __traits_type::reference 	reference;
      typedef typename __traits_type::pointer   	pointer;

#if __cplusplus > 201703L && __cpp_lib_concepts
      using iterator_concept = std::__detail::__iter_concept<_Iterator>;
#endif

      _GLIBCXX_CONSTEXPR __normal_iterator() _GLIBCXX_NOEXCEPT
      : _M_current(_Iterator()) { }

      explicit _GLIBCXX20_CONSTEXPR
      __normal_iterator(const _Iterator& __i) _GLIBCXX_NOEXCEPT
      : _M_current(__i) { }

      // Allow iterator to const_iterator conversion
      template<typename _Iter>
        _GLIBCXX20_CONSTEXPR
        __normal_iterator(const __normal_iterator<_Iter,
			  typename __enable_if<
      	       (std::__are_same<_Iter, typename _Container::pointer>::__value),
		      _Container>::__type>& __i) _GLIBCXX_NOEXCEPT
        : _M_current(__i.base()) { }

      // Forward iterator requirements
      _GLIBCXX20_CONSTEXPR
      reference
      operator*() const _GLIBCXX_NOEXCEPT
      { return *_M_current; }

      _GLIBCXX20_CONSTEXPR
      pointer
      operator->() const _GLIBCXX_NOEXCEPT
      { return _M_current; }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator&
      operator++() _GLIBCXX_NOEXCEPT
      {
	++_M_current;
	return *this;
      }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator
      operator++(int) _GLIBCXX_NOEXCEPT
      { return __normal_iterator(_M_current++); }

      // Bidirectional iterator requirements
      _GLIBCXX20_CONSTEXPR
      __normal_iterator&
      operator--() _GLIBCXX_NOEXCEPT
      {
	--_M_current;
	return *this;
      }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator
      operator--(int) _GLIBCXX_NOEXCEPT
      { return __normal_iterator(_M_current--); }

      // Random access iterator requirements
      _GLIBCXX20_CONSTEXPR
      reference
      operator[](difference_type __n) const _GLIBCXX_NOEXCEPT
      { return _M_current[__n]; }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator&
      operator+=(difference_type __n) _GLIBCXX_NOEXCEPT
      { _M_current += __n; return *this; }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator
      operator+(difference_type __n) const _GLIBCXX_NOEXCEPT
      { return __normal_iterator(_M_current + __n); }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator&
      operator-=(difference_type __n) _GLIBCXX_NOEXCEPT
      { _M_current -= __n; return *this; }

      _GLIBCXX20_CONSTEXPR
      __normal_iterator
      operator-(difference_type __n) const _GLIBCXX_NOEXCEPT
      { return __normal_iterator(_M_current - __n); }

      _GLIBCXX20_CONSTEXPR
      const _Iterator&
      base() const _GLIBCXX_NOEXCEPT
      { return _M_current; }
    };

  // Note: In what follows, the left- and right-hand-side iterators are
  // allowed to vary in types (conceptually in cv-qualification) so that
  // comparison between cv-qualified and non-cv-qualified iterators be
  // valid.  However, the greedy and unfriendly operators in std::rel_ops
  // will make overload resolution ambiguous (when in scope) if we don't
  // provide overloads whose operands are of the same type.  Can someone
  // remind me what generic programming is about? -- Gaby

#if __cpp_lib_three_way_comparison
  template<typename _IteratorL, typename _IteratorR, typename _Container>
    requires requires (_IteratorL __lhs, _IteratorR __rhs)
    { { __lhs == __rhs } -> std::convertible_to<bool>; }
    constexpr bool
    operator==(const __normal_iterator<_IteratorL, _Container>& __lhs,
	       const __normal_iterator<_IteratorR, _Container>& __rhs)
    noexcept(noexcept(__lhs.base() == __rhs.base()))
    { return __lhs.base() == __rhs.base(); }

  template<typename _IteratorL, typename _IteratorR, typename _Container>
    constexpr std::__detail::__synth3way_t<_IteratorR, _IteratorL>
    operator<=>(const __normal_iterator<_IteratorL, _Container>& __lhs,
		const __normal_iterator<_IteratorR, _Container>& __rhs)
    noexcept(noexcept(std::__detail::__synth3way(__lhs.base(), __rhs.base())))
    { return std::__detail::__synth3way(__lhs.base(), __rhs.base()); }
#else
   // Forward iterator requirements
  template<typename _IteratorL, typename _IteratorR, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator==(const __normal_iterator<_IteratorL, _Container>& __lhs,
	       const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() == __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator==(const __normal_iterator<_Iterator, _Container>& __lhs,
	       const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() == __rhs.base(); }

  template<typename _IteratorL, typename _IteratorR, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator!=(const __normal_iterator<_IteratorL, _Container>& __lhs,
	       const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() != __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator!=(const __normal_iterator<_Iterator, _Container>& __lhs,
	       const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() != __rhs.base(); }

  // Random access iterator requirements
  template<typename _IteratorL, typename _IteratorR, typename _Container>
    inline bool
    operator<(const __normal_iterator<_IteratorL, _Container>& __lhs,
	      const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() < __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator<(const __normal_iterator<_Iterator, _Container>& __lhs,
	      const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() < __rhs.base(); }

  template<typename _IteratorL, typename _IteratorR, typename _Container>
    inline bool
    operator>(const __normal_iterator<_IteratorL, _Container>& __lhs,
	      const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() > __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator>(const __normal_iterator<_Iterator, _Container>& __lhs,
	      const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() > __rhs.base(); }

  template<typename _IteratorL, typename _IteratorR, typename _Container>
    inline bool
    operator<=(const __normal_iterator<_IteratorL, _Container>& __lhs,
	       const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() <= __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator<=(const __normal_iterator<_Iterator, _Container>& __lhs,
	       const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() <= __rhs.base(); }

  template<typename _IteratorL, typename _IteratorR, typename _Container>
    inline bool
    operator>=(const __normal_iterator<_IteratorL, _Container>& __lhs,
	       const __normal_iterator<_IteratorR, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() >= __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline bool
    operator>=(const __normal_iterator<_Iterator, _Container>& __lhs,
	       const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() >= __rhs.base(); }
#endif // three-way comparison

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // According to the resolution of DR179 not only the various comparison
  // operators but also operator- must accept mixed iterator/const_iterator
  // parameters.
  template<typename _IteratorL, typename _IteratorR, typename _Container>
#if __cplusplus >= 201103L
    // DR 685.
    _GLIBCXX20_CONSTEXPR
    inline auto
    operator-(const __normal_iterator<_IteratorL, _Container>& __lhs,
	      const __normal_iterator<_IteratorR, _Container>& __rhs) noexcept
    -> decltype(__lhs.base() - __rhs.base())
#else
    inline typename __normal_iterator<_IteratorL, _Container>::difference_type
    operator-(const __normal_iterator<_IteratorL, _Container>& __lhs,
	      const __normal_iterator<_IteratorR, _Container>& __rhs)
#endif
    { return __lhs.base() - __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline typename __normal_iterator<_Iterator, _Container>::difference_type
    operator-(const __normal_iterator<_Iterator, _Container>& __lhs,
	      const __normal_iterator<_Iterator, _Container>& __rhs)
    _GLIBCXX_NOEXCEPT
    { return __lhs.base() - __rhs.base(); }

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    inline __normal_iterator<_Iterator, _Container>
    operator+(typename __normal_iterator<_Iterator, _Container>::difference_type
	      __n, const __normal_iterator<_Iterator, _Container>& __i)
    _GLIBCXX_NOEXCEPT
    { return __normal_iterator<_Iterator, _Container>(__i.base() + __n); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  template<typename _Iterator, typename _Container>
    _GLIBCXX20_CONSTEXPR
    _Iterator
    __niter_base(__gnu_cxx::__normal_iterator<_Iterator, _Container> __it)
    _GLIBCXX_NOEXCEPT_IF(std::is_nothrow_copy_constructible<_Iterator>::value)
    { return __it.base(); }

#if __cplusplus >= 201103L
  /**
   * @addtogroup iterators
   * @{
   */

#if __cplusplus > 201703L && __cpp_lib_concepts
  template<semiregular _Sent>
    class move_sentinel
    {
    public:
      constexpr
      move_sentinel()
      noexcept(is_nothrow_default_constructible_v<_Sent>)
      : _M_last() { }

      constexpr explicit
      move_sentinel(_Sent __s)
      noexcept(is_nothrow_move_constructible_v<_Sent>)
      : _M_last(std::move(__s)) { }

      template<typename _S2> requires convertible_to<const _S2&, _Sent>
	constexpr
	move_sentinel(const move_sentinel<_S2>& __s)
	noexcept(is_nothrow_constructible_v<_Sent, const _S2&>)
	: _M_last(__s.base())
	{ }

      template<typename _S2> requires assignable_from<_Sent&, const _S2&>
	constexpr move_sentinel&
	operator=(const move_sentinel<_S2>& __s)
	noexcept(is_nothrow_assignable_v<_Sent, const _S2&>)
	{
	  _M_last = __s.base();
	  return *this;
	}

      constexpr _Sent
      base() const
      noexcept(is_nothrow_copy_constructible_v<_Sent>)
      { return _M_last; }

    private:
      _Sent _M_last;
    };
#endif // C++20

  // 24.4.3  Move iterators
  /**
   *  Class template move_iterator is an iterator adapter with the same
   *  behavior as the underlying iterator except that its dereference
   *  operator implicitly converts the value returned by the underlying
   *  iterator's dereference operator to an rvalue reference.  Some
   *  generic algorithms can be called with move iterators to replace
   *  copying with moving.
   */
  template<typename _Iterator>
    class move_iterator
    {
      _Iterator _M_current;

      using __traits_type = iterator_traits<_Iterator>;
#if __cplusplus > 201703L && __cpp_lib_concepts
      using __base_cat = typename __traits_type::iterator_category;
#else
      using __base_ref = typename __traits_type::reference;
#endif

    public:
      using iterator_type = _Iterator;

#if __cplusplus > 201703L && __cpp_lib_concepts
      using iterator_concept = input_iterator_tag;
      using iterator_category
	= __detail::__clamp_iter_cat<__base_cat, random_access_iterator_tag>;
      using value_type = iter_value_t<_Iterator>;
      using difference_type = iter_difference_t<_Iterator>;
      using pointer = _Iterator;
      using reference = iter_rvalue_reference_t<_Iterator>;
#else
      typedef typename __traits_type::iterator_category iterator_category;
      typedef typename __traits_type::value_type  	value_type;
      typedef typename __traits_type::difference_type	difference_type;
      // NB: DR 680.
      typedef _Iterator					pointer;
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2106. move_iterator wrapping iterators returning prvalues
      typedef typename conditional<is_reference<__base_ref>::value,
			 typename remove_reference<__base_ref>::type&&,
			 __base_ref>::type		reference;
#endif

      _GLIBCXX17_CONSTEXPR
      move_iterator()
      : _M_current() { }

      explicit _GLIBCXX17_CONSTEXPR
      move_iterator(iterator_type __i)
      : _M_current(std::move(__i)) { }

      template<typename _Iter>
	_GLIBCXX17_CONSTEXPR
	move_iterator(const move_iterator<_Iter>& __i)
	: _M_current(__i.base()) { }

#if __cplusplus <= 201703L
      _GLIBCXX17_CONSTEXPR iterator_type
      base() const
      { return _M_current; }
#else
      constexpr iterator_type
      base() const &
#if __cpp_lib_concepts
	requires copy_constructible<iterator_type>
#endif
      { return _M_current; }

      constexpr iterator_type
      base() &&
      { return std::move(_M_current); }
#endif

      _GLIBCXX17_CONSTEXPR reference
      operator*() const
#if __cplusplus > 201703L && __cpp_lib_concepts
      { return ranges::iter_move(_M_current); }
#else
      { return static_cast<reference>(*_M_current); }
#endif

      _GLIBCXX17_CONSTEXPR pointer
      operator->() const
      { return _M_current; }

      _GLIBCXX17_CONSTEXPR move_iterator&
      operator++()
      {
	++_M_current;
	return *this;
      }

      _GLIBCXX17_CONSTEXPR move_iterator
      operator++(int)
      {
	move_iterator __tmp = *this;
	++_M_current;
	return __tmp;
      }

#if __cpp_lib_concepts
      constexpr void
      operator++(int) requires (!forward_iterator<_Iterator>)
      { ++_M_current; }
#endif

      _GLIBCXX17_CONSTEXPR move_iterator&
      operator--()
      {
	--_M_current;
	return *this;
      }

      _GLIBCXX17_CONSTEXPR move_iterator
      operator--(int)
      {
	move_iterator __tmp = *this;
	--_M_current;
	return __tmp;
      }

      _GLIBCXX17_CONSTEXPR move_iterator
      operator+(difference_type __n) const
      { return move_iterator(_M_current + __n); }

      _GLIBCXX17_CONSTEXPR move_iterator&
      operator+=(difference_type __n)
      {
	_M_current += __n;
	return *this;
      }

      _GLIBCXX17_CONSTEXPR move_iterator
      operator-(difference_type __n) const
      { return move_iterator(_M_current - __n); }
    
      _GLIBCXX17_CONSTEXPR move_iterator&
      operator-=(difference_type __n)
      { 
	_M_current -= __n;
	return *this;
      }

      _GLIBCXX17_CONSTEXPR reference
      operator[](difference_type __n) const
#if __cplusplus > 201703L && __cpp_lib_concepts
      { return ranges::iter_move(_M_current + __n); }
#else
      { return std::move(_M_current[__n]); }
#endif

#if __cplusplus > 201703L && __cpp_lib_concepts
      template<sentinel_for<_Iterator> _Sent>
	friend constexpr bool
	operator==(const move_iterator& __x, const move_sentinel<_Sent>& __y)
	{ return __x.base() == __y.base(); }

      template<sized_sentinel_for<_Iterator> _Sent>
	friend constexpr iter_difference_t<_Iterator>
	operator-(const move_sentinel<_Sent>& __x, const move_iterator& __y)
	{ return __x.base() - __y.base(); }

      template<sized_sentinel_for<_Iterator> _Sent>
	friend constexpr iter_difference_t<_Iterator>
	operator-(const move_iterator& __x, const move_sentinel<_Sent>& __y)
	{ return __x.base() - __y.base(); }

      friend constexpr iter_rvalue_reference_t<_Iterator>
      iter_move(const move_iterator& __i)
      noexcept(noexcept(ranges::iter_move(__i._M_current)))
      { return ranges::iter_move(__i._M_current); }

      template<indirectly_swappable<_Iterator> _Iter2>
	friend constexpr void
	iter_swap(const move_iterator& __x, const move_iterator<_Iter2>& __y)
	noexcept(noexcept(ranges::iter_swap(__x._M_current, __y._M_current)))
	{ return ranges::iter_swap(__x._M_current, __y._M_current); }
#endif // C++20
    };

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator==(const move_iterator<_IteratorL>& __x,
	       const move_iterator<_IteratorR>& __y)
#if __cplusplus > 201703L && __cpp_lib_concepts
    requires requires { { __x.base() == __y.base() } -> convertible_to<bool>; }
#endif
    { return __x.base() == __y.base(); }

#if __cpp_lib_three_way_comparison
  template<typename _IteratorL,
	   three_way_comparable_with<_IteratorL> _IteratorR>
    constexpr compare_three_way_result_t<_IteratorL, _IteratorR>
    operator<=>(const move_iterator<_IteratorL>& __x,
		const move_iterator<_IteratorR>& __y)
    { return __x.base() <=> __y.base(); }
#else
  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator!=(const move_iterator<_IteratorL>& __x,
	       const move_iterator<_IteratorR>& __y)
    { return !(__x == __y); }
#endif

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<(const move_iterator<_IteratorL>& __x,
	      const move_iterator<_IteratorR>& __y)
#if __cplusplus > 201703L && __cpp_lib_concepts
    requires requires { { __x.base() < __y.base() } -> convertible_to<bool>; }
#endif
    { return __x.base() < __y.base(); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<=(const move_iterator<_IteratorL>& __x,
	       const move_iterator<_IteratorR>& __y)
#if __cplusplus > 201703L && __cpp_lib_concepts
    requires requires { { __y.base() < __x.base() } -> convertible_to<bool>; }
#endif
    { return !(__y < __x); }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>(const move_iterator<_IteratorL>& __x,
	      const move_iterator<_IteratorR>& __y)
#if __cplusplus > 201703L && __cpp_lib_concepts
    requires requires { { __y.base() < __x.base() } -> convertible_to<bool>; }
#endif
    { return __y < __x; }

  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>=(const move_iterator<_IteratorL>& __x,
	       const move_iterator<_IteratorR>& __y)
#if __cplusplus > 201703L && __cpp_lib_concepts
    requires requires { { __x.base() < __y.base() } -> convertible_to<bool>; }
#endif
    { return !(__x < __y); }

#if ! (__cplusplus > 201703L && __cpp_lib_concepts)
  // Note: See __normal_iterator operators note from Gaby to understand
  // why we have these extra overloads for some move_iterator operators.

  // These extra overloads are not needed in C++20, because the ones above
  // are constrained with a requires-clause and so overload resolution will
  // prefer them to greedy unconstrained function templates.

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator==(const move_iterator<_Iterator>& __x,
	       const move_iterator<_Iterator>& __y)
    { return __x.base() == __y.base(); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator!=(const move_iterator<_Iterator>& __x,
	       const move_iterator<_Iterator>& __y)
    { return !(__x == __y); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<(const move_iterator<_Iterator>& __x,
	      const move_iterator<_Iterator>& __y)
    { return __x.base() < __y.base(); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator<=(const move_iterator<_Iterator>& __x,
	       const move_iterator<_Iterator>& __y)
    { return !(__y < __x); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>(const move_iterator<_Iterator>& __x,
	      const move_iterator<_Iterator>& __y)
    { return __y < __x; }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR bool
    operator>=(const move_iterator<_Iterator>& __x,
	       const move_iterator<_Iterator>& __y)
    { return !(__x < __y); }
#endif // ! C++20

  // DR 685.
  template<typename _IteratorL, typename _IteratorR>
    inline _GLIBCXX17_CONSTEXPR auto
    operator-(const move_iterator<_IteratorL>& __x,
	      const move_iterator<_IteratorR>& __y)
    -> decltype(__x.base() - __y.base())
    { return __x.base() - __y.base(); }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR move_iterator<_Iterator>
    operator+(typename move_iterator<_Iterator>::difference_type __n,
	      const move_iterator<_Iterator>& __x)
    { return __x + __n; }

  template<typename _Iterator>
    inline _GLIBCXX17_CONSTEXPR move_iterator<_Iterator>
    make_move_iterator(_Iterator __i)
    { return move_iterator<_Iterator>(std::move(__i)); }

  template<typename _Iterator, typename _ReturnType
    = typename conditional<__move_if_noexcept_cond
      <typename iterator_traits<_Iterator>::value_type>::value,
                _Iterator, move_iterator<_Iterator>>::type>
    inline _GLIBCXX17_CONSTEXPR _ReturnType
    __make_move_if_noexcept_iterator(_Iterator __i)
    { return _ReturnType(__i); }

  // Overload for pointers that matches std::move_if_noexcept more closely,
  // returning a constant iterator when we don't want to move.
  template<typename _Tp, typename _ReturnType
    = typename conditional<__move_if_noexcept_cond<_Tp>::value,
			   const _Tp*, move_iterator<_Tp*>>::type>
    inline _GLIBCXX17_CONSTEXPR _ReturnType
    __make_move_if_noexcept_iterator(_Tp* __i)
    { return _ReturnType(__i); }

#if __cplusplus > 201703L && __cpp_lib_concepts
  // [iterators.common] Common iterators

  namespace __detail
  {
    template<typename _It>
      concept __common_iter_has_arrow = indirectly_readable<const _It>
	&& (requires(const _It& __it) { __it.operator->(); }
	    || is_reference_v<iter_reference_t<_It>>
	    || constructible_from<iter_value_t<_It>, iter_reference_t<_It>>);

  } // namespace __detail

  /// An iterator/sentinel adaptor for representing a non-common range.
  template<input_or_output_iterator _It, sentinel_for<_It> _Sent>
    requires (!same_as<_It, _Sent>) && copyable<_It>
  class common_iterator
  {
    template<typename _Tp, typename _Up>
      static constexpr bool
      _S_noexcept1()
      {
	if constexpr (is_trivially_default_constructible_v<_Tp>)
	  return is_nothrow_assignable_v<_Tp, _Up>;
	else
	  return is_nothrow_constructible_v<_Tp, _Up>;
      }

    template<typename _It2, typename _Sent2>
      static constexpr bool
      _S_noexcept()
      { return _S_noexcept1<_It, _It2>() && _S_noexcept1<_Sent, _Sent2>(); }

    class _Proxy
    {
      iter_value_t<_It> _M_keep;

      _Proxy(iter_reference_t<_It>&& __x)
      : _M_keep(std::move(__x)) { }

      friend class common_iterator;

    public:
      const iter_value_t<_It>*
      operator->() const
      { return std::__addressof(_M_keep); }
    };

  public:
    constexpr
    common_iterator()
    noexcept(is_nothrow_default_constructible_v<_It>)
    : _M_it(), _M_index(0)
    { }

    constexpr
    common_iterator(_It __i)
    noexcept(is_nothrow_move_constructible_v<_It>)
    : _M_it(std::move(__i)), _M_index(0)
    { }

    constexpr
    common_iterator(_Sent __s)
    noexcept(is_nothrow_move_constructible_v<_Sent>)
    : _M_sent(std::move(__s)), _M_index(1)
    { }

    template<typename _It2, typename _Sent2>
      requires convertible_to<const _It2&, _It>
	&& convertible_to<const _Sent2&, _Sent>
      constexpr
      common_iterator(const common_iterator<_It2, _Sent2>& __x)
      noexcept(_S_noexcept<const _It2&, const _Sent2&>())
      : _M_valueless(), _M_index(__x._M_index)
      {
	if (_M_index == 0)
	  {
	    if constexpr (is_trivially_default_constructible_v<_It>)
	      _M_it = std::move(__x._M_it);
	    else
	      ::new((void*)std::__addressof(_M_it)) _It(__x._M_it);
	  }
	else if (_M_index == 1)
	  {
	    if constexpr (is_trivially_default_constructible_v<_Sent>)
	      _M_sent = std::move(__x._M_sent);
	    else
	      ::new((void*)std::__addressof(_M_sent)) _Sent(__x._M_sent);
	  }
      }

    constexpr
    common_iterator(const common_iterator& __x)
    noexcept(_S_noexcept<const _It&, const _Sent&>())
    : _M_valueless(), _M_index(__x._M_index)
    {
      if (_M_index == 0)
	{
	  if constexpr (is_trivially_default_constructible_v<_It>)
	    _M_it = std::move(__x._M_it);
	  else
	    ::new((void*)std::__addressof(_M_it)) _It(__x._M_it);
	}
      else if (_M_index == 1)
	{
	  if constexpr (is_trivially_default_constructible_v<_Sent>)
	    _M_sent = std::move(__x._M_sent);
	  else
	    ::new((void*)std::__addressof(_M_sent)) _Sent(__x._M_sent);
	}
    }

    common_iterator&
    operator=(const common_iterator& __x)
    noexcept(is_nothrow_copy_assignable_v<_It>
	     && is_nothrow_copy_assignable_v<_Sent>
	     && is_nothrow_copy_constructible_v<_It>
	     && is_nothrow_copy_constructible_v<_Sent>)
    {
      return this->operator=<_It, _Sent>(__x);
    }

    template<typename _It2, typename _Sent2>
      requires convertible_to<const _It2&, _It>
	&& convertible_to<const _Sent2&, _Sent>
	&& assignable_from<_It&, const _It2&>
	&& assignable_from<_Sent&, const _Sent2&>
      common_iterator&
      operator=(const common_iterator<_It2, _Sent2>& __x)
      noexcept(is_nothrow_constructible_v<_It, const _It2&>
	       && is_nothrow_constructible_v<_Sent, const _Sent2&>
	       && is_nothrow_assignable_v<_It, const _It2&>
	       && is_nothrow_assignable_v<_Sent, const _Sent2&>)
      {
	switch(_M_index << 2 | __x._M_index)
	  {
	  case 0b0000:
	    _M_it = __x._M_it;
	    break;
	  case 0b0101:
	    _M_sent = __x._M_sent;
	    break;
	  case 0b0001:
	    _M_it.~_It();
	    _M_index = -1;
	    [[fallthrough]];
	  case 0b1001:
	    ::new((void*)std::__addressof(_M_sent)) _Sent(__x._M_sent);
	    _M_index = 1;
	    break;
	  case 0b0100:
	    _M_sent.~_Sent();
	    _M_index = -1;
	    [[fallthrough]];
	  case 0b1000:
	    ::new((void*)std::__addressof(_M_it)) _It(__x._M_it);
	    _M_index = 0;
	    break;
	  default:
	    __glibcxx_assert(__x._M_has_value());
	    __builtin_unreachable();
	  }
	return *this;
      }

    ~common_iterator()
    {
      switch (_M_index)
	{
	case 0:
	  _M_it.~_It();
	  break;
	case 1:
	  _M_sent.~_Sent();
	  break;
	}
    }

    decltype(auto)
    operator*()
    {
      __glibcxx_assert(_M_index == 0);
      return *_M_it;
    }

    decltype(auto)
    operator*() const requires __detail::__dereferenceable<const _It>
    {
      __glibcxx_assert(_M_index == 0);
      return *_M_it;
    }

    decltype(auto)
    operator->() const requires __detail::__common_iter_has_arrow<_It>
    {
      __glibcxx_assert(_M_index == 0);
      if constexpr (is_pointer_v<_It> || requires { _M_it.operator->(); })
	return _M_it;
      else if constexpr (is_reference_v<iter_reference_t<_It>>)
	{
	  auto&& __tmp = *_M_it;
	  return std::__addressof(__tmp);
	}
      else
	return _Proxy{*_M_it};
    }

    common_iterator&
    operator++()
    {
      __glibcxx_assert(_M_index == 0);
      ++_M_it;
      return *this;
    }

    decltype(auto)
    operator++(int)
    {
      __glibcxx_assert(_M_index == 0);
      if constexpr (forward_iterator<_It>)
	{
	  common_iterator __tmp = *this;
	  ++*this;
	  return __tmp;
	}
      else
	return _M_it++;
    }

    template<typename _It2, sentinel_for<_It> _Sent2>
      requires sentinel_for<_Sent, _It2>
      friend bool
      operator==(const common_iterator& __x,
		 const common_iterator<_It2, _Sent2>& __y)
      {
	switch(__x._M_index << 2 | __y._M_index)
	  {
	  case 0b0000:
	  case 0b0101:
	    return true;
	  case 0b0001:
	    return __x._M_it == __y._M_sent;
	  case 0b0100:
	    return __x._M_sent == __y._M_it;
	  default:
	    __glibcxx_assert(__x._M_has_value());
	    __glibcxx_assert(__y._M_has_value());
	    __builtin_unreachable();
	  }
      }

    template<typename _It2, sentinel_for<_It> _Sent2>
      requires sentinel_for<_Sent, _It2> && equality_comparable_with<_It, _It2>
      friend bool
      operator==(const common_iterator& __x,
		 const common_iterator<_It2, _Sent2>& __y)
      {
	switch(__x._M_index << 2 | __y._M_index)
	  {
	  case 0b0101:
	    return true;
	  case 0b0000:
	    return __x._M_it == __y._M_it;
	  case 0b0001:
	    return __x._M_it == __y._M_sent;
	  case 0b0100:
	    return __x._M_sent == __y._M_it;
	  default:
	    __glibcxx_assert(__x._M_has_value());
	    __glibcxx_assert(__y._M_has_value());
	    __builtin_unreachable();
	  }
      }

    template<sized_sentinel_for<_It> _It2, sized_sentinel_for<_It> _Sent2>
      requires sized_sentinel_for<_Sent, _It2>
      friend iter_difference_t<_It2>
      operator-(const common_iterator& __x,
		const common_iterator<_It2, _Sent2>& __y)
      {
	switch(__x._M_index << 2 | __y._M_index)
	  {
	  case 0b0101:
	    return 0;
	  case 0b0000:
	    return __x._M_it - __y._M_it;
	  case 0b0001:
	    return __x._M_it - __y._M_sent;
	  case 0b0100:
	    return __x._M_sent - __y._M_it;
	  default:
	    __glibcxx_assert(__x._M_has_value());
	    __glibcxx_assert(__y._M_has_value());
	    __builtin_unreachable();
	  }
      }

    friend iter_rvalue_reference_t<_It>
    iter_move(const common_iterator& __i)
    noexcept(noexcept(ranges::iter_move(std::declval<const _It&>())))
    requires input_iterator<_It>
    {
      __glibcxx_assert(__i._M_index == 0);
      return ranges::iter_move(__i._M_it);
    }

    template<indirectly_swappable<_It> _It2, typename _Sent2>
      friend void
      iter_swap(const common_iterator& __x,
		const common_iterator<_It2, _Sent2>& __y)
      noexcept(noexcept(ranges::iter_swap(std::declval<const _It&>(),
					  std::declval<const _It2&>())))
      {
	__glibcxx_assert(__x._M_index == 0);
	__glibcxx_assert(__y._M_index == 0);
	return ranges::iter_swap(__x._M_it, __y._M_it);
      }

  private:
    template<input_or_output_iterator _It2, sentinel_for<_It2> _Sent2>
      friend class common_iterator;

    bool _M_has_value() const noexcept { return _M_index < 2; }

    union
    {
      _It _M_it;
      _Sent _M_sent;
      unsigned char _M_valueless;
    };
    unsigned char _M_index; // 0==_M_it, 1==_M_sent, 2==valueless
  };

  template<typename _It, typename _Sent>
    struct incrementable_traits<common_iterator<_It, _Sent>>
    {
      using difference_type = iter_difference_t<_It>;
    };

  template<input_iterator _It, typename _Sent>
    struct iterator_traits<common_iterator<_It, _Sent>>
    {
    private:
      template<typename _Iter>
	struct __ptr
	{
	  using type = void;
	};

      template<typename _Iter>
	requires __detail::__common_iter_has_arrow<_Iter>
	struct __ptr<_Iter>
	{
	  using _CIter = common_iterator<_Iter, _Sent>;
	  using type = decltype(std::declval<const _CIter&>().operator->());
	};

    public:
      using iterator_concept = conditional_t<forward_iterator<_It>,
	    forward_iterator_tag, input_iterator_tag>;
      using iterator_category = __detail::__clamp_iter_cat<
	typename iterator_traits<_It>::iterator_category,
	forward_iterator_tag, input_iterator_tag>;
      using value_type = iter_value_t<_It>;
      using difference_type = iter_difference_t<_It>;
      using pointer = typename __ptr<_It>::type;
      using reference = iter_reference_t<_It>;
    };

  // [iterators.counted] Counted iterators

  /// An iterator adaptor that keeps track of the distance to the end.
  template<input_or_output_iterator _It>
    class counted_iterator
    {
    public:
      using iterator_type = _It;

      constexpr counted_iterator() = default;

      constexpr
      counted_iterator(_It __i, iter_difference_t<_It> __n)
      : _M_current(std::move(__i)), _M_length(__n)
      { __glibcxx_assert(__n >= 0); }

      template<typename _It2>
	requires convertible_to<const _It2&, _It>
	constexpr
	counted_iterator(const counted_iterator<_It2>& __x)
	: _M_current(__x._M_current), _M_length(__x._M_length)
	{ }

      template<typename _It2>
	requires assignable_from<_It&, const _It2&>
	constexpr counted_iterator&
	operator=(const counted_iterator<_It2>& __x)
	{
	  _M_current = __x._M_current;
	  _M_length = __x._M_length;
	  return *this;
	}

      constexpr _It
      base() const &
      noexcept(is_nothrow_copy_constructible_v<_It>)
      requires copy_constructible<_It>
      { return _M_current; }

      constexpr _It
      base() &&
      noexcept(is_nothrow_move_constructible_v<_It>)
      { return std::move(_M_current); }

      constexpr iter_difference_t<_It>
      count() const noexcept { return _M_length; }

      constexpr decltype(auto)
      operator*()
      noexcept(noexcept(*_M_current))
      { return *_M_current; }

      constexpr decltype(auto)
      operator*() const
      noexcept(noexcept(*_M_current))
      requires __detail::__dereferenceable<const _It>
      { return *_M_current; }

      constexpr counted_iterator&
      operator++()
      {
	__glibcxx_assert(_M_length > 0);
	++_M_current;
	--_M_length;
	return *this;
      }

      decltype(auto)
      operator++(int)
      {
	__glibcxx_assert(_M_length > 0);
	--_M_length;
	__try
	  {
	    return _M_current++;
	  } __catch(...) {
	    ++_M_length;
	    __throw_exception_again;
	  }

      }

      constexpr counted_iterator
      operator++(int) requires forward_iterator<_It>
      {
	auto __tmp = *this;
	++*this;
	return __tmp;
      }

      constexpr counted_iterator&
      operator--() requires bidirectional_iterator<_It>
      {
	--_M_current;
	++_M_length;
	return *this;
      }

      constexpr counted_iterator
      operator--(int) requires bidirectional_iterator<_It>
      {
	auto __tmp = *this;
	--*this;
	return __tmp;
      }

      constexpr counted_iterator
      operator+(iter_difference_t<_It> __n) const
	requires random_access_iterator<_It>
      { return counted_iterator(_M_current + __n, _M_length - __n); }

      friend constexpr counted_iterator
      operator+(iter_difference_t<_It> __n, const counted_iterator& __x)
      requires random_access_iterator<_It>
      { return __x + __n; }

      constexpr counted_iterator&
      operator+=(iter_difference_t<_It> __n)
      requires random_access_iterator<_It>
      {
	__glibcxx_assert(__n <= _M_length);
	_M_current += __n;
	_M_length -= __n;
	return *this;
      }

      constexpr counted_iterator
      operator-(iter_difference_t<_It> __n) const
      requires random_access_iterator<_It>
      { return counted_iterator(_M_current - __n, _M_length + __n); }

      template<common_with<_It> _It2>
	friend constexpr iter_difference_t<_It2>
	operator-(const counted_iterator& __x,
		  const counted_iterator<_It2>& __y)
	{ return __y._M_length - __x._M_length; }

      friend constexpr iter_difference_t<_It>
      operator-(const counted_iterator& __x, default_sentinel_t)
      { return -__x._M_length; }

      friend constexpr iter_difference_t<_It>
      operator-(default_sentinel_t, const counted_iterator& __y)
      { return __y._M_length; }

      constexpr counted_iterator&
      operator-=(iter_difference_t<_It> __n)
      requires random_access_iterator<_It>
      {
	__glibcxx_assert(-__n <= _M_length);
	_M_current -= __n;
	_M_length += __n;
	return *this;
      }

      constexpr decltype(auto)
      operator[](iter_difference_t<_It> __n) const
      noexcept(noexcept(_M_current[__n]))
      requires random_access_iterator<_It>
      {
	__glibcxx_assert(__n < _M_length);
	return _M_current[__n];
      }

      template<common_with<_It> _It2>
	friend constexpr bool
	operator==(const counted_iterator& __x,
		   const counted_iterator<_It2>& __y)
	{ return __x._M_length == __y._M_length; }

      friend constexpr bool
      operator==(const counted_iterator& __x, default_sentinel_t)
      { return __x._M_length == 0; }

      template<common_with<_It> _It2>
	friend constexpr strong_ordering
	operator<=>(const counted_iterator& __x,
		    const counted_iterator<_It2>& __y)
	{ return __y._M_length <=> __x._M_length; }

      friend constexpr iter_rvalue_reference_t<_It>
      iter_move(const counted_iterator& __i)
      noexcept(noexcept(ranges::iter_move(__i._M_current)))
      requires input_iterator<_It>
      { return ranges::iter_move(__i._M_current); }

      template<indirectly_swappable<_It> _It2>
	friend constexpr void
	iter_swap(const counted_iterator& __x,
		  const counted_iterator<_It2>& __y)
	noexcept(noexcept(ranges::iter_swap(__x._M_current, __y._M_current)))
	{ ranges::iter_swap(__x._M_current, __y._M_current); }

    private:
      template<input_or_output_iterator _It2> friend class counted_iterator;

      _It _M_current = _It();
      iter_difference_t<_It> _M_length = 0;
    };

  template<typename _It>
    struct incrementable_traits<counted_iterator<_It>>
    {
      using difference_type = iter_difference_t<_It>;
    };

  template<input_iterator _It>
    struct iterator_traits<counted_iterator<_It>> : iterator_traits<_It>
    {
      using pointer = void;
    };
#endif // C++20

  // @} group iterators

  template<typename _Iterator>
    auto
    __niter_base(move_iterator<_Iterator> __it)
    -> decltype(make_move_iterator(__niter_base(__it.base())))
    { return make_move_iterator(__niter_base(__it.base())); }

  template<typename _Iterator>
    struct __is_move_iterator<move_iterator<_Iterator> >
    {
      enum { __value = 1 };
      typedef __true_type __type;
    };

  template<typename _Iterator>
    auto
    __miter_base(move_iterator<_Iterator> __it)
    -> decltype(__miter_base(__it.base()))
    { return __miter_base(__it.base()); }

#define _GLIBCXX_MAKE_MOVE_ITERATOR(_Iter) std::make_move_iterator(_Iter)
#define _GLIBCXX_MAKE_MOVE_IF_NOEXCEPT_ITERATOR(_Iter) \
  std::__make_move_if_noexcept_iterator(_Iter)
#else
#define _GLIBCXX_MAKE_MOVE_ITERATOR(_Iter) (_Iter)
#define _GLIBCXX_MAKE_MOVE_IF_NOEXCEPT_ITERATOR(_Iter) (_Iter)
#endif // C++11

#if __cpp_deduction_guides >= 201606
  // These helper traits are used for deduction guides
  // of associative containers.
  template<typename _InputIterator>
    using __iter_key_t = remove_const_t<
    typename iterator_traits<_InputIterator>::value_type::first_type>;

  template<typename _InputIterator>
    using __iter_val_t =
    typename iterator_traits<_InputIterator>::value_type::second_type;

  template<typename _T1, typename _T2>
    struct pair;

  template<typename _InputIterator>
    using __iter_to_alloc_t =
    pair<add_const_t<__iter_key_t<_InputIterator>>,
	 __iter_val_t<_InputIterator>>;
#endif // __cpp_deduction_guides

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#ifdef _GLIBCXX_DEBUG
# include <debug/stl_iterator.h>
#endif

#endif
