// Set implementation -*- C++ -*-

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

/** @file bits/stl_set.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{set}
 */

#ifndef _STL_SET_H
#define _STL_SET_H 1

#include <bits/concept_check.h>
#if __cplusplus >= 201103L
#include <initializer_list>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CONTAINER

  template<typename _Key, typename _Compare, typename _Alloc>
    class multiset;

  /**
   *  @brief A standard container made up of unique keys, which can be
   *  retrieved in logarithmic time.
   *
   *  @ingroup associative_containers
   *
   *  @tparam _Key  Type of key objects.
   *  @tparam _Compare  Comparison function object type, defaults to less<_Key>.
   *  @tparam _Alloc  Allocator type, defaults to allocator<_Key>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, a
   *  <a href="tables.html#66">reversible container</a>, and an
   *  <a href="tables.html#69">associative container</a> (using unique keys).
   *
   *  Sets support bidirectional iterators.
   *
   *  The private tree data is declared exactly the same way for set and
   *  multiset; the distinction is made entirely in how the tree functions are
   *  called (*_unique versus *_equal, same as the standard).
  */
  template<typename _Key, typename _Compare = std::less<_Key>,
	   typename _Alloc = std::allocator<_Key> >
    class set
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      // concept requirements
      typedef typename _Alloc::value_type		_Alloc_value_type;
# if __cplusplus < 201103L
      __glibcxx_class_requires(_Key, _SGIAssignableConcept)
# endif
      __glibcxx_class_requires4(_Compare, bool, _Key, _Key,
				_BinaryFunctionConcept)
      __glibcxx_class_requires2(_Key, _Alloc_value_type, _SameTypeConcept)
#endif

#if __cplusplus >= 201103L
      static_assert(is_same<typename remove_cv<_Key>::type, _Key>::value,
	  "std::set must have a non-const, non-volatile value_type");
# if __cplusplus > 201703L || defined __STRICT_ANSI__
      static_assert(is_same<typename _Alloc::value_type, _Key>::value,
	  "std::set must have the same value_type as its allocator");
# endif
#endif

    public:
      // typedefs:
      //@{
      /// Public typedefs.
      typedef _Key     key_type;
      typedef _Key     value_type;
      typedef _Compare key_compare;
      typedef _Compare value_compare;
      typedef _Alloc   allocator_type;
      //@}

    private:
      typedef typename __gnu_cxx::__alloc_traits<_Alloc>::template
	rebind<_Key>::other _Key_alloc_type;

      typedef _Rb_tree<key_type, value_type, _Identity<value_type>,
		       key_compare, _Key_alloc_type> _Rep_type;
      _Rep_type _M_t;  // Red-black tree representing set.

      typedef __gnu_cxx::__alloc_traits<_Key_alloc_type> _Alloc_traits;

    public:
      //@{
      ///  Iterator-related typedefs.
      typedef typename _Alloc_traits::pointer		 pointer;
      typedef typename _Alloc_traits::const_pointer	 const_pointer;
      typedef typename _Alloc_traits::reference		 reference;
      typedef typename _Alloc_traits::const_reference	 const_reference;
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 103. set::iterator is required to be modifiable,
      // but this allows modification of keys.
      typedef typename _Rep_type::const_iterator	 iterator;
      typedef typename _Rep_type::const_iterator	 const_iterator;
      typedef typename _Rep_type::const_reverse_iterator reverse_iterator;
      typedef typename _Rep_type::const_reverse_iterator const_reverse_iterator;
      typedef typename _Rep_type::size_type		 size_type;
      typedef typename _Rep_type::difference_type	 difference_type;
      //@}

#if __cplusplus > 201402L
      using node_type = typename _Rep_type::node_type;
      using insert_return_type = typename _Rep_type::insert_return_type;
#endif

      // allocation/deallocation
      /**
       *  @brief  Default constructor creates no elements.
       */
#if __cplusplus < 201103L
      set() : _M_t() { }
#else
      set() = default;
#endif

      /**
       *  @brief  Creates a %set with no elements.
       *  @param  __comp  Comparator to use.
       *  @param  __a  An allocator object.
       */
      explicit
      set(const _Compare& __comp,
	  const allocator_type& __a = allocator_type())
      : _M_t(__comp, _Key_alloc_type(__a)) { }

      /**
       *  @brief  Builds a %set from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *
       *  Create a %set consisting of copies of the elements from
       *  [__first,__last).  This is linear in N if the range is
       *  already sorted, and NlogN otherwise (where N is
       *  distance(__first,__last)).
       */
      template<typename _InputIterator>
	set(_InputIterator __first, _InputIterator __last)
	: _M_t()
	{ _M_t._M_insert_range_unique(__first, __last); }

      /**
       *  @brief  Builds a %set from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @param  __comp  A comparison functor.
       *  @param  __a  An allocator object.
       *
       *  Create a %set consisting of copies of the elements from
       *  [__first,__last).  This is linear in N if the range is
       *  already sorted, and NlogN otherwise (where N is
       *  distance(__first,__last)).
       */
      template<typename _InputIterator>
	set(_InputIterator __first, _InputIterator __last,
	    const _Compare& __comp,
	    const allocator_type& __a = allocator_type())
	: _M_t(__comp, _Key_alloc_type(__a))
	{ _M_t._M_insert_range_unique(__first, __last); }

      /**
       *  @brief  %Set copy constructor.
       *
       *  Whether the allocator is copied depends on the allocator traits.
       */
#if __cplusplus < 201103L
      set(const set& __x)
      : _M_t(__x._M_t) { }
#else
      set(const set&) = default;

     /**
       *  @brief %Set move constructor
       *
       *  The newly-created %set contains the exact contents of the moved
       *  instance. The moved instance is a valid, but unspecified, %set.
       */
      set(set&&) = default;

      /**
       *  @brief  Builds a %set from an initializer_list.
       *  @param  __l  An initializer_list.
       *  @param  __comp  A comparison functor.
       *  @param  __a  An allocator object.
       *
       *  Create a %set consisting of copies of the elements in the list.
       *  This is linear in N if the list is already sorted, and NlogN
       *  otherwise (where N is @a __l.size()).
       */
      set(initializer_list<value_type> __l,
	  const _Compare& __comp = _Compare(),
	  const allocator_type& __a = allocator_type())
      : _M_t(__comp, _Key_alloc_type(__a))
      { _M_t._M_insert_range_unique(__l.begin(), __l.end()); }

      /// Allocator-extended default constructor.
      explicit
      set(const allocator_type& __a)
      : _M_t(_Key_alloc_type(__a)) { }

      /// Allocator-extended copy constructor.
      set(const set& __x, const allocator_type& __a)
      : _M_t(__x._M_t, _Key_alloc_type(__a)) { }

      /// Allocator-extended move constructor.
      set(set&& __x, const allocator_type& __a)
      noexcept(is_nothrow_copy_constructible<_Compare>::value
	       && _Alloc_traits::_S_always_equal())
      : _M_t(std::move(__x._M_t), _Key_alloc_type(__a)) { }

      /// Allocator-extended initialier-list constructor.
      set(initializer_list<value_type> __l, const allocator_type& __a)
      : _M_t(_Key_alloc_type(__a))
      { _M_t._M_insert_range_unique(__l.begin(), __l.end()); }

      /// Allocator-extended range constructor.
      template<typename _InputIterator>
	set(_InputIterator __first, _InputIterator __last,
	    const allocator_type& __a)
	: _M_t(_Key_alloc_type(__a))
	{ _M_t._M_insert_range_unique(__first, __last); }

      /**
       *  The dtor only erases the elements, and note that if the elements
       *  themselves are pointers, the pointed-to memory is not touched in any
       *  way. Managing the pointer is the user's responsibility.
       */
      ~set() = default;
#endif

      /**
       *  @brief  %Set assignment operator.
       *
       *  Whether the allocator is copied depends on the allocator traits.
       */
#if __cplusplus < 201103L
      set&
      operator=(const set& __x)
      {
	_M_t = __x._M_t;
	return *this;
      }
