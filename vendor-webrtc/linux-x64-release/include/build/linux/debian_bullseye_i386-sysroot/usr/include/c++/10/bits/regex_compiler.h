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
 *  @file bits/regex_compiler.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
_GLIBCXX_BEGIN_NAMESPACE_CXX11

  template<typename>
    class regex_traits;

_GLIBCXX_END_NAMESPACE_CXX11

namespace __detail
{
  /**
   * @addtogroup regex-detail
   * @{
   */

  template<typename, bool, bool>
    struct _BracketMatcher;

  /**
   * @brief Builds an NFA from an input iterator range.
   *
   * The %_TraitsT type should fulfill requirements [28.3].
   */
  template<typename _TraitsT>
    class _Compiler
    {
    public:
      typedef typename _TraitsT::char_type        _CharT;
      typedef const _CharT*                       _IterT;
      typedef _NFA<_TraitsT>              	  _RegexT;
      typedef regex_constants::syntax_option_type _FlagT;

      _Compiler(_IterT __b, _IterT __e,
		const typename _TraitsT::locale_type& __traits, _FlagT __flags);

      shared_ptr<const _RegexT>
      _M_get_nfa()
      { return std::move(_M_nfa); }

    private:
      typedef _Scanner<_CharT>               _ScannerT;
      typedef typename _TraitsT::string_type _StringT;
      typedef typename _ScannerT::_TokenT    _TokenT;
      typedef _StateSeq<_TraitsT>            _StateSeqT;
      typedef std::stack<_StateSeqT>         _StackT;
      typedef std::ctype<_CharT>             _CtypeT;

      // accepts a specific token or returns false.
      bool
      _M_match_token(_TokenT __token);

      void
      _M_disjunction();

      void
      _M_alternative();

      bool
      _M_term();

      bool
      _M_assertion();

      bool
      _M_quantifier();

      bool
      _M_atom();

      bool
      _M_bracket_expression();

      template<bool __icase, bool __collate>
	void
	_M_insert_any_matcher_ecma();

      template<bool __icase, bool __collate>
	void
	_M_insert_any_matcher_posix();

      template<bool __icase, bool __collate>
	void
	_M_insert_char_matcher();

      template<bool __icase, bool __collate>
	void
	_M_insert_character_class_matcher();

      template<bool __icase, bool __collate>
	void
	_M_insert_bracket_matcher(bool __neg);

      // Returns true if successfully matched one term and should continue.
      // Returns false if the compiler should move on.
      template<bool __icase, bool __collate>
	bool
	_M_expression_term(pair<bool, _CharT>& __last_char,
			   _BracketMatcher<_TraitsT, __icase, __collate>&
			   __matcher);

      int
      _M_cur_int_value(int __radix);

      bool
      _M_try_char();

      _StateSeqT
      _M_pop()
      {
	auto ret = _M_stack.top();
	_M_stack.pop();
	return ret;
      }

      _FlagT              _M_flags;
      _ScannerT           _M_scanner;
      shared_ptr<_RegexT> _M_nfa;
      _StringT            _M_value;
      _StackT             _M_stack;
      const _TraitsT&     _M_traits;
      const _CtypeT&      _M_ctype;
    };

  template<typename _Tp>
    struct __is_contiguous_iter : is_pointer<_Tp>::type { };

  template<typename _Tp, typename _Cont>
    struct
    __is_contiguous_iter<__gnu_cxx::__normal_iterator<_Tp*, _Cont>>
    : true_type { };

  template<typename _Iter, typename _TraitsT>
    using __enable_if_contiguous_iter
      = __enable_if_t< __is_contiguous_iter<_Iter>::value,
                       std::shared_ptr<const _NFA<_TraitsT>> >;

  template<typename _Iter, typename _TraitsT>
    using __disable_if_contiguous_iter
      = __enable_if_t< !__is_contiguous_iter<_Iter>::value,
                       std::shared_ptr<const _NFA<_TraitsT>> >;

