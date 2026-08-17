// class template regex -*- C++ -*-

// Copyright (C) 2010-2020 Free Software Foundation, Inc.
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
 *  @file bits/regex.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CXX11
  template<typename, typename>
    class basic_regex;

  template<typename, typename>
    class match_results;

_GLIBCXX_END_NAMESPACE_CXX11

namespace __detail
{
  enum class _RegexExecutorPolicy : int { _S_auto, _S_alternate };

  template<typename _BiIter, typename _Alloc,
	   typename _CharT, typename _TraitsT,
	   _RegexExecutorPolicy __policy,
	   bool __match_mode>
    bool
    __regex_algo_impl(_BiIter			      __s,
		      _BiIter			      __e,
		      match_results<_BiIter, _Alloc>&      __m,
		      const basic_regex<_CharT, _TraitsT>& __re,
		      regex_constants::match_flag_type     __flags);

  template<typename, typename, typename, bool>
    class _Executor;
}

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  /**
   * @addtogroup regex
   * @{
   */

  /**
   * @brief Describes aspects of a regular expression.
   *
   * A regular expression traits class that satisfies the requirements of
   * section [28.7].
   *
   * The class %regex is parameterized around a set of related types and
   * functions used to complete the definition of its semantics.  This class
   * satisfies the requirements of such a traits class.
   */
  template<typename _Ch_type>
    struct regex_traits
    {
    public:
      typedef _Ch_type				char_type;
      typedef std::basic_string<char_type>	string_type;
      typedef std::locale			locale_type;
    private:
      struct _RegexMask
	{
	  typedef std::ctype_base::mask _BaseType;
	  _BaseType _M_base;
	  unsigned char _M_extended;
	  static constexpr unsigned char _S_under = 1 << 0;
	  static constexpr unsigned char _S_valid_mask = 0x1;

	  constexpr _RegexMask(_BaseType __base = 0,
			       unsigned char __extended = 0)
	  : _M_base(__base), _M_extended(__extended)
	  { }

	  constexpr _RegexMask
	  operator&(_RegexMask __other) const
	  {
	    return _RegexMask(_M_base & __other._M_base,
			      _M_extended & __other._M_extended);
	  }

	  constexpr _RegexMask
	  operator|(_RegexMask __other) const
	  {
	    return _RegexMask(_M_base | __other._M_base,
			      _M_extended | __other._M_extended);
	  }

	  constexpr _RegexMask
	  operator^(_RegexMask __other) const
	  {
	    return _RegexMask(_M_base ^ __other._M_base,
			      _M_extended ^ __other._M_extended);
	  }

	  constexpr _RegexMask
	  operator~() const
	  { return _RegexMask(~_M_base, ~_M_extended); }

	  _RegexMask&
	  operator&=(_RegexMask __other)
	  { return *this = (*this) & __other; }

	  _RegexMask&
	  operator|=(_RegexMask __other)
	  { return *this = (*this) | __other; }

	  _RegexMask&
	  operator^=(_RegexMask __other)
	  { return *this = (*this) ^ __other; }

	  constexpr bool
	  operator==(_RegexMask __other) const
	  {
	    return (_M_extended & _S_valid_mask)
		   == (__other._M_extended & _S_valid_mask)
		     && _M_base == __other._M_base;
	  }

#if __cpp_impl_three_way_comparison < 201907L
	  constexpr bool
	  operator!=(_RegexMask __other) const
	  { return !((*this) == __other); }
#endif
	};

    public:
      typedef _RegexMask char_class_type;

    public:
      /**
       * @brief Constructs a default traits object.
       */
      regex_traits() { }

      /**
       * @brief Gives the length of a C-style string starting at @p __p.
       *
       * @param __p a pointer to the start of a character sequence.
       *
       * @returns the number of characters between @p *__p and the first
       * default-initialized value of type @p char_type.  In other words, uses
       * the C-string algorithm for determining the length of a sequence of
       * characters.
       */
      static std::size_t
      length(const char_type* __p)
      { return string_type::traits_type::length(__p); }

      /**
       * @brief Performs the identity translation.
       *
       * @param __c A character to the locale-specific character set.
       *
       * @returns __c.
       */
      char_type
      translate(char_type __c) const
      { return __c; }

      /**
       * @brief Translates a character into a case-insensitive equivalent.
       *
       * @param __c A character to the locale-specific character set.
       *
       * @returns the locale-specific lower-case equivalent of __c.
       * @throws std::bad_cast if the imbued locale does not support the ctype
       *         facet.
       */
      char_type
      translate_nocase(char_type __c) const
      {
	typedef std::ctype<char_type> __ctype_type;
	const __ctype_type& __fctyp(use_facet<__ctype_type>(_M_locale));
	return __fctyp.tolower(__c);
      }

      /**
       * @brief Gets a sort key for a character sequence.
       *
       * @param __first beginning of the character sequence.
       * @param __last  one-past-the-end of the character sequence.
       *
       * Returns a sort key for the character sequence designated by the
       * iterator range [F1, F2) such that if the character sequence [G1, G2)
       * sorts before the character sequence [H1, H2) then
       * v.transform(G1, G2) < v.transform(H1, H2).
       *
       * What this really does is provide a more efficient way to compare a
       * string to multiple other strings in locales with fancy collation
       * rules and equivalence classes.
       *
       * @returns a locale-specific sort key equivalent to the input range.
       *
       * @throws std::bad_cast if the current locale does not have a collate
       *         facet.
       */
      template<typename _Fwd_iter>
	string_type
	transform(_Fwd_iter __first, _Fwd_iter __last) const
	{
	  typedef std::collate<char_type> __collate_type;
	  const __collate_type& __fclt(use_facet<__collate_type>(_M_locale));
	  string_type __s(__first, __last);
	  return __fclt.transform(__s.data(), __s.data() + __s.size());
	}

      /**
       * @brief Gets a sort key for a character sequence, independent of case.
       *
       * @param __first beginning of the character sequence.
       * @param __last  one-past-the-end of the character sequence.
       *
       * Effects: if typeid(use_facet<collate<_Ch_type> >) ==
       * typeid(collate_byname<_Ch_type>) and the form of the sort key
       * returned by collate_byname<_Ch_type>::transform(__first, __last)
       * is known and can be converted into a primary sort key
       * then returns that key, otherwise returns an empty string.
       *
       * @todo Implement this function correctly.
       */
      template<typename _Fwd_iter>
	string_type
	transform_primary(_Fwd_iter __first, _Fwd_iter __last) const
	{
	  // TODO : this is not entirely correct.
	  // This function requires extra support from the platform.
	  //
	  // Read http://gcc.gnu.org/ml/libstdc++/2013-09/msg00117.html and
	  // http://www.open-std.org/Jtc1/sc22/wg21/docs/papers/2003/n1429.htm
	  // for details.
	  typedef std::ctype<char_type> __ctype_type;
	  const __ctype_type& __fctyp(use_facet<__ctype_type>(_M_locale));
	  std::vector<char_type> __s(__first, __last);
	  __fctyp.tolower(__s.data(), __s.data() + __s.size());
	  return this->transform(__s.data(), __s.data() + __s.size());
	}

      /**
       * @brief Gets a collation element by name.
       *
       * @param __first beginning of the collation element name.
       * @param __last  one-past-the-end of the collation element name.
       *
       * @returns a sequence of one or more characters that represents the
       * collating element consisting of the character sequence designated by
       * the iterator range [__first, __last). Returns an empty string if the
       * character sequence is not a valid collating element.
       */
      template<typename _Fwd_iter>
	string_type
	lookup_collatename(_Fwd_iter __first, _Fwd_iter __last) const;

      /**
       * @brief Maps one or more characters to a named character
       *        classification.
       *
       * @param __first beginning of the character sequence.
       * @param __last  one-past-the-end of the character sequence.
       * @param __icase ignores the case of the classification name.
       *
       * @returns an unspecified value that represents the character
       * classification named by the character sequence designated by
       * the iterator range [__first, __last). If @p icase is true,
       * the returned mask identifies the classification regardless of
       * the case of the characters to be matched (for example,
       * [[:lower:]] is the same as [[:alpha:]]), otherwise a
       * case-dependent classification is returned.  The value
       * returned shall be independent of the case of the characters
       * in the character sequence. If the name is not recognized then
       * returns a value that compares equal to 0.
       *
       * At least the following names (or their wide-character equivalent) are
       * supported.
       * - d
       * - w
       * - s
       * - alnum
       * - alpha
       * - blank
       * - cntrl
       * - digit
       * - graph
       * - lower
       * - print
       * - punct
       * - space
       * - upper
       * - xdigit
       */
      template<typename _Fwd_iter>
	char_class_type
	lookup_classname(_Fwd_iter __first, _Fwd_iter __last,
			 bool __icase = false) const;

      /**
       * @brief Determines if @p c is a member of an identified class.
       *
       * @param __c a character.
       * @param __f a class type (as returned from lookup_classname).
       *
       * @returns true if the character @p __c is a member of the classification
       * represented by @p __f, false otherwise.
       *
       * @throws std::bad_cast if the current locale does not have a ctype
       *         facet.
       */
      bool
      isctype(_Ch_type __c, char_class_type __f) const;

      /**
       * @brief Converts a digit to an int.
       *
       * @param __ch    a character representing a digit.
       * @param __radix the radix if the numeric conversion (limited to 8, 10,
       *              or 16).
       *
       * @returns the value represented by the digit __ch in base radix if the
       * character __ch is a valid digit in base radix; otherwise returns -1.
       */
      int
      value(_Ch_type __ch, int __radix) const;

      /**
       * @brief Imbues the regex_traits object with a copy of a new locale.
       *
       * @param __loc A locale.
       *
       * @returns a copy of the previous locale in use by the regex_traits
       *          object.
       *
       * @note Calling imbue with a different locale than the one currently in
       *       use invalidates all cached data held by *this.
       */
      locale_type
      imbue(locale_type __loc)
      {
	std::swap(_M_locale, __loc);
	return __loc;
      }

      /**
       * @brief Gets a copy of the current locale in use by the regex_traits
       * object.
       */
      locale_type
      getloc() const
      { return _M_locale; }

    protected:
      locale_type _M_locale;
    };

  // [7.8] Class basic_regex
  /**
   * Objects of specializations of this class represent regular expressions
   * constructed from sequences of character type @p _Ch_type.
   *
   * Storage for the regular expression is allocated and deallocated as
   * necessary by the member functions of this class.
   */
  template<typename _Ch_type, typename _Rx_traits = regex_traits<_Ch_type>>
    class basic_regex
    {
    public:
      static_assert(is_same<_Ch_type, typename _Rx_traits::char_type>::value,
		    "regex traits class must have the same char_type");

      // types:
      typedef _Ch_type				  value_type;
      typedef _Rx_traits			  traits_type;
      typedef typename traits_type::string_type   string_type;
      typedef regex_constants::syntax_option_type flag_type;
      typedef typename traits_type::locale_type   locale_type;

      /**
       * @name Constants
       * std [28.8.1](1)
       */
      //@{
      static constexpr flag_type icase = regex_constants::icase;
      static constexpr flag_type nosubs = regex_constants::nosubs;
      static constexpr flag_type optimize = regex_constants::optimize;
      static constexpr flag_type collate = regex_constants::collate;
      static constexpr flag_type ECMAScript = regex_constants::ECMAScript;
      static constexpr flag_type basic = regex_constants::basic;
      static constexpr flag_type extended = regex_constants::extended;
      static constexpr flag_type awk = regex_constants::awk;
      static constexpr flag_type grep = regex_constants::grep;
      static constexpr flag_type egrep = regex_constants::egrep;
      //@}

      // [7.8.2] construct/copy/destroy
      /**
       * Constructs a basic regular expression that does not match any
       * character sequence.
       */
      basic_regex()
      : _M_flags(ECMAScript), _M_loc(), _M_automaton(nullptr)
      { }

      /**
       * @brief Constructs a basic regular expression from the
       * sequence [__p, __p + char_traits<_Ch_type>::length(__p))
       * interpreted according to the flags in @p __f.
       *
       * @param __p A pointer to the start of a C-style null-terminated string
       *          containing a regular expression.
       * @param __f Flags indicating the syntax rules and options.
       *
       * @throws regex_error if @p __p is not a valid regular expression.
       */
      explicit
      basic_regex(const _Ch_type* __p, flag_type __f = ECMAScript)
      : basic_regex(__p, __p + char_traits<_Ch_type>::length(__p), __f)
      { }

      /**
       * @brief Constructs a basic regular expression from the sequence
       * [p, p + len) interpreted according to the flags in @p f.
       *
       * @param __p   A pointer to the start of a string containing a regular
       *              expression.
       * @param __len The length of the string containing the regular
       *              expression.
       * @param __f   Flags indicating the syntax rules and options.
       *
       * @throws regex_error if @p __p is not a valid regular expression.
       */
      basic_regex(const _Ch_type* __p, std::size_t __len,
		  flag_type __f = ECMAScript)
      : basic_regex(__p, __p + __len, __f)
      { }

      /**
       * @brief Copy-constructs a basic regular expression.
       *
       * @param __rhs A @p regex object.
       */
      basic_regex(const basic_regex& __rhs) = default;

      /**
       * @brief Move-constructs a basic regular expression.
       *
       * @param __rhs A @p regex object.
       */
      basic_regex(basic_regex&& __rhs) noexcept = default;

      /**
       * @brief Constructs a basic regular expression from the string
       * @p s interpreted according to the flags in @p f.
       *
       * @param __s A string containing a regular expression.
       * @param __f Flags indicating the syntax rules and options.
       *
       * @throws regex_error if @p __s is not a valid regular expression.
       */
      template<typename _Ch_traits, typename _Ch_alloc>
	explicit
	basic_regex(const std::basic_string<_Ch_type, _Ch_traits,
					    _Ch_alloc>& __s,
		    flag_type __f = ECMAScript)
	: basic_regex(__s.data(), __s.data() + __s.size(), __f)
	{ }

      /**
       * @brief Constructs a basic regular expression from the range
       * [first, last) interpreted according to the flags in @p f.
       *
       * @param __first The start of a range containing a valid regular
       *                expression.
       * @param __last  The end of a range containing a valid regular
       *                expression.
       * @param __f     The format flags of the regular expression.
       *
       * @throws regex_error if @p [__first, __last) is not a valid regular
       *         expression.
       */
      template<typename _FwdIter>
	basic_regex(_FwdIter __first, _FwdIter __last,
		    flag_type __f = ECMAScript)
	: basic_regex(std::move(__first), std::move(__last), locale_type(), __f)
	{ }

      /**
       * @brief Constructs a basic regular expression from an initializer list.
       *
       * @param __l  The initializer list.
       * @param __f  The format flags of the regular expression.
       *
       * @throws regex_error if @p __l is not a valid regular expression.
       */
      basic_regex(initializer_list<_Ch_type> __l, flag_type __f = ECMAScript)
      : basic_regex(__l.begin(), __l.end(), __f)
      { }

      /**
       * @brief Destroys a basic regular expression.
       */
      ~basic_regex()
      { }

      /**
       * @brief Assigns one regular expression to another.
       */
      basic_regex&
      operator=(const basic_regex& __rhs)
      { return this->assign(__rhs); }

      /**
       * @brief Move-assigns one regular expression to another.
       */
      basic_regex&
      operator=(basic_regex&& __rhs) noexcept
      { return this->assign(std::move(__rhs)); }

      /**
       * @brief Replaces a regular expression with a new one constructed from
       * a C-style null-terminated string.
       *
       * @param __p A pointer to the start of a null-terminated C-style string
       *        containing a regular expression.
       */
      basic_regex&
      operator=(const _Ch_type* __p)
      { return this->assign(__p); }

      /**
       * @brief Replaces a regular expression with a new one constructed from
       * an initializer list.
       *
       * @param __l  The initializer list.
       *
       * @throws regex_error if @p __l is not a valid regular expression.
       */
      basic_regex&
      operator=(initializer_list<_Ch_type> __l)
      { return this->assign(__l.begin(), __l.end()); }

      /**
       * @brief Replaces a regular expression with a new one constructed from
       * a string.
       *
       * @param __s A pointer to a string containing a regular expression.
       */
      template<typename _Ch_traits, typename _Alloc>
	basic_regex&
	operator=(const basic_string<_Ch_type, _Ch_traits, _Alloc>& __s)
	{ return this->assign(__s); }

      // [7.8.3] assign
      /**
       * @brief the real assignment operator.
       *
       * @param __rhs Another regular expression object.
       */
      basic_regex&
      assign(const basic_regex& __rhs)
      {
	basic_regex __tmp(__rhs);
	this->swap(__tmp);
	return *this;
      }

      /**
       * @brief The move-assignment operator.
       *
       * @param __rhs Another regular expression object.
       */
      basic_regex&
      assign(basic_regex&& __rhs) noexcept
      {
	basic_regex __tmp(std::move(__rhs));
	this->swap(__tmp);
	return *this;
      }

      /**
       * @brief Assigns a new regular expression to a regex object from a
       * C-style null-terminated string containing a regular expression
       * pattern.
       *
       * @param __p     A pointer to a C-style null-terminated string containing
       *              a regular expression pattern.
       * @param __flags Syntax option flags.
       *
       * @throws regex_error if __p does not contain a valid regular
       * expression pattern interpreted according to @p __flags.  If
       * regex_error is thrown, *this remains unchanged.
       */
      basic_regex&
      assign(const _Ch_type* __p, flag_type __flags = ECMAScript)
      { return this->assign(string_type(__p), __flags); }

      /**
       * @brief Assigns a new regular expression to a regex object from a
       * C-style string containing a regular expression pattern.
       *
       * @param __p     A pointer to a C-style string containing a
       *                regular expression pattern.
       * @param __len   The length of the regular expression pattern string.
       * @param __flags Syntax option flags.
       *
       * @throws regex_error if p does not contain a valid regular
       * expression pattern interpreted according to @p __flags.  If
       * regex_error is thrown, *this remains unchanged.
       */
      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 3296. Inconsistent default argument for basic_regex<>::assign
      basic_regex&
      assign(const _Ch_type* __p, size_t __len, flag_type __flags = ECMAScript)
      { return this->assign(string_type(__p, __len), __flags); }

      /**
       * @brief Assigns a new regular expression to a regex object from a
       * string containing a regular expression pattern.
       *
       * @param __s     A string containing a regular expression pattern.
       * @param __flags Syntax option flags.
       *
       * @throws regex_error if __s does not contain a valid regular
       * expression pattern interpreted according to @p __flags.  If
       * regex_error is thrown, *this remains unchanged.
       */
      template<typename _Ch_traits, typename _Alloc>
	basic_regex&
	assign(const basic_string<_Ch_type, _Ch_traits, _Alloc>& __s,
	       flag_type __flags = ECMAScript)
	{
	  return this->assign(basic_regex(__s.data(), __s.data() + __s.size(),
					  _M_loc, __flags));
	}

      /**
       * @brief Assigns a new regular expression to a regex object.
       *
       * @param __first The start of a range containing a valid regular
       *                expression.
       * @param __last  The end of a range containing a valid regular
       *                expression.
       * @param __flags Syntax option flags.
       *
       * @throws regex_error if p does not contain a valid regular
       * expression pattern interpreted according to @p __flags.  If
       * regex_error is thrown, the object remains unchanged.
       */
      template<typename _InputIterator>
	basic_regex&
	assign(_InputIterator __first, _InputIterator __last,
	       flag_type __flags = ECMAScript)
	{ return this->assign(string_type(__first, __last), __flags); }

      /**
       * @brief Assigns a new regular expression to a regex object.
       *
       * @param __l     An initializer list representing a regular expression.
       * @param __flags Syntax option flags.
       *
       * @throws regex_error if @p __l does not contain a valid
       * regular expression pattern interpreted according to @p
       * __flags.  If regex_error is thrown, the object remains
       * unchanged.
       */
      basic_regex&
      assign(initializer_list<_Ch_type> __l, flag_type __flags = ECMAScript)
      { return this->assign(__l.begin(), __l.end(), __flags); }

      // [7.8.4] const operations
      /**
       * @brief Gets the number of marked subexpressions within the regular
       * expression.
       */
      unsigned int
      mark_count() const
      {
	if (_M_automaton)
	  return _M_automaton->_M_sub_count() - 1;
	return 0;
      }

      /**
       * @brief Gets the flags used to construct the regular expression
       * or in the last call to assign().
       */
      flag_type
      flags() const
      { return _M_flags; }

      // [7.8.5] locale
      /**
       * @brief Imbues the regular expression object with the given locale.
       *
       * @param __loc A locale.
       */
      locale_type
      imbue(locale_type __loc)
      {
	std::swap(__loc, _M_loc);
	_M_automaton.reset();
	return __loc;
      }

      /**
       * @brief Gets the locale currently imbued in the regular expression
       *        object.
       */
      locale_type
      getloc() const
      { return _M_loc; }

      // [7.8.6] swap
      /**
       * @brief Swaps the contents of two regular expression objects.
       *
       * @param __rhs Another regular expression object.
       */
      void
      swap(basic_regex& __rhs)
      {
	std::swap(_M_flags, __rhs._M_flags);
	std::swap(_M_loc, __rhs._M_loc);
	std::swap(_M_automaton, __rhs._M_automaton);
      }

#ifdef _GLIBCXX_DEBUG
      void
      _M_dot(std::ostream& __ostr)
      { _M_automaton->_M_dot(__ostr); }
#endif

    private:
      typedef std::shared_ptr<const __detail::_NFA<_Rx_traits>> _AutomatonPtr;

      template<typename _FwdIter>
	basic_regex(_FwdIter __first, _FwdIter __last, locale_type __loc,
		    flag_type __f)
	: _M_flags(__f), _M_loc(std::move(__loc)),
	_M_automaton(__detail::__compile_nfa<_Rx_traits>(
	  std::move(__first), std::move(__last), _M_loc, _M_flags))
	{ }

      template<typename _Bp, typename _Ap, typename _Cp, typename _Rp,
	__detail::_RegexExecutorPolicy, bool>
	friend bool
	__detail::__regex_algo_impl(_Bp, _Bp, match_results<_Bp, _Ap>&,
				    const basic_regex<_Cp, _Rp>&,
				    regex_constants::match_flag_type);

      template<typename, typename, typename, bool>
	friend class __detail::_Executor;

      flag_type		_M_flags;
      locale_type	_M_loc;
      _AutomatonPtr	_M_automaton;
    };

