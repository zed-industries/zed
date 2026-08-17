// shared_ptr and weak_ptr implementation details -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
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

// GCC Note: Based on files from version 1.32.0 of the Boost library.

//  shared_count.hpp
//  Copyright (c) 2001, 2002, 2003 Peter Dimov and Multi Media Ltd.

//  shared_ptr.hpp
//  Copyright (C) 1998, 1999 Greg Colvin and Beman Dawes.
//  Copyright (C) 2001, 2002, 2003 Peter Dimov

//  weak_ptr.hpp
//  Copyright (C) 2001, 2002, 2003 Peter Dimov

//  enable_shared_from_this.hpp
//  Copyright (C) 2002 Peter Dimov

// Distributed under the Boost Software License, Version 1.0. (See
// accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

/** @file bits/shared_ptr_base.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _SHARED_PTR_BASE_H
#define _SHARED_PTR_BASE_H 1

#include <typeinfo>
#include <bits/allocated_ptr.h>
#include <bits/refwrap.h>
#include <bits/stl_function.h>
#include <ext/aligned_buffer.h>
#if __cplusplus > 201703L
# include <compare>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
  template<typename> class auto_ptr;
#pragma GCC diagnostic pop
#endif

 /**
   *  @brief  Exception possibly thrown by @c shared_ptr.
   *  @ingroup exceptions
   */
  class bad_weak_ptr : public std::exception
  {
  public:
    virtual char const* what() const noexcept;

    virtual ~bad_weak_ptr() noexcept;
  };

  // Substitute for bad_weak_ptr object in the case of -fno-exceptions.
  inline void
  __throw_bad_weak_ptr()
  { _GLIBCXX_THROW_OR_ABORT(bad_weak_ptr()); }

  using __gnu_cxx::_Lock_policy;
  using __gnu_cxx::__default_lock_policy;
  using __gnu_cxx::_S_single;
  using __gnu_cxx::_S_mutex;
  using __gnu_cxx::_S_atomic;

  // Empty helper class except when the template argument is _S_mutex.
  template<_Lock_policy _Lp>
    class _Mutex_base
    {
    protected:
      // The atomic policy uses fully-fenced builtins, single doesn't care.
      enum { _S_need_barriers = 0 };
    };

  template<>
    class _Mutex_base<_S_mutex>
    : public __gnu_cxx::__mutex
    {
    protected:
      // This policy is used when atomic builtins are not available.
      // The replacement atomic operations might not have the necessary
      // memory barriers.
      enum { _S_need_barriers = 1 };
    };

  template<_Lock_policy _Lp = __default_lock_policy>
    class _Sp_counted_base
    : public _Mutex_base<_Lp>
    {
    public:
      _Sp_counted_base() noexcept
      : _M_use_count(1), _M_weak_count(1) { }

      virtual
      ~_Sp_counted_base() noexcept
      { }

      // Called when _M_use_count drops to zero, to release the resources
      // managed by *this.
      virtual void
      _M_dispose() noexcept = 0;

      // Called when _M_weak_count drops to zero.
      virtual void
      _M_destroy() noexcept
      { delete this; }

      virtual void*
      _M_get_deleter(const std::type_info&) noexcept = 0;

      void
      _M_add_ref_copy()
      { __gnu_cxx::__atomic_add_dispatch(&_M_use_count, 1); }

      void
      _M_add_ref_lock();

      bool
      _M_add_ref_lock_nothrow();

      void
      _M_release() noexcept
      {
        // Be race-detector-friendly.  For more info see bits/c++config.
        _GLIBCXX_SYNCHRONIZATION_HAPPENS_BEFORE(&_M_use_count);
	if (__gnu_cxx::__exchange_and_add_dispatch(&_M_use_count, -1) == 1)
	  {
            _GLIBCXX_SYNCHRONIZATION_HAPPENS_AFTER(&_M_use_count);
	    _M_dispose();
	    // There must be a memory barrier between dispose() and destroy()
	    // to ensure that the effects of dispose() are observed in the
	    // thread that runs destroy().
	    // See http://gcc.gnu.org/ml/libstdc++/2005-11/msg00136.html
	    if (_Mutex_base<_Lp>::_S_need_barriers)
	      {
		__atomic_thread_fence (__ATOMIC_ACQ_REL);
	      }

            // Be race-detector-friendly.  For more info see bits/c++config.
            _GLIBCXX_SYNCHRONIZATION_HAPPENS_BEFORE(&_M_weak_count);
	    if (__gnu_cxx::__exchange_and_add_dispatch(&_M_weak_count,
						       -1) == 1)
              {
                _GLIBCXX_SYNCHRONIZATION_HAPPENS_AFTER(&_M_weak_count);
	        _M_destroy();
              }
	  }
      }

      void
      _M_weak_add_ref() noexcept
      { __gnu_cxx::__atomic_add_dispatch(&_M_weak_count, 1); }

      void
      _M_weak_release() noexcept
      {
        // Be race-detector-friendly. For more info see bits/c++config.
        _GLIBCXX_SYNCHRONIZATION_HAPPENS_BEFORE(&_M_weak_count);
	if (__gnu_cxx::__exchange_and_add_dispatch(&_M_weak_count, -1) == 1)
	  {
            _GLIBCXX_SYNCHRONIZATION_HAPPENS_AFTER(&_M_weak_count);
	    if (_Mutex_base<_Lp>::_S_need_barriers)
	      {
	        // See _M_release(),
	        // destroy() must observe results of dispose()
		__atomic_thread_fence (__ATOMIC_ACQ_REL);
	      }
	    _M_destroy();
	  }
      }

      long
      _M_get_use_count() const noexcept
      {
        // No memory barrier is used here so there is no synchronization
        // with other threads.
        return __atomic_load_n(&_M_use_count, __ATOMIC_RELAXED);
      }

    private:
      _Sp_counted_base(_Sp_counted_base const&) = delete;
      _Sp_counted_base& operator=(_Sp_counted_base const&) = delete;

      _Atomic_word  _M_use_count;     // #shared
      _Atomic_word  _M_weak_count;    // #weak + (#shared != 0)
    };

  template<>
    inline void
    _Sp_counted_base<_S_single>::
    _M_add_ref_lock()
    {
      if (_M_use_count == 0)
	__throw_bad_weak_ptr();
      ++_M_use_count;
    }

  template<>
    inline void
    _Sp_counted_base<_S_mutex>::
    _M_add_ref_lock()
    {
      __gnu_cxx::__scoped_lock sentry(*this);
      if (__gnu_cxx::__exchange_and_add_dispatch(&_M_use_count, 1) == 0)
	{
	  _M_use_count = 0;
	  __throw_bad_weak_ptr();
	}
    }

  template<>
    inline void
    _Sp_counted_base<_S_atomic>::
    _M_add_ref_lock()
    {
      // Perform lock-free add-if-not-zero operation.
      _Atomic_word __count = _M_get_use_count();
      do
	{
	  if (__count == 0)
	    __throw_bad_weak_ptr();
	  // Replace the current counter value with the old value + 1, as
	  // long as it's not changed meanwhile.
	}
      while (!__atomic_compare_exchange_n(&_M_use_count, &__count, __count + 1,
					  true, __ATOMIC_ACQ_REL,
					  __ATOMIC_RELAXED));
    }

  template<>
    inline bool
    _Sp_counted_base<_S_single>::
    _M_add_ref_lock_nothrow()
    {
      if (_M_use_count == 0)
	return false;
      ++_M_use_count;
      return true;
    }

  template<>
    inline bool
    _Sp_counted_base<_S_mutex>::
    _M_add_ref_lock_nothrow()
    {
      __gnu_cxx::__scoped_lock sentry(*this);
      if (__gnu_cxx::__exchange_and_add_dispatch(&_M_use_count, 1) == 0)
	{
	  _M_use_count = 0;
	  return false;
	}
      return true;
    }

  template<>
    inline bool
    _Sp_counted_base<_S_atomic>::
    _M_add_ref_lock_nothrow()
    {
      // Perform lock-free add-if-not-zero operation.
      _Atomic_word __count = _M_get_use_count();
      do
	{
	  if (__count == 0)
	    return false;
	  // Replace the current counter value with the old value + 1, as
	  // long as it's not changed meanwhile.
	}
      while (!__atomic_compare_exchange_n(&_M_use_count, &__count, __count + 1,
					  true, __ATOMIC_ACQ_REL,
					  __ATOMIC_RELAXED));
      return true;
    }

  template<>
    inline void
    _Sp_counted_base<_S_single>::_M_add_ref_copy()
    { ++_M_use_count; }

  template<>
    inline void
    _Sp_counted_base<_S_single>::_M_release() noexcept
    {
      if (--_M_use_count == 0)
        {
          _M_dispose();
          if (--_M_weak_count == 0)
            _M_destroy();
        }
    }

  template<>
    inline void
    _Sp_counted_base<_S_single>::_M_weak_add_ref() noexcept
    { ++_M_weak_count; }

  template<>
    inline void
    _Sp_counted_base<_S_single>::_M_weak_release() noexcept
    {
      if (--_M_weak_count == 0)
        _M_destroy();
    }

  template<>
    inline long
    _Sp_counted_base<_S_single>::_M_get_use_count() const noexcept
    { return _M_use_count; }


  // Forward declarations.
  template<typename _Tp, _Lock_policy _Lp = __default_lock_policy>
    class __shared_ptr;

  template<typename _Tp, _Lock_policy _Lp = __default_lock_policy>
    class __weak_ptr;

  template<typename _Tp, _Lock_policy _Lp = __default_lock_policy>
    class __enable_shared_from_this;

  template<typename _Tp>
    class shared_ptr;

  template<typename _Tp>
    class weak_ptr;

  template<typename _Tp>
    struct owner_less;

  template<typename _Tp>
    class enable_shared_from_this;

  template<_Lock_policy _Lp = __default_lock_policy>
    class __weak_count;

  template<_Lock_policy _Lp = __default_lock_policy>
    class __shared_count;


  // Counted ptr with no deleter or allocator support
  template<typename _Ptr, _Lock_policy _Lp>
    class _Sp_counted_ptr final : public _Sp_counted_base<_Lp>
    {
    public:
      explicit
      _Sp_counted_ptr(_Ptr __p) noexcept
      : _M_ptr(__p) { }

      virtual void
      _M_dispose() noexcept
      { delete _M_ptr; }

      virtual void
      _M_destroy() noexcept
      { delete this; }

      virtual void*
      _M_get_deleter(const std::type_info&) noexcept
      { return nullptr; }

      _Sp_counted_ptr(const _Sp_counted_ptr&) = delete;
      _Sp_counted_ptr& operator=(const _Sp_counted_ptr&) = delete;

    private:
      _Ptr             _M_ptr;
    };

  template<>
    inline void
    _Sp_counted_ptr<nullptr_t, _S_single>::_M_dispose() noexcept { }

  template<>
    inline void
    _Sp_counted_ptr<nullptr_t, _S_mutex>::_M_dispose() noexcept { }

  template<>
    inline void
    _Sp_counted_ptr<nullptr_t, _S_atomic>::_M_dispose() noexcept { }

  template<int _Nm, typename _Tp,
	   bool __use_ebo = !__is_final(_Tp) && __is_empty(_Tp)>
    struct _Sp_ebo_helper;

  /// Specialization using EBO.
  template<int _Nm, typename _Tp>
    struct _Sp_ebo_helper<_Nm, _Tp, true> : private _Tp
    {
      explicit _Sp_ebo_helper(const _Tp& __tp) : _Tp(__tp) { }
      explicit _Sp_ebo_helper(_Tp&& __tp) : _Tp(std::move(__tp)) { }

      static _Tp&
      _S_get(_Sp_ebo_helper& __eboh) { return static_cast<_Tp&>(__eboh); }
    };

  /// Specialization not using EBO.
  template<int _Nm, typename _Tp>
    struct _Sp_ebo_helper<_Nm, _Tp, false>
    {
      explicit _Sp_ebo_helper(const _Tp& __tp) : _M_tp(__tp) { }
      explicit _Sp_ebo_helper(_Tp&& __tp) : _M_tp(std::move(__tp)) { }

      static _Tp&
      _S_get(_Sp_ebo_helper& __eboh)
      { return __eboh._M_tp; }

    private:
      _Tp _M_tp;
    };

  // Support for custom deleter and/or allocator
  template<typename _Ptr, typename _Deleter, typename _Alloc, _Lock_policy _Lp>
    class _Sp_counted_deleter final : public _Sp_counted_base<_Lp>
    {
      class _Impl : _Sp_ebo_helper<0, _Deleter>, _Sp_ebo_helper<1, _Alloc>
      {
	typedef _Sp_ebo_helper<0, _Deleter>	_Del_base;
	typedef _Sp_ebo_helper<1, _Alloc>	_Alloc_base;

      public:
	_Impl(_Ptr __p, _Deleter __d, const _Alloc& __a) noexcept
	: _M_ptr(__p), _Del_base(std::move(__d)), _Alloc_base(__a)
	{ }

	_Deleter& _M_del() noexcept { return _Del_base::_S_get(*this); }
	_Alloc& _M_alloc() noexcept { return _Alloc_base::_S_get(*this); }

	_Ptr _M_ptr;
      };

    public:
      using __allocator_type = __alloc_rebind<_Alloc, _Sp_counted_deleter>;

      // __d(__p) must not throw.
      _Sp_counted_deleter(_Ptr __p, _Deleter __d) noexcept
      : _M_impl(__p, std::move(__d), _Alloc()) { }

      // __d(__p) must not throw.
      _Sp_counted_deleter(_Ptr __p, _Deleter __d, const _Alloc& __a) noexcept
      : _M_impl(__p, std::move(__d), __a) { }

      ~_Sp_counted_deleter() noexcept { }

      virtual void
      _M_dispose() noexcept
      { _M_impl._M_del()(_M_impl._M_ptr); }

      virtual void
      _M_destroy() noexcept
      {
	__allocator_type __a(_M_impl._M_alloc());
	__allocated_ptr<__allocator_type> __guard_ptr{ __a, this };
	this->~_Sp_counted_deleter();
      }

      virtual void*
      _M_get_deleter(const std::type_info& __ti) noexcept
      {
#if __cpp_rtti
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 2400. shared_ptr's get_deleter() should use addressof()
        return __ti == typeid(_Deleter)
	  ? std::__addressof(_M_impl._M_del())
	  : nullptr;
#else
        return nullptr;
#endif
      }

    private:
      _Impl _M_impl;
    };

  // helpers for make_shared / allocate_shared

  struct _Sp_make_shared_tag
  {
  private:
    template<typename _Tp, typename _Alloc, _Lock_policy _Lp>
      friend class _Sp_counted_ptr_inplace;

    static const type_info&
    _S_ti() noexcept _GLIBCXX_VISIBILITY(default)
    {
      alignas(type_info) static constexpr char __tag[sizeof(type_info)] = { };
      return reinterpret_cast<const type_info&>(__tag);
    }

    static bool _S_eq(const type_info&) noexcept;
  };

  template<typename _Alloc>
    struct _Sp_alloc_shared_tag
    {
      const _Alloc& _M_a;
    };

  template<typename _Tp, typename _Alloc, _Lock_policy _Lp>
    class _Sp_counted_ptr_inplace final : public _Sp_counted_base<_Lp>
    {
      class _Impl : _Sp_ebo_helper<0, _Alloc>
      {
	typedef _Sp_ebo_helper<0, _Alloc>	_A_base;

      public:
	explicit _Impl(_Alloc __a) noexcept : _A_base(__a) { }

	_Alloc& _M_alloc() noexcept { return _A_base::_S_get(*this); }

	__gnu_cxx::__aligned_buffer<_Tp> _M_storage;
      };

    public:
      using __allocator_type = __alloc_rebind<_Alloc, _Sp_counted_ptr_inplace>;

      // Alloc parameter is not a reference so doesn't alias anything in __args
      template<typename... _Args>
	_Sp_counted_ptr_inplace(_Alloc __a, _Args&&... __args)
	: _M_impl(__a)
	{
	  // _GLIBCXX_RESOLVE_LIB_DEFECTS
	  // 2070.  allocate_shared should use allocator_traits<A>::construct
	  allocator_traits<_Alloc>::construct(__a, _M_ptr(),
	      std::forward<_Args>(__args)...); // might throw
	}

      ~_Sp_counted_ptr_inplace() noexcept { }

      virtual void
      _M_dispose() noexcept
      {
	allocator_traits<_Alloc>::destroy(_M_impl._M_alloc(), _M_ptr());
      }

      // Override because the allocator needs to know the dynamic type
      virtual void
      _M_destroy() noexcept
      {
	__allocator_type __a(_M_impl._M_alloc());
	__allocated_ptr<__allocator_type> __guard_ptr{ __a, this };
	this->~_Sp_counted_ptr_inplace();
      }

    private:
      friend class __shared_count<_Lp>; // To be able to call _M_ptr().

      // No longer used, but code compiled against old libstdc++ headers
      // might still call it from __shared_ptr ctor to get the pointer out.
      virtual void*
      _M_get_deleter(const std::type_info& __ti) noexcept override
      {
	auto __ptr = const_cast<typename remove_cv<_Tp>::type*>(_M_ptr());
	// Check for the fake type_info first, so we don't try to access it
	// as a real type_info object. Otherwise, check if it's the real
	// type_info for this class. With RTTI enabled we can check directly,
	// or call a library function to do it.
	if (&__ti == &_Sp_make_shared_tag::_S_ti()
	    ||
#if __cpp_rtti
	    __ti == typeid(_Sp_make_shared_tag)
#else
	    _Sp_make_shared_tag::_S_eq(__ti)
#endif
	   )
	  return __ptr;
	return nullptr;
      }

      _Tp* _M_ptr() noexcept { return _M_impl._M_storage._M_ptr(); }

      _Impl _M_impl;
    };

  // The default deleter for shared_ptr<T[]> and shared_ptr<T[N]>.
  struct __sp_array_delete
  {
    template<typename _Yp>
      void operator()(_Yp* __p) const { delete[] __p; }
  };

  template<_Lock_policy _Lp>
    class __shared_count
    {
      template<typename _Tp>
	struct __not_alloc_shared_tag { using type = void; };

      template<typename _Tp>
	struct __not_alloc_shared_tag<_Sp_alloc_shared_tag<_Tp>> { };

    public:
      constexpr __shared_count() noexcept : _M_pi(0)
      { }

      template<typename _Ptr>
        explicit
	__shared_count(_Ptr __p) : _M_pi(0)
	{
	  __try
	    {
	      _M_pi = new _Sp_counted_ptr<_Ptr, _Lp>(__p);
	    }
	  __catch(...)
	    {
	      delete __p;
	      __throw_exception_again;
	    }
	}

      template<typename _Ptr>
	__shared_count(_Ptr __p, /* is_array = */ false_type)
	: __shared_count(__p)
	{ }

      template<typename _Ptr>
	__shared_count(_Ptr __p, /* is_array = */ true_type)
	: __shared_count(__p, __sp_array_delete{}, allocator<void>())
	{ }

      template<typename _Ptr, typename _Deleter,
	       typename = typename __not_alloc_shared_tag<_Deleter>::type>
	__shared_count(_Ptr __p, _Deleter __d)
	: __shared_count(__p, std::move(__d), allocator<void>())
	{ }

      template<typename _Ptr, typename _Deleter, typename _Alloc,
	       typename = typename __not_alloc_shared_tag<_Deleter>::type>
	__shared_count(_Ptr __p, _Deleter __d, _Alloc __a) : _M_pi(0)
	{
	  typedef _Sp_counted_deleter<_Ptr, _Deleter, _Alloc, _Lp> _Sp_cd_type;
	  __try
	    {
	      typename _Sp_cd_type::__allocator_type __a2(__a);
	      auto __guard = std::__allocate_guarded(__a2);
	      _Sp_cd_type* __mem = __guard.get();
	      ::new (__mem) _Sp_cd_type(__p, std::move(__d), std::move(__a));
	      _M_pi = __mem;
	      __guard = nullptr;
	    }
	  __catch(...)
	    {
	      __d(__p); // Call _Deleter on __p.
	      __throw_exception_again;
	    }
	}

      template<typename _Tp, typename _Alloc, typename... _Args>
	__shared_count(_Tp*& __p, _Sp_alloc_shared_tag<_Alloc> __a,
		       _Args&&... __args)
	{
	  typedef _Sp_counted_ptr_inplace<_Tp, _Alloc, _Lp> _Sp_cp_type;
	  typename _Sp_cp_type::__allocator_type __a2(__a._M_a);
	  auto __guard = std::__allocate_guarded(__a2);
	  _Sp_cp_type* __mem = __guard.get();
	  auto __pi = ::new (__mem)
	    _Sp_cp_type(__a._M_a, std::forward<_Args>(__args)...);
	  __guard = nullptr;
	  _M_pi = __pi;
	  __p = __pi->_M_ptr();
	}

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
      // Special case for auto_ptr<_Tp> to provide the strong guarantee.
      template<typename _Tp>
        explicit
	__shared_count(std::auto_ptr<_Tp>&& __r);
#pragma GCC diagnostic pop
#endif

      // Special case for unique_ptr<_Tp,_Del> to provide the strong guarantee.
      template<typename _Tp, typename _Del>
        explicit
	__shared_count(std::unique_ptr<_Tp, _Del>&& __r) : _M_pi(0)
	{
	  // _GLIBCXX_RESOLVE_LIB_DEFECTS
	  // 2415. Inconsistency between unique_ptr and shared_ptr
	  if (__r.get() == nullptr)
	    return;

	  using _Ptr = typename unique_ptr<_Tp, _Del>::pointer;
	  using _Del2 = typename conditional<is_reference<_Del>::value,
	      reference_wrapper<typename remove_reference<_Del>::type>,
	      _Del>::type;
	  using _Sp_cd_type
	    = _Sp_counted_deleter<_Ptr, _Del2, allocator<void>, _Lp>;
	  using _Alloc = allocator<_Sp_cd_type>;
	  using _Alloc_traits = allocator_traits<_Alloc>;
	  _Alloc __a;
	  _Sp_cd_type* __mem = _Alloc_traits::allocate(__a, 1);
	  _Alloc_traits::construct(__a, __mem, __r.release(),
				   __r.get_deleter());  // non-throwing
	  _M_pi = __mem;
	}

      // Throw bad_weak_ptr when __r._M_get_use_count() == 0.
      explicit __shared_count(const __weak_count<_Lp>& __r);

      // Does not throw if __r._M_get_use_count() == 0, caller must check.
      explicit __shared_count(const __weak_count<_Lp>& __r, std::nothrow_t);

      ~__shared_count() noexcept
      {
	if (_M_pi != nullptr)
	  _M_pi->_M_release();
      }

      __shared_count(const __shared_count& __r) noexcept
      : _M_pi(__r._M_pi)
      {
	if (_M_pi != 0)
	  _M_pi->_M_add_ref_copy();
      }

      __shared_count&
      operator=(const __shared_count& __r) noexcept
      {
	_Sp_counted_base<_Lp>* __tmp = __r._M_pi;
	if (__tmp != _M_pi)
	  {
	    if (__tmp != 0)
	      __tmp->_M_add_ref_copy();
	    if (_M_pi != 0)
	      _M_pi->_M_release();
	    _M_pi = __tmp;
	  }
	return *this;
      }

      void
      _M_swap(__shared_count& __r) noexcept
      {
	_Sp_counted_base<_Lp>* __tmp = __r._M_pi;
	__r._M_pi = _M_pi;
	_M_pi = __tmp;
      }

      long
      _M_get_use_count() const noexcept
      { return _M_pi != 0 ? _M_pi->_M_get_use_count() : 0; }

      bool
      _M_unique() const noexcept
      { return this->_M_get_use_count() == 1; }

      void*
      _M_get_deleter(const std::type_info& __ti) const noexcept
      { return _M_pi ? _M_pi->_M_get_deleter(__ti) : nullptr; }

      bool
      _M_less(const __shared_count& __rhs) const noexcept
      { return std::less<_Sp_counted_base<_Lp>*>()(this->_M_pi, __rhs._M_pi); }

      bool
      _M_less(const __weak_count<_Lp>& __rhs) const noexcept
      { return std::less<_Sp_counted_base<_Lp>*>()(this->_M_pi, __rhs._M_pi); }

      // Friend function injected into enclosing namespace and found by ADL
      friend inline bool
      operator==(const __shared_count& __a, const __shared_count& __b) noexcept
      { return __a._M_pi == __b._M_pi; }

    private:
      friend class __weak_count<_Lp>;

      _Sp_counted_base<_Lp>*  _M_pi;
    };


  template<_Lock_policy _Lp>
    class __weak_count
    {
    public:
      constexpr __weak_count() noexcept : _M_pi(nullptr)
      { }

      __weak_count(const __shared_count<_Lp>& __r) noexcept
      : _M_pi(__r._M_pi)
      {
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_add_ref();
      }

      __weak_count(const __weak_count& __r) noexcept
      : _M_pi(__r._M_pi)
      {
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_add_ref();
      }

      __weak_count(__weak_count&& __r) noexcept
      : _M_pi(__r._M_pi)
      { __r._M_pi = nullptr; }

      ~__weak_count() noexcept
      {
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_release();
      }

      __weak_count&
      operator=(const __shared_count<_Lp>& __r) noexcept
      {
	_Sp_counted_base<_Lp>* __tmp = __r._M_pi;
	if (__tmp != nullptr)
	  __tmp->_M_weak_add_ref();
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_release();
	_M_pi = __tmp;
	return *this;
      }

      __weak_count&
      operator=(const __weak_count& __r) noexcept
      {
	_Sp_counted_base<_Lp>* __tmp = __r._M_pi;
	if (__tmp != nullptr)
	  __tmp->_M_weak_add_ref();
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_release();
	_M_pi = __tmp;
	return *this;
      }

      __weak_count&
      operator=(__weak_count&& __r) noexcept
      {
	if (_M_pi != nullptr)
	  _M_pi->_M_weak_release();
	_M_pi = __r._M_pi;
        __r._M_pi = nullptr;
	return *this;
      }

      void
      _M_swap(__weak_count& __r) noexcept
      {
	_Sp_counted_base<_Lp>* __tmp = __r._M_pi;
	__r._M_pi = _M_pi;
	_M_pi = __tmp;
      }

      long
      _M_get_use_count() const noexcept
      { return _M_pi != nullptr ? _M_pi->_M_get_use_count() : 0; }

      bool
      _M_less(const __weak_count& __rhs) const noexcept
      { return std::less<_Sp_counted_base<_Lp>*>()(this->_M_pi, __rhs._M_pi); }

      bool
      _M_less(const __shared_count<_Lp>& __rhs) const noexcept
      { return std::less<_Sp_counted_base<_Lp>*>()(this->_M_pi, __rhs._M_pi); }

      // Friend function injected into enclosing namespace and found by ADL
      friend inline bool
      operator==(const __weak_count& __a, const __weak_count& __b) noexcept
      { return __a._M_pi == __b._M_pi; }

    private:
      friend class __shared_count<_Lp>;

      _Sp_counted_base<_Lp>*  _M_pi;
    };

  // Now that __weak_count is defined we can define this constructor:
  template<_Lock_policy _Lp>
    inline
    __shared_count<_Lp>::__shared_count(const __weak_count<_Lp>& __r)
    : _M_pi(__r._M_pi)
    {
      if (_M_pi != nullptr)
	_M_pi->_M_add_ref_lock();
      else
	__throw_bad_weak_ptr();
    }

  // Now that __weak_count is defined we can define this constructor:
  template<_Lock_policy _Lp>
    inline
    __shared_count<_Lp>::
    __shared_count(const __weak_count<_Lp>& __r, std::nothrow_t)
    : _M_pi(__r._M_pi)
    {
      if (_M_pi != nullptr)
	if (!_M_pi->_M_add_ref_lock_nothrow())
	  _M_pi = nullptr;
    }

