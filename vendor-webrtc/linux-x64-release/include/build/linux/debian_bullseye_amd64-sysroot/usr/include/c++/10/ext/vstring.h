// Versatile string -*- C++ -*-

// Copyright (C) 2005-2020 Free Software Foundation, Inc.
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

/** @file ext/vstring.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _VSTRING_H
#define _VSTRING_H 1

#pragma GCC system_header

#if __cplusplus >= 201103L
#include <initializer_list>
#endif

#include <ext/vstring_util.h>
#include <ext/rc_string_base.h>
#include <ext/sso_string_base.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @class __versa_string vstring.h
   *  @brief  Template class __versa_string. 
   *  @ingroup extensions
   *
   *  Data structure managing sequences of characters and
   *  character-like objects. 
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    class __versa_string
    : private _Base<_CharT, _Traits, _Alloc>
    {
      typedef _Base<_CharT, _Traits, _Alloc>                __vstring_base;    
      typedef typename __vstring_base::_CharT_alloc_type    _CharT_alloc_type;
      typedef __alloc_traits<_CharT_alloc_type> _CharT_alloc_traits;

      // Types:
    public:
      typedef _Traits					    traits_type;
      typedef typename _Traits::char_type		    value_type;
      typedef _Alloc					    allocator_type;
      typedef typename _CharT_alloc_type::size_type	    size_type;
      typedef typename _CharT_alloc_type::difference_type   difference_type;
      typedef value_type&               	            reference;
      typedef const value_type&                             const_reference;
      typedef typename _CharT_alloc_traits::pointer	    pointer;
      typedef typename _CharT_alloc_traits::const_pointer   const_pointer;
      typedef __gnu_cxx::__normal_iterator<pointer, __versa_string>  iterator;
      typedef __gnu_cxx::__normal_iterator<const_pointer, __versa_string>
                                                            const_iterator;
      typedef std::reverse_iterator<const_iterator>	const_reverse_iterator;
      typedef std::reverse_iterator<iterator>		    reverse_iterator;

      // Data Member (public):
      ///  Value returned by various member functions when they fail.
      static const size_type	npos = static_cast<size_type>(-1);

    private:
      size_type
      _M_check(size_type __pos, const char* __s) const
      {
	if (__pos > this->size())
	  std::__throw_out_of_range_fmt(__N("%s: __pos (which is %zu) > "
					    "this->size() (which is %zu)"),
					__s, __pos, this->size());
	return __pos;
      }

      void
      _M_check_length(size_type __n1, size_type __n2, const char* __s) const
      {
	if (this->max_size() - (this->size() - __n1) < __n2)
	  std::__throw_length_error(__N(__s));
      }

      // NB: _M_limit doesn't check for a bad __pos value.
      size_type
      _M_limit(size_type __pos, size_type __off) const _GLIBCXX_NOEXCEPT
      {
	const bool __testoff =  __off < this->size() - __pos;
	return __testoff ? __off : this->size() - __pos;
      }

      // True if _Rep and source do not overlap.
      bool
      _M_disjunct(const _CharT* __s) const _GLIBCXX_NOEXCEPT
      {
	return (std::less<const _CharT*>()(__s, this->_M_data())
		|| std::less<const _CharT*>()(this->_M_data()
					      + this->size(), __s));
      }

      // For the internal use we have functions similar to `begin'/`end'
      // but they do not call _M_leak.
      iterator
      _M_ibegin() const _GLIBCXX_NOEXCEPT
      { return iterator(this->_M_data()); }

      iterator
      _M_iend() const _GLIBCXX_NOEXCEPT
      { return iterator(this->_M_data() + this->_M_length()); }

    public:
      // Construct/copy/destroy:
      // NB: We overload ctors in some cases instead of using default
      // arguments, per 17.4.4.4 para. 2 item 2.

      /**
       *  @brief  Construct an empty string using allocator @a a.
       */
      explicit
      __versa_string(const _Alloc& __a = _Alloc()) _GLIBCXX_NOEXCEPT
      : __vstring_base(__a) { }

      // NB: per LWG issue 42, semantics different from IS:
      /**
       *  @brief  Construct string with copy of value of @a __str.
       *  @param  __str  Source string.
       */
      __versa_string(const __versa_string& __str)
      : __vstring_base(__str) { }

#if __cplusplus >= 201103L
      /**
       *  @brief  String move constructor.
       *  @param  __str  Source string.
       *
       *  The newly-constructed %string contains the exact contents of
       *  @a __str.  The contents of @a __str are a valid, but unspecified
       *  string.
       */
      __versa_string(__versa_string&& __str) noexcept
      : __vstring_base(std::move(__str)) { }

      /**
       *  @brief  Construct string from an initializer list.
       *  @param  __l  std::initializer_list of characters.
       *  @param  __a  Allocator to use (default is default allocator).
       */
      __versa_string(std::initializer_list<_CharT> __l,
		     const _Alloc& __a = _Alloc())
      : __vstring_base(__l.begin(), __l.end(), __a) { }
#endif

      /**
       *  @brief  Construct string as copy of a substring.
       *  @param  __str  Source string.
       *  @param  __pos  Index of first character to copy from.
       *  @param  __n  Number of characters to copy (default remainder).
       */
      __versa_string(const __versa_string& __str, size_type __pos,
		     size_type __n = npos)
      : __vstring_base(__str._M_data()
		       + __str._M_check(__pos,
					"__versa_string::__versa_string"),
		       __str._M_data() + __str._M_limit(__pos, __n)
		       + __pos, _Alloc()) { }

      /**
       *  @brief  Construct string as copy of a substring.
       *  @param  __str  Source string.
       *  @param  __pos  Index of first character to copy from.
       *  @param  __n  Number of characters to copy.
       *  @param  __a  Allocator to use.
       */
      __versa_string(const __versa_string& __str, size_type __pos,
		     size_type __n, const _Alloc& __a)
      : __vstring_base(__str._M_data()
		       + __str._M_check(__pos,
					"__versa_string::__versa_string"),
		       __str._M_data() + __str._M_limit(__pos, __n)
		       + __pos, __a) { }

      /**
       *  @brief  Construct string initialized by a character array.
       *  @param  __s  Source character array.
       *  @param  __n  Number of characters to copy.
       *  @param  __a  Allocator to use (default is default allocator).
       *
       *  NB: @a __s must have at least @a __n characters, '\\0' has no special
       *  meaning.
       */
      __versa_string(const _CharT* __s, size_type __n,
		     const _Alloc& __a = _Alloc())
      : __vstring_base(__s, __s + __n, __a) { }

      /**
       *  @brief  Construct string as copy of a C string.
       *  @param  __s  Source C string.
       *  @param  __a  Allocator to use (default is default allocator).
       */
      __versa_string(const _CharT* __s, const _Alloc& __a = _Alloc())
      : __vstring_base(__s, __s ? __s + traits_type::length(__s) :
		       __s + npos, __a) { }

      /**
       *  @brief  Construct string as multiple characters.
       *  @param  __n  Number of characters.
       *  @param  __c  Character to use.
       *  @param  __a  Allocator to use (default is default allocator).
       */
      __versa_string(size_type __n, _CharT __c, const _Alloc& __a = _Alloc())
      : __vstring_base(__n, __c, __a) { }

      /**
       *  @brief  Construct string as copy of a range.
       *  @param  __beg  Start of range.
       *  @param  __end  End of range.
       *  @param  __a  Allocator to use (default is default allocator).
       */
#if __cplusplus >= 201103L
      template<class _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
#else
      template<class _InputIterator>
#endif
        __versa_string(_InputIterator __beg, _InputIterator __end,
		       const _Alloc& __a = _Alloc())
	: __vstring_base(__beg, __end, __a) { }

      /**
       *  @brief  Destroy the string instance.
       */
      ~__versa_string() _GLIBCXX_NOEXCEPT { }	

      /**
       *  @brief  Assign the value of @a str to this string.
       *  @param  __str  Source string.
       */
      __versa_string&
      operator=(const __versa_string& __str) 
      { return this->assign(__str); }

#if __cplusplus >= 201103L
      /**
       *  @brief  String move assignment operator.
       *  @param  __str  Source string.
       *
       *  The contents of @a __str are moved into this string (without
       *  copying).  @a __str is a valid, but unspecified string.
       */
      __versa_string&
      operator=(__versa_string&& __str) noexcept
      {
	// NB: DR 1204.
	this->swap(__str);
	return *this;
      }

      /**
       *  @brief  Set value to string constructed from initializer list.
       *  @param  __l  std::initializer_list.
       */
      __versa_string&
      operator=(std::initializer_list<_CharT> __l)
      {
	this->assign(__l.begin(), __l.end());
	return *this;
      }
