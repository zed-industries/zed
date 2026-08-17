// Custom pointer adapter and sample storage policies

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
 *  @file ext/pointer.h
 *  This file is a GNU extension to the Standard C++ Library.
 *
 *  @author Bob Walters
 *
 * Provides reusable _Pointer_adapter for assisting in the development of
 * custom pointer types that can be used with the standard containers via
 * the allocator::pointer and allocator::const_pointer typedefs.
 */

#ifndef _POINTER_H
#define _POINTER_H 1

#pragma GCC system_header

#include <iosfwd>
#include <bits/stl_iterator_base_types.h>
#include <ext/cast.h>
#include <ext/type_traits.h>
#if __cplusplus >= 201103L
# include <bits/move.h>
# include <bits/ptr_traits.h>
#endif
#if __cplusplus > 201703L
# include <iterator> // for indirectly_readable_traits
#endif

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /** 
   * @brief A storage policy for use with _Pointer_adapter<> which yields a
   *        standard pointer.
   * 
   *  A _Storage_policy is required to provide 4 things:
   *    1) A get() API for returning the stored pointer value.
   *    2) An set() API for storing a pointer value.
   *    3) An element_type typedef to define the type this points to.
   *    4) An operator<() to support pointer comparison.
   *    5) An operator==() to support pointer comparison.
   */
  template<typename _Tp> 
    class _Std_pointer_impl 
    {
    public:
      // the type this pointer points to.
      typedef _Tp element_type;
  
      // A method to fetch the pointer value as a standard T* value;
      inline _Tp* 
      get() const 
      { return _M_value; }
  
      // A method to set the pointer value, from a standard T* value;
      inline void 
      set(element_type* __arg) 
      { _M_value = __arg; }
  
      // Comparison of pointers
      inline bool
      operator<(const _Std_pointer_impl& __rarg) const
      { return (_M_value < __rarg._M_value); }
  
      inline bool
      operator==(const _Std_pointer_impl& __rarg) const
      { return (_M_value == __rarg._M_value); }

    private:
      element_type* _M_value;
    };

  /**
   * @brief A storage policy for use with _Pointer_adapter<> which stores
   *        the pointer's address as an offset value which is relative to
   *        its own address.
   * 
   * This is intended for pointers within shared memory regions which
   * might be mapped at different addresses by different processes.
   * For null pointers, a value of 1 is used.  (0 is legitimate
   * sometimes for nodes in circularly linked lists) This value was
   * chosen as the least likely to generate an incorrect null, As
   * there is no reason why any normal pointer would point 1 byte into
   * its own pointer address.
   */
  template<typename _Tp> 
    class _Relative_pointer_impl 
    {
    public:
      typedef _Tp element_type;
  
      _Tp*
      get() const 
      {
        if (_M_diff == 1)
          return 0;
        else
          return reinterpret_cast<_Tp*>(reinterpret_cast<_UIntPtrType>(this)
					+ _M_diff);
      }
  
      void 
      set(_Tp* __arg)
      {
        if (!__arg)
          _M_diff = 1;
        else
          _M_diff = reinterpret_cast<_UIntPtrType>(__arg) 
                    - reinterpret_cast<_UIntPtrType>(this);
      }
  
      // Comparison of pointers
      inline bool
      operator<(const _Relative_pointer_impl& __rarg) const
      { return (reinterpret_cast<_UIntPtrType>(this->get())
		< reinterpret_cast<_UIntPtrType>(__rarg.get())); }

      inline bool
      operator==(const _Relative_pointer_impl& __rarg) const
      { return (reinterpret_cast<_UIntPtrType>(this->get())
		== reinterpret_cast<_UIntPtrType>(__rarg.get())); }

    private:
#ifdef _GLIBCXX_USE_LONG_LONG
      typedef __gnu_cxx::__conditional_type<
	 (sizeof(unsigned long) >= sizeof(void*)),
	 unsigned long, unsigned long long>::__type _UIntPtrType;
#else
      typedef unsigned long _UIntPtrType;
#endif
      _UIntPtrType _M_diff;
    };
  
  /**
   * Relative_pointer_impl needs a specialization for const T because of
   * the casting done during pointer arithmetic.
   */
  template<typename _Tp> 
    class _Relative_pointer_impl<const _Tp> 
    {
    public:
      typedef const _Tp element_type;
  
      const _Tp*
      get() const
      {
        if (_M_diff == 1)
          return 0;
        else
          return reinterpret_cast<const _Tp*>
	      (reinterpret_cast<_UIntPtrType>(this) + _M_diff);
      }
  
      void 
      set(const _Tp* __arg)
      {
        if (!__arg)
          _M_diff = 1;
        else
          _M_diff = reinterpret_cast<_UIntPtrType>(__arg) 
                    - reinterpret_cast<_UIntPtrType>(this);
      }
  
      // Comparison of pointers
      inline bool
      operator<(const _Relative_pointer_impl& __rarg) const
      { return (reinterpret_cast<_UIntPtrType>(this->get())
		< reinterpret_cast<_UIntPtrType>(__rarg.get())); }

      inline bool
      operator==(const _Relative_pointer_impl& __rarg) const
      { return (reinterpret_cast<_UIntPtrType>(this->get())
		== reinterpret_cast<_UIntPtrType>(__rarg.get())); }
  
    private:
#ifdef _GLIBCXX_USE_LONG_LONG
      typedef __gnu_cxx::__conditional_type<
	 (sizeof(unsigned long) >= sizeof(void*)),
	 unsigned long, unsigned long long>::__type _UIntPtrType;
#else
      typedef unsigned long _UIntPtrType;
#endif
       _UIntPtrType _M_diff;
    };

  /**
   * The specialization on this type helps resolve the problem of
   * reference to void, and eliminates the need to specialize
   * _Pointer_adapter for cases of void*, const void*, and so on.
   */
  struct _Invalid_type { };
  
  template<typename _Tp>
    struct _Reference_type 
    { typedef _Tp& reference; };

  template<> 
    struct _Reference_type<void> 
    { typedef _Invalid_type& reference; };

  template<> 
    struct _Reference_type<const void> 
    { typedef const _Invalid_type& reference; };

  template<> 
    struct _Reference_type<volatile void> 
    { typedef volatile _Invalid_type&  reference; };

  template<> 
    struct _Reference_type<volatile const void> 
    { typedef const volatile _Invalid_type&  reference; };

  /**
   * This structure accommodates the way in which
   * std::iterator_traits<> is normally specialized for const T*, so
   * that value_type is still T.
   */
  template<typename _Tp> 
    struct _Unqualified_type 
    { typedef _Tp type; };
    
  template<typename _Tp> 
    struct _Unqualified_type<const _Tp> 
    { typedef _Tp type; };
    
  /**
   * The following provides an 'alternative pointer' that works with
   * the containers when specified as the pointer typedef of the
   * allocator.
   *
   * The pointer type used with the containers doesn't have to be this
   * class, but it must support the implicit conversions, pointer
   * arithmetic, comparison operators, etc. that are supported by this
   * class, and avoid raising compile-time ambiguities.  Because
   * creating a working pointer can be challenging, this pointer
   * template was designed to wrapper an easier storage policy type,
   * so that it becomes reusable for creating other pointer types.
   *
   * A key point of this class is also that it allows container
   * writers to 'assume' Allocator::pointer is a typedef for a normal
   * pointer.  This class supports most of the conventions of a true
   * pointer, and can, for instance handle implicit conversion to
   * const and base class pointer types.  The only impositions on
   * container writers to support extended pointers are: 1) use the
   * Allocator::pointer typedef appropriately for pointer types.  2)
   * if you need pointer casting, use the __pointer_cast<> functions
   * from ext/cast.h.  This allows pointer cast operations to be
   * overloaded as necessary by custom pointers.
   *
   * Note: The const qualifier works with this pointer adapter as
   * follows:
   *
   * _Tp*             == _Pointer_adapter<_Std_pointer_impl<_Tp> >;
   * const _Tp*       == _Pointer_adapter<_Std_pointer_impl<const _Tp> >;
   * _Tp* const       == const _Pointer_adapter<_Std_pointer_impl<_Tp> >;
   * const _Tp* const == const _Pointer_adapter<_Std_pointer_impl<const _Tp> >;
   */
  template<typename _Storage_policy>
    class _Pointer_adapter : public _Storage_policy 
    {
    public:
      typedef typename _Storage_policy::element_type element_type;

      // These are needed for iterator_traits
      typedef std::random_access_iterator_tag                iterator_category;
      typedef typename _Unqualified_type<element_type>::type value_type;
      typedef std::ptrdiff_t                                 difference_type;
      typedef _Pointer_adapter                               pointer;
      typedef typename _Reference_type<element_type>::reference  reference;

      // Reminder: 'const' methods mean that the method is valid when the 
      // pointer is immutable, and has nothing to do with whether the 
      // 'pointee' is const.

      // Default Constructor (Convert from element_type*)
      _Pointer_adapter(element_type* __arg = 0)
      { _Storage_policy::set(__arg); }

      // Copy constructor from _Pointer_adapter of same type.
      _Pointer_adapter(const _Pointer_adapter& __arg) 
      { _Storage_policy::set(__arg.get()); }

      // Convert from _Up* if conversion to element_type* is valid.
      template<typename _Up>
        _Pointer_adapter(_Up* __arg)
        { _Storage_policy::set(__arg); }

      // Conversion from another _Pointer_adapter if _Up if static cast is
      // valid.
      template<typename _Up>
        _Pointer_adapter(const _Pointer_adapter<_Up>& __arg)
        { _Storage_policy::set(__arg.get()); }

      // Destructor
      ~_Pointer_adapter() { }
  
      // Assignment operator
      _Pointer_adapter&
      operator=(const _Pointer_adapter& __arg) 
      {
        _Storage_policy::set(__arg.get()); 
        return *this; 
      }

      template<typename _Up>
        _Pointer_adapter&
        operator=(const _Pointer_adapter<_Up>& __arg)
        {
          _Storage_policy::set(__arg.get()); 
          return *this; 
        }

      template<typename _Up>
        _Pointer_adapter&
        operator=(_Up* __arg)
        {
          _Storage_policy::set(__arg); 
          return *this; 
        }

      // Operator*, returns element_type&
      inline reference 
      operator*() const 
      { return *(_Storage_policy::get()); }

      // Operator->, returns element_type*
      inline element_type* 
      operator->() const 
      { return _Storage_policy::get(); }

      // Operator[], returns a element_type& to the item at that loc.
      inline reference
      operator[](std::ptrdiff_t __index) const
      { return _Storage_policy::get()[__index]; }

      // To allow implicit conversion to "bool", for "if (ptr)..."
#if __cplusplus >= 201103L
      explicit operator bool() const { return _Storage_policy::get() != 0; }
#else
    private:
      typedef element_type*(_Pointer_adapter::*__unspecified_bool_type)() const;

    public:
      operator __unspecified_bool_type() const
      {
        return _Storage_policy::get() == 0 ? 0 : 
                         &_Pointer_adapter::operator->; 
      }

      // ! operator (for: if (!ptr)...)
      inline bool
      operator!() const 
      { return (_Storage_policy::get() == 0); }
#endif
  
      // Pointer differences
      inline friend std::ptrdiff_t 
      operator-(const _Pointer_adapter& __lhs, element_type* __rhs) 
      { return (__lhs.get() - __rhs); }
  
      inline friend std::ptrdiff_t 
      operator-(element_type* __lhs, const _Pointer_adapter& __rhs) 
      { return (__lhs - __rhs.get()); }
  
      template<typename _Up>
        inline friend std::ptrdiff_t 
        operator-(const _Pointer_adapter& __lhs, _Up* __rhs) 
        { return (__lhs.get() - __rhs); }
    
      template<typename _Up>
        inline friend std::ptrdiff_t 
        operator-(_Up* __lhs, const _Pointer_adapter& __rhs)
        { return (__lhs - __rhs.get()); }

      template<typename _Up>
        inline std::ptrdiff_t 
        operator-(const _Pointer_adapter<_Up>& __rhs) const 
        { return (_Storage_policy::get() - __rhs.get()); }
  
      // Pointer math
      // Note: There is a reason for all this overloading based on different
      // integer types.  In some libstdc++-v3 test cases, a templated
      // operator+ is declared which can match any types.  This operator
      // tends to "steal" the recognition of _Pointer_adapter's own operator+ 
      // unless the integer type matches perfectly.

#define _CXX_POINTER_ARITH_OPERATOR_SET(INT_TYPE) \
      inline friend _Pointer_adapter \
      operator+(const _Pointer_adapter& __lhs, INT_TYPE __offset) \
      { return _Pointer_adapter(__lhs.get() + __offset); } \
\
      inline friend _Pointer_adapter \
      operator+(INT_TYPE __offset, const _Pointer_adapter& __rhs) \
      { return _Pointer_adapter(__rhs.get() + __offset); } \
\
      inline friend _Pointer_adapter \
      operator-(const _Pointer_adapter& __lhs, INT_TYPE __offset) \
      { return _Pointer_adapter(__lhs.get() - __offset); } \
\
      inline _Pointer_adapter& \
      operator+=(INT_TYPE __offset) \
      { \
        _Storage_policy::set(_Storage_policy::get() + __offset); \
        return *this; \
      } \
\
      inline _Pointer_adapter& \
      operator-=(INT_TYPE __offset) \
      { \
        _Storage_policy::set(_Storage_policy::get() - __offset); \
        return *this; \
      } \
// END of _CXX_POINTER_ARITH_OPERATOR_SET macro
  
      // Expand into the various pointer arithmetic operators needed.
      _CXX_POINTER_ARITH_OPERATOR_SET(short);
      _CXX_POINTER_ARITH_OPERATOR_SET(unsigned short);
      _CXX_POINTER_ARITH_OPERATOR_SET(int);
      _CXX_POINTER_ARITH_OPERATOR_SET(unsigned int);
      _CXX_POINTER_ARITH_OPERATOR_SET(long);
      _CXX_POINTER_ARITH_OPERATOR_SET(unsigned long);
#ifdef _GLIBCXX_USE_LONG_LONG
      _CXX_POINTER_ARITH_OPERATOR_SET(long long);
      _CXX_POINTER_ARITH_OPERATOR_SET(unsigned long long);
#endif

      // Mathematical Manipulators
      inline _Pointer_adapter& 
      operator++()
      {
        _Storage_policy::set(_Storage_policy::get() + 1); 
        return *this;
      }
  
      inline _Pointer_adapter 
      operator++(int)
      {
        _Pointer_adapter __tmp(*this);
        _Storage_policy::set(_Storage_policy::get() + 1);
        return __tmp;
      }
  
      inline _Pointer_adapter& 
      operator--() 
      {
        _Storage_policy::set(_Storage_policy::get() - 1); 
        return *this;
      }
  
      inline _Pointer_adapter
      operator--(int) 
      {
        _Pointer_adapter __tmp(*this);
        _Storage_policy::set(_Storage_policy::get() - 1);
        return __tmp;
      }

#if __cpp_lib_three_way_comparison
      friend std::strong_ordering
      operator<=>(const _Pointer_adapter& __lhs, const _Pointer_adapter& __rhs)
      noexcept
      { return __lhs.get() <=> __rhs.get(); }
#endif
    }; // class _Pointer_adapter