#define __cpp_lib_shared_ptr_arrays 201611L

  // Helper traits for shared_ptr of array:

  // A pointer type Y* is said to be compatible with a pointer type T* when
  // either Y* is convertible to T* or Y is U[N] and T is U cv [].
  template<typename _Yp_ptr, typename _Tp_ptr>
    struct __sp_compatible_with
    : false_type
    { };

  template<typename _Yp, typename _Tp>
    struct __sp_compatible_with<_Yp*, _Tp*>
    : is_convertible<_Yp*, _Tp*>::type
    { };

  template<typename _Up, size_t _Nm>
    struct __sp_compatible_with<_Up(*)[_Nm], _Up(*)[]>
    : true_type
    { };

  template<typename _Up, size_t _Nm>
    struct __sp_compatible_with<_Up(*)[_Nm], const _Up(*)[]>
    : true_type
    { };

  template<typename _Up, size_t _Nm>
    struct __sp_compatible_with<_Up(*)[_Nm], volatile _Up(*)[]>
    : true_type
    { };

  template<typename _Up, size_t _Nm>
    struct __sp_compatible_with<_Up(*)[_Nm], const volatile _Up(*)[]>
    : true_type
    { };

  // Test conversion from Y(*)[N] to U(*)[N] without forming invalid type Y[N].
  template<typename _Up, size_t _Nm, typename _Yp, typename = void>
    struct __sp_is_constructible_arrN
    : false_type
    { };

  template<typename _Up, size_t _Nm, typename _Yp>
    struct __sp_is_constructible_arrN<_Up, _Nm, _Yp, __void_t<_Yp[_Nm]>>
    : is_convertible<_Yp(*)[_Nm], _Up(*)[_Nm]>::type
    { };

  // Test conversion from Y(*)[] to U(*)[] without forming invalid type Y[].
  template<typename _Up, typename _Yp, typename = void>
    struct __sp_is_constructible_arr
    : false_type
    { };

  template<typename _Up, typename _Yp>
    struct __sp_is_constructible_arr<_Up, _Yp, __void_t<_Yp[]>>
    : is_convertible<_Yp(*)[], _Up(*)[]>::type
    { };

  // Trait to check if shared_ptr<T> can be constructed from Y*.
  template<typename _Tp, typename _Yp>
    struct __sp_is_constructible;

  // When T is U[N], Y(*)[N] shall be convertible to T*;
  template<typename _Up, size_t _Nm, typename _Yp>
    struct __sp_is_constructible<_Up[_Nm], _Yp>
    : __sp_is_constructible_arrN<_Up, _Nm, _Yp>::type
    { };

  // when T is U[], Y(*)[] shall be convertible to T*;
  template<typename _Up, typename _Yp>
    struct __sp_is_constructible<_Up[], _Yp>
    : __sp_is_constructible_arr<_Up, _Yp>::type
    { };

  // otherwise, Y* shall be convertible to T*.
  template<typename _Tp, typename _Yp>
    struct __sp_is_constructible
    : is_convertible<_Yp*, _Tp*>::type
    { };


  // Define operator* and operator-> for shared_ptr<T>.
  template<typename _Tp, _Lock_policy _Lp,
	   bool = is_array<_Tp>::value, bool = is_void<_Tp>::value>
    class __shared_ptr_access
    {
    public:
      using element_type = _Tp;

      element_type&
      operator*() const noexcept
      {
	__glibcxx_assert(_M_get() != nullptr);
	return *_M_get();
      }

      element_type*
      operator->() const noexcept
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_get() != nullptr);
	return _M_get();
      }

    private:
      element_type*
      _M_get() const noexcept
      { return static_cast<const __shared_ptr<_Tp, _Lp>*>(this)->get(); }
    };

  // Define operator-> for shared_ptr<cv void>.
  template<typename _Tp, _Lock_policy _Lp>
    class __shared_ptr_access<_Tp, _Lp, false, true>
    {
    public:
      using element_type = _Tp;

      element_type*
      operator->() const noexcept
      {
	auto __ptr = static_cast<const __shared_ptr<_Tp, _Lp>*>(this)->get();
	_GLIBCXX_DEBUG_PEDASSERT(__ptr != nullptr);
	return __ptr;
      }
    };

  // Define operator[] for shared_ptr<T[]> and shared_ptr<T[N]>.
  template<typename _Tp, _Lock_policy _Lp>
    class __shared_ptr_access<_Tp, _Lp, true, false>
    {
    public:
      using element_type = typename remove_extent<_Tp>::type;

#if __cplusplus <= 201402L
      [[__deprecated__("shared_ptr<T[]>::operator* is absent from C++17")]]
      element_type&
      operator*() const noexcept
      {
	__glibcxx_assert(_M_get() != nullptr);
	return *_M_get();
      }

      [[__deprecated__("shared_ptr<T[]>::operator-> is absent from C++17")]]
      element_type*
      operator->() const noexcept
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_get() != nullptr);
	return _M_get();
      }
