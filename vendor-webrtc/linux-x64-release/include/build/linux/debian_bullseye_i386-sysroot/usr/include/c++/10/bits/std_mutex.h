// std::mutex implementation -*- C++ -*-

// Copyright (C) 2003-2020 Free Software Foundation, Inc.
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

/** @file bits/std_mutex.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{mutex}
 */

#ifndef _GLIBCXX_MUTEX_H
#define _GLIBCXX_MUTEX_H 1

#pragma GCC system_header

#if __cplusplus < 201103L
# include <bits/c++0x_warning.h>
#else

#include <system_error>
#include <bits/functexcept.h>
#include <bits/gthr.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @defgroup mutexes Mutexes
   * @ingroup concurrency
   *
   * Classes for mutex support.
   * @{
   */

#ifdef _GLIBCXX_HAS_GTHREADS
  // Common base class for std::mutex and std::timed_mutex
  class __mutex_base
  {
  protected:
    typedef __gthread_mutex_t			__native_type;

#ifdef __GTHREAD_MUTEX_INIT
    __native_type  _M_mutex = __GTHREAD_MUTEX_INIT;

    constexpr __mutex_base() noexcept = default;
#else
    __native_type  _M_mutex;

    __mutex_base() noexcept
    {
      // XXX EAGAIN, ENOMEM, EPERM, EBUSY(may), EINVAL(may)
      __GTHREAD_MUTEX_INIT_FUNCTION(&_M_mutex);
    }

    ~__mutex_base() noexcept { __gthread_mutex_destroy(&_M_mutex); }
#endif

    __mutex_base(const __mutex_base&) = delete;
    __mutex_base& operator=(const __mutex_base&) = delete;
  };

  /// The standard mutex type.
  class mutex : private __mutex_base
  {
  public:
    typedef __native_type* 			native_handle_type;

#ifdef __GTHREAD_MUTEX_INIT
    constexpr
#endif
    mutex() noexcept = default;
    ~mutex() = default;

    mutex(const mutex&) = delete;
    mutex& operator=(const mutex&) = delete;

    void
    lock()
    {
      int __e = __gthread_mutex_lock(&_M_mutex);

      // EINVAL, EAGAIN, EBUSY, EINVAL, EDEADLK(may)
      if (__e)
	__throw_system_error(__e);
    }

    bool
    try_lock() noexcept
    {
      // XXX EINVAL, EAGAIN, EBUSY
      return !__gthread_mutex_trylock(&_M_mutex);
    }

    void
    unlock()
    {
      // XXX EINVAL, EAGAIN, EPERM
      __gthread_mutex_unlock(&_M_mutex);
    }

    native_handle_type
    native_handle() noexcept
    { return &_M_mutex; }
  };

#endif // _GLIBCXX_HAS_GTHREADS

  /// Do not acquire ownership of the mutex.
  struct defer_lock_t { explicit defer_lock_t() = default; };

  /// Try to acquire ownership of the mutex without blocking.
  struct try_to_lock_t { explicit try_to_lock_t() = default; };

  /// Assume the calling thread has already obtained mutex ownership
  /// and manage it.
  struct adopt_lock_t { explicit adopt_lock_t() = default; };

  /// Tag used to prevent a scoped lock from acquiring ownership of a mutex.
  _GLIBCXX17_INLINE constexpr defer_lock_t	defer_lock { };

  /// Tag used to prevent a scoped lock from blocking if a mutex is locked.
  _GLIBCXX17_INLINE constexpr try_to_lock_t	try_to_lock { };

  /// Tag used to make a scoped lock take ownership of a locked mutex.
  _GLIBCXX17_INLINE constexpr adopt_lock_t	adopt_lock { };

  /** @brief A simple scoped lock type.
   *
   * A lock_guard controls mutex ownership within a scope, releasing
   * ownership in the destructor.
   */
  template<typename _Mutex>
    class lock_guard
    {
    public:
      typedef _Mutex mutex_type;

      explicit lock_guard(mutex_type& __m) : _M_device(__m)
      { _M_device.lock(); }

      lock_guard(mutex_type& __m, adopt_lock_t) noexcept : _M_device(__m)
      { } // calling thread owns mutex

      ~lock_guard()
      { _M_device.unlock(); }

      lock_guard(const lock_guard&) = delete;
      lock_guard& operator=(const lock_guard&) = delete;

    private:
      mutex_type&  _M_device;
    };

  // @} group mutexes
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
#endif // C++11
#endif // _GLIBCXX_MUTEX_H