#endif

      /**
       *  @brief  Copy contents of @a __s into this string.
       *  @param  __s  Source null-terminated string.
       */
      __versa_string&
      operator=(const _CharT* __s) 
      { return this->assign(__s); }

      /**
       *  @brief  Set value to string of length 1.
       *  @param  __c  Source character.
       *
       *  Assigning to a character makes this string length 1 and
       *  (*this)[0] == @a __c.
       */
      __versa_string&
      operator=(_CharT __c) 
      { 
	this->assign(1, __c); 
	return *this;
      }

      // Iterators:
      /**
       *  Returns a read/write iterator that points to the first character in
       *  the %string.  Unshares the string.
       */
      iterator
      begin() _GLIBCXX_NOEXCEPT
      {
	this->_M_leak();
	return iterator(this->_M_data());
      }

      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  character in the %string.
       */
      const_iterator
      begin() const _GLIBCXX_NOEXCEPT
      { return const_iterator(this->_M_data()); }

      /**
       *  Returns a read/write iterator that points one past the last
       *  character in the %string.  Unshares the string.
       */
      iterator
      end() _GLIBCXX_NOEXCEPT
      {
	this->_M_leak();
	return iterator(this->_M_data() + this->size());
      }

      /**
       *  Returns a read-only (constant) iterator that points one past the
       *  last character in the %string.
       */
      const_iterator
      end() const _GLIBCXX_NOEXCEPT
      { return const_iterator(this->_M_data() + this->size()); }

      /**
       *  Returns a read/write reverse iterator that points to the last
       *  character in the %string.  Iteration is done in reverse element
       *  order.  Unshares the string.
       */
      reverse_iterator
      rbegin() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(this->end()); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to the last character in the %string.  Iteration is done in
       *  reverse element order.
       */
      const_reverse_iterator
      rbegin() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(this->end()); }

      /**
       *  Returns a read/write reverse iterator that points to one before the
       *  first character in the %string.  Iteration is done in reverse
       *  element order.  Unshares the string.
       */
      reverse_iterator
      rend() _GLIBCXX_NOEXCEPT
      { return reverse_iterator(this->begin()); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to one before the first character in the %string.  Iteration
       *  is done in reverse element order.
       */
      const_reverse_iterator
      rend() const _GLIBCXX_NOEXCEPT
      { return const_reverse_iterator(this->begin()); }

#if __cplusplus >= 201103L
      /**
       *  Returns a read-only (constant) iterator that points to the first
       *  character in the %string.
       */
      const_iterator
      cbegin() const noexcept
      { return const_iterator(this->_M_data()); }

      /**
       *  Returns a read-only (constant) iterator that points one past the
       *  last character in the %string.
       */
      const_iterator
      cend() const noexcept
      { return const_iterator(this->_M_data() + this->size()); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to the last character in the %string.  Iteration is done in
       *  reverse element order.
       */
      const_reverse_iterator
      crbegin() const noexcept
      { return const_reverse_iterator(this->end()); }

      /**
       *  Returns a read-only (constant) reverse iterator that points
       *  to one before the first character in the %string.  Iteration
       *  is done in reverse element order.
       */
      const_reverse_iterator
      crend() const noexcept
      { return const_reverse_iterator(this->begin()); }
#endif

    public:
      // Capacity:
      ///  Returns the number of characters in the string, not including any
      ///  null-termination.
      size_type
      size() const _GLIBCXX_NOEXCEPT
      { return this->_M_length(); }

      ///  Returns the number of characters in the string, not including any
      ///  null-termination.
      size_type
      length() const _GLIBCXX_NOEXCEPT
      { return this->_M_length(); }

      /// Returns the size() of the largest possible %string.
      size_type
      max_size() const _GLIBCXX_NOEXCEPT
      { return this->_M_max_size(); }

      /**
       *  @brief  Resizes the %string to the specified number of characters.
       *  @param  __n  Number of characters the %string should contain.
       *  @param  __c  Character to fill any new elements.
       *
       *  This function will %resize the %string to the specified
       *  number of characters.  If the number is smaller than the
       *  %string's current size the %string is truncated, otherwise
       *  the %string is extended and new elements are set to @a __c.
       */
      void
      resize(size_type __n, _CharT __c);

      /**
       *  @brief  Resizes the %string to the specified number of characters.
       *  @param  __n  Number of characters the %string should contain.
       *
       *  This function will resize the %string to the specified
       *  length.  If the new size is smaller than the %string's
       *  current size the %string is truncated, otherwise the %string
       *  is extended and new characters are default-constructed.  For
       *  basic types such as char, this means setting them to 0.
       */
      void
      resize(size_type __n)
      { this->resize(__n, _CharT()); }

#if __cplusplus >= 201103L
      /// A non-binding request to reduce capacity() to size().
      void
      shrink_to_fit() noexcept
      {
	if (capacity() > size())
	  {
	    __try
	      { this->reserve(0); }
	    __catch(...)
	      { }
	  }
      }
#endif

      /**
       *  Returns the total number of characters that the %string can
       *  hold before needing to allocate more memory.
       */
      size_type
      capacity() const _GLIBCXX_NOEXCEPT
      { return this->_M_capacity(); }

      /**
       *  @brief  Attempt to preallocate enough memory for specified number of
       *          characters.
       *  @param  __res_arg  Number of characters required.
       *  @throw  std::length_error  If @a __res_arg exceeds @c max_size().
       *
       *  This function attempts to reserve enough memory for the
       *  %string to hold the specified number of characters.  If the
       *  number requested is more than max_size(), length_error is
       *  thrown.
       *
       *  The advantage of this function is that if optimal code is a
       *  necessity and the user can determine the string length that
       *  will be required, the user can reserve the memory in
       *  %advance, and thus prevent a possible reallocation of memory
       *  and copying of %string data.
       */
      void
      reserve(size_type __res_arg = 0)
      { this->_M_reserve(__res_arg); }

      /**
       *  Erases the string, making it empty.
       */
      void
      clear() _GLIBCXX_NOEXCEPT
      { this->_M_clear(); }

      /**
       *  Returns true if the %string is empty.  Equivalent to 
       *  <code>*this == ""</code>.
       */
      _GLIBCXX_NODISCARD bool
      empty() const _GLIBCXX_NOEXCEPT
      { return this->size() == 0; }

      // Element access:
      /**
       *  @brief  Subscript access to the data contained in the %string.
       *  @param  __pos  The index of the character to access.
       *  @return  Read-only (constant) reference to the character.
       *
       *  This operator allows for easy, array-style, data access.
       *  Note that data access with this operator is unchecked and
       *  out_of_range lookups are not defined. (For checked lookups
       *  see at().)
       */
      const_reference
      operator[] (size_type __pos) const _GLIBCXX_NOEXCEPT
      {
	__glibcxx_assert(__pos <= this->size());
	return this->_M_data()[__pos];
      }

      /**
       *  @brief  Subscript access to the data contained in the %string.
       *  @param  __pos  The index of the character to access.
       *  @return  Read/write reference to the character.
       *
       *  This operator allows for easy, array-style, data access.
       *  Note that data access with this operator is unchecked and
       *  out_of_range lookups are not defined. (For checked lookups
       *  see at().)  Unshares the string.
       */
      reference
      operator[](size_type __pos) _GLIBCXX_NOEXCEPT
      {
        // Allow pos == size() both in C++98 mode, as v3 extension,
	// and in C++11 mode.
	__glibcxx_assert(__pos <= this->size());
        // In pedantic mode be strict in C++98 mode.
	_GLIBCXX_DEBUG_PEDASSERT(__cplusplus >= 201103L
				 || __pos < this->size());
	this->_M_leak();
	return this->_M_data()[__pos];
      }

      /**
       *  @brief  Provides access to the data contained in the %string.
       *  @param __n The index of the character to access.
       *  @return  Read-only (const) reference to the character.
       *  @throw  std::out_of_range  If @a __n is an invalid index.
       *
       *  This function provides for safer data access.  The parameter
       *  is first checked that it is in the range of the string.  The
       *  function throws out_of_range if the check fails.
       */
      const_reference
      at(size_type __n) const
      {
	if (__n >= this->size())
	  std::__throw_out_of_range_fmt(__N("__versa_string::at: __n "
					    "(which is %zu) >= this->size() "
					    "(which is %zu)"),
					__n, this->size());
	return this->_M_data()[__n];
      }

      /**
       *  @brief  Provides access to the data contained in the %string.
       *  @param __n The index of the character to access.
       *  @return  Read/write reference to the character.
       *  @throw  std::out_of_range  If @a __n is an invalid index.
       *
       *  This function provides for safer data access.  The parameter
       *  is first checked that it is in the range of the string.  The
       *  function throws out_of_range if the check fails.  Success
       *  results in unsharing the string.
       */
      reference
      at(size_type __n)
      {
	if (__n >= this->size())
	  std::__throw_out_of_range_fmt(__N("__versa_string::at: __n "
					    "(which is %zu) >= this->size() "
					    "(which is %zu)"),
					__n, this->size());
	this->_M_leak();
	return this->_M_data()[__n];
      }

#if __cplusplus >= 201103L
      /**
       *  Returns a read/write reference to the data at the first
       *  element of the %string.
       */
      reference
      front() noexcept
      { return operator[](0); }

      /**
       *  Returns a read-only (constant) reference to the data at the first
       *  element of the %string.
       */
      const_reference
      front() const noexcept
      { return operator[](0); }

      /**
       *  Returns a read/write reference to the data at the last
       *  element of the %string.
       */
      reference
      back() noexcept
      { return operator[](this->size() - 1); }

      /**
       *  Returns a read-only (constant) reference to the data at the
       *  last element of the %string.
       */
      const_reference
      back() const noexcept
      { return operator[](this->size() - 1); }
#endif

      // Modifiers:
      /**
       *  @brief  Append a string to this string.
       *  @param __str  The string to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      operator+=(const __versa_string& __str)
      { return this->append(__str); }

      /**
       *  @brief  Append a C string.
       *  @param __s  The C string to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      operator+=(const _CharT* __s)
      { return this->append(__s); }

      /**
       *  @brief  Append a character.
       *  @param __c  The character to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      operator+=(_CharT __c)
      { 
	this->push_back(__c);
	return *this;
      }

#if __cplusplus >= 201103L
      /**
       *  @brief  Append an initializer_list of characters.
       *  @param __l  The initializer_list of characters to be appended.
       *  @return  Reference to this string.
       */
      __versa_string&
      operator+=(std::initializer_list<_CharT> __l)
      { return this->append(__l.begin(), __l.end()); }
#endif // C++11

      /**
       *  @brief  Append a string to this string.
       *  @param __str  The string to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      append(const __versa_string& __str)
      { return _M_append(__str._M_data(), __str.size()); }

      /**
       *  @brief  Append a substring.
       *  @param __str  The string to append.
       *  @param __pos  Index of the first character of str to append.
       *  @param __n  The number of characters to append.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range if @a pos is not a valid index.
       *
       *  This function appends @a __n characters from @a __str
       *  starting at @a __pos to this string.  If @a __n is is larger
       *  than the number of available characters in @a __str, the
       *  remainder of @a __str is appended.
       */
      __versa_string&
      append(const __versa_string& __str, size_type __pos, size_type __n)
      { return _M_append(__str._M_data()
			 + __str._M_check(__pos, "__versa_string::append"),
			 __str._M_limit(__pos, __n)); }

      /**
       *  @brief  Append a C substring.
       *  @param __s  The C string to append.
       *  @param __n  The number of characters to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      append(const _CharT* __s, size_type __n)
      {
	__glibcxx_requires_string_len(__s, __n);
	_M_check_length(size_type(0), __n, "__versa_string::append");
	return _M_append(__s, __n);
      }

      /**
       *  @brief  Append a C string.
       *  @param __s  The C string to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      append(const _CharT* __s)
      {
	__glibcxx_requires_string(__s);
	const size_type __n = traits_type::length(__s);
	_M_check_length(size_type(0), __n, "__versa_string::append");
	return _M_append(__s, __n);
      }

      /**
       *  @brief  Append multiple characters.
       *  @param __n  The number of characters to append.
       *  @param __c  The character to use.
       *  @return  Reference to this string.
       *
       *  Appends n copies of c to this string.
       */
      __versa_string&
      append(size_type __n, _CharT __c)
      { return _M_replace_aux(this->size(), size_type(0), __n, __c); }

#if __cplusplus >= 201103L
      /**
       *  @brief  Append an initializer_list of characters.
       *  @param __l  The initializer_list of characters to append.
       *  @return  Reference to this string.
       */
      __versa_string&
      append(std::initializer_list<_CharT> __l)
      { return this->append(__l.begin(), __l.end()); }
#endif // C++11

      /**
       *  @brief  Append a range of characters.
       *  @param __first  Iterator referencing the first character to append.
       *  @param __last  Iterator marking the end of the range.
       *  @return  Reference to this string.
       *
       *  Appends characters in the range [first,last) to this string.
       */
#if __cplusplus >= 201103L
      template<class _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
#else
      template<class _InputIterator>
#endif
        __versa_string&
        append(_InputIterator __first, _InputIterator __last)
        { return this->replace(_M_iend(), _M_iend(), __first, __last); }

      /**
       *  @brief  Append a single character.
       *  @param __c  Character to append.
       */
      void
      push_back(_CharT __c)
      { 
	const size_type __size = this->size();
	if (__size + 1 > this->capacity() || this->_M_is_shared())
	  this->_M_mutate(__size, size_type(0), 0, size_type(1));
	traits_type::assign(this->_M_data()[__size], __c);
	this->_M_set_length(__size + 1);
      }

      /**
       *  @brief  Set value to contents of another string.
       *  @param  __str  Source string to use.
       *  @return  Reference to this string.
       */
      __versa_string&
      assign(const __versa_string& __str)
      {
	this->_M_assign(__str);
	return *this;
      }

#if __cplusplus >= 201103L
      /**
       *  @brief  Set value to contents of another string.
       *  @param  __str  Source string to use.
       *  @return  Reference to this string.
       *
       *  This function sets this string to the exact contents of @a __str.
       *  @a __str is a valid, but unspecified string.
       */
      __versa_string&
      assign(__versa_string&& __str) noexcept
      {
	this->swap(__str);
	return *this;
      }
#endif // C++11

      /**
       *  @brief  Set value to a substring of a string.
       *  @param __str  The string to use.
       *  @param __pos  Index of the first character of str.
       *  @param __n  Number of characters to use.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range if @a __pos is not a valid index.
       *
       *  This function sets this string to the substring of @a __str
       *  consisting of @a __n characters at @a __pos.  If @a __n is
       *  is larger than the number of available characters in @a
       *  __str, the remainder of @a __str is used.
       */
      __versa_string&
      assign(const __versa_string& __str, size_type __pos, size_type __n)
      { return _M_replace(size_type(0), this->size(), __str._M_data()
			  + __str._M_check(__pos, "__versa_string::assign"),
			  __str._M_limit(__pos, __n)); }

      /**
       *  @brief  Set value to a C substring.
       *  @param __s  The C string to use.
       *  @param __n  Number of characters to use.
       *  @return  Reference to this string.
       *
       *  This function sets the value of this string to the first @a
       *  __n characters of @a __s.  If @a __n is is larger than the
       *  number of available characters in @a __s, the remainder of
       *  @a __s is used.
       */
      __versa_string&
      assign(const _CharT* __s, size_type __n)
      {
	__glibcxx_requires_string_len(__s, __n);
	return _M_replace(size_type(0), this->size(), __s, __n);
      }

      /**
       *  @brief  Set value to contents of a C string.
       *  @param __s  The C string to use.
       *  @return  Reference to this string.
       *
       *  This function sets the value of this string to the value of
       *  @a __s.  The data is copied, so there is no dependence on @a
       *  __s once the function returns.
       */
      __versa_string&
      assign(const _CharT* __s)
      {
	__glibcxx_requires_string(__s);
	return _M_replace(size_type(0), this->size(), __s,
			  traits_type::length(__s));
      }

      /**
       *  @brief  Set value to multiple characters.
       *  @param __n  Length of the resulting string.
       *  @param __c  The character to use.
       *  @return  Reference to this string.
       *
       *  This function sets the value of this string to @a __n copies of
       *  character @a __c.
       */
      __versa_string&
      assign(size_type __n, _CharT __c)
      { return _M_replace_aux(size_type(0), this->size(), __n, __c); }

      /**
       *  @brief  Set value to a range of characters.
       *  @param __first  Iterator referencing the first character to append.
       *  @param __last  Iterator marking the end of the range.
       *  @return  Reference to this string.
       *
       *  Sets value of string to characters in the range
       *  [first,last).
      */
#if __cplusplus >= 201103L
      template<class _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
#else
      template<class _InputIterator>
#endif
        __versa_string&
        assign(_InputIterator __first, _InputIterator __last)
        { return this->replace(_M_ibegin(), _M_iend(), __first, __last); }

#if __cplusplus >= 201103L
      /**
       *  @brief  Set value to an initializer_list of characters.
       *  @param __l  The initializer_list of characters to assign.
       *  @return  Reference to this string.
       */
      __versa_string&
      assign(std::initializer_list<_CharT> __l)
      { return this->assign(__l.begin(), __l.end()); }
#endif // C++11

#if __cplusplus >= 201103L
      /**
       *  @brief  Insert multiple characters.
       *  @param __p  Const_iterator referencing location in string to
       *              insert at.
       *  @param __n  Number of characters to insert
       *  @param __c  The character to insert.
       *  @return  Iterator referencing the first inserted char.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts @a __n copies of character @a __c starting at the
       *  position referenced by iterator @a __p.  If adding
       *  characters causes the length to exceed max_size(),
       *  length_error is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      iterator
      insert(const_iterator __p, size_type __n, _CharT __c)
      {
	_GLIBCXX_DEBUG_PEDASSERT(__p >= _M_ibegin() && __p <= _M_iend());
	const size_type __pos = __p - _M_ibegin();
	this->replace(__p, __p, __n, __c);
	return iterator(this->_M_data() + __pos); 
      }
#else
      /**
       *  @brief  Insert multiple characters.
       *  @param __p  Iterator referencing location in string to insert at.
       *  @param __n  Number of characters to insert
       *  @param __c  The character to insert.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts @a __n copies of character @a __c starting at the
       *  position referenced by iterator @a __p.  If adding
       *  characters causes the length to exceed max_size(),
       *  length_error is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      void
      insert(iterator __p, size_type __n, _CharT __c)
      {	this->replace(__p, __p, __n, __c);  }
#endif

#if __cplusplus >= 201103L
      /**
       *  @brief  Insert a range of characters.
       *  @param __p  Const_iterator referencing location in string to
       *              insert at.
       *  @param __beg  Start of range.
       *  @param __end  End of range.
       *  @return  Iterator referencing the first inserted char.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts characters in range [beg,end).  If adding characters
       *  causes the length to exceed max_size(), length_error is
       *  thrown.  The value of the string doesn't change if an error
       *  is thrown.
      */
      template<class _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
	iterator
        insert(const_iterator __p, _InputIterator __beg, _InputIterator __end)
        {
	  _GLIBCXX_DEBUG_PEDASSERT(__p >= _M_ibegin() && __p <= _M_iend());
	  const size_type __pos = __p - _M_ibegin();
	  this->replace(__p, __p, __beg, __end);
	  return iterator(this->_M_data() + __pos);
	}
#else
      /**
       *  @brief  Insert a range of characters.
       *  @param __p  Iterator referencing location in string to insert at.
       *  @param __beg  Start of range.
       *  @param __end  End of range.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts characters in range [beg,end).  If adding characters
       *  causes the length to exceed max_size(), length_error is
       *  thrown.  The value of the string doesn't change if an error
       *  is thrown.
      */
      template<class _InputIterator>
        void
        insert(iterator __p, _InputIterator __beg, _InputIterator __end)
        { this->replace(__p, __p, __beg, __end); }
#endif

#if __cplusplus >= 201103L
      /**
       *  @brief  Insert an initializer_list of characters.
       *  @param __p  Const_iterator referencing location in string to
       *              insert at.
       *  @param __l  The initializer_list of characters to insert.
       *  @return  Iterator referencing the first inserted char.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       */
      iterator
      insert(const_iterator __p, std::initializer_list<_CharT> __l)
      { return this->insert(__p, __l.begin(), __l.end()); }
#endif // C++11

      /**
       *  @brief  Insert value of a string.
       *  @param __pos1  Iterator referencing location in string to insert at.
       *  @param __str  The string to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts value of @a __str starting at @a __pos1.  If adding
       *  characters causes the length to exceed max_size(),
       *  length_error is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      __versa_string&
      insert(size_type __pos1, const __versa_string& __str)
      { return this->replace(__pos1, size_type(0),
			     __str._M_data(), __str.size()); }

      /**
       *  @brief  Insert a substring.
       *  @param __pos1  Iterator referencing location in string to insert at.
       *  @param __str  The string to insert.
       *  @param __pos2  Start of characters in str to insert.
       *  @param __n  Number of characters to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *  @throw  std::out_of_range  If @a __pos1 > size() or
       *  @a __pos2 > @a __str.size().
       *
       *  Starting at @a __pos1, insert @a __n character of @a __str
       *  beginning with @a __pos2.  If adding characters causes the
       *  length to exceed max_size(), length_error is thrown.  If @a
       *  __pos1 is beyond the end of this string or @a __pos2 is
       *  beyond the end of @a __str, out_of_range is thrown.  The
       *  value of the string doesn't change if an error is thrown.
      */
      __versa_string&
      insert(size_type __pos1, const __versa_string& __str,
	     size_type __pos2, size_type __n)
      { return this->replace(__pos1, size_type(0), __str._M_data()
			     + __str._M_check(__pos2, "__versa_string::insert"),
			     __str._M_limit(__pos2, __n)); }

      /**
       *  @brief  Insert a C substring.
       *  @param __pos  Iterator referencing location in string to insert at.
       *  @param __s  The C string to insert.
       *  @param __n  The number of characters to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *  @throw  std::out_of_range  If @a __pos is beyond the end of this
       *  string.
       *
       *  Inserts the first @a __n characters of @a __s starting at @a
       *  __pos.  If adding characters causes the length to exceed
       *  max_size(), length_error is thrown.  If @a __pos is beyond
       *  end(), out_of_range is thrown.  The value of the string
       *  doesn't change if an error is thrown.
      */
      __versa_string&
      insert(size_type __pos, const _CharT* __s, size_type __n)
      { return this->replace(__pos, size_type(0), __s, __n); }

      /**
       *  @brief  Insert a C string.
       *  @param __pos  Iterator referencing location in string to insert at.
       *  @param __s  The C string to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *  @throw  std::out_of_range  If @a __pos is beyond the end of this
       *  string.
       *
       *  Inserts the first @a __n characters of @a __s starting at @a
       *  __pos.  If adding characters causes the length to exceed
       *  max_size(), length_error is thrown.  If @a __pos is beyond
       *  end(), out_of_range is thrown.  The value of the string
       *  doesn't change if an error is thrown.
      */
      __versa_string&
      insert(size_type __pos, const _CharT* __s)
      {
	__glibcxx_requires_string(__s);
	return this->replace(__pos, size_type(0), __s,
			     traits_type::length(__s));
      }

      /**
       *  @brief  Insert multiple characters.
       *  @param __pos  Index in string to insert at.
       *  @param __n  Number of characters to insert
       *  @param __c  The character to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *  @throw  std::out_of_range  If @a __pos is beyond the end of this
       *  string.
       *
       *  Inserts @a __n copies of character @a __c starting at index
       *  @a __pos.  If adding characters causes the length to exceed
       *  max_size(), length_error is thrown.  If @a __pos > length(),
       *  out_of_range is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      __versa_string&
      insert(size_type __pos, size_type __n, _CharT __c)
      { return _M_replace_aux(_M_check(__pos, "__versa_string::insert"),
			      size_type(0), __n, __c); }

      /**
       *  @brief  Insert one character.
       *  @param __p  Iterator referencing position in string to insert at.
       *  @param __c  The character to insert.
       *  @return  Iterator referencing newly inserted char.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Inserts character @a __c at position referenced by @a __p.
       *  If adding character causes the length to exceed max_size(),
       *  length_error is thrown.  If @a __p is beyond end of string,
       *  out_of_range is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      iterator
#if __cplusplus >= 201103L
      insert(const_iterator __p, _CharT __c)
#else
      insert(iterator __p, _CharT __c)	
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(__p >= _M_ibegin() && __p <= _M_iend());
	const size_type __pos = __p - _M_ibegin();
	_M_replace_aux(__pos, size_type(0), size_type(1), __c);
	this->_M_set_leaked();
	return iterator(this->_M_data() + __pos);
      }

      /**
       *  @brief  Remove characters.
       *  @param __pos  Index of first character to remove (default 0).
       *  @param __n  Number of characters to remove (default remainder).
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos is beyond the end of this
       *  string.
       *
       *  Removes @a __n characters from this string starting at @a
       *  __pos.  The length of the string is reduced by @a __n.  If
       *  there are < @a __n characters to remove, the remainder of
       *  the string is truncated.  If @a __p is beyond end of string,
       *  out_of_range is thrown.  The value of the string doesn't
       *  change if an error is thrown.
      */
      __versa_string&
      erase(size_type __pos = 0, size_type __n = npos)
      { 
	this->_M_erase(_M_check(__pos, "__versa_string::erase"),
		       _M_limit(__pos, __n));
	return *this;
      }

      /**
       *  @brief  Remove one character.
       *  @param __position  Iterator referencing the character to remove.
       *  @return  iterator referencing same location after removal.
       *
       *  Removes the character at @a __position from this string. The
       *  value of the string doesn't change if an error is thrown.
      */
      iterator
#if __cplusplus >= 201103L
      erase(const_iterator __position)
#else
      erase(iterator __position)	
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(__position >= _M_ibegin()
				 && __position < _M_iend());
	const size_type __pos = __position - _M_ibegin();
	this->_M_erase(__pos, size_type(1));
	this->_M_set_leaked();
	return iterator(this->_M_data() + __pos);
      }

      /**
       *  @brief  Remove a range of characters.
       *  @param __first  Iterator referencing the first character to remove.
       *  @param __last  Iterator referencing the end of the range.
       *  @return  Iterator referencing location of first after removal.
       *
       *  Removes the characters in the range [first,last) from this
       *  string.  The value of the string doesn't change if an error
       *  is thrown.
      */
      iterator
#if __cplusplus >= 201103L
      erase(const_iterator __first, const_iterator __last)
#else
      erase(iterator __first, iterator __last)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(__first >= _M_ibegin() && __first <= __last
				 && __last <= _M_iend());
        const size_type __pos = __first - _M_ibegin();
	this->_M_erase(__pos, __last - __first);
	this->_M_set_leaked();
	return iterator(this->_M_data() + __pos);
      }