#else
      set&
      operator=(const set&) = default;

      /// Move assignment operator.
      set&
      operator=(set&&) = default;

      /**
       *  @brief  %Set list assignment operator.
       *  @param  __l  An initializer_list.
       *
       *  This function fills a %set with copies of the elements in the
       *  initializer list @a __l.
       *
       *  Note that the assignment completely changes the %set and
       *  that the resulting %set's size is the same as the number
       *  of elements assigned.
       */
      set&
      operator=(initializer_list<value_type> __l)
      {
	_M_t._M_assign_unique(__l.begin(), __l.end());
	return *this;
      }
#endif

      // accessors:

      ///  Returns the comparison object with which the %set was constructed.
      key_compare
      key_comp() const
      { return _M_t.key_comp(); }
      ///  Returns the comparison object with which the %set was constructed.
      value_compare
      value_comp() const
      { return _M_t.key_comp(); }
      ///  Returns the allocator object with which the %set was constructed.
      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return allocator_type(_M_t.get_allocator()); }

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %set.  Iteration is done in ascending order according
       *  to the keys.
       */
      iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return _M_t.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %set.  Iteration is done in ascending order according
       *  to the keys.
       */
      iterator
      end() const _GLIBCXX_NOEXCEPT
      { return _M_t.end(); }

      /**
       *  Returns a read-only (constant) iterator that points to the last
       *  element in the %set.  Iteration is done in descending order according
       *  to the keys.
       */
      reverse_iterator
      rbegin() const _GLIBCXX_NOEXCEPT
      { return _M_t.rbegin(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to the
       *  last pair in the %set.  Iteration is done in descending order
       *  according to the keys.
       */
      reverse_iterator
      rend() const _GLIBCXX_NOEXCEPT
      { return _M_t.rend(); }

#if __cplusplus >= 201103L
      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %set.  Iteration is done in ascending order according
       *  to the keys.
       */
      iterator
      cbegin() const noexcept
      { return _M_t.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %set.  Iteration is done in ascending order according
       *  to the keys.
       */
      iterator
      cend() const noexcept
      { return _M_t.end(); }

      /**
       *  Returns a read-only (constant) iterator that points to the last
       *  element in the %set.  Iteration is done in descending order according
       *  to the keys.
       */
      reverse_iterator
      crbegin() const noexcept
      { return _M_t.rbegin(); }

      /**
       *  Returns a read-only (constant) reverse iterator that points to the
       *  last pair in the %set.  Iteration is done in descending order
       *  according to the keys.
       */
      reverse_iterator
      crend() const noexcept
      { return _M_t.rend(); }
#endif

      ///  Returns true if the %set is empty.
      _GLIBCXX_NODISCARD bool
      empty() const _GLIBCXX_NOEXCEPT
      { return _M_t.empty(); }

      ///  Returns the size of the %set.
      size_type
      size() const _GLIBCXX_NOEXCEPT
      { return _M_t.size(); }

      ///  Returns the maximum size of the %set.
      size_type
      max_size() const _GLIBCXX_NOEXCEPT
      { return _M_t.max_size(); }

      /**
       *  @brief  Swaps data with another %set.
       *  @param  __x  A %set of the same element and allocator types.
       *
       *  This exchanges the elements between two sets in constant
       *  time.  (It is only swapping a pointer, an integer, and an
       *  instance of the @c Compare type (which itself is often
       *  stateless and empty), so it should be quite fast.)  Note
       *  that the global std::swap() function is specialized such
       *  that std::swap(s1,s2) will feed to this function.
       *
       *  Whether the allocators are swapped depends on the allocator traits.
       */
      void
      swap(set& __x)
      _GLIBCXX_NOEXCEPT_IF(__is_nothrow_swappable<_Compare>::value)
      { _M_t.swap(__x._M_t); }

      // insert/erase
#if __cplusplus >= 201103L
      /**
       *  @brief Attempts to build and insert an element into the %set.
       *  @param __args  Arguments used to generate an element.
       *  @return  A pair, of which the first element is an iterator that points
       *           to the possibly inserted element, and the second is a bool
       *           that is true if the element was actually inserted.
       *
       *  This function attempts to build and insert an element into the %set.
       *  A %set relies on unique keys and thus an element is only inserted if
       *  it is not already present in the %set.
       *
       *  Insertion requires logarithmic time.
       */
      template<typename... _Args>
	std::pair<iterator, bool>
	emplace(_Args&&... __args)
	{ return _M_t._M_emplace_unique(std::forward<_Args>(__args)...); }

      /**
       *  @brief Attempts to insert an element into the %set.
       *  @param  __pos  An iterator that serves as a hint as to where the
       *                element should be inserted.
       *  @param  __args  Arguments used to generate the element to be
       *                 inserted.
       *  @return An iterator that points to the element with key equivalent to
       *          the one generated from @a __args (may or may not be the
       *          element itself).
       *
       *  This function is not concerned about whether the insertion took place,
       *  and thus does not return a boolean like the single-argument emplace()
       *  does.  Note that the first parameter is only a hint and can
       *  potentially improve the performance of the insertion process.  A bad
       *  hint would cause no gains in efficiency.
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
	  return _M_t._M_emplace_hint_unique(__pos,
					     std::forward<_Args>(__args)...);
	}
#endif

      /**
       *  @brief Attempts to insert an element into the %set.
       *  @param  __x  Element to be inserted.
       *  @return  A pair, of which the first element is an iterator that points
       *           to the possibly inserted element, and the second is a bool
       *           that is true if the element was actually inserted.
       *
       *  This function attempts to insert an element into the %set.  A %set
       *  relies on unique keys and thus an element is only inserted if it is
       *  not already present in the %set.
       *
       *  Insertion requires logarithmic time.
       */
      std::pair<iterator, bool>
      insert(const value_type& __x)
      {
	std::pair<typename _Rep_type::iterator, bool> __p =
	  _M_t._M_insert_unique(__x);
	return std::pair<iterator, bool>(__p.first, __p.second);
      }

#if __cplusplus >= 201103L
      std::pair<iterator, bool>
      insert(value_type&& __x)
      {
	std::pair<typename _Rep_type::iterator, bool> __p =
	  _M_t._M_insert_unique(std::move(__x));
	return std::pair<iterator, bool>(__p.first, __p.second);
      }
#endif

      /**
       *  @brief Attempts to insert an element into the %set.
       *  @param  __position  An iterator that serves as a hint as to where the
       *                    element should be inserted.
       *  @param  __x  Element to be inserted.
       *  @return An iterator that points to the element with key of
       *           @a __x (may or may not be the element passed in).
       *
       *  This function is not concerned about whether the insertion took place,
       *  and thus does not return a boolean like the single-argument insert()
       *  does.  Note that the first parameter is only a hint and can
       *  potentially improve the performance of the insertion process.  A bad
       *  hint would cause no gains in efficiency.
       *
       *  For more on @a hinting, see:
       *  https://gcc.gnu.org/onlinedocs/libstdc++/manual/associative.html#containers.associative.insert_hints
       *
       *  Insertion requires logarithmic time (if the hint is not taken).
       */
      iterator
      insert(const_iterator __position, const value_type& __x)
      { return _M_t._M_insert_unique_(__position, __x); }

#if __cplusplus >= 201103L
      iterator
      insert(const_iterator __position, value_type&& __x)
      { return _M_t._M_insert_unique_(__position, std::move(__x)); }
#endif

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
	{ _M_t._M_insert_range_unique(__first, __last); }

#if __cplusplus >= 201103L
      /**
       *  @brief Attempts to insert a list of elements into the %set.
       *  @param  __l  A std::initializer_list<value_type> of elements
       *               to be inserted.
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
      insert_return_type
      insert(node_type&& __nh)
      { return _M_t._M_reinsert_node_unique(std::move(__nh)); }

      /// Re-insert an extracted node.
      iterator
      insert(const_iterator __hint, node_type&& __nh)
      { return _M_t._M_reinsert_node_hint_unique(__hint, std::move(__nh)); }

      template<typename, typename>
	friend class std::_Rb_tree_merge_helper;

      template<typename _Compare1>
	void
	merge(set<_Key, _Compare1, _Alloc>& __source)
	{
	  using _Merge_helper = _Rb_tree_merge_helper<set, _Compare1>;
	  _M_t._M_merge_unique(_Merge_helper::_S_get_tree(__source));
	}

      template<typename _Compare1>
	void
	merge(set<_Key, _Compare1, _Alloc>&& __source)
	{ merge(__source); }

      template<typename _Compare1>
	void
	merge(multiset<_Key, _Compare1, _Alloc>& __source)
	{
	  using _Merge_helper = _Rb_tree_merge_helper<set, _Compare1>;
	  _M_t._M_merge_unique(_Merge_helper::_S_get_tree(__source));
	}

      template<typename _Compare1>
	void
	merge(multiset<_Key, _Compare1, _Alloc>&& __source)
	{ merge(__source); }
#endif // C++17

#if __cplusplus >= 201103L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 130. Associative erase should return an iterator.
      /**
       *  @brief Erases an element from a %set.
       *  @param  __position  An iterator pointing to the element to be erased.
       *  @return An iterator pointing to the element immediately following
       *          @a __position prior to the element being erased. If no such
       *          element exists, end() is returned.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from a %set.  Note that this function only erases the element, and
       *  that if the element is itself a pointer, the pointed-to memory is not
       *  touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __position)
      { return _M_t.erase(__position); }
#else
      /**
       *  @brief Erases an element from a %set.
       *  @param  position  An iterator pointing to the element to be erased.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from a %set.  Note that this function only erases the element, and
       *  that if the element is itself a pointer, the pointed-to memory is not
       *  touched in any way.  Managing the pointer is the user's
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
       *  This function erases all the elements located by the given key from
       *  a %set.
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
       *  @brief Erases a [__first,__last) range of elements from a %set.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                 erased.

       *  @param __last Iterator pointing to the end of the range to
       *  be erased.
       *  @return The iterator @a __last.
       *
       *  This function erases a sequence of elements from a %set.
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      _GLIBCXX_ABI_TAG_CXX11
      iterator
      erase(const_iterator __first, const_iterator __last)
      { return _M_t.erase(__first, __last); }
#else
      /**
       *  @brief Erases a [first,last) range of elements from a %set.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                 erased.
       *  @param __last Iterator pointing to the end of the range to
       *  be erased.
       *
       *  This function erases a sequence of elements from a %set.
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      void
      erase(iterator __first, iterator __last)
      { _M_t.erase(__first, __last); }
#endif

      /**
       *  Erases all elements in a %set.  Note that this function only erases
       *  the elements, and that if the elements themselves are pointers, the
       *  pointed-to memory is not touched in any way.  Managing the pointer is
       *  the user's responsibility.
       */
      void
      clear() _GLIBCXX_NOEXCEPT
      { _M_t.clear(); }

      // set operations:

      //@{
      /**
       *  @brief  Finds the number of elements.
       *  @param  __x  Element to located.
       *  @return  Number of elements with specified key.
       *
       *  This function only makes sense for multisets; for set the result will
       *  either be 0 (not present) or 1 (present).
       */
      size_type
      count(const key_type& __x) const
      { return _M_t.find(__x) == _M_t.end() ? 0 : 1; }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	count(const _Kt& __x) const
	-> decltype(_M_t._M_count_tr(__x))
	{ return _M_t._M_count_tr(__x); }
#endif
      //@}

#if __cplusplus > 201703L
      //@{
      /**
       *  @brief  Finds whether an element with the given key exists.
       *  @param  __x  Key of elements to be located.
       *  @return  True if there is an element with the specified key.
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

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 214.  set::find() missing const overload
      //@{
      /**
       *  @brief Tries to locate an element in a %set.
       *  @param  __x  Element to be located.
       *  @return  Iterator pointing to sought-after element, or end() if not
       *           found.
       *
       *  This function takes a key and tries to locate the element with which
       *  the key matches.  If successful the function returns an iterator
       *  pointing to the sought after element.  If unsuccessful it returns the
       *  past-the-end ( @c end() ) iterator.
       */
      iterator
      find(const key_type& __x)
      { return _M_t.find(__x); }

      const_iterator
      find(const key_type& __x) const
      { return _M_t.find(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	find(const _Kt& __x)
	-> decltype(iterator{_M_t._M_find_tr(__x)})
	{ return iterator{_M_t._M_find_tr(__x)}; }

      template<typename _Kt>
	auto
	find(const _Kt& __x) const
	-> decltype(const_iterator{_M_t._M_find_tr(__x)})
	{ return const_iterator{_M_t._M_find_tr(__x)}; }
#endif
      //@}

      //@{
      /**
       *  @brief Finds the beginning of a subsequence matching given key.
       *  @param  __x  Key to be located.
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

      const_iterator
      lower_bound(const key_type& __x) const
      { return _M_t.lower_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	lower_bound(const _Kt& __x)
	-> decltype(iterator(_M_t._M_lower_bound_tr(__x)))
	{ return iterator(_M_t._M_lower_bound_tr(__x)); }

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
       *  @param  __x  Key to be located.
       *  @return Iterator pointing to the first element
       *          greater than key, or end().
       */
      iterator
      upper_bound(const key_type& __x)
      { return _M_t.upper_bound(__x); }

      const_iterator
      upper_bound(const key_type& __x) const
      { return _M_t.upper_bound(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	upper_bound(const _Kt& __x)
	-> decltype(iterator(_M_t._M_upper_bound_tr(__x)))
	{ return iterator(_M_t._M_upper_bound_tr(__x)); }

      template<typename _Kt>
	auto
	upper_bound(const _Kt& __x) const
	-> decltype(iterator(_M_t._M_upper_bound_tr(__x)))
	{ return const_iterator(_M_t._M_upper_bound_tr(__x)); }
#endif
      //@}

      //@{
      /**
       *  @brief Finds a subsequence matching given key.
       *  @param  __x  Key to be located.
       *  @return  Pair of iterators that possibly points to the subsequence
       *           matching given key.
       *
       *  This function is equivalent to
       *  @code
       *    std::make_pair(c.lower_bound(val),
       *                   c.upper_bound(val))
       *  @endcode
       *  (but is faster than making the calls separately).
       *
       *  This function probably only makes sense for multisets.
       */
      std::pair<iterator, iterator>
      equal_range(const key_type& __x)
      { return _M_t.equal_range(__x); }

      std::pair<const_iterator, const_iterator>
      equal_range(const key_type& __x) const
      { return _M_t.equal_range(__x); }

#if __cplusplus > 201103L
      template<typename _Kt>
	auto
	equal_range(const _Kt& __x)
	-> decltype(pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)))
	{ return pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)); }

      template<typename _Kt>
	auto
	equal_range(const _Kt& __x) const
	-> decltype(pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)))
	{ return pair<iterator, iterator>(_M_t._M_equal_range_tr(__x)); }
#endif
      //@}

      template<typename _K1, typename _C1, typename _A1>
	friend bool
	operator==(const set<_K1, _C1, _A1>&, const set<_K1, _C1, _A1>&);

#if __cpp_lib_three_way_comparison
      template<typename _K1, typename _C1, typename _A1>
	friend __detail::__synth3way_t<_K1>
	operator<=>(const set<_K1, _C1, _A1>&, const set<_K1, _C1, _A1>&);
#else
      template<typename _K1, typename _C1, typename _A1>
	friend bool
	operator<(const set<_K1, _C1, _A1>&, const set<_K1, _C1, _A1>&);
#endif
    };

