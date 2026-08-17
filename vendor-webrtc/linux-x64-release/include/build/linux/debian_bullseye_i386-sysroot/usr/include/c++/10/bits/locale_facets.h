// Locale support -*- C++ -*-

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

/** @file bits/locale_facets.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{locale}
 */

//
// ISO C++ 14882: 22.1  Locales
//

#ifndef _LOCALE_FACETS_H
#define _LOCALE_FACETS_H 1

#pragma GCC system_header

#include <cwctype>	// For wctype_t
#include <cctype>
#include <bits/ctype_base.h>
#include <iosfwd>
#include <bits/ios_base.h>  // For ios_base, ios_base::iostate
#include <streambuf>
#include <bits/cpp_type_traits.h>
#include <ext/type_traits.h>
#include <ext/numeric_traits.h>
#include <bits/streambuf_iterator.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // NB: Don't instantiate required wchar_t facets if no wchar_t support.
#ifdef _GLIBCXX_USE_WCHAR_T
# define  _GLIBCXX_NUM_FACETS 28
# define  _GLIBCXX_NUM_CXX11_FACETS 16
#else
# define  _GLIBCXX_NUM_FACETS 14
# define  _GLIBCXX_NUM_CXX11_FACETS 8
#endif
#ifdef _GLIBCXX_USE_CHAR8_T
# define _GLIBCXX_NUM_UNICODE_FACETS 4
#else
# define _GLIBCXX_NUM_UNICODE_FACETS 2
#endif

  // Convert string to numeric value of type _Tp and store results.
  // NB: This is specialized for all required types, there is no
  // generic definition.
  template<typename _Tp>
    void
    __convert_to_v(const char*, _Tp&, ios_base::iostate&,
		   const __c_locale&) throw();

  // Explicit specializations for required types.
  template<>
    void
    __convert_to_v(const char*, float&, ios_base::iostate&,
		   const __c_locale&) throw();

  template<>
    void
    __convert_to_v(const char*, double&, ios_base::iostate&,
		   const __c_locale&) throw();

  template<>
    void
    __convert_to_v(const char*, long double&, ios_base::iostate&,
		   const __c_locale&) throw();

  // NB: __pad is a struct, rather than a function, so it can be
  // partially-specialized.
  template<typename _CharT, typename _Traits>
    struct __pad
    {
      static void
      _S_pad(ios_base& __io, _CharT __fill, _CharT* __news,
	     const _CharT* __olds, streamsize __newlen, streamsize __oldlen);
    };

  // Used by both numeric and monetary facets.
  // Inserts "group separator" characters into an array of characters.
  // It's recursive, one iteration per group.  It moves the characters
  // in the buffer this way: "xxxx12345" -> "12,345xxx".  Call this
  // only with __gsize != 0.
  template<typename _CharT>
    _CharT*
    __add_grouping(_CharT* __s, _CharT __sep,
		   const char* __gbeg, size_t __gsize,
		   const _CharT* __first, const _CharT* __last);

  // This template permits specializing facet output code for
  // ostreambuf_iterator.  For ostreambuf_iterator, sputn is
  // significantly more efficient than incrementing iterators.
  template<typename _CharT>
    inline
    ostreambuf_iterator<_CharT>
    __write(ostreambuf_iterator<_CharT> __s, const _CharT* __ws, int __len)
    {
      __s._M_put(__ws, __len);
      return __s;
    }

  // This is the unspecialized form of the template.
  template<typename _CharT, typename _OutIter>
    inline
    _OutIter
    __write(_OutIter __s, const _CharT* __ws, int __len)
    {
      for (int __j = 0; __j < __len; __j++, ++__s)
	*__s = __ws[__j];
      return __s;
    }


  // 22.2.1.1  Template class ctype
  // Include host and configuration specific ctype enums for ctype_base.

  /**
   *  @brief  Common base for ctype facet
   *
   *  This template class provides implementations of the public functions
   *  that forward to the protected virtual functions.
   *
   *  This template also provides abstract stubs for the protected virtual
   *  functions.
  */
  template<typename _CharT>
    class __ctype_abstract_base : public locale::facet, public ctype_base
    {
    public:
      // Types:
      /// Typedef for the template parameter
      typedef _CharT char_type;

      /**
       *  @brief  Test char_type classification.
       *
       *  This function finds a mask M for @a __c and compares it to
       *  mask @a __m.  It does so by returning the value of
       *  ctype<char_type>::do_is().
       *
       *  @param __c  The char_type to compare the mask of.
       *  @param __m  The mask to compare against.
       *  @return  (M & __m) != 0.
      */
      bool
      is(mask __m, char_type __c) const
      { return this->do_is(__m, __c); }

      /**
       *  @brief  Return a mask array.
       *
       *  This function finds the mask for each char_type in the range [lo,hi)
       *  and successively writes it to vec.  vec must have as many elements
       *  as the char array.  It does so by returning the value of
       *  ctype<char_type>::do_is().
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __vec  Pointer to an array of mask storage.
       *  @return  @a __hi.
      */
      const char_type*
      is(const char_type *__lo, const char_type *__hi, mask *__vec) const
      { return this->do_is(__lo, __hi, __vec); }

      /**
       *  @brief  Find char_type matching a mask
       *
       *  This function searches for and returns the first char_type c in
       *  [lo,hi) for which is(m,c) is true.  It does so by returning
       *  ctype<char_type>::do_scan_is().
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to matching char_type if found, else @a __hi.
      */
      const char_type*
      scan_is(mask __m, const char_type* __lo, const char_type* __hi) const
      { return this->do_scan_is(__m, __lo, __hi); }

      /**
       *  @brief  Find char_type not matching a mask
       *
       *  This function searches for and returns the first char_type c in
       *  [lo,hi) for which is(m,c) is false.  It does so by returning
       *  ctype<char_type>::do_scan_not().
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to first char in range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to non-matching char if found, else @a __hi.
      */
      const char_type*
      scan_not(mask __m, const char_type* __lo, const char_type* __hi) const
      { return this->do_scan_not(__m, __lo, __hi); }

      /**
       *  @brief  Convert to uppercase.
       *
       *  This function converts the argument to uppercase if possible.
       *  If not possible (for example, '2'), returns the argument.  It does
       *  so by returning ctype<char_type>::do_toupper().
       *
       *  @param __c  The char_type to convert.
       *  @return  The uppercase char_type if convertible, else @a __c.
      */
      char_type
      toupper(char_type __c) const
      { return this->do_toupper(__c); }

      /**
       *  @brief  Convert array to uppercase.
       *
       *  This function converts each char_type in the range [lo,hi) to
       *  uppercase if possible.  Other elements remain untouched.  It does so
       *  by returning ctype<char_type>:: do_toupper(lo, hi).
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      const char_type*
      toupper(char_type *__lo, const char_type* __hi) const
      { return this->do_toupper(__lo, __hi); }

      /**
       *  @brief  Convert to lowercase.
       *
       *  This function converts the argument to lowercase if possible.  If
       *  not possible (for example, '2'), returns the argument.  It does so
       *  by returning ctype<char_type>::do_tolower(c).
       *
       *  @param __c  The char_type to convert.
       *  @return  The lowercase char_type if convertible, else @a __c.
      */
      char_type
      tolower(char_type __c) const
      { return this->do_tolower(__c); }

      /**
       *  @brief  Convert array to lowercase.
       *
       *  This function converts each char_type in the range [__lo,__hi) to
       *  lowercase if possible.  Other elements remain untouched.  It does so
       *  by returning ctype<char_type>:: do_tolower(__lo, __hi).
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      const char_type*
      tolower(char_type* __lo, const char_type* __hi) const
      { return this->do_tolower(__lo, __hi); }

      /**
       *  @brief  Widen char to char_type
       *
       *  This function converts the char argument to char_type using the
       *  simplest reasonable transformation.  It does so by returning
       *  ctype<char_type>::do_widen(c).
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @return  The converted char_type.
      */
      char_type
      widen(char __c) const
      { return this->do_widen(__c); }

      /**
       *  @brief  Widen array to char_type
       *
       *  This function converts each char in the input to char_type using the
       *  simplest reasonable transformation.  It does so by returning
       *  ctype<char_type>::do_widen(c).
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      const char*
      widen(const char* __lo, const char* __hi, char_type* __to) const
      { return this->do_widen(__lo, __hi, __to); }

      /**
       *  @brief  Narrow char_type to char
       *
       *  This function converts the char_type to char using the simplest
       *  reasonable transformation.  If the conversion fails, dfault is
       *  returned instead.  It does so by returning
       *  ctype<char_type>::do_narrow(__c).
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char_type to convert.
       *  @param __dfault  Char to return if conversion fails.
       *  @return  The converted char.
      */
      char
      narrow(char_type __c, char __dfault) const
      { return this->do_narrow(__c, __dfault); }

      /**
       *  @brief  Narrow array to char array
       *
       *  This function converts each char_type in the input to char using the
       *  simplest reasonable transformation and writes the results to the
       *  destination array.  For any char_type in the input that cannot be
       *  converted, @a dfault is used instead.  It does so by returning
       *  ctype<char_type>::do_narrow(__lo, __hi, __dfault, __to).
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __dfault  Char to use if conversion fails.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      const char_type*
      narrow(const char_type* __lo, const char_type* __hi,
	      char __dfault, char* __to) const
      { return this->do_narrow(__lo, __hi, __dfault, __to); }

    protected:
      explicit
      __ctype_abstract_base(size_t __refs = 0): facet(__refs) { }

      virtual
      ~__ctype_abstract_base() { }

      /**
       *  @brief  Test char_type classification.
       *
       *  This function finds a mask M for @a c and compares it to mask @a m.
       *
       *  do_is() is a hook for a derived facet to change the behavior of
       *  classifying.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __c  The char_type to find the mask of.
       *  @param __m  The mask to compare against.
       *  @return  (M & __m) != 0.
      */
      virtual bool
      do_is(mask __m, char_type __c) const = 0;

      /**
       *  @brief  Return a mask array.
       *
       *  This function finds the mask for each char_type in the range [lo,hi)
       *  and successively writes it to vec.  vec must have as many elements
       *  as the input.
       *
       *  do_is() is a hook for a derived facet to change the behavior of
       *  classifying.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __vec  Pointer to an array of mask storage.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_is(const char_type* __lo, const char_type* __hi,
	    mask* __vec) const = 0;

      /**
       *  @brief  Find char_type matching mask
       *
       *  This function searches for and returns the first char_type c in
       *  [__lo,__hi) for which is(__m,c) is true.
       *
       *  do_scan_is() is a hook for a derived facet to change the behavior of
       *  match searching.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a matching char_type if found, else @a __hi.
      */
      virtual const char_type*
      do_scan_is(mask __m, const char_type* __lo,
		 const char_type* __hi) const = 0;

      /**
       *  @brief  Find char_type not matching mask
       *
       *  This function searches for and returns a pointer to the first
       *  char_type c of [lo,hi) for which is(m,c) is false.
       *
       *  do_scan_is() is a hook for a derived facet to change the behavior of
       *  match searching.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a non-matching char_type if found, else @a __hi.
      */
      virtual const char_type*
      do_scan_not(mask __m, const char_type* __lo,
		  const char_type* __hi) const = 0;

      /**
       *  @brief  Convert to uppercase.
       *
       *  This virtual function converts the char_type argument to uppercase
       *  if possible.  If not possible (for example, '2'), returns the
       *  argument.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __c  The char_type to convert.
       *  @return  The uppercase char_type if convertible, else @a __c.
      */
      virtual char_type
      do_toupper(char_type __c) const = 0;

      /**
       *  @brief  Convert array to uppercase.
       *
       *  This virtual function converts each char_type in the range [__lo,__hi)
       *  to uppercase if possible.  Other elements remain untouched.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_toupper(char_type* __lo, const char_type* __hi) const = 0;

      /**
       *  @brief  Convert to lowercase.
       *
       *  This virtual function converts the argument to lowercase if
       *  possible.  If not possible (for example, '2'), returns the argument.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __c  The char_type to convert.
       *  @return  The lowercase char_type if convertible, else @a __c.
      */
      virtual char_type
      do_tolower(char_type __c) const = 0;

      /**
       *  @brief  Convert array to lowercase.
       *
       *  This virtual function converts each char_type in the range [__lo,__hi)
       *  to lowercase if possible.  Other elements remain untouched.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_tolower(char_type* __lo, const char_type* __hi) const = 0;

      /**
       *  @brief  Widen char
       *
       *  This virtual function converts the char to char_type using the
       *  simplest reasonable transformation.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @return  The converted char_type
      */
      virtual char_type
      do_widen(char __c) const = 0;

      /**
       *  @brief  Widen char array
       *
       *  This function converts each char in the input to char_type using the
       *  simplest reasonable transformation.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start range.
       *  @param __hi  Pointer to end of range.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char*
      do_widen(const char* __lo, const char* __hi, char_type* __to) const = 0;

      /**
       *  @brief  Narrow char_type to char
       *
       *  This virtual function converts the argument to char using the
       *  simplest reasonable transformation.  If the conversion fails, dfault
       *  is returned instead.
       *
       *  do_narrow() is a hook for a derived facet to change the behavior of
       *  narrowing.  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char_type to convert.
       *  @param __dfault  Char to return if conversion fails.
       *  @return  The converted char.
      */
      virtual char
      do_narrow(char_type __c, char __dfault) const = 0;

      /**
       *  @brief  Narrow char_type array to char
       *
       *  This virtual function converts each char_type in the range
       *  [__lo,__hi) to char using the simplest reasonable
       *  transformation and writes the results to the destination
       *  array.  For any element in the input that cannot be
       *  converted, @a __dfault is used instead.
       *
       *  do_narrow() is a hook for a derived facet to change the behavior of
       *  narrowing.  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __dfault  Char to use if conversion fails.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_narrow(const char_type* __lo, const char_type* __hi,
		char __dfault, char* __to) const = 0;
    };

  /**
   *  @brief  Primary class template ctype facet.
   *  @ingroup locales
   *
   *  This template class defines classification and conversion functions for
   *  character sets.  It wraps cctype functionality.  Ctype gets used by
   *  streams for many I/O operations.
   *
   *  This template provides the protected virtual functions the developer
   *  will have to replace in a derived class or specialization to make a
   *  working facet.  The public functions that access them are defined in
   *  __ctype_abstract_base, to allow for implementation flexibility.  See
   *  ctype<wchar_t> for an example.  The functions are documented in
   *  __ctype_abstract_base.
   *
   *  Note: implementations are provided for all the protected virtual
   *  functions, but will likely not be useful.
  */
  template<typename _CharT>
    class ctype : public __ctype_abstract_base<_CharT>
    {
    public:
      // Types:
      typedef _CharT			char_type;
      typedef typename __ctype_abstract_base<_CharT>::mask mask;

      /// The facet id for ctype<char_type>
      static locale::id			id;

      explicit
      ctype(size_t __refs = 0) : __ctype_abstract_base<_CharT>(__refs) { }

   protected:
      virtual
      ~ctype();

      virtual bool
      do_is(mask __m, char_type __c) const;

      virtual const char_type*
      do_is(const char_type* __lo, const char_type* __hi, mask* __vec) const;

      virtual const char_type*
      do_scan_is(mask __m, const char_type* __lo, const char_type* __hi) const;

      virtual const char_type*
      do_scan_not(mask __m, const char_type* __lo,
		  const char_type* __hi) const;

      virtual char_type
      do_toupper(char_type __c) const;

      virtual const char_type*
      do_toupper(char_type* __lo, const char_type* __hi) const;

      virtual char_type
      do_tolower(char_type __c) const;

      virtual const char_type*
      do_tolower(char_type* __lo, const char_type* __hi) const;

      virtual char_type
      do_widen(char __c) const;

      virtual const char*
      do_widen(const char* __lo, const char* __hi, char_type* __dest) const;

      virtual char
      do_narrow(char_type, char __dfault) const;

      virtual const char_type*
      do_narrow(const char_type* __lo, const char_type* __hi,
		char __dfault, char* __to) const;
    };

  template<typename _CharT>
    locale::id ctype<_CharT>::id;

  /**
   *  @brief  The ctype<char> specialization.
   *  @ingroup locales
   *
   *  This class defines classification and conversion functions for
   *  the char type.  It gets used by char streams for many I/O
   *  operations.  The char specialization provides a number of
   *  optimizations as well.
  */
  template<>
    class ctype<char> : public locale::facet, public ctype_base
    {
    public:
      // Types:
      /// Typedef for the template parameter char.
      typedef char		char_type;

    protected:
      // Data Members:
      __c_locale		_M_c_locale_ctype;
      bool			_M_del;
      __to_type			_M_toupper;
      __to_type			_M_tolower;
      const mask*		_M_table;
      mutable char		_M_widen_ok;
      mutable char		_M_widen[1 + static_cast<unsigned char>(-1)];
      mutable char		_M_narrow[1 + static_cast<unsigned char>(-1)];
      mutable char		_M_narrow_ok;	// 0 uninitialized, 1 init,
						// 2 memcpy can't be used

    public:
      /// The facet id for ctype<char>
      static locale::id        id;
      /// The size of the mask table.  It is SCHAR_MAX + 1.
      static const size_t      table_size = 1 + static_cast<unsigned char>(-1);

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __table If non-zero, table is used as the per-char mask.
       *               Else classic_table() is used.
       *  @param __del   If true, passes ownership of table to this facet.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      ctype(const mask* __table = 0, bool __del = false, size_t __refs = 0);

      /**
       *  @brief  Constructor performs static initialization.
       *
       *  This constructor is used to construct the initial C locale facet.
       *
       *  @param __cloc  Handle to C locale data.
       *  @param __table If non-zero, table is used as the per-char mask.
       *  @param __del   If true, passes ownership of table to this facet.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      ctype(__c_locale __cloc, const mask* __table = 0, bool __del = false,
	    size_t __refs = 0);

      /**
       *  @brief  Test char classification.
       *
       *  This function compares the mask table[c] to @a __m.
       *
       *  @param __c  The char to compare the mask of.
       *  @param __m  The mask to compare against.
       *  @return  True if __m & table[__c] is true, false otherwise.
      */
      inline bool
      is(mask __m, char __c) const;

      /**
       *  @brief  Return a mask array.
       *
       *  This function finds the mask for each char in the range [lo, hi) and
       *  successively writes it to vec.  vec must have as many elements as
       *  the char array.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __vec  Pointer to an array of mask storage.
       *  @return  @a __hi.
      */
      inline const char*
      is(const char* __lo, const char* __hi, mask* __vec) const;

      /**
       *  @brief  Find char matching a mask
       *
       *  This function searches for and returns the first char in [lo,hi) for
       *  which is(m,char) is true.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a matching char if found, else @a __hi.
      */
      inline const char*
      scan_is(mask __m, const char* __lo, const char* __hi) const;

      /**
       *  @brief  Find char not matching a mask
       *
       *  This function searches for and returns a pointer to the first char
       *  in [__lo,__hi) for which is(m,char) is false.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a non-matching char if found, else @a __hi.
      */
      inline const char*
      scan_not(mask __m, const char* __lo, const char* __hi) const;

      /**
       *  @brief  Convert to uppercase.
       *
       *  This function converts the char argument to uppercase if possible.
       *  If not possible (for example, '2'), returns the argument.
       *
       *  toupper() acts as if it returns ctype<char>::do_toupper(c).
       *  do_toupper() must always return the same result for the same input.
       *
       *  @param __c  The char to convert.
       *  @return  The uppercase char if convertible, else @a __c.
      */
      char_type
      toupper(char_type __c) const
      { return this->do_toupper(__c); }

      /**
       *  @brief  Convert array to uppercase.
       *
       *  This function converts each char in the range [__lo,__hi) to uppercase
       *  if possible.  Other chars remain untouched.
       *
       *  toupper() acts as if it returns ctype<char>:: do_toupper(__lo, __hi).
       *  do_toupper() must always return the same result for the same input.
       *
       *  @param __lo  Pointer to first char in range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      const char_type*
      toupper(char_type *__lo, const char_type* __hi) const
      { return this->do_toupper(__lo, __hi); }

      /**
       *  @brief  Convert to lowercase.
       *
       *  This function converts the char argument to lowercase if possible.
       *  If not possible (for example, '2'), returns the argument.
       *
       *  tolower() acts as if it returns ctype<char>::do_tolower(__c).
       *  do_tolower() must always return the same result for the same input.
       *
       *  @param __c  The char to convert.
       *  @return  The lowercase char if convertible, else @a __c.
      */
      char_type
      tolower(char_type __c) const
      { return this->do_tolower(__c); }

      /**
       *  @brief  Convert array to lowercase.
       *
       *  This function converts each char in the range [lo,hi) to lowercase
       *  if possible.  Other chars remain untouched.
       *
       *  tolower() acts as if it returns ctype<char>:: do_tolower(__lo, __hi).
       *  do_tolower() must always return the same result for the same input.
       *
       *  @param __lo  Pointer to first char in range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      const char_type*
      tolower(char_type* __lo, const char_type* __hi) const
      { return this->do_tolower(__lo, __hi); }

      /**
       *  @brief  Widen char
       *
       *  This function converts the char to char_type using the simplest
       *  reasonable transformation.  For an underived ctype<char> facet, the
       *  argument will be returned unchanged.
       *
       *  This function works as if it returns ctype<char>::do_widen(c).
       *  do_widen() must always return the same result for the same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @return  The converted character.
      */
      char_type
      widen(char __c) const
      {
	if (_M_widen_ok)
	  return _M_widen[static_cast<unsigned char>(__c)];
	this->_M_widen_init();
	return this->do_widen(__c);
      }

      /**
       *  @brief  Widen char array
       *
       *  This function converts each char in the input to char using the
       *  simplest reasonable transformation.  For an underived ctype<char>
       *  facet, the argument will be copied unchanged.
       *
       *  This function works as if it returns ctype<char>::do_widen(c).
       *  do_widen() must always return the same result for the same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to first char in range.
       *  @param __hi  Pointer to end of range.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      const char*
      widen(const char* __lo, const char* __hi, char_type* __to) const
      {
	if (_M_widen_ok == 1)
	  {
	    if (__builtin_expect(__hi != __lo, true))
	      __builtin_memcpy(__to, __lo, __hi - __lo);
	    return __hi;
	  }
	if (!_M_widen_ok)
	  _M_widen_init();
	return this->do_widen(__lo, __hi, __to);
      }

      /**
       *  @brief  Narrow char
       *
       *  This function converts the char to char using the simplest
       *  reasonable transformation.  If the conversion fails, dfault is
       *  returned instead.  For an underived ctype<char> facet, @a c
       *  will be returned unchanged.
       *
       *  This function works as if it returns ctype<char>::do_narrow(c).
       *  do_narrow() must always return the same result for the same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @param __dfault  Char to return if conversion fails.
       *  @return  The converted character.
      */
      char
      narrow(char_type __c, char __dfault) const
      {
	if (_M_narrow[static_cast<unsigned char>(__c)])
	  return _M_narrow[static_cast<unsigned char>(__c)];
	const char __t = do_narrow(__c, __dfault);
	if (__t != __dfault)
	  _M_narrow[static_cast<unsigned char>(__c)] = __t;
	return __t;
      }

      /**
       *  @brief  Narrow char array
       *
       *  This function converts each char in the input to char using the
       *  simplest reasonable transformation and writes the results to the
       *  destination array.  For any char in the input that cannot be
       *  converted, @a dfault is used instead.  For an underived ctype<char>
       *  facet, the argument will be copied unchanged.
       *
       *  This function works as if it returns ctype<char>::do_narrow(lo, hi,
       *  dfault, to).  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __dfault  Char to use if conversion fails.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      const char_type*
      narrow(const char_type* __lo, const char_type* __hi,
	     char __dfault, char* __to) const
      {
	if (__builtin_expect(_M_narrow_ok == 1, true))
	  {
	    if (__builtin_expect(__hi != __lo, true))
	      __builtin_memcpy(__to, __lo, __hi - __lo);
	    return __hi;
	  }
	if (!_M_narrow_ok)
	  _M_narrow_init();
	return this->do_narrow(__lo, __hi, __dfault, __to);
      }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // DR 695. ctype<char>::classic_table() not accessible.
      /// Returns a pointer to the mask table provided to the constructor, or
      /// the default from classic_table() if none was provided.
      const mask*
      table() const throw()
      { return _M_table; }

      /// Returns a pointer to the C locale mask table.
      static const mask*
      classic_table() throw();
    protected:

      /**
       *  @brief  Destructor.
       *
       *  This function deletes table() if @a del was true in the
       *  constructor.
      */
      virtual
      ~ctype();

      /**
       *  @brief  Convert to uppercase.
       *
       *  This virtual function converts the char argument to uppercase if
       *  possible.  If not possible (for example, '2'), returns the argument.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __c  The char to convert.
       *  @return  The uppercase char if convertible, else @a __c.
      */
      virtual char_type
      do_toupper(char_type __c) const;

      /**
       *  @brief  Convert array to uppercase.
       *
       *  This virtual function converts each char in the range [lo,hi) to
       *  uppercase if possible.  Other chars remain untouched.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_toupper(char_type* __lo, const char_type* __hi) const;

      /**
       *  @brief  Convert to lowercase.
       *
       *  This virtual function converts the char argument to lowercase if
       *  possible.  If not possible (for example, '2'), returns the argument.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __c  The char to convert.
       *  @return  The lowercase char if convertible, else @a __c.
      */
      virtual char_type
      do_tolower(char_type __c) const;

      /**
       *  @brief  Convert array to lowercase.
       *
       *  This virtual function converts each char in the range [lo,hi) to
       *  lowercase if possible.  Other chars remain untouched.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to first char in range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_tolower(char_type* __lo, const char_type* __hi) const;

      /**
       *  @brief  Widen char
       *
       *  This virtual function converts the char to char using the simplest
       *  reasonable transformation.  For an underived ctype<char> facet, the
       *  argument will be returned unchanged.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @return  The converted character.
      */
      virtual char_type
      do_widen(char __c) const
      { return __c; }

      /**
       *  @brief  Widen char array
       *
       *  This function converts each char in the range [lo,hi) to char using
       *  the simplest reasonable transformation.  For an underived
       *  ctype<char> facet, the argument will be copied unchanged.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char*
      do_widen(const char* __lo, const char* __hi, char_type* __to) const
      {
	if (__builtin_expect(__hi != __lo, true))
	  __builtin_memcpy(__to, __lo, __hi - __lo);
	return __hi;
      }

      /**
       *  @brief  Narrow char
       *
       *  This virtual function converts the char to char using the simplest
       *  reasonable transformation.  If the conversion fails, dfault is
       *  returned instead.  For an underived ctype<char> facet, @a c will be
       *  returned unchanged.
       *
       *  do_narrow() is a hook for a derived facet to change the behavior of
       *  narrowing.  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @param __dfault  Char to return if conversion fails.
       *  @return  The converted char.
      */
      virtual char
      do_narrow(char_type __c, char __dfault __attribute__((__unused__))) const
      { return __c; }

      /**
       *  @brief  Narrow char array to char array
       *
       *  This virtual function converts each char in the range [lo,hi) to
       *  char using the simplest reasonable transformation and writes the
       *  results to the destination array.  For any char in the input that
       *  cannot be converted, @a dfault is used instead.  For an underived
       *  ctype<char> facet, the argument will be copied unchanged.
       *
       *  do_narrow() is a hook for a derived facet to change the behavior of
       *  narrowing.  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __dfault  Char to use if conversion fails.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_narrow(const char_type* __lo, const char_type* __hi,
		char __dfault __attribute__((__unused__)), char* __to) const
      {
	if (__builtin_expect(__hi != __lo, true))
	  __builtin_memcpy(__to, __lo, __hi - __lo);
	return __hi;
      }

    private:
      void _M_narrow_init() const;
      void _M_widen_init() const;
    };

#ifdef _GLIBCXX_USE_WCHAR_T
  /**
   *  @brief  The ctype<wchar_t> specialization.
   *  @ingroup locales
   *
   *  This class defines classification and conversion functions for the
   *  wchar_t type.  It gets used by wchar_t streams for many I/O operations.
   *  The wchar_t specialization provides a number of optimizations as well.
   *
   *  ctype<wchar_t> inherits its public methods from
   *  __ctype_abstract_base<wchar_t>.
  */
  template<>
    class ctype<wchar_t> : public __ctype_abstract_base<wchar_t>
    {
    public:
      // Types:
      /// Typedef for the template parameter wchar_t.
      typedef wchar_t		char_type;
      typedef wctype_t		__wmask_type;

    protected:
      __c_locale		_M_c_locale_ctype;

      // Pre-computed narrowed and widened chars.
      bool                      _M_narrow_ok;
      char                      _M_narrow[128];
      wint_t                    _M_widen[1 + static_cast<unsigned char>(-1)];

      // Pre-computed elements for do_is.
      mask                      _M_bit[16];
      __wmask_type              _M_wmask[16];

    public:
      // Data Members:
      /// The facet id for ctype<wchar_t>
      static locale::id		id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      ctype(size_t __refs = 0);

      /**
       *  @brief  Constructor performs static initialization.
       *
       *  This constructor is used to construct the initial C locale facet.
       *
       *  @param __cloc  Handle to C locale data.
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      ctype(__c_locale __cloc, size_t __refs = 0);

    protected:
      __wmask_type
      _M_convert_to_wmask(const mask __m) const throw();

      /// Destructor
      virtual
      ~ctype();

      /**
       *  @brief  Test wchar_t classification.
       *
       *  This function finds a mask M for @a c and compares it to mask @a m.
       *
       *  do_is() is a hook for a derived facet to change the behavior of
       *  classifying.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __c  The wchar_t to find the mask of.
       *  @param __m  The mask to compare against.
       *  @return  (M & __m) != 0.
      */
      virtual bool
      do_is(mask __m, char_type __c) const;

      /**
       *  @brief  Return a mask array.
       *
       *  This function finds the mask for each wchar_t in the range [lo,hi)
       *  and successively writes it to vec.  vec must have as many elements
       *  as the input.
       *
       *  do_is() is a hook for a derived facet to change the behavior of
       *  classifying.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __vec  Pointer to an array of mask storage.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_is(const char_type* __lo, const char_type* __hi, mask* __vec) const;

      /**
       *  @brief  Find wchar_t matching mask
       *
       *  This function searches for and returns the first wchar_t c in
       *  [__lo,__hi) for which is(__m,c) is true.
       *
       *  do_scan_is() is a hook for a derived facet to change the behavior of
       *  match searching.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a matching wchar_t if found, else @a __hi.
      */
      virtual const char_type*
      do_scan_is(mask __m, const char_type* __lo, const char_type* __hi) const;

      /**
       *  @brief  Find wchar_t not matching mask
       *
       *  This function searches for and returns a pointer to the first
       *  wchar_t c of [__lo,__hi) for which is(__m,c) is false.
       *
       *  do_scan_is() is a hook for a derived facet to change the behavior of
       *  match searching.  do_is() must always return the same result for the
       *  same input.
       *
       *  @param __m  The mask to compare against.
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  Pointer to a non-matching wchar_t if found, else @a __hi.
      */
      virtual const char_type*
      do_scan_not(mask __m, const char_type* __lo,
		  const char_type* __hi) const;

      /**
       *  @brief  Convert to uppercase.
       *
       *  This virtual function converts the wchar_t argument to uppercase if
       *  possible.  If not possible (for example, '2'), returns the argument.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __c  The wchar_t to convert.
       *  @return  The uppercase wchar_t if convertible, else @a __c.
      */
      virtual char_type
      do_toupper(char_type __c) const;

      /**
       *  @brief  Convert array to uppercase.
       *
       *  This virtual function converts each wchar_t in the range [lo,hi) to
       *  uppercase if possible.  Other elements remain untouched.
       *
       *  do_toupper() is a hook for a derived facet to change the behavior of
       *  uppercasing.  do_toupper() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_toupper(char_type* __lo, const char_type* __hi) const;

      /**
       *  @brief  Convert to lowercase.
       *
       *  This virtual function converts the argument to lowercase if
       *  possible.  If not possible (for example, '2'), returns the argument.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __c  The wchar_t to convert.
       *  @return  The lowercase wchar_t if convertible, else @a __c.
      */
      virtual char_type
      do_tolower(char_type __c) const;

      /**
       *  @brief  Convert array to lowercase.
       *
       *  This virtual function converts each wchar_t in the range [lo,hi) to
       *  lowercase if possible.  Other elements remain untouched.
       *
       *  do_tolower() is a hook for a derived facet to change the behavior of
       *  lowercasing.  do_tolower() must always return the same result for
       *  the same input.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_tolower(char_type* __lo, const char_type* __hi) const;

      /**
       *  @brief  Widen char to wchar_t
       *
       *  This virtual function converts the char to wchar_t using the
       *  simplest reasonable transformation.  For an underived ctype<wchar_t>
       *  facet, the argument will be cast to wchar_t.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The char to convert.
       *  @return  The converted wchar_t.
      */
      virtual char_type
      do_widen(char __c) const;

      /**
       *  @brief  Widen char array to wchar_t array
       *
       *  This function converts each char in the input to wchar_t using the
       *  simplest reasonable transformation.  For an underived ctype<wchar_t>
       *  facet, the argument will be copied, casting each element to wchar_t.
       *
       *  do_widen() is a hook for a derived facet to change the behavior of
       *  widening.  do_widen() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start range.
       *  @param __hi  Pointer to end of range.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char*
      do_widen(const char* __lo, const char* __hi, char_type* __to) const;

      /**
       *  @brief  Narrow wchar_t to char
       *
       *  This virtual function converts the argument to char using
       *  the simplest reasonable transformation.  If the conversion
       *  fails, dfault is returned instead.  For an underived
       *  ctype<wchar_t> facet, @a c will be cast to char and
       *  returned.
       *
       *  do_narrow() is a hook for a derived facet to change the
       *  behavior of narrowing.  do_narrow() must always return the
       *  same result for the same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __c  The wchar_t to convert.
       *  @param __dfault  Char to return if conversion fails.
       *  @return  The converted char.
      */
      virtual char
      do_narrow(char_type __c, char __dfault) const;

      /**
       *  @brief  Narrow wchar_t array to char array
       *
       *  This virtual function converts each wchar_t in the range [lo,hi) to
       *  char using the simplest reasonable transformation and writes the
       *  results to the destination array.  For any wchar_t in the input that
       *  cannot be converted, @a dfault is used instead.  For an underived
       *  ctype<wchar_t> facet, the argument will be copied, casting each
       *  element to char.
       *
       *  do_narrow() is a hook for a derived facet to change the behavior of
       *  narrowing.  do_narrow() must always return the same result for the
       *  same input.
       *
       *  Note: this is not what you want for codepage conversions.  See
       *  codecvt for that.
       *
       *  @param __lo  Pointer to start of range.
       *  @param __hi  Pointer to end of range.
       *  @param __dfault  Char to use if conversion fails.
       *  @param __to  Pointer to the destination array.
       *  @return  @a __hi.
      */
      virtual const char_type*
      do_narrow(const char_type* __lo, const char_type* __hi,
		char __dfault, char* __to) const;

      // For use at construction time only.
      void
      _M_initialize_ctype() throw();
    };
#endif //_GLIBCXX_USE_WCHAR_T

  /// class ctype_byname [22.2.1.2].
  template<typename _CharT>
    class ctype_byname : public ctype<_CharT>
    {
    public:
      typedef typename ctype<_CharT>::mask  mask;

      explicit
      ctype_byname(const char* __s, size_t __refs = 0);

#if __cplusplus >= 201103L
      explicit
      ctype_byname(const string& __s, size_t __refs = 0)
      : ctype_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~ctype_byname() { }
    };

  /// 22.2.1.4  Class ctype_byname specializations.
  template<>
    class ctype_byname<char> : public ctype<char>
    {
    public:
      explicit
      ctype_byname(const char* __s, size_t __refs = 0);

#if __cplusplus >= 201103L
      explicit
      ctype_byname(const string& __s, size_t __refs = 0);
#endif

    protected:
      virtual
      ~ctype_byname();
    };

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    class ctype_byname<wchar_t> : public ctype<wchar_t>
    {
    public:
      explicit
      ctype_byname(const char* __s, size_t __refs = 0);

#if __cplusplus >= 201103L
      explicit
      ctype_byname(const string& __s, size_t __refs = 0);
#endif

    protected:
      virtual
      ~ctype_byname();
    };
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

// Include host and configuration specific ctype inlines.
#include <bits/ctype_inline.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // 22.2.2  The numeric category.
  class __num_base
  {
  public:
    // NB: Code depends on the order of _S_atoms_out elements.
    // Below are the indices into _S_atoms_out.
    enum
      {
	_S_ominus,
	_S_oplus,
	_S_ox,
	_S_oX,
	_S_odigits,
	_S_odigits_end = _S_odigits + 16,
	_S_oudigits = _S_odigits_end,
	_S_oudigits_end = _S_oudigits + 16,
	_S_oe = _S_odigits + 14,  // For scientific notation, 'e'
	_S_oE = _S_oudigits + 14, // For scientific notation, 'E'
	_S_oend = _S_oudigits_end
      };

    // A list of valid numeric literals for output.  This array
    // contains chars that will be passed through the current locale's
    // ctype<_CharT>.widen() and then used to render numbers.
    // For the standard "C" locale, this is
    // "-+xX0123456789abcdef0123456789ABCDEF".
    static const char* _S_atoms_out;

    // String literal of acceptable (narrow) input, for num_get.
    // "-+xX0123456789abcdefABCDEF"
    static const char* _S_atoms_in;

    enum
    {
      _S_iminus,
      _S_iplus,
      _S_ix,
      _S_iX,
      _S_izero,
      _S_ie = _S_izero + 14,
      _S_iE = _S_izero + 20,
      _S_iend = 26
    };

    // num_put
    // Construct and return valid scanf format for floating point types.
    static void
    _S_format_float(const ios_base& __io, char* __fptr, char __mod) throw();
  };

  template<typename _CharT>
    struct __numpunct_cache : public locale::facet
    {
      const char*			_M_grouping;
      size_t                            _M_grouping_size;
      bool				_M_use_grouping;
      const _CharT*			_M_truename;
      size_t                            _M_truename_size;
      const _CharT*			_M_falsename;
      size_t                            _M_falsename_size;
      _CharT				_M_decimal_point;
      _CharT				_M_thousands_sep;

      // A list of valid numeric literals for output: in the standard
      // "C" locale, this is "-+xX0123456789abcdef0123456789ABCDEF".
      // This array contains the chars after having been passed
      // through the current locale's ctype<_CharT>.widen().
      _CharT				_M_atoms_out[__num_base::_S_oend];

      // A list of valid numeric literals for input: in the standard
      // "C" locale, this is "-+xX0123456789abcdefABCDEF"
      // This array contains the chars after having been passed
      // through the current locale's ctype<_CharT>.widen().
      _CharT				_M_atoms_in[__num_base::_S_iend];

      bool				_M_allocated;

      __numpunct_cache(size_t __refs = 0)
      : facet(__refs), _M_grouping(0), _M_grouping_size(0),
	_M_use_grouping(false),
	_M_truename(0), _M_truename_size(0), _M_falsename(0),
	_M_falsename_size(0), _M_decimal_point(_CharT()),
	_M_thousands_sep(_CharT()), _M_allocated(false)
	{ }

      ~__numpunct_cache();

      void
      _M_cache(const locale& __loc);

    private:
      __numpunct_cache&
      operator=(const __numpunct_cache&);

      explicit
      __numpunct_cache(const __numpunct_cache&);
    };

  template<typename _CharT>
    __numpunct_cache<_CharT>::~__numpunct_cache()
    {
      if (_M_allocated)
	{
	  delete [] _M_grouping;
	  delete [] _M_truename;
	  delete [] _M_falsename;
	}
    }

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  /**
   *  @brief  Primary class template numpunct.
   *  @ingroup locales
   *
   *  This facet stores several pieces of information related to printing and
   *  scanning numbers, such as the decimal point character.  It takes a
   *  template parameter specifying the char type.  The numpunct facet is
   *  used by streams for many I/O operations involving numbers.
   *
   *  The numpunct template uses protected virtual functions to provide the
   *  actual results.  The public accessors forward the call to the virtual
   *  functions.  These virtual functions are hooks for developers to
   *  implement the behavior they require from a numpunct facet.
  */
  template<typename _CharT>
    class numpunct : public locale::facet
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;
      //@}
      typedef __numpunct_cache<_CharT>  __cache_type;

    protected:
      __cache_type*			_M_data;

    public:
      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Numpunct constructor.
       *
       *  @param  __refs  Refcount to pass to the base class.
       */
      explicit
      numpunct(size_t __refs = 0)
      : facet(__refs), _M_data(0)
      { _M_initialize_numpunct(); }

      /**
       *  @brief  Internal constructor.  Not for general use.
       *
       *  This is a constructor for use by the library itself to set up the
       *  predefined locale facets.
       *
       *  @param  __cache  __numpunct_cache object.
       *  @param  __refs  Refcount to pass to the base class.
       */
      explicit
      numpunct(__cache_type* __cache, size_t __refs = 0)
      : facet(__refs), _M_data(__cache)
      { _M_initialize_numpunct(); }

      /**
       *  @brief  Internal constructor.  Not for general use.
       *
       *  This is a constructor for use by the library itself to set up new
       *  locales.
       *
       *  @param  __cloc  The C locale.
       *  @param  __refs  Refcount to pass to the base class.
       */
      explicit
      numpunct(__c_locale __cloc, size_t __refs = 0)
      : facet(__refs), _M_data(0)
      { _M_initialize_numpunct(__cloc); }

      /**
       *  @brief  Return decimal point character.
       *
       *  This function returns a char_type to use as a decimal point.  It
       *  does so by returning returning
       *  numpunct<char_type>::do_decimal_point().
       *
       *  @return  @a char_type representing a decimal point.
      */
      char_type
      decimal_point() const
      { return this->do_decimal_point(); }

      /**
       *  @brief  Return thousands separator character.
       *
       *  This function returns a char_type to use as a thousands
       *  separator.  It does so by returning returning
       *  numpunct<char_type>::do_thousands_sep().
       *
       *  @return  char_type representing a thousands separator.
      */
      char_type
      thousands_sep() const
      { return this->do_thousands_sep(); }

      /**
       *  @brief  Return grouping specification.
       *
       *  This function returns a string representing groupings for the
       *  integer part of a number.  Groupings indicate where thousands
       *  separators should be inserted in the integer part of a number.
       *
       *  Each char in the return string is interpret as an integer
       *  rather than a character.  These numbers represent the number
       *  of digits in a group.  The first char in the string
       *  represents the number of digits in the least significant
       *  group.  If a char is negative, it indicates an unlimited
       *  number of digits for the group.  If more chars from the
       *  string are required to group a number, the last char is used
       *  repeatedly.
       *
       *  For example, if the grouping() returns "\003\002" and is
       *  applied to the number 123456789, this corresponds to
       *  12,34,56,789.  Note that if the string was "32", this would
       *  put more than 50 digits into the least significant group if
       *  the character set is ASCII.
       *
       *  The string is returned by calling
       *  numpunct<char_type>::do_grouping().
       *
       *  @return  string representing grouping specification.
      */
      string
      grouping() const
      { return this->do_grouping(); }

      /**
       *  @brief  Return string representation of bool true.
       *
       *  This function returns a string_type containing the text
       *  representation for true bool variables.  It does so by calling
       *  numpunct<char_type>::do_truename().
       *
       *  @return  string_type representing printed form of true.
      */
      string_type
      truename() const
      { return this->do_truename(); }

      /**
       *  @brief  Return string representation of bool false.
       *
       *  This function returns a string_type containing the text
       *  representation for false bool variables.  It does so by calling
       *  numpunct<char_type>::do_falsename().
       *
       *  @return  string_type representing printed form of false.
      */
      string_type
      falsename() const
      { return this->do_falsename(); }

    protected:
      /// Destructor.
      virtual
      ~numpunct();

      /**
       *  @brief  Return decimal point character.
       *
       *  Returns a char_type to use as a decimal point.  This function is a
       *  hook for derived classes to change the value returned.
       *
       *  @return  @a char_type representing a decimal point.
      */
      virtual char_type
      do_decimal_point() const
      { return _M_data->_M_decimal_point; }

      /**
       *  @brief  Return thousands separator character.
       *
       *  Returns a char_type to use as a thousands separator.  This function
       *  is a hook for derived classes to change the value returned.
       *
       *  @return  @a char_type representing a thousands separator.
      */
      virtual char_type
      do_thousands_sep() const
      { return _M_data->_M_thousands_sep; }

      /**
       *  @brief  Return grouping specification.
       *
       *  Returns a string representing groupings for the integer part of a
       *  number.  This function is a hook for derived classes to change the
       *  value returned.  @see grouping() for details.
       *
       *  @return  String representing grouping specification.
      */
      virtual string
      do_grouping() const
      { return _M_data->_M_grouping; }

      /**
       *  @brief  Return string representation of bool true.
       *
       *  Returns a string_type containing the text representation for true
       *  bool variables.  This function is a hook for derived classes to
       *  change the value returned.
       *
       *  @return  string_type representing printed form of true.
      */
      virtual string_type
      do_truename() const
      { return _M_data->_M_truename; }

      /**
       *  @brief  Return string representation of bool false.
       *
       *  Returns a string_type containing the text representation for false
       *  bool variables.  This function is a hook for derived classes to
       *  change the value returned.
       *
       *  @return  string_type representing printed form of false.
      */
      virtual string_type
      do_falsename() const
      { return _M_data->_M_falsename; }

      // For use at construction time only.
      void
      _M_initialize_numpunct(__c_locale __cloc = 0);
    };

  template<typename _CharT>
    locale::id numpunct<_CharT>::id;

  template<>
    numpunct<char>::~numpunct();

  template<>
    void
    numpunct<char>::_M_initialize_numpunct(__c_locale __cloc);

#ifdef _GLIBCXX_USE_WCHAR_T
  template<>
    numpunct<wchar_t>::~numpunct();

  template<>
    void
    numpunct<wchar_t>::_M_initialize_numpunct(__c_locale __cloc);
#endif

  /// class numpunct_byname [22.2.3.2].
  template<typename _CharT>
    class numpunct_byname : public numpunct<_CharT>
    {
    public:
      typedef _CharT			char_type;
      typedef basic_string<_CharT>	string_type;

      explicit
      numpunct_byname(const char* __s, size_t __refs = 0)
      : numpunct<_CharT>(__refs)
      {
	if (__builtin_strcmp(__s, "C") != 0
	    && __builtin_strcmp(__s, "POSIX") != 0)
	  {
	    __c_locale __tmp;
	    this->_S_create_c_locale(__tmp, __s);
	    this->_M_initialize_numpunct(__tmp);
	    this->_S_destroy_c_locale(__tmp);
	  }
      }

#if __cplusplus >= 201103L
      explicit
      numpunct_byname(const string& __s, size_t __refs = 0)
      : numpunct_byname(__s.c_str(), __refs) { }
#endif

    protected:
      virtual
      ~numpunct_byname() { }
    };

_GLIBCXX_END_NAMESPACE_CXX11

_GLIBCXX_BEGIN_NAMESPACE_LDBL

  /**
   *  @brief  Primary class template num_get.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to parse and return a number
   *  from a string.  It is used by the istream numeric extraction
   *  operators.
   *
   *  The num_get template uses protected virtual functions to provide the
   *  actual results.  The public accessors forward the call to the virtual
   *  functions.  These virtual functions are hooks for developers to
   *  implement the behavior they require from the num_get facet.
  */
  template<typename _CharT, typename _InIter>
    class num_get : public locale::facet
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT			char_type;
      typedef _InIter			iter_type;
      //@}

      /// Numpunct facet id.
      static locale::id			id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      num_get(size_t __refs = 0) : facet(__refs) { }

      /**
       *  @brief  Numeric parsing.
       *
       *  Parses the input stream into the bool @a v.  It does so by calling
       *  num_get::do_get().
       *
       *  If ios_base::boolalpha is set, attempts to read
       *  ctype<CharT>::truename() or ctype<CharT>::falsename().  Sets
       *  @a v to true or false if successful.  Sets err to
       *  ios_base::failbit if reading the string fails.  Sets err to
       *  ios_base::eofbit if the stream is emptied.
       *
       *  If ios_base::boolalpha is not set, proceeds as with reading a long,
       *  except if the value is 1, sets @a v to true, if the value is 0, sets
       *  @a v to false, and otherwise set err to ios_base::failbit.
       *
       *  @param  __in  Start of input stream.
       *  @param  __end  End of input stream.
       *  @param  __io  Source of locale and flags.
       *  @param  __err  Error flags to set.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after reading.
      */
      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, bool& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      //@{
      /**
       *  @brief  Numeric parsing.
       *
       *  Parses the input stream into the integral variable @a v.  It does so
       *  by calling num_get::do_get().
       *
       *  Parsing is affected by the flag settings in @a io.
       *
       *  The basic parse is affected by the value of io.flags() &
       *  ios_base::basefield.  If equal to ios_base::oct, parses like the
       *  scanf %o specifier.  Else if equal to ios_base::hex, parses like %X
       *  specifier.  Else if basefield equal to 0, parses like the %i
       *  specifier.  Otherwise, parses like %d for signed and %u for unsigned
       *  types.  The matching type length modifier is also used.
       *
       *  Digit grouping is interpreted according to
       *  numpunct::grouping() and numpunct::thousands_sep().  If the
       *  pattern of digit groups isn't consistent, sets err to
       *  ios_base::failbit.
       *
       *  If parsing the string yields a valid value for @a v, @a v is set.
       *  Otherwise, sets err to ios_base::failbit and leaves @a v unaltered.
       *  Sets err to ios_base::eofbit if the stream is emptied.
       *
       *  @param  __in  Start of input stream.
       *  @param  __end  End of input stream.
       *  @param  __io  Source of locale and flags.
       *  @param  __err  Error flags to set.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after reading.
      */
      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, long& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, unsigned short& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, unsigned int& __v)   const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, unsigned long& __v)  const
      { return this->do_get(__in, __end, __io, __err, __v); }