#if __cplusplus >= 201103L
      /**
       *  @brief  Remove the last character.
       *
       *  The string must be non-empty.
       */
      void
      pop_back()
      { this->_M_erase(size()-1, 1); }
#endif // C++11

      /**
       *  @brief  Replace characters with value from another string.
       *  @param __pos  Index of first character to replace.
       *  @param __n  Number of characters to be replaced.
       *  @param __str  String to insert.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos is beyond the end of this
       *  string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [pos,pos+n) from this
       *  string.  In place, the value of @a __str is inserted.  If @a
       *  __pos is beyond end of string, out_of_range is thrown.  If
       *  the length of the result exceeds max_size(), length_error is
       *  thrown.  The value of the string doesn't change if an error
       *  is thrown.
      */
      __versa_string&
      replace(size_type __pos, size_type __n, const __versa_string& __str)
      { return this->replace(__pos, __n, __str._M_data(), __str.size()); }

      /**
       *  @brief  Replace characters with value from another string.
       *  @param __pos1  Index of first character to replace.
       *  @param __n1  Number of characters to be replaced.
       *  @param __str  String to insert.
       *  @param __pos2  Index of first character of str to use.
       *  @param __n2  Number of characters from str to use.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos1 > size() or @a __pos2 >
       *  str.size().
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [pos1,pos1 + n) from
       *  this string.  In place, the value of @a __str is inserted.
       *  If @a __pos is beyond end of string, out_of_range is thrown.
       *  If the length of the result exceeds max_size(), length_error
       *  is thrown.  The value of the string doesn't change if an
       *  error is thrown.
      */
      __versa_string&
      replace(size_type __pos1, size_type __n1, const __versa_string& __str,
	      size_type __pos2, size_type __n2)
      {
	return this->replace(__pos1, __n1, __str._M_data()
			     + __str._M_check(__pos2,
					      "__versa_string::replace"),
			     __str._M_limit(__pos2, __n2));
      }

      /**
       *  @brief  Replace characters with value of a C substring.
       *  @param __pos  Index of first character to replace.
       *  @param __n1  Number of characters to be replaced.
       *  @param __s  C string to insert.
       *  @param __n2  Number of characters from @a __s to use.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos1 > size().
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [pos,pos + n1) from this
       *  string.  In place, the first @a __n2 characters of @a __s
       *  are inserted, or all of @a __s if @a __n2 is too large.  If
       *  @a __pos is beyond end of string, out_of_range is thrown.
       *  If the length of result exceeds max_size(), length_error is
       *  thrown.  The value of the string doesn't change if an error
       *  is thrown.
      */
      __versa_string&
      replace(size_type __pos, size_type __n1, const _CharT* __s,
	      size_type __n2)
      {
	__glibcxx_requires_string_len(__s, __n2);
	return _M_replace(_M_check(__pos, "__versa_string::replace"),
			  _M_limit(__pos, __n1), __s, __n2);
      }

      /**
       *  @brief  Replace characters with value of a C string.
       *  @param __pos  Index of first character to replace.
       *  @param __n1  Number of characters to be replaced.
       *  @param __s  C string to insert.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos > size().
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [pos,pos + n1) from this
       *  string.  In place, the characters of @a __s are inserted.  If
       *  @a pos is beyond end of string, out_of_range is thrown.  If
       *  the length of result exceeds max_size(), length_error is thrown.  
       *  The value of the string doesn't change if an error is thrown.
      */
      __versa_string&
      replace(size_type __pos, size_type __n1, const _CharT* __s)
      {
	__glibcxx_requires_string(__s);
	return this->replace(__pos, __n1, __s, traits_type::length(__s));
      }

      /**
       *  @brief  Replace characters with multiple characters.
       *  @param __pos  Index of first character to replace.
       *  @param __n1  Number of characters to be replaced.
       *  @param __n2  Number of characters to insert.
       *  @param __c  Character to insert.
       *  @return  Reference to this string.
       *  @throw  std::out_of_range  If @a __pos > size().
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [pos,pos + n1) from this
       *  string.  In place, @a __n2 copies of @a __c are inserted.
       *  If @a __pos is beyond end of string, out_of_range is thrown.
       *  If the length of result exceeds max_size(), length_error is
       *  thrown.  The value of the string doesn't change if an error
       *  is thrown.
      */
      __versa_string&
      replace(size_type __pos, size_type __n1, size_type __n2, _CharT __c)
      { return _M_replace_aux(_M_check(__pos, "__versa_string::replace"),
			      _M_limit(__pos, __n1), __n2, __c); }

      /**
       *  @brief  Replace range of characters with string.
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __str  String value to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place, the
       *  value of @a __str is inserted.  If the length of result
       *  exceeds max_size(), length_error is thrown.  The value of
       *  the string doesn't change if an error is thrown.
      */
      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      const __versa_string& __str)
