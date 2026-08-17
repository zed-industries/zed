// <forward_list.h> -*- C++ -*-

// Copyright (C) 2008-2020 Free Software Foundation, Inc.
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

/** @file bits/forward_list.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{forward_list}
 */

#ifndef _FORWARD_LIST_H
#define _FORWARD_LIST_H 1

#pragma GCC system_header

#include <initializer_list>
#include <bits/stl_iterator_base_types.h>
#include <bits/stl_iterator.h>
#include <bits/stl_algobase.h>
#include <bits/stl_function.h>
#include <bits/allocator.h>
#include <ext/alloc_traits.h>
#include <ext/aligned_buffer.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CONTAINER

  /**
   *  @brief  A helper basic node class for %forward_list.
   *          This is just a linked list with nothing inside it.
   *          There are purely list shuffling utility methods here.
   */
  struct _Fwd_list_node_base
  {
    _Fwd_list_node_base() = default;
    _Fwd_list_node_base(_Fwd_list_node_base&& __x) noexcept
      : _M_next(__x._M_next)
    { __x._M_next = nullptr; }

    _Fwd_list_node_base(const _Fwd_list_node_base&) = delete;
    _Fwd_list_node_base& operator=(const _Fwd_list_node_base&) = delete;

    _Fwd_list_node_base&
    operator=(_Fwd_list_node_base&& __x) noexcept
    {
      _M_next = __x._M_next;
      __x._M_next = nullptr;
      return *this;
    }

    _Fwd_list_node_base* _M_next = nullptr;

    _Fwd_list_node_base*
    _M_transfer_after(_Fwd_list_node_base* __begin,
		      _Fwd_list_node_base* __end) noexcept
    {
      _Fwd_list_node_base* __keep = __begin->_M_next;
      if (__end)
	{
	  __begin->_M_next = __end->_M_next;
	  __end->_M_next = _M_next;
	}
      else
	__begin->_M_next = nullptr;
      _M_next = __keep;
      return __end;
    }

    void
    _M_reverse_after() noexcept
    {
      _Fwd_list_node_base* __tail = _M_next;
      if (!__tail)
	return;
      while (_Fwd_list_node_base* __temp = __tail->_M_next)
	{
	  _Fwd_list_node_base* __keep = _M_next;
	  _M_next = __temp;
	  __tail->_M_next = __temp->_M_next;
	  _M_next->_M_next = __keep;
	}
    }
  };

  /**
   *  @brief  A helper node class for %forward_list.
   *          This is just a linked list with uninitialized storage for a
   *          data value in each node.
   *          There is a sorting utility method.
   */
  template<typename _Tp>
    struct _Fwd_list_node
    : public _Fwd_list_node_base
    {
      _Fwd_list_node() = default;

      __gnu_cxx::__aligned_buffer<_Tp> _M_storage;

      _Tp*
      _M_valptr() noexcept
      { return _M_storage._M_ptr(); }

      const _Tp*
      _M_valptr() const noexcept
      { return _M_storage._M_ptr(); }
    };

  /**
   *   @brief A forward_list::iterator.
   *
   *   All the functions are op overloads.
   */
  template<typename _Tp>
    struct _Fwd_list_iterator
    {
      typedef _Fwd_list_iterator<_Tp>		_Self;
      typedef _Fwd_list_node<_Tp>		_Node;

      typedef _Tp				value_type;
      typedef _Tp*				pointer;
      typedef _Tp&				reference;
      typedef ptrdiff_t				difference_type;
      typedef std::forward_iterator_tag		iterator_category;

      _Fwd_list_iterator() noexcept
      : _M_node() { }

      explicit
      _Fwd_list_iterator(_Fwd_list_node_base* __n) noexcept
      : _M_node(__n) { }

      reference
      operator*() const noexcept
      { return *static_cast<_Node*>(this->_M_node)->_M_valptr(); }

      pointer
      operator->() const noexcept
      { return static_cast<_Node*>(this->_M_node)->_M_valptr(); }

      _Self&
      operator++() noexcept
      {
	_M_node = _M_node->_M_next;
	return *this;
      }

      _Self
      operator++(int) noexcept
      {
	_Self __tmp(*this);
	_M_node = _M_node->_M_next;
	return __tmp;
      }

      /**
       *  @brief  Forward list iterator equality comparison.
       */
      friend bool
      operator==(const _Self& __x, const _Self& __y) noexcept
      { return __x._M_node == __y._M_node; }

#if __cpp_impl_three_way_comparison < 201907L
      /**
       *  @brief  Forward list iterator inequality comparison.
       */
      friend bool
      operator!=(const _Self& __x, const _Self& __y) noexcept
      { return __x._M_node != __y._M_node; }
#endif

      _Self
      _M_next() const noexcept
      {
	if (_M_node)
	  return _Fwd_list_iterator(_M_node->_M_next);
	else
	  return _Fwd_list_iterator(nullptr);
      }

      _Fwd_list_node_base* _M_node;
    };

  /**
   *   @brief A forward_list::const_iterator.
   *
   *   All the functions are op overloads.
   */
  template<typename _Tp>
    struct _Fwd_list_const_iterator
    {
      typedef _Fwd_list_const_iterator<_Tp>	_Self;
      typedef const _Fwd_list_node<_Tp>		_Node;
      typedef _Fwd_list_iterator<_Tp>		iterator;

      typedef _Tp				value_type;
      typedef const _Tp*			pointer;
      typedef const _Tp&			reference;
      typedef ptrdiff_t				difference_type;
      typedef std::forward_iterator_tag		iterator_category;

      _Fwd_list_const_iterator() noexcept
      : _M_node() { }

      explicit
      _Fwd_list_const_iterator(const _Fwd_list_node_base* __n)  noexcept
      : _M_node(__n) { }

      _Fwd_list_const_iterator(const iterator& __iter) noexcept
      : _M_node(__iter._M_node) { }

      reference
      operator*() const noexcept
      { return *static_cast<_Node*>(this->_M_node)->_M_valptr(); }

      pointer
      operator->() const noexcept
      { return static_cast<_Node*>(this->_M_node)->_M_valptr(); }

      _Self&
      operator++() noexcept
      {
	_M_node = _M_node->_M_next;
	return *this;
      }

      _Self
      operator++(int) noexcept
      {
	_Self __tmp(*this);
	_M_node = _M_node->_M_next;
	return __tmp;
      }

      /**
       *  @brief  Forward list const_iterator equality comparison.
       */
      friend bool
      operator==(const _Self& __x, const _Self& __y) noexcept
      { return __x._M_node == __y._M_node; }

#if __cpp_impl_three_way_comparison < 201907L
      /**
       *  @brief  Forward list const_iterator inequality comparison.
       */
      friend bool
      operator!=(const _Self& __x, const _Self& __y) noexcept
      { return __x._M_node != __y._M_node; }
#endif

      _Self
      _M_next() const noexcept
      {
	if (this->_M_node)
	  return _Fwd_list_const_iterator(_M_node->_M_next);
	else
	  return _Fwd_list_const_iterator(nullptr);
      }

      const _Fwd_list_node_base* _M_node;
    };

  /**
   *  @brief  Base class for %forward_list.
   */
  template<typename _Tp, typename _Alloc>
    struct _Fwd_list_base
    {
    protected:
      typedef __alloc_rebind<_Alloc, _Fwd_list_node<_Tp>> _Node_alloc_type;
      typedef __gnu_cxx::__alloc_traits<_Node_alloc_type> _Node_alloc_traits;

      struct _Fwd_list_impl
      : public _Node_alloc_type
      {
	_Fwd_list_node_base _M_head;

	_Fwd_list_impl()
	  noexcept(is_nothrow_default_constructible<_Node_alloc_type>::value)
	: _Node_alloc_type(), _M_head()
	{ }

	_Fwd_list_impl(_Fwd_list_impl&&) = default;

	_Fwd_list_impl(_Fwd_list_impl&& __fl, _Node_alloc_type&& __a)
	: _Node_alloc_type(std::move(__a)), _M_head(std::move(__fl._M_head))
	{ }

	_Fwd_list_impl(_Node_alloc_type&& __a)
	: _Node_alloc_type(std::move(__a)), _M_head()
	{ }
      };

      _Fwd_list_impl _M_impl;

    public:
      typedef _Fwd_list_iterator<_Tp>		iterator;
      typedef _Fwd_list_const_iterator<_Tp>	const_iterator;
      typedef _Fwd_list_node<_Tp>		_Node;

      _Node_alloc_type&
      _M_get_Node_allocator() noexcept
      { return this->_M_impl; }

      const _Node_alloc_type&
      _M_get_Node_allocator() const noexcept
      { return this->_M_impl; }

      _Fwd_list_base() = default;

      _Fwd_list_base(_Node_alloc_type&& __a)
      : _M_impl(std::move(__a)) { }

      // When allocators are always equal.
      _Fwd_list_base(_Fwd_list_base&& __lst, _Node_alloc_type&& __a,
		     std::true_type)
      : _M_impl(std::move(__lst._M_impl), std::move(__a))
      { }

      // When allocators are not always equal.
      _Fwd_list_base(_Fwd_list_base&& __lst, _Node_alloc_type&& __a);

      _Fwd_list_base(_Fwd_list_base&&) = default;

      ~_Fwd_list_base()
      { _M_erase_after(&_M_impl._M_head, nullptr); }

    protected:
      _Node*
      _M_get_node()
      {
	auto __ptr = _Node_alloc_traits::allocate(_M_get_Node_allocator(), 1);
	return std::__to_address(__ptr);
      }

      template<typename... _Args>
	_Node*
	_M_create_node(_Args&&... __args)
	{
	  _Node* __node = this->_M_get_node();
	  __try
	    {
	      ::new ((void*)__node) _Node;
	      _Node_alloc_traits::construct(_M_get_Node_allocator(),
					    __node->_M_valptr(),
					    std::forward<_Args>(__args)...);
	    }
	  __catch(...)
	    {
	      this->_M_put_node(__node);
	      __throw_exception_again;
	    }
	  return __node;
	}

      template<typename... _Args>
	_Fwd_list_node_base*
	_M_insert_after(const_iterator __pos, _Args&&... __args);

      void
      _M_put_node(_Node* __p)
      {
	typedef typename _Node_alloc_traits::pointer _Ptr;
	auto __ptr = std::pointer_traits<_Ptr>::pointer_to(*__p);
	_Node_alloc_traits::deallocate(_M_get_Node_allocator(), __ptr, 1);
      }

      _Fwd_list_node_base*
      _M_erase_after(_Fwd_list_node_base* __pos);

      _Fwd_list_node_base*
      _M_erase_after(_Fwd_list_node_base* __pos,
		     _Fwd_list_node_base* __last);
    };

  /**
   *  @brief A standard container with linear time access to elements,
   *  and fixed time insertion/deletion at any point in the sequence.
   *
   *  @ingroup sequences
   *
   *  @tparam _Tp  Type of element.
   *  @tparam _Alloc  Allocator type, defaults to allocator<_Tp>.
   *
   *  Meets the requirements of a <a href="tables.html#65">container</a>, a
   *  <a href="tables.html#67">sequence</a>, including the
   *  <a href="tables.html#68">optional sequence requirements</a> with the
   *  %exception of @c at and @c operator[].
   *
   *  This is a @e singly @e linked %list.  Traversal up the
   *  %list requires linear time, but adding and removing elements (or
   *  @e nodes) is done in constant time, regardless of where the
   *  change takes place.  Unlike std::vector and std::deque,
   *  random-access iterators are not provided, so subscripting ( @c
   *  [] ) access is not allowed.  For algorithms which only need
   *  sequential access, this lack makes no difference.
   *
   *  Also unlike the other standard containers, std::forward_list provides
   *  specialized algorithms %unique to linked lists, such as
   *  splicing, sorting, and in-place reversal.
   */
  template<typename _Tp, typename _Alloc = allocator<_Tp>>
    class forward_list : private _Fwd_list_base<_Tp, _Alloc>
    {
      static_assert(is_same<typename remove_cv<_Tp>::type, _Tp>::value,
	  "std::forward_list must have a non-const, non-volatile value_type");
#if __cplusplus > 201703L || defined __STRICT_ANSI__
      static_assert(is_same<typename _Alloc::value_type, _Tp>::value,
	  "std::forward_list must have the same value_type as its allocator");
#endif

    private:
      typedef _Fwd_list_base<_Tp, _Alloc>		_Base;
      typedef _Fwd_list_node_base			_Node_base;
      typedef typename _Base::_Node			_Node;
      typedef typename _Base::_Node_alloc_type		_Node_alloc_type;
      typedef typename _Base::_Node_alloc_traits	_Node_alloc_traits;
      typedef allocator_traits<__alloc_rebind<_Alloc, _Tp>>	_Alloc_traits;

    public:
      // types:
      typedef _Tp					value_type;
      typedef typename _Alloc_traits::pointer		pointer;
      typedef typename _Alloc_traits::const_pointer	const_pointer;
      typedef value_type&				reference;
      typedef const value_type&				const_reference;

      typedef typename _Base::iterator			iterator;
      typedef typename _Base::const_iterator		const_iterator;
      typedef std::size_t				size_type;
      typedef std::ptrdiff_t				difference_type;
      typedef _Alloc					allocator_type;

      // 23.3.4.2 construct/copy/destroy:

      /**
       *  @brief  Creates a %forward_list with no elements.
       */
      forward_list() = default;

      /**
       *  @brief  Creates a %forward_list with no elements.
       *  @param  __al  An allocator object.
       */
      explicit
      forward_list(const _Alloc& __al) noexcept
      : _Base(_Node_alloc_type(__al))
      { }

      /**
       *  @brief  Copy constructor with allocator argument.
       *  @param  __list  Input list to copy.
       *  @param  __al    An allocator object.
       */
      forward_list(const forward_list& __list, const _Alloc& __al)
      : _Base(_Node_alloc_type(__al))
      { _M_range_initialize(__list.begin(), __list.end()); }

    private:
      forward_list(forward_list&& __list, _Node_alloc_type&& __al,
		   false_type)
      : _Base(std::move(__list), std::move(__al))
      {
	// If __list is not empty it means its allocator is not equal to __a,
	// so we need to move from each element individually.
	insert_after(cbefore_begin(),
		     std::__make_move_if_noexcept_iterator(__list.begin()),
		     std::__make_move_if_noexcept_iterator(__list.end()));
      }

      forward_list(forward_list&& __list, _Node_alloc_type&& __al,
		   true_type)
      noexcept
      : _Base(std::move(__list), _Node_alloc_type(__al), true_type{})
      { }

    public:
      /**
       *  @brief  Move constructor with allocator argument.
       *  @param  __list  Input list to move.
       *  @param  __al    An allocator object.
       */
      forward_list(forward_list&& __list, const _Alloc& __al)
      noexcept(_Node_alloc_traits::_S_always_equal())
      : forward_list(std::move(__list), _Node_alloc_type(__al),
		     typename _Node_alloc_traits::is_always_equal{})
      { }

      /**
       *  @brief  Creates a %forward_list with default constructed elements.
       *  @param  __n   The number of elements to initially create.
       *  @param  __al  An allocator object.
       *
       *  This constructor creates the %forward_list with @a __n default
       *  constructed elements.
       */
      explicit
      forward_list(size_type __n, const _Alloc& __al = _Alloc())
      : _Base(_Node_alloc_type(__al))
      { _M_default_initialize(__n); }

      /**
       *  @brief  Creates a %forward_list with copies of an exemplar element.
       *  @param  __n      The number of elements to initially create.
       *  @param  __value  An element to copy.
       *  @param  __al     An allocator object.
       *
       *  This constructor fills the %forward_list with @a __n copies of
       *  @a __value.
       */
      forward_list(size_type __n, const _Tp& __value,
		   const _Alloc& __al = _Alloc())
      : _Base(_Node_alloc_type(__al))
      { _M_fill_initialize(__n, __value); }

      /**
       *  @brief  Builds a %forward_list from a range.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *  @param  __al     An allocator object.
       *
       *  Create a %forward_list consisting of copies of the elements from
       *  [@a __first,@a __last).  This is linear in N (where N is
       *  distance(@a __first,@a __last)).
       */
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	forward_list(_InputIterator __first, _InputIterator __last,
		     const _Alloc& __al = _Alloc())
	: _Base(_Node_alloc_type(__al))
	{ _M_range_initialize(__first, __last); }

      /**
       *  @brief  The %forward_list copy constructor.
       *  @param  __list  A %forward_list of identical element and allocator
       *                  types.
       */
      forward_list(const forward_list& __list)
      : _Base(_Node_alloc_traits::_S_select_on_copy(
		__list._M_get_Node_allocator()))
      { _M_range_initialize(__list.begin(), __list.end()); }

      /**
       *  @brief  The %forward_list move constructor.
       *  @param  __list  A %forward_list of identical element and allocator
       *                  types.
       *
       *  The newly-created %forward_list contains the exact contents of the
       *  moved instance. The contents of the moved instance are a valid, but
       *  unspecified %forward_list.
       */
      forward_list(forward_list&&) = default;

      /**
       *  @brief  Builds a %forward_list from an initializer_list
       *  @param  __il  An initializer_list of value_type.
       *  @param  __al  An allocator object.
       *
       *  Create a %forward_list consisting of copies of the elements
       *  in the initializer_list @a __il.  This is linear in __il.size().
       */
      forward_list(std::initializer_list<_Tp> __il,
		   const _Alloc& __al = _Alloc())
      : _Base(_Node_alloc_type(__al))
      { _M_range_initialize(__il.begin(), __il.end()); }

      /**
       *  @brief  The forward_list dtor.
       */
      ~forward_list() noexcept
      { }

      /**
       *  @brief  The %forward_list assignment operator.
       *  @param  __list  A %forward_list of identical element and allocator
       *                types.
       *
       *  All the elements of @a __list are copied.
       *
       *  Whether the allocator is copied depends on the allocator traits.
       */
      forward_list&
      operator=(const forward_list& __list);

      /**
       *  @brief  The %forward_list move assignment operator.
       *  @param  __list  A %forward_list of identical element and allocator
       *                types.
       *
       *  The contents of @a __list are moved into this %forward_list
       *  (without copying, if the allocators permit it).
       *
       *  Afterwards @a __list is a valid, but unspecified %forward_list
       *
       *  Whether the allocator is moved depends on the allocator traits.
       */
      forward_list&
      operator=(forward_list&& __list)
      noexcept(_Node_alloc_traits::_S_nothrow_move())
      {
	constexpr bool __move_storage =
	  _Node_alloc_traits::_S_propagate_on_move_assign()
	  || _Node_alloc_traits::_S_always_equal();
	_M_move_assign(std::move(__list), __bool_constant<__move_storage>());
	return *this;
      }

      /**
       *  @brief  The %forward_list initializer list assignment operator.
       *  @param  __il  An initializer_list of value_type.
       *
       *  Replace the contents of the %forward_list with copies of the
       *  elements in the initializer_list @a __il.  This is linear in
       *  __il.size().
       */
      forward_list&
      operator=(std::initializer_list<_Tp> __il)
      {
	assign(__il);
	return *this;
      }

      /**
       *  @brief  Assigns a range to a %forward_list.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *
       *  This function fills a %forward_list with copies of the elements
       *  in the range [@a __first,@a __last).
       *
       *  Note that the assignment completely changes the %forward_list and
       *  that the number of elements of the resulting %forward_list is the
       *  same as the number of elements assigned.
       */
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	void
	assign(_InputIterator __first, _InputIterator __last)
	{
	  typedef is_assignable<_Tp, decltype(*__first)> __assignable;
	  _M_assign(__first, __last, __assignable());
	}

      /**
       *  @brief  Assigns a given value to a %forward_list.
       *  @param  __n  Number of elements to be assigned.
       *  @param  __val  Value to be assigned.
       *
       *  This function fills a %forward_list with @a __n copies of the
       *  given value.  Note that the assignment completely changes the
       *  %forward_list, and that the resulting %forward_list has __n
       *  elements.
       */
      void
      assign(size_type __n, const _Tp& __val)
      { _M_assign_n(__n, __val, is_copy_assignable<_Tp>()); }

      /**
       *  @brief  Assigns an initializer_list to a %forward_list.
       *  @param  __il  An initializer_list of value_type.
       *
       *  Replace the contents of the %forward_list with copies of the
       *  elements in the initializer_list @a __il.  This is linear in
       *  il.size().
       */
      void
      assign(std::initializer_list<_Tp> __il)
      { assign(__il.begin(), __il.end()); }

      /// Get a copy of the memory allocation object.
      allocator_type
      get_allocator() const noexcept
      { return allocator_type(this->_M_get_Node_allocator()); }

      // 23.3.4.3 iterators:

      /**
       *  Returns a read/write iterator that points before the first element
       *  in the %forward_list.  Iteration is done in ordinary element order.
       */
      iterator
      before_begin() noexcept
      { return iterator(&this->_M_impl._M_head); }

      /**
       *  Returns a read-only (constant) iterator that points before the
       *  first element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      const_iterator
      before_begin() const noexcept
      { return const_iterator(&this->_M_impl._M_head); }

      /**
       *  Returns a read/write iterator that points to the first element
       *  in the %forward_list.  Iteration is done in ordinary element order.
       */
      iterator
      begin() noexcept
      { return iterator(this->_M_impl._M_head._M_next); }

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      const_iterator
      begin() const noexcept
      { return const_iterator(this->_M_impl._M_head._M_next); }

      /**
       *  Returns a read/write iterator that points one past the last
       *  element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      iterator
      end() noexcept
      { return iterator(nullptr); }

      /**
       *  Returns a read-only iterator that points one past the last
       *  element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      const_iterator
      end() const noexcept
      { return const_iterator(nullptr); }

      /**
       *  Returns a read-only (constant) iterator that points to the
       *  first element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      const_iterator
      cbegin() const noexcept
      { return const_iterator(this->_M_impl._M_head._M_next); }

      /**
       *  Returns a read-only (constant) iterator that points before the
       *  first element in the %forward_list.  Iteration is done in ordinary
       *  element order.
       */
      const_iterator
      cbefore_begin() const noexcept
      { return const_iterator(&this->_M_impl._M_head); }

      /**
       *  Returns a read-only (constant) iterator that points one past
       *  the last element in the %forward_list.  Iteration is done in
       *  ordinary element order.
       */
      const_iterator
      cend() const noexcept
      { return const_iterator(nullptr); }

      /**
       *  Returns true if the %forward_list is empty.  (Thus begin() would
       *  equal end().)
       */
      _GLIBCXX_NODISCARD bool
      empty() const noexcept
      { return this->_M_impl._M_head._M_next == nullptr; }

      /**
       *  Returns the largest possible number of elements of %forward_list.
       */
      size_type
      max_size() const noexcept
      { return _Node_alloc_traits::max_size(this->_M_get_Node_allocator()); }

      // 23.3.4.4 element access:

      /**
       *  Returns a read/write reference to the data at the first
       *  element of the %forward_list.
       */
      reference
      front()
      {
	_Node* __front = static_cast<_Node*>(this->_M_impl._M_head._M_next);
	return *__front->_M_valptr();
      }

      /**
       *  Returns a read-only (constant) reference to the data at the first
       *  element of the %forward_list.
       */
      const_reference
      front() const
      {
	_Node* __front = static_cast<_Node*>(this->_M_impl._M_head._M_next);
	return *__front->_M_valptr();
      }

      // 23.3.4.5 modifiers:

      /**
       *  @brief  Constructs object in %forward_list at the front of the
       *          list.
       *  @param  __args  Arguments.
       *
       *  This function will insert an object of type Tp constructed
       *  with Tp(std::forward<Args>(args)...) at the front of the list
       *  Due to the nature of a %forward_list this operation can
       *  be done in constant time, and does not invalidate iterators
       *  and references.
       */
      template<typename... _Args>
#if __cplusplus > 201402L
	reference
#else
	void
#endif
	emplace_front(_Args&&... __args)
	{
	  this->_M_insert_after(cbefore_begin(),
				std::forward<_Args>(__args)...);
#if __cplusplus > 201402L
	  return front();
#endif
	}

      /**
       *  @brief  Add data to the front of the %forward_list.
       *  @param  __val  Data to be added.
       *
       *  This is a typical stack operation.  The function creates an
       *  element at the front of the %forward_list and assigns the given
       *  data to it.  Due to the nature of a %forward_list this operation
       *  can be done in constant time, and does not invalidate iterators
       *  and references.
       */
      void
      push_front(const _Tp& __val)
      { this->_M_insert_after(cbefore_begin(), __val); }

      /**
       *
       */
      void
      push_front(_Tp&& __val)
      { this->_M_insert_after(cbefore_begin(), std::move(__val)); }

      /**
       *  @brief  Removes first element.
       *
       *  This is a typical stack operation.  It shrinks the %forward_list
       *  by one.  Due to the nature of a %forward_list this operation can
       *  be done in constant time, and only invalidates iterators/references
       *  to the element being removed.
       *
       *  Note that no data is returned, and if the first element's data
       *  is needed, it should be retrieved before pop_front() is
       *  called.
       */
      void
      pop_front()
      { this->_M_erase_after(&this->_M_impl._M_head); }

      /**
       *  @brief  Constructs object in %forward_list after the specified
       *          iterator.
       *  @param  __pos  A const_iterator into the %forward_list.
       *  @param  __args  Arguments.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert an object of type T constructed
       *  with T(std::forward<Args>(args)...) after the specified
       *  location.  Due to the nature of a %forward_list this operation can
       *  be done in constant time, and does not invalidate iterators
       *  and references.
       */
      template<typename... _Args>
	iterator
	emplace_after(const_iterator __pos, _Args&&... __args)
	{ return iterator(this->_M_insert_after(__pos,
					  std::forward<_Args>(__args)...)); }

      /**
       *  @brief  Inserts given value into %forward_list after specified
       *          iterator.
       *  @param  __pos  An iterator into the %forward_list.
       *  @param  __val  Data to be inserted.
       *  @return  An iterator that points to the inserted data.
       *
       *  This function will insert a copy of the given value after
       *  the specified location.  Due to the nature of a %forward_list this
       *  operation can be done in constant time, and does not
       *  invalidate iterators and references.
       */
      iterator
      insert_after(const_iterator __pos, const _Tp& __val)
      { return iterator(this->_M_insert_after(__pos, __val)); }

      /**
       *
       */
      iterator
      insert_after(const_iterator __pos, _Tp&& __val)
      { return iterator(this->_M_insert_after(__pos, std::move(__val))); }

      /**
       *  @brief  Inserts a number of copies of given data into the
       *          %forward_list.
       *  @param  __pos  An iterator into the %forward_list.
       *  @param  __n  Number of elements to be inserted.
       *  @param  __val  Data to be inserted.
       *  @return  An iterator pointing to the last inserted copy of
       *           @a val or @a pos if @a n == 0.
       *
       *  This function will insert a specified number of copies of the
       *  given data after the location specified by @a pos.
       *
       *  This operation is linear in the number of elements inserted and
       *  does not invalidate iterators and references.
       */
      iterator
      insert_after(const_iterator __pos, size_type __n, const _Tp& __val);

      /**
       *  @brief  Inserts a range into the %forward_list.
       *  @param  __pos  An iterator into the %forward_list.
       *  @param  __first  An input iterator.
       *  @param  __last   An input iterator.
       *  @return  An iterator pointing to the last inserted element or
       *           @a __pos if @a __first == @a __last.
       *
       *  This function will insert copies of the data in the range
       *  [@a __first,@a __last) into the %forward_list after the
       *  location specified by @a __pos.
       *
       *  This operation is linear in the number of elements inserted and
       *  does not invalidate iterators and references.
       */
      template<typename _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	iterator
	insert_after(const_iterator __pos,
		     _InputIterator __first, _InputIterator __last);

      /**
       *  @brief  Inserts the contents of an initializer_list into
       *          %forward_list after the specified iterator.
       *  @param  __pos  An iterator into the %forward_list.
       *  @param  __il  An initializer_list of value_type.
       *  @return  An iterator pointing to the last inserted element
       *           or @a __pos if @a __il is empty.
       *
       *  This function will insert copies of the data in the
       *  initializer_list @a __il into the %forward_list before the location
       *  specified by @a __pos.
       *
       *  This operation is linear in the number of elements inserted and
       *  does not invalidate iterators and references.
       */
      iterator
      insert_after(const_iterator __pos, std::initializer_list<_Tp> __il)
      { return insert_after(__pos, __il.begin(), __il.end()); }

      /**
       *  @brief  Removes the element pointed to by the iterator following
       *          @c pos.
       *  @param  __pos  Iterator pointing before element to be erased.
       *  @return  An iterator pointing to the element following the one
       *           that was erased, or end() if no such element exists.
       *
       *  This function will erase the element at the given position and
       *  thus shorten the %forward_list by one.
       *
       *  Due to the nature of a %forward_list this operation can be done
       *  in constant time, and only invalidates iterators/references to
       *  the element being removed.  The user is also cautioned that
       *  this function only erases the element, and that if the element
       *  is itself a pointer, the pointed-to memory is not touched in
       *  any way.  Managing the pointer is the user's responsibility.
       */
      iterator
      erase_after(const_iterator __pos)
      { return iterator(this->_M_erase_after(const_cast<_Node_base*>
					     (__pos._M_node))); }

      /**
       *  @brief  Remove a range of elements.
       *  @param  __pos  Iterator pointing before the first element to be
       *                 erased.
       *  @param  __last  Iterator pointing to one past the last element to be
       *                  erased.
       *  @return  @ __last.
       *
       *  This function will erase the elements in the range
       *  @a (__pos,__last) and shorten the %forward_list accordingly.
       *
       *  This operation is linear time in the size of the range and only
       *  invalidates iterators/references to the element being removed.
       *  The user is also cautioned that this function only erases the
       *  elements, and that if the elements themselves are pointers, the
       *  pointed-to memory is not touched in any way.  Managing the pointer
       *  is the user's responsibility.
       */
      iterator
      erase_after(const_iterator __pos, const_iterator __last)
      { return iterator(this->_M_erase_after(const_cast<_Node_base*>
					     (__pos._M_node),
					     const_cast<_Node_base*>
					     (__last._M_node))); }

      /**
       *  @brief  Swaps data with another %forward_list.
       *  @param  __list  A %forward_list of the same element and allocator
       *                  types.
       *
       *  This exchanges the elements between two lists in constant
       *  time.  Note that the global std::swap() function is
       *  specialized such that std::swap(l1,l2) will feed to this
       *  function.
       *
       *  Whether the allocators are swapped depends on the allocator traits.
       */
      void
      swap(forward_list& __list) noexcept
      {
	std::swap(this->_M_impl._M_head._M_next,
		  __list._M_impl._M_head._M_next);
	_Node_alloc_traits::_S_on_swap(this->_M_get_Node_allocator(),
				       __list._M_get_Node_allocator());
      }

      /**
       *  @brief Resizes the %forward_list to the specified number of
       *         elements.
       *  @param __sz Number of elements the %forward_list should contain.
       *
       *  This function will %resize the %forward_list to the specified
       *  number of elements.  If the number is smaller than the
       *  %forward_list's current number of elements the %forward_list
       *  is truncated, otherwise the %forward_list is extended and the
       *  new elements are default constructed.
       */
      void
      resize(size_type __sz);

      /**
       *  @brief Resizes the %forward_list to the specified number of
       *         elements.
       *  @param __sz Number of elements the %forward_list should contain.
       *  @param __val Data with which new elements should be populated.
       *
       *  This function will %resize the %forward_list to the specified
       *  number of elements.  If the number is smaller than the
       *  %forward_list's current number of elements the %forward_list
       *  is truncated, otherwise the %forward_list is extended and new
       *  elements are populated with given data.
       */
      void
      resize(size_type __sz, const value_type& __val);

      /**
       *  @brief  Erases all the elements.
       *
       *  Note that this function only erases
       *  the elements, and that if the elements themselves are
       *  pointers, the pointed-to memory is not touched in any way.
       *  Managing the pointer is the user's responsibility.
       */
      void
      clear() noexcept
      { this->_M_erase_after(&this->_M_impl._M_head, nullptr); }

      // 23.3.4.6 forward_list operations:

      /**
       *  @brief  Insert contents of another %forward_list.
       *  @param  __pos  Iterator referencing the element to insert after.
       *  @param  __list  Source list.
       *
       *  The elements of @a list are inserted in constant time after
       *  the element referenced by @a pos.  @a list becomes an empty
       *  list.
       *
       *  Requires this != @a x.
       */
      void
      splice_after(const_iterator __pos, forward_list&& __list) noexcept
      {
	if (!__list.empty())
	  _M_splice_after(__pos, __list.before_begin(), __list.end());
      }

      void
      splice_after(const_iterator __pos, forward_list& __list) noexcept
      { splice_after(__pos, std::move(__list)); }

      /**
       *  @brief  Insert element from another %forward_list.
       *  @param  __pos  Iterator referencing the element to insert after.
       *  @param  __list  Source list.
       *  @param  __i   Iterator referencing the element before the element
       *                to move.
       *
       *  Removes the element in list @a list referenced by @a i and
       *  inserts it into the current list after @a pos.
       */
      void
      splice_after(const_iterator __pos, forward_list&& __list,
		   const_iterator __i) noexcept;

      void
      splice_after(const_iterator __pos, forward_list& __list,
		   const_iterator __i) noexcept
      { splice_after(__pos, std::move(__list), __i); }

      /**
       *  @brief  Insert range from another %forward_list.
       *  @param  __pos  Iterator referencing the element to insert after.
       *  @param  __list  Source list.
       *  @param  __before  Iterator referencing before the start of range
       *                    in list.
       *  @param  __last  Iterator referencing the end of range in list.
       *
       *  Removes elements in the range (__before,__last) and inserts them
       *  after @a __pos in constant time.
       *
       *  Undefined if @a __pos is in (__before,__last).
       *  @{
       */
      void
      splice_after(const_iterator __pos, forward_list&&,
		   const_iterator __before, const_iterator __last) noexcept
      { _M_splice_after(__pos, __before, __last); }

      void
      splice_after(const_iterator __pos, forward_list&,
		   const_iterator __before, const_iterator __last) noexcept
      { _M_splice_after(__pos, __before, __last); }
      // @}

    private:
#if __cplusplus > 201703L
# define __cpp_lib_list_remove_return_type 201806L
      using __remove_return_type = size_type;
# define _GLIBCXX_FWDLIST_REMOVE_RETURN_TYPE_TAG \
      __attribute__((__abi_tag__("__cxx20")))
#else
      using __remove_return_type = void;
# define _GLIBCXX_FWDLIST_REMOVE_RETURN_TYPE_TAG
#endif
    public:

      /**
       *  @brief  Remove all elements equal to value.
       *  @param  __val  The value to remove.
       *
       *  Removes every element in the list equal to @a __val.
       *  Remaining elements stay in list order.  Note that this
       *  function only erases the elements, and that if the elements
       *  themselves are pointers, the pointed-to memory is not
       *  touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      _GLIBCXX_FWDLIST_REMOVE_RETURN_TYPE_TAG
      __remove_return_type
      remove(const _Tp& __val);

      /**
       *  @brief  Remove all elements satisfying a predicate.
       *  @param  __pred  Unary predicate function or object.
       *
       *  Removes every element in the list for which the predicate
       *  returns true.  Remaining elements stay in list order.  Note
       *  that this function only erases the elements, and that if the
       *  elements themselves are pointers, the pointed-to memory is
       *  not touched in any way.  Managing the pointer is the user's
       *  responsibility.
       */
      template<typename _Pred>
	__remove_return_type
	remove_if(_Pred __pred);

      /**
       *  @brief  Remove consecutive duplicate elements.
       *
       *  For each consecutive set of elements with the same value,
       *  remove all but the first one.  Remaining elements stay in
       *  list order.  Note that this function only erases the
       *  elements, and that if the elements themselves are pointers,
       *  the pointed-to memory is not touched in any way.  Managing
       *  the pointer is the user's responsibility.
       */
      _GLIBCXX_FWDLIST_REMOVE_RETURN_TYPE_TAG
      __remove_return_type
      unique()
      { return unique(std::equal_to<_Tp>()); }

#undef _GLIBCXX_FWDLIST_REMOVE_RETURN_TYPE_TAG

      /**
       *  @brief  Remove consecutive elements satisfying a predicate.
       *  @param  __binary_pred  Binary predicate function or object.
       *
       *  For each consecutive set of elements [first,last) that
       *  satisfy predicate(first,i) where i is an iterator in
       *  [first,last), remove all but the first one.  Remaining
       *  elements stay in list order.  Note that this function only
       *  erases the elements, and that if the elements themselves are
       *  pointers, the pointed-to memory is not touched in any way.
       *  Managing the pointer is the user's responsibility.
       */
      template<typename _BinPred>
	__remove_return_type
	unique(_BinPred __binary_pred);

      /**
       *  @brief  Merge sorted lists.
       *  @param  __list  Sorted list to merge.
       *
       *  Assumes that both @a list and this list are sorted according to
       *  operator<().  Merges elements of @a __list into this list in
       *  sorted order, leaving @a __list empty when complete.  Elements in
       *  this list precede elements in @a __list that are equal.
       */
      void
      merge(forward_list&& __list)
      { merge(std::move(__list), std::less<_Tp>()); }

      void
      merge(forward_list& __list)
      { merge(std::move(__list)); }

      /**
       *  @brief  Merge sorted lists according to comparison function.
       *  @param  __list  Sorted list to merge.
       *  @param  __comp Comparison function defining sort order.
       *
       *  Assumes that both @a __list and this list are sorted according to
       *  comp.  Merges elements of @a __list into this list
       *  in sorted order, leaving @a __list empty when complete.  Elements
       *  in this list precede elements in @a __list that are equivalent
       *  according to comp().
       */
      template<typename _Comp>
	void
	merge(forward_list&& __list, _Comp __comp);

      template<typename _Comp>
	void
	merge(forward_list& __list, _Comp __comp)
	{ merge(std::move(__list), __comp); }

      /**
       *  @brief  Sort the elements of the list.
       *
       *  Sorts the elements of this list in NlogN time.  Equivalent
       *  elements remain in list order.
       */
      void
      sort()
      { sort(std::less<_Tp>()); }

      /**
       *  @brief  Sort the forward_list using a comparison function.
       *
       *  Sorts the elements of this list in NlogN time.  Equivalent
       *  elements remain in list order.
       */
      template<typename _Comp>
	void
	sort(_Comp __comp);

      /**
       *  @brief  Reverse the elements in list.
       *
       *  Reverse the order of elements in the list in linear time.
       */
      void
      reverse() noexcept
      { this->_M_impl._M_head._M_reverse_after(); }

    private:
      // Called by the range constructor to implement [23.3.4.2]/9
      template<typename _InputIterator>
	void
	_M_range_initialize(_InputIterator __first, _InputIterator __last);

      // Called by forward_list(n,v,a), and the range constructor when it
      // turns out to be the same thing.
      void
      _M_fill_initialize(size_type __n, const value_type& __value);

      // Called by splice_after and insert_after.
      iterator
      _M_splice_after(const_iterator __pos, const_iterator __before,
		      const_iterator __last);

      // Called by forward_list(n).
      void
      _M_default_initialize(size_type __n);

      // Called by resize(sz).
      void
      _M_default_insert_after(const_iterator __pos, size_type __n);

      // Called by operator=(forward_list&&)
      void
      _M_move_assign(forward_list&& __list, true_type) noexcept
      {
	clear();
	this->_M_impl._M_head._M_next = __list._M_impl._M_head._M_next;
	__list._M_impl._M_head._M_next = nullptr;
	std::__alloc_on_move(this->_M_get_Node_allocator(),
			     __list._M_get_Node_allocator());
      }

      // Called by operator=(forward_list&&)
      void
      _M_move_assign(forward_list&& __list, false_type)
      {
	if (__list._M_get_Node_allocator() == this->_M_get_Node_allocator())
	  _M_move_assign(std::move(__list), true_type());
	else
	  // The rvalue's allocator cannot be moved, or is not equal,
	  // so we need to individually move each element.
	  this->assign(std::make_move_iterator(__list.begin()),
		       std::make_move_iterator(__list.end()));
      }

      // Called by assign(_InputIterator, _InputIterator) if _Tp is
      // CopyAssignable.
      template<typename _InputIterator>
	void
	_M_assign(_InputIterator __first, _InputIterator __last, true_type)
	{
	  auto __prev = before_begin();
	  auto __curr = begin();
	  auto __end = end();
	  while (__curr != __end && __first != __last)
	    {
	      *__curr = *__first;
	      ++__prev;
	      ++__curr;
	      ++__first;
	    }
	  if (__first != __last)
	    insert_after(__prev, __first, __last);
	  else if (__curr != __end)
	    erase_after(__prev, __end);
	}

      // Called by assign(_InputIterator, _InputIterator) if _Tp is not
      // CopyAssignable.
      template<typename _InputIterator>
	void
	_M_assign(_InputIterator __first, _InputIterator __last, false_type)
	{
	  clear();
	  insert_after(cbefore_begin(), __first, __last);
	}

      // Called by assign(size_type, const _Tp&) if Tp is CopyAssignable
      void
      _M_assign_n(size_type __n, const _Tp& __val, true_type)
      {
	auto __prev = before_begin();
	auto __curr = begin();
	auto __end = end();
	while (__curr != __end && __n > 0)
	  {
	    *__curr = __val;
	    ++__prev;
	    ++__curr;
	    --__n;
	  }
	if (__n > 0)
	  insert_after(__prev, __n, __val);
	else if (__curr != __end)
	  erase_after(__prev, __end);
      }

      // Called by assign(size_type, const _Tp&) if Tp is non-CopyAssignable
      void
      _M_assign_n(size_type __n, const _Tp& __val, false_type)
      {
	clear();
	insert_after(cbefore_begin(), __n, __val);
      }
    };

