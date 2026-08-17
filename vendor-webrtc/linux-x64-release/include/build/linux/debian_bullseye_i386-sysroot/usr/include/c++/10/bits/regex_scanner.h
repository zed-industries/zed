// class template regex -*- C++ -*-

// Copyright (C) 2013-2020 Free Software Foundation, Inc.
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
 *  @file bits/regex_scanner.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace __detail
{
  /**
   * @addtogroup regex-detail
   * @{
   */

  struct _ScannerBase
  {
  public:
    /// Token types returned from the scanner.
    enum _TokenT : unsigned
    {
      _S_token_anychar,
      _S_token_ord_char,
      _S_token_oct_num,
      _S_token_hex_num,
      _S_token_backref,
      _S_token_subexpr_begin,
      _S_token_subexpr_no_group_begin,
      _S_token_subexpr_lookahead_begin, // neg if _M_value[0] == 'n'
      _S_token_subexpr_end,
      _S_token_bracket_begin,
      _S_token_bracket_neg_begin,
      _S_token_bracket_end,
      _S_token_interval_begin,
      _S_token_interval_end,
      _S_token_quoted_class,
      _S_token_char_class_name,
      _S_token_collsymbol,
      _S_token_equiv_class_name,
      _S_token_opt,
      _S_token_or,
      _S_token_closure0,
      _S_token_closure1,
      _S_token_line_begin,
      _S_token_line_end,
      _S_token_word_bound, // neg if _M_value[0] == 'n'
      _S_token_comma,
      _S_token_dup_count,
      _S_token_eof,
      _S_token_bracket_dash,
      _S_token_unknown = -1u
    };

  protected:
    typedef regex_constants::syntax_option_type _FlagT;

    enum _StateT
    {
      _S_state_normal,
      _S_state_in_brace,
      _S_state_in_bracket,
    };

  protected:
    _ScannerBase(_FlagT __flags)
    : _M_state(_S_state_normal),
    _M_flags(__flags),
    _M_escape_tbl(_M_is_ecma()
		  ? _M_ecma_escape_tbl
		  : _M_awk_escape_tbl),
    _M_spec_char(_M_is_ecma()
		 ? _M_ecma_spec_char
		 : _M_flags & regex_constants::basic
		 ? _M_basic_spec_char
		 : _M_flags & regex_constants::extended
		 ? _M_extended_spec_char
		 : _M_flags & regex_constants::grep
		 ?  ".[\\*^$\n"
		 : _M_flags & regex_constants::egrep
		 ? ".[\\()*+?{|^$\n"
		 : _M_flags & regex_constants::awk
		 ? _M_extended_spec_char
		 : nullptr),
    _M_at_bracket_start(false)
    { __glibcxx_assert(_M_spec_char); }

  protected:
    const char*
    _M_find_escape(char __c)
    {
      auto __it = _M_escape_tbl;
      for (; __it->first != '\0'; ++__it)
	if (__it->first == __c)
	  return &__it->second;
      return nullptr;
    }

    bool
    _M_is_ecma() const
    { return _M_flags & regex_constants::ECMAScript; }

    bool
    _M_is_basic() const
    { return _M_flags & (regex_constants::basic | regex_constants::grep); }

    bool
    _M_is_extended() const
    {
      return _M_flags & (regex_constants::extended
			 | regex_constants::egrep
			 | regex_constants::awk);
    }

    bool
    _M_is_grep() const
    { return _M_flags & (regex_constants::grep | regex_constants::egrep); }

    bool
    _M_is_awk() const
    { return _M_flags & regex_constants::awk; }

  protected:
    // TODO: Make them static in the next abi change.
    const std::pair<char, _TokenT> _M_token_tbl[9] =
      {
	{'^', _S_token_line_begin},
	{'$', _S_token_line_end},
	{'.', _S_token_anychar},
	{'*', _S_token_closure0},
	{'+', _S_token_closure1},
	{'?', _S_token_opt},
	{'|', _S_token_or},
	{'\n', _S_token_or}, // grep and egrep
	{'\0', _S_token_or},
      };
    const std::pair<char, char> _M_ecma_escape_tbl[8] =
      {
	{'0', '\0'},
	{'b', '\b'},
	{'f', '\f'},
	{'n', '\n'},
	{'r', '\r'},
	{'t', '\t'},
	{'v', '\v'},
	{'\0', '\0'},
      };
    const std::pair<char, char> _M_awk_escape_tbl[11] =
      {
	{'"', '"'},
	{'/', '/'},
	{'\\', '\\'},
	{'a', '\a'},
	{'b', '\b'},
	{'f', '\f'},
	{'n', '\n'},
	{'r', '\r'},
	{'t', '\t'},
	{'v', '\v'},
	{'\0', '\0'},
      };
    const char* _M_ecma_spec_char = "^$\\.*+?()[]{}|";
    const char* _M_basic_spec_char = ".[\\*^$";
    const char* _M_extended_spec_char = ".[\\()*+?{|^$";

    _StateT                       _M_state;
    _FlagT                        _M_flags;
    _TokenT                       _M_token;
    const std::pair<char, char>*  _M_escape_tbl;
    const char*                   _M_spec_char;
    bool                          _M_at_bracket_start;
  };

  /**
   * @brief Scans an input range for regex tokens.
   *
   * The %_Scanner class interprets the regular expression pattern in
   * the input range passed to its constructor as a sequence of parse
   * tokens passed to the regular expression compiler.  The sequence
   * of tokens provided depends on the flag settings passed to the
   * constructor: different regular expression grammars will interpret
   * the same input pattern in syntactically different ways.
   */
  template<typename _CharT>
    class _Scanner
    : public _ScannerBase
    {
    public:
      typedef const _CharT*                                       _IterT;
      typedef std::basic_string<_CharT>                           _StringT;
      typedef regex_constants::syntax_option_type                 _FlagT;
      typedef const std::ctype<_CharT>                            _CtypeT;

      _Scanner(_IterT __begin, _IterT __end,
	       _FlagT __flags, std::locale __loc);

      void
      _M_advance();

      _TokenT
      _M_get_token() const
      { return _M_token; }

      const _StringT&
      _M_get_value() const
      { return _M_value; }

#ifdef _GLIBCXX_DEBUG
      std::ostream&
      _M_print(std::ostream&);
#endif

    private:
      void
      _M_scan_normal();

      void
      _M_scan_in_bracket();

      void
      _M_scan_in_brace();

      void
      _M_eat_escape_ecma();

      void
      _M_eat_escape_posix();

      void
      _M_eat_escape_awk();

      void
      _M_eat_class(char);

      _IterT                        _M_current;
      _IterT                        _M_end;
      _CtypeT&                      _M_ctype;
      _StringT                      _M_value;
      void (_Scanner::* _M_eat_escape)();
    };

 //@} regex-detail
} // namespace __detail
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#include <bits/regex_scanner.tcc>
