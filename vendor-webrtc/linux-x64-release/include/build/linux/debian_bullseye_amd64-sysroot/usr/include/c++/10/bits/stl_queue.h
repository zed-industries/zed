// Queue implementation -*- C++ -*-

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

/** @file bits/stl_queue.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{queue}
 */

#ifndef _STL_QUEUE_H
#define _STL_QUEUE_H 1

#include <bits/concept_check.h>
#include <debug/debug.h>
#if __cplusplus >= 201103L
# include <bits/uses_allocator.h>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief  A standard container giving FIFO behavior.
   *
   *  @ingroup sequences
   *
   *  @tparam _Tp  Type of element.
   *  @tparam _Sequence  Type of underlying sequence, defaults to deque<_Tp>.
   *
   *  Meets many of the requirements of a
   *  <a href="tables.html#65">container</a>,
   *  but does not define anything to do with iterators.  Very few of the
   *  other standard container interfaces are defined.
   *
   *  This is not a true container, but an @e adaptor.  It holds another
   *  container, and provides a wrapper interface to that container.  The
   *  wrapper is what enforces strict first-in-first-out %queue behavior.
   *
   *  The second template parameter defines the type of the underlying
   *  sequence/container.  It defaults to std::deque, but it can be any type
   *  that supports @c front, @c back, @c push_back, and @c pop_front,
   *  such as std::list or an appropriate user-defined type.
   *
   *  Members not found in @a normal containers are @c container_type,
   *  which is a typedef for the second Sequence parameter, and @c push and
   *  @c pop, which are standard %queue/FIFO operations.
  */
  template<typename _Tp, typename _Sequence = deque<_Tp> >
    class queue
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      // concept requirements
      typedef typename _Sequence::value_type _Sequence_value_type;
# if __cplusplus < 201103L
      __glibcxx_class_requires(_Tp, _SGIAssignableConcept)
# endif
      __glibcxx_class_requires(_Sequence, _FrontInsertionSequenceConcept)
      __glibcxx_class_requires(_Sequence, _BackInsertionSequenceConcept)
      __glibcxx_class_requires2(_Tp, _Sequence_value_type, _SameTypeConcept)
#endif

      template<typename _Tp1, typename _Seq1>
	friend bool
	operator==(const queue<_Tp1, _Seq1>&, const queue<_Tp1, _Seq1>&);

      template<typename _Tp1, typename _Seq1>
	friend bool
	operator<(const queue<_Tp1, _Seq1>&, const queue<_Tp1, _Seq1>&);

#if __cpp_lib_three_way_comparison
      template<typename _Tp1, three_way_comparable _Seq1>
	friend compare_three_way_result_t<_Seq1>
	operator<=>(const queue<_Tp1, _Seq1>&, const queue<_Tp1, _Seq1>&);
#endif

#if __cplusplus >= 201103L
      template<typename _Alloc>
	using _Uses = typename
	  enable_if<uses_allocator<_Sequence, _Alloc>::value>::type;

#if __cplusplus >= 201703L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2566. Requirements on the first template parameter of container
      // adaptors
      static_assert(is_same<_Tp, typename _Sequence::value_type>::value,
	  "value_type must be the same as the underlying container");
#endif // C++17
#endif // C++11

    public:
      typedef typename	_Sequence::value_type		value_type;
      typedef typename	_Sequence::reference		reference;
      typedef typename	_Sequence::const_reference	const_reference;
      typedef typename	_Sequence::size_type		size_type;
      typedef		_Sequence			container_type;

    protected:
      /*  Maintainers wondering why this isn't uglified as per style
       *  guidelines should note that this name is specified in the standard,
       *  C++98 [23.2.3.1].
       *  (Why? Presumably for the same reason that it's protected instead
       *  of private: to allow derivation.  But none of the other
       *  containers allow for derivation.  Odd.)
       */
       ///  @c c is the underlying container.
      _Sequence c;

    public:
      /**
       *  @brief  Default constructor creates no elements.
       */
#if __cplusplus < 201103L
      explicit
      queue(const _Sequence& __c = _Sequence())
      : c(__c) { }