#if __cpp_deduction_guides >= 201606
  template<typename _InputIterator, typename _ValT
	     = typename iterator_traits<_InputIterator>::value_type,
	   typename _Allocator = allocator<_ValT>,
	   typename = _RequireInputIter<_InputIterator>,
	   typename = _RequireAllocator<_Allocator>>
    forward_list(_InputIterator, _InputIterator, _Allocator = _Allocator())
      -> forward_list<_ValT, _Allocator>;
#endif

  /**
   *  @brief  Forward list equality comparison.
   *  @param  __lx  A %forward_list
   *  @param  __ly  A %forward_list of the same type as @a __lx.
   *  @return  True iff the elements of the forward lists are equal.
   *
   *  This is an equivalence relation.  It is linear in the number of
   *  elements of the forward lists.  Deques are considered equivalent
   *  if corresponding elements compare equal.
   */
  template<typename _Tp, typename _Alloc>
    bool
    operator==(const forward_list<_Tp, _Alloc>& __lx,
	       const forward_list<_Tp, _Alloc>& __ly);

#if __cpp_lib_three_way_comparison
  /**
   *  @brief  Forward list ordering relation.
   *  @param  __x  A `forward_list`.
   *  @param  __y  A `forward_list` of the same type as `__x`.
   *  @return  A value indicating whether `__x` is less than, equal to,
   *           greater than, or incomparable with `__y`.
   *
   *  See `std::lexicographical_compare_three_way()` for how the determination
   *  is made. This operator is used to synthesize relational operators like
   *  `<` and `>=` etc.
  */
  template<typename _Tp, typename _Alloc>
    inline __detail::__synth3way_t<_Tp>
    operator<=>(const forward_list<_Tp, _Alloc>& __x,
		const forward_list<_Tp, _Alloc>& __y)
    {
      return std::lexicographical_compare_three_way(__x.begin(), __x.end(),
						    __y.begin(), __y.end(),
						    __detail::__synth3way);
    }