#else
      replace(iterator __i1, iterator __i2, const __versa_string& __str)
#endif
      { return this->replace(__i1, __i2, __str._M_data(), __str.size()); }

      /**
       *  @brief  Replace range of characters with C substring.
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __s  C string value to insert.
       *  @param __n  Number of characters from s to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place, the
       *  first @a n characters of @a __s are inserted.  If the length
       *  of result exceeds max_size(), length_error is thrown.  The
       *  value of the string doesn't change if an error is thrown.
      */
      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      const _CharT* __s, size_type __n)
#else
      replace(iterator __i1, iterator __i2, const _CharT* __s, size_type __n)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	return this->replace(__i1 - _M_ibegin(), __i2 - __i1, __s, __n);
      }

      /**
       *  @brief  Replace range of characters with C string.
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __s  C string value to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place, the
       *  characters of @a __s are inserted.  If the length of result
       *  exceeds max_size(), length_error is thrown.  The value of
       *  the string doesn't change if an error is thrown.
      */
      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2, const _CharT* __s)
#else
      replace(iterator __i1, iterator __i2, const _CharT* __s)	
#endif
      {
	__glibcxx_requires_string(__s);
	return this->replace(__i1, __i2, __s, traits_type::length(__s));
      }

      /**
       *  @brief  Replace range of characters with multiple characters
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __n  Number of characters to insert.
       *  @param __c  Character to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place, @a
       *  __n copies of @a __c are inserted.  If the length of result
       *  exceeds max_size(), length_error is thrown.  The value of
       *  the string doesn't change if an error is thrown.
      */
      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2, size_type __n,
	      _CharT __c)