#else
      template<typename _Seq = _Sequence, typename _Requires = typename
	       enable_if<is_default_constructible<_Seq>::value>::type>
	queue()
	: c() { }

      explicit
      queue(const _Sequence& __c)
      : c(__c) { }

      explicit
      queue(_Sequence&& __c)
      : c(std::move(__c)) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	explicit
	queue(const _Alloc& __a)
	: c(__a) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	queue(const _Sequence& __c, const _Alloc& __a)
	: c(__c, __a) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	queue(_Sequence&& __c, const _Alloc& __a)
	: c(std::move(__c), __a) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	queue(const queue& __q, const _Alloc& __a)
	: c(__q.c, __a) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	queue(queue&& __q, const _Alloc& __a)
	: c(std::move(__q.c), __a) { }
#endif

      /**
       *  Returns true if the %queue is empty.
       */
      _GLIBCXX_NODISCARD bool
      empty() const
      { return c.empty(); }

      /**  Returns the number of elements in the %queue.  */
      size_type
      size() const
      { return c.size(); }

      /**
       *  Returns a read/write reference to the data at the first
       *  element of the %queue.
       */
      reference
      front()
      {
	__glibcxx_requires_nonempty();
	return c.front();
      }

      /**
       *  Returns a read-only (constant) reference to the data at the first
       *  element of the %queue.
       */
      const_reference
      front() const
      {
	__glibcxx_requires_nonempty();
	return c.front();
      }

      /**
       *  Returns a read/write reference to the data at the last
       *  element of the %queue.
       */
      reference
      back()
      {
	__glibcxx_requires_nonempty();
	return c.back();
      }

      /**
       *  Returns a read-only (constant) reference to the data at the last
       *  element of the %queue.
       */
      const_reference
      back() const
      {
	__glibcxx_requires_nonempty();
	return c.back();
      }

      /**
       *  @brief  Add data to the end of the %queue.
       *  @param  __x  Data to be added.
       *
       *  This is a typical %queue operation.  The function creates an
       *  element at the end of the %queue and assigns the given data
       *  to it.  The time complexity of the operation depends on the
       *  underlying sequence.
       */
      void
      push(const value_type& __x)
      { c.push_back(__x); }

#if __cplusplus >= 201103L
      void
      push(value_type&& __x)
      { c.push_back(std::move(__x)); }

#if __cplusplus > 201402L
      template<typename... _Args>
	decltype(auto)
	emplace(_Args&&... __args)
	{ return c.emplace_back(std::forward<_Args>(__args)...); }
#else
      template<typename... _Args>
	void
	emplace(_Args&&... __args)
	{ c.emplace_back(std::forward<_Args>(__args)...); }
#endif
#endif

      /**
       *  @brief  Removes first element.
       *
       *  This is a typical %queue operation.  It shrinks the %queue by one.
       *  The time complexity of the operation depends on the underlying
       *  sequence.
       *
       *  Note that no data is returned, and if the first element's
       *  data is needed, it should be retrieved before pop() is
       *  called.
       */
      void
      pop()
      {
	__glibcxx_requires_nonempty();
	c.pop_front();
      }

#if __cplusplus >= 201103L
      void
      swap(queue& __q)
#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
      noexcept(__is_nothrow_swappable<_Sequence>::value)
#else
      noexcept(__is_nothrow_swappable<_Tp>::value)
#endif
      {
	using std::swap;
	swap(c, __q.c);
      }
#endif // __cplusplus >= 201103L
    };

#if __cpp_deduction_guides >= 201606
  template<typename _Container,
	   typename = _RequireNotAllocator<_Container>>
    queue(_Container) -> queue<typename _Container::value_type, _Container>;

  template<typename _Container, typename _Allocator,
	   typename = _RequireNotAllocator<_Container>,
	   typename = _RequireAllocator<_Allocator>>
    queue(_Container, _Allocator)
    -> queue<typename _Container::value_type, _Container>;