#else
  /**
   *  @brief  Forward list ordering relation.
   *  @param  __lx  A %forward_list.
   *  @param  __ly  A %forward_list of the same type as @a __lx.
   *  @return  True iff @a __lx is lexicographically less than @a __ly.
   *
   *  This is a total ordering relation.  It is linear in the number of
   *  elements of the forward lists.  The elements must be comparable
   *  with @c <.
   *
   *  See std::lexicographical_compare() for how the determination is made.
   */
  template<typename _Tp, typename _Alloc>
    inline bool
    operator<(const forward_list<_Tp, _Alloc>& __lx,
	      const forward_list<_Tp, _Alloc>& __ly)
    { return std::lexicographical_compare(__lx.cbegin(), __lx.cend(),
					  __ly.cbegin(), __ly.cend()); }

  /// Based on operator==
  template<typename _Tp, typename _Alloc>
    inline bool
    operator!=(const forward_list<_Tp, _Alloc>& __lx,
	       const forward_list<_Tp, _Alloc>& __ly)
    { return !(__lx == __ly); }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator>(const forward_list<_Tp, _Alloc>& __lx,
	      const forward_list<_Tp, _Alloc>& __ly)
    { return (__ly < __lx); }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator>=(const forward_list<_Tp, _Alloc>& __lx,
	       const forward_list<_Tp, _Alloc>& __ly)
    { return !(__lx < __ly); }

  /// Based on operator<
  template<typename _Tp, typename _Alloc>
    inline bool
    operator<=(const forward_list<_Tp, _Alloc>& __lx,
	       const forward_list<_Tp, _Alloc>& __ly)
    { return !(__ly < __lx); }
#endif // three-way comparison

  /// See std::forward_list::swap().
  template<typename _Tp, typename _Alloc>
    inline void
    swap(forward_list<_Tp, _Alloc>& __lx,
	 forward_list<_Tp, _Alloc>& __ly)
    noexcept(noexcept(__lx.swap(__ly)))
    { __lx.swap(__ly); }

_GLIBCXX_END_NAMESPACE_CONTAINER
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // _FORWARD_LIST_H