#ifdef _GLIBCXX_USE_LONG_LONG
      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, long long& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, unsigned long long& __v)  const
      { return this->do_get(__in, __end, __io, __err, __v); }
#endif
      //@}

      //@{
      /**
       *  @brief  Numeric parsing.
       *
       *  Parses the input stream into the integral variable @a v.  It does so
       *  by calling num_get::do_get().
       *
       *  The input characters are parsed like the scanf %g specifier.  The
       *  matching type length modifier is also used.
       *
       *  The decimal point character used is numpunct::decimal_point().
       *  Digit grouping is interpreted according to
       *  numpunct::grouping() and numpunct::thousands_sep().  If the
       *  pattern of digit groups isn't consistent, sets err to
       *  ios_base::failbit.
       *
       *  If parsing the string yields a valid value for @a v, @a v is set.
       *  Otherwise, sets err to ios_base::failbit and leaves @a v unaltered.
       *  Sets err to ios_base::eofbit if the stream is emptied.
       *
       *  @param  __in  Start of input stream.
       *  @param  __end  End of input stream.
       *  @param  __io  Source of locale and flags.
       *  @param  __err  Error flags to set.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after reading.
      */
      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, float& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, double& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, long double& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }
      //@}

      /**
       *  @brief  Numeric parsing.
       *
       *  Parses the input stream into the pointer variable @a v.  It does so
       *  by calling num_get::do_get().
       *
       *  The input characters are parsed like the scanf %p specifier.
       *
       *  Digit grouping is interpreted according to
       *  numpunct::grouping() and numpunct::thousands_sep().  If the
       *  pattern of digit groups isn't consistent, sets err to
       *  ios_base::failbit.
       *
       *  Note that the digit grouping effect for pointers is a bit ambiguous
       *  in the standard and shouldn't be relied on.  See DR 344.
       *
       *  If parsing the string yields a valid value for @a v, @a v is set.
       *  Otherwise, sets err to ios_base::failbit and leaves @a v unaltered.
       *  Sets err to ios_base::eofbit if the stream is emptied.
       *
       *  @param  __in  Start of input stream.
       *  @param  __end  End of input stream.
       *  @param  __io  Source of locale and flags.
       *  @param  __err  Error flags to set.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after reading.
      */
      iter_type
      get(iter_type __in, iter_type __end, ios_base& __io,
	  ios_base::iostate& __err, void*& __v) const
      { return this->do_get(__in, __end, __io, __err, __v); }

    protected:
      /// Destructor.
      virtual ~num_get() { }

      _GLIBCXX_DEFAULT_ABI_TAG
      iter_type
      _M_extract_float(iter_type, iter_type, ios_base&, ios_base::iostate&,
		       string&) const;

      template<typename _ValueT>
	_GLIBCXX_DEFAULT_ABI_TAG
	iter_type
	_M_extract_int(iter_type, iter_type, ios_base&, ios_base::iostate&,
		       _ValueT&) const;

      template<typename _CharT2>
      typename __gnu_cxx::__enable_if<__is_char<_CharT2>::__value, int>::__type
	_M_find(const _CharT2*, size_t __len, _CharT2 __c) const
	{
	  int __ret = -1;
	  if (__len <= 10)
	    {
	      if (__c >= _CharT2('0') && __c < _CharT2(_CharT2('0') + __len))
		__ret = __c - _CharT2('0');
	    }
	  else
	    {
	      if (__c >= _CharT2('0') && __c <= _CharT2('9'))
		__ret = __c - _CharT2('0');
	      else if (__c >= _CharT2('a') && __c <= _CharT2('f'))
		__ret = 10 + (__c - _CharT2('a'));
	      else if (__c >= _CharT2('A') && __c <= _CharT2('F'))
		__ret = 10 + (__c - _CharT2('A'));
	    }
	  return __ret;
	}

      template<typename _CharT2>
      typename __gnu_cxx::__enable_if<!__is_char<_CharT2>::__value,
				      int>::__type
	_M_find(const _CharT2* __zero, size_t __len, _CharT2 __c) const
	{
	  int __ret = -1;
	  const char_type* __q = char_traits<_CharT2>::find(__zero, __len, __c);
	  if (__q)
	    {
	      __ret = __q - __zero;
	      if (__ret > 15)
		__ret -= 6;
	    }
	  return __ret;
	}

      //@{
      /**
       *  @brief  Numeric parsing.
       *
       *  Parses the input stream into the variable @a v.  This function is a
       *  hook for derived classes to change the value returned.  @see get()
       *  for more details.
       *
       *  @param  __beg  Start of input stream.
       *  @param  __end  End of input stream.
       *  @param  __io  Source of locale and flags.
       *  @param  __err  Error flags to set.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after reading.
      */
      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&, bool&) const;

      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, long& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }

      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, unsigned short& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }

      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, unsigned int& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }

      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, unsigned long& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }

#ifdef _GLIBCXX_USE_LONG_LONG
      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, long long& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }

      virtual iter_type
      do_get(iter_type __beg, iter_type __end, ios_base& __io,
	     ios_base::iostate& __err, unsigned long long& __v) const
      { return _M_extract_int(__beg, __end, __io, __err, __v); }
#endif

      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&, float&) const;

      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&,
	     double&) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__
      virtual iter_type
      __do_get(iter_type, iter_type, ios_base&, ios_base::iostate&,
	       double&) const;
#else
      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&,
	     long double&) const;
#endif

      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&, void*&) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__
      virtual iter_type
      do_get(iter_type, iter_type, ios_base&, ios_base::iostate&,
	     long double&) const;
#endif
      //@}
    };

  template<typename _CharT, typename _InIter>
    locale::id num_get<_CharT, _InIter>::id;


  /**
   *  @brief  Primary class template num_put.
   *  @ingroup locales
   *
   *  This facet encapsulates the code to convert a number to a string.  It is
   *  used by the ostream numeric insertion operators.
   *
   *  The num_put template uses protected virtual functions to provide the
   *  actual results.  The public accessors forward the call to the virtual
   *  functions.  These virtual functions are hooks for developers to
   *  implement the behavior they require from the num_put facet.
  */
  template<typename _CharT, typename _OutIter>
    class num_put : public locale::facet
    {
    public:
      // Types:
      //@{
      /// Public typedefs
      typedef _CharT		char_type;
      typedef _OutIter		iter_type;
      //@}

      /// Numpunct facet id.
      static locale::id		id;

      /**
       *  @brief  Constructor performs initialization.
       *
       *  This is the constructor provided by the standard.
       *
       *  @param __refs  Passed to the base facet class.
      */
      explicit
      num_put(size_t __refs = 0) : facet(__refs) { }

      /**
       *  @brief  Numeric formatting.
       *
       *  Formats the boolean @a v and inserts it into a stream.  It does so
       *  by calling num_put::do_put().
       *
       *  If ios_base::boolalpha is set, writes ctype<CharT>::truename() or
       *  ctype<CharT>::falsename().  Otherwise formats @a v as an int.
       *
       *  @param  __s  Stream to write to.
       *  @param  __io  Source of locale and flags.
       *  @param  __fill  Char_type to use for filling.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after writing.
      */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill, bool __v) const
      { return this->do_put(__s, __io, __fill, __v); }

      //@{
      /**
       *  @brief  Numeric formatting.
       *
       *  Formats the integral value @a v and inserts it into a
       *  stream.  It does so by calling num_put::do_put().
       *
       *  Formatting is affected by the flag settings in @a io.
       *
       *  The basic format is affected by the value of io.flags() &
       *  ios_base::basefield.  If equal to ios_base::oct, formats like the
       *  printf %o specifier.  Else if equal to ios_base::hex, formats like
       *  %x or %X with ios_base::uppercase unset or set respectively.
       *  Otherwise, formats like %d, %ld, %lld for signed and %u, %lu, %llu
       *  for unsigned values.  Note that if both oct and hex are set, neither
       *  will take effect.
       *
       *  If ios_base::showpos is set, '+' is output before positive values.
       *  If ios_base::showbase is set, '0' precedes octal values (except 0)
       *  and '0[xX]' precedes hex values.
       *
       *  The decimal point character used is numpunct::decimal_point().
       *  Thousands separators are inserted according to
       *  numpunct::grouping() and numpunct::thousands_sep().
       *
       *  If io.width() is non-zero, enough @a fill characters are inserted to
       *  make the result at least that wide.  If
       *  (io.flags() & ios_base::adjustfield) == ios_base::left, result is
       *  padded at the end.  If ios_base::internal, then padding occurs
       *  immediately after either a '+' or '-' or after '0x' or '0X'.
       *  Otherwise, padding occurs at the beginning.
       *
       *  @param  __s  Stream to write to.
       *  @param  __io  Source of locale and flags.
       *  @param  __fill  Char_type to use for filling.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after writing.
      */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill, long __v) const
      { return this->do_put(__s, __io, __fill, __v); }

      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill,
	  unsigned long __v) const
      { return this->do_put(__s, __io, __fill, __v); }