#if __cpp_deduction_guides >= 201606

  template<typename _InputIterator,
	   typename _Compare =
	     less<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Allocator =
	     allocator<typename iterator_traits<_InputIterator>::value_type>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    set(_InputIterator, _InputIterator,
	_Compare = _Compare(), _Allocator = _Allocator())
    -> set<typename iterator_traits<_InputIterator>::value_type,
	  _Compare, _Allocator>;

  template<typename _Key, typename _Compare = less<_Key>,
	   typename _Allocator = allocator<_Key>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireAllocator<_Allocator>>
    set(initializer_list<_Key>,
	_Compare = _Compare(), _Allocator = _Allocator())
    -> set<_Key, _Compare, _Allocator>;

  template<typename _InputIterator, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    set(_InputIterator, _InputIterator, _Allocator)
    -> set<typename iterator_traits<_InputIterator>::value_type,
	   less<typename iterator_traits<_InputIterator>::value_type>,
	   _Allocator>;

  template<typename _Key, typename _Allocator,
	   typename = _RequireAllocator<_Allocator>>
    set(initializer_list<_Key>, _Allocator)
    -> set<_Key, less<_Key>, _Allocator>;

#endif // deduction guides

  /**
   *  @brief  Set equality comparison.
   *  @param  __x  A %set.
   *  @param  __y  A %set of the same type as @a x.
   *  @return  True iff the size and elements of the sets are equal.
   *
   *  This is an equivalence relation.  It is linear in the size of the sets.
   *  Sets are considered equivalent if their sizes are equal, and if
   *  corresponding elements compare equal.
  */
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator==(const set<_Key, _Compare, _Alloc>& __x,
	       const set<_Key, _Compare, _Alloc>& __y)
    { return __x._M_t == __y._M_t; }