#endif

      element_type&
      operator[](ptrdiff_t __i) const
      {
	__glibcxx_assert(_M_get() != nullptr);
	__glibcxx_assert(!extent<_Tp>::value || __i < extent<_Tp>::value);
	return _M_get()[__i];
      }

    private:
      element_type*
      _M_get() const noexcept
      { return static_cast<const __shared_ptr<_Tp, _Lp>*>(this)->get(); }
    };

  template<typename _Tp, _Lock_policy _Lp>
    class __shared_ptr
    : public __shared_ptr_access<_Tp, _Lp>
    {
    public:
      using element_type = typename remove_extent<_Tp>::type;

    private:
      // Constraint for taking ownership of a pointer of type _Yp*:
      template<typename _Yp>
	using _SafeConv
	  = typename enable_if<__sp_is_constructible<_Tp, _Yp>::value>::type;

      // Constraint for construction from shared_ptr and weak_ptr:
      template<typename _Yp, typename _Res = void>
	using _Compatible = typename
	  enable_if<__sp_compatible_with<_Yp*, _Tp*>::value, _Res>::type;

      // Constraint for assignment from shared_ptr and weak_ptr:
      template<typename _Yp>
	using _Assignable = _Compatible<_Yp, __shared_ptr&>;

      // Constraint for construction from unique_ptr:
      template<typename _Yp, typename _Del, typename _Res = void,
	       typename _Ptr = typename unique_ptr<_Yp, _Del>::pointer>
	using _UniqCompatible = typename enable_if<__and_<
	  __sp_compatible_with<_Yp*, _Tp*>, is_convertible<_Ptr, element_type*>
	  >::value, _Res>::type;

      // Constraint for assignment from unique_ptr:
      template<typename _Yp, typename _Del>
	using _UniqAssignable = _UniqCompatible<_Yp, _Del, __shared_ptr&>;

    public:

#if __cplusplus > 201402L
      using weak_type = __weak_ptr<_Tp, _Lp>;
#endif

      constexpr __shared_ptr() noexcept
      : _M_ptr(0), _M_refcount()
      { }

      template<typename _Yp, typename = _SafeConv<_Yp>>
	explicit
	__shared_ptr(_Yp* __p)
	: _M_ptr(__p), _M_refcount(__p, typename is_array<_Tp>::type())
	{
	  static_assert( !is_void<_Yp>::value, "incomplete type" );
	  static_assert( sizeof(_Yp) > 0, "incomplete type" );
	  _M_enable_shared_from_this_with(__p);
	}

      template<typename _Yp, typename _Deleter, typename = _SafeConv<_Yp>>
	__shared_ptr(_Yp* __p, _Deleter __d)
	: _M_ptr(__p), _M_refcount(__p, std::move(__d))
	{
	  static_assert(__is_invocable<_Deleter&, _Yp*&>::value,
	      "deleter expression d(p) is well-formed");
	  _M_enable_shared_from_this_with(__p);
	}

      template<typename _Yp, typename _Deleter, typename _Alloc,
	       typename = _SafeConv<_Yp>>
	__shared_ptr(_Yp* __p, _Deleter __d, _Alloc __a)
	: _M_ptr(__p), _M_refcount(__p, std::move(__d), std::move(__a))
	{
	  static_assert(__is_invocable<_Deleter&, _Yp*&>::value,
	      "deleter expression d(p) is well-formed");
	  _M_enable_shared_from_this_with(__p);
	}

      template<typename _Deleter>
	__shared_ptr(nullptr_t __p, _Deleter __d)
	: _M_ptr(0), _M_refcount(__p, std::move(__d))
	{ }

      template<typename _Deleter, typename _Alloc>
        __shared_ptr(nullptr_t __p, _Deleter __d, _Alloc __a)
	: _M_ptr(0), _M_refcount(__p, std::move(__d), std::move(__a))
	{ }

      // Aliasing constructor
      template<typename _Yp>
	__shared_ptr(const __shared_ptr<_Yp, _Lp>& __r,
		     element_type* __p) noexcept
	: _M_ptr(__p), _M_refcount(__r._M_refcount) // never throws
	{ }

      // Aliasing constructor
      template<typename _Yp>
	__shared_ptr(__shared_ptr<_Yp, _Lp>&& __r,
		     element_type* __p) noexcept
	: _M_ptr(__p), _M_refcount()
	{
	  _M_refcount._M_swap(__r._M_refcount);
	  __r._M_ptr = 0;
	}

      __shared_ptr(const __shared_ptr&) noexcept = default;
      __shared_ptr& operator=(const __shared_ptr&) noexcept = default;
      ~__shared_ptr() = default;

      template<typename _Yp, typename = _Compatible<_Yp>>
	__shared_ptr(const __shared_ptr<_Yp, _Lp>& __r) noexcept
	: _M_ptr(__r._M_ptr), _M_refcount(__r._M_refcount)
	{ }

      __shared_ptr(__shared_ptr&& __r) noexcept
      : _M_ptr(__r._M_ptr), _M_refcount()
      {
	_M_refcount._M_swap(__r._M_refcount);
	__r._M_ptr = 0;
      }

      template<typename _Yp, typename = _Compatible<_Yp>>
	__shared_ptr(__shared_ptr<_Yp, _Lp>&& __r) noexcept
	: _M_ptr(__r._M_ptr), _M_refcount()
	{
	  _M_refcount._M_swap(__r._M_refcount);
	  __r._M_ptr = 0;
	}

      template<typename _Yp, typename = _Compatible<_Yp>>
	explicit __shared_ptr(const __weak_ptr<_Yp, _Lp>& __r)
	: _M_refcount(__r._M_refcount) // may throw
	{
	  // It is now safe to copy __r._M_ptr, as
	  // _M_refcount(__r._M_refcount) did not throw.
	  _M_ptr = __r._M_ptr;
	}

      // If an exception is thrown this constructor has no effect.
      template<typename _Yp, typename _Del,
	       typename = _UniqCompatible<_Yp, _Del>>
	__shared_ptr(unique_ptr<_Yp, _Del>&& __r)
	: _M_ptr(__r.get()), _M_refcount()
	{
	  auto __raw = __to_address(__r.get());
	  _M_refcount = __shared_count<_Lp>(std::move(__r));
	  _M_enable_shared_from_this_with(__raw);
	}

#if __cplusplus <= 201402L && _GLIBCXX_USE_DEPRECATED
    protected:
      // If an exception is thrown this constructor has no effect.
      template<typename _Tp1, typename _Del,
	       typename enable_if<__and_<
		 __not_<is_array<_Tp>>, is_array<_Tp1>,
	         is_convertible<typename unique_ptr<_Tp1, _Del>::pointer, _Tp*>
	       >::value, bool>::type = true>
	__shared_ptr(unique_ptr<_Tp1, _Del>&& __r, __sp_array_delete)
	: _M_ptr(__r.get()), _M_refcount()
	{
	  auto __raw = __to_address(__r.get());
	  _M_refcount = __shared_count<_Lp>(std::move(__r));
	  _M_enable_shared_from_this_with(__raw);
	}
    public:
#endif

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
      // Postcondition: use_count() == 1 and __r.get() == 0
      template<typename _Yp, typename = _Compatible<_Yp>>
	__shared_ptr(auto_ptr<_Yp>&& __r);
#pragma GCC diagnostic pop
#endif

      constexpr __shared_ptr(nullptr_t) noexcept : __shared_ptr() { }

      template<typename _Yp>
	_Assignable<_Yp>
	operator=(const __shared_ptr<_Yp, _Lp>& __r) noexcept
	{
	  _M_ptr = __r._M_ptr;
	  _M_refcount = __r._M_refcount; // __shared_count::op= doesn't throw
	  return *this;
	}

#if _GLIBCXX_USE_DEPRECATED
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wdeprecated-declarations"
      template<typename _Yp>
	_Assignable<_Yp>
	operator=(auto_ptr<_Yp>&& __r)
	{
	  __shared_ptr(std::move(__r)).swap(*this);
	  return *this;
	}
#pragma GCC diagnostic pop
#endif

      __shared_ptr&
      operator=(__shared_ptr&& __r) noexcept
      {
	__shared_ptr(std::move(__r)).swap(*this);
	return *this;
      }

      template<class _Yp>
	_Assignable<_Yp>
	operator=(__shared_ptr<_Yp, _Lp>&& __r) noexcept
	{
	  __shared_ptr(std::move(__r)).swap(*this);
	  return *this;
	}

      template<typename _Yp, typename _Del>
	_UniqAssignable<_Yp, _Del>
	operator=(unique_ptr<_Yp, _Del>&& __r)
	{
	  __shared_ptr(std::move(__r)).swap(*this);
	  return *this;
	}

      void
      reset() noexcept
      { __shared_ptr().swap(*this); }

      template<typename _Yp>
	_SafeConv<_Yp>
	reset(_Yp* __p) // _Yp must be complete.
	{
	  // Catch self-reset errors.
	  __glibcxx_assert(__p == 0 || __p != _M_ptr);
	  __shared_ptr(__p).swap(*this);
	}

      template<typename _Yp, typename _Deleter>
	_SafeConv<_Yp>
	reset(_Yp* __p, _Deleter __d)
	{ __shared_ptr(__p, std::move(__d)).swap(*this); }

      template<typename _Yp, typename _Deleter, typename _Alloc>
	_SafeConv<_Yp>
	reset(_Yp* __p, _Deleter __d, _Alloc __a)
        { __shared_ptr(__p, std::move(__d), std::move(__a)).swap(*this); }

      /// Return the stored pointer.
      element_type*
      get() const noexcept
      { return _M_ptr; }

      /// Return true if the stored pointer is not null.
      explicit operator bool() const // never throws
      { return _M_ptr == 0 ? false : true; }

      /// Return true if use_count() == 1.
      bool
      unique() const noexcept
      { return _M_refcount._M_unique(); }

      /// If *this owns a pointer, return the number of owners, otherwise zero.
      long
      use_count() const noexcept
      { return _M_refcount._M_get_use_count(); }

      /// Exchange both the owned pointer and the stored pointer.
      void
      swap(__shared_ptr<_Tp, _Lp>& __other) noexcept
      {
	std::swap(_M_ptr, __other._M_ptr);
	_M_refcount._M_swap(__other._M_refcount);
      }

      /** @brief Define an ordering based on ownership.
       *
       * This function defines a strict weak ordering between two shared_ptr
       * or weak_ptr objects, such that one object is less than the other
       * unless they share ownership of the same pointer, or are both empty.
       * @{
      */
      template<typename _Tp1>
	bool
	owner_before(__shared_ptr<_Tp1, _Lp> const& __rhs) const noexcept
	{ return _M_refcount._M_less(__rhs._M_refcount); }

      template<typename _Tp1>
	bool
	owner_before(__weak_ptr<_Tp1, _Lp> const& __rhs) const noexcept
	{ return _M_refcount._M_less(__rhs._M_refcount); }
      // @}

    protected:
      // This constructor is non-standard, it is used by allocate_shared.
      template<typename _Alloc, typename... _Args>
	__shared_ptr(_Sp_alloc_shared_tag<_Alloc> __tag, _Args&&... __args)
	: _M_ptr(), _M_refcount(_M_ptr, __tag, std::forward<_Args>(__args)...)
	{ _M_enable_shared_from_this_with(_M_ptr); }

      template<typename _Tp1, _Lock_policy _Lp1, typename _Alloc,
	       typename... _Args>
	friend __shared_ptr<_Tp1, _Lp1>
	__allocate_shared(const _Alloc& __a, _Args&&... __args);

      // This constructor is used by __weak_ptr::lock() and
      // shared_ptr::shared_ptr(const weak_ptr&, std::nothrow_t).
      __shared_ptr(const __weak_ptr<_Tp, _Lp>& __r, std::nothrow_t)
      : _M_refcount(__r._M_refcount, std::nothrow)
      {
	_M_ptr = _M_refcount._M_get_use_count() ? __r._M_ptr : nullptr;
      }

      friend class __weak_ptr<_Tp, _Lp>;

    private:

      template<typename _Yp>
	using __esft_base_t = decltype(__enable_shared_from_this_base(
	      std::declval<const __shared_count<_Lp>&>(),
	      std::declval<_Yp*>()));

      // Detect an accessible and unambiguous enable_shared_from_this base.
      template<typename _Yp, typename = void>
	struct __has_esft_base
	: false_type { };

      template<typename _Yp>
	struct __has_esft_base<_Yp, __void_t<__esft_base_t<_Yp>>>
	: __not_<is_array<_Tp>> { }; // No enable shared_from_this for arrays

      template<typename _Yp, typename _Yp2 = typename remove_cv<_Yp>::type>
	typename enable_if<__has_esft_base<_Yp2>::value>::type
	_M_enable_shared_from_this_with(_Yp* __p) noexcept
	{
	  if (auto __base = __enable_shared_from_this_base(_M_refcount, __p))
	    __base->_M_weak_assign(const_cast<_Yp2*>(__p), _M_refcount);
	}

      template<typename _Yp, typename _Yp2 = typename remove_cv<_Yp>::type>
	typename enable_if<!__has_esft_base<_Yp2>::value>::type
	_M_enable_shared_from_this_with(_Yp*) noexcept
	{ }

      void*
      _M_get_deleter(const std::type_info& __ti) const noexcept
      { return _M_refcount._M_get_deleter(__ti); }

      template<typename _Tp1, _Lock_policy _Lp1> friend class __shared_ptr;
      template<typename _Tp1, _Lock_policy _Lp1> friend class __weak_ptr;

      template<typename _Del, typename _Tp1, _Lock_policy _Lp1>
	friend _Del* get_deleter(const __shared_ptr<_Tp1, _Lp1>&) noexcept;

      template<typename _Del, typename _Tp1>
	friend _Del* get_deleter(const shared_ptr<_Tp1>&) noexcept;

      element_type*	   _M_ptr;         // Contained pointer.
      __shared_count<_Lp>  _M_refcount;    // Reference counter.
    };


  // 20.7.2.2.7 shared_ptr comparisons
  template<typename _Tp1, typename _Tp2, _Lock_policy _Lp>
    inline bool
    operator==(const __shared_ptr<_Tp1, _Lp>& __a,
	       const __shared_ptr<_Tp2, _Lp>& __b) noexcept
    { return __a.get() == __b.get(); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator==(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    { return !__a; }

#ifdef __cpp_lib_three_way_comparison
  template<typename _Tp, typename _Up, _Lock_policy _Lp>
    inline strong_ordering
    operator<=>(const __shared_ptr<_Tp, _Lp>& __a,
		const __shared_ptr<_Up, _Lp>& __b) noexcept
    { return compare_three_way()(__a.get(), __b.get()); }

  template<typename _Tp, _Lock_policy _Lp>
    inline strong_ordering
    operator<=>(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    {
      using pointer = typename __shared_ptr<_Tp, _Lp>::element_type*;
      return compare_three_way()(__a.get(), static_cast<pointer>(nullptr));
    }
#else
  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator==(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    { return !__a; }

  template<typename _Tp1, typename _Tp2, _Lock_policy _Lp>
    inline bool
    operator!=(const __shared_ptr<_Tp1, _Lp>& __a,
	       const __shared_ptr<_Tp2, _Lp>& __b) noexcept
    { return __a.get() != __b.get(); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator!=(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    { return (bool)__a; }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator!=(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    { return (bool)__a; }

  template<typename _Tp, typename _Up, _Lock_policy _Lp>
    inline bool
    operator<(const __shared_ptr<_Tp, _Lp>& __a,
	      const __shared_ptr<_Up, _Lp>& __b) noexcept
    {
      using _Tp_elt = typename __shared_ptr<_Tp, _Lp>::element_type;
      using _Up_elt = typename __shared_ptr<_Up, _Lp>::element_type;
      using _Vp = typename common_type<_Tp_elt*, _Up_elt*>::type;
      return less<_Vp>()(__a.get(), __b.get());
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator<(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    {
      using _Tp_elt = typename __shared_ptr<_Tp, _Lp>::element_type;
      return less<_Tp_elt*>()(__a.get(), nullptr);
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator<(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    {
      using _Tp_elt = typename __shared_ptr<_Tp, _Lp>::element_type;
      return less<_Tp_elt*>()(nullptr, __a.get());
    }

  template<typename _Tp1, typename _Tp2, _Lock_policy _Lp>
    inline bool
    operator<=(const __shared_ptr<_Tp1, _Lp>& __a,
	       const __shared_ptr<_Tp2, _Lp>& __b) noexcept
    { return !(__b < __a); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator<=(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    { return !(nullptr < __a); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator<=(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    { return !(__a < nullptr); }

  template<typename _Tp1, typename _Tp2, _Lock_policy _Lp>
    inline bool
    operator>(const __shared_ptr<_Tp1, _Lp>& __a,
	      const __shared_ptr<_Tp2, _Lp>& __b) noexcept
    { return (__b < __a); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator>(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    { return nullptr < __a; }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator>(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    { return __a < nullptr; }

  template<typename _Tp1, typename _Tp2, _Lock_policy _Lp>
    inline bool
    operator>=(const __shared_ptr<_Tp1, _Lp>& __a,
	       const __shared_ptr<_Tp2, _Lp>& __b) noexcept
    { return !(__a < __b); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator>=(const __shared_ptr<_Tp, _Lp>& __a, nullptr_t) noexcept
    { return !(__a < nullptr); }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    operator>=(nullptr_t, const __shared_ptr<_Tp, _Lp>& __a) noexcept
    { return !(nullptr < __a); }
#endif // three-way comparison

  // 20.7.2.2.8 shared_ptr specialized algorithms.
  template<typename _Tp, _Lock_policy _Lp>
    inline void
    swap(__shared_ptr<_Tp, _Lp>& __a, __shared_ptr<_Tp, _Lp>& __b) noexcept
    { __a.swap(__b); }

  // 20.7.2.2.9 shared_ptr casts

  // The seemingly equivalent code:
  // shared_ptr<_Tp, _Lp>(static_cast<_Tp*>(__r.get()))
  // will eventually result in undefined behaviour, attempting to
  // delete the same object twice.
  /// static_pointer_cast
  template<typename _Tp, typename _Tp1, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    static_pointer_cast(const __shared_ptr<_Tp1, _Lp>& __r) noexcept
    {
      using _Sp = __shared_ptr<_Tp, _Lp>;
      return _Sp(__r, static_cast<typename _Sp::element_type*>(__r.get()));
    }

  // The seemingly equivalent code:
  // shared_ptr<_Tp, _Lp>(const_cast<_Tp*>(__r.get()))
  // will eventually result in undefined behaviour, attempting to
  // delete the same object twice.
  /// const_pointer_cast
  template<typename _Tp, typename _Tp1, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    const_pointer_cast(const __shared_ptr<_Tp1, _Lp>& __r) noexcept
    {
      using _Sp = __shared_ptr<_Tp, _Lp>;
      return _Sp(__r, const_cast<typename _Sp::element_type*>(__r.get()));
    }

  // The seemingly equivalent code:
  // shared_ptr<_Tp, _Lp>(dynamic_cast<_Tp*>(__r.get()))
  // will eventually result in undefined behaviour, attempting to
  // delete the same object twice.
  /// dynamic_pointer_cast
  template<typename _Tp, typename _Tp1, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    dynamic_pointer_cast(const __shared_ptr<_Tp1, _Lp>& __r) noexcept
    {
      using _Sp = __shared_ptr<_Tp, _Lp>;
      if (auto* __p = dynamic_cast<typename _Sp::element_type*>(__r.get()))
	return _Sp(__r, __p);
      return _Sp();
    }

#if __cplusplus > 201402L
  template<typename _Tp, typename _Tp1, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    reinterpret_pointer_cast(const __shared_ptr<_Tp1, _Lp>& __r) noexcept
    {
      using _Sp = __shared_ptr<_Tp, _Lp>;
      return _Sp(__r, reinterpret_cast<typename _Sp::element_type*>(__r.get()));
    }
#endif

  template<typename _Tp, _Lock_policy _Lp>
    class __weak_ptr
    {
      template<typename _Yp, typename _Res = void>
	using _Compatible = typename
	  enable_if<__sp_compatible_with<_Yp*, _Tp*>::value, _Res>::type;

      // Constraint for assignment from shared_ptr and weak_ptr:
      template<typename _Yp>
	using _Assignable = _Compatible<_Yp, __weak_ptr&>;

    public:
      using element_type = typename remove_extent<_Tp>::type;

      constexpr __weak_ptr() noexcept
      : _M_ptr(nullptr), _M_refcount()
      { }

      __weak_ptr(const __weak_ptr&) noexcept = default;

      ~__weak_ptr() = default;

      // The "obvious" converting constructor implementation:
      //
      //  template<typename _Tp1>
      //    __weak_ptr(const __weak_ptr<_Tp1, _Lp>& __r)
      //    : _M_ptr(__r._M_ptr), _M_refcount(__r._M_refcount) // never throws
      //    { }
      //
      // has a serious problem.
      //
      //  __r._M_ptr may already have been invalidated. The _M_ptr(__r._M_ptr)
      //  conversion may require access to *__r._M_ptr (virtual inheritance).
      //
      // It is not possible to avoid spurious access violations since
      // in multithreaded programs __r._M_ptr may be invalidated at any point.
      template<typename _Yp, typename = _Compatible<_Yp>>
	__weak_ptr(const __weak_ptr<_Yp, _Lp>& __r) noexcept
	: _M_refcount(__r._M_refcount)
        { _M_ptr = __r.lock().get(); }

      template<typename _Yp, typename = _Compatible<_Yp>>
	__weak_ptr(const __shared_ptr<_Yp, _Lp>& __r) noexcept
	: _M_ptr(__r._M_ptr), _M_refcount(__r._M_refcount)
	{ }

      __weak_ptr(__weak_ptr&& __r) noexcept
      : _M_ptr(__r._M_ptr), _M_refcount(std::move(__r._M_refcount))
      { __r._M_ptr = nullptr; }

      template<typename _Yp, typename = _Compatible<_Yp>>
	__weak_ptr(__weak_ptr<_Yp, _Lp>&& __r) noexcept
	: _M_ptr(__r.lock().get()), _M_refcount(std::move(__r._M_refcount))
        { __r._M_ptr = nullptr; }

      __weak_ptr&
      operator=(const __weak_ptr& __r) noexcept = default;

      template<typename _Yp>
	_Assignable<_Yp>
	operator=(const __weak_ptr<_Yp, _Lp>& __r) noexcept
	{
	  _M_ptr = __r.lock().get();
	  _M_refcount = __r._M_refcount;
	  return *this;
	}

      template<typename _Yp>
	_Assignable<_Yp>
	operator=(const __shared_ptr<_Yp, _Lp>& __r) noexcept
	{
	  _M_ptr = __r._M_ptr;
	  _M_refcount = __r._M_refcount;
	  return *this;
	}

      __weak_ptr&
      operator=(__weak_ptr&& __r) noexcept
      {
	_M_ptr = __r._M_ptr;
	_M_refcount = std::move(__r._M_refcount);
	__r._M_ptr = nullptr;
	return *this;
      }

      template<typename _Yp>
	_Assignable<_Yp>
	operator=(__weak_ptr<_Yp, _Lp>&& __r) noexcept
	{
	  _M_ptr = __r.lock().get();
	  _M_refcount = std::move(__r._M_refcount);
	  __r._M_ptr = nullptr;
	  return *this;
	}

      __shared_ptr<_Tp, _Lp>
      lock() const noexcept
      { return __shared_ptr<element_type, _Lp>(*this, std::nothrow); }

      long
      use_count() const noexcept
      { return _M_refcount._M_get_use_count(); }

      bool
      expired() const noexcept
      { return _M_refcount._M_get_use_count() == 0; }

      template<typename _Tp1>
	bool
	owner_before(const __shared_ptr<_Tp1, _Lp>& __rhs) const noexcept
	{ return _M_refcount._M_less(__rhs._M_refcount); }

      template<typename _Tp1>
	bool
	owner_before(const __weak_ptr<_Tp1, _Lp>& __rhs) const noexcept
	{ return _M_refcount._M_less(__rhs._M_refcount); }

      void
      reset() noexcept
      { __weak_ptr().swap(*this); }

      void
      swap(__weak_ptr& __s) noexcept
      {
	std::swap(_M_ptr, __s._M_ptr);
	_M_refcount._M_swap(__s._M_refcount);
      }

    private:
      // Used by __enable_shared_from_this.
      void
      _M_assign(_Tp* __ptr, const __shared_count<_Lp>& __refcount) noexcept
      {
	if (use_count() == 0)
	  {
	    _M_ptr = __ptr;
	    _M_refcount = __refcount;
	  }
      }

      template<typename _Tp1, _Lock_policy _Lp1> friend class __shared_ptr;
      template<typename _Tp1, _Lock_policy _Lp1> friend class __weak_ptr;
      friend class __enable_shared_from_this<_Tp, _Lp>;
      friend class enable_shared_from_this<_Tp>;

      element_type*	 _M_ptr;         // Contained pointer.
      __weak_count<_Lp>  _M_refcount;    // Reference counter.
    };

  // 20.7.2.3.6 weak_ptr specialized algorithms.
  template<typename _Tp, _Lock_policy _Lp>
    inline void
    swap(__weak_ptr<_Tp, _Lp>& __a, __weak_ptr<_Tp, _Lp>& __b) noexcept
    { __a.swap(__b); }

  template<typename _Tp, typename _Tp1>
    struct _Sp_owner_less : public binary_function<_Tp, _Tp, bool>
    {
      bool
      operator()(const _Tp& __lhs, const _Tp& __rhs) const noexcept
      { return __lhs.owner_before(__rhs); }

      bool
      operator()(const _Tp& __lhs, const _Tp1& __rhs) const noexcept
      { return __lhs.owner_before(__rhs); }

      bool
      operator()(const _Tp1& __lhs, const _Tp& __rhs) const noexcept
      { return __lhs.owner_before(__rhs); }
    };

  template<>
    struct _Sp_owner_less<void, void>
    {
      template<typename _Tp, typename _Up>
	auto
	operator()(const _Tp& __lhs, const _Up& __rhs) const noexcept
	-> decltype(__lhs.owner_before(__rhs))
	{ return __lhs.owner_before(__rhs); }

      using is_transparent = void;
    };

  template<typename _Tp, _Lock_policy _Lp>
    struct owner_less<__shared_ptr<_Tp, _Lp>>
    : public _Sp_owner_less<__shared_ptr<_Tp, _Lp>, __weak_ptr<_Tp, _Lp>>
    { };

  template<typename _Tp, _Lock_policy _Lp>
    struct owner_less<__weak_ptr<_Tp, _Lp>>
    : public _Sp_owner_less<__weak_ptr<_Tp, _Lp>, __shared_ptr<_Tp, _Lp>>
    { };


  template<typename _Tp, _Lock_policy _Lp>
    class __enable_shared_from_this
    {
    protected:
      constexpr __enable_shared_from_this() noexcept { }

      __enable_shared_from_this(const __enable_shared_from_this&) noexcept { }

      __enable_shared_from_this&
      operator=(const __enable_shared_from_this&) noexcept
      { return *this; }

      ~__enable_shared_from_this() { }

    public:
      __shared_ptr<_Tp, _Lp>
      shared_from_this()
      { return __shared_ptr<_Tp, _Lp>(this->_M_weak_this); }

      __shared_ptr<const _Tp, _Lp>
      shared_from_this() const
      { return __shared_ptr<const _Tp, _Lp>(this->_M_weak_this); }

#if __cplusplus > 201402L || !defined(__STRICT_ANSI__) // c++1z or gnu++11
      __weak_ptr<_Tp, _Lp>
      weak_from_this() noexcept
      { return this->_M_weak_this; }

      __weak_ptr<const _Tp, _Lp>
      weak_from_this() const noexcept
      { return this->_M_weak_this; }
#endif

    private:
      template<typename _Tp1>
	void
	_M_weak_assign(_Tp1* __p, const __shared_count<_Lp>& __n) const noexcept
	{ _M_weak_this._M_assign(__p, __n); }

      friend const __enable_shared_from_this*
      __enable_shared_from_this_base(const __shared_count<_Lp>&,
				     const __enable_shared_from_this* __p)
      { return __p; }

      template<typename, _Lock_policy>
	friend class __shared_ptr;

      mutable __weak_ptr<_Tp, _Lp>  _M_weak_this;
    };

  template<typename _Tp, _Lock_policy _Lp = __default_lock_policy,
	   typename _Alloc, typename... _Args>
    inline __shared_ptr<_Tp, _Lp>
    __allocate_shared(const _Alloc& __a, _Args&&... __args)
    {
      return __shared_ptr<_Tp, _Lp>(_Sp_alloc_shared_tag<_Alloc>{__a},
				    std::forward<_Args>(__args)...);
    }

  template<typename _Tp, _Lock_policy _Lp = __default_lock_policy,
	   typename... _Args>
    inline __shared_ptr<_Tp, _Lp>
    __make_shared(_Args&&... __args)
    {
      typedef typename std::remove_const<_Tp>::type _Tp_nc;
      return std::__allocate_shared<_Tp, _Lp>(std::allocator<_Tp_nc>(),
					      std::forward<_Args>(__args)...);
    }

  /// std::hash specialization for __shared_ptr.
  template<typename _Tp, _Lock_policy _Lp>
    struct hash<__shared_ptr<_Tp, _Lp>>
    : public __hash_base<size_t, __shared_ptr<_Tp, _Lp>>
    {
      size_t
      operator()(const __shared_ptr<_Tp, _Lp>& __s) const noexcept
      {
	return hash<typename __shared_ptr<_Tp, _Lp>::element_type*>()(
	    __s.get());
      }
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // _SHARED_PTR_BASE_H
