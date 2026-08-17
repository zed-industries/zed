// shared_ptr atomic access -*- C++ -*-

// Copyright (C) 2014-2020 Free Software Foundation, Inc.
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

/** @file bits/shared_ptr_atomic.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _SHARED_PTR_ATOMIC_H
#define _SHARED_PTR_ATOMIC_H 1

#include <bits/atomic_base.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup pointer_abstractions
   * @{
   */
  /// @relates shared_ptr @{

  /// @cond undocumented

  struct _Sp_locker
  {
    _Sp_locker(const _Sp_locker&) = delete;
    _Sp_locker& operator=(const _Sp_locker&) = delete;

#ifdef __GTHREADS
    explicit
    _Sp_locker(const void*) noexcept;
    _Sp_locker(const void*, const void*) noexcept;
    ~_Sp_locker();

  private:
    unsigned char _M_key1;
    unsigned char _M_key2;
#else
    explicit _Sp_locker(const void*, const void* = nullptr) { }
#endif
  };

  /// @endcond

  /**
   *  @brief  Report whether shared_ptr atomic operations are lock-free.
   *  @param  __p A non-null pointer to a shared_ptr object.
   *  @return True if atomic access to @c *__p is lock-free, false otherwise.
   *  @{
  */
  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    atomic_is_lock_free(const __shared_ptr<_Tp, _Lp>* __p)
    {
#ifdef __GTHREADS
      return __gthread_active_p() == 0;
#else
      return true;
#endif
    }

  template<typename _Tp>
    inline bool
    atomic_is_lock_free(const shared_ptr<_Tp>* __p)
    { return std::atomic_is_lock_free<_Tp, __default_lock_policy>(__p); }

  // @}

  /**
   *  @brief  Atomic load for shared_ptr objects.
   *  @param  __p A non-null pointer to a shared_ptr object.
   *  @return @c *__p
   *
   *  The memory order shall not be @c memory_order_release or
   *  @c memory_order_acq_rel.
   *  @{
  */
  template<typename _Tp>
    inline shared_ptr<_Tp>
    atomic_load_explicit(const shared_ptr<_Tp>* __p, memory_order)
    {
      _Sp_locker __lock{__p};
      return *__p;
    }

  template<typename _Tp>
    inline shared_ptr<_Tp>
    atomic_load(const shared_ptr<_Tp>* __p)
    { return std::atomic_load_explicit(__p, memory_order_seq_cst); }

  template<typename _Tp, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    atomic_load_explicit(const __shared_ptr<_Tp, _Lp>* __p, memory_order)
    {
      _Sp_locker __lock{__p};
      return *__p;
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    atomic_load(const __shared_ptr<_Tp, _Lp>* __p)
    { return std::atomic_load_explicit(__p, memory_order_seq_cst); }
  // @}

  /**
   *  @brief  Atomic store for shared_ptr objects.
   *  @param  __p A non-null pointer to a shared_ptr object.
   *  @param  __r The value to store.
   *
   *  The memory order shall not be @c memory_order_acquire or
   *  @c memory_order_acq_rel.
   *  @{
  */
  template<typename _Tp>
    inline void
    atomic_store_explicit(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r,
			  memory_order)
    {
      _Sp_locker __lock{__p};
      __p->swap(__r); // use swap so that **__p not destroyed while lock held
    }

  template<typename _Tp>
    inline void
    atomic_store(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r)
    { std::atomic_store_explicit(__p, std::move(__r), memory_order_seq_cst); }

  template<typename _Tp, _Lock_policy _Lp>
    inline void
    atomic_store_explicit(__shared_ptr<_Tp, _Lp>* __p,
			  __shared_ptr<_Tp, _Lp> __r,
			  memory_order)
    {
      _Sp_locker __lock{__p};
      __p->swap(__r); // use swap so that **__p not destroyed while lock held
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline void
    atomic_store(__shared_ptr<_Tp, _Lp>* __p, __shared_ptr<_Tp, _Lp> __r)
    { std::atomic_store_explicit(__p, std::move(__r), memory_order_seq_cst); }
  // @}

  /**
   *  @brief  Atomic exchange for shared_ptr objects.
   *  @param  __p A non-null pointer to a shared_ptr object.
   *  @param  __r New value to store in @c *__p.
   *  @return The original value of @c *__p
   *  @{
  */
  template<typename _Tp>
    inline shared_ptr<_Tp>
    atomic_exchange_explicit(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r,
			     memory_order)
    {
      _Sp_locker __lock{__p};
      __p->swap(__r);
      return __r;
    }

  template<typename _Tp>
    inline shared_ptr<_Tp>
    atomic_exchange(shared_ptr<_Tp>* __p, shared_ptr<_Tp> __r)
    {
      return std::atomic_exchange_explicit(__p, std::move(__r),
					   memory_order_seq_cst);
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    atomic_exchange_explicit(__shared_ptr<_Tp, _Lp>* __p,
			     __shared_ptr<_Tp, _Lp> __r,
			     memory_order)
    {
      _Sp_locker __lock{__p};
      __p->swap(__r);
      return __r;
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline __shared_ptr<_Tp, _Lp>
    atomic_exchange(__shared_ptr<_Tp, _Lp>* __p, __shared_ptr<_Tp, _Lp> __r)
    {
      return std::atomic_exchange_explicit(__p, std::move(__r),
					   memory_order_seq_cst);
    }
  // @}

  /**
   *  @brief  Atomic compare-and-swap for shared_ptr objects.
   *  @param  __p A non-null pointer to a shared_ptr object.
   *  @param  __v A non-null pointer to a shared_ptr object.
   *  @param  __w A non-null pointer to a shared_ptr object.
   *  @return True if @c *__p was equivalent to @c *__v, false otherwise.
   *
   *  The memory order for failure shall not be @c memory_order_release or
   *  @c memory_order_acq_rel, or stronger than the memory order for success.
   *  @{
  */
  template<typename _Tp>
    bool
    atomic_compare_exchange_strong_explicit(shared_ptr<_Tp>* __p,
					    shared_ptr<_Tp>* __v,
					    shared_ptr<_Tp> __w,
					    memory_order,
					    memory_order)
    {
      shared_ptr<_Tp> __x; // goes out of scope after __lock
      _Sp_locker __lock{__p, __v};
      owner_less<shared_ptr<_Tp>> __less;
      if (*__p == *__v && !__less(*__p, *__v) && !__less(*__v, *__p))
	{
	  __x = std::move(*__p);
	  *__p = std::move(__w);
	  return true;
	}
      __x = std::move(*__v);
      *__v = *__p;
      return false;
    }

  template<typename _Tp>
    inline bool
    atomic_compare_exchange_strong(shared_ptr<_Tp>* __p, shared_ptr<_Tp>* __v,
				 shared_ptr<_Tp> __w)
    {
      return std::atomic_compare_exchange_strong_explicit(__p, __v,
	  std::move(__w), memory_order_seq_cst, memory_order_seq_cst);
    }

  template<typename _Tp>
    inline bool
    atomic_compare_exchange_weak_explicit(shared_ptr<_Tp>* __p,
					  shared_ptr<_Tp>* __v,
					  shared_ptr<_Tp> __w,
					  memory_order __success,
					  memory_order __failure)
    {
      return std::atomic_compare_exchange_strong_explicit(__p, __v,
	  std::move(__w), __success, __failure);
    }

  template<typename _Tp>
    inline bool
    atomic_compare_exchange_weak(shared_ptr<_Tp>* __p, shared_ptr<_Tp>* __v,
				 shared_ptr<_Tp> __w)
    {
      return std::atomic_compare_exchange_weak_explicit(__p, __v,
	  std::move(__w), memory_order_seq_cst, memory_order_seq_cst);
    }

  template<typename _Tp, _Lock_policy _Lp>
    bool
    atomic_compare_exchange_strong_explicit(__shared_ptr<_Tp, _Lp>* __p,
					    __shared_ptr<_Tp, _Lp>* __v,
					    __shared_ptr<_Tp, _Lp> __w,
					    memory_order,
					    memory_order)
    {
      __shared_ptr<_Tp, _Lp> __x; // goes out of scope after __lock
      _Sp_locker __lock{__p, __v};
      owner_less<__shared_ptr<_Tp, _Lp>> __less;
      if (*__p == *__v && !__less(*__p, *__v) && !__less(*__v, *__p))
	{
	  __x = std::move(*__p);
	  *__p = std::move(__w);
	  return true;
	}
      __x = std::move(*__v);
      *__v = *__p;
      return false;
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    atomic_compare_exchange_strong(__shared_ptr<_Tp, _Lp>* __p,
				   __shared_ptr<_Tp, _Lp>* __v,
				   __shared_ptr<_Tp, _Lp> __w)
    {
      return std::atomic_compare_exchange_strong_explicit(__p, __v,
	  std::move(__w), memory_order_seq_cst, memory_order_seq_cst);
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    atomic_compare_exchange_weak_explicit(__shared_ptr<_Tp, _Lp>* __p,
					  __shared_ptr<_Tp, _Lp>* __v,
					  __shared_ptr<_Tp, _Lp> __w,
					  memory_order __success,
					  memory_order __failure)
    {
      return std::atomic_compare_exchange_strong_explicit(__p, __v,
	  std::move(__w), __success, __failure);
    }

  template<typename _Tp, _Lock_policy _Lp>
    inline bool
    atomic_compare_exchange_weak(__shared_ptr<_Tp, _Lp>* __p,
				 __shared_ptr<_Tp, _Lp>* __v,
				 __shared_ptr<_Tp, _Lp> __w)
    {
      return std::atomic_compare_exchange_weak_explicit(__p, __v,
	  std::move(__w), memory_order_seq_cst, memory_order_seq_cst);
    }
  // @}

  // @} relates shared_ptr
  // @} group pointer_abstractions

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // _SHARED_PTR_ATOMIC_H