#if __cpp_lib_three_way_comparison
  /**
   *  @brief  Set ordering relation.
   *  @param  __x  A `set`.
   *  @param  __y  A `set` of the same type as `x`.
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
  template<typename _Key, typename _Compare, typename _Alloc>
    inline __detail::__synth3way_t<_Key>
    operator<=>(const set<_Key, _Compare, _Alloc>& __x,
		const set<_Key, _Compare, _Alloc>& __y)
    { return __x._M_t <=> __y._M_t; }
#else
  /**
   *  @brief  Set ordering relation.
   *  @param  __x  A %set.
   *  @param  __y  A %set of the same type as @a x.
   *  @return  True iff @a __x is lexicographically less than @a __y.
   *
   *  This is a total ordering relation.  It is linear in the size of the
   *  sets.  The elements must be comparable with @c <.
   *
   *  See std::lexicographical_compare() for how the determination is made.
  */
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator<(const set<_Key, _Compare, _Alloc>& __x,
	      const set<_Key, _Compare, _Alloc>& __y)
    { return __x._M_t < __y._M_t; }

  ///  Returns !(x == y).
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator!=(const set<_Key, _Compare, _Alloc>& __x,
	       const set<_Key, _Compare, _Alloc>& __y)
    { return !(__x == __y); }

  ///  Returns y < x.
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator>(const set<_Key, _Compare, _Alloc>& __x,
	      const set<_Key, _Compare, _Alloc>& __y)
    { return __y < __x; }

  ///  Returns !(y < x)
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator<=(const set<_Key, _Compare, _Alloc>& __x,
	       const set<_Key, _Compare, _Alloc>& __y)
    { return !(__y < __x); }

  ///  Returns !(x < y)
  template<typename _Key, typename _Compare, typename _Alloc>
    inline bool
    operator>=(const set<_Key, _Compare, _Alloc>& __x,
	       const set<_Key, _Compare, _Alloc>& __y)
    { return !(__x < __y); }
#endif // three-way comparison

  /// See std::set::swap().
  template<typename _Key, typename _Compare, typename _Alloc>
    inline void
    swap(set<_Key, _Compare, _Alloc>& __x, set<_Key, _Compare, _Alloc>& __y)
    _GLIBCXX_NOEXCEPT_IF(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

_GLIBCXX_END_NAMESPACE_CONTAINER

#if __cplusplus > 201402L
  // Allow std::set access to internals of compatible sets.
  template<typename _Val, typename _Cmp1, typename _Alloc, typename _Cmp2>
    struct
    _Rb_tree_merge_helper<_GLIBCXX_STD_C::set<_Val, _Cmp1, _Alloc>, _Cmp2>
    {
    private:
      friend class _GLIBCXX_STD_C::set<_Val, _Cmp1, _Alloc>;

      static auto&
      _S_get_tree(_GLIBCXX_STD_C::set<_Val, _Cmp2, _Alloc>& __set)
      { return __set._M_t; }

      static auto&
      _S_get_tree(_GLIBCXX_STD_C::multiset<_Val, _Cmp2, _Alloc>& __set)
      { return __set._M_t; }
    };
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} //namespace std
#endif /* _STL_SET_H */