#else
      replace(iterator __i1, iterator __i2, size_type __n, _CharT __c)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	return _M_replace_aux(__i1 - _M_ibegin(), __i2 - __i1, __n, __c);
      }

      /**
       *  @brief  Replace range of characters with range.
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __k1  Iterator referencing start of range to insert.
       *  @param __k2  Iterator referencing end of range to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place,
       *  characters in the range [k1,k2) are inserted.  If the length
       *  of result exceeds max_size(), length_error is thrown.  The
       *  value of the string doesn't change if an error is thrown.
      */
#if __cplusplus >= 201103L
      template<class _InputIterator,
	       typename = std::_RequireInputIter<_InputIterator>>
        __versa_string&
        replace(const_iterator __i1, const_iterator __i2,
		_InputIterator __k1, _InputIterator __k2)
        {
	  _GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				   && __i2 <= _M_iend());
	  __glibcxx_requires_valid_range(__k1, __k2);
	  return this->_M_replace_dispatch(__i1, __i2, __k1, __k2,
					   std::__false_type());
	}
#else
      template<class _InputIterator>
        __versa_string&
        replace(iterator __i1, iterator __i2,
		_InputIterator __k1, _InputIterator __k2)
        {
	  _GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				   && __i2 <= _M_iend());
	  __glibcxx_requires_valid_range(__k1, __k2);
	  typedef typename std::__is_integer<_InputIterator>::__type _Integral;
	  return this->_M_replace_dispatch(__i1, __i2, __k1, __k2, _Integral());
	}
#endif

      // Specializations for the common case of pointer and iterator:
      // useful to avoid the overhead of temporary buffering in _M_replace.
      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      _CharT* __k1, _CharT* __k2)
#else
      replace(iterator __i1, iterator __i2,
	      _CharT* __k1, _CharT* __k2)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	__glibcxx_requires_valid_range(__k1, __k2);
	return this->replace(__i1 - _M_ibegin(), __i2 - __i1,
			     __k1, __k2 - __k1);
      }

      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      const _CharT* __k1, const _CharT* __k2)
#else
      replace(iterator __i1, iterator __i2,
	      const _CharT* __k1, const _CharT* __k2)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	__glibcxx_requires_valid_range(__k1, __k2);
	return this->replace(__i1 - _M_ibegin(), __i2 - __i1,
			     __k1, __k2 - __k1);
      }

      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      iterator __k1, iterator __k2)
#else
      replace(iterator __i1, iterator __i2,
	      iterator __k1, iterator __k2)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	__glibcxx_requires_valid_range(__k1, __k2);
	return this->replace(__i1 - _M_ibegin(), __i2 - __i1,
			     __k1.base(), __k2 - __k1);
      }

      __versa_string&
#if __cplusplus >= 201103L
      replace(const_iterator __i1, const_iterator __i2,
	      const_iterator __k1, const_iterator __k2)
#else
      replace(iterator __i1, iterator __i2,
	      const_iterator __k1, const_iterator __k2)
#endif
      {
	_GLIBCXX_DEBUG_PEDASSERT(_M_ibegin() <= __i1 && __i1 <= __i2
				 && __i2 <= _M_iend());
	__glibcxx_requires_valid_range(__k1, __k2);
	return this->replace(__i1 - _M_ibegin(), __i2 - __i1,
			     __k1.base(), __k2 - __k1);
      }
      
#if __cplusplus >= 201103L
      /**
       *  @brief  Replace range of characters with initializer_list.
       *  @param __i1  Iterator referencing start of range to replace.
       *  @param __i2  Iterator referencing end of range to replace.
       *  @param __l  The initializer_list of characters to insert.
       *  @return  Reference to this string.
       *  @throw  std::length_error  If new length exceeds @c max_size().
       *
       *  Removes the characters in the range [i1,i2).  In place,
       *  characters in the range [k1,k2) are inserted.  If the length
       *  of result exceeds max_size(), length_error is thrown.  The
       *  value of the string doesn't change if an error is thrown.
      */
      __versa_string&
      replace(const_iterator __i1, const_iterator __i2,
	      std::initializer_list<_CharT> __l)
      { return this->replace(__i1, __i2, __l.begin(), __l.end()); }