#if __cplusplus < 201703L
  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::icase;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::nosubs;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::optimize;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::collate;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::ECMAScript;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::basic;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::extended;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::awk;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::grep;

  template<typename _Ch, typename _Tr>
    constexpr regex_constants::syntax_option_type
    basic_regex<_Ch, _Tr>::egrep;
#endif // ! C++17

#if __cpp_deduction_guides >= 201606
  template<typename _ForwardIterator>
    basic_regex(_ForwardIterator, _ForwardIterator,
		regex_constants::syntax_option_type = {})
      -> basic_regex<typename iterator_traits<_ForwardIterator>::value_type>;
#endif

  /** @brief Standard regular expressions. */
  typedef basic_regex<char>    regex;

#ifdef _GLIBCXX_USE_WCHAR_T
  /** @brief Standard wide-character regular expressions. */
  typedef basic_regex<wchar_t> wregex;
#endif


  // [7.8.6] basic_regex swap
  /**
   * @brief Swaps the contents of two regular expression objects.
   * @param __lhs First regular expression.
   * @param __rhs Second regular expression.
   * @relates basic_regex
   */
  template<typename _Ch_type, typename _Rx_traits>
    inline void
    swap(basic_regex<_Ch_type, _Rx_traits>& __lhs,
	 basic_regex<_Ch_type, _Rx_traits>& __rhs)
    { __lhs.swap(__rhs); }


  // C++11 28.9 [re.submatch] Class template sub_match
  /**
   * A sequence of characters matched by a particular marked sub-expression.
   *
   * An object of this class is essentially a pair of iterators marking a
   * matched subexpression within a regular expression pattern match. Such
   * objects can be converted to and compared with std::basic_string objects
   * of a similar base character type as the pattern matched by the regular
   * expression.
   *
   * The iterators that make up the pair are the usual half-open interval
   * referencing the actual original pattern matched.
   */
  template<typename _BiIter>
    class sub_match : public std::pair<_BiIter, _BiIter>
    {
      typedef iterator_traits<_BiIter>			__iter_traits;
	
    public:
      typedef typename __iter_traits::value_type      	value_type;
      typedef typename __iter_traits::difference_type 	difference_type;
      typedef _BiIter					iterator;
      typedef basic_string<value_type>			string_type;

      bool matched;

      constexpr sub_match() noexcept : matched() { }

      /// Gets the length of the matching sequence.
      difference_type
      length() const noexcept
      { return this->matched ? std::distance(this->first, this->second) : 0; }

      /**
       * @brief Gets the matching sequence as a string.
       *
       * @returns the matching sequence as a string.
       *
       * This is the implicit conversion operator.  It is identical to the
       * str() member function except that it will want to pop up in
       * unexpected places and cause a great deal of confusion and cursing
       * from the unwary.
       */
      operator string_type() const
      { return str(); }

      /**
       * @brief Gets the matching sequence as a string.
       *
       * @returns the matching sequence as a string.
       */
      string_type
      str() const
      {
	return this->matched
	  ? string_type(this->first, this->second)
	  : string_type();
      }

      /**
       * @brief Compares this and another matched sequence.
       *
       * @param __s Another matched sequence to compare to this one.
       *
       * @retval <0 this matched sequence will collate before @p __s.
       * @retval =0 this matched sequence is equivalent to @p __s.
       * @retval <0 this matched sequence will collate after @p __s.
       */
      int
      compare(const sub_match& __s) const
      { return this->_M_str().compare(__s._M_str()); }

      /**
       * @{
       * @brief Compares this sub_match to a string.
       *
       * @param __s A string to compare to this sub_match.
       *
       * @retval <0 this matched sequence will collate before @p __s.
       * @retval =0 this matched sequence is equivalent to @p __s.
       * @retval <0 this matched sequence will collate after @p __s.
       */
      int
      compare(const string_type& __s) const
      { return this->_M_str().compare(__s); }

      int
      compare(const value_type* __s) const
      { return this->_M_str().compare(__s); }
      // @}

      /// @cond undocumented
      // Non-standard, used by comparison operators
      int
      _M_compare(const value_type* __s, size_t __n) const
      { return this->_M_str().compare({__s, __n}); }
      /// @endcond

    private:
      // Simplified basic_string_view for C++11
      struct __string_view
      {
	using traits_type = typename string_type::traits_type;

	__string_view() = default;

	__string_view(const value_type* __s, size_t __n) noexcept
	: _M_data(__s), _M_len(__n) { }

	__string_view(const value_type* __s) noexcept
	: _M_data(__s), _M_len(traits_type::length(__s)) { }

	__string_view(const string_type& __s) noexcept
	: _M_data(__s.data()), _M_len(__s.length()) { }

	int
	compare(__string_view __s) const noexcept
	{
	  if (const size_t __n = std::min(_M_len, __s._M_len))
	    if (int __ret = traits_type::compare(_M_data, __s._M_data, __n))
	      return __ret;
	  const difference_type __diff = _M_len - __s._M_len;
	  if (__diff > std::numeric_limits<int>::max())
	    return std::numeric_limits<int>::max();
	  if (__diff < std::numeric_limits<int>::min())
	    return std::numeric_limits<int>::min();
	  return static_cast<int>(__diff);
	}

      private:
	const value_type* _M_data = nullptr;
	size_t _M_len = 0;
      };

      // Create a __string_view over the iterator range.
      template<typename _Iter = _BiIter>
	__enable_if_t<__detail::__is_contiguous_iter<_Iter>::value,
		      __string_view>
	_M_str() const noexcept
	{
	  if (this->matched)
	    if (auto __len = this->second - this->first)
	      return { std::__addressof(*this->first), __len };
	  return {};
	}

      // Create a temporary string that can be converted to __string_view.
      template<typename _Iter = _BiIter>
	__enable_if_t<!__detail::__is_contiguous_iter<_Iter>::value,
		      string_type>
	_M_str() const
	{ return str(); }
    };


  /** @brief Standard regex submatch over a C-style null-terminated string. */
  typedef sub_match<const char*>	     csub_match;

  /** @brief Standard regex submatch over a standard string. */
  typedef sub_match<string::const_iterator>  ssub_match;

