// Stream iterators

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

/** @file bits/stream_iterator.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iterator}
 */

#ifndef _STREAM_ITERATOR_H
#define _STREAM_ITERATOR_H 1

#pragma GCC system_header

#include <debug/debug.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup iterators
   * @{
   */

  /// Provides input iterator semantics for streams.
  template<typename _Tp, typename _CharT = char,
           typename _Traits = char_traits<_CharT>, typename _Dist = ptrdiff_t>
    class istream_iterator
    : public iterator<input_iterator_tag, _Tp, _Dist, const _Tp*, const _Tp&>
    {
    public:
      typedef _CharT                         char_type;
      typedef _Traits                        traits_type;
      typedef basic_istream<_CharT, _Traits> istream_type;

    private:
      istream_type*	_M_stream;
      _Tp		_M_value;
      // This bool becomes false at end-of-stream. It should be sufficient to
      // check _M_stream != nullptr instead, but historically we did not set
      // _M_stream to null when reaching the end, so we need to keep this flag.
      bool		_M_ok;

    public:
      ///  Construct end of input stream iterator.
      _GLIBCXX_CONSTEXPR istream_iterator()
      : _M_stream(0), _M_value(), _M_ok(false) {}

      ///  Construct start of input stream iterator.
      istream_iterator(istream_type& __s)
      : _M_stream(std::__addressof(__s)), _M_ok(true)
      { _M_read(); }

      istream_iterator(const istream_iterator& __obj)
      : _M_stream(__obj._M_stream), _M_value(__obj._M_value),
        _M_ok(__obj._M_ok)
      { }

#if __cplusplus > 201703L && __cpp_lib_concepts
      constexpr
      istream_iterator(default_sentinel_t)
      noexcept(is_nothrow_default_constructible_v<_Tp>)
      : istream_iterator() { }
#endif

#if __cplusplus >= 201103L
      istream_iterator& operator=(const istream_iterator&) = default;
      ~istream_iterator() = default;
#endif

      const _Tp&
      operator*() const
      {
	__glibcxx_requires_cond(_M_ok,
				_M_message(__gnu_debug::__msg_deref_istream)
				._M_iterator(*this));
	return _M_value;
      }

      const _Tp*
      operator->() const { return std::__addressof((operator*())); }

      istream_iterator&
      operator++()
      {
	__glibcxx_requires_cond(_M_ok,
				_M_message(__gnu_debug::__msg_inc_istream)
				._M_iterator(*this));
	_M_read();
	return *this;
      }

      istream_iterator
      operator++(int)
      {
	__glibcxx_requires_cond(_M_ok,
				_M_message(__gnu_debug::__msg_inc_istream)
				._M_iterator(*this));
	istream_iterator __tmp = *this;
	_M_read();
	return __tmp;
      }

    private:
      bool
      _M_equal(const istream_iterator& __x) const
      {
	// Ideally this would just return _M_stream == __x._M_stream,
	// but code compiled with old versions never sets _M_stream to null.
	return (_M_ok == __x._M_ok) && (!_M_ok || _M_stream == __x._M_stream);
      }

      void
      _M_read()
      {
        if (_M_stream && !(*_M_stream >> _M_value))
          {
            _M_stream = 0;
            _M_ok = false;
          }
      }

      /// Return true if the iterators refer to the same stream,
      /// or are both at end-of-stream.
      friend bool
      operator==(const istream_iterator& __x, const istream_iterator& __y)
      { return __x._M_equal(__y); }

      /// Return true if the iterators refer to different streams,
      /// or if one is at end-of-stream and the other is not.
      friend bool
      operator!=(const istream_iterator& __x, const istream_iterator& __y)
      { return !__x._M_equal(__y); }

#if __cplusplus > 201703L && __cpp_lib_concepts
      friend bool
      operator==(const istream_iterator& __i, default_sentinel_t)
      { return !__i._M_stream; }
#endif
    };

  /**
   *  @brief  Provides output iterator semantics for streams.
   *
   *  This class provides an iterator to write to an ostream.  The type Tp is
   *  the only type written by this iterator and there must be an
   *  operator<<(Tp) defined.
   *
   *  @tparam  _Tp  The type to write to the ostream.
   *  @tparam  _CharT  The ostream char_type.
   *  @tparam  _Traits  The ostream char_traits.
  */
  template<typename _Tp, typename _CharT = char,
           typename _Traits = char_traits<_CharT> >
    class ostream_iterator
    : public iterator<output_iterator_tag, void, void, void, void>
    {
    public:
      //@{
      /// Public typedef
#if __cplusplus > 201703L
      using difference_type = ptrdiff_t;
#endif
      typedef _CharT                         char_type;
      typedef _Traits                        traits_type;
      typedef basic_ostream<_CharT, _Traits> ostream_type;
      //@}

    private:
      ostream_type*	_M_stream;
      const _CharT*	_M_string;

    public:
#if __cplusplus > 201703L
      constexpr ostream_iterator() noexcept
      : _M_stream(nullptr), _M_string(nullptr) { }
#endif

      /// Construct from an ostream.
      ostream_iterator(ostream_type& __s)
      : _M_stream(std::__addressof(__s)), _M_string(0) {}

      /**
       *  Construct from an ostream.
       *
       *  The delimiter string @a c is written to the stream after every Tp
       *  written to the stream.  The delimiter is not copied, and thus must
       *  not be destroyed while this iterator is in use.
       *
       *  @param  __s  Underlying ostream to write to.
       *  @param  __c  CharT delimiter string to insert.
      */
      ostream_iterator(ostream_type& __s, const _CharT* __c)
      : _M_stream(std::__addressof(__s)), _M_string(__c)  { }

      /// Copy constructor.
      ostream_iterator(const ostream_iterator& __obj)
      : _M_stream(__obj._M_stream), _M_string(__obj._M_string)  { }

#if __cplusplus >= 201103L
      ostream_iterator& operator=(const ostream_iterator&) = default;
#endif

      /// Writes @a value to underlying ostream using operator<<.  If
      /// constructed with delimiter string, writes delimiter to ostream.
      ostream_iterator&
      operator=(const _Tp& __value)
      {
	__glibcxx_requires_cond(_M_stream != 0,
				_M_message(__gnu_debug::__msg_output_ostream)
				._M_iterator(*this));
	*_M_stream << __value;
	if (_M_string)
          *_M_stream << _M_string;
	return *this;
      }

      ostream_iterator&
      operator*()
      { return *this; }

      ostream_iterator&
      operator++()
      { return *this; }

      ostream_iterator&
      operator++(int)
      { return *this; }
    };

  // @} group iterators

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
