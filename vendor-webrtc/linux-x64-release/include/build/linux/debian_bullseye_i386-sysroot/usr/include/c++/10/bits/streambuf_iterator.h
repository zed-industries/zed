// Streambuf iterators

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

/** @file bits/streambuf_iterator.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{iterator}
 */

#ifndef _STREAMBUF_ITERATOR_H
#define _STREAMBUF_ITERATOR_H 1

#pragma GCC system_header

#include <streambuf>
#include <debug/debug.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @addtogroup iterators
   * @{
   */

  // 24.5.3 Template class istreambuf_iterator
  /// Provides input iterator semantics for streambufs.
  template<typename _CharT, typename _Traits>
    class istreambuf_iterator
    : public iterator<input_iterator_tag, _CharT, typename _Traits::off_type,
		      _CharT*, _CharT>
    {
    public:
      // Types:
      //@{
      /// Public typedefs
#if __cplusplus < 201103L
      typedef _CharT& reference; // Changed to _CharT by LWG 445
#elif __cplusplus > 201703L
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 3188. istreambuf_iterator::pointer should not be unspecified
      using pointer = void;
#endif

      typedef _CharT					char_type;
      typedef _Traits					traits_type;
      typedef typename _Traits::int_type		int_type;
      typedef basic_streambuf<_CharT, _Traits>		streambuf_type;
      typedef basic_istream<_CharT, _Traits>		istream_type;
      //@}

      template<typename _CharT2>
	friend typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value,
				    ostreambuf_iterator<_CharT2> >::__type
	copy(istreambuf_iterator<_CharT2>, istreambuf_iterator<_CharT2>,
	     ostreambuf_iterator<_CharT2>);

      template<bool _IsMove, typename _CharT2>
	friend typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value,
					       _CharT2*>::__type
	__copy_move_a2(istreambuf_iterator<_CharT2>,
		       istreambuf_iterator<_CharT2>, _CharT2*);

#if __cplusplus >= 201103L
      template<typename _CharT2, typename _Size>
	friend __enable_if_t<__is_char<_CharT2>::__value, _CharT2*>
	__copy_n_a(istreambuf_iterator<_CharT2>, _Size, _CharT2*);
#endif

      template<typename _CharT2>
	friend typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value,
				    istreambuf_iterator<_CharT2> >::__type
	find(istreambuf_iterator<_CharT2>, istreambuf_iterator<_CharT2>,
	     const _CharT2&);

      template<typename _CharT2, typename _Distance>
	friend typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value,
					       void>::__type
	advance(istreambuf_iterator<_CharT2>&, _Distance);

    private:
      // 24.5.3 istreambuf_iterator
      // p 1
      // If the end of stream is reached (streambuf_type::sgetc()
      // returns traits_type::eof()), the iterator becomes equal to
      // the "end of stream" iterator value.
      // NB: This implementation assumes the "end of stream" value
      // is EOF, or -1.
      mutable streambuf_type*	_M_sbuf;
      int_type			_M_c;

    public:
      ///  Construct end of input stream iterator.
      _GLIBCXX_CONSTEXPR istreambuf_iterator() _GLIBCXX_USE_NOEXCEPT
      : _M_sbuf(0), _M_c(traits_type::eof()) { }

#if __cplusplus > 201703L && __cpp_lib_concepts
      constexpr istreambuf_iterator(default_sentinel_t) noexcept
      : istreambuf_iterator() { }
#endif

#if __cplusplus >= 201103L
      istreambuf_iterator(const istreambuf_iterator&) noexcept = default;

      ~istreambuf_iterator() = default;
#endif

      ///  Construct start of input stream iterator.
      istreambuf_iterator(istream_type& __s) _GLIBCXX_USE_NOEXCEPT
      : _M_sbuf(__s.rdbuf()), _M_c(traits_type::eof()) { }

      ///  Construct start of streambuf iterator.
      istreambuf_iterator(streambuf_type* __s) _GLIBCXX_USE_NOEXCEPT
      : _M_sbuf(__s), _M_c(traits_type::eof()) { }

#if __cplusplus >= 201103L
      istreambuf_iterator&
      operator=(const istreambuf_iterator&) noexcept = default;
#endif

      ///  Return the current character pointed to by iterator.  This returns
      ///  streambuf.sgetc().  It cannot be assigned.  NB: The result of
      ///  operator*() on an end of stream is undefined.
      char_type
      operator*() const
      {
	int_type __c = _M_get();

#ifdef _GLIBCXX_DEBUG_PEDANTIC
	// Dereferencing a past-the-end istreambuf_iterator is a
	// libstdc++ extension
	__glibcxx_requires_cond(!_S_is_eof(__c),
				_M_message(__gnu_debug::__msg_deref_istreambuf)
				._M_iterator(*this));
#endif
	return traits_type::to_char_type(__c);
      }

      /// Advance the iterator.  Calls streambuf.sbumpc().
      istreambuf_iterator&
      operator++()
      {
	__glibcxx_requires_cond(_M_sbuf &&
				(!_S_is_eof(_M_c) || !_S_is_eof(_M_sbuf->sgetc())),
				_M_message(__gnu_debug::__msg_inc_istreambuf)
				._M_iterator(*this));

	_M_sbuf->sbumpc();
	_M_c = traits_type::eof();
	return *this;
      }

      /// Advance the iterator.  Calls streambuf.sbumpc().
      istreambuf_iterator
      operator++(int)
      {
	__glibcxx_requires_cond(_M_sbuf &&
				(!_S_is_eof(_M_c) || !_S_is_eof(_M_sbuf->sgetc())),
				_M_message(__gnu_debug::__msg_inc_istreambuf)
				._M_iterator(*this));

	istreambuf_iterator __old = *this;
	__old._M_c = _M_sbuf->sbumpc();
	_M_c = traits_type::eof();
	return __old;
      }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 110 istreambuf_iterator::equal not const
      // NB: there is also number 111 (NAD) relevant to this function.
      /// Return true both iterators are end or both are not end.
      bool
      equal(const istreambuf_iterator& __b) const
      { return _M_at_eof() == __b._M_at_eof(); }

    private:
      int_type
      _M_get() const
      {
	int_type __ret = _M_c;
	if (_M_sbuf && _S_is_eof(__ret) && _S_is_eof(__ret = _M_sbuf->sgetc()))
	  _M_sbuf = 0;
	return __ret;
      }

      bool
      _M_at_eof() const
      { return _S_is_eof(_M_get()); }

      static bool
      _S_is_eof(int_type __c)
      {
	const int_type __eof = traits_type::eof();
	return traits_type::eq_int_type(__c, __eof);
      }

#if __cplusplus > 201703L && __cpp_lib_concepts
      friend bool
      operator==(const istreambuf_iterator& __i, default_sentinel_t __s)
      { return __i._M_at_eof(); }
#endif
    };

  template<typename _CharT, typename _Traits>
    inline bool
    operator==(const istreambuf_iterator<_CharT, _Traits>& __a,
	       const istreambuf_iterator<_CharT, _Traits>& __b)
    { return __a.equal(__b); }

  template<typename _CharT, typename _Traits>
    inline bool
    operator!=(const istreambuf_iterator<_CharT, _Traits>& __a,
	       const istreambuf_iterator<_CharT, _Traits>& __b)
    { return !__a.equal(__b); }

  /// Provides output iterator semantics for streambufs.
  template<typename _CharT, typename _Traits>
    class ostreambuf_iterator
    : public iterator<output_iterator_tag, void, void, void, void>
    {
    public:
      // Types:
      //@{
      /// Public typedefs
#if __cplusplus > 201703L
      using difference_type = ptrdiff_t;
#endif
      typedef _CharT			       char_type;
      typedef _Traits			       traits_type;
      typedef basic_streambuf<_CharT, _Traits> streambuf_type;
      typedef basic_ostream<_CharT, _Traits>   ostream_type;
      //@}

      template<typename _CharT2>
	friend typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value,
				    ostreambuf_iterator<_CharT2> >::__type
	copy(istreambuf_iterator<_CharT2>, istreambuf_iterator<_CharT2>,
	     ostreambuf_iterator<_CharT2>);

    private:
      streambuf_type*	_M_sbuf;
      bool		_M_failed;

    public:

#if __cplusplus > 201703L
      constexpr
      ostreambuf_iterator() noexcept
      : _M_sbuf(nullptr), _M_failed(true) { }
#endif

      ///  Construct output iterator from ostream.
      ostreambuf_iterator(ostream_type& __s) _GLIBCXX_USE_NOEXCEPT
      : _M_sbuf(__s.rdbuf()), _M_failed(!_M_sbuf) { }

      ///  Construct output iterator from streambuf.
      ostreambuf_iterator(streambuf_type* __s) _GLIBCXX_USE_NOEXCEPT
      : _M_sbuf(__s), _M_failed(!_M_sbuf) { }

      ///  Write character to streambuf.  Calls streambuf.sputc().
      ostreambuf_iterator&
      operator=(_CharT __c)
      {
	if (!_M_failed &&
	    _Traits::eq_int_type(_M_sbuf->sputc(__c), _Traits::eof()))
	  _M_failed = true;
	return *this;
      }

      /// Return *this.
      ostreambuf_iterator&
      operator*()
      { return *this; }

      /// Return *this.
      ostreambuf_iterator&
      operator++(int)
      { return *this; }

      /// Return *this.
      ostreambuf_iterator&
      operator++()
      { return *this; }

      /// Return true if previous operator=() failed.
      bool
      failed() const _GLIBCXX_USE_NOEXCEPT
      { return _M_failed; }

      ostreambuf_iterator&
      _M_put(const _CharT* __ws, streamsize __len)
      {
	if (__builtin_expect(!_M_failed, true)
	    && __builtin_expect(this->_M_sbuf->sputn(__ws, __len) != __len,
				false))
	  _M_failed = true;
	return *this;
      }
    };

  // Overloads for streambuf iterators.
  template<typename _CharT>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
				    ostreambuf_iterator<_CharT> >::__type
    copy(istreambuf_iterator<_CharT> __first,
	 istreambuf_iterator<_CharT> __last,
	 ostreambuf_iterator<_CharT> __result)
    {
      if (__first._M_sbuf && !__last._M_sbuf && !__result._M_failed)
	{
	  bool __ineof;
	  __copy_streambufs_eof(__first._M_sbuf, __result._M_sbuf, __ineof);
	  if (!__ineof)
	    __result._M_failed = true;
	}
      return __result;
    }

  template<bool _IsMove, typename _CharT>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
				    ostreambuf_iterator<_CharT> >::__type
    __copy_move_a2(_CharT* __first, _CharT* __last,
		   ostreambuf_iterator<_CharT> __result)
    {
      const streamsize __num = __last - __first;
      if (__num > 0)
	__result._M_put(__first, __num);
      return __result;
    }

  template<bool _IsMove, typename _CharT>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
				    ostreambuf_iterator<_CharT> >::__type
    __copy_move_a2(const _CharT* __first, const _CharT* __last,
		   ostreambuf_iterator<_CharT> __result)
    {
      const streamsize __num = __last - __first;
      if (__num > 0)
	__result._M_put(__first, __num);
      return __result;
    }

  template<bool _IsMove, typename _CharT>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
				    _CharT*>::__type
    __copy_move_a2(istreambuf_iterator<_CharT> __first,
		   istreambuf_iterator<_CharT> __last, _CharT* __result)
    {
      typedef istreambuf_iterator<_CharT>		   __is_iterator_type;
      typedef typename __is_iterator_type::traits_type	   traits_type;
      typedef typename __is_iterator_type::streambuf_type  streambuf_type;
      typedef typename traits_type::int_type		   int_type;

      if (__first._M_sbuf && !__last._M_sbuf)
	{
	  streambuf_type* __sb = __first._M_sbuf;
	  int_type __c = __sb->sgetc();
	  while (!traits_type::eq_int_type(__c, traits_type::eof()))
	    {
	      const streamsize __n = __sb->egptr() - __sb->gptr();
	      if (__n > 1)
		{
		  traits_type::copy(__result, __sb->gptr(), __n);
		  __sb->__safe_gbump(__n);
		  __result += __n;
		  __c = __sb->underflow();
		}
	      else
		{
		  *__result++ = traits_type::to_char_type(__c);
		  __c = __sb->snextc();
		}
	    }
	}
      return __result;
    }

#if __cplusplus >= 201103L
  template<typename _CharT, typename _Size>
    __enable_if_t<__is_char<_CharT>::__value, _CharT*>
    __copy_n_a(istreambuf_iterator<_CharT> __it, _Size __n, _CharT* __result)
    {
      if (__n == 0)
	return __result;

      __glibcxx_requires_cond(__it._M_sbuf,
			      _M_message(__gnu_debug::__msg_inc_istreambuf)
			      ._M_iterator(__it));
      _CharT* __beg = __result;
      __result += __it._M_sbuf->sgetn(__beg, __n);
      __glibcxx_requires_cond(__result - __beg == __n,
			      _M_message(__gnu_debug::__msg_inc_istreambuf)
			      ._M_iterator(__it));
      return __result;
    }
#endif // C++11

  template<typename _CharT>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
		  		    istreambuf_iterator<_CharT> >::__type
    find(istreambuf_iterator<_CharT> __first,
	 istreambuf_iterator<_CharT> __last, const _CharT& __val)
    {
      typedef istreambuf_iterator<_CharT>		   __is_iterator_type;
      typedef typename __is_iterator_type::traits_type     traits_type;
      typedef typename __is_iterator_type::streambuf_type  streambuf_type;
      typedef typename traits_type::int_type		   int_type;
      const int_type __eof = traits_type::eof();

      if (__first._M_sbuf && !__last._M_sbuf)
	{
	  const int_type __ival = traits_type::to_int_type(__val);
	  streambuf_type* __sb = __first._M_sbuf;
	  int_type __c = __sb->sgetc();
	  while (!traits_type::eq_int_type(__c, __eof)
		 && !traits_type::eq_int_type(__c, __ival))
	    {
	      streamsize __n = __sb->egptr() - __sb->gptr();
	      if (__n > 1)
		{
		  const _CharT* __p = traits_type::find(__sb->gptr(),
							__n, __val);
		  if (__p)
		    __n = __p - __sb->gptr();
		  __sb->__safe_gbump(__n);
		  __c = __sb->sgetc();
		}
	      else
		__c = __sb->snextc();
	    }

	  __first._M_c = __eof;
	}

      return __first;
    }

  template<typename _CharT, typename _Distance>
    typename __gnu_cxx::__enable_if<__is_char<_CharT>::__value,
				    void>::__type
    advance(istreambuf_iterator<_CharT>& __i, _Distance __n)
    {
      if (__n == 0)
	return;

      __glibcxx_assert(__n > 0);
      __glibcxx_requires_cond(!__i._M_at_eof(),
			      _M_message(__gnu_debug::__msg_inc_istreambuf)
			      ._M_iterator(__i));

      typedef istreambuf_iterator<_CharT>		   __is_iterator_type;
      typedef typename __is_iterator_type::traits_type	   traits_type;
      typedef typename __is_iterator_type::streambuf_type  streambuf_type;
      typedef typename traits_type::int_type		   int_type;
      const int_type __eof = traits_type::eof();

      streambuf_type* __sb = __i._M_sbuf;
      while (__n > 0)
	{
	  streamsize __size = __sb->egptr() - __sb->gptr();
	  if (__size > __n)
	    {
	      __sb->__safe_gbump(__n);
	      break;
	    }

	  __sb->__safe_gbump(__size);
	  __n -= __size;
	  if (traits_type::eq_int_type(__sb->underflow(), __eof))
	    {
	      __glibcxx_requires_cond(__n == 0,
				_M_message(__gnu_debug::__msg_inc_istreambuf)
				._M_iterator(__i));
	      break;
	    }
	}

      __i._M_c = __eof;
    }

// @} group iterators

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif
