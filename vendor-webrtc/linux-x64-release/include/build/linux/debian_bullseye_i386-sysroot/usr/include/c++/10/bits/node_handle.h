// Node handles for containers -*- C++ -*-

// Copyright (C) 2016-2020 Free Software Foundation, Inc.
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

/** @file bits/node_handle.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly.
 *  @headername{map,set,unordered_map,unordered_set}
 */

#ifndef _NODE_HANDLE
#define _NODE_HANDLE 1

#pragma GCC system_header

#if __cplusplus > 201402L
# define __cpp_lib_node_extract 201606

#include <optional>
#include <bits/alloc_traits.h>
#include <bits/ptr_traits.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// Base class for node handle types of maps and sets.
  template<typename _Val, typename _NodeAlloc>
    class _Node_handle_common
    {
      using _AllocTraits = allocator_traits<_NodeAlloc>;

    public:
      using allocator_type = __alloc_rebind<_NodeAlloc, _Val>;

      allocator_type
      get_allocator() const noexcept
      {
	__glibcxx_assert(!this->empty());
	return allocator_type(*_M_alloc);
      }

      explicit operator bool() const noexcept { return _M_ptr != nullptr; }

      [[nodiscard]] bool empty() const noexcept { return _M_ptr == nullptr; }

    protected:
      constexpr _Node_handle_common() noexcept : _M_ptr(), _M_alloc() {}

      ~_Node_handle_common() { _M_destroy(); }

      _Node_handle_common(_Node_handle_common&& __nh) noexcept
      : _M_ptr(__nh._M_ptr), _M_alloc(std::move(__nh._M_alloc))
      {
	__nh._M_ptr = nullptr;
	__nh._M_alloc = nullopt;
      }

      _Node_handle_common&
      operator=(_Node_handle_common&& __nh) noexcept
      {
	_M_destroy();
	_M_ptr = __nh._M_ptr;
	if constexpr (is_move_assignable_v<_NodeAlloc>)
	  {
	    if (_AllocTraits::propagate_on_container_move_assignment::value
		|| !this->_M_alloc)
	      this->_M_alloc = std::move(__nh._M_alloc);
	    else
	      {
		__glibcxx_assert(this->_M_alloc == __nh._M_alloc);
	      }
	  }
	else
	  {
	    __glibcxx_assert(_M_alloc);
	  }
	__nh._M_ptr = nullptr;
	__nh._M_alloc = nullopt;
	return *this;
      }

      _Node_handle_common(typename _AllocTraits::pointer __ptr,
			  const _NodeAlloc& __alloc)
      : _M_ptr(__ptr), _M_alloc(__alloc) { }

      void
      _M_swap(_Node_handle_common& __nh) noexcept
      {
	using std::swap;
	swap(_M_ptr, __nh._M_ptr);
	if (_AllocTraits::propagate_on_container_swap::value
	    || !_M_alloc || !__nh._M_alloc)
	  _M_alloc.swap(__nh._M_alloc);
	else
	  {
	    __glibcxx_assert(_M_alloc == __nh._M_alloc);
	  }
      }

    private:
      void
      _M_destroy() noexcept
      {
	if (_M_ptr != nullptr)
	  {
	    allocator_type __alloc(*_M_alloc);
	    allocator_traits<allocator_type>::destroy(__alloc,
						      _M_ptr->_M_valptr());
	    _AllocTraits::deallocate(*_M_alloc, _M_ptr, 1);
	  }
      }

    protected:
      typename _AllocTraits::pointer	_M_ptr;
    private:
      optional<_NodeAlloc>		_M_alloc;

      template<typename _Key2, typename _Value2, typename _KeyOfValue,
	       typename _Compare, typename _ValueAlloc>
	friend class _Rb_tree;
    };

  /// Node handle type for maps.
  template<typename _Key, typename _Value, typename _NodeAlloc>
    class _Node_handle : public _Node_handle_common<_Value, _NodeAlloc>
    {
    public:
      constexpr _Node_handle() noexcept = default;
      ~_Node_handle() = default;
      _Node_handle(_Node_handle&&) noexcept = default;

      _Node_handle&
      operator=(_Node_handle&&) noexcept = default;

      using key_type = _Key;
      using mapped_type = typename _Value::second_type;

      key_type&
      key() const noexcept
      {
	__glibcxx_assert(!this->empty());
	return *_M_pkey;
      }

      mapped_type&
      mapped() const noexcept
      {
	__glibcxx_assert(!this->empty());
	return *_M_pmapped;
      }

      void
      swap(_Node_handle& __nh) noexcept
      {
	this->_M_swap(__nh);
	using std::swap;
	swap(_M_pkey, __nh._M_pkey);
	swap(_M_pmapped, __nh._M_pmapped);
      }

      friend void
      swap(_Node_handle& __x, _Node_handle& __y)
      noexcept(noexcept(__x.swap(__y)))
      { __x.swap(__y); }

    private:
      using _AllocTraits = allocator_traits<_NodeAlloc>;

      _Node_handle(typename _AllocTraits::pointer __ptr,
		   const _NodeAlloc& __alloc)
      : _Node_handle_common<_Value, _NodeAlloc>(__ptr, __alloc)
      {
	if (__ptr)
	  {
	    auto& __key = const_cast<_Key&>(__ptr->_M_valptr()->first);
	    _M_pkey = _S_pointer_to(__key);
	    _M_pmapped = _S_pointer_to(__ptr->_M_valptr()->second);
	  }
	else
	  {
	    _M_pkey = nullptr;
	    _M_pmapped = nullptr;
	  }
      }

      template<typename _Tp>
	using __pointer
	  = __ptr_rebind<typename _AllocTraits::pointer,
			 remove_reference_t<_Tp>>;

      __pointer<_Key>				_M_pkey = nullptr;
      __pointer<typename _Value::second_type>	_M_pmapped = nullptr;

      template<typename _Tp>
	__pointer<_Tp>
	_S_pointer_to(_Tp& __obj)
	{ return pointer_traits<__pointer<_Tp>>::pointer_to(__obj); }

      const key_type&
      _M_key() const noexcept { return key(); }

      template<typename _Key2, typename _Value2, typename _KeyOfValue,
	       typename _Compare, typename _ValueAlloc>
	friend class _Rb_tree;

      template<typename _Key2, typename _Value2, typename _ValueAlloc,
	       typename _ExtractKey, typename _Equal,
	       typename _H1, typename _H2, typename _Hash,
	       typename _RehashPolicy, typename _Traits>
	friend class _Hashtable;
    };

  /// Node handle type for sets.
  template<typename _Value, typename _NodeAlloc>
    class _Node_handle<_Value, _Value, _NodeAlloc>
    : public _Node_handle_common<_Value, _NodeAlloc>
    {
    public:
      constexpr _Node_handle() noexcept = default;
      ~_Node_handle() = default;
      _Node_handle(_Node_handle&&) noexcept = default;

      _Node_handle&
      operator=(_Node_handle&&) noexcept = default;

      using value_type = _Value;

      value_type&
      value() const noexcept
      {
	__glibcxx_assert(!this->empty());
	return *this->_M_ptr->_M_valptr();
      }

      void
      swap(_Node_handle& __nh) noexcept
      { this->_M_swap(__nh); }

      friend void
      swap(_Node_handle& __x, _Node_handle& __y)
      noexcept(noexcept(__x.swap(__y)))
      { __x.swap(__y); }

    private:
      using _AllocTraits = allocator_traits<_NodeAlloc>;

      _Node_handle(typename _AllocTraits::pointer __ptr,
		   const _NodeAlloc& __alloc)
      : _Node_handle_common<_Value, _NodeAlloc>(__ptr, __alloc) { }

      const value_type&
      _M_key() const noexcept { return value(); }

      template<typename _Key, typename _Val, typename _KeyOfValue,
	       typename _Compare, typename _Alloc>
	friend class _Rb_tree;

      template<typename _Key2, typename _Value2, typename _ValueAlloc,
	       typename _ExtractKey, typename _Equal,
	       typename _H1, typename _H2, typename _Hash,
	       typename _RehashPolicy, typename _Traits>
	friend class _Hashtable;
    };

  /// Return type of insert(node_handle&&) on unique maps/sets.
  template<typename _Iterator, typename _NodeHandle>
    struct _Node_insert_return
    {
      _Iterator		position = _Iterator();
      bool		inserted = false;
      _NodeHandle	node;
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif // C++17
#endif
