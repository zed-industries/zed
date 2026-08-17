// Support for concurrent programing -*- C++ -*-

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

/** @file ext/concurrence.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _CONCURRENCE_H
#define _CONCURRENCE_H 1

#pragma GCC system_header

#include <exception>
#include <bits/gthr.h> 
#include <bits/functexcept.h>
#include <bits/cpp_type_traits.h>
#include <ext/type_traits.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // Available locking policies:
  // _S_single    single-threaded code that doesn't need to be locked.
  // _S_mutex     multi-threaded code that requires additional support
  //              from gthr.h or abstraction layers in concurrence.h.
  // _S_atomic    multi-threaded code using atomic operations.
  enum _Lock_policy { _S_single, _S_mutex, _S_atomic }; 

  // Compile time constant that indicates prefered locking policy in
  // the current configuration.
  static const _Lock_policy __default_lock_policy = 
#ifndef __GTHREADS
  _S_single;
#elif defined _GLIBCXX_HAVE_ATOMIC_LOCK_POLICY
  _S_atomic;
#else
  _S_mutex;
#endif

  // NB: As this is used in libsupc++, need to only depend on
  // exception. No stdexception classes, no use of std::string.
  class __concurrence_lock_error : public std::exception
  {
  public:
    virtual char const*
    what() const throw()
    { return "__gnu_cxx::__concurrence_lock_error"; }
  };

  class __concurrence_unlock_error : public std::exception
  {
  public:
    virtual char const*
    what() const throw()
    { return "__gnu_cxx::__concurrence_unlock_error"; }
  };

  class __concurrence_broadcast_error : public std::exception
  {
  public:
    virtual char const*
    what() const throw()
    { return "__gnu_cxx::__concurrence_broadcast_error"; }
  };

  class __concurrence_wait_error : public std::exception
  {
  public:
    virtual char const*
    what() const throw()
    { return "__gnu_cxx::__concurrence_wait_error"; }
  };

  // Substitute for concurrence_error object in the case of -fno-exceptions.
  inline void
  __throw_concurrence_lock_error()
  { _GLIBCXX_THROW_OR_ABORT(__concurrence_lock_error()); }

  inline void
  __throw_concurrence_unlock_error()
  { _GLIBCXX_THROW_OR_ABORT(__concurrence_unlock_error()); }

#ifdef __GTHREAD_HAS_COND
  inline void
  __throw_concurrence_broadcast_error()
  { _GLIBCXX_THROW_OR_ABORT(__concurrence_broadcast_error()); }

  inline void
  __throw_concurrence_wait_error()
  { _GLIBCXX_THROW_OR_ABORT(__concurrence_wait_error()); }
#endif
 
  class __mutex 
  {
  private:
#if __GTHREADS && defined __GTHREAD_MUTEX_INIT
    __gthread_mutex_t _M_mutex = __GTHREAD_MUTEX_INIT;
#else
    __gthread_mutex_t _M_mutex;
#endif

    __mutex(const __mutex&);
    __mutex& operator=(const __mutex&);

  public:
    __mutex() 
    { 
#if __GTHREADS && ! defined __GTHREAD_MUTEX_INIT
      if (__gthread_active_p())
	__GTHREAD_MUTEX_INIT_FUNCTION(&_M_mutex);
#endif
    }

#if __GTHREADS && ! defined __GTHREAD_MUTEX_INIT
    ~__mutex() 
    { 
      if (__gthread_active_p())
	__gthread_mutex_destroy(&_M_mutex); 
    }
#endif 

    void lock()
    {
#if __GTHREADS
      if (__gthread_active_p())
	{
	  if (__gthread_mutex_lock(&_M_mutex) != 0)
	    __throw_concurrence_lock_error();
	}
#endif
    }
    
    void unlock()
    {
#if __GTHREADS
      if (__gthread_active_p())
	{
	  if (__gthread_mutex_unlock(&_M_mutex) != 0)
	    __throw_concurrence_unlock_error();
	}
#endif
    }

    __gthread_mutex_t* gthread_mutex(void)
      { return &_M_mutex; }
  };

  class __recursive_mutex 
  {
  private:
#if __GTHREADS && defined __GTHREAD_RECURSIVE_MUTEX_INIT
    __gthread_recursive_mutex_t _M_mutex = __GTHREAD_RECURSIVE_MUTEX_INIT;
#else
    __gthread_recursive_mutex_t _M_mutex;
#endif

    __recursive_mutex(const __recursive_mutex&);
    __recursive_mutex& operator=(const __recursive_mutex&);

  public:
    __recursive_mutex() 
    { 
#if __GTHREADS && ! defined __GTHREAD_RECURSIVE_MUTEX_INIT
      if (__gthread_active_p())
	__GTHREAD_RECURSIVE_MUTEX_INIT_FUNCTION(&_M_mutex);
#endif
    }

#if __GTHREADS && ! defined __GTHREAD_RECURSIVE_MUTEX_INIT
    ~__recursive_mutex()
    {
      if (__gthread_active_p())
	__gthread_recursive_mutex_destroy(&_M_mutex);
    }
#endif

    void lock()
    { 
#if __GTHREADS
      if (__gthread_active_p())
	{
	  if (__gthread_recursive_mutex_lock(&_M_mutex) != 0)
	    __throw_concurrence_lock_error();
	}
#endif
    }
    
    void unlock()
    { 
#if __GTHREADS
      if (__gthread_active_p())
	{
	  if (__gthread_recursive_mutex_unlock(&_M_mutex) != 0)
	    __throw_concurrence_unlock_error();
	}
#endif
    }

    __gthread_recursive_mutex_t* gthread_recursive_mutex(void)
    { return &_M_mutex; }
  };

  /// Scoped lock idiom.
  // Acquire the mutex here with a constructor call, then release with
  // the destructor call in accordance with RAII style.
  class __scoped_lock
  {
  public:
    typedef __mutex __mutex_type;

  private:
    __mutex_type& _M_device;

    __scoped_lock(const __scoped_lock&);
    __scoped_lock& operator=(const __scoped_lock&);

  public:
    explicit __scoped_lock(__mutex_type& __name) : _M_device(__name)
    { _M_device.lock(); }

    ~__scoped_lock() throw()
    { _M_device.unlock(); }
  };

#ifdef __GTHREAD_HAS_COND
  class __cond
  {
  private:
#if __GTHREADS && defined __GTHREAD_COND_INIT
    __gthread_cond_t _M_cond = __GTHREAD_COND_INIT;
#else
    __gthread_cond_t _M_cond;
#endif

    __cond(const __cond&);
    __cond& operator=(const __cond&);

  public:
    __cond() 
    { 
#if __GTHREADS && ! defined __GTHREAD_COND_INIT
      if (__gthread_active_p())
	__GTHREAD_COND_INIT_FUNCTION(&_M_cond);
#endif
    }

#if __GTHREADS && ! defined __GTHREAD_COND_INIT
    ~__cond() 
    { 
      if (__gthread_active_p())
	__gthread_cond_destroy(&_M_cond); 
    }
#endif 

    void broadcast()
    {
#if __GTHREADS
      if (__gthread_active_p())
	{
	  if (__gthread_cond_broadcast(&_M_cond) != 0)
	    __throw_concurrence_broadcast_error();
	}
#endif
    }

    void wait(__mutex *mutex)
    {
#if __GTHREADS
      {
	  if (__gthread_cond_wait(&_M_cond, mutex->gthread_mutex()) != 0)
	    __throw_concurrence_wait_error();
      }
#endif
    }

    void wait_recursive(__recursive_mutex *mutex)
    {
#if __GTHREADS
      {
	  if (__gthread_cond_wait_recursive(&_M_cond,
					    mutex->gthread_recursive_mutex())
	      != 0)
	    __throw_concurrence_wait_error();
      }
#endif
    }
  };
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
