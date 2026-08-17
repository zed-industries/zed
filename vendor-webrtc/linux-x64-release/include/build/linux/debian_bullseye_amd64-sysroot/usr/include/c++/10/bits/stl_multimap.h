// Multimap implementation -*- C++ -*-

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
 * Copyright (c) 1996,1997
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

/** @file bits/stl_multimap.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{map}
 */

#ifndef _STL_MULTIMAP_H
#define _STL_MULTIMAP_H 1

#include <bits/concept_check.h>
#if __cplusplus >= 201103L
#include <initializer_list>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CONTAINER

  template <typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    class map;

  /**
   *  @brief A standard container made up of (key,value) pairs, which can be
   *  retrieved based on a key, in logarithmic time.
   *
   *  @ingroup associative_containers
   *
   *  @tparam _Key  Type of key objects.
   *  @tparam  _Tp  Type of mapped objects.
   *  @tparam _Compare  Comparison function object type, defaults to less<_Key>.
   *  @tparam _Alloc  Allocator type, defaults to
   *                  allocator<pair<const _Key, _Tp>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, a
   *  <a href="tables.html#66">reversible container</a>, and an
   *  <a href="tables.html#69">associative container</a> (using equivalent
   *  keys).  For a @c multimap<Key,T> the key_type is Key, the mapped_type
   *  is T, and the value_type is std::pair<const Key,T>.
   *
   *  Multimaps support bidirectional iterators.
   *
   *  The private tree data is declared exactly the same way for map and
   *  multimap; the distinction is made entirely in how the tree functions are
   *  called (*_unique versus *_equal, same as the standard).
  */
  template <typename _Key, typename _Tp,
	    typename _Compare = std::less<_Key>,
	    typename _Alloc = std::allocator<std::pair<const _Key, _Tp> > >
    class multimap
    {
    public:
      typedef _Key					key_type;
      typedef _Tp					mapped_type;
      typedef std::pair<const _Key, _Tp>		value_type;
      typedef _Compare					key_compare;
      typedef _Alloc					allocator_type;

    private:
#ifdef _GLIBCXX_CONCEPT_CHECKS
      // concept requirements
      typedef typename _Alloc::value_type		_Alloc_value_type;
# if __cplusplus < 201103L
      __glibcxx_class_requires(_Tp, _SGIAssignableConcept)
# endif
      __glibcxx_class_requires4(_Compare, bool, _Key, _Key,
				_BinaryFunctionConcept)
      __glibcxx_class_requires2(value_type, _Alloc_value_type, _SameTypeConcept)
#endif

#if __cplusplus >= 201103L
#if __cplusplus > 201703L || defined __STRICT_ANSI__
      static_assert(is_same<typename _Alloc::value_type, value_type>::value,
	  "std::multimap must have the same value_type as its allocator");
#endif
#endif

    public:
      class value_compare
      : public std::binary_function<value_type, value_type, bool>
      {
	friend class multimap<_Key, _Tp, _Compare, _Alloc>;
      protected:
	_Compare comp;

	value_compare(_Compare __c)
	: comp(__c) { }

      public:
	bool operator()(const value_type& __x, const value_type& __y) const
	{ return comp(__x.first, __y.first); }
      };

    private:
      /// This turns a red-black tree into a [multi]map.
      typedef typename __gnu_cxx::__alloc_traits<_Alloc>::template
	rebind<value_type>::other _Pair_alloc_type;

      typedef _Rb_tree<key_type, value_type, _Select1st<value_type>,
		       key_compare, _Pair_alloc_type> _Rep_type;
      /// The actual tree structure.
      _Rep_type _M_t;

      typedef __gnu_cxx::__alloc_traits<_Pair_alloc_type> _Alloc_traits;

    public:
      // many of these are specified differently in ISO, but the following are
      // "functionally equivalent"
      typedef typename _Alloc_traits::pointer		 pointer;
      typedef typename _Alloc_traits::const_pointer	 const_pointer;
      typedef typename _Alloc_traits::reference		 reference;
      typedef typename _Alloc_traits::const_reference	 const_reference;
      typedef typename _Rep_type::iterator		 iterator;
      typedef typename _Rep_type::const_iterator	 const_iterator;
      typedef typename _Rep_type::size_type		 size_type;
      typedef typename _Rep_type::difference_type	 difference_type;
      typedef typename _Rep_type::reverse_iterator	 reverse_iterator;
      typedef typename _Rep_type::const_reverse_iterator const_reverse_iterator;

#if __cplusplus > 201402L
      using node_type = typename _Rep_type::node_type;
#endif

      // [23.3.2] construct/copy/destroy
      // (get_allocator() is also listed in this section)

      /**
       *  @brief  Default constructor creates no elements.
       */
#if __cplusplus < 201103L
      multimap() : _M_t() { }
#else
      multimap() = default;
#endif

      /**
       *  @brief  Creates a %multimap with no elements.
       *  @param  __comp  A comparison object.
       *  @param  __a  An allocator object.
       */
      explicit
      multimap(const _Compare& __comp,
	       const allocator_type& __a = allocator_type())
      : _M_t(__comp, _Pair_alloc_type(__a)) { }

      /**
       *  @brief  %Multimap copy constructor.
       *
       *  Whether the allocator is copied depends on the allocator traits.
       */
#if __cplusplus < 201103L
      multimap(const multimap& __x)
      : _M_t(__x._M_t) { }
#else
      multimap(const multimap&) = default;

      /**
       *  @brief  %Multimap move constructor.
       *
       *  The newly-created %multimap contains the exact contents of the
       *  moved instance. The moved instance is a valid, but unspecified
       *  %multimap.
       */
      multimap(multimap&&) = default;

      /**
       *  @brief  Builds a %multimap from an initializer_list.
       *  @param  __l  An initializer_list.
       *  @param  __comp  A comparison functor.
       *  @param  __a  An allocator object.
       *
       *  Create a %multimap consisting of copies of the elements from
       *  the initializer_list.  This is linear in N if the list is already
       *  sorted, and NlogN otherwise (where N is @a __l.size()).
       */
      multimap(initializer_list<value_type> __l,
	       const _Compare& __comp = _Compare(),
	       const allocator_type& __a = allocator_type())
      : _M_t(__comp, _Pair_alloc_type(__a))
      { _M_t._M_insert_range_equal(__l.begin(), __l.end()); }

      /// Allocator-extended default constructor.
      explicit
      multimap(const allocator_type& __a)
      : _M_t(_Pair_alloc_type(__a)) { }

      /// Allocator-extended copy constructor.
      multimap(const multimap& __m, const allocator_type& __a)
      : _M_t(__m._M_t, _Pair_alloc_type(__a)) { }

      /// Allocator-extended move constructor.
      multimap(multimap&& __m, const allocator_type& __a)
      noexcept(is_nothrow_copy_constructible<_Compare>::value
	       && _Alloc_traits::_S_always_equal())
      : _M_t(std::move(__m._M_t), _Pair_alloc_type(__a)) { }

      /// Allocator-extended initialier-list constructor.
      multimap(initializer_list<value_type> __l, const allocator_type& __a)
      : _M_t(_Pair_alloc_type(__a))
      { _M_t._M_insert_range_equal(__l.begin(), __l.end()); }

      /// Allocator-extended range constructor.
      template<typename _InputIterator>
	multimap(_InputIterator __first, _InputIterator __last,
		 const allocator_type& __a)
	: _M_t(_Pair_alloc_type(__a))
	{ _M_t._M_insert_range_equal(__first, __last); }
#endif

      /**
       *  @brief  Builds a %multimap from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *
       *  Create a %multimap consisting of copies of the elements from
       *  [__first,__last).  This is linear in N if the range is already sorted,
       *  and NlogN otherwise (where N is distance(__first,__last)).
       */
      template<typename _InputIterator>
	multimap(_InputIterator __first, _InputIterator __last)
	: _M_t()
	{ _M_t._M_insert_range_equal(__first, __last); }

      /**
       *  @brief  Builds a %multimap from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @param  __comp  A comparison functor.
       *  @param  __a  An allocator object.
       *
       *  Create a %multimap consisting of copies of the elements from
       *  [__first,__last).  This is linear in N if the range is already sorted,
       *  and NlogN otherwise (where N is distance(__first,__last)).
       */
      template<typename _InputIterator>
	multimap(_InputIterator __first, _InputIterator __last,
		 const _Compare& __comp,
		 const allocator_type& __a = allocator_type())
	: _M_t(__comp, _Pair_alloc_type(__a))
	{ _M_t._M_insert_range_equal(__first, __last); }

#if __cplusplus >= 201103L
      /**
       *  The dtor only erases the elements, and note that if the elements
       *  themselves are pointers, the pointed-to memory is not touched in any
       *  way. Managing the pointer is the user's responsibility.
       */
      ~multimap() = default;
#endif

      /**
       *  @brief  %Multimap assignment operator.
       *
       *  Whether the allocator is copied depends on the allocator traits.
       */
#if __cplusplus < 201103L
      multimap&
      operator=(const multimap& __x)
      {
	_M_t = __x._M_t;
	return *this;
      }
#else
      multimap&
      operator=(const multimap&) = default;

      /// Move assignment operator.
      multimap&
      operator=(multimap&&) = default;

      /**
       *  @brief  %Multimap list assignment operator.
       *  @param  __l  An initializer_list.
       *
       *  This function fills a %multimap with copies of the elements
       *  in the initializer list @a __l.
       *
       *  Note that the assignment completely changes the %multimap and
       *  that the resulting %multimap's size is the same as the number
       *  of elements assigned.
       */
      multimap&
      operator=(initializer_list<value_type> __l)
      {
	_M_t._M_assign_equal(__l.begin(), __l.end());
	return *this;
      }
#endif

      /// Get a copy of the memory allocation object.
      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return allocator_type(_M_t.get_allocator()); }

      // iterators
      /**
       *  Returns a read/write iterator that points to the first pair in the
       *  %multimap.  Iteration is done in ascending order according to the
       *  keys.
       */
      iterator
      begin() _GLIBCXX_NOEXCEPT
      { return _M_t.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points to the first pair
       *  in the %multimap.  Iteration is done in ascending order according to
       *  the keys.
       */
      const_iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return _M_t.begin(); }

      /**
       *  Returns a read/write iterator that points one past the last pair in
       *  the %multimap.  Iteration is done in ascending order according to the
       *  keys.
       */
      iterator
      end() _GLIBCXX_NOEXCEPT
      { return _M_t.end(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  pair in the %multimap.  Iteration is done in ascending order according
       *  to the keys.
       */
      const_iterator
      end() const _GLIBCXX_NOEXCEPT
      { return _M_t.end(); }

      /**
       *  Returns a read/write reverse iterator that points to the last pair in
       *  the %multimap.  Iteration is done in descending order according to the
       *  keys.
       */
      reverse_iterator
      rbegin() _GLIBCXX_NOEXCEPT
      { return _M_t.rbegin(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to the
       *  last pair in the %multimap.  Iteration is done in descending order
       *  according to the keys.
       */
      const_reverse_iterator
      rbegin() const _GLIBCXX_NOEXCEPT
      { return _M_t.rbegin(); }

      /**
       *  Returns a read/write reverse iterator that points to one before the
       *  first pair in the %multimap.  Iteration is done in descending order
       *  according to the keys.
       */
      reverse_iterator
      rend() _GLIBCXX_NOEXCEPT
      { return _M_t.rend(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to one
       *  before the first pair in the %multimap.  Iteration is done in
       *  descending order according to the keys.
       */
      const_reverse_iterator
      rend() const _GLIBCXX_NOEXCEPT
      { return _M_t.rend(); }

#if __cplusplus >= 201103L
      /**
       *  Returns a read-only (constant) iterator that points to the first pair
       *  in the %multimap.  Iteration is done in ascending order according to
       *  the keys.
       */
      const_iterator
      cbegin() const noexcept
      { return _M_t.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  pair in the %multimap.  Iteration is done in ascending order according
       *  to the keys.
       */
      const_iterator
      cend() const noexcept
      { return _M_t.end(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to the
       *  last pair in the %multimap.  Iteration is done in descending order
       *  according to the keys.
       */
      const_reverse_iterator
      crbegin() const noexcept
      { return _M_t.rbegin(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to one
       *  before the first pair in the %multimap.  Iteration is done in
       *  descending order according to the keys.
       */
      const_reverse_iterator
      crend() const noexcept
      { return _M_t.rend(); }
#endif

      // capacity
      /** Returns true if the %multimap is empty.  */
      _GLIBCXX_NODISCARD bool
      empty() const _GLIBCXX_NOEXCEPT
      { return _M_t.empty(); }

      /** Returns the size of the %multimap.  */
      size_type
      size() const _GLIBCXX_NOEXCEPT
      { return _M_t.size(); }

      /** Returns the maximum size of the %multimap.  */
      size_type
      max_size() const _GLIBCXX_NOEXCEPT
      { return _M_t.max_size(); }

      // modifiers
#if __cplusplus >= 201103L
      /**
       *  @brief Build and insert a std::pair into the %multimap.
       *
       *  @param __args  Arguments used to generate a new pair instance (see
       *	        std::piecewise_contruct for passing arguments to each
       *	        part of the pair constructor).
       *
       *  @return An iterator that points to the inserted (key,value) pair.
       *
       *  This function builds and inserts a (key, value) %pair into the
       *  %multimap.
       *  Contrary to a std::map the %multimap does not rely on unique keys and
       *  thus multiple pairs with the same key can be inserted.
       *
       *  Insertion requires logarithmic time.
       */
      template<typename... _Args>
	iterator
	emplace(_Args&&... __args)
	{ return _M_t._M_emplace_equal(std::forward<_Args>(__args)...); }

      /**
       *  @brief Builds and inserts a std::pair into the %multimap.
       *
       *  @param  __pos  An iterator that serves as a hint as to where the pair
       *                should be inserted.
       *  @param  __args  Arguments used to generate a new pair instance (see
       *	         std::piecewise_contruct for passing arguments to each
       *	         part of the pair constructor).
       *  @return An iterator that points to the inserted (key,value) pair.
       *
       *  This function inserts a (key, value) pair into the %multimap.
       *  Contrary to a std::map the %multimap does not rely on unique keys and
       *  thus multiple pairs with the same key can be inserted.
       *  Note that the first parameter is only a hint and can potentially
       *  improve the performance of the insertion process.  A bad hint would
       *  cause no gains in efficiency.
       *
       *  For more on @a hinting, see:
       *  https://gcc.gnu.org/onlinedocs/libstdc++/manual/associative.html#containers.associative.insert_hints
       *
       *  Insertion requires logarithmic time (if the hint is not taken).
       */
      template<typename... _Args>
	iterator
	emplace_hint(const_iterator __pos, _Args&&... __args)
	{
	  return _M_t._M_emplace_hint_equal(__pos,
					    std::forward<_Args>(__args)...);
	}
#endif

      /**
       *  @brief Inserts a std::pair into the %multimap.
       *  @param  __x  Pair to be inserted (see std::make_pair for easy creation
       *             of pairs).
       *  @return An iterator that points to the inserted (key,value) pair.
       *
       *  This function inserts a (key, value) pair into the %multimap.
       *  Contrary to a std::map the %multimap does not rely on unique keys and
       *  thus multiple pairs with the same key can be inserted.
       *
       *  Insertion requires logarithmic time.
       *  @{
       */
      iterator
      insert(const value_type& __x)
      { return _M_t._M_insert_equal(__x); }

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2354. Unnecessary copying when inserting into maps with braced-init
      iterator
      insert(value_type&& __x)
      { return _M_t._M_insert_equal(std::move(__x)); }

      template<typename _Pair>
	__enable_if_t<is_constructible<value_type, _Pair>::value, iterator>
	insert(_Pair&& __x)
	{ return _M_t._M_emplace_equal(std::forward<_Pair>(__x)); }
#endif
      // @}

      /**
       *  @brief Inserts a std::pair into the %multimap.
       *  @param  __position  An iterator that serves as a hint as to where the
       *                      pair should be inserted.
       *  @param  __x  Pair to be inserted (see std::make_pair for easy creation
       *               of pairs).
       *  @return An iterator that points to the inserted (key,value) pair.
       *
       *  This function inserts a (key, value) pair into the %multimap.
       *  Contrary to a std::map the %multimap does not rely on unique keys and
       *  thus multiple pairs with the same key can be inserted.
       *  Note that the first parameter is only a hint and can potentially
       *  improve the performance of the insertion process.  A bad hint would
       *  cause no gains in efficiency.
       *
       *  For more on @a hinting, see:
       *  https://gcc.gnu.org/onlinedocs/libstdc++/manual/associative.html#containers.associative.insert_hints
       *
       *  Insertion requires logarithmic time (if the hint is not taken).
       * @{
       */
      iterator
#if __cplusplus >= 201103L
      insert(const_iterator __position, const value_type& __x)
#else
      insert(iterator __position, const value_type& __x)
#endif
      { return _M_t._M_insert_equal_(__position, __x); }

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2354. Unnecessary copying when inserting into maps with braced-init
      iterator
      insert(const_iterator __position, value_type&& __x)
      { return _M_t._M_insert_equal_(__position, std::move(__x)); }

      template<typename _Pair>
	__enable_if_t<is_constructible<value_type, _Pair&&>::value, iterator>
	insert(const_iterator __position, _Pair&& __x)
	{
	  return _M_t._M_emplace_hint_equal(__position,
					    std::forward<_Pair>(__x));
	}
#endif
      // @}

      /**
       *  @brief A template function that attempts to insert a range
       *  of elements.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                   inserted.
       *  @param  __last  Iterator pointing to the end of the range.
       *
       *  Complexity similar to that of the range constructor.
       */
      template<typename _InputIterator>
	void
	insert(_InputIterator __first, _InputIterator __last)
	{ _M_t._M_insert_range_equal(__first, __last); }

#if __cplusplus >= 201103L
      /**
       *  @brief Attempts to insert a list of std::pairs into the %multimap.
       *  @param  __l  A std::initializer_list<value_type> of pairs to be
       *               inserted.
       *
       *  Complexity similar to that of the range constructor.
       */
      void
      insert(initializer_list<value_type> __l)
      { this->insert(__l.begin(), __l.end()); }
#endif

#if __cplusplus > 201402L
      /// Extract a node.
      node_type
      extract(const_iterator __pos)
      {
	__glibcxx_assert(__pos != end());
	return _M_t.extract(__pos);
      }

      /// Extract a node.
      node_type
      extract(const key_type& __x)
      { return _M_t.extract(__x); }

      /// Re-insert an extracted node.
      iterator
      insert(node_type&& __nh)
      { return _M_t._M_reinsert_node_equal(std::move(__nh)); }

      /// Re-insert an extracted node.
      iterator
      insert(const_iterator __hint, node_type&& __nh)
      { return _M_t._M_reinsert_node_hint_equal(__hint, std::move(__nh)); }

      template<typename, typename>
	friend class std::_Rb_tree_merge_helper;

      template<typename _Cmp2>
	void
	merge(multimap<_Key, _Tp, _Cmp2, _Alloc>& __source)
	{
	  using _Merge_helper = _Rb_tree_merge_helper<multimap, _Cmp2>;
	  _M_t._M_merge_equal(_Merge_helper::_S_get_tree(__source));
	}

      template<typename _Cmp2>
	void
	merge(multimap<_Key, _Tp, _Cmp2, _Alloc>&& __source)
	{ merge(__source); }

      template<typename _Cmp2>
	void
	merge(map<_Key, _Tp, _Cmp2, _Alloc>& __source)
	{
	  using _Merge_helper = _Rb_tree_merge_helper<multimap, _Cmp2>;
	  _M_t._M_merge_equal(_Merge_helper::_S_get_tree(__source));
	}

      template<typename _Cmp2>
	void
	merge(map<_Key, _Tp, _Cmp2, _Alloc>&& __source)
	{ merge(__source); }
#endif // C++17

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      /**
       *  @brief Erases an element from a %multimap.
       *  @param  __position  An iterator pointing to the element to be erased.
       *  @return An iterator pointing to the element immediately following
       *          @a position prior to the element being erased. If no such
       *          element exists, end() is returned.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from a %multimap.  Note that this function only erases the element,
       *  and that if the element is itself a pointer, the pointed-to memory is
       *  not touched in any way.  Managing the pointer is the user's
       *  responsibility.
       *
       * @{
       */
      iterator
      erase(const_iterator __position)
      { return _M_t.erase(__position); }

      // LWG 2059.
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(iterator __position)
      { return _M_t.erase(__position); }
      // @}
#else
      /**
       *  @brief Erases an element from a %multimap.
       *  @param  __position  An iterator pointing to the element to be erased.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from a %multimap.  Note that this function only erases the element,
       *  and that if the element is itself a pointer, the pointed-to memory is
       *  not touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      void
      erase(iterator __position)
      { _M_t.erase(__position); }
#endif

      /**
       *  @brief Erases elements according to the provided key.
       *  @param  __x  Key of element to be erased.
       *  @return  The number of elements erased.
       *
       *  This function erases all elements located by the given key from a
       *  %multimap.
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      size_type
      erase(const key_type& __x)
      { return _M_t.erase(__x); }

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      /**
       *  @brief Erases a [first,last) range of elements from a %multimap.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                   erased.
       *  @param __last Iterator pointing to the end of the range to be
       *                erased .
       *  @return The iterator @a __last.
       *
       *  This function erases a sequence of elements from a %multimap.
       *  Note that this function only erases the elements, and that if
       *  the elements themselves are pointers, the pointed-to memory is not
       *  touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      iterator
      erase(const_iterator __first, const_iterator __last)
      { return _M_t.erase(__first, __last); }
#else
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      /**
       *  @brief Erases a [first,last) range of elements from a %multimap.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                 erased.
       *  @param __last Iterator pointing to the end of the range to
       *                be erased.
       *
       *  This function erases a sequence of elements from a %multimap.
       *  Note that this function only erases the elements, and that if
       *  the elements themselves are pointers, the pointed-to memory is not
       *  touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      void
      erase(iterator __first, iterator __last)
      { _M_t.erase(__first, __last); }
#endif

      /**
       *  @brief  Swaps data with another %multimap.
       *  @param  __x  A %multimap of the same element and allocator types.
       *
       *  This exchanges the elements between two multimaps in constant time.
       *  (It is only swapping a pointer, an integer, and an instance of
       *  the @c Compare type (which itself is often stateless and empty), so it
       *  should be quite fast.)
       *  Note that the global std::swap() function is specialized such that
       *  std::swap(m1,m2) will feed to this function.
       *
       *  Whether the allocators are swapped depends on the allocator traits.
       */
      void
      swap(multimap& __x)
      _GLIBCXX_NOEXCEPT_IF(__is_nothrow_swappable<_Compare>::value)
      { _M_t.swap(__x._M_t); }

      /**
       *  Erases all elements in a %multimap.  Note that this function only
       *  erases the elements, and that if the elements themselves are pointers,
       *  the pointed-to memory is not touched in any way.  Managing the pointer
       *  is the user's responsibility.
       */
      void
      clear() _GLIBCXX_NOEXCEPT
      { _M_t.clear(); }

      // observers
      /**
       *  Returns the key comparison object out of which the %multimap
       *  was constructed.
       */
      key_compare
      key_comp() const
      { return _M_t.key_comp(); }

      /**
       *  Returns a value comparison object, built from the key comparison
       *  object out of which the %multimap was constructed.
       */
      value_compare
      value_comp() const
      { return value_compare(_M_t.key_comp()); }

      // multimap operations

      //@{
      /**
       *  @brief Tries to locate an element in a %multimap.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return  Iterator pointing to sought-after element,
       *           or end() if not found.
       *
       *  This function takes a key and tries to locate the element with which
       *  the key matches.  If successful the function returns an iterator
       *  pointing to the sought after %pair.  If unsuccessful it returns the
       *  past-the-end ( @c end() ) iterator.
       */
      iterator
      find(const key_type& __x)
      { return _M_t.find(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	find(const _Kt& __x) -> decltype(_M_t._M_find_tr(__x))
	{ return _M_t._M_find_tr(__x); }
#endif
      //@}

      //@{
      /**
       *  @brief Tries to locate an element in a %multimap.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return  Read-only (constant) iterator pointing to sought-after
       *           element, or end() if not found.
       *
       *  This function takes a key and tries to locate the element with which
       *  the key matches.  If successful the function returns a constant
       *  iterator pointing to the sought after %pair.  If unsuccessful it
       *  returns the past-the-end ( @c end() ) iterator.
       */
      const_iterator
      find(const key_type& __x) const
      { return _M_t.find(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	find(const _Kt& __x) const -> decltype(_M_t._M_find_tr(__x))
	{ return _M_t._M_find_tr(__x); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds the number of elements with given key.
       *  @param  __x  Key of (key, value) pairs to be located.
       *  @return Number of elements with specified key.
       */
      size_type
      count(const key_type& __x) const
      { return _M_t.count(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	count(const _Kt& __x) const -> decltype(_M_t._M_count_tr(__x))
	{ return _M_t._M_count_tr(__x); }
#endif
      //@}

#if __cplusplus > 201703L
      //@{
      /**
       *  @brief  Finds whether an element with the given key exists.
       *  @param  __x  Key of (key, value) pairs to be located.
       *  @return  True if there is any element with the specified key.
       */
      bool
      contains(const key_type& __x) const
      { return _M_t.find(__x) != _M_t.end(); }

      template<typename _Kt>
	auto
	contains(const _Kt& __x) const
	-> decltype(_M_t._M_find_tr(__x), void(), true)
	{ return _M_t._M_find_tr(__x) != _M_t.end(); }
      //@}
#endif

      //@{
      /**
       *  @brief Finds the beginning of a subsequence matching given key.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return  Iterator pointing to first element equal to or greater
       *           than key, or end().
       *
       *  This function returns the first element of a subsequence of elements
       *  that matches the given key.  If unsuccessful it returns an iterator
       *  pointing to the first element that has a greater value than given key
       *  or end() if no such element exists.
       */
      iterator
      lower_bound(const key_type& __x)
      { return _M_t.lower_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	lower_bound(const _Kt& __x)
	-> decltype(iterator(_M_t._M_lower_bound_tr(__x)))
	{ return iterator(_M_t._M_lower_bound_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds the beginning of a subsequence matching given key.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return  Read-only (constant) iterator pointing to first element
       *           equal to or greater than key, or end().
       *
       *  This function returns the first element of a subsequence of
       *  elements that matches the given key.  If unsuccessful the
       *  iterator will point to the next greatest element or, if no
       *  such greater element exists, to end().
       */
      const_iterator
      lower_bound(const key_type& __x) const
      { return _M_t.lower_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	lower_bound(const _Kt& __x) const
	-> decltype(const_iterator(_M_t._M_lower_bound_tr(__x)))
	{ return const_iterator(_M_t._M_lower_bound_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds the end of a subsequence matching given key.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return Iterator pointing to the first element
       *          greater than key, or end().
       */
      iterator
      upper_bound(const key_type& __x)
      { return _M_t.upper_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	upper_bound(const _Kt& __x)
	-> decltype(iterator(_M_t._M_upper_bound_tr(__x)))
	{ return iterator(_M_t._M_upper_bound_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds the end of a subsequence matching given key.
       *  @param  __x  Key of (key, value) pair to be located.
       *  @return  Read-only (constant) iterator pointing to first iterator
       *           greater than key, or end().
       */
      const_iterator
      upper_bound(const key_type& __x) const
      { return _M_t.upper_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	upper_bound(const _Kt& __x) const
	-> decltype(const_iterator(_M_t._M_upper_bound_tr(__x)))
	{ return const_iterator(_M_t._M_upper_bound_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds a subsequence matching given key.
       *  @param  __x  Key of (key, value) pairs to be located.
       *  @return  Pair of iterators that possibly points to the subsequence
       *           matching given key.
       *
       *  This function is equivalent to
       *  @code
       *    std::make_pair(c.lower_bound(val),
       *                   c.upper_bound(val))
       *  @endcode
       *  (but is faster than making the calls separately).
       */
      std::pair<iterator, iterator>
      equal_range(const key_type& __x)
      { return _M_t.equal_range(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	equal_range(const _Kt& __x)
	-> decltype(pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)))
	{ return pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds a subsequence matching given key.
       *  @param  __x  Key of (key, value) pairs to be located.
       *  @return  Pair of read-only (constant) iterators that possibly points
       *           to the subsequence matching given key.
       *
       *  This function is equivalent to
       *  @code
       *    std::make_pair(c.lower_bound(val),
       *                   c.upper_bound(val))
       *  @endcode
       *  (but is faster than making the calls separately).
       */
      std::pair<const_iterator, const_iterator>
      equal_range(const key_type& __x) const
      { return _M_t.equal_range(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	equal_range(const _Kt& __x) const
	-> decltype(pair<const_iterator, const_iterator>(
	      _M_t._M_equal_range_tr(__x)))
	{
	  return pair<const_iterator, const_iterator>(
	      _M_t._M_equal_range_tr(__x));
	}
#endif
      //@}

      template<typename _K1, typename _T1, typename _C1, typename _A1>
	friend bool
	operator==(const multimap<_K1, _T1, _C1, _A1>&,
		   const multimap<_K1, _T1, _C1, _A1>&);

#if __cpp_lib_three_way_comparison
      template<typename _K1, typename _T1, typename _C1, typename _A1>
	friend __detail::__synth3way_t<pair<const _K1, _T1>>
	operator<=>(const multimap<_K1, _T1, _C1, _A1>&,
		    const multimap<_K1, _T1, _C1, _A1>&);
#else
      template<typename _K1, typename _T1, typename _C1, typename _A1>
	friend bool
	operator<(const multimap<_K1, _T1, _C1, _A1>&,
		  const multimap<_K1, _T1, _C1, _A1>&);
#endif
  };

#if __cpp_deduction_guides >= 201606

  template<typename _InputIterator,
	   typename _Compare = less<__iter_key_t<_InputIterator>>,
	   typename _Allocator = allocator<__iter_to_alloc_t<_InputIterator>>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    multimap(_InputIterator, _InputIterator,
	     _Compare = _Compare(), _Allocator = _Allocator())
    -> multimap<__iter_key_t<_InputIterator>, __iter_val_t<_InputIterator>,
		_Compare, _Allocator>;

  template<typename _Key, typename _Tp, typename _Compare = less<_Key>,
	   typename _Allocator = allocator<pair<const _Key, _Tp>>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    multimap(initializer_list<pair<_Key, _Tp>>,
	     _Compare = _Compare(), _Allocator = _Allocator())
    -> multimap<_Key, _Tp, _Compare, _Allocator>;

  template<typename _InputIterator, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    multimap(_InputIterator, _InputIterator, _Allocator)
    -> multimap<__iter_key_t<_InputIterator>, __iter_val_t<_InputIterator>,
		less<__iter_key_t<_InputIterator>>, _Allocator>;

  template<typename _Key, typename _Tp, typename _Allocator,
	   typename = _RequireAllocator<_Allocator>>
    multimap(initializer_list<pair<_Key, _Tp>>, _Allocator)
    -> multimap<_Key, _Tp, less<_Key>, _Allocator>;

#endif // deduction guides

  /**
   *  @brief  Multimap equality comparison.
   *  @param  __x  A %multimap.
   *  @param  __y  A %multimap of the same type as @a __x.
   *  @return  True iff the size and elements of the maps are equal.
   *
   *  This is an equivalence relation.  It is linear in the size of the
   *  multimaps.  Multimaps are considered equivalent if their sizes are equal,
   *  and if corresponding elements compare equal.
  */
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator==(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	       const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return __x._M_t == __y._M_t; }

#if __cpp_lib_three_way_comparison
  /**
   *  @brief  Multimap ordering relation.
   *  @param  __x  A `multimap`.
   *  @param  __y  A `multimap` of the same type as `x`.
   *  @return  A value indicating whether `__x` is less than, equal to,
   *           greater than, or incomparable with `__y`.
   *
   *  This is a total ordering relation.  It is linear in the size of the
   *  maps.  The elements must be comparable with @c <.
   *
   *  See `std::lexicographical_compare_three_way()` for how the determination
   *  is made. This operator is used to synthesize relational operators like
   *  `<` and `>=` etc.
  */
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline __detail::__synth3way_t<pair<const _Key, _Tp>>
    operator<=>(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
		const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return __x._M_t <=> __y._M_t; }
#else
  /**
   *  @brief  Multimap ordering relation.
   *  @param  __x  A %multimap.
   *  @param  __y  A %multimap of the same type as @a __x.
   *  @return  True iff @a x is lexicographically less than @a y.
   *
   *  This is a total ordering relation.  It is linear in the size of the
   *  multimaps.  The elements must be comparable with @c <.
   *
   *  See std::lexicographical_compare() for how the determination is made.
  */
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator<(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	      const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return __x._M_t < __y._M_t; }

  /// Based on operator==
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator!=(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	       const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return !(__x == __y); }

  /// Based on operator<
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator>(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	      const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return __y < __x; }

  /// Based on operator<
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator<=(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	       const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return !(__y < __x); }

  /// Based on operator<
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline bool
    operator>=(const multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	       const multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    { return !(__x < __y); }
#endif // three-way comparison

  /// See std::multimap::swap().
  template<typename _Key, typename _Tp, typename _Compare, typename _Alloc>
    inline void
    swap(multimap<_Key, _Tp, _Compare, _Alloc>& __x,
	 multimap<_Key, _Tp, _Compare, _Alloc>& __y)
    _GLIBCXX_NOEXCEPT_IF(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

_GLIBCXX_END_NAMESPACE_CONTAINER

#if __cplusplus > 201402L
  // Allow std::multimap access to internals of compatible maps.
  template<typename _Key, typename _Val, typename _Cmp1, typename _Alloc,
	   typename _Cmp2>
    struct
    _Rb_tree_merge_helper<_GLIBCXX_STD_C::multimap<_Key, _Val, _Cmp1, _Alloc>,
			  _Cmp2>
    {
    private:
      friend class _GLIBCXX_STD_C::multimap<_Key, _Val, _Cmp1, _Alloc>;

      static auto&
      _S_get_tree(_GLIBCXX_STD_C::map<_Key, _Val, _Cmp2, _Alloc>& __map)
      { return __map._M_t; }

      static auto&
      _S_get_tree(_GLIBCXX_STD_C::multimap<_Key, _Val, _Cmp2, _Alloc>& __map)
      { return __map._M_t; }
    };
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif /* _STL_MULTIMAP_H */
