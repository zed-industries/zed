// unordered_set implementation -*- C++ -*-

// Copyright (C) 2010-2020 Free Software Foundation, Inc.
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

/** @file bits/unordered_set.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{unordered_set}
 */

#ifndef _UNORDERED_SET_H
#define _UNORDERED_SET_H

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CONTAINER

  /// Base types for unordered_set.
  template<bool _Cache>
    using __uset_traits = __detail::_Hashtable_traits<_Cache, true, true>;

  template<typename _Value,
	   typename _Hash = hash<_Value>,
	   typename _Pred = std::equal_to<_Value>,
  	   typename _Alloc = std::allocator<_Value>,
	   typename _Tr = __uset_traits<__cache_default<_Value, _Hash>::value>>
    using __uset_hashtable = _Hashtable<_Value, _Value, _Alloc,
					__detail::_Identity, _Pred, _Hash,
					__detail::_Mod_range_hashing,
					__detail::_Default_ranged_hash,
					__detail::_Prime_rehash_policy, _Tr>;

  /// Base types for unordered_multiset.
  template<bool _Cache>
    using __umset_traits = __detail::_Hashtable_traits<_Cache, true, false>;

  template<typename _Value,
	   typename _Hash = hash<_Value>,
	   typename _Pred = std::equal_to<_Value>,
	   typename _Alloc = std::allocator<_Value>,
	   typename _Tr = __umset_traits<__cache_default<_Value, _Hash>::value>>
    using __umset_hashtable = _Hashtable<_Value, _Value, _Alloc,
					 __detail::_Identity,
					 _Pred, _Hash,
					 __detail::_Mod_range_hashing,
					 __detail::_Default_ranged_hash,
					 __detail::_Prime_rehash_policy, _Tr>;

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    class unordered_multiset;

  /**
   *  @brief A standard container composed of unique keys (containing
   *  at most one of each key value) in which the elements' keys are
   *  the elements themselves.
   *
   *  @ingroup unordered_associative_containers
   *
   *  @tparam  _Value  Type of key objects.
   *  @tparam  _Hash  Hashing function object type, defaults to hash<_Value>.

   *  @tparam _Pred Predicate function object type, defaults to
   *                equal_to<_Value>.
   *
   *  @tparam  _Alloc  Allocator type, defaults to allocator<_Key>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, and
   *  <a href="tables.html#xx">unordered associative container</a>
   *
   *  Base is _Hashtable, dispatched at compile time via template
   *  alias __uset_hashtable.
   */
  template<typename _Value,
	   typename _Hash = hash<_Value>,
	   typename _Pred = equal_to<_Value>,
	   typename _Alloc = allocator<_Value>>
    class unordered_set
    {
      typedef __uset_hashtable<_Value, _Hash, _Pred, _Alloc>  _Hashtable;
      _Hashtable _M_h;

    public:
      // typedefs:
      //@{
      /// Public typedefs.
      typedef typename _Hashtable::key_type	key_type;
      typedef typename _Hashtable::value_type	value_type;
      typedef typename _Hashtable::hasher	hasher;
      typedef typename _Hashtable::key_equal	key_equal;
      typedef typename _Hashtable::allocator_type allocator_type;
      //@}

      //@{
      ///  Iterator-related typedefs.
      typedef typename _Hashtable::pointer		pointer;
      typedef typename _Hashtable::const_pointer	const_pointer;
      typedef typename _Hashtable::reference		reference;
      typedef typename _Hashtable::const_reference	const_reference;
      typedef typename _Hashtable::iterator		iterator;
      typedef typename _Hashtable::const_iterator	const_iterator;
      typedef typename _Hashtable::local_iterator	local_iterator;
      typedef typename _Hashtable::const_local_iterator	const_local_iterator;
      typedef typename _Hashtable::size_type		size_type;
      typedef typename _Hashtable::difference_type	difference_type;
      //@}

#if __cplusplus > 201402L
      using node_type = typename _Hashtable::node_type;
      using insert_return_type = typename _Hashtable::insert_return_type;
#endif

      // construct/destroy/copy

      /// Default constructor.
      unordered_set() = default;

      /**
       *  @brief  Default constructor creates no elements.
       *  @param __n  Minimal initial number of buckets.
       *  @param __hf  A hash functor.
       *  @param __eql  A key equality functor.
       *  @param __a  An allocator object.
       */
      explicit
      unordered_set(size_type __n,
		    const hasher& __hf = hasher(),
		    const key_equal& __eql = key_equal(),
		    const allocator_type& __a = allocator_type())
      : _M_h(__n, __hf, __eql, __a)
      { }

      /**
       *  @brief  Builds an %unordered_set from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @param __n  Minimal initial number of buckets.
       *  @param __hf  A hash functor.
       *  @param __eql  A key equality functor.
       *  @param __a  An allocator object.
       *
       *  Create an %unordered_set consisting of copies of the elements from
       *  [__first,__last).  This is linear in N (where N is
       *  distance(__first,__last)).
       */
      template<typename _InputIterator>
	unordered_set(_InputIterator __first, _InputIterator __last,
		      size_type __n = 0,
		      const hasher& __hf = hasher(),
		      const key_equal& __eql = key_equal(),
		      const allocator_type& __a = allocator_type())
	: _M_h(__first, __last, __n, __hf, __eql, __a)
	{ }

      /// Copy constructor.
      unordered_set(const unordered_set&) = default;

      /// Move constructor.
      unordered_set(unordered_set&&) = default;

      /**
       *  @brief Creates an %unordered_set with no elements.
       *  @param __a An allocator object.
       */
      explicit
      unordered_set(const allocator_type& __a)
      : _M_h(__a)
      { }

      /*
       *  @brief Copy constructor with allocator argument.
       * @param  __uset  Input %unordered_set to copy.
       * @param  __a  An allocator object.
       */
      unordered_set(const unordered_set& __uset,
		    const allocator_type& __a)
      : _M_h(__uset._M_h, __a)
      { }

      /*
       *  @brief  Move constructor with allocator argument.
       *  @param  __uset Input %unordered_set to move.
       *  @param  __a    An allocator object.
       */
      unordered_set(unordered_set&& __uset,
		    const allocator_type& __a)
      : _M_h(std::move(__uset._M_h), __a)
      { }

      /**
       *  @brief  Builds an %unordered_set from an initializer_list.
       *  @param  __l  An initializer_list.
       *  @param __n  Minimal initial number of buckets.
       *  @param __hf  A hash functor.
       *  @param __eql  A key equality functor.
       *  @param  __a  An allocator object.
       *
       *  Create an %unordered_set consisting of copies of the elements in the
       *  list. This is linear in N (where N is @a __l.size()).
       */
      unordered_set(initializer_list<value_type> __l,
		    size_type __n = 0,
		    const hasher& __hf = hasher(),
		    const key_equal& __eql = key_equal(),
		    const allocator_type& __a = allocator_type())
      : _M_h(__l, __n, __hf, __eql, __a)
      { }

      unordered_set(size_type __n, const allocator_type& __a)
      : unordered_set(__n, hasher(), key_equal(), __a)
      { }

      unordered_set(size_type __n, const hasher& __hf,
		    const allocator_type& __a)
      : unordered_set(__n, __hf, key_equal(), __a)
      { }

      template<typename _InputIterator>
	unordered_set(_InputIterator __first, _InputIterator __last,
		      size_type __n,
		      const allocator_type& __a)
	: unordered_set(__first, __last, __n, hasher(), key_equal(), __a)
	{ }

      template<typename _InputIterator>
	unordered_set(_InputIterator __first, _InputIterator __last,
		      size_type __n, const hasher& __hf,
		      const allocator_type& __a)
	: unordered_set(__first, __last, __n, __hf, key_equal(), __a)
	{ }

      unordered_set(initializer_list<value_type> __l,
		    size_type __n,
		    const allocator_type& __a)
      : unordered_set(__l, __n, hasher(), key_equal(), __a)
      { }

      unordered_set(initializer_list<value_type> __l,
		    size_type __n, const hasher& __hf,
		    const allocator_type& __a)
      : unordered_set(__l, __n, __hf, key_equal(), __a)
      { }

      /// Copy assignment operator.
      unordered_set&
      operator=(const unordered_set&) = default;

      /// Move assignment operator.
      unordered_set&
      operator=(unordered_set&&) = default;

      /**
       *  @brief  %Unordered_set list assignment operator.
       *  @param  __l  An initializer_list.
       *
       *  This function fills an %unordered_set with copies of the elements in
       *  the initializer list @a __l.
       *
       *  Note that the assignment completely changes the %unordered_set and
       *  that the resulting %unordered_set's size is the same as the number
       *  of elements assigned.
       */
      unordered_set&
      operator=(initializer_list<value_type> __l)
      {
	_M_h = __l;
	return *this;
      }

      ///  Returns the allocator object used by the %unordered_set.
      allocator_type
      get_allocator() const noexcept
      { return _M_h.get_allocator(); }

      // size and capacity:

      ///  Returns true if the %unordered_set is empty.
      _GLIBCXX_NODISCARD bool
      empty() const noexcept
      { return _M_h.empty(); }

      ///  Returns the size of the %unordered_set.
      size_type
      size() const noexcept
      { return _M_h.size(); }

      ///  Returns the maximum size of the %unordered_set.
      size_type
      max_size() const noexcept
      { return _M_h.max_size(); }

      // iterators.

      //@{
      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %unordered_set.
       */
      iterator
      begin() noexcept
      { return _M_h.begin(); }

      const_iterator
      begin() const noexcept
      { return _M_h.begin(); }
      //@}

      //@{
      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %unordered_set.
       */
      iterator
      end() noexcept
      { return _M_h.end(); }

      const_iterator
      end() const noexcept
      { return _M_h.end(); }
      //@}

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %unordered_set.
       */
      const_iterator
      cbegin() const noexcept
      { return _M_h.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %unordered_set.
       */
      const_iterator
      cend() const noexcept
      { return _M_h.end(); }

      // modifiers.

      /**
       *  @brief Attempts to build and insert an element into the
       *  %unordered_set.
       *  @param __args  Arguments used to generate an element.
       *  @return  A pair, of which the first element is an iterator that points
       *           to the possibly inserted element, and the second is a bool
       *           that is true if the element was actually inserted.
       *
       *  This function attempts to build and insert an element into the
       *  %unordered_set. An %unordered_set relies on unique keys and thus an
       *  element is only inserted if it is not already present in the
       *  %unordered_set.
       *
       *  Insertion requires amortized constant time.
       */
      template<typename... _Args>
	std::pair<iterator, bool>
	emplace(_Args&&... __args)
	{ return _M_h.emplace(std::forward<_Args>(__args)...); }

      /**
       *  @brief Attempts to insert an element into the %unordered_set.
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
       *  Insertion requires amortized constant time.
       */
      template<typename... _Args>
	iterator
	emplace_hint(const_iterator __pos, _Args&&... __args)
	{ return _M_h.emplace_hint(__pos, std::forward<_Args>(__args)...); }

      //@{
      /**
       *  @brief Attempts to insert an element into the %unordered_set.
       *  @param  __x  Element to be inserted.
       *  @return  A pair, of which the first element is an iterator that points
       *           to the possibly inserted element, and the second is a bool
       *           that is true if the element was actually inserted.
       *
       *  This function attempts to insert an element into the %unordered_set.
       *  An %unordered_set relies on unique keys and thus an element is only
       *  inserted if it is not already present in the %unordered_set.
       *
       *  Insertion requires amortized constant time.
       */
      std::pair<iterator, bool>
      insert(const value_type& __x)
      { return _M_h.insert(__x); }

      std::pair<iterator, bool>
      insert(value_type&& __x)
      { return _M_h.insert(std::move(__x)); }
      //@}

      //@{
      /**
       *  @brief Attempts to insert an element into the %unordered_set.
       *  @param  __hint  An iterator that serves as a hint as to where the
       *                 element should be inserted.
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
       *  Insertion requires amortized constant.
       */
      iterator
      insert(const_iterator __hint, const value_type& __x)
      { return _M_h.insert(__hint, __x); }

      iterator
      insert(const_iterator __hint, value_type&& __x)
      { return _M_h.insert(__hint, std::move(__x)); }
      //@}

      /**
       *  @brief A template function that attempts to insert a range of
       *  elements.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                   inserted.
       *  @param  __last  Iterator pointing to the end of the range.
       *
       *  Complexity similar to that of the range constructor.
       */
      template<typename _InputIterator>
	void
	insert(_InputIterator __first, _InputIterator __last)
	{ _M_h.insert(__first, __last); }

      /**
       *  @brief Attempts to insert a list of elements into the %unordered_set.
       *  @param  __l  A std::initializer_list<value_type> of elements
       *               to be inserted.
       *
       *  Complexity similar to that of the range constructor.
       */
      void
      insert(initializer_list<value_type> __l)
      { _M_h.insert(__l); }

#if __cplusplus > 201402L
      /// Extract a node.
      node_type
      extract(const_iterator __pos)
      {
	__glibcxx_assert(__pos != end());
	return _M_h.extract(__pos);
      }

      /// Extract a node.
      node_type
      extract(const key_type& __key)
      { return _M_h.extract(__key); }

      /// Re-insert an extracted node.
      insert_return_type
      insert(node_type&& __nh)
      { return _M_h._M_reinsert_node(std::move(__nh)); }

      /// Re-insert an extracted node.
      iterator
      insert(const_iterator, node_type&& __nh)
      { return _M_h._M_reinsert_node(std::move(__nh)).position; }
#endif // C++17

      //@{
      /**
       *  @brief Erases an element from an %unordered_set.
       *  @param  __position  An iterator pointing to the element to be erased.
       *  @return An iterator pointing to the element immediately following
       *          @a __position prior to the element being erased. If no such
       *          element exists, end() is returned.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from an %unordered_set.  Note that this function only erases the
       *  element, and that if the element is itself a pointer, the pointed-to
       *  memory is not touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      iterator
      erase(const_iterator __position)
      { return _M_h.erase(__position); }

      // LWG 2059.
      iterator
      erase(iterator __position)
      { return _M_h.erase(__position); }
      //@}

      /**
       *  @brief Erases elements according to the provided key.
       *  @param  __x  Key of element to be erased.
       *  @return  The number of elements erased.
       *
       *  This function erases all the elements located by the given key from
       *  an %unordered_set. For an %unordered_set the result of this function
       *  can only be 0 (not present) or 1 (present).
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      size_type
      erase(const key_type& __x)
      { return _M_h.erase(__x); }

      /**
       *  @brief Erases a [__first,__last) range of elements from an
       *  %unordered_set.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                  erased.
       *  @param __last  Iterator pointing to the end of the range to
       *                be erased.
       *  @return The iterator @a __last.
       *
       *  This function erases a sequence of elements from an %unordered_set.
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      iterator
      erase(const_iterator __first, const_iterator __last)
      { return _M_h.erase(__first, __last); }

      /**
       *  Erases all elements in an %unordered_set. Note that this function only
       *  erases the elements, and that if the elements themselves are pointers,
       *  the pointed-to memory is not touched in any way. Managing the pointer
       *  is the user's responsibility.
       */
      void
      clear() noexcept
      { _M_h.clear(); }

      /**
       *  @brief  Swaps data with another %unordered_set.
       *  @param  __x  An %unordered_set of the same element and allocator
       *  types.
       *
       *  This exchanges the elements between two sets in constant time.
       *  Note that the global std::swap() function is specialized such that
       *  std::swap(s1,s2) will feed to this function.
       */
      void
      swap(unordered_set& __x)
      noexcept( noexcept(_M_h.swap(__x._M_h)) )
      { _M_h.swap(__x._M_h); }

#if __cplusplus > 201402L
      template<typename, typename, typename>
	friend class std::_Hash_merge_helper;

      template<typename _H2, typename _P2>
	void
	merge(unordered_set<_Value, _H2, _P2, _Alloc>& __source)
	{
	  using _Merge_helper = _Hash_merge_helper<unordered_set, _H2, _P2>;
	  _M_h._M_merge_unique(_Merge_helper::_S_get_table(__source));
	}

      template<typename _H2, typename _P2>
	void
	merge(unordered_set<_Value, _H2, _P2, _Alloc>&& __source)
	{ merge(__source); }

      template<typename _H2, typename _P2>
	void
	merge(unordered_multiset<_Value, _H2, _P2, _Alloc>& __source)
	{
	  using _Merge_helper = _Hash_merge_helper<unordered_set, _H2, _P2>;
	  _M_h._M_merge_unique(_Merge_helper::_S_get_table(__source));
	}

      template<typename _H2, typename _P2>
	void
	merge(unordered_multiset<_Value, _H2, _P2, _Alloc>&& __source)
	{ merge(__source); }
#endif // C++17

      // observers.

      ///  Returns the hash functor object with which the %unordered_set was
      ///  constructed.
      hasher
      hash_function() const
      { return _M_h.hash_function(); }

      ///  Returns the key comparison object with which the %unordered_set was
      ///  constructed.
      key_equal
      key_eq() const
      { return _M_h.key_eq(); }

      // lookup.

      //@{
      /**
       *  @brief Tries to locate an element in an %unordered_set.
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
      { return _M_h.find(__x); }

      const_iterator
      find(const key_type& __x) const
      { return _M_h.find(__x); }
      //@}

      /**
       *  @brief  Finds the number of elements.
       *  @param  __x  Element to located.
       *  @return  Number of elements with specified key.
       *
       *  This function only makes sense for unordered_multisets; for
       *  unordered_set the result will either be 0 (not present) or 1
       *  (present).
       */
      size_type
      count(const key_type& __x) const
      { return _M_h.count(__x); }

#if __cplusplus > 201703L
      /**
       *  @brief  Finds whether an element with the given key exists.
       *  @param  __x  Key of elements to be located.
       *  @return  True if there is any element with the specified key.
       */
      bool
      contains(const key_type& __x) const
      { return _M_h.find(__x) != _M_h.end(); }
#endif

      //@{
      /**
       *  @brief Finds a subsequence matching given key.
       *  @param  __x  Key to be located.
       *  @return  Pair of iterators that possibly points to the subsequence
       *           matching given key.
       *
       *  This function probably only makes sense for multisets.
       */
      std::pair<iterator, iterator>
      equal_range(const key_type& __x)
      { return _M_h.equal_range(__x); }

      std::pair<const_iterator, const_iterator>
      equal_range(const key_type& __x) const
      { return _M_h.equal_range(__x); }
      //@}

      // bucket interface.

      /// Returns the number of buckets of the %unordered_set.
      size_type
      bucket_count() const noexcept
      { return _M_h.bucket_count(); }

      /// Returns the maximum number of buckets of the %unordered_set.
      size_type
      max_bucket_count() const noexcept
      { return _M_h.max_bucket_count(); }

      /*
       * @brief  Returns the number of elements in a given bucket.
       * @param  __n  A bucket index.
       * @return  The number of elements in the bucket.
       */
      size_type
      bucket_size(size_type __n) const
      { return _M_h.bucket_size(__n); }

      /*
       * @brief  Returns the bucket index of a given element.
       * @param  __key  A key instance.
       * @return  The key bucket index.
       */
      size_type
      bucket(const key_type& __key) const
      { return _M_h.bucket(__key); }

      //@{
      /**
       *  @brief  Returns a read-only (constant) iterator pointing to the first
       *         bucket element.
       *  @param  __n The bucket index.
       *  @return  A read-only local iterator.
       */
      local_iterator
      begin(size_type __n)
      { return _M_h.begin(__n); }

      const_local_iterator
      begin(size_type __n) const
      { return _M_h.begin(__n); }

      const_local_iterator
      cbegin(size_type __n) const
      { return _M_h.cbegin(__n); }
      //@}

      //@{
      /**
       *  @brief  Returns a read-only (constant) iterator pointing to one past
       *         the last bucket elements.
       *  @param  __n The bucket index.
       *  @return  A read-only local iterator.
       */
      local_iterator
      end(size_type __n)
      { return _M_h.end(__n); }

      const_local_iterator
      end(size_type __n) const
      { return _M_h.end(__n); }

      const_local_iterator
      cend(size_type __n) const
      { return _M_h.cend(__n); }
      //@}

      // hash policy.

      /// Returns the average number of elements per bucket.
      float
      load_factor() const noexcept
      { return _M_h.load_factor(); }

      /// Returns a positive number that the %unordered_set tries to keep the
      /// load factor less than or equal to.
      float
      max_load_factor() const noexcept
      { return _M_h.max_load_factor(); }

      /**
       *  @brief  Change the %unordered_set maximum load factor.
       *  @param  __z The new maximum load factor.
       */
      void
      max_load_factor(float __z)
      { _M_h.max_load_factor(__z); }

      /**
       *  @brief  May rehash the %unordered_set.
       *  @param  __n The new number of buckets.
       *
       *  Rehash will occur only if the new number of buckets respect the
       *  %unordered_set maximum load factor.
       */
      void
      rehash(size_type __n)
      { _M_h.rehash(__n); }

      /**
       *  @brief  Prepare the %unordered_set for a specified number of
       *          elements.
       *  @param  __n Number of elements required.
       *
       *  Same as rehash(ceil(n / max_load_factor())).
       */
      void
      reserve(size_type __n)
      { _M_h.reserve(__n); }

      template<typename _Value1, typename _Hash1, typename _Pred1,
	       typename _Alloc1>
        friend bool
        operator==(const unordered_set<_Value1, _Hash1, _Pred1, _Alloc1>&,
		   const unordered_set<_Value1, _Hash1, _Pred1, _Alloc1>&);
    };

#if __cpp_deduction_guides >= 201606

  template<typename _InputIterator,
	   typename _Hash =
	     hash<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Pred =
	     equal_to<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Allocator =
	     allocator<typename iterator_traits<_InputIterator>::value_type>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireNotAllocator<_Pred>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(_InputIterator, _InputIterator,
		  unordered_set<int>::size_type = {},
		  _Hash = _Hash(), _Pred = _Pred(), _Allocator = _Allocator())
    -> unordered_set<typename iterator_traits<_InputIterator>::value_type,
		     _Hash, _Pred, _Allocator>;

  template<typename _Tp, typename _Hash = hash<_Tp>,
	   typename _Pred = equal_to<_Tp>,
	   typename _Allocator = allocator<_Tp>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireNotAllocator<_Pred>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(initializer_list<_Tp>,
		  unordered_set<int>::size_type = {},
		  _Hash = _Hash(), _Pred = _Pred(), _Allocator = _Allocator())
    -> unordered_set<_Tp, _Hash, _Pred, _Allocator>;

  template<typename _InputIterator, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(_InputIterator, _InputIterator,
		  unordered_set<int>::size_type, _Allocator)
    -> unordered_set<typename iterator_traits<_InputIterator>::value_type,
		     hash<
		       typename iterator_traits<_InputIterator>::value_type>,
		     equal_to<
		       typename iterator_traits<_InputIterator>::value_type>,
		     _Allocator>;

  template<typename _InputIterator, typename _Hash, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(_InputIterator, _InputIterator,
		  unordered_set<int>::size_type,
		  _Hash, _Allocator)
    -> unordered_set<typename iterator_traits<_InputIterator>::value_type,
		     _Hash,
		     equal_to<
		       typename iterator_traits<_InputIterator>::value_type>,
		     _Allocator>;

  template<typename _Tp, typename _Allocator,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(initializer_list<_Tp>,
		  unordered_set<int>::size_type, _Allocator)
    -> unordered_set<_Tp, hash<_Tp>, equal_to<_Tp>, _Allocator>;

  template<typename _Tp, typename _Hash, typename _Allocator,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_set(initializer_list<_Tp>,
		  unordered_set<int>::size_type, _Hash, _Allocator)
    -> unordered_set<_Tp, _Hash, equal_to<_Tp>, _Allocator>;

#endif

  /**
   *  @brief A standard container composed of equivalent keys
   *  (possibly containing multiple of each key value) in which the
   *  elements' keys are the elements themselves.
   *
   *  @ingroup unordered_associative_containers
   *
   *  @tparam  _Value  Type of key objects.
   *  @tparam  _Hash  Hashing function object type, defaults to hash<_Value>.
   *  @tparam  _Pred  Predicate function object type, defaults
   *                  to equal_to<_Value>.
   *  @tparam  _Alloc  Allocator type, defaults to allocator<_Key>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, and
   *  <a href="tables.html#xx">unordered associative container</a>
   *
   *  Base is _Hashtable, dispatched at compile time via template
   *  alias __umset_hashtable.
   */
  template<typename _Value,
	   typename _Hash = hash<_Value>,
	   typename _Pred = equal_to<_Value>,
	   typename _Alloc = allocator<_Value>>
    class unordered_multiset
    {
      typedef __umset_hashtable<_Value, _Hash, _Pred, _Alloc>  _Hashtable;
      _Hashtable _M_h;

    public:
      // typedefs:
      //@{
      /// Public typedefs.
      typedef typename _Hashtable::key_type	key_type;
      typedef typename _Hashtable::value_type	value_type;
      typedef typename _Hashtable::hasher	hasher;
      typedef typename _Hashtable::key_equal	key_equal;
      typedef typename _Hashtable::allocator_type allocator_type;
      //@}

      //@{
      ///  Iterator-related typedefs.
      typedef typename _Hashtable::pointer		pointer;
      typedef typename _Hashtable::const_pointer	const_pointer;
      typedef typename _Hashtable::reference		reference;
      typedef typename _Hashtable::const_reference	const_reference;
      typedef typename _Hashtable::iterator		iterator;
      typedef typename _Hashtable::const_iterator	const_iterator;
      typedef typename _Hashtable::local_iterator	local_iterator;
      typedef typename _Hashtable::const_local_iterator	const_local_iterator;
      typedef typename _Hashtable::size_type		size_type;
      typedef typename _Hashtable::difference_type	difference_type;
      //@}

#if __cplusplus > 201402L
      using node_type = typename _Hashtable::node_type;
#endif

      // construct/destroy/copy

      /// Default constructor.
      unordered_multiset() = default;

      /**
       *  @brief  Default constructor creates no elements.
       *  @param __n  Minimal initial number of buckets.
       *  @param __hf  A hash functor.
       *  @param __eql  A key equality functor.
       *  @param __a  An allocator object.
       */
      explicit
      unordered_multiset(size_type __n,
			 const hasher& __hf = hasher(),
			 const key_equal& __eql = key_equal(),
			 const allocator_type& __a = allocator_type())
      : _M_h(__n, __hf, __eql, __a)
      { }

      /**
       *  @brief  Builds an %unordered_multiset from a range.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *  @param __n       Minimal initial number of buckets.
       *  @param __hf      A hash functor.
       *  @param __eql     A key equality functor.
       *  @param __a       An allocator object.
       *
       *  Create an %unordered_multiset consisting of copies of the elements
       *  from [__first,__last).  This is linear in N (where N is
       *  distance(__first,__last)).
       */
      template<typename _InputIterator>
	unordered_multiset(_InputIterator __first, _InputIterator __last,
			   size_type __n = 0,
			   const hasher& __hf = hasher(),
			   const key_equal& __eql = key_equal(),
			   const allocator_type& __a = allocator_type())
	: _M_h(__first, __last, __n, __hf, __eql, __a)
	{ }

      /// Copy constructor.
      unordered_multiset(const unordered_multiset&) = default;

      /// Move constructor.
      unordered_multiset(unordered_multiset&&) = default;

      /**
       *  @brief  Builds an %unordered_multiset from an initializer_list.
       *  @param  __l  An initializer_list.
       *  @param __n  Minimal initial number of buckets.
       *  @param __hf  A hash functor.
       *  @param __eql  A key equality functor.
       *  @param  __a  An allocator object.
       *
       *  Create an %unordered_multiset consisting of copies of the elements in
       *  the list. This is linear in N (where N is @a __l.size()).
       */
      unordered_multiset(initializer_list<value_type> __l,
			 size_type __n = 0,
			 const hasher& __hf = hasher(),
			 const key_equal& __eql = key_equal(),
			 const allocator_type& __a = allocator_type())
      : _M_h(__l, __n, __hf, __eql, __a)
      { }

      /// Copy assignment operator.
      unordered_multiset&
      operator=(const unordered_multiset&) = default;

      /// Move assignment operator.
      unordered_multiset&
      operator=(unordered_multiset&&) = default;

      /**
       *  @brief Creates an %unordered_multiset with no elements.
       *  @param __a An allocator object.
       */
      explicit
      unordered_multiset(const allocator_type& __a)
      : _M_h(__a)
      { }

      /*
       *  @brief Copy constructor with allocator argument.
       * @param  __uset  Input %unordered_multiset to copy.
       * @param  __a  An allocator object.
       */
      unordered_multiset(const unordered_multiset& __umset,
			 const allocator_type& __a)
      : _M_h(__umset._M_h, __a)
      { }

      /*
       *  @brief  Move constructor with allocator argument.
       *  @param  __umset  Input %unordered_multiset to move.
       *  @param  __a  An allocator object.
       */
      unordered_multiset(unordered_multiset&& __umset,
			 const allocator_type& __a)
      : _M_h(std::move(__umset._M_h), __a)
      { }

      unordered_multiset(size_type __n, const allocator_type& __a)
      : unordered_multiset(__n, hasher(), key_equal(), __a)
      { }

      unordered_multiset(size_type __n, const hasher& __hf,
			 const allocator_type& __a)
      : unordered_multiset(__n, __hf, key_equal(), __a)
      { }

      template<typename _InputIterator>
	unordered_multiset(_InputIterator __first, _InputIterator __last,
			   size_type __n,
			   const allocator_type& __a)
	: unordered_multiset(__first, __last, __n, hasher(), key_equal(), __a)
	{ }

      template<typename _InputIterator>
	unordered_multiset(_InputIterator __first, _InputIterator __last,
			   size_type __n, const hasher& __hf,
			   const allocator_type& __a)
	: unordered_multiset(__first, __last, __n, __hf, key_equal(), __a)
	{ }

      unordered_multiset(initializer_list<value_type> __l,
			 size_type __n,
			 const allocator_type& __a)
      : unordered_multiset(__l, __n, hasher(), key_equal(), __a)
      { }

      unordered_multiset(initializer_list<value_type> __l,
			 size_type __n, const hasher& __hf,
			 const allocator_type& __a)
      : unordered_multiset(__l, __n, __hf, key_equal(), __a)
      { }

      /**
       *  @brief  %Unordered_multiset list assignment operator.
       *  @param  __l  An initializer_list.
       *
       *  This function fills an %unordered_multiset with copies of the elements
       *  in the initializer list @a __l.
       *
       *  Note that the assignment completely changes the %unordered_multiset
       *  and that the resulting %unordered_multiset's size is the same as the
       *  number of elements assigned.
       */
      unordered_multiset&
      operator=(initializer_list<value_type> __l)
      {
	_M_h = __l;
	return *this;
      }

      ///  Returns the allocator object used by the %unordered_multiset.
      allocator_type
      get_allocator() const noexcept
      { return _M_h.get_allocator(); }

      // size and capacity:

      ///  Returns true if the %unordered_multiset is empty.
      _GLIBCXX_NODISCARD bool
      empty() const noexcept
      { return _M_h.empty(); }

      ///  Returns the size of the %unordered_multiset.
      size_type
      size() const noexcept
      { return _M_h.size(); }

      ///  Returns the maximum size of the %unordered_multiset.
      size_type
      max_size() const noexcept
      { return _M_h.max_size(); }

      // iterators.

      //@{
      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %unordered_multiset.
       */
      iterator
      begin() noexcept
      { return _M_h.begin(); }

      const_iterator
      begin() const noexcept
      { return _M_h.begin(); }
      //@}

      //@{
      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %unordered_multiset.
       */
      iterator
      end() noexcept
      { return _M_h.end(); }

      const_iterator
      end() const noexcept
      { return _M_h.end(); }
      //@}

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %unordered_multiset.
       */
      const_iterator
      cbegin() const noexcept
      { return _M_h.begin(); }

      /**
       *  Returns a read-only (constant) iterator that points one past the last
       *  element in the %unordered_multiset.
       */
      const_iterator
      cend() const noexcept
      { return _M_h.end(); }

      // modifiers.

      /**
       *  @brief Builds and insert an element into the %unordered_multiset.
       *  @param __args  Arguments used to generate an element.
       *  @return  An iterator that points to the inserted element.
       *
       *  Insertion requires amortized constant time.
       */
      template<typename... _Args>
	iterator
	emplace(_Args&&... __args)
	{ return _M_h.emplace(std::forward<_Args>(__args)...); }

      /**
       *  @brief Inserts an element into the %unordered_multiset.
       *  @param  __pos  An iterator that serves as a hint as to where the
       *                element should be inserted.
       *  @param  __args  Arguments used to generate the element to be
       *                 inserted.
       *  @return An iterator that points to the inserted element.
       *
       *  Note that the first parameter is only a hint and can potentially
       *  improve the performance of the insertion process.  A bad hint would
       *  cause no gains in efficiency.
       *
       *  For more on @a hinting, see:
       *  https://gcc.gnu.org/onlinedocs/libstdc++/manual/associative.html#containers.associative.insert_hints
       *
       *  Insertion requires amortized constant time.
       */
      template<typename... _Args>
	iterator
	emplace_hint(const_iterator __pos, _Args&&... __args)
	{ return _M_h.emplace_hint(__pos, std::forward<_Args>(__args)...); }

      //@{
      /**
       *  @brief Inserts an element into the %unordered_multiset.
       *  @param  __x  Element to be inserted.
       *  @return  An iterator that points to the inserted element.
       *
       *  Insertion requires amortized constant time.
       */
      iterator
      insert(const value_type& __x)
      { return _M_h.insert(__x); }

      iterator
      insert(value_type&& __x)
      { return _M_h.insert(std::move(__x)); }
      //@}

      //@{
      /**
       *  @brief Inserts an element into the %unordered_multiset.
       *  @param  __hint  An iterator that serves as a hint as to where the
       *                 element should be inserted.
       *  @param  __x  Element to be inserted.
       *  @return An iterator that points to the inserted element.
       *
       *  Note that the first parameter is only a hint and can potentially
       *  improve the performance of the insertion process.  A bad hint would
       *  cause no gains in efficiency.
       *
       *  For more on @a hinting, see:
       *  https://gcc.gnu.org/onlinedocs/libstdc++/manual/associative.html#containers.associative.insert_hints
       *
       *  Insertion requires amortized constant.
       */
      iterator
      insert(const_iterator __hint, const value_type& __x)
      { return _M_h.insert(__hint, __x); }

      iterator
      insert(const_iterator __hint, value_type&& __x)
      { return _M_h.insert(__hint, std::move(__x)); }
      //@}

      /**
       *  @brief A template function that inserts a range of elements.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                   inserted.
       *  @param  __last  Iterator pointing to the end of the range.
       *
       *  Complexity similar to that of the range constructor.
       */
      template<typename _InputIterator>
	void
	insert(_InputIterator __first, _InputIterator __last)
	{ _M_h.insert(__first, __last); }

      /**
       *  @brief Inserts a list of elements into the %unordered_multiset.
       *  @param  __l  A std::initializer_list<value_type> of elements to be
       *              inserted.
       *
       *  Complexity similar to that of the range constructor.
       */
      void
      insert(initializer_list<value_type> __l)
      { _M_h.insert(__l); }

#if __cplusplus > 201402L
      /// Extract a node.
      node_type
      extract(const_iterator __pos)
      {
	__glibcxx_assert(__pos != end());
	return _M_h.extract(__pos);
      }

      /// Extract a node.
      node_type
      extract(const key_type& __key)
      { return _M_h.extract(__key); }

      /// Re-insert an extracted node.
      iterator
      insert(node_type&& __nh)
      { return _M_h._M_reinsert_node_multi(cend(), std::move(__nh)); }

      /// Re-insert an extracted node.
      iterator
      insert(const_iterator __hint, node_type&& __nh)
      { return _M_h._M_reinsert_node_multi(__hint, std::move(__nh)); }
#endif // C++17

      //@{
      /**
       *  @brief Erases an element from an %unordered_multiset.
       *  @param  __position  An iterator pointing to the element to be erased.
       *  @return An iterator pointing to the element immediately following
       *          @a __position prior to the element being erased. If no such
       *          element exists, end() is returned.
       *
       *  This function erases an element, pointed to by the given iterator,
       *  from an %unordered_multiset.
       *
       *  Note that this function only erases the element, and that if the
       *  element is itself a pointer, the pointed-to memory is not touched in
       *  any way.  Managing the pointer is the user's responsibility.
       */
      iterator
      erase(const_iterator __position)
      { return _M_h.erase(__position); }

      // LWG 2059.
      iterator
      erase(iterator __position)
      { return _M_h.erase(__position); }
      //@}


      /**
       *  @brief Erases elements according to the provided key.
       *  @param  __x  Key of element to be erased.
       *  @return  The number of elements erased.
       *
       *  This function erases all the elements located by the given key from
       *  an %unordered_multiset.
       *
       *  Note that this function only erases the element, and that if the
       *  element is itself a pointer, the pointed-to memory is not touched in
       *  any way.  Managing the pointer is the user's responsibility.
       */
      size_type
      erase(const key_type& __x)
      { return _M_h.erase(__x); }

      /**
       *  @brief Erases a [__first,__last) range of elements from an
       *  %unordered_multiset.
       *  @param  __first  Iterator pointing to the start of the range to be
       *                  erased.
       *  @param __last  Iterator pointing to the end of the range to
       *                be erased.
       *  @return The iterator @a __last.
       *
       *  This function erases a sequence of elements from an
       *  %unordered_multiset.
       *
       *  Note that this function only erases the element, and that if
       *  the element is itself a pointer, the pointed-to memory is not touched
       *  in any way.  Managing the pointer is the user's responsibility.
       */
      iterator
      erase(const_iterator __first, const_iterator __last)
      { return _M_h.erase(__first, __last); }

      /**
       *  Erases all elements in an %unordered_multiset.
       *
       *  Note that this function only erases the elements, and that if the
       *  elements themselves are pointers, the pointed-to memory is not touched
       *  in any way. Managing the pointer is the user's responsibility.
       */
      void
      clear() noexcept
      { _M_h.clear(); }

      /**
       *  @brief  Swaps data with another %unordered_multiset.
       *  @param  __x  An %unordered_multiset of the same element and allocator
       *  types.
       *
       *  This exchanges the elements between two sets in constant time.
       *  Note that the global std::swap() function is specialized such that
       *  std::swap(s1,s2) will feed to this function.
       */
      void
      swap(unordered_multiset& __x)
      noexcept( noexcept(_M_h.swap(__x._M_h)) )
      { _M_h.swap(__x._M_h); }

#if __cplusplus > 201402L
      template<typename, typename, typename>
	friend class std::_Hash_merge_helper;

      template<typename _H2, typename _P2>
	void
	merge(unordered_multiset<_Value, _H2, _P2, _Alloc>& __source)
	{
	  using _Merge_helper
	    = _Hash_merge_helper<unordered_multiset, _H2, _P2>;
	  _M_h._M_merge_multi(_Merge_helper::_S_get_table(__source));
	}

      template<typename _H2, typename _P2>
	void
	merge(unordered_multiset<_Value, _H2, _P2, _Alloc>&& __source)
	{ merge(__source); }

      template<typename _H2, typename _P2>
	void
	merge(unordered_set<_Value, _H2, _P2, _Alloc>& __source)
	{
	  using _Merge_helper
	    = _Hash_merge_helper<unordered_multiset, _H2, _P2>;
	  _M_h._M_merge_multi(_Merge_helper::_S_get_table(__source));
	}

      template<typename _H2, typename _P2>
	void
	merge(unordered_set<_Value, _H2, _P2, _Alloc>&& __source)
	{ merge(__source); }
#endif // C++17

      // observers.

      ///  Returns the hash functor object with which the %unordered_multiset
      ///  was constructed.
      hasher
      hash_function() const
      { return _M_h.hash_function(); }

      ///  Returns the key comparison object with which the %unordered_multiset
      ///  was constructed.
      key_equal
      key_eq() const
      { return _M_h.key_eq(); }

      // lookup.

      //@{
      /**
       *  @brief Tries to locate an element in an %unordered_multiset.
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
      { return _M_h.find(__x); }

      const_iterator
      find(const key_type& __x) const
      { return _M_h.find(__x); }
      //@}

      /**
       *  @brief  Finds the number of elements.
       *  @param  __x  Element to located.
       *  @return  Number of elements with specified key.
       */
      size_type
      count(const key_type& __x) const
      { return _M_h.count(__x); }

#if __cplusplus > 201703L
      /**
       *  @brief  Finds whether an element with the given key exists.
       *  @param  __x  Key of elements to be located.
       *  @return  True if there is any element with the specified key.
       */
      bool
      contains(const key_type& __x) const
      { return _M_h.find(__x) != _M_h.end(); }
#endif

      //@{
      /**
       *  @brief Finds a subsequence matching given key.
       *  @param  __x  Key to be located.
       *  @return  Pair of iterators that possibly points to the subsequence
       *           matching given key.
       */
      std::pair<iterator, iterator>
      equal_range(const key_type& __x)
      { return _M_h.equal_range(__x); }

      std::pair<const_iterator, const_iterator>
      equal_range(const key_type& __x) const
      { return _M_h.equal_range(__x); }
      //@}

      // bucket interface.

      /// Returns the number of buckets of the %unordered_multiset.
      size_type
      bucket_count() const noexcept
      { return _M_h.bucket_count(); }

      /// Returns the maximum number of buckets of the %unordered_multiset.
      size_type
      max_bucket_count() const noexcept
      { return _M_h.max_bucket_count(); }

      /*
       * @brief  Returns the number of elements in a given bucket.
       * @param  __n  A bucket index.
       * @return  The number of elements in the bucket.
       */
      size_type
      bucket_size(size_type __n) const
      { return _M_h.bucket_size(__n); }

      /*
       * @brief  Returns the bucket index of a given element.
       * @param  __key  A key instance.
       * @return  The key bucket index.
       */
      size_type
      bucket(const key_type& __key) const
      { return _M_h.bucket(__key); }

      //@{
      /**
       *  @brief  Returns a read-only (constant) iterator pointing to the first
       *         bucket element.
       *  @param  __n The bucket index.
       *  @return  A read-only local iterator.
       */
      local_iterator
      begin(size_type __n)
      { return _M_h.begin(__n); }

      const_local_iterator
      begin(size_type __n) const
      { return _M_h.begin(__n); }

      const_local_iterator
      cbegin(size_type __n) const
      { return _M_h.cbegin(__n); }
      //@}

      //@{
      /**
       *  @brief  Returns a read-only (constant) iterator pointing to one past
       *         the last bucket elements.
       *  @param  __n The bucket index.
       *  @return  A read-only local iterator.
       */
      local_iterator
      end(size_type __n)
      { return _M_h.end(__n); }

      const_local_iterator
      end(size_type __n) const
      { return _M_h.end(__n); }

      const_local_iterator
      cend(size_type __n) const
      { return _M_h.cend(__n); }
      //@}

      // hash policy.

      /// Returns the average number of elements per bucket.
      float
      load_factor() const noexcept
      { return _M_h.load_factor(); }

      /// Returns a positive number that the %unordered_multiset tries to keep the
      /// load factor less than or equal to.
      float
      max_load_factor() const noexcept
      { return _M_h.max_load_factor(); }

      /**
       *  @brief  Change the %unordered_multiset maximum load factor.
       *  @param  __z The new maximum load factor.
       */
      void
      max_load_factor(float __z)
      { _M_h.max_load_factor(__z); }

      /**
       *  @brief  May rehash the %unordered_multiset.
       *  @param  __n The new number of buckets.
       *
       *  Rehash will occur only if the new number of buckets respect the
       *  %unordered_multiset maximum load factor.
       */
      void
      rehash(size_type __n)
      { _M_h.rehash(__n); }

      /**
       *  @brief  Prepare the %unordered_multiset for a specified number of
       *          elements.
       *  @param  __n Number of elements required.
       *
       *  Same as rehash(ceil(n / max_load_factor())).
       */
      void
      reserve(size_type __n)
      { _M_h.reserve(__n); }

      template<typename _Value1, typename _Hash1, typename _Pred1,
	       typename _Alloc1>
        friend bool
      operator==(const unordered_multiset<_Value1, _Hash1, _Pred1, _Alloc1>&,
		 const unordered_multiset<_Value1, _Hash1, _Pred1, _Alloc1>&);
    };


#if __cpp_deduction_guides >= 201606

  template<typename _InputIterator,
	   typename _Hash =
	     hash<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Pred =
	     equal_to<typename iterator_traits<_InputIterator>::value_type>,
	   typename _Allocator =
	     allocator<typename iterator_traits<_InputIterator>::value_type>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireNotAllocator<_Pred>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(_InputIterator, _InputIterator,
		       unordered_multiset<int>::size_type = {},
		       _Hash = _Hash(), _Pred = _Pred(),
		       _Allocator = _Allocator())
    -> unordered_multiset<typename iterator_traits<_InputIterator>::value_type,
                          _Hash, _Pred, _Allocator>;

  template<typename _Tp, typename _Hash = hash<_Tp>,
	   typename _Pred = equal_to<_Tp>,
	   typename _Allocator = allocator<_Tp>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireNotAllocator<_Pred>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(initializer_list<_Tp>,
		       unordered_multiset<int>::size_type = {},
		       _Hash = _Hash(), _Pred = _Pred(),
		       _Allocator = _Allocator())
    -> unordered_multiset<_Tp, _Hash, _Pred, _Allocator>;

  template<typename _InputIterator, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(_InputIterator, _InputIterator,
		       unordered_multiset<int>::size_type, _Allocator)
    -> unordered_multiset<typename iterator_traits<_InputIterator>::value_type,
			  hash<typename
			       iterator_traits<_InputIterator>::value_type>,
			  equal_to<typename
				   iterator_traits<_InputIterator>::value_type>,
			  _Allocator>;

  template<typename _InputIterator, typename _Hash, typename _Allocator,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(_InputIterator, _InputIterator,
		       unordered_multiset<int>::size_type,
		       _Hash, _Allocator)
    -> unordered_multiset<typename
			  iterator_traits<_InputIterator>::value_type,
			  _Hash,
			  equal_to<
			    typename
			    iterator_traits<_InputIterator>::value_type>,
			  _Allocator>;

  template<typename _Tp, typename _Allocator,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(initializer_list<_Tp>,
		       unordered_multiset<int>::size_type, _Allocator)
    -> unordered_multiset<_Tp, hash<_Tp>, equal_to<_Tp>, _Allocator>;

  template<typename _Tp, typename _Hash, typename _Allocator,
	   typename = _RequireNotAllocatorOrIntegral<_Hash>,
	   typename = _RequireAllocator<_Allocator>>
    unordered_multiset(initializer_list<_Tp>,
		       unordered_multiset<int>::size_type, _Hash, _Allocator)
    -> unordered_multiset<_Tp, _Hash, equal_to<_Tp>, _Allocator>;

#endif

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline void
    swap(unordered_set<_Value, _Hash, _Pred, _Alloc>& __x,
	 unordered_set<_Value, _Hash, _Pred, _Alloc>& __y)
    noexcept(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline void
    swap(unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __x,
	 unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __y)
    noexcept(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline bool
    operator==(const unordered_set<_Value, _Hash, _Pred, _Alloc>& __x,
	       const unordered_set<_Value, _Hash, _Pred, _Alloc>& __y)
    { return __x._M_h._M_equal(__y._M_h); }

#if __cpp_impl_three_way_comparison < 201907L
  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline bool
    operator!=(const unordered_set<_Value, _Hash, _Pred, _Alloc>& __x,
	       const unordered_set<_Value, _Hash, _Pred, _Alloc>& __y)
    { return !(__x == __y); }
#endif

  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline bool
    operator==(const unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __x,
	       const unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __y)
    { return __x._M_h._M_equal(__y._M_h); }

#if __cpp_impl_three_way_comparison < 201907L
  template<class _Value, class _Hash, class _Pred, class _Alloc>
    inline bool
    operator!=(const unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __x,
	       const unordered_multiset<_Value, _Hash, _Pred, _Alloc>& __y)
    { return !(__x == __y); }
#endif

_GLIBCXX_END_NAMESPACE_CONTAINER

#if __cplusplus > 201402L
  // Allow std::unordered_set access to internals of compatible sets.
  template<typename _Val, typename _Hash1, typename _Eq1, typename _Alloc,
	   typename _Hash2, typename _Eq2>
    struct _Hash_merge_helper<
      _GLIBCXX_STD_C::unordered_set<_Val, _Hash1, _Eq1, _Alloc>, _Hash2, _Eq2>
    {
    private:
      template<typename... _Tp>
	using unordered_set = _GLIBCXX_STD_C::unordered_set<_Tp...>;
      template<typename... _Tp>
	using unordered_multiset = _GLIBCXX_STD_C::unordered_multiset<_Tp...>;

      friend unordered_set<_Val, _Hash1, _Eq1, _Alloc>;

      static auto&
      _S_get_table(unordered_set<_Val, _Hash2, _Eq2, _Alloc>& __set)
      { return __set._M_h; }

      static auto&
      _S_get_table(unordered_multiset<_Val, _Hash2, _Eq2, _Alloc>& __set)
      { return __set._M_h; }
    };

  // Allow std::unordered_multiset access to internals of compatible sets.
  template<typename _Val, typename _Hash1, typename _Eq1, typename _Alloc,
	   typename _Hash2, typename _Eq2>
    struct _Hash_merge_helper<
      _GLIBCXX_STD_C::unordered_multiset<_Val, _Hash1, _Eq1, _Alloc>,
      _Hash2, _Eq2>
    {
    private:
      template<typename... _Tp>
	using unordered_set = _GLIBCXX_STD_C::unordered_set<_Tp...>;
      template<typename... _Tp>
	using unordered_multiset = _GLIBCXX_STD_C::unordered_multiset<_Tp...>;

      friend unordered_multiset<_Val, _Hash1, _Eq1, _Alloc>;

      static auto&
      _S_get_table(unordered_set<_Val, _Hash2, _Eq2, _Alloc>& __set)
      { return __set._M_h; }

      static auto&
      _S_get_table(unordered_multiset<_Val, _Hash2, _Eq2, _Alloc>& __set)
      { return __set._M_h; }
    };
#endif // C++17

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif /* _UNORDERED_SET_H */