#endif // C++11

    private:
      template<class _Integer>
	__versa_string&
	_M_replace_dispatch(const_iterator __i1, const_iterator __i2,
			    _Integer __n, _Integer __val, std::__true_type)
        { return _M_replace_aux(__i1 - _M_ibegin(), __i2 - __i1, __n, __val); }

      template<class _InputIterator>
	__versa_string&
	_M_replace_dispatch(const_iterator __i1, const_iterator __i2,
			    _InputIterator __k1, _InputIterator __k2,
			    std::__false_type);

      __versa_string&
      _M_replace_aux(size_type __pos1, size_type __n1, size_type __n2,
		     _CharT __c);

      __versa_string&
      _M_replace(size_type __pos, size_type __len1, const _CharT* __s,
		 const size_type __len2);

      __versa_string&
      _M_append(const _CharT* __s, size_type __n);

    public:

      /**
       *  @brief  Copy substring into C string.
       *  @param __s  C string to copy value into.
       *  @param __n  Number of characters to copy.
       *  @param __pos  Index of first character to copy.
       *  @return  Number of characters actually copied
       *  @throw  std::out_of_range  If pos > size().
       *
       *  Copies up to @a __n characters starting at @a __pos into the
       *  C string @a s.  If @a __pos is greater than size(),
       *  out_of_range is thrown.
      */
      size_type
      copy(_CharT* __s, size_type __n, size_type __pos = 0) const;

      /**
       *  @brief  Swap contents with another string.
       *  @param __s  String to swap with.
       *
       *  Exchanges the contents of this string with that of @a __s in
       *  constant time.
      */
      void
      swap(__versa_string& __s) _GLIBCXX_NOEXCEPT
      { this->_M_swap(__s); }

      // String operations:
      /**
       *  @brief  Return const pointer to null-terminated contents.
       *
       *  This is a handle to internal data.  Do not modify or dire things may
       *  happen.
      */
      const _CharT*
      c_str() const _GLIBCXX_NOEXCEPT
      { return this->_M_data(); }

      /**
       *  @brief  Return const pointer to contents.
       *
       *  This is a handle to internal data.  Do not modify or dire things may
       *  happen.
      */
      const _CharT*
      data() const _GLIBCXX_NOEXCEPT
      { return this->_M_data(); }

      /**
       *  @brief  Return copy of allocator used to construct this string.
      */
      allocator_type
      get_allocator() const _GLIBCXX_NOEXCEPT
      { return allocator_type(this->_M_get_allocator()); }

      /**
       *  @brief  Find position of a C substring.
       *  @param __s  C string to locate.
       *  @param __pos  Index of character to search from.
       *  @param __n  Number of characters from @a __s to search for.
       *  @return  Index of start of first occurrence.
       *
       *  Starting from @a __pos, searches forward for the first @a
       *  __n characters in @a __s within this string.  If found,
       *  returns the index where it begins.  If not found, returns
       *  npos.
      */
      size_type
      find(const _CharT* __s, size_type __pos, size_type __n) const;

      /**
       *  @brief  Find position of a string.
       *  @param __str  String to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of start of first occurrence.
       *
       *  Starting from @a __pos, searches forward for value of @a
       *  __str within this string.  If found, returns the index where
       *  it begins.  If not found, returns npos.
      */
      size_type
      find(const __versa_string& __str, size_type __pos = 0) const
	_GLIBCXX_NOEXCEPT
      { return this->find(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find position of a C string.
       *  @param __s  C string to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of start of first occurrence.
       *
       *  Starting from @a __pos, searches forward for the value of @a
       *  __s within this string.  If found, returns the index where
       *  it begins.  If not found, returns npos.
      */
      size_type
      find(const _CharT* __s, size_type __pos = 0) const
      {
	__glibcxx_requires_string(__s);
	return this->find(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find position of a character.
       *  @param __c  Character to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for @a __c within
       *  this string.  If found, returns the index where it was
       *  found.  If not found, returns npos.
      */
      size_type
      find(_CharT __c, size_type __pos = 0) const _GLIBCXX_NOEXCEPT;

      /**
       *  @brief  Find last position of a string.
       *  @param __str  String to locate.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of start of last occurrence.
       *
       *  Starting from @a __pos, searches backward for value of @a
       *  __str within this string.  If found, returns the index where
       *  it begins.  If not found, returns npos.
      */
      size_type
      rfind(const __versa_string& __str, size_type __pos = npos) const
	_GLIBCXX_NOEXCEPT
      { return this->rfind(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find last position of a C substring.
       *  @param __s  C string to locate.
       *  @param __pos  Index of character to search back from.
       *  @param __n  Number of characters from s to search for.
       *  @return  Index of start of last occurrence.
       *
       *  Starting from @a __pos, searches backward for the first @a
       *  __n characters in @a __s within this string.  If found,
       *  returns the index where it begins.  If not found, returns
       *  npos.
      */
      size_type
      rfind(const _CharT* __s, size_type __pos, size_type __n) const;

      /**
       *  @brief  Find last position of a C string.
       *  @param __s  C string to locate.
       *  @param __pos  Index of character to start search at (default end).
       *  @return  Index of start of  last occurrence.
       *
       *  Starting from @a __pos, searches backward for the value of
       *  @a __s within this string.  If found, returns the index
       *  where it begins.  If not found, returns npos.
      */
      size_type
      rfind(const _CharT* __s, size_type __pos = npos) const
      {
	__glibcxx_requires_string(__s);
	return this->rfind(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find last position of a character.
       *  @param __c  Character to locate.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for @a __c within
       *  this string.  If found, returns the index where it was
       *  found.  If not found, returns npos.
      */
      size_type
      rfind(_CharT __c, size_type __pos = npos) const _GLIBCXX_NOEXCEPT;

      /**
       *  @brief  Find position of a character of string.
       *  @param __str  String containing characters to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for one of the characters of
       *  @a __str within this string.  If found, returns the index where it was
       *  found.  If not found, returns npos.
      */
      size_type
      find_first_of(const __versa_string& __str, size_type __pos = 0) const
	_GLIBCXX_NOEXCEPT
      { return this->find_first_of(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find position of a character of C substring.
       *  @param __s  String containing characters to locate.
       *  @param __pos  Index of character to search from.
       *  @param __n  Number of characters from s to search for.
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for one of the
       *  first @a __n characters of @a __s within this string.  If
       *  found, returns the index where it was found.  If not found,
       *  returns npos.
      */
      size_type
      find_first_of(const _CharT* __s, size_type __pos, size_type __n) const;

      /**
       *  @brief  Find position of a character of C string.
       *  @param __s  String containing characters to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for one of the
       *  characters of @a __s within this string.  If found, returns
       *  the index where it was found.  If not found, returns npos.
      */
      size_type
      find_first_of(const _CharT* __s, size_type __pos = 0) const
      {
	__glibcxx_requires_string(__s);
	return this->find_first_of(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find position of a character.
       *  @param __c  Character to locate.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for the character
       *  @a __c within this string.  If found, returns the index
       *  where it was found.  If not found, returns npos.
       *
       *  Note: equivalent to find(c, pos).
      */
      size_type
      find_first_of(_CharT __c, size_type __pos = 0) const _GLIBCXX_NOEXCEPT
      { return this->find(__c, __pos); }

      /**
       *  @brief  Find last position of a character of string.
       *  @param __str  String containing characters to locate.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for one of the
       *  characters of @a __str within this string.  If found,
       *  returns the index where it was found.  If not found, returns
       *  npos.
      */
      size_type
      find_last_of(const __versa_string& __str, size_type __pos = npos) const
	_GLIBCXX_NOEXCEPT
      { return this->find_last_of(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find last position of a character of C substring.
       *  @param __s  C string containing characters to locate.
       *  @param __pos  Index of character to search back from.
       *  @param __n  Number of characters from s to search for.
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for one of the
       *  first @a __n characters of @a __s within this string.  If
       *  found, returns the index where it was found.  If not found,
       *  returns npos.
      */
      size_type
      find_last_of(const _CharT* __s, size_type __pos, size_type __n) const;

      /**
       *  @brief  Find last position of a character of C string.
       *  @param __s  C string containing characters to locate.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for one of the
       *  characters of @a __s within this string.  If found, returns
       *  the index where it was found.  If not found, returns npos.
      */
      size_type
      find_last_of(const _CharT* __s, size_type __pos = npos) const
      {
	__glibcxx_requires_string(__s);
	return this->find_last_of(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find last position of a character.
       *  @param __c  Character to locate.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for @a __c within
       *  this string.  If found, returns the index where it was
       *  found.  If not found, returns npos.
       *
       *  Note: equivalent to rfind(c, pos).
      */
      size_type
      find_last_of(_CharT __c, size_type __pos = npos) const _GLIBCXX_NOEXCEPT
      { return this->rfind(__c, __pos); }

      /**
       *  @brief  Find position of a character not in string.
       *  @param __str  String containing characters to avoid.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for a character not
       *  contained in @a __str within this string.  If found, returns
       *  the index where it was found.  If not found, returns npos.
      */
      size_type
      find_first_not_of(const __versa_string& __str, size_type __pos = 0) const
	_GLIBCXX_NOEXCEPT
      { return this->find_first_not_of(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find position of a character not in C substring.
       *  @param __s  C string containing characters to avoid.
       *  @param __pos  Index of character to search from.
       *  @param __n  Number of characters from s to consider.
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for a character not
       *  contained in the first @a __n characters of @a __s within
       *  this string.  If found, returns the index where it was
       *  found.  If not found, returns npos.
      */
      size_type
      find_first_not_of(const _CharT* __s, size_type __pos,
			size_type __n) const;

      /**
       *  @brief  Find position of a character not in C string.
       *  @param __s  C string containing characters to avoid.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for a character not
       *  contained in @a __s within this string.  If found, returns
       *  the index where it was found.  If not found, returns npos.
      */
      size_type
      find_first_not_of(const _CharT* __s, size_type __pos = 0) const
      {
	__glibcxx_requires_string(__s);
	return this->find_first_not_of(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find position of a different character.
       *  @param __c  Character to avoid.
       *  @param __pos  Index of character to search from (default 0).
       *  @return  Index of first occurrence.
       *
       *  Starting from @a __pos, searches forward for a character
       *  other than @a __c within this string.  If found, returns the
       *  index where it was found.  If not found, returns npos.
      */
      size_type
      find_first_not_of(_CharT __c, size_type __pos = 0) const
	_GLIBCXX_NOEXCEPT;

      /**
       *  @brief  Find last position of a character not in string.
       *  @param __str  String containing characters to avoid.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for a character
       *  not contained in @a __str within this string.  If found,
       *  returns the index where it was found.  If not found, returns
       *  npos.
      */
      size_type
      find_last_not_of(const __versa_string& __str,
		       size_type __pos = npos) const _GLIBCXX_NOEXCEPT
      { return this->find_last_not_of(__str.data(), __pos, __str.size()); }

      /**
       *  @brief  Find last position of a character not in C substring.
       *  @param __s  C string containing characters to avoid.
       *  @param __pos  Index of character to search back from.
       *  @param __n  Number of characters from s to consider.
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for a character
       *  not contained in the first @a __n characters of @a __s
       *  within this string.  If found, returns the index where it
       *  was found.  If not found, returns npos.
      */
      size_type
      find_last_not_of(const _CharT* __s, size_type __pos,
		       size_type __n) const;
      /**
       *  @brief  Find last position of a character not in C string.
       *  @param __s  C string containing characters to avoid.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for a character
       *  not contained in @a __s within this string.  If found,
       *  returns the index where it was found.  If not found, returns
       *  npos.
      */
      size_type
      find_last_not_of(const _CharT* __s, size_type __pos = npos) const
      {
	__glibcxx_requires_string(__s);
	return this->find_last_not_of(__s, __pos, traits_type::length(__s));
      }

      /**
       *  @brief  Find last position of a different character.
       *  @param __c  Character to avoid.
       *  @param __pos  Index of character to search back from (default end).
       *  @return  Index of last occurrence.
       *
       *  Starting from @a __pos, searches backward for a character
       *  other than @a __c within this string.  If found, returns the
       *  index where it was found.  If not found, returns npos.
      */
      size_type
      find_last_not_of(_CharT __c, size_type __pos = npos) const
	_GLIBCXX_NOEXCEPT;

      /**
       *  @brief  Get a substring.
       *  @param __pos  Index of first character (default 0).
       *  @param __n  Number of characters in substring (default remainder).
       *  @return  The new string.
       *  @throw  std::out_of_range  If pos > size().
       *
       *  Construct and return a new string using the @a __n
       *  characters starting at @a __pos.  If the string is too
       *  short, use the remainder of the characters.  If @a __pos is
       *  beyond the end of the string, out_of_range is thrown.
      */
      __versa_string
      substr(size_type __pos = 0, size_type __n = npos) const
      {
	return __versa_string(*this, _M_check(__pos, "__versa_string::substr"),
			      __n);
      }

      /**
       *  @brief  Compare to a string.
       *  @param __str  String to compare against.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Returns an integer < 0 if this string is ordered before @a
       *  __str, 0 if their values are equivalent, or > 0 if this
       *  string is ordered after @a __str.  Determines the effective
       *  length rlen of the strings to compare as the smallest of
       *  size() and str.size().  The function then compares the two
       *  strings by calling traits::compare(data(), str.data(),rlen).
       *  If the result of the comparison is nonzero returns it,
       *  otherwise the shorter one is ordered first.
      */
      int
      compare(const __versa_string& __str) const
      {
	if (this->_M_compare(__str))
	  return 0;

	const size_type __size = this->size();
	const size_type __osize = __str.size();
	const size_type __len = std::min(__size, __osize);

	int __r = traits_type::compare(this->_M_data(), __str.data(), __len);
	if (!__r)
	  __r = this->_S_compare(__size, __osize);
	return __r;
      }

      /**
       *  @brief  Compare substring to a string.
       *  @param __pos  Index of first character of substring.
       *  @param __n  Number of characters in substring.
       *  @param __str  String to compare against.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Form the substring of this string from the @a __n characters
       *  starting at @a __pos.  Returns an integer < 0 if the
       *  substring is ordered before @a __str, 0 if their values are
       *  equivalent, or > 0 if the substring is ordered after @a
       *  __str.  Determines the effective length rlen of the strings
       *  to compare as the smallest of the length of the substring
       *  and @a __str.size().  The function then compares the two
       *  strings by calling
       *  traits::compare(substring.data(),str.data(),rlen).  If the
       *  result of the comparison is nonzero returns it, otherwise
       *  the shorter one is ordered first.
      */
      int
      compare(size_type __pos, size_type __n,
	      const __versa_string& __str) const;

      /**
       *  @brief  Compare substring to a substring.
       *  @param __pos1  Index of first character of substring.
       *  @param __n1  Number of characters in substring.
       *  @param __str  String to compare against.
       *  @param __pos2  Index of first character of substring of str.
       *  @param __n2  Number of characters in substring of str.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Form the substring of this string from the @a __n1
       *  characters starting at @a __pos1.  Form the substring of @a
       *  __str from the @a __n2 characters starting at @a __pos2.
       *  Returns an integer < 0 if this substring is ordered before
       *  the substring of @a __str, 0 if their values are equivalent,
       *  or > 0 if this substring is ordered after the substring of
       *  @a __str.  Determines the effective length rlen of the
       *  strings to compare as the smallest of the lengths of the
       *  substrings.  The function then compares the two strings by
       *  calling
       *  traits::compare(substring.data(),str.substr(pos2,n2).data(),rlen).
       *  If the result of the comparison is nonzero returns it,
       *  otherwise the shorter one is ordered first.
      */
      int
      compare(size_type __pos1, size_type __n1, const __versa_string& __str,
	      size_type __pos2, size_type __n2) const;

      /**
       *  @brief  Compare to a C string.
       *  @param __s  C string to compare against.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Returns an integer < 0 if this string is ordered before @a
       *  __s, 0 if their values are equivalent, or > 0 if this string
       *  is ordered after @a __s.  Determines the effective length
       *  rlen of the strings to compare as the smallest of size() and
       *  the length of a string constructed from @a __s.  The
       *  function then compares the two strings by calling
       *  traits::compare(data(),s,rlen).  If the result of the
       *  comparison is nonzero returns it, otherwise the shorter one
       *  is ordered first.
      */
      int
      compare(const _CharT* __s) const;

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 5 String::compare specification questionable
      /**
       *  @brief  Compare substring to a C string.
       *  @param __pos  Index of first character of substring.
       *  @param __n1  Number of characters in substring.
       *  @param __s  C string to compare against.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Form the substring of this string from the @a __n1
       *  characters starting at @a __pos.  Returns an integer < 0 if
       *  the substring is ordered before @a __s, 0 if their values
       *  are equivalent, or > 0 if the substring is ordered after @a
       *  __s.  Determines the effective length rlen of the strings to
       *  compare as the smallest of the length of the substring and
       *  the length of a string constructed from @a __s.  The
       *  function then compares the two string by calling
       *  traits::compare(substring.data(),s,rlen).  If the result of
       *  the comparison is nonzero returns it, otherwise the shorter
       *  one is ordered first.
      */
      int
      compare(size_type __pos, size_type __n1, const _CharT* __s) const;

      /**
       *  @brief  Compare substring against a character array.
       *  @param __pos  Index of first character of substring.
       *  @param __n1  Number of characters in substring.
       *  @param __s  character array to compare against.
       *  @param __n2  Number of characters of s.
       *  @return  Integer < 0, 0, or > 0.
       *
       *  Form the substring of this string from the @a __n1
       *  characters starting at @a __pos.  Form a string from the
       *  first @a __n2 characters of @a __s.  Returns an integer < 0
       *  if this substring is ordered before the string from @a __s,
       *  0 if their values are equivalent, or > 0 if this substring
       *  is ordered after the string from @a __s.  Determines the
       *  effective length rlen of the strings to compare as the
       *  smallest of the length of the substring and @a __n2.  The
       *  function then compares the two strings by calling
       *  traits::compare(substring.data(),__s,rlen).  If the result of
       *  the comparison is nonzero returns it, otherwise the shorter
       *  one is ordered first.
       *
       *  NB: __s must have at least n2 characters, <em>\\0</em> has no special
       *  meaning.
      */
      int
      compare(size_type __pos, size_type __n1, const _CharT* __s,
	      size_type __n2) const;
    };

  // operator+
  /**
   *  @brief  Concatenate two strings.
   *  @param __lhs  First string.
   *  @param __rhs  Last string.
   *  @return  New string with value of @a __lhs followed by @a __rhs.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs);

  /**
   *  @brief  Concatenate C string and string.
   *  @param __lhs  First string.
   *  @param __rhs  Last string.
   *  @return  New string with value of @a __lhs followed by @a __rhs.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const _CharT* __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs);

  /**
   *  @brief  Concatenate character and string.
   *  @param __lhs  First string.
   *  @param __rhs  Last string.
   *  @return  New string with @a __lhs followed by @a __rhs.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(_CharT __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs);

  /**
   *  @brief  Concatenate string and C string.
   *  @param __lhs  First string.
   *  @param __rhs  Last string.
   *  @return  New string with @a __lhs followed by @a __rhs.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const _CharT* __rhs);

  /**
   *  @brief  Concatenate string and character.
   *  @param __lhs  First string.
   *  @param __rhs  Last string.
   *  @return  New string with @a __lhs followed by @a __rhs.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      _CharT __rhs);

#if __cplusplus >= 201103L
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(__versa_string<_CharT, _Traits, _Alloc, _Base>&& __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return std::move(__lhs.append(__rhs)); }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      __versa_string<_CharT, _Traits, _Alloc, _Base>&& __rhs)
    { return std::move(__rhs.insert(0, __lhs)); }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(__versa_string<_CharT, _Traits, _Alloc, _Base>&& __lhs,
	      __versa_string<_CharT, _Traits, _Alloc, _Base>&& __rhs)
    {
      const auto __size = __lhs.size() + __rhs.size();
      const bool __cond = (__size > __lhs.capacity()
			   && __size <= __rhs.capacity());
      return __cond ? std::move(__rhs.insert(0, __lhs))
	            : std::move(__lhs.append(__rhs));
    }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(const _CharT* __lhs,
	      __versa_string<_CharT, _Traits, _Alloc, _Base>&& __rhs)
    { return std::move(__rhs.insert(0, __lhs)); }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(_CharT __lhs,
	      __versa_string<_CharT, _Traits, _Alloc, _Base>&& __rhs)
    { return std::move(__rhs.insert(0, 1, __lhs)); }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(__versa_string<_CharT, _Traits, _Alloc, _Base>&& __lhs,
	      const _CharT* __rhs)
    { return std::move(__lhs.append(__rhs)); }

  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline __versa_string<_CharT, _Traits, _Alloc, _Base>
    operator+(__versa_string<_CharT, _Traits, _Alloc, _Base>&& __lhs,
	      _CharT __rhs)
    { return std::move(__lhs.append(1, __rhs)); }
#endif

  // operator ==
  /**
   *  @brief  Test equivalence of two strings.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs.compare(@a __rhs) == 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator==(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __lhs.compare(__rhs) == 0; }

  template<typename _CharT,
	   template <typename, typename, typename> class _Base>
    inline typename __enable_if<std::__is_char<_CharT>::__value, bool>::__type
    operator==(const __versa_string<_CharT, std::char_traits<_CharT>,
	       std::allocator<_CharT>, _Base>& __lhs,
	       const __versa_string<_CharT, std::char_traits<_CharT>,
	       std::allocator<_CharT>, _Base>& __rhs)
    { return (__lhs.size() == __rhs.size()
	      && !std::char_traits<_CharT>::compare(__lhs.data(), __rhs.data(),
						    __lhs.size())); }

  /**
   *  @brief  Test equivalence of C string and string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __rhs.compare(@a __lhs) == 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator==(const _CharT* __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __rhs.compare(__lhs) == 0; }

  /**
   *  @brief  Test equivalence of string and C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs.compare(@a __rhs) == 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator==(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const _CharT* __rhs)
    { return __lhs.compare(__rhs) == 0; }

  // operator !=
  /**
   *  @brief  Test difference of two strings.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs.compare(@a __rhs) != 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator!=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   *  @brief  Test difference of C string and string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __rhs.compare(@a __lhs) != 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator!=(const _CharT* __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   *  @brief  Test difference of string and C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs.compare(@a __rhs) != 0.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator!=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const _CharT* __rhs)
    { return !(__lhs == __rhs); }

  // operator <
  /**
   *  @brief  Test if string precedes string.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs precedes @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __lhs.compare(__rhs) < 0; }

  /**
   *  @brief  Test if string precedes C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs precedes @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const _CharT* __rhs)
    { return __lhs.compare(__rhs) < 0; }

  /**
   *  @brief  Test if C string precedes string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __lhs precedes @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<(const _CharT* __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __rhs.compare(__lhs) > 0; }

  // operator >
  /**
   *  @brief  Test if string follows string.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs follows @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __lhs.compare(__rhs) > 0; }

  /**
   *  @brief  Test if string follows C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs follows @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	      const _CharT* __rhs)
    { return __lhs.compare(__rhs) > 0; }

  /**
   *  @brief  Test if C string follows string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __lhs follows @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>(const _CharT* __lhs,
	      const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __rhs.compare(__lhs) < 0; }

  // operator <=
  /**
   *  @brief  Test if string doesn't follow string.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs doesn't follow @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __lhs.compare(__rhs) <= 0; }

  /**
   *  @brief  Test if string doesn't follow C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs doesn't follow @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const _CharT* __rhs)
    { return __lhs.compare(__rhs) <= 0; }

  /**
   *  @brief  Test if C string doesn't follow string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __lhs doesn't follow @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator<=(const _CharT* __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __rhs.compare(__lhs) >= 0; }

  // operator >=
  /**
   *  @brief  Test if string doesn't precede string.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *  @return  True if @a __lhs doesn't precede @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __lhs.compare(__rhs) >= 0; }

  /**
   *  @brief  Test if string doesn't precede C string.
   *  @param __lhs  String.
   *  @param __rhs  C string.
   *  @return  True if @a __lhs doesn't precede @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>=(const __versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	       const _CharT* __rhs)
    { return __lhs.compare(__rhs) >= 0; }

  /**
   *  @brief  Test if C string doesn't precede string.
   *  @param __lhs  C string.
   *  @param __rhs  String.
   *  @return  True if @a __lhs doesn't precede @a __rhs.  False otherwise.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline bool
    operator>=(const _CharT* __lhs,
	       const __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { return __rhs.compare(__lhs) <= 0; }

  /**
   *  @brief  Swap contents of two strings.
   *  @param __lhs  First string.
   *  @param __rhs  Second string.
   *
   *  Exchanges the contents of @a __lhs and @a __rhs in constant time.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline void
    swap(__versa_string<_CharT, _Traits, _Alloc, _Base>& __lhs,
	 __versa_string<_CharT, _Traits, _Alloc, _Base>& __rhs)
    { __lhs.swap(__rhs); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   *  @brief  Read stream into a string.
   *  @param __is  Input stream.
   *  @param __str  Buffer to store into.
   *  @return  Reference to the input stream.
   *
   *  Stores characters from @a __is into @a __str until whitespace is
   *  found, the end of the stream is encountered, or str.max_size()
   *  is reached.  If is.width() is non-zero, that is the limit on the
   *  number of characters stored into @a __str.  Any previous
   *  contents of @a __str are erased.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
           template <typename, typename, typename> class _Base>
    basic_istream<_CharT, _Traits>&
    operator>>(basic_istream<_CharT, _Traits>& __is,
	       __gnu_cxx::__versa_string<_CharT, _Traits,
	                                 _Alloc, _Base>& __str);

  /**
   *  @brief  Write string to a stream.
   *  @param __os  Output stream.
   *  @param __str  String to write out.
   *  @return  Reference to the output stream.
   *
   *  Output characters of @a __str into os following the same rules as for
   *  writing a C string.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
	   template <typename, typename, typename> class _Base>
    inline basic_ostream<_CharT, _Traits>&
    operator<<(basic_ostream<_CharT, _Traits>& __os,
	       const __gnu_cxx::__versa_string<_CharT, _Traits, _Alloc,
	       _Base>& __str)
    {
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 586. string inserter not a formatted function
      return __ostream_insert(__os, __str.data(), __str.size());
    }

  /**
   *  @brief  Read a line from stream into a string.
   *  @param __is  Input stream.
   *  @param __str  Buffer to store into.
   *  @param __delim  Character marking end of line.
   *  @return  Reference to the input stream.
   *
   *  Stores characters from @a __is into @a __str until @a __delim is
   *  found, the end of the stream is encountered, or str.max_size()
   *  is reached.  If is.width() is non-zero, that is the limit on the
   *  number of characters stored into @a __str.  Any previous
   *  contents of @a __str are erased.  If @a delim was encountered,
   *  it is extracted but not stored into @a __str.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
           template <typename, typename, typename> class _Base>
    basic_istream<_CharT, _Traits>&
    getline(basic_istream<_CharT, _Traits>& __is,
	    __gnu_cxx::__versa_string<_CharT, _Traits, _Alloc, _Base>& __str,
	    _CharT __delim);

  /**
   *  @brief  Read a line from stream into a string.
   *  @param __is  Input stream.
   *  @param __str  Buffer to store into.
   *  @return  Reference to the input stream.
   *
   *  Stores characters from is into @a __str until &apos;\n&apos; is
   *  found, the end of the stream is encountered, or str.max_size()
   *  is reached.  If is.width() is non-zero, that is the limit on the
   *  number of characters stored into @a __str.  Any previous
   *  contents of @a __str are erased.  If end of line was
   *  encountered, it is extracted but not stored into @a __str.
   */
  template<typename _CharT, typename _Traits, typename _Alloc,
           template <typename, typename, typename> class _Base>
    inline basic_istream<_CharT, _Traits>&
    getline(basic_istream<_CharT, _Traits>& __is,
	    __gnu_cxx::__versa_string<_CharT, _Traits, _Alloc, _Base>& __str)
    { return getline(__is, __str, __is.widen('\n')); }      

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#if __cplusplus >= 201103L

#include <ext/string_conversions.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

#if _GLIBCXX_USE_C99_STDLIB
  // 21.4 Numeric Conversions [string.conversions].
  inline int
  stoi(const __vstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa<long, int>(&std::strtol, "stoi", __str.c_str(),
					__idx, __base); }

  inline long
  stol(const __vstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::strtol, "stol", __str.c_str(),
			     __idx, __base); }

  inline unsigned long
  stoul(const __vstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::strtoul, "stoul", __str.c_str(),
			     __idx, __base); }

  inline long long
  stoll(const __vstring& __str, std::size_t* __idx = 0,	int __base = 10)
  { return __gnu_cxx::__stoa(&std::strtoll, "stoll", __str.c_str(),
			     __idx, __base); }

  inline unsigned long long
  stoull(const __vstring& __str, std::size_t* __idx, int __base = 10)
  { return __gnu_cxx::__stoa(&std::strtoull, "stoull", __str.c_str(),
			     __idx, __base); }

  // NB: strtof vs strtod.
  inline float
  stof(const __vstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::strtof, "stof", __str.c_str(), __idx); }

  inline double
  stod(const __vstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::strtod, "stod", __str.c_str(), __idx); }

  inline long double
  stold(const __vstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::strtold, "stold", __str.c_str(), __idx); }
#endif // _GLIBCXX_USE_C99_STDLIB

#if _GLIBCXX_USE_C99_STDIO
  // NB: (v)snprintf vs sprintf.

  // DR 1261.
  inline __vstring
  to_string(int __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf, 4 * sizeof(int),
					      "%d", __val); }

  inline __vstring
  to_string(unsigned __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf,
					      4 * sizeof(unsigned),
					      "%u", __val); }

  inline __vstring
  to_string(long __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf,
					      4 * sizeof(long),
					      "%ld", __val); }

  inline __vstring
  to_string(unsigned long __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf,
					      4 * sizeof(unsigned long),
					      "%lu", __val); }


  inline __vstring
  to_string(long long __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf,
					      4 * sizeof(long long),
					      "%lld", __val); }

  inline __vstring
  to_string(unsigned long long __val)
  { return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf,
					      4 * sizeof(unsigned long long),
					      "%llu", __val); }

  inline __vstring
  to_string(float __val)
  {
    const int __n = __numeric_traits<float>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf, __n,
					      "%f", __val);
  }

  inline __vstring
  to_string(double __val)
  {
    const int __n = __numeric_traits<double>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf, __n,
					      "%f", __val);
  }

  inline __vstring
  to_string(long double __val)
  {
    const int __n = __numeric_traits<long double>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__vstring>(&std::vsnprintf, __n,
					      "%Lf", __val);
  }
#endif // _GLIBCXX_USE_C99_STDIO

#if defined(_GLIBCXX_USE_WCHAR_T) && _GLIBCXX_USE_C99_WCHAR
  inline int 
  stoi(const __wvstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa<long, int>(&std::wcstol, "stoi", __str.c_str(),
					__idx, __base); }

  inline long 
  stol(const __wvstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::wcstol, "stol", __str.c_str(),
			     __idx, __base); }

  inline unsigned long
  stoul(const __wvstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::wcstoul, "stoul", __str.c_str(),
			     __idx, __base); }

  inline long long
  stoll(const __wvstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::wcstoll, "stoll", __str.c_str(),
			     __idx, __base); }

  inline unsigned long long
  stoull(const __wvstring& __str, std::size_t* __idx = 0, int __base = 10)
  { return __gnu_cxx::__stoa(&std::wcstoull, "stoull", __str.c_str(),
			     __idx, __base); }

  // NB: wcstof vs wcstod.
  inline float
  stof(const __wvstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::wcstof, "stof", __str.c_str(), __idx); }

  inline double
  stod(const __wvstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::wcstod, "stod", __str.c_str(), __idx); }

  inline long double
  stold(const __wvstring& __str, std::size_t* __idx = 0)
  { return __gnu_cxx::__stoa(&std::wcstold, "stold", __str.c_str(), __idx); }

#ifndef _GLIBCXX_HAVE_BROKEN_VSWPRINTF
  // DR 1261.
  inline __wvstring
  to_wstring(int __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(int),
					       L"%d", __val); }

  inline __wvstring
  to_wstring(unsigned __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(unsigned),
					       L"%u", __val); }

  inline __wvstring
  to_wstring(long __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(long),
					       L"%ld", __val); }

  inline __wvstring
  to_wstring(unsigned long __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(unsigned long),
					       L"%lu", __val); }

  inline __wvstring
  to_wstring(long long __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(long long),
					       L"%lld", __val); }

  inline __wvstring
  to_wstring(unsigned long long __val)
  { return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf,
					       4 * sizeof(unsigned long long),
					       L"%llu", __val); }

  inline __wvstring
  to_wstring(float __val)
  {
    const int __n = __numeric_traits<float>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf, __n,
					       L"%f", __val);
  }

  inline __wvstring
  to_wstring(double __val)
  {
    const int __n = __numeric_traits<double>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf, __n,
					       L"%f", __val);
  }

  inline __wvstring
  to_wstring(long double __val)
  {
    const int __n = __numeric_traits<long double>::__max_exponent10 + 20;
    return __gnu_cxx::__to_xstring<__wvstring>(&std::vswprintf, __n,
					       L"%Lf", __val);
  }