#endif

  /**
   *  @brief  Queue equality comparison.
   *  @param  __x  A %queue.
   *  @param  __y  A %queue of the same type as @a __x.
   *  @return  True iff the size and elements of the queues are equal.
   *
   *  This is an equivalence relation.  Complexity and semantics depend on the
   *  underlying sequence type, but the expected rules are:  this relation is
   *  linear in the size of the sequences, and queues are considered equivalent
   *  if their sequences compare equal.
  */
  template<typename _Tp, typename _Seq>
    inline bool
    operator==(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return __x.c == __y.c; }

  /**
   *  @brief  Queue ordering relation.
   *  @param  __x  A %queue.
   *  @param  __y  A %queue of the same type as @a x.
   *  @return  True iff @a __x is lexicographically less than @a __y.
   *
   *  This is an total ordering relation.  Complexity and semantics
   *  depend on the underlying sequence type, but the expected rules
   *  are: this relation is linear in the size of the sequences, the
   *  elements must be comparable with @c <, and
   *  std::lexicographical_compare() is usually used to make the
   *  determination.
  */
  template<typename _Tp, typename _Seq>
    inline bool
    operator<(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return __x.c < __y.c; }

  /// Based on operator==
  template<typename _Tp, typename _Seq>
    inline bool
    operator!=(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return !(__x == __y); }

  /// Based on operator<
  template<typename _Tp, typename _Seq>
    inline bool
    operator>(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return __y < __x; }

  /// Based on operator<
  template<typename _Tp, typename _Seq>
    inline bool
    operator<=(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return !(__y < __x); }

  /// Based on operator<
  template<typename _Tp, typename _Seq>
    inline bool
    operator>=(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return !(__x < __y); }

#if __cpp_lib_three_way_comparison
  template<typename _Tp, three_way_comparable _Seq>
    inline compare_three_way_result_t<_Seq>
    operator<=>(const queue<_Tp, _Seq>& __x, const queue<_Tp, _Seq>& __y)
    { return __x.c <=> __y.c; }
#endif

#if __cplusplus >= 201103L
  template<typename _Tp, typename _Seq>
    inline
#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
    // Constrained free swap overload, see p0185r1
    typename enable_if<__is_swappable<_Seq>::value>::type
#else
    void
