// The template and inlines for the -*- C++ -*- gslice class.

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

/** @file bits/gslice.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{valarray}
 */

// Written by Gabriel Dos Reis <Gabriel.Dos-Reis@DPTMaths.ENS-Cachan.Fr>

#ifndef _GSLICE_H
#define _GSLICE_H 1

#pragma GCC system_header

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup numeric_arrays
   * @{
   */

  /**
   *  @brief  Class defining multi-dimensional subset of an array.
   *
   *  The slice class represents a multi-dimensional subset of an array,
   *  specified by three parameter sets: start offset, size array, and stride
   *  array.  The start offset is the index of the first element of the array
   *  that is part of the subset.  The size and stride array describe each
   *  dimension of the slice.  Size is the number of elements in that
   *  dimension, and stride is the distance in the array between successive
   *  elements in that dimension.  Each dimension's size and stride is taken
   *  to begin at an array element described by the previous dimension.  The
   *  size array and stride array must be the same size.
   *
   *  For example, if you have offset==3, stride[0]==11, size[1]==3,
   *  stride[1]==3, then slice[0,0]==array[3], slice[0,1]==array[6],
   *  slice[0,2]==array[9], slice[1,0]==array[14], slice[1,1]==array[17],
   *  slice[1,2]==array[20].
   */
  class gslice
  {
  public:
    ///  Construct an empty slice.
    gslice();

    /**
     *  @brief  Construct a slice.
     *
     *  Constructs a slice with as many dimensions as the length of the @a l
     *  and @a s arrays.
     *
     *  @param  __o  Offset in array of first element.
     *  @param  __l  Array of dimension lengths.
     *  @param  __s  Array of dimension strides between array elements.
     */
    gslice(size_t __o, const valarray<size_t>& __l, 
	   const valarray<size_t>& __s);

    // XXX: the IS says the copy-ctor and copy-assignment operators are
    //      synthesized by the compiler but they are just unsuitable
    //      for a ref-counted semantic
    ///  Copy constructor.
    gslice(const gslice&);

    ///  Destructor.
    ~gslice();

    // XXX: See the note above.
    ///  Assignment operator.
    gslice& operator=(const gslice&);

    ///  Return array offset of first slice element.
    size_t           start() const;

    ///  Return array of sizes of slice dimensions.
    valarray<size_t> size() const;
    
    ///  Return array of array strides for each dimension.
    valarray<size_t> stride() const;

  private:
    struct _Indexer
    {
      size_t _M_count;
      size_t _M_start;
      valarray<size_t> _M_size;
      valarray<size_t> _M_stride;
      valarray<size_t> _M_index; // Linear array of referenced indices

      _Indexer()
      : _M_count(1), _M_start(0), _M_size(), _M_stride(), _M_index() {}

      _Indexer(size_t, const valarray<size_t>&,
	       const valarray<size_t>&);

      void
      _M_increment_use()
      { ++_M_count; }
      
      size_t
      _M_decrement_use()
      { return --_M_count; }
    };

    _Indexer* _M_index;

    template<typename _Tp> friend class valarray;
  };

  inline size_t
  gslice::start() const
  { return _M_index ? _M_index->_M_start : 0; }

  inline valarray<size_t>
  gslice::size() const
  { return _M_index ? _M_index->_M_size : valarray<size_t>(); }

  inline valarray<size_t>
  gslice::stride() const
  { return _M_index ? _M_index->_M_stride : valarray<size_t>(); }

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 543. valarray slice default constructor
  inline
  gslice::gslice()
  : _M_index(new gslice::_Indexer()) {}

  inline
  gslice::gslice(size_t __o, const valarray<size_t>& __l,
		 const valarray<size_t>& __s)
  : _M_index(new gslice::_Indexer(__o, __l, __s)) {}

  inline
  gslice::gslice(const gslice& __g)
  : _M_index(__g._M_index)
  { if (_M_index) _M_index->_M_increment_use(); }

  inline
  gslice::~gslice()
  {
    if (_M_index && _M_index->_M_decrement_use() == 0)
      delete _M_index;
  }

  inline gslice&
  gslice::operator=(const gslice& __g)
  {
    if (__g._M_index)
      __g._M_index->_M_increment_use();
    if (_M_index && _M_index->_M_decrement_use() == 0)
      delete _M_index;
    _M_index = __g._M_index;
    return *this;
  }

  // @} group numeric_arrays

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _GSLICE_H */