#ifdef _GLIBCXX_USE_WCHAR_T
  /** @brief Regex submatch over a C-style null-terminated wide string. */
  typedef sub_match<const wchar_t*>	  wcsub_match;

  /** @brief Regex submatch over a standard wide string. */
  typedef sub_match<wstring::const_iterator> wssub_match;
#endif

  // [7.9.2] sub_match non-member operators

  /// @relates sub_match @{

  /**
   * @brief Tests the equivalence of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs  is equivalent to @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator==(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) == 0; }

#if __cpp_lib_three_way_comparison
  /**
   * @brief Three-way comparison of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns A value indicating whether `__lhs` is less than, equal to,
   *	      greater than, or incomparable with `__rhs`.
   */
  template<typename _BiIter>
    inline auto
    operator<=>(const sub_match<_BiIter>& __lhs,
		const sub_match<_BiIter>& __rhs)
    noexcept(__detail::__is_contiguous_iter<_BiIter>::value)
    {
      using _Tr = char_traits<typename iterator_traits<_BiIter>::value_type>;
      return __detail::__char_traits_cmp_cat<_Tr>(__lhs.compare(__rhs));
    }
#else
  /**
   * @brief Tests the inequivalence of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs  is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator!=(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) != 0; }

  /**
   * @brief Tests the ordering of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator<(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) < 0; }

  /**
   * @brief Tests the ordering of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator<=(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) <= 0; }

  /**
   * @brief Tests the ordering of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator>=(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) >= 0; }

  /**
   * @brief Tests the ordering of two regular expression submatches.
   * @param __lhs First regular expression submatch.
   * @param __rhs Second regular expression submatch.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _BiIter>
    inline bool
    operator>(const sub_match<_BiIter>& __lhs, const sub_match<_BiIter>& __rhs)
    { return __lhs.compare(__rhs) > 0; }
#endif // three-way comparison

  /// @cond undocumented

  // Alias for a basic_string that can be compared to a sub_match.
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    using __sub_match_string = basic_string<
			      typename iterator_traits<_Bi_iter>::value_type,
			      _Ch_traits, _Ch_alloc>;
  /// @endcond

#if ! __cpp_lib_three_way_comparison
  /**
   * @brief Tests the equivalence of a string and a regular expression
   *        submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs  is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator==(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return __rhs._M_compare(__lhs.data(), __lhs.size()) == 0; }

  /**
   * @brief Tests the inequivalence of a string and a regular expression
   *        submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs  is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator!=(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a string and a regular expression submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator<(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs._M_compare(__lhs.data(), __lhs.size()) > 0; }

  /**
   * @brief Tests the ordering of a string and a regular expression submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator>(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a string and a regular expression submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator>=(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a string and a regular expression submatch.
   * @param __lhs A string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator<=(const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__rhs < __lhs); }
#endif // three-way comparison

  /**
   * @brief Tests the equivalence of a regular expression submatch and a
   *        string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator==(const sub_match<_Bi_iter>& __lhs,
	       const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return __lhs._M_compare(__rhs.data(), __rhs.size()) == 0; }

#if __cpp_lib_three_way_comparison
  /**
   * @brief Three-way comparison of a regular expression submatch and a string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns A value indicating whether `__lhs` is less than, equal to,
   *	      greater than, or incomparable with `__rhs`.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Alloc>
    inline auto
    operator<=>(const sub_match<_Bi_iter>& __lhs,
		const __sub_match_string<_Bi_iter, _Ch_traits, _Alloc>& __rhs)
    noexcept(__detail::__is_contiguous_iter<_Bi_iter>::value)
    {
      return __detail::__char_traits_cmp_cat<_Ch_traits>(
	  __lhs._M_compare(__rhs.data(), __rhs.size()));
    }
#else
  /**
   * @brief Tests the inequivalence of a regular expression submatch and a
   *        string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator!=(const sub_match<_Bi_iter>& __lhs,
	       const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator<(const sub_match<_Bi_iter>& __lhs,
	      const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return __lhs._M_compare(__rhs.data(), __rhs.size()) < 0; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator>(const sub_match<_Bi_iter>& __lhs,
	      const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator>=(const sub_match<_Bi_iter>& __lhs,
	       const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A string.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter, typename _Ch_traits, typename _Ch_alloc>
    inline bool
    operator<=(const sub_match<_Bi_iter>& __lhs,
	       const __sub_match_string<_Bi_iter, _Ch_traits, _Ch_alloc>& __rhs)
    { return !(__rhs < __lhs); }

  /**
   * @brief Tests the equivalence of a C string and a regular expression
   *        submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs  is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator==(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return __rhs.compare(__lhs) == 0; }

  /**
   * @brief Tests the inequivalence of a C string and a regular
   *        expression submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator!=(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a C string and a regular expression submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs.compare(__lhs) > 0; }

  /**
   * @brief Tests the ordering of a C string and a regular expression submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a C string and a regular expression submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>=(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a C string and a regular expression submatch.
   * @param __lhs A null-terminated string.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<=(typename iterator_traits<_Bi_iter>::value_type const* __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__rhs < __lhs); }
#endif // three-way comparison

  /**
   * @brief Tests the equivalence of a regular expression submatch and a C
   *        string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs  is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator==(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return __lhs.compare(__rhs) == 0; }

#if __cpp_lib_three_way_comparison
  /**
   * @brief Three-way comparison of a regular expression submatch and a C
   *	    string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns A value indicating whether `__lhs` is less than, equal to,
   *	      greater than, or incomparable with `__rhs`.
   */
  template<typename _Bi_iter>
    inline auto
    operator<=>(const sub_match<_Bi_iter>& __lhs,
		typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    noexcept(__detail::__is_contiguous_iter<_Bi_iter>::value)
    {
      using _Tr = char_traits<typename iterator_traits<_Bi_iter>::value_type>;
      return __detail::__char_traits_cmp_cat<_Tr>(__lhs.compare(__rhs));
    }
#else
  /**
   * @brief Tests the inequivalence of a regular expression submatch and a
   *        string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator!=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a C string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<(const sub_match<_Bi_iter>& __lhs,
	      typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return __lhs.compare(__rhs) < 0; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a C string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>(const sub_match<_Bi_iter>& __lhs,
	      typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a C string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a C string.
   * @param __lhs A regular expression submatch.
   * @param __rhs A null-terminated string.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const* __rhs)
    { return !(__rhs < __lhs); }

  /**
   * @brief Tests the equivalence of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator==(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return __rhs._M_compare(std::__addressof(__lhs), 1) == 0; }

  /**
   * @brief Tests the inequivalence of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator!=(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs._M_compare(std::__addressof(__lhs), 1) > 0; }

  /**
   * @brief Tests the ordering of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	      const sub_match<_Bi_iter>& __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>=(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a character and a regular expression
   *        submatch.
   * @param __lhs A character.
   * @param __rhs A regular expression submatch.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<=(typename iterator_traits<_Bi_iter>::value_type const& __lhs,
	       const sub_match<_Bi_iter>& __rhs)
    { return !(__rhs < __lhs); }
#endif // three-way comparison

  /**
   * @brief Tests the equivalence of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs  is equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator==(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return __lhs._M_compare(std::__addressof(__rhs), 1) == 0; }

#if __cpp_lib_three_way_comparison
  /**
   * @brief Three-way comparison of a regular expression submatch and a
   *	    character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns A value indicating whether `__lhs` is less than, equal to,
   *	      greater than, or incomparable with `__rhs`.
   */

  template<typename _Bi_iter>
    inline auto
    operator<=>(const sub_match<_Bi_iter>& __lhs,
		typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    noexcept(__detail::__is_contiguous_iter<_Bi_iter>::value)
    {
      using _Tr = char_traits<typename iterator_traits<_Bi_iter>::value_type>;
      return __detail::__char_traits_cmp_cat<_Tr>(
	  __lhs._M_compare(std::__addressof(__rhs), 1));
    }
#else
  /**
   * @brief Tests the inequivalence of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs is not equivalent to @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator!=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return !(__lhs == __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs precedes @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<(const sub_match<_Bi_iter>& __lhs,
	      typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return __lhs._M_compare(std::__addressof(__rhs), 1) < 0; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs succeeds @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>(const sub_match<_Bi_iter>& __lhs,
	      typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return __rhs < __lhs; }

  /**
   * @brief Tests the ordering of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs does not precede @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator>=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return !(__lhs < __rhs); }

  /**
   * @brief Tests the ordering of a regular expression submatch and a
   *        character.
   * @param __lhs A regular expression submatch.
   * @param __rhs A character.
   * @returns true if @a __lhs does not succeed @a __rhs, false otherwise.
   */
  template<typename _Bi_iter>
    inline bool
    operator<=(const sub_match<_Bi_iter>& __lhs,
	       typename iterator_traits<_Bi_iter>::value_type const& __rhs)
    { return !(__rhs < __lhs); }
#endif // three-way comparison

  /**
   * @brief Inserts a matched string into an output stream.
   *
   * @param __os The output stream.
   * @param __m  A submatch string.
   *
   * @returns the output stream with the submatch string inserted.
   */
  template<typename _Ch_type, typename _Ch_traits, typename _Bi_iter>
    inline
    basic_ostream<_Ch_type, _Ch_traits>&
    operator<<(basic_ostream<_Ch_type, _Ch_traits>& __os,
	       const sub_match<_Bi_iter>& __m)
    { return __os << __m.str(); }

  // @} relates sub_match

  // [7.10] Class template match_results

  /**
   * @brief The results of a match or search operation.
   *
   * A collection of character sequences representing the result of a regular
   * expression match.  Storage for the collection is allocated and freed as
   * necessary by the member functions of class template match_results.
   *
   * This class satisfies the Sequence requirements, with the exception that
   * only the operations defined for a const-qualified Sequence are supported.
   *
   * The sub_match object stored at index 0 represents sub-expression 0, i.e.
   * the whole match. In this case the %sub_match member matched is always true.
   * The sub_match object stored at index n denotes what matched the marked
   * sub-expression n within the matched expression. If the sub-expression n
   * participated in a regular expression match then the %sub_match member
   * matched evaluates to true, and members first and second denote the range
   * of characters [first, second) which formed that match. Otherwise matched
   * is false, and members first and second point to the end of the sequence
   * that was searched.
   */
  template<typename _Bi_iter,
	   typename _Alloc = allocator<sub_match<_Bi_iter> > >
    class match_results
    : private std::vector<sub_match<_Bi_iter>, _Alloc>
    {
    private:
      /*
       * The vector base is empty if this does not represent a match (!ready());
       * Otherwise if it's a match failure, it contains 3 elements:
       * [0] unmatched
       * [1] prefix
       * [2] suffix
       * Otherwise it contains n+4 elements where n is the number of marked
       * sub-expressions:
       * [0] entire match
       * [1] 1st marked subexpression
       * ...
       * [n] nth marked subexpression
       * [n+1] unmatched
       * [n+2] prefix
       * [n+3] suffix
       */
      typedef std::vector<sub_match<_Bi_iter>, _Alloc>     _Base_type;
      typedef std::iterator_traits<_Bi_iter>   	   	   __iter_traits;
      typedef regex_constants::match_flag_type		   match_flag_type;

    public:
      /**
       * @name 28.10 Public Types
       */
      //@{
      typedef sub_match<_Bi_iter>			   value_type;
      typedef const value_type&				   const_reference;
      typedef value_type&				   reference;
      typedef typename _Base_type::const_iterator	   const_iterator;
      typedef const_iterator				   iterator;
      typedef typename __iter_traits::difference_type	   difference_type;
      typedef typename allocator_traits<_Alloc>::size_type size_type;
      typedef _Alloc					   allocator_type;
      typedef typename __iter_traits::value_type 	   char_type;
      typedef std::basic_string<char_type>		   string_type;
      //@}

    public:
      /**
       * @name 28.10.1 Construction, Copying, and Destruction
       */
      //@{

      /**
       * @brief Constructs a default %match_results container.
       * @post size() returns 0 and str() returns an empty string.
       */
      match_results() : match_results(_Alloc()) { }

      /**
       * @brief Constructs a default %match_results container.
       * @post size() returns 0 and str() returns an empty string.
       */
      explicit
      match_results(const _Alloc& __a) noexcept
      : _Base_type(__a)
      { }

      /**
       * @brief Copy constructs a %match_results.
       */
      match_results(const match_results&) = default;

      /**
       * @brief Move constructs a %match_results.
       */
      match_results(match_results&&) noexcept = default;

      /**
       * @brief Assigns rhs to *this.
       */
      match_results&
      operator=(const match_results&) = default;

      /**
       * @brief Move-assigns rhs to *this.
       */
      match_results&
      operator=(match_results&&) = default;

      /**
       * @brief Destroys a %match_results object.
       */
      ~match_results() = default;

      //@}

      // 28.10.2, state:
      /**
       * @brief Indicates if the %match_results is ready.
       * @retval true   The object has a fully-established result state.
       * @retval false  The object is not ready.
       */
      bool ready() const noexcept { return !_Base_type::empty(); }

      /**
       * @name 28.10.2 Size
       */
      //@{

      /**
       * @brief Gets the number of matches and submatches.
       *
       * The number of matches for a given regular expression will be either 0
       * if there was no match or mark_count() + 1 if a match was successful.
       * Some matches may be empty.
       *
       * @returns the number of matches found.
       */
      size_type
      size() const noexcept
      { return _Base_type::empty() ? 0 : _Base_type::size() - 3; }

      size_type
      max_size() const noexcept
      { return _Base_type::max_size() - 3; }

      /**
       * @brief Indicates if the %match_results contains no results.
       * @retval true The %match_results object is empty.
       * @retval false The %match_results object is not empty.
       */
      _GLIBCXX_NODISCARD bool
      empty() const noexcept
      { return size() == 0; }

      //@}

      /**
       * @name 28.10.4 Element Access
       */
      //@{

      /**
       * @brief Gets the length of the indicated submatch.
       * @param __sub indicates the submatch.
       * @pre   ready() == true
       *
       * This function returns the length of the indicated submatch, or the
       * length of the entire match if @p __sub is zero (the default).
       */
      difference_type
      length(size_type __sub = 0) const
      { return (*this)[__sub].length(); }

      /**
       * @brief Gets the offset of the beginning of the indicated submatch.
       * @param __sub indicates the submatch.
       * @pre   ready() == true
       *
       * This function returns the offset from the beginning of the target
       * sequence to the beginning of the submatch, unless the value of @p __sub
       * is zero (the default), in which case this function returns the offset
       * from the beginning of the target sequence to the beginning of the
       * match.
       */
      difference_type
      position(size_type __sub = 0) const
      { return std::distance(_M_begin, (*this)[__sub].first); }

      /**
       * @brief Gets the match or submatch converted to a string type.
       * @param __sub indicates the submatch.
       * @pre   ready() == true
       *
       * This function gets the submatch (or match, if @p __sub is
       * zero) extracted from the target range and converted to the
       * associated string type.
       */
      string_type
      str(size_type __sub = 0) const
      { return string_type((*this)[__sub]); }

      /**
       * @brief Gets a %sub_match reference for the match or submatch.
       * @param __sub indicates the submatch.
       * @pre   ready() == true
       *
       * This function gets a reference to the indicated submatch, or
       * the entire match if @p __sub is zero.
       *
       * If @p __sub >= size() then this function returns a %sub_match with a
       * special value indicating no submatch.
       */
      const_reference
      operator[](size_type __sub) const
      {
	__glibcxx_assert( ready() );
	return __sub < size()
	       ? _Base_type::operator[](__sub)
	       : _M_unmatched_sub();
      }

      /**
       * @brief Gets a %sub_match representing the match prefix.
       * @pre   ready() == true
       *
       * This function gets a reference to a %sub_match object representing the
       * part of the target range between the start of the target range and the
       * start of the match.
       */
      const_reference
      prefix() const
      {
	__glibcxx_assert( ready() );
	return !empty() ? _M_prefix() : _M_unmatched_sub();
      }

      /**
       * @brief Gets a %sub_match representing the match suffix.
       * @pre   ready() == true
       *
       * This function gets a reference to a %sub_match object representing the
       * part of the target range between the end of the match and the end of
       * the target range.
       */
      const_reference
      suffix() const
      {
	__glibcxx_assert( ready() );
	return !empty() ? _M_suffix() : _M_unmatched_sub();
      }

      /**
       * @brief Gets an iterator to the start of the %sub_match collection.
       */
      const_iterator
      begin() const noexcept
      { return _Base_type::begin(); }

      /**
       * @brief Gets an iterator to the start of the %sub_match collection.
       */
      const_iterator
      cbegin() const noexcept
      { return this->begin(); }

      /**
       * @brief Gets an iterator to one-past-the-end of the collection.
       */
      const_iterator
      end() const noexcept
      { return _Base_type::end() - (empty() ? 0 : 3); }

      /**
       * @brief Gets an iterator to one-past-the-end of the collection.
       */
      const_iterator
      cend() const noexcept
      { return this->end(); }

      //@}

      /**
       * @name 28.10.5 Formatting
       *
       * These functions perform formatted substitution of the matched
       * character sequences into their target.  The format specifiers and
       * escape sequences accepted by these functions are determined by
       * their @p flags parameter as documented above.
       */
       //@{

      /**
       * @pre   ready() == true
       */
      template<typename _Out_iter>
	_Out_iter
	format(_Out_iter __out, const char_type* __fmt_first,
	       const char_type* __fmt_last,
	       match_flag_type __flags = regex_constants::format_default) const;

      /**
       * @pre   ready() == true
       */
      template<typename _Out_iter, typename _St, typename _Sa>
	_Out_iter
	format(_Out_iter __out, const basic_string<char_type, _St, _Sa>& __fmt,
	       match_flag_type __flags = regex_constants::format_default) const
	{
	  return format(__out, __fmt.data(), __fmt.data() + __fmt.size(),
			__flags);
	}

      /**
       * @pre   ready() == true
       */
      template<typename _St, typename _Sa>
	basic_string<char_type, _St, _Sa>
	format(const basic_string<char_type, _St, _Sa>& __fmt,
	       match_flag_type __flags = regex_constants::format_default) const
	{
	  basic_string<char_type, _St, _Sa> __result;
	  format(std::back_inserter(__result), __fmt, __flags);
	  return __result;
	}

      /**
       * @pre   ready() == true
       */
      string_type
      format(const char_type* __fmt,
	     match_flag_type __flags = regex_constants::format_default) const
      {
	string_type __result;
	format(std::back_inserter(__result),
	       __fmt,
	       __fmt + char_traits<char_type>::length(__fmt),
	       __flags);
	return __result;
      }

      //@}

      /**
       * @name 28.10.6 Allocator
       */
      //@{

      /**
       * @brief Gets a copy of the allocator.
       */
      allocator_type
      get_allocator() const noexcept
      { return _Base_type::get_allocator(); }

      //@}

      /**
       * @name 28.10.7 Swap
       */
       //@{

      /**
       * @brief Swaps the contents of two match_results.
       */
      void
      swap(match_results& __that) noexcept
      {
	using std::swap;
	_Base_type::swap(__that);
	swap(_M_begin, __that._M_begin);
      }
      //@}

    private:
      template<typename, typename, typename>
	friend class regex_iterator;

      /// @cond undocumented

      template<typename, typename, typename, bool>
	friend class __detail::_Executor;

      template<typename _Bp, typename _Ap, typename _Cp, typename _Rp,
	__detail::_RegexExecutorPolicy, bool>
	friend bool
	__detail::__regex_algo_impl(_Bp, _Bp, match_results<_Bp, _Ap>&,
				    const basic_regex<_Cp, _Rp>&,
				    regex_constants::match_flag_type);

      // Reset contents to __size unmatched sub_match objects
      // (plus additional objects for prefix, suffix and unmatched sub).
      void
      _M_resize(unsigned int __size)
      { _Base_type::assign(__size + 3, sub_match<_Bi_iter>{}); }

      // Set state to a failed match for the given past-the-end iterator.
      void
      _M_establish_failed_match(_Bi_iter __end)
      {
	sub_match<_Bi_iter> __sm;
	__sm.first = __sm.second = __end;
	_Base_type::assign(3, __sm);
      }

      const_reference
      _M_unmatched_sub() const
      { return _Base_type::operator[](_Base_type::size() - 3); }

      sub_match<_Bi_iter>&
      _M_unmatched_sub()
      { return _Base_type::operator[](_Base_type::size() - 3); }

      const_reference
      _M_prefix() const
      { return _Base_type::operator[](_Base_type::size() - 2); }

      sub_match<_Bi_iter>&
      _M_prefix()
      { return _Base_type::operator[](_Base_type::size() - 2); }

      const_reference
      _M_suffix() const
      { return _Base_type::operator[](_Base_type::size() - 1); }

      sub_match<_Bi_iter>&
      _M_suffix()
      { return _Base_type::operator[](_Base_type::size() - 1); }

      _Bi_iter _M_begin;
      /// @endcond
    };

  typedef match_results<const char*>		 cmatch;
  typedef match_results<string::const_iterator>	 smatch;
#ifdef _GLIBCXX_USE_WCHAR_T
  typedef match_results<const wchar_t*>		 wcmatch;
  typedef match_results<wstring::const_iterator> wsmatch;
#endif

  // match_results comparisons

  /**
   * @brief Compares two match_results for equality.
   * @returns true if the two objects refer to the same match,
   *          false otherwise.
   */
  template<typename _Bi_iter, typename _Alloc>
    inline bool
    operator==(const match_results<_Bi_iter, _Alloc>& __m1,
	       const match_results<_Bi_iter, _Alloc>& __m2)
    {
      if (__m1.ready() != __m2.ready())
	return false;
      if (!__m1.ready())  // both are not ready
	return true;
      if (__m1.empty() != __m2.empty())
	return false;
      if (__m1.empty())   // both are empty
	return true;
      return __m1.prefix() == __m2.prefix()
	&& __m1.size() == __m2.size()
	&& std::equal(__m1.begin(), __m1.end(), __m2.begin())
	&& __m1.suffix() == __m2.suffix();
    }

#if ! __cpp_lib_three_way_comparison
  /**
   * @brief Compares two match_results for inequality.
   * @returns true if the two objects do not refer to the same match,
   *          false otherwise.
   */
  template<typename _Bi_iter, class _Alloc>
    inline bool
    operator!=(const match_results<_Bi_iter, _Alloc>& __m1,
	       const match_results<_Bi_iter, _Alloc>& __m2)
    { return !(__m1 == __m2); }
#endif

  // [7.10.6] match_results swap
  /**
   * @brief Swaps two match results.
   * @param __lhs A match result.
   * @param __rhs A match result.
   *
   * The contents of the two match_results objects are swapped.
   */
  template<typename _Bi_iter, typename _Alloc>
    inline void
    swap(match_results<_Bi_iter, _Alloc>& __lhs,
	 match_results<_Bi_iter, _Alloc>& __rhs) noexcept
    { __lhs.swap(__rhs); }

_GLIBCXX_END_NAMESPACE_CXX11

  // [28.11.2] Function template regex_match
  /**
   * @name Matching, Searching, and Replacing
   */
  //@{

  /**
   * @brief Determines if there is a match between the regular expression @p e
   * and all of the character sequence [first, last).
   *
   * @param __s     Start of the character sequence to match.
   * @param __e     One-past-the-end of the character sequence to match.
   * @param __m     The match results.
   * @param __re    The regular expression.
   * @param __flags Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Bi_iter, typename _Alloc,
	   typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_match(_Bi_iter				 __s,
		_Bi_iter				 __e,
		match_results<_Bi_iter, _Alloc>&	 __m,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type	 __flags
			       = regex_constants::match_default)
    {
      return __detail::__regex_algo_impl<_Bi_iter, _Alloc, _Ch_type, _Rx_traits,
	__detail::_RegexExecutorPolicy::_S_auto, true>
	  (__s, __e, __m, __re, __flags);
    }

  /**
   * @brief Indicates if there is a match between the regular expression @p e
   * and all of the character sequence [first, last).
   *
   * @param __first Beginning of the character sequence to match.
   * @param __last  One-past-the-end of the character sequence to match.
   * @param __re    The regular expression.
   * @param __flags Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Bi_iter, typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_match(_Bi_iter __first, _Bi_iter __last,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type __flags
		= regex_constants::match_default)
    {
      match_results<_Bi_iter> __what;
      return regex_match(__first, __last, __what, __re, __flags);
    }

  /**
   * @brief Determines if there is a match between the regular expression @p e
   * and a C-style null-terminated string.
   *
   * @param __s  The C-style null-terminated string to match.
   * @param __m  The match results.
   * @param __re The regular expression.
   * @param __f  Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_type, typename _Alloc, typename _Rx_traits>
    inline bool
    regex_match(const _Ch_type* __s,
		match_results<const _Ch_type*, _Alloc>& __m,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type __f
		= regex_constants::match_default)
    { return regex_match(__s, __s + _Rx_traits::length(__s), __m, __re, __f); }

  /**
   * @brief Determines if there is a match between the regular expression @p e
   * and a string.
   *
   * @param __s     The string to match.
   * @param __m     The match results.
   * @param __re    The regular expression.
   * @param __flags Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_traits, typename _Ch_alloc,
	   typename _Alloc, typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_match(const basic_string<_Ch_type, _Ch_traits, _Ch_alloc>& __s,
		match_results<typename basic_string<_Ch_type,
		_Ch_traits, _Ch_alloc>::const_iterator, _Alloc>& __m,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type __flags
		= regex_constants::match_default)
    { return regex_match(__s.begin(), __s.end(), __m, __re, __flags); }

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 2329. regex_match() with match_results should forbid temporary strings
  /// Prevent unsafe attempts to get match_results from a temporary string.
  template<typename _Ch_traits, typename _Ch_alloc,
	   typename _Alloc, typename _Ch_type, typename _Rx_traits>
    bool
    regex_match(const basic_string<_Ch_type, _Ch_traits, _Ch_alloc>&&,
		match_results<typename basic_string<_Ch_type,
		_Ch_traits, _Ch_alloc>::const_iterator, _Alloc>&,
		const basic_regex<_Ch_type, _Rx_traits>&,
		regex_constants::match_flag_type
		= regex_constants::match_default) = delete;

  /**
   * @brief Indicates if there is a match between the regular expression @p e
   * and a C-style null-terminated string.
   *
   * @param __s  The C-style null-terminated string to match.
   * @param __re The regular expression.
   * @param __f  Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_type, class _Rx_traits>
    inline bool
    regex_match(const _Ch_type* __s,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type __f
		= regex_constants::match_default)
    { return regex_match(__s, __s + _Rx_traits::length(__s), __re, __f); }

  /**
   * @brief Indicates if there is a match between the regular expression @p e
   * and a string.
   *
   * @param __s     [IN] The string to match.
   * @param __re    [IN] The regular expression.
   * @param __flags [IN] Controls how the regular expression is matched.
   *
   * @retval true  A match exists.
   * @retval false Otherwise.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_traits, typename _Str_allocator,
	   typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_match(const basic_string<_Ch_type, _Ch_traits, _Str_allocator>& __s,
		const basic_regex<_Ch_type, _Rx_traits>& __re,
		regex_constants::match_flag_type __flags
		= regex_constants::match_default)
    { return regex_match(__s.begin(), __s.end(), __re, __flags); }

  // [7.11.3] Function template regex_search
  /**
   * Searches for a regular expression within a range.
   * @param __s     [IN]  The start of the string to search.
   * @param __e     [IN]  One-past-the-end of the string to search.
   * @param __m     [OUT] The match results.
   * @param __re    [IN]  The regular expression to search for.
   * @param __flags [IN]  Search policy flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string, the content of %m is
   *               undefined.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Bi_iter, typename _Alloc,
	   typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_search(_Bi_iter __s, _Bi_iter __e,
		 match_results<_Bi_iter, _Alloc>& __m,
		 const basic_regex<_Ch_type, _Rx_traits>& __re,
		 regex_constants::match_flag_type __flags
		 = regex_constants::match_default)
    {
      return __detail::__regex_algo_impl<_Bi_iter, _Alloc, _Ch_type, _Rx_traits,
	__detail::_RegexExecutorPolicy::_S_auto, false>
	  (__s, __e, __m, __re, __flags);
    }

  /**
   * Searches for a regular expression within a range.
   * @param __first [IN]  The start of the string to search.
   * @param __last  [IN]  One-past-the-end of the string to search.
   * @param __re    [IN]  The regular expression to search for.
   * @param __flags [IN]  Search policy flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Bi_iter, typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_search(_Bi_iter __first, _Bi_iter __last,
		 const basic_regex<_Ch_type, _Rx_traits>& __re,
		 regex_constants::match_flag_type __flags
		 = regex_constants::match_default)
    {
      match_results<_Bi_iter> __what;
      return regex_search(__first, __last, __what, __re, __flags);
    }

  /**
   * @brief Searches for a regular expression within a C-string.
   * @param __s [IN]  A C-string to search for the regex.
   * @param __m [OUT] The set of regex matches.
   * @param __e [IN]  The regex to search for in @p s.
   * @param __f [IN]  The search flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string, the content of %m is
   *               undefined.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_type, class _Alloc, class _Rx_traits>
    inline bool
    regex_search(const _Ch_type* __s,
		 match_results<const _Ch_type*, _Alloc>& __m,
		 const basic_regex<_Ch_type, _Rx_traits>& __e,
		 regex_constants::match_flag_type __f
		 = regex_constants::match_default)
    { return regex_search(__s, __s + _Rx_traits::length(__s), __m, __e, __f); }

  /**
   * @brief Searches for a regular expression within a C-string.
   * @param __s [IN]  The C-string to search.
   * @param __e [IN]  The regular expression to search for.
   * @param __f [IN]  Search policy flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_search(const _Ch_type* __s,
		 const basic_regex<_Ch_type, _Rx_traits>& __e,
		 regex_constants::match_flag_type __f
		 = regex_constants::match_default)
    { return regex_search(__s, __s + _Rx_traits::length(__s), __e, __f); }

  /**
   * @brief Searches for a regular expression within a string.
   * @param __s     [IN]  The string to search.
   * @param __e     [IN]  The regular expression to search for.
   * @param __flags [IN]  Search policy flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_traits, typename _String_allocator,
	   typename _Ch_type, typename _Rx_traits>
    inline bool
    regex_search(const basic_string<_Ch_type, _Ch_traits,
		 _String_allocator>& __s,
		 const basic_regex<_Ch_type, _Rx_traits>& __e,
		 regex_constants::match_flag_type __flags
		 = regex_constants::match_default)
    { return regex_search(__s.begin(), __s.end(), __e, __flags); }

  /**
   * @brief Searches for a regular expression within a string.
   * @param __s [IN]  A C++ string to search for the regex.
   * @param __m [OUT] The set of regex matches.
   * @param __e [IN]  The regex to search for in @p s.
   * @param __f [IN]  The search flags.
   * @retval true  A match was found within the string.
   * @retval false No match was found within the string, the content of %m is
   *               undefined.
   *
   * @throws an exception of type regex_error.
   */
  template<typename _Ch_traits, typename _Ch_alloc,
	   typename _Alloc, typename _Ch_type,
	   typename _Rx_traits>
    inline bool
    regex_search(const basic_string<_Ch_type, _Ch_traits, _Ch_alloc>& __s,
		 match_results<typename basic_string<_Ch_type,
		 _Ch_traits, _Ch_alloc>::const_iterator, _Alloc>& __m,
		 const basic_regex<_Ch_type, _Rx_traits>& __e,
		 regex_constants::match_flag_type __f
		 = regex_constants::match_default)
    { return regex_search(__s.begin(), __s.end(), __m, __e, __f); }

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // 2329. regex_search() with match_results should forbid temporary strings
  /// Prevent unsafe attempts to get match_results from a temporary string.
  template<typename _Ch_traits, typename _Ch_alloc,
	   typename _Alloc, typename _Ch_type,
	   typename _Rx_traits>
    bool
    regex_search(const basic_string<_Ch_type, _Ch_traits, _Ch_alloc>&&,
		 match_results<typename basic_string<_Ch_type,
		 _Ch_traits, _Ch_alloc>::const_iterator, _Alloc>&,
		 const basic_regex<_Ch_type, _Rx_traits>&,
		 regex_constants::match_flag_type
		 = regex_constants::match_default) = delete;

  // std [28.11.4] Function template regex_replace
  /**
   * @brief Search for a regular expression within a range for multiple times,
   and replace the matched parts through filling a format string.
   * @param __out   [OUT] The output iterator.
   * @param __first [IN]  The start of the string to search.
   * @param __last  [IN]  One-past-the-end of the string to search.
   * @param __e     [IN]  The regular expression to search for.
   * @param __fmt   [IN]  The format string.
   * @param __flags [IN]  Search and replace policy flags.
   *
   * @returns __out
   * @throws an exception of type regex_error.
   */
  template<typename _Out_iter, typename _Bi_iter,
	   typename _Rx_traits, typename _Ch_type,
	   typename _St, typename _Sa>
    inline _Out_iter
    regex_replace(_Out_iter __out, _Bi_iter __first, _Bi_iter __last,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const basic_string<_Ch_type, _St, _Sa>& __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default)
    {
      return regex_replace(__out, __first, __last, __e, __fmt.c_str(), __flags);
    }

  /**
   * @brief Search for a regular expression within a range for multiple times,
   and replace the matched parts through filling a format C-string.
   * @param __out   [OUT] The output iterator.
   * @param __first [IN]  The start of the string to search.
   * @param __last  [IN]  One-past-the-end of the string to search.
   * @param __e     [IN]  The regular expression to search for.
   * @param __fmt   [IN]  The format C-string.
   * @param __flags [IN]  Search and replace policy flags.
   *
   * @returns __out
   * @throws an exception of type regex_error.
   */
  template<typename _Out_iter, typename _Bi_iter,
	   typename _Rx_traits, typename _Ch_type>
    _Out_iter
    regex_replace(_Out_iter __out, _Bi_iter __first, _Bi_iter __last,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const _Ch_type* __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default);

  /**
   * @brief Search for a regular expression within a string for multiple times,
   and replace the matched parts through filling a format string.
   * @param __s     [IN] The string to search and replace.
   * @param __e     [IN] The regular expression to search for.
   * @param __fmt   [IN] The format string.
   * @param __flags [IN] Search and replace policy flags.
   *
   * @returns The string after replacing.
   * @throws an exception of type regex_error.
   */
  template<typename _Rx_traits, typename _Ch_type,
	   typename _St, typename _Sa, typename _Fst, typename _Fsa>
    inline basic_string<_Ch_type, _St, _Sa>
    regex_replace(const basic_string<_Ch_type, _St, _Sa>& __s,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const basic_string<_Ch_type, _Fst, _Fsa>& __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default)
    {
      basic_string<_Ch_type, _St, _Sa> __result;
      regex_replace(std::back_inserter(__result),
		    __s.begin(), __s.end(), __e, __fmt, __flags);
      return __result;
    }

  /**
   * @brief Search for a regular expression within a string for multiple times,
   and replace the matched parts through filling a format C-string.
   * @param __s     [IN] The string to search and replace.
   * @param __e     [IN] The regular expression to search for.
   * @param __fmt   [IN] The format C-string.
   * @param __flags [IN] Search and replace policy flags.
   *
   * @returns The string after replacing.
   * @throws an exception of type regex_error.
   */
  template<typename _Rx_traits, typename _Ch_type,
	   typename _St, typename _Sa>
    inline basic_string<_Ch_type, _St, _Sa>
    regex_replace(const basic_string<_Ch_type, _St, _Sa>& __s,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const _Ch_type* __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default)
    {
      basic_string<_Ch_type, _St, _Sa> __result;
      regex_replace(std::back_inserter(__result),
		    __s.begin(), __s.end(), __e, __fmt, __flags);
      return __result;
    }

  /**
   * @brief Search for a regular expression within a C-string for multiple
   times, and replace the matched parts through filling a format string.
   * @param __s     [IN] The C-string to search and replace.
   * @param __e     [IN] The regular expression to search for.
   * @param __fmt   [IN] The format string.
   * @param __flags [IN] Search and replace policy flags.
   *
   * @returns The string after replacing.
   * @throws an exception of type regex_error.
   */
  template<typename _Rx_traits, typename _Ch_type,
	   typename _St, typename _Sa>
    inline basic_string<_Ch_type>
    regex_replace(const _Ch_type* __s,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const basic_string<_Ch_type, _St, _Sa>& __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default)
    {
      basic_string<_Ch_type> __result;
      regex_replace(std::back_inserter(__result), __s,
		    __s + char_traits<_Ch_type>::length(__s),
		    __e, __fmt, __flags);
      return __result;
    }

  /**
   * @brief Search for a regular expression within a C-string for multiple
   times, and replace the matched parts through filling a format C-string.
   * @param __s     [IN] The C-string to search and replace.
   * @param __e     [IN] The regular expression to search for.
   * @param __fmt   [IN] The format C-string.
   * @param __flags [IN] Search and replace policy flags.
   *
   * @returns The string after replacing.
   * @throws an exception of type regex_error.
   */
  template<typename _Rx_traits, typename _Ch_type>
    inline basic_string<_Ch_type>
    regex_replace(const _Ch_type* __s,
		  const basic_regex<_Ch_type, _Rx_traits>& __e,
		  const _Ch_type* __fmt,
		  regex_constants::match_flag_type __flags
		  = regex_constants::match_default)
    {
      basic_string<_Ch_type> __result;
      regex_replace(std::back_inserter(__result), __s,
		    __s + char_traits<_Ch_type>::length(__s),
		    __e, __fmt, __flags);
      return __result;
    }

  //@}

_GLIBCXX_BEGIN_NAMESPACE_CXX11

  // std [28.12] Class template regex_iterator
  /**
   * An iterator adaptor that will provide repeated calls of regex_search over
   * a range until no more matches remain.
   */
  template<typename _Bi_iter,
	   typename _Ch_type = typename iterator_traits<_Bi_iter>::value_type,
	   typename _Rx_traits = regex_traits<_Ch_type> >
    class regex_iterator
    {
    public:
      typedef basic_regex<_Ch_type, _Rx_traits>  regex_type;
      typedef match_results<_Bi_iter>	    value_type;
      typedef std::ptrdiff_t		     difference_type;
      typedef const value_type*		  pointer;
      typedef const value_type&		  reference;
      typedef std::forward_iterator_tag	  iterator_category;

      /**
       * @brief Provides a singular iterator, useful for indicating
       * one-past-the-end of a range.
       */
      regex_iterator() = default;

      /**
       * Constructs a %regex_iterator...
       * @param __a  [IN] The start of a text range to search.
       * @param __b  [IN] One-past-the-end of the text range to search.
       * @param __re [IN] The regular expression to match.
       * @param __m  [IN] Policy flags for match rules.
       */
      regex_iterator(_Bi_iter __a, _Bi_iter __b, const regex_type& __re,
		     regex_constants::match_flag_type __m
		     = regex_constants::match_default)
      : _M_begin(__a), _M_end(__b), _M_pregex(&__re), _M_flags(__m), _M_match()
      {
	if (!regex_search(_M_begin, _M_end, _M_match, *_M_pregex, _M_flags))
	  *this = regex_iterator();
      }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2332. regex_iterator should forbid temporary regexes
      regex_iterator(_Bi_iter, _Bi_iter, const regex_type&&,
		     regex_constants::match_flag_type
		     = regex_constants::match_default) = delete;

      /// Copy constructs a %regex_iterator.
      regex_iterator(const regex_iterator&) = default;

      /// Copy assigns one %regex_iterator to another.
      regex_iterator&
      operator=(const regex_iterator&) = default;

      ~regex_iterator() = default;

      /**
       * @brief Tests the equivalence of two regex iterators.
       */
      bool
      operator==(const regex_iterator&) const noexcept;

      /**
       * @brief Tests the inequivalence of two regex iterators.
       */
      bool
      operator!=(const regex_iterator& __rhs) const noexcept
      { return !(*this == __rhs); }

      /**
       * @brief Dereferences a %regex_iterator.
       */
      const value_type&
      operator*() const noexcept
      { return _M_match; }

      /**
       * @brief Selects a %regex_iterator member.
       */
      const value_type*
      operator->() const noexcept
      { return &_M_match; }

      /**
       * @brief Increments a %regex_iterator.
       */
      regex_iterator&
      operator++();

      /**
       * @brief Postincrements a %regex_iterator.
       */
      regex_iterator
      operator++(int)
      {
	auto __tmp = *this;
	++(*this);
	return __tmp;
      }

    private:
      _Bi_iter				_M_begin {};
      _Bi_iter				_M_end {};
      const regex_type*			_M_pregex = nullptr;
      regex_constants::match_flag_type	_M_flags {};
      match_results<_Bi_iter>		_M_match;
    };

  typedef regex_iterator<const char*>			cregex_iterator;
  typedef regex_iterator<string::const_iterator>	sregex_iterator;
#ifdef _GLIBCXX_USE_WCHAR_T
  typedef regex_iterator<const wchar_t*>		wcregex_iterator;
  typedef regex_iterator<wstring::const_iterator>	wsregex_iterator;
#endif

  // [7.12.2] Class template regex_token_iterator
  /**
   * Iterates over submatches in a range (or @a splits a text string).
   *
   * The purpose of this iterator is to enumerate all, or all specified,
   * matches of a regular expression within a text range.  The dereferenced
   * value of an iterator of this class is a std::sub_match object.
   */
  template<typename _Bi_iter,
	   typename _Ch_type = typename iterator_traits<_Bi_iter>::value_type,
	   typename _Rx_traits = regex_traits<_Ch_type> >
    class regex_token_iterator
    {
    public:
      typedef basic_regex<_Ch_type, _Rx_traits>	regex_type;
      typedef sub_match<_Bi_iter>		value_type;
      typedef std::ptrdiff_t			difference_type;
      typedef const value_type*			pointer;
      typedef const value_type&			reference;
      typedef std::forward_iterator_tag		iterator_category;

    public:
      /**
       * @brief Default constructs a %regex_token_iterator.
       *
       * A default-constructed %regex_token_iterator is a singular iterator
       * that will compare equal to the one-past-the-end value for any
       * iterator of the same type.
       */
      regex_token_iterator()
      : _M_position(), _M_subs(), _M_suffix(), _M_n(0), _M_result(nullptr),
      _M_has_m1(false)
      { }

      /**
       * Constructs a %regex_token_iterator...
       * @param __a          [IN] The start of the text to search.
       * @param __b          [IN] One-past-the-end of the text to search.
       * @param __re         [IN] The regular expression to search for.
       * @param __submatch   [IN] Which submatch to return.  There are some
       *                        special values for this parameter:
       *                        - -1 each enumerated subexpression does NOT
       *                          match the regular expression (aka field
       *                          splitting)
       *                        - 0 the entire string matching the
       *                          subexpression is returned for each match
       *                          within the text.
       *                        - >0 enumerates only the indicated
       *                          subexpression from a match within the text.
       * @param __m          [IN] Policy flags for match rules.
       */
      regex_token_iterator(_Bi_iter __a, _Bi_iter __b, const regex_type& __re,
			   int __submatch = 0,
			   regex_constants::match_flag_type __m
			   = regex_constants::match_default)
      : _M_position(__a, __b, __re, __m), _M_subs(1, __submatch), _M_n(0)
      { _M_init(__a, __b); }

      /**
       * Constructs a %regex_token_iterator...
       * @param __a          [IN] The start of the text to search.
       * @param __b          [IN] One-past-the-end of the text to search.
       * @param __re         [IN] The regular expression to search for.
       * @param __submatches [IN] A list of subexpressions to return for each
       *                          regular expression match within the text.
       * @param __m          [IN] Policy flags for match rules.
       */
      regex_token_iterator(_Bi_iter __a, _Bi_iter __b,
			   const regex_type& __re,
			   const std::vector<int>& __submatches,
			   regex_constants::match_flag_type __m
			     = regex_constants::match_default)
      : _M_position(__a, __b, __re, __m), _M_subs(__submatches), _M_n(0)
      { _M_init(__a, __b); }

      /**
       * Constructs a %regex_token_iterator...
       * @param __a          [IN] The start of the text to search.
       * @param __b          [IN] One-past-the-end of the text to search.
       * @param __re         [IN] The regular expression to search for.
       * @param __submatches [IN] A list of subexpressions to return for each
       *                          regular expression match within the text.
       * @param __m          [IN] Policy flags for match rules.
       */
      regex_token_iterator(_Bi_iter __a, _Bi_iter __b,
			   const regex_type& __re,
			   initializer_list<int> __submatches,
			   regex_constants::match_flag_type __m
			     = regex_constants::match_default)
      : _M_position(__a, __b, __re, __m), _M_subs(__submatches), _M_n(0)
      { _M_init(__a, __b); }

      /**
       * Constructs a %regex_token_iterator...
       * @param __a          [IN] The start of the text to search.
       * @param __b          [IN] One-past-the-end of the text to search.
       * @param __re         [IN] The regular expression to search for.
       * @param __submatches [IN] A list of subexpressions to return for each
       *                          regular expression match within the text.
       * @param __m          [IN] Policy flags for match rules.
       */
      template<std::size_t _Nm>
	regex_token_iterator(_Bi_iter __a, _Bi_iter __b,
			     const regex_type& __re,
			     const int (&__submatches)[_Nm],
			     regex_constants::match_flag_type __m
			     = regex_constants::match_default)
      : _M_position(__a, __b, __re, __m),
      _M_subs(__submatches, __submatches + _Nm), _M_n(0)
      { _M_init(__a, __b); }

      // _GLIBCXX_RESOLVE_LIB_DEFECTS
      // 2332. regex_token_iterator should forbid temporary regexes
      regex_token_iterator(_Bi_iter, _Bi_iter, const regex_type&&, int = 0,
			   regex_constants::match_flag_type =
			   regex_constants::match_default) = delete;
      regex_token_iterator(_Bi_iter, _Bi_iter, const regex_type&&,
			   const std::vector<int>&,
			   regex_constants::match_flag_type =
			   regex_constants::match_default) = delete;
      regex_token_iterator(_Bi_iter, _Bi_iter, const regex_type&&,
			   initializer_list<int>,
			   regex_constants::match_flag_type =
			   regex_constants::match_default) = delete;
      template <std::size_t _Nm>
	regex_token_iterator(_Bi_iter, _Bi_iter, const regex_type&&,
			     const int (&)[_Nm],
			     regex_constants::match_flag_type =
			     regex_constants::match_default) = delete;

      /**
       * @brief Copy constructs a %regex_token_iterator.
       * @param __rhs [IN] A %regex_token_iterator to copy.
       */
      regex_token_iterator(const regex_token_iterator& __rhs)
      : _M_position(__rhs._M_position), _M_subs(__rhs._M_subs),
      _M_suffix(__rhs._M_suffix), _M_n(__rhs._M_n), _M_has_m1(__rhs._M_has_m1)
      { _M_normalize_result(); }

      /**
       * @brief Assigns a %regex_token_iterator to another.
       * @param __rhs [IN] A %regex_token_iterator to copy.
       */
      regex_token_iterator&
      operator=(const regex_token_iterator& __rhs);

      /**
       * @brief Compares a %regex_token_iterator to another for equality.
       */
      bool
      operator==(const regex_token_iterator& __rhs) const;

      /**
       * @brief Compares a %regex_token_iterator to another for inequality.
       */
      bool
      operator!=(const regex_token_iterator& __rhs) const
      { return !(*this == __rhs); }

      /**
       * @brief Dereferences a %regex_token_iterator.
       */
      const value_type&
      operator*() const
      { return *_M_result; }

      /**
       * @brief Selects a %regex_token_iterator member.
       */
      const value_type*
      operator->() const
      { return _M_result; }

      /**
       * @brief Increments a %regex_token_iterator.
       */
      regex_token_iterator&
      operator++();

      /**
       * @brief Postincrements a %regex_token_iterator.
       */
      regex_token_iterator
      operator++(int)
      {
	auto __tmp = *this;
	++(*this);
	return __tmp;
      }

    private:
      typedef regex_iterator<_Bi_iter, _Ch_type, _Rx_traits> _Position;

      void
      _M_init(_Bi_iter __a, _Bi_iter __b);

      const value_type&
      _M_current_match() const
      {
	if (_M_subs[_M_n] == -1)
	  return (*_M_position).prefix();
	else
	  return (*_M_position)[_M_subs[_M_n]];
      }

      constexpr bool
      _M_end_of_seq() const
      { return _M_result == nullptr; }

      // [28.12.2.2.4]
      void
      _M_normalize_result()
      {
	if (_M_position != _Position())
	  _M_result = &_M_current_match();
	else if (_M_has_m1)
	  _M_result = &_M_suffix;
	else
	  _M_result = nullptr;
      }

      _Position		_M_position;
      std::vector<int>	_M_subs;
      value_type	_M_suffix;
      std::size_t	_M_n;
      const value_type*	_M_result;

      // Show whether _M_subs contains -1
      bool		_M_has_m1;
    };

  /** @brief Token iterator for C-style NULL-terminated strings. */
  typedef regex_token_iterator<const char*>		cregex_token_iterator;

  /** @brief Token iterator for standard strings. */
  typedef regex_token_iterator<string::const_iterator>	sregex_token_iterator;

#ifdef _GLIBCXX_USE_WCHAR_T
  /** @brief Token iterator for C-style NULL-terminated wide strings. */
  typedef regex_token_iterator<const wchar_t*>		wcregex_token_iterator;

  /** @brief Token iterator for standard wide-character strings. */
  typedef regex_token_iterator<wstring::const_iterator> wsregex_token_iterator;
#endif

  //@} // group regex

_GLIBCXX_END_NAMESPACE_CXX11
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#include <bits/regex.tcc>