  template<typename _TraitsT, typename _FwdIter>
    inline __enable_if_contiguous_iter<_FwdIter, _TraitsT>
    __compile_nfa(_FwdIter __first, _FwdIter __last,
		  const typename _TraitsT::locale_type& __loc,
		  regex_constants::syntax_option_type __flags)
    {
      size_t __len = __last - __first;
      const auto* __cfirst = __len ? std::__addressof(*__first) : nullptr;
      using _Cmplr = _Compiler<_TraitsT>;
      return _Cmplr(__cfirst, __cfirst + __len, __loc, __flags)._M_get_nfa();
    }

  template<typename _TraitsT, typename _FwdIter>
    inline __disable_if_contiguous_iter<_FwdIter, _TraitsT>
    __compile_nfa(_FwdIter __first, _FwdIter __last,
		  const typename _TraitsT::locale_type& __loc,
		  regex_constants::syntax_option_type __flags)
    {
      const basic_string<typename _TraitsT::char_type> __str(__first, __last);
      return __compile_nfa<_TraitsT>(__str.data(), __str.data() + __str.size(),
				     __loc, __flags);
    }

  // [28.13.14]
  template<typename _TraitsT, bool __icase, bool __collate>
    class _RegexTranslatorBase
    {
    public:
      typedef typename _TraitsT::char_type	      _CharT;
      typedef typename _TraitsT::string_type	      _StringT;
      typedef _StringT _StrTransT;

      explicit
      _RegexTranslatorBase(const _TraitsT& __traits)
      : _M_traits(__traits)
      { }

      _CharT
      _M_translate(_CharT __ch) const
      {
	if (__icase)
	  return _M_traits.translate_nocase(__ch);
	else if (__collate)
	  return _M_traits.translate(__ch);
	else
	  return __ch;
      }

      _StrTransT
      _M_transform(_CharT __ch) const
      {
	_StrTransT __str(1, __ch);
	return _M_traits.transform(__str.begin(), __str.end());
      }

      // See LWG 523. It's not efficiently implementable when _TraitsT is not
      // std::regex_traits<>, and __collate is true. See specializations for
      // implementations of other cases.
      bool
      _M_match_range(const _StrTransT& __first, const _StrTransT& __last,
		     const _StrTransT& __s) const
      { return __first <= __s && __s <= __last; }

    protected:
      bool _M_in_range_icase(_CharT __first, _CharT __last, _CharT __ch) const
      {
	typedef std::ctype<_CharT> __ctype_type;
	const auto& __fctyp = use_facet<__ctype_type>(this->_M_traits.getloc());
	auto __lower = __fctyp.tolower(__ch);
	auto __upper = __fctyp.toupper(__ch);
	return (__first <= __lower && __lower <= __last)
	  || (__first <= __upper && __upper <= __last);
      }

      const _TraitsT& _M_traits;
    };

  template<typename _TraitsT, bool __icase, bool __collate>
    class _RegexTranslator
    : public _RegexTranslatorBase<_TraitsT, __icase, __collate>
    {
    public:
      typedef _RegexTranslatorBase<_TraitsT, __icase, __collate> _Base;
      using _Base::_Base;
    };

  template<typename _TraitsT, bool __icase>
    class _RegexTranslator<_TraitsT, __icase, false>
    : public _RegexTranslatorBase<_TraitsT, __icase, false>
    {
    public:
      typedef _RegexTranslatorBase<_TraitsT, __icase, false> _Base;
      typedef typename _Base::_CharT _CharT;
      typedef _CharT _StrTransT;

      using _Base::_Base;

      _StrTransT
      _M_transform(_CharT __ch) const
      { return __ch; }

      bool
      _M_match_range(_CharT __first, _CharT __last, _CharT __ch) const
      {
	if (!__icase)
	  return __first <= __ch && __ch <= __last;
	return this->_M_in_range_icase(__first, __last, __ch);
      }
    };

  template<typename _CharType>
    class _RegexTranslator<std::regex_traits<_CharType>, true, true>
    : public _RegexTranslatorBase<std::regex_traits<_CharType>, true, true>
    {
    public:
      typedef _RegexTranslatorBase<std::regex_traits<_CharType>, true, true>
	_Base;
      typedef typename _Base::_CharT _CharT;
      typedef typename _Base::_StrTransT _StrTransT;

      using _Base::_Base;

      bool
      _M_match_range(const _StrTransT& __first, const _StrTransT& __last,
		     const _StrTransT& __str) const
      {
	__glibcxx_assert(__first.size() == 1);
	__glibcxx_assert(__last.size() == 1);
	__glibcxx_assert(__str.size() == 1);
	return this->_M_in_range_icase(__first[0], __last[0], __str[0]);
      }
    };

