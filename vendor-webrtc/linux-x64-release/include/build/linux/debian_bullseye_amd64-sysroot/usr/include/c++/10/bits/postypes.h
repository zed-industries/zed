// Position types -*- C++ -*-

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

/** @file bits/postypes.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iosfwd}
 */

//
// ISO C++ 14882: 27.4.1 - Types
// ISO C++ 14882: 27.4.3 - Template class fpos
//

#ifndef _GLIBCXX_POSTYPES_H
#define _GLIBCXX_POSTYPES_H 1

#pragma GCC system_header

#include <cwchar> // For mbstate_t

// XXX If <stdint.h> is really needed, make sure to define the macros
// before including it, in order not to break <tr1/cstdint> (and <cstdint>
// in C++11).  Reconsider all this as soon as possible...
#if (defined(_GLIBCXX_HAVE_INT64_T) && !defined(_GLIBCXX_HAVE_INT64_T_LONG) \
     && !defined(_GLIBCXX_HAVE_INT64_T_LONG_LONG))

#ifndef __STDC_LIMIT_MACROS
# define _UNDEF__STDC_LIMIT_MACROS
# define __STDC_LIMIT_MACROS
#endif
#ifndef __STDC_CONSTANT_MACROS
# define _UNDEF__STDC_CONSTANT_MACROS
# define __STDC_CONSTANT_MACROS
#endif
#include <stdint.h> // For int64_t
#ifdef _UNDEF__STDC_LIMIT_MACROS
# undef __STDC_LIMIT_MACROS
# undef _UNDEF__STDC_LIMIT_MACROS
#endif
#ifdef _UNDEF__STDC_CONSTANT_MACROS
# undef __STDC_CONSTANT_MACROS
# undef _UNDEF__STDC_CONSTANT_MACROS
#endif

#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // The types streamoff, streampos and wstreampos and the class
  // template fpos<> are described in clauses 21.1.2, 21.1.3, 27.1.2,
  // 27.2, 27.4.1, 27.4.3 and D.6. Despite all this verbiage, the
  // behaviour of these types is mostly implementation defined or
  // unspecified. The behaviour in this implementation is as noted
  // below.

  /**
   *  @brief  Type used by fpos, char_traits<char>, and char_traits<wchar_t>.
   *
   *  In clauses 21.1.3.1 and 27.4.1 streamoff is described as an
   *  implementation defined type.
   *  Note: In versions of GCC up to and including GCC 3.3, streamoff
   *  was typedef long.
  */  
#ifdef _GLIBCXX_HAVE_INT64_T_LONG
  typedef long          streamoff;
#elif defined(_GLIBCXX_HAVE_INT64_T_LONG_LONG)
  typedef long long     streamoff;
#elif defined(_GLIBCXX_HAVE_INT64_T) 
  typedef int64_t       streamoff;
#else
  typedef long long     streamoff;
#endif

  /// Integral type for I/O operation counts and buffer sizes.
  typedef ptrdiff_t	streamsize; // Signed integral type

  /**
   *  @brief  Class representing stream positions.
   *
   *  The standard places no requirements upon the template parameter StateT.
   *  In this implementation StateT must be DefaultConstructible,
   *  CopyConstructible and Assignable.  The standard only requires that fpos
   *  should contain a member of type StateT. In this implementation it also
   *  contains an offset stored as a signed integer.
   *
   *  @param  StateT  Type passed to and returned from state().
   */
  template<typename _StateT>
    class fpos
    {
    private:
      streamoff	                _M_off;
      _StateT			_M_state;

    public:
      // The standard doesn't require that fpos objects can be default
      // constructed. This implementation provides a default
      // constructor that initializes the offset to 0 and default
      // constructs the state.
      fpos()
      : _M_off(0), _M_state() { }

      // The standard requires that fpos objects can be constructed
      // from streamoff objects using the constructor syntax, and
      // fails to give any meaningful semantics. In this
      // implementation implicit conversion is also allowed, and this
      // constructor stores the streamoff as the offset and default
      // constructs the state.
      /// Construct position from offset.
      fpos(streamoff __off)
      : _M_off(__off), _M_state() { }

#if __cplusplus >= 201103L
      fpos(const fpos&) = default;
      fpos& operator=(const fpos&) = default;
      ~fpos() = default;
#endif

      /// Convert to streamoff.
      operator streamoff() const { return _M_off; }

      /// Remember the value of @a st.
      void
      state(_StateT __st)
      { _M_state = __st; }

      /// Return the last set value of @a st.
      _StateT
      state() const
      { return _M_state; }

      // The standard requires that this operator must be defined, but
      // gives no semantics. In this implementation it just adds its
      // argument to the stored offset and returns *this.
      /// Add offset to this position.
      fpos&
      operator+=(streamoff __off)
      {
	_M_off += __off;
	return *this;
      }

      // The standard requires that this operator must be defined, but
      // gives no semantics. In this implementation it just subtracts
      // its argument from the stored offset and returns *this.
      /// Subtract offset from this position.
      fpos&
      operator-=(streamoff __off)
      {
	_M_off -= __off;
	return *this;
      }

      // The standard requires that this operator must be defined, but
      // defines its semantics only in terms of operator-. In this
      // implementation it constructs a copy of *this, adds the
      // argument to that copy using operator+= and then returns the
      // copy.
      /// Add position and offset.
      fpos
      operator+(streamoff __off) const
      {
	fpos __pos(*this);
	__pos += __off;
	return __pos;
      }

      // The standard requires that this operator must be defined, but
      // defines its semantics only in terms of operator+. In this
      // implementation it constructs a copy of *this, subtracts the
      // argument from that copy using operator-= and then returns the
      // copy.
      /// Subtract offset from position.
      fpos
      operator-(streamoff __off) const
      {
	fpos __pos(*this);
	__pos -= __off;
	return __pos;
      }

      // The standard requires that this operator must be defined, but
      // defines its semantics only in terms of operator+. In this
      // implementation it returns the difference between the offset
      // stored in *this and in the argument.
      /// Subtract position to return offset.
      streamoff
      operator-(const fpos& __other) const
      { return _M_off - __other._M_off; }
    };

  // The standard only requires that operator== must be an
  // equivalence relation. In this implementation two fpos<StateT>
  // objects belong to the same equivalence class if the contained
  // offsets compare equal.
  /// Test if equivalent to another position.
  template<typename _StateT>
    inline bool
    operator==(const fpos<_StateT>& __lhs, const fpos<_StateT>& __rhs)
    { return streamoff(__lhs) == streamoff(__rhs); }

  template<typename _StateT>
    inline bool
    operator!=(const fpos<_StateT>& __lhs, const fpos<_StateT>& __rhs)
    { return streamoff(__lhs) != streamoff(__rhs); }

  // Clauses 21.1.3.1 and 21.1.3.2 describe streampos and wstreampos
  // as implementation defined types, but clause 27.2 requires that
  // they must both be typedefs for fpos<mbstate_t>
  /// File position for char streams.
  typedef fpos<mbstate_t> streampos;
  /// File position for wchar_t streams.
  typedef fpos<mbstate_t> wstreampos;

#ifdef _GLIBCXX_USE_CHAR8_T
  /// File position for char8_t streams.
  typedef fpos<mbstate_t> u8streampos;
#endif

#if __cplusplus >= 201103L
  /// File position for char16_t streams.
  typedef fpos<mbstate_t> u16streampos;
  /// File position for char32_t streams.
  typedef fpos<mbstate_t> u32streampos;
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