#endif
    swap(queue<_Tp, _Seq>& __x, queue<_Tp, _Seq>& __y)
    noexcept(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

  template<typename _Tp, typename _Seq, typename _Alloc>
    struct uses_allocator<queue<_Tp, _Seq>, _Alloc>
    : public uses_allocator<_Seq, _Alloc>::type { };
#endif // __cplusplus >= 201103L

  /**
   *  @brief  A standard container automatically sorting its contents.
   *
   *  @ingroup sequences
   *
   *  @tparam _Tp  Type of element.
   *  @tparam _Sequence  Type of underlying sequence, defaults to vector<_Tp>.
   *  @tparam _Compare  Comparison function object type, defaults to
   *                    less<_Sequence::value_type>.
   *
   *  This is not a true container, but an @e adaptor.  It holds
   *  another container, and provides a wrapper interface to that
   *  container.  The wrapper is what enforces priority-based sorting
   *  and %queue behavior.  Very few of the standard container/sequence
   *  interface requirements are met (e.g., iterators).
   *
   *  The second template parameter defines the type of the underlying
   *  sequence/container.  It defaults to std::vector, but it can be
   *  any type that supports @c front(), @c push_back, @c pop_back,
   *  and random-access iterators, such as std::deque or an
   *  appropriate user-defined type.
   *
   *  The third template parameter supplies the means of making
   *  priority comparisons.  It defaults to @c less<value_type> but
   *  can be anything defining a strict weak ordering.
   *
   *  Members not found in @a normal containers are @c container_type,
   *  which is a typedef for the second Sequence parameter, and @c
   *  push, @c pop, and @c top, which are standard %queue operations.
   *
   *  @note No equality/comparison operators are provided for
   *  %priority_queue.
   *
   *  @note Sorting of the elements takes place as they are added to,
   *  and removed from, the %priority_queue using the
   *  %priority_queue's member functions.  If you access the elements
   *  by other means, and change their data such that the sorting
   *  order would be different, the %priority_queue will not re-sort
   *  the elements for you.  (How could it know to do so?)
  */
  template<typename _Tp, typename _Sequence = vector<_Tp>,
	   typename _Compare  = less<typename _Sequence::value_type> >
    class priority_queue
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      // concept requirements
      typedef typename _Sequence::value_type _Sequence_value_type;
# if __cplusplus < 201103L
      __glibcxx_class_requires(_Tp, _SGIAssignableConcept)
# endif
      __glibcxx_class_requires(_Sequence, _SequenceConcept)
      __glibcxx_class_requires(_Sequence, _RandomAccessContainerConcept)
      __glibcxx_class_requires2(_Tp, _Sequence_value_type, _SameTypeConcept)
      __glibcxx_class_requires4(_Compare, bool, _Tp, _Tp,
				_BinaryFunctionConcept)
#endif

#if __cplusplus >= 201103L
      template<typename _Alloc>
	using _Uses = typename
	  enable_if<uses_allocator<_Sequence, _Alloc>::value>::type;

#if __cplusplus >= 201703L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2566. Requirements on the first template parameter of container
      // adaptors
      static_assert(is_same<_Tp, typename _Sequence::value_type>::value,
	  "value_type must be the same as the underlying container");
#endif // C++17
#endif // C++11

    public:
      typedef typename	_Sequence::value_type		value_type;
      typedef typename	_Sequence::reference		reference;
      typedef typename	_Sequence::const_reference	const_reference;
      typedef typename	_Sequence::size_type		size_type;
      typedef		_Sequence			container_type;
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 2684. priority_queue lacking comparator typedef
      typedef	       _Compare				value_compare;

    protected:
      //  See queue::c for notes on these names.
      _Sequence  c;
      _Compare   comp;

    public:
      /**
       *  @brief  Default constructor creates no elements.
       */
#if __cplusplus < 201103L
      explicit
      priority_queue(const _Compare& __x = _Compare(),
		     const _Sequence& __s = _Sequence())
      : c(__s), comp(__x)
      { std::make_heap(c.begin(), c.end(), comp); }
#else
      template<typename _Seq = _Sequence, typename _Requires = typename
	       enable_if<__and_<is_default_constructible<_Compare>,
				is_default_constructible<_Seq>>::value>::type>
	priority_queue()
	: c(), comp() { }

      explicit
      priority_queue(const _Compare& __x, const _Sequence& __s)
      : c(__s), comp(__x)
      { std::make_heap(c.begin(), c.end(), comp); }

      explicit
      priority_queue(const _Compare& __x, _Sequence&& __s = _Sequence())
      : c(std::move(__s)), comp(__x)
      { std::make_heap(c.begin(), c.end(), comp); }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	explicit
	priority_queue(const _Alloc& __a)
	: c(__a), comp() { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	priority_queue(const _Compare& __x, const _Alloc& __a)
	: c(__a), comp(__x) { }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2537. Constructors [...] taking allocators should call make_heap
      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	priority_queue(const _Compare& __x, const _Sequence& __c,
		       const _Alloc& __a)
	: c(__c, __a), comp(__x)
	{ std::make_heap(c.begin(), c.end(), comp); }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	priority_queue(const _Compare& __x, _Sequence&& __c, const _Alloc& __a)
	: c(std::move(__c), __a), comp(__x)
	{ std::make_heap(c.begin(), c.end(), comp); }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	priority_queue(const priority_queue& __q, const _Alloc& __a)
	: c(__q.c, __a), comp(__q.comp) { }

      template<typename _Alloc, typename _Requires = _Uses<_Alloc>>
	priority_queue(priority_queue&& __q, const _Alloc& __a)
	: c(std::move(__q.c), __a), comp(std::move(__q.comp)) { }
#endif

      /**
       *  @brief  Builds a %queue from a range.
       *  @param  __first  An input iterator.
       *  @param  __last  An input iterator.
       *  @param  __x  A comparison functor describing a strict weak ordering.
       *  @param  __s  An initial sequence with which to start.
       *
       *  Begins by copying @a __s, inserting a copy of the elements
       *  from @a [first,last) into the copy of @a __s, then ordering
       *  the copy according to @a __x.
       *
       *  For more information on function objects, see the
       *  documentation on @link functors functor base
       *  classes@endlink.
       */
#if __cplusplus < 201103L
      template<typename _InputIterator>
	priority_queue(_InputIterator __first, _InputIterator __last,
		       const _Compare& __x = _Compare(),
		       const _Sequence& __s = _Sequence())
	: c(__s), comp(__x)
	{
	  __glibcxx_requires_valid_range(__first, __last);
	  c.insert(c.end(), __first, __last);
	  std::make_heap(c.begin(), c.end(), comp);
	}
#else
      template<typename _InputIterator>
	priority_queue(_InputIterator __first, _InputIterator __last,
		       const _Compare& __x,
		       const _Sequence& __s)
	: c(__s), comp(__x)
	{
	  __glibcxx_requires_valid_range(__first, __last);
	  c.insert(c.end(), __first, __last);
	  std::make_heap(c.begin(), c.end(), comp);
	}

      template<typename _InputIterator>
	priority_queue(_InputIterator __first, _InputIterator __last,
		       const _Compare& __x = _Compare(),
		       _Sequence&& __s = _Sequence())
	: c(std::move(__s)), comp(__x)
	{
	  __glibcxx_requires_valid_range(__first, __last);
	  c.insert(c.end(), __first, __last);
	  std::make_heap(c.begin(), c.end(), comp);
	}
#endif

      /**
       *  Returns true if the %queue is empty.
       */
      _GLIBCXX_NODISCARD bool
      empty() const
      { return c.empty(); }

      /**  Returns the number of elements in the %queue.  */
      size_type
      size() const
      { return c.size(); }

      /**
       *  Returns a read-only (constant) reference to the data at the first
       *  element of the %queue.
       */
      const_reference
      top() const
      {
	__glibcxx_requires_nonempty();
	return c.front();
      }

      /**
       *  @brief  Add data to the %queue.
       *  @param  __x  Data to be added.
       *
       *  This is a typical %queue operation.
       *  The time complexity of the operation depends on the underlying
       *  sequence.
       */
      void
      push(const value_type& __x)
      {
	c.push_back(__x);
	std::push_heap(c.begin(), c.end(), comp);
      }

#if __cplusplus >= 201103L
      void
      push(value_type&& __x)
      {
	c.push_back(std::move(__x));
	std::push_heap(c.begin(), c.end(), comp);
      }

      template<typename... _Args>
	void
	emplace(_Args&&... __args)
	{
	  c.emplace_back(std::forward<_Args>(__args)...);
	  std::push_heap(c.begin(), c.end(), comp);
	}
#endif

      /**
       *  @brief  Removes first element.
       *
       *  This is a typical %queue operation.  It shrinks the %queue
       *  by one.  The time complexity of the operation depends on the
       *  underlying sequence.
       *
       *  Note that no data is returned, and if the first element's
       *  data is needed, it should be retrieved before pop() is
       *  called.
       */
      void
      pop()
      {
	__glibcxx_requires_nonempty();
	std::pop_heap(c.begin(), c.end(), comp);
	c.pop_back();
      }

#if __cplusplus >= 201103L
      void
      swap(priority_queue& __pq)
      noexcept(__and_<
#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
		 __is_nothrow_swappable<_Sequence>,
#else
		 __is_nothrow_swappable<_Tp>,
#endif
		 __is_nothrow_swappable<_Compare>
	       >::value)
      {
	using std::swap;
	swap(c, __pq.c);
	swap(comp, __pq.comp);
      }
#endif // __cplusplus >= 201103L
    };

#if __cpp_deduction_guides >= 201606
  template<typename _Compare, typename _Container,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireNotAllocator<_Container>>
    priority_queue(_Compare, _Container)
    -> priority_queue<typename _Container::value_type, _Container, _Compare>;

  template<typename _InputIterator, typename _ValT
	   = typename iterator_traits<_InputIterator>::value_type,
	   typename _Compare = less<_ValT>,
	   typename _Container = vector<_ValT>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireNotAllocator<_Container>>
    priority_queue(_InputIterator, _InputIterator, _Compare = _Compare(),
		   _Container = _Container())
    -> priority_queue<_ValT, _Container, _Compare>;

  template<typename _Compare, typename _Container, typename _Allocator,
	   typename = _RequireNotAllocator<_Compare>,
	   typename = _RequireNotAllocator<_Container>,
	   typename = _RequireAllocator<_Allocator>>
    priority_queue(_Compare, _Container, _Allocator)
    -> priority_queue<typename _Container::value_type, _Container, _Compare>;
#endif

  // No equality/comparison operators are provided for priority_queue.

#if __cplusplus >= 201103L
  template<typename _Tp, typename _Sequence, typename _Compare>
    inline
#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
    // Constrained free swap overload, see p0185r1
    typename enable_if<__and_<__is_swappable<_Sequence>,
			      __is_swappable<_Compare>>::value>::type
#else
    void
#endif
    swap(priority_queue<_Tp, _Sequence, _Compare>& __x,
	 priority_queue<_Tp, _Sequence, _Compare>& __y)
    noexcept(noexcept(__x.swap(__y)))
    { __x.swap(__y); }

  template<typename _Tp, typename _Sequence, typename _Compare,
	   typename _Alloc>
    struct uses_allocator<priority_queue<_Tp, _Sequence, _Compare>, _Alloc>
    : public uses_allocator<_Sequence, _Alloc>::type { };
#endif // __cplusplus >= 201103L

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _STL_QUEUE_H */