  template<typename _TraitsT>
    class _RegexTranslator<_TraitsT, false, false>
    {
    public:
      typedef typename _TraitsT::char_type _CharT;
      typedef _CharT                       _StrTransT;

      explicit
      _RegexTranslator(const _TraitsT&)
      { }

      _CharT
      _M_translate(_CharT __ch) const
      { return __ch; }

      _StrTransT
      _M_transform(_CharT __ch) const
      { return __ch; }

      bool
      _M_match_range(_CharT __first, _CharT __last, _CharT __ch) const
      { return __first <= __ch && __ch <= __last; }
    };

  template<typename _TraitsT, bool __is_ecma, bool __icase, bool __collate>
    struct _AnyMatcher;

  template<typename _TraitsT, bool __icase, bool __collate>
    struct _AnyMatcher<_TraitsT, false, __icase, __collate>
    {
      typedef _RegexTranslator<_TraitsT, __icase, __collate> _TransT;
      typedef typename _TransT::_CharT                       _CharT;

      explicit
      _AnyMatcher(const _TraitsT& __traits)
      : _M_translator(__traits)
      { }

      bool
      operator()(_CharT __ch) const
      {
	static auto __nul = _M_translator._M_translate('\0');
	return _M_translator._M_translate(__ch) != __nul;
      }

      _TransT _M_translator;
    };

  template<typename _TraitsT, bool __icase, bool __collate>
    struct _AnyMatcher<_TraitsT, true, __icase, __collate>
    {
      typedef _RegexTranslator<_TraitsT, __icase, __collate> _TransT;
      typedef typename _TransT::_CharT                       _CharT;

      explicit
      _AnyMatcher(const _TraitsT& __traits)
      : _M_translator(__traits)
      { }

      bool
      operator()(_CharT __ch) const
      { return _M_apply(__ch, typename is_same<_CharT, char>::type()); }

      bool
      _M_apply(_CharT __ch, true_type) const
      {
	auto __c = _M_translator._M_translate(__ch);
	auto __n = _M_translator._M_translate('\n');
	auto __r = _M_translator._M_translate('\r');
	return __c != __n && __c != __r;
      }

      bool
      _M_apply(_CharT __ch, false_type) const
      {
	auto __c = _M_translator._M_translate(__ch);
	auto __n = _M_translator._M_translate('\n');
	auto __r = _M_translator._M_translate('\r');
	auto __u2028 = _M_translator._M_translate(u'\u2028');
	auto __u2029 = _M_translator._M_translate(u'\u2029');
	return __c != __n && __c != __r && __c != __u2028 && __c != __u2029;
      }

      _TransT _M_translator;
    };

  template<typename _TraitsT, bool __icase, bool __collate>
    struct _CharMatcher
    {
      typedef _RegexTranslator<_TraitsT, __icase, __collate> _TransT;
      typedef typename _TransT::_CharT                       _CharT;

      _CharMatcher(_CharT __ch, const _TraitsT& __traits)
      : _M_translator(__traits), _M_ch(_M_translator._M_translate(__ch))
      { }

      bool
      operator()(_CharT __ch) const
      { return _M_ch == _M_translator._M_translate(__ch); }

      _TransT _M_translator;
      _CharT  _M_ch;
    };

  /// Matches a character range (bracket expression)
  template<typename _TraitsT, bool __icase, bool __collate>
    struct _BracketMatcher
    {
    public:
      typedef _RegexTranslator<_TraitsT, __icase, __collate> _TransT;
      typedef typename _TransT::_CharT                       _CharT;
      typedef typename _TransT::_StrTransT                   _StrTransT;
      typedef typename _TraitsT::string_type                 _StringT;
      typedef typename _TraitsT::char_class_type             _CharClassT;

    public:
      _BracketMatcher(bool __is_non_matching,
		      const _TraitsT& __traits)
      : _M_class_set(0), _M_translator(__traits), _M_traits(__traits),
      _M_is_non_matching(__is_non_matching)
      { }