#ifdef _GLIBCXX_USE_LONG_LONG
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill, long long __v) const
      { return this->do_put(__s, __io, __fill, __v); }

      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill,
	  unsigned long long __v) const
      { return this->do_put(__s, __io, __fill, __v); }
#endif
      //@}

      //@{
      /**
       *  @brief  Numeric formatting.
       *
       *  Formats the floating point value @a v and inserts it into a stream.
       *  It does so by calling num_put::do_put().
       *
       *  Formatting is affected by the flag settings in @a io.
       *
       *  The basic format is affected by the value of io.flags() &
       *  ios_base::floatfield.  If equal to ios_base::fixed, formats like the
       *  printf %f specifier.  Else if equal to ios_base::scientific, formats
       *  like %e or %E with ios_base::uppercase unset or set respectively.
       *  Otherwise, formats like %g or %G depending on uppercase.  Note that
       *  if both fixed and scientific are set, the effect will also be like
       *  %g or %G.
       *
       *  The output precision is given by io.precision().  This precision is
       *  capped at numeric_limits::digits10 + 2 (different for double and
       *  long double).  The default precision is 6.
       *
       *  If ios_base::showpos is set, '+' is output before positive values.
       *  If ios_base::showpoint is set, a decimal point will always be
       *  output.
       *
       *  The decimal point character used is numpunct::decimal_point().
       *  Thousands separators are inserted according to
       *  numpunct::grouping() and numpunct::thousands_sep().
       *
       *  If io.width() is non-zero, enough @a fill characters are inserted to
       *  make the result at least that wide.  If
       *  (io.flags() & ios_base::adjustfield) == ios_base::left, result is
       *  padded at the end.  If ios_base::internal, then padding occurs
       *  immediately after either a '+' or '-' or after '0x' or '0X'.
       *  Otherwise, padding occurs at the beginning.
       *
       *  @param  __s  Stream to write to.
       *  @param  __io  Source of locale and flags.
       *  @param  __fill  Char_type to use for filling.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after writing.
      */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill, double __v) const
      { return this->do_put(__s, __io, __fill, __v); }

      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill,
	  long double __v) const
      { return this->do_put(__s, __io, __fill, __v); }
      //@}

      /**
       *  @brief  Numeric formatting.
       *
       *  Formats the pointer value @a v and inserts it into a stream.  It
       *  does so by calling num_put::do_put().
       *
       *  This function formats @a v as an unsigned long with ios_base::hex
       *  and ios_base::showbase set.
       *
       *  @param  __s  Stream to write to.
       *  @param  __io  Source of locale and flags.
       *  @param  __fill  Char_type to use for filling.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after writing.
      */
      iter_type
      put(iter_type __s, ios_base& __io, char_type __fill,
	  const void* __v) const
      { return this->do_put(__s, __io, __fill, __v); }

    protected:
      template<typename _ValueT>
	iter_type
	_M_insert_float(iter_type, ios_base& __io, char_type __fill,
			char __mod, _ValueT __v) const;

      void
      _M_group_float(const char* __grouping, size_t __grouping_size,
		     char_type __sep, const char_type* __p, char_type* __new,
		     char_type* __cs, int& __len) const;

      template<typename _ValueT>
	iter_type
	_M_insert_int(iter_type, ios_base& __io, char_type __fill,
		      _ValueT __v) const;

      void
      _M_group_int(const char* __grouping, size_t __grouping_size,
		   char_type __sep, ios_base& __io, char_type* __new,
		   char_type* __cs, int& __len) const;

      void
      _M_pad(char_type __fill, streamsize __w, ios_base& __io,
	     char_type* __new, const char_type* __cs, int& __len) const;

      /// Destructor.
      virtual
      ~num_put() { }

      //@{
      /**
       *  @brief  Numeric formatting.
       *
       *  These functions do the work of formatting numeric values and
       *  inserting them into a stream. This function is a hook for derived
       *  classes to change the value returned.
       *
       *  @param  __s  Stream to write to.
       *  @param  __io  Source of locale and flags.
       *  @param  __fill  Char_type to use for filling.
       *  @param  __v  Value to format and insert.
       *  @return  Iterator after writing.
      */
      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill, bool __v) const;

      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill, long __v) const
      { return _M_insert_int(__s, __io, __fill, __v); }

      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill,
	     unsigned long __v) const
      { return _M_insert_int(__s, __io, __fill, __v); }

