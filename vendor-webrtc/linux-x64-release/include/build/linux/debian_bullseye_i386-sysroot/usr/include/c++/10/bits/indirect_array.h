// The template and inlines for the -*- C++ -*- indirect_array class.

// Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/** @file bits/indirect_array.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{valarray}
 */

// Written by Gabriel Dos Reis <Gabriel.Dos-Reis@DPTMaths.ENS-Cachan.Fr>

#ifndef _INDIRECT_ARRAY_H
#define _INDIRECT_ARRAY_H 1

#pragma GCC system_header

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup numeric_arrays
   * @{
   */

  /**
   *  @brief  Reference to arbitrary subset of an array.
   *
   *  An indirect_array is a reference to the actual elements of an array
   *  specified by an ordered array of indices.  The way to get an
   *  indirect_array is to call operator[](valarray<size_t>) on a valarray.
   *  The returned indirect_array then permits carrying operations out on the
   *  referenced subset of elements in the original valarray.
   *
   *  For example, if an indirect_array is obtained using the array (4,2,0) as
   *  an argument, and then assigned to an array containing (1,2,3), then the
   *  underlying array will have array[0]==3, array[2]==2, and array[4]==1.
   *
   *  @param  Tp  Element type.
   */
  template <class _Tp>
    class indirect_array
    {
    public:
      typedef _Tp value_type;

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 253. valarray helper functions are almost entirely useless

      ///  Copy constructor.  Both slices refer to the same underlying array.
      indirect_array(const indirect_array&);

      ///  Assignment operator.  Assigns elements to corresponding elements
      ///  of @a a.
      indirect_array& operator=(const indirect_array&);

      ///  Assign slice elements to corresponding elements of @a v.
      void operator=(const valarray<_Tp>&) const;
      ///  Multiply slice elements by corresponding elements of @a v.
      void operator*=(const valarray<_Tp>&) const;
      ///  Divide slice elements by corresponding elements of @a v.
      void operator/=(const valarray<_Tp>&) const;
      ///  Modulo slice elements by corresponding elements of @a v.
      void operator%=(const valarray<_Tp>&) const;
      ///  Add corresponding elements of @a v to slice elements.
      void operator+=(const valarray<_Tp>&) const;
      ///  Subtract corresponding elements of @a v from slice elements.
      void operator-=(const valarray<_Tp>&) const;
      ///  Logical xor slice elements with corresponding elements of @a v.
      void operator^=(const valarray<_Tp>&) const;
      ///  Logical and slice elements with corresponding elements of @a v.
      void operator&=(const valarray<_Tp>&) const;
      ///  Logical or slice elements with corresponding elements of @a v.
      void operator|=(const valarray<_Tp>&) const;
      ///  Left shift slice elements by corresponding elements of @a v.
      void operator<<=(const valarray<_Tp>&) const;
      ///  Right shift slice elements by corresponding elements of @a v.
      void operator>>=(const valarray<_Tp>&) const;
      ///  Assign all slice elements to @a t.
      void operator= (const _Tp&) const;
      //    ~indirect_array();

      template<class _Dom>
      void operator=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator*=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator/=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator%=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator+=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator-=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator^=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator&=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator|=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator<<=(const _Expr<_Dom, _Tp>&) const;
      template<class _Dom>
      void operator>>=(const _Expr<_Dom, _Tp>&) const;

    private:
      ///  Copy constructor.  Both slices refer to the same underlying array.
      indirect_array(_Array<_Tp>, size_t, _Array<size_t>);

      friend class valarray<_Tp>;
      friend class gslice_array<_Tp>;

      const size_t	 _M_sz;
      const _Array<size_t> _M_index;
      const _Array<_Tp>	 _M_array;

      // not implemented
      indirect_array();
    };

  template<typename _Tp>
    inline
    indirect_array<_Tp>::indirect_array(const indirect_array<_Tp>& __a)
    : _M_sz(__a._M_sz), _M_index(__a._M_index), _M_array(__a._M_array) {}

  template<typename _Tp>
    inline
    indirect_array<_Tp>::indirect_array(_Array<_Tp> __a, size_t __s,
					_Array<size_t> __i)
    : _M_sz(__s), _M_index(__i), _M_array(__a) {}

  template<typename _Tp>
    inline indirect_array<_Tp>&
    indirect_array<_Tp>::operator=(const indirect_array<_Tp>& __a)
    {
      std::__valarray_copy(__a._M_array, _M_sz, __a._M_index, _M_array,
			   _M_index);
      return *this;
    }

  template<typename _Tp>
    inline void
    indirect_array<_Tp>::operator=(const _Tp& __t) const
    { std::__valarray_fill(_M_array, _M_index, _M_sz, __t); }

  template<typename _Tp>
    inline void
    indirect_array<_Tp>::operator=(const valarray<_Tp>& __v) const
    { std::__valarray_copy(_Array<_Tp>(__v), _M_sz, _M_array, _M_index); }

  template<typename _Tp>
    template<class _Dom>
      inline void
      indirect_array<_Tp>::operator=(const _Expr<_Dom, _Tp>& __e) const
      { std::__valarray_copy(__e, _M_sz, _M_array, _M_index); }

#undef _DEFINE_VALARRAY_OPERATOR
#define _DEFINE_VALARRAY_OPERATOR(_Op, _Name)				\
  template<typename _Tp>						\
    inline void								\
    indirect_array<_Tp>::operator _Op##=(const valarray<_Tp>& __v) const\
    {									\
      _Array_augmented_##_Name(_M_array, _M_index, _Array<_Tp>(__v), _M_sz); \
    }									\
									\
  template<typename _Tp>                                                \
    template<class _Dom>				                \
      inline void							\
      indirect_array<_Tp>::operator _Op##=(const _Expr<_Dom,_Tp>& __e) const\
      {									\
	_Array_augmented_##_Name(_M_array, _M_index, __e, _M_sz);	\
      }

_DEFINE_VALARRAY_OPERATOR(*, __multiplies)
_DEFINE_VALARRAY_OPERATOR(/, __divides)
_DEFINE_VALARRAY_OPERATOR(%, __modulus)
_DEFINE_VALARRAY_OPERATOR(+, __plus)
_DEFINE_VALARRAY_OPERATOR(-, __minus)
_DEFINE_VALARRAY_OPERATOR(^, __bitwise_xor)
_DEFINE_VALARRAY_OPERATOR(&, __bitwise_and)
_DEFINE_VALARRAY_OPERATOR(|, __bitwise_or)
_DEFINE_VALARRAY_OPERATOR(<<, __shift_left)
_DEFINE_VALARRAY_OPERATOR(>>, __shift_right)

#undef _DEFINE_VALARRAY_OPERATOR

  // @} group numeric_arrays

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _INDIRECT_ARRAY_H */
