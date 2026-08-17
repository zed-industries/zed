// Temporary buffer implementation -*- C++ -*-

// Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

/*
 *
 * Copyright (c) 1994
 * Hewlett-Packard Company
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Hewlett-Packard Company makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 *
 * Copyright (c) 1996,1997
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 */

/** @file bits/stl_tempbuf.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{memory}
 */

#ifndef _STL_TEMPBUF_H
#define _STL_TEMPBUF_H 1

#include <bits/stl_algobase.h>
#include <bits/stl_construct.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  namespace __detail
  {
    template<typename _Tp>
      inline void
      __return_temporary_buffer(_Tp* __p,
				size_t __len __attribute__((__unused__)))
      {
#if __cpp_sized_deallocation
	::operator delete(__p, __len * sizeof(_Tp));
#else
	::operator delete(__p);
#endif
      }
  }

  /**
   *  @brief Allocates a temporary buffer.
   *  @param  __len  The number of objects of type Tp.
   *  @return See full description.
   *
   *  Reinventing the wheel, but this time with prettier spokes!
   *
   *  This function tries to obtain storage for @c __len adjacent Tp
   *  objects.  The objects themselves are not constructed, of course.
   *  A pair<> is returned containing <em>the buffer s address and
   *  capacity (in the units of sizeof(_Tp)), or a pair of 0 values if
   *  no storage can be obtained.</em>  Note that the capacity obtained
   *  may be less than that requested if the memory is unavailable;
   *  you should compare len with the .second return value.
   *
   * Provides the nothrow exception guarantee.
   */
  template<typename _Tp>
    pair<_Tp*, ptrdiff_t>
    get_temporary_buffer(ptrdiff_t __len) _GLIBCXX_NOEXCEPT
    {
      const ptrdiff_t __max =
	__gnu_cxx::__numeric_traits<ptrdiff_t>::__max / sizeof(_Tp);
      if (__len > __max)
	__len = __max;

      while (__len > 0)
	{
	  _Tp* __tmp = static_cast<_Tp*>(::operator new(__len * sizeof(_Tp),
							std::nothrow));
	  if (__tmp != 0)
	    return std::pair<_Tp*, ptrdiff_t>(__tmp, __len);
	  __len /= 2;
	}
      return std::pair<_Tp*, ptrdiff_t>(static_cast<_Tp*>(0), 0);
    }

  /**
   *  @brief The companion to get_temporary_buffer().
   *  @param  __p  A buffer previously allocated by get_temporary_buffer.
   *  @return   None.
   *
   *  Frees the memory pointed to by __p.
   */
  template<typename _Tp>
    inline void
    return_temporary_buffer(_Tp* __p)
    { ::operator delete(__p); }

  /**
   *  This class is used in two places: stl_algo.h and ext/memory,
   *  where it is wrapped as the temporary_buffer class.  See
   *  temporary_buffer docs for more notes.
   */
  template<typename _ForwardIterator, typename _Tp>
    class _Temporary_buffer
    {
      // concept requirements
      __glibcxx_class_requires(_ForwardIterator, _ForwardIteratorConcept)

    public:
      typedef _Tp         value_type;
      typedef value_type* pointer;
      typedef pointer     iterator;
      typedef ptrdiff_t   size_type;

    protected:
      size_type  _M_original_len;
      size_type  _M_len;
      pointer    _M_buffer;

    public:
      /// As per Table mumble.
      size_type
      size() const
      { return _M_len; }

      /// Returns the size requested by the constructor; may be >size().
      size_type
      requested_size() const
      { return _M_original_len; }

      /// As per Table mumble.
      iterator
      begin()
      { return _M_buffer; }

      /// As per Table mumble.
      iterator
      end()
      { return _M_buffer + _M_len; }

      /**
       * Constructs a temporary buffer of a size somewhere between
       * zero and the given length.
       */
      _Temporary_buffer(_ForwardIterator __seed, size_type __original_len);

      ~_Temporary_buffer()
      {
	std::_Destroy(_M_buffer, _M_buffer + _M_len);
	std::__detail::__return_temporary_buffer(_M_buffer, _M_len);
      }

    private:
      // Disable copy constructor and assignment operator.
      _Temporary_buffer(const _Temporary_buffer&);

      void
      operator=(const _Temporary_buffer&);
    };


  template<bool>
    struct __uninitialized_construct_buf_dispatch
    {
      template<typename _Pointer, typename _ForwardIterator>
        static void
        __ucr(_Pointer __first, _Pointer __last,
	      _ForwardIterator __seed)
        {
	  if (__first == __last)
	    return;

	  _Pointer __cur = __first;
	  __try
	    {
	      std::_Construct(std::__addressof(*__first),
			      _GLIBCXX_MOVE(*__seed));
	      _Pointer __prev = __cur;
	      ++__cur;
	      for(; __cur != __last; ++__cur, ++__prev)
		std::_Construct(std::__addressof(*__cur),
				_GLIBCXX_MOVE(*__prev));
	      *__seed = _GLIBCXX_MOVE(*__prev);
	    }
	  __catch(...)
	    {
	      std::_Destroy(__first, __cur);
	      __throw_exception_again;
	    }
	}
    };

  template<>
    struct __uninitialized_construct_buf_dispatch<true>
    {
      template<typename _Pointer, typename _ForwardIterator>
        static void
        __ucr(_Pointer, _Pointer, _ForwardIterator) { }
    };

  // Constructs objects in the range [first, last).
  // Note that while these new objects will take valid values,
  // their exact value is not defined. In particular they may
  // be 'moved from'.
  //
  // While *__seed may be altered during this algorithm, it will have
  // the same value when the algorithm finishes, unless one of the
  // constructions throws.
  //
  // Requirements: _Pointer::value_type(_Tp&&) is valid.
  template<typename _Pointer, typename _ForwardIterator>
    inline void
    __uninitialized_construct_buf(_Pointer __first, _Pointer __last,
				  _ForwardIterator __seed)
    {
      typedef typename std::iterator_traits<_Pointer>::value_type
	_ValueType;

      std::__uninitialized_construct_buf_dispatch<
        __has_trivial_constructor(_ValueType)>::
	  __ucr(__first, __last, __seed);
    }

  template<typename _ForwardIterator, typename _Tp>
    _Temporary_buffer<_ForwardIterator, _Tp>::
    _Temporary_buffer(_ForwardIterator __seed, size_type __original_len)
    : _M_original_len(__original_len), _M_len(0), _M_buffer(0)
    {
      std::pair<pointer, size_type> __p(
		std::get_temporary_buffer<value_type>(_M_original_len));

      if (__p.first)
	{
	  __try
	    {
	      std::__uninitialized_construct_buf(__p.first, __p.first + __p.second,
						 __seed);
	      _M_buffer = __p.first;
	      _M_len = __p.second;
	    }
	  __catch(...)
	    {
	      std::__detail::__return_temporary_buffer(__p.first, __p.second);
	      __throw_exception_again;
	    }
	}
    }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _STL_TEMPBUF_H */
