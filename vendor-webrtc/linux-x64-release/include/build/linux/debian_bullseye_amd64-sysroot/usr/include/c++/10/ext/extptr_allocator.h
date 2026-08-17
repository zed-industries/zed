// <extptr_allocator.h> -*- C++ -*-

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

/**
 *  @file ext/extptr_allocator.h
 *  This file is a GNU extension to the Standard C++ Library.
 *
 *  @author Bob Walters
 *
 * An example allocator which uses an alternative pointer type from
 * bits/pointer.h.  Supports test cases which confirm container support
 * for alternative pointers.
 */

#ifndef _EXTPTR_ALLOCATOR_H
#define _EXTPTR_ALLOCATOR_H 1

#include <memory>
#include <ext/numeric_traits.h>
#include <ext/pointer.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @brief An example allocator which uses a non-standard pointer type.
   * @ingroup allocators
   *
   * This allocator specifies that containers use a 'relative pointer' as it's
   * pointer type.  (See ext/pointer.h)  Memory allocation in this example
   * is still performed using std::allocator.
   */
  template<typename _Tp>
    class _ExtPtr_allocator
    {
    public:
      typedef std::size_t     size_type;
      typedef std::ptrdiff_t  difference_type;

      // Note the non-standard pointer types.
      typedef _Pointer_adapter<_Relative_pointer_impl<_Tp> >       pointer;
      typedef _Pointer_adapter<_Relative_pointer_impl<const _Tp> > 
                                                             const_pointer;

      typedef _Tp&       reference;
      typedef const _Tp& const_reference;
      typedef _Tp        value_type;

      template<typename _Up>
        struct rebind
        { typedef _ExtPtr_allocator<_Up> other; };

      _ExtPtr_allocator() _GLIBCXX_USE_NOEXCEPT 
      : _M_real_alloc() { }

      _ExtPtr_allocator(const _ExtPtr_allocator& __rarg) _GLIBCXX_USE_NOEXCEPT
      : _M_real_alloc(__rarg._M_real_alloc) { }

      template<typename _Up>
        _ExtPtr_allocator(const _ExtPtr_allocator<_Up>& __rarg)
	_GLIBCXX_USE_NOEXCEPT
        : _M_real_alloc(__rarg._M_getUnderlyingImp()) { }

      ~_ExtPtr_allocator() _GLIBCXX_USE_NOEXCEPT
      { }

      pointer address(reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }

      const_pointer address(const_reference __x) const _GLIBCXX_NOEXCEPT
      { return std::__addressof(__x); }

      _GLIBCXX_NODISCARD pointer allocate(size_type __n, const void* = 0)
      { return _M_real_alloc.allocate(__n); }

      void deallocate(pointer __p, size_type __n)
      { _M_real_alloc.deallocate(__p.get(), __n); }

      size_type max_size() const _GLIBCXX_USE_NOEXCEPT
      { return __numeric_traits<size_type>::__max / sizeof(_Tp); }

#if __cplusplus >= 201103L
      template<typename _Up, typename... _Args>
        void
        construct(_Up* __p, _Args&&... __args)
	{ ::new((void *)__p) _Up(std::forward<_Args>(__args)...); }

      template<typename... _Args>
        void
        construct(pointer __p, _Args&&... __args)
	{ construct(__p.get(), std::forward<_Args>(__args)...); }

      template<typename _Up>
        void 
        destroy(_Up* __p)
        { __p->~_Up(); }

      void destroy(pointer __p)
      { destroy(__p.get()); }

#else

      void construct(pointer __p, const _Tp& __val)
      { ::new(__p.get()) _Tp(__val); }

      void destroy(pointer __p)
      { __p->~_Tp(); }
#endif

      template<typename _Up>
        inline bool
        operator==(const _ExtPtr_allocator<_Up>& __rarg) const
        { return _M_real_alloc == __rarg._M_getUnderlyingImp(); }

      inline bool
      operator==(const _ExtPtr_allocator& __rarg) const
      { return _M_real_alloc == __rarg._M_real_alloc; }

#if __cpp_impl_three_way_comparison < 201907L
      template<typename _Up>
        inline bool
        operator!=(const _ExtPtr_allocator<_Up>& __rarg) const
        { return _M_real_alloc != __rarg._M_getUnderlyingImp(); }

      inline bool
      operator!=(const _ExtPtr_allocator& __rarg) const
      { return _M_real_alloc != __rarg._M_real_alloc; }
#endif

      template<typename _Up>
        inline friend void
        swap(_ExtPtr_allocator<_Up>&, _ExtPtr_allocator<_Up>&);

      // A method specific to this implementation.
      const std::allocator<_Tp>&
      _M_getUnderlyingImp() const
      { return _M_real_alloc; }

    private:
      std::allocator<_Tp>  _M_real_alloc;
    };

  // _ExtPtr_allocator<void> specialization.
  template<>
    class _ExtPtr_allocator<void>
    {
    public:
      typedef std::size_t      size_type;
      typedef std::ptrdiff_t   difference_type;
      typedef void             value_type;

      // Note the non-standard pointer types
      typedef _Pointer_adapter<_Relative_pointer_impl<void> >       pointer;
      typedef _Pointer_adapter<_Relative_pointer_impl<const void> >
                                                              const_pointer;

      template<typename _Up>
        struct rebind
        { typedef _ExtPtr_allocator<_Up> other; };

    private:
      std::allocator<void>  _M_real_alloc;
    };

  template<typename _Tp>
    inline void
    swap(_ExtPtr_allocator<_Tp>& __larg, _ExtPtr_allocator<_Tp>& __rarg)
    {
      std::allocator<_Tp> __tmp( __rarg._M_real_alloc );
      __rarg._M_real_alloc = __larg._M_real_alloc;
      __larg._M_real_alloc = __tmp;
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _EXTPTR_ALLOCATOR_H */