      bool
      operator()(_CharT __ch) const
      {
	_GLIBCXX_DEBUG_ASSERT(_M_is_ready);
	return _M_apply(__ch, _UseCache());
      }

      void
      _M_add_char(_CharT __c)
      {
	_M_char_set.push_back(_M_translator._M_translate(__c));
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = false);
      }

      _StringT
      _M_add_collate_element(const _StringT& __s)
      {
	auto __st = _M_traits.lookup_collatename(__s.data(),
						 __s.data() + __s.size());
	if (__st.empty())
	  __throw_regex_error(regex_constants::error_collate,
			      "Invalid collate element.");
	_M_char_set.push_back(_M_translator._M_translate(__st[0]));
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = false);
	return __st;
      }

      void
      _M_add_equivalence_class(const _StringT& __s)
      {
	auto __st = _M_traits.lookup_collatename(__s.data(),
						 __s.data() + __s.size());
	if (__st.empty())
	  __throw_regex_error(regex_constants::error_collate,
			      "Invalid equivalence class.");
	__st = _M_traits.transform_primary(__st.data(),
					   __st.data() + __st.size());
	_M_equiv_set.push_back(__st);
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = false);
      }

      // __neg should be true for \D, \S and \W only.
      void
      _M_add_character_class(const _StringT& __s, bool __neg)
      {
	auto __mask = _M_traits.lookup_classname(__s.data(),
						 __s.data() + __s.size(),
						 __icase);
	if (__mask == 0)
	  __throw_regex_error(regex_constants::error_collate,
			      "Invalid character class.");
	if (!__neg)
	  _M_class_set |= __mask;
	else
	  _M_neg_class_set.push_back(__mask);
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = false);
      }

      void
      _M_make_range(_CharT __l, _CharT __r)
      {
	if (__l > __r)
	  __throw_regex_error(regex_constants::error_range,
			      "Invalid range in bracket expression.");
	_M_range_set.push_back(make_pair(_M_translator._M_transform(__l),
					 _M_translator._M_transform(__r)));
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = false);
      }

      void
      _M_ready()
      {
	std::sort(_M_char_set.begin(), _M_char_set.end());
	auto __end = std::unique(_M_char_set.begin(), _M_char_set.end());
	_M_char_set.erase(__end, _M_char_set.end());
	_M_make_cache(_UseCache());
	_GLIBCXX_DEBUG_ONLY(_M_is_ready = true);
      }

    private:
      // Currently we only use the cache for char
      typedef typename std::is_same<_CharT, char>::type _UseCache;

      static constexpr size_t
      _S_cache_size =
	1ul << (sizeof(_CharT) * __CHAR_BIT__ * int(_UseCache::value));

      struct _Dummy { };
      typedef typename std::conditional<_UseCache::value,
					std::bitset<_S_cache_size>,
					_Dummy>::type _CacheT;
      typedef typename std::make_unsigned<_CharT>::type _UnsignedCharT;

      bool
      _M_apply(_CharT __ch, false_type) const;

      bool
      _M_apply(_CharT __ch, true_type) const
      { return _M_cache[static_cast<_UnsignedCharT>(__ch)]; }

      void
      _M_make_cache(true_type)
      {
	for (unsigned __i = 0; __i < _M_cache.size(); __i++)
	  _M_cache[__i] = _M_apply(static_cast<_CharT>(__i), false_type());
      }

      void
      _M_make_cache(false_type)
      { }

    private:
      std::vector<_CharT>                       _M_char_set;
      std::vector<_StringT>                     _M_equiv_set;
      std::vector<pair<_StrTransT, _StrTransT>> _M_range_set;
      std::vector<_CharClassT>                  _M_neg_class_set;
      _CharClassT                               _M_class_set;
      _TransT                                   _M_translator;
      const _TraitsT&                           _M_traits;
      bool                                      _M_is_non_matching;
      _CacheT					_M_cache;
#ifdef _GLIBCXX_DEBUG
      bool                                      _M_is_ready = false;
#endif
    };

 //@} regex-detail
} // namespace __detail
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#include <bits/regex_compiler.tcc>