#endif // _GLIBCXX_HAVE_BROKEN_VSWPRINTF
#endif // _GLIBCXX_USE_WCHAR_T && _GLIBCXX_USE_C99_WCHAR

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif

#if __cplusplus >= 201103L

#include <bits/functional_hash.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// std::hash specialization for __vstring.
  template<>
    struct hash<__gnu_cxx::__vstring>
    : public __hash_base<size_t, __gnu_cxx::__vstring>
    {
      size_t
      operator()(const __gnu_cxx::__vstring& __s) const noexcept
      { return std::_Hash_impl::hash(__s.data(), __s.length()); }
    };

#ifdef _GLIBCXX_USE_WCHAR_T
  /// std::hash specialization for __wvstring.
  template<>
    struct hash<__gnu_cxx::__wvstring>
    : public __hash_base<size_t, __gnu_cxx::__wvstring>
    {
      size_t
      operator()(const __gnu_cxx::__wvstring& __s) const noexcept
      { return std::_Hash_impl::hash(__s.data(),
                                     __s.length() * sizeof(wchar_t)); }
    };
#endif

  /// std::hash specialization for __u16vstring.
  template<>
    struct hash<__gnu_cxx::__u16vstring>
    : public __hash_base<size_t, __gnu_cxx::__u16vstring>
    {
      size_t
      operator()(const __gnu_cxx::__u16vstring& __s) const noexcept
      { return std::_Hash_impl::hash(__s.data(),
                                     __s.length() * sizeof(char16_t)); }
    };

  /// std::hash specialization for __u32vstring.
  template<>
    struct hash<__gnu_cxx::__u32vstring>
    : public __hash_base<size_t, __gnu_cxx::__u32vstring>
    {
      size_t
      operator()(const __gnu_cxx::__u32vstring& __s) const noexcept
      { return std::_Hash_impl::hash(__s.data(),
                                     __s.length() * sizeof(char32_t)); }
    };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif // C++11

#include <ext/vstring.tcc>

#endif /* _VSTRING_H */