#ifdef _GLIBCXX_USE_LONG_LONG
      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill,
	     long long __v) const
      { return _M_insert_int(__s, __io, __fill, __v); }

      virtual iter_type
      do_put(iter_type __s, ios_base& __io, char_type __fill,
	     unsigned long long __v) const
      { return _M_insert_int(__s, __io, __fill, __v); }
#endif

      virtual iter_type
      do_put(iter_type, ios_base&, char_type, double) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__
      virtual iter_type
      __do_put(iter_type, ios_base&, char_type, double) const;
#else
      virtual iter_type
      do_put(iter_type, ios_base&, char_type, long double) const;
#endif

      virtual iter_type
      do_put(iter_type, ios_base&, char_type, const void*) const;

      // XXX GLIBCXX_ABI Deprecated
#if defined _GLIBCXX_LONG_DOUBLE_COMPAT && defined __LONG_DOUBLE_128__
      virtual iter_type
      do_put(iter_type, ios_base&, char_type, long double) const;
#endif
      //@}
    };

  template <typename _CharT, typename _OutIter>
    locale::id num_put<_CharT, _OutIter>::id;

_GLIBCXX_END_NAMESPACE_LDBL

  // Subclause convenience interfaces, inlines.
  // NB: These are inline because, when used in a loop, some compilers
  // can hoist the body out of the loop; then it's just as fast as the
  // C is*() function.

  /// Convenience interface to ctype.is(ctype_base::space, __c).
  template<typename _CharT>
    inline bool
    isspace(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::space, __c); }

  /// Convenience interface to ctype.is(ctype_base::print, __c).
  template<typename _CharT>
    inline bool
    isprint(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::print, __c); }

  /// Convenience interface to ctype.is(ctype_base::cntrl, __c).
  template<typename _CharT>
    inline bool
    iscntrl(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::cntrl, __c); }

  /// Convenience interface to ctype.is(ctype_base::upper, __c).
  template<typename _CharT>
    inline bool
    isupper(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::upper, __c); }

  /// Convenience interface to ctype.is(ctype_base::lower, __c).
  template<typename _CharT>
    inline bool
    islower(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::lower, __c); }

  /// Convenience interface to ctype.is(ctype_base::alpha, __c).
  template<typename _CharT>
    inline bool
    isalpha(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::alpha, __c); }

  /// Convenience interface to ctype.is(ctype_base::digit, __c).
  template<typename _CharT>
    inline bool
    isdigit(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::digit, __c); }

  /// Convenience interface to ctype.is(ctype_base::punct, __c).
  template<typename _CharT>
    inline bool
    ispunct(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::punct, __c); }

  /// Convenience interface to ctype.is(ctype_base::xdigit, __c).
  template<typename _CharT>
    inline bool
    isxdigit(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::xdigit, __c); }

  /// Convenience interface to ctype.is(ctype_base::alnum, __c).
  template<typename _CharT>
    inline bool
    isalnum(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::alnum, __c); }

  /// Convenience interface to ctype.is(ctype_base::graph, __c).
  template<typename _CharT>
    inline bool
    isgraph(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::graph, __c); }

#if __cplusplus >= 201103L
  /// Convenience interface to ctype.is(ctype_base::blank, __c).
  template<typename _CharT>
    inline bool
    isblank(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).is(ctype_base::blank, __c); }
#endif

  /// Convenience interface to ctype.toupper(__c).
  template<typename _CharT>
    inline _CharT
    toupper(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).toupper(__c); }

  /// Convenience interface to ctype.tolower(__c).
  template<typename _CharT>
    inline _CharT
    tolower(_CharT __c, const locale& __loc)
    { return use_facet<ctype<_CharT> >(__loc).tolower(__c); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

# include <bits/locale_facets.tcc>

#endif