#define _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(OPERATOR) \
  template<typename _Tp1, typename _Tp2> \
    inline bool \
    operator OPERATOR(const _Pointer_adapter<_Tp1>& __lhs, _Tp2 __rhs) \
    { return __lhs.get() OPERATOR __rhs; } \
\
  template<typename _Tp1, typename _Tp2> \
    inline bool \
    operator OPERATOR(_Tp1 __lhs, const _Pointer_adapter<_Tp2>& __rhs) \
    { return __lhs OPERATOR __rhs.get(); } \
\
  template<typename _Tp1, typename _Tp2> \
    inline bool \
    operator OPERATOR(const _Pointer_adapter<_Tp1>& __lhs, \
                              const _Pointer_adapter<_Tp2>& __rhs) \
    { return __lhs.get() OPERATOR __rhs.get(); } \
\
// End GCC_CXX_POINTER_COMPARISON_OPERATION_SET Macro
  
  // Expand into the various comparison operators needed.
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(==)
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(!=)
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(<)
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(<=)
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(>)
  _GCC_CXX_POINTER_COMPARISON_OPERATION_SET(>=)

  // These are here for expressions like "ptr == 0", "ptr != 0"
  template<typename _Tp>
    inline bool
    operator==(const _Pointer_adapter<_Tp>& __lhs, int __rhs)
    { return __lhs.get() == reinterpret_cast<void*>(__rhs); } 

  template<typename _Tp>
    inline bool
    operator==(int __lhs, const _Pointer_adapter<_Tp>& __rhs)
    { return __rhs.get() == reinterpret_cast<void*>(__lhs); } 

  template<typename _Tp>
    inline bool
    operator!=(const _Pointer_adapter<_Tp>& __lhs, int __rhs)
    { return __lhs.get() != reinterpret_cast<void*>(__rhs); } 

  template<typename _Tp>
    inline bool
    operator!=(int __lhs, const _Pointer_adapter<_Tp>& __rhs)
    { return __rhs.get() != reinterpret_cast<void*>(__lhs); } 

  /**
   * Comparison operators for _Pointer_adapter defer to the base class'
   * comparison operators, when possible.
   */
  template<typename _Tp>
    inline bool
    operator==(const _Pointer_adapter<_Tp>& __lhs, 
               const _Pointer_adapter<_Tp>& __rhs)
    { return __lhs._Tp::operator==(__rhs); }

  template<typename _Tp>
    inline bool
    operator<=(const _Pointer_adapter<_Tp>& __lhs, 
               const _Pointer_adapter<_Tp>& __rhs)
    { return __lhs._Tp::operator<(__rhs) || __lhs._Tp::operator==(__rhs); }

  template<typename _Tp>
    inline bool
    operator!=(const _Pointer_adapter<_Tp>& __lhs, 
               const _Pointer_adapter<_Tp>& __rhs)
    { return !(__lhs._Tp::operator==(__rhs)); }

  template<typename _Tp>
    inline bool
    operator>(const _Pointer_adapter<_Tp>& __lhs, 
              const _Pointer_adapter<_Tp>& __rhs)
    { return !(__lhs._Tp::operator<(__rhs) || __lhs._Tp::operator==(__rhs)); }

  template<typename _Tp>
    inline bool
    operator>=(const _Pointer_adapter<_Tp>& __lhs, 
               const _Pointer_adapter<_Tp>& __rhs)
    { return !(__lhs._Tp::operator<(__rhs)); }

  template<typename _CharT, typename _Traits, typename _StoreT>
    inline std::basic_ostream<_CharT, _Traits>&
    operator<<(std::basic_ostream<_CharT, _Traits>& __os, 
               const _Pointer_adapter<_StoreT>& __p)
    { return (__os << __p.get()); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#if __cplusplus >= 201103L
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  template<typename _Storage_policy>
    struct pointer_traits<__gnu_cxx::_Pointer_adapter<_Storage_policy>>
    {
      /// The pointer type
      typedef __gnu_cxx::_Pointer_adapter<_Storage_policy>         pointer;
      /// The type pointed to
      typedef typename pointer::element_type            element_type;
      /// Type used to represent the difference between two pointers
      typedef typename pointer::difference_type         difference_type;

      template<typename _Up>
        using rebind = typename __gnu_cxx::_Pointer_adapter<
	typename pointer_traits<_Storage_policy>::template rebind<_Up>>;

      static pointer pointer_to(typename pointer::reference __r) noexcept
      { return pointer(std::addressof(__r)); }
    };

#if __cpp_lib_concepts
  template<typename _Policy>
    struct indirectly_readable_traits<__gnu_cxx::_Pointer_adapter<_Policy>>
    {
      using value_type
	= typename __gnu_cxx::_Pointer_adapter<_Policy>::value_type;
    };
#endif
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
#endif

#endif // _POINTER_H
