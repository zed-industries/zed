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
 *  @file bits/regex_constants.h
 *  @brief Constant definitions for the std regex library.
 *
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

/**
 * @defgroup regex Regular Expressions
 *
 * A facility for performing regular expression pattern matching.
 * @{
 */

/**
 * @namespace std::regex_constants
 * @brief ISO C++ 2011 namespace for options and flags used with std::regex
 */
namespace regex_constants
{
  /**
   * @name 5.1 Regular Expression Syntax Options
   */
  //@{
  enum __syntax_option
  {
    _S_icase,
    _S_nosubs,
    _S_optimize,
    _S_collate,
    _S_ECMAScript,
    _S_basic,
    _S_extended,
    _S_awk,
    _S_grep,
    _S_egrep,
    _S_polynomial,
    _S_syntax_last
  };

  /**
   * @brief This is a bitmask type indicating how to interpret the regex.
   *
   * The @c syntax_option_type is implementation defined but it is valid to
   * perform bitwise operations on these values and expect the right thing to
   * happen.
   *
   * A valid value of type syntax_option_type shall have exactly one of the
   * elements @c ECMAScript, @c basic, @c extended, @c awk, @c grep, @c egrep
   * %set.
   */
  enum syntax_option_type : unsigned int { };

  /**
   * Specifies that the matching of regular expressions against a character
   * sequence shall be performed without regard to case.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type icase =
    static_cast<syntax_option_type>(1 << _S_icase);

  /**
   * Specifies that when a regular expression is matched against a character
   * container sequence, no sub-expression matches are to be stored in the
   * supplied match_results structure.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type nosubs =
    static_cast<syntax_option_type>(1 << _S_nosubs);

  /**
   * Specifies that the regular expression engine should pay more attention to
   * the speed with which regular expressions are matched, and less to the
   * speed with which regular expression objects are constructed. Otherwise
   * it has no detectable effect on the program output.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type optimize =
    static_cast<syntax_option_type>(1 << _S_optimize);

  /**
   * Specifies that character ranges of the form [a-b] should be locale
   * sensitive.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type collate =
    static_cast<syntax_option_type>(1 << _S_collate);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by ECMAScript in ECMA-262 [Ecma International, ECMAScript
   * Language Specification, Standard Ecma-262, third edition, 1999], as
   * modified in section [28.13].  This grammar is similar to that defined
   * in the PERL scripting language but extended with elements found in the
   * POSIX regular expression grammar.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type ECMAScript =
    static_cast<syntax_option_type>(1 << _S_ECMAScript);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by POSIX basic regular expressions in IEEE Std 1003.1-2001,
   * Portable Operating System Interface (POSIX), Base Definitions and
   * Headers, Section 9, Regular Expressions [IEEE, Information Technology --
   * Portable Operating System Interface (POSIX), IEEE Standard 1003.1-2001].
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type basic =
    static_cast<syntax_option_type>(1 << _S_basic);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by POSIX extended regular expressions in IEEE Std 1003.1-2001,
   * Portable Operating System Interface (POSIX), Base Definitions and
   * Headers, Section 9, Regular Expressions.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type extended =
    static_cast<syntax_option_type>(1 << _S_extended);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by POSIX utility awk in IEEE Std 1003.1-2001.  This option is
   * identical to syntax_option_type extended, except that C-style escape
   * sequences are supported.  These sequences are:
   * \\\\, \\a, \\b, \\f, \\n, \\r, \\t , \\v, \\&apos,, &apos,,
   * and \\ddd (where ddd is one, two, or three octal digits).
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type awk =
    static_cast<syntax_option_type>(1 << _S_awk);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by POSIX utility grep in IEEE Std 1003.1-2001.  This option is
   * identical to syntax_option_type basic, except that newlines are treated
   * as whitespace.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type grep =
    static_cast<syntax_option_type>(1 << _S_grep);

  /**
   * Specifies that the grammar recognized by the regular expression engine is
   * that used by POSIX utility grep when given the -E option in
   * IEEE Std 1003.1-2001.  This option is identical to syntax_option_type
   * extended, except that newlines are treated as whitespace.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type egrep =
    static_cast<syntax_option_type>(1 << _S_egrep);

  /**
   * Extension: Ensure both space complexity of compiled regex and
   * time complexity execution are not exponential.
   * If specified in a regex with back-references, the exception
   * regex_constants::error_complexity will be thrown.
   */
  _GLIBCXX17_INLINE constexpr syntax_option_type __polynomial =
    static_cast<syntax_option_type>(1 << _S_polynomial);

  constexpr inline syntax_option_type
  operator&(syntax_option_type __a, syntax_option_type __b)
  {
    return (syntax_option_type)(static_cast<unsigned int>(__a)
				& static_cast<unsigned int>(__b));
  }

  constexpr inline syntax_option_type
  operator|(syntax_option_type __a, syntax_option_type __b)
  {
    return (syntax_option_type)(static_cast<unsigned int>(__a)
				| static_cast<unsigned int>(__b));
  }

  constexpr inline syntax_option_type
  operator^(syntax_option_type __a, syntax_option_type __b)
  {
    return (syntax_option_type)(static_cast<unsigned int>(__a)
				^ static_cast<unsigned int>(__b));
  }

  constexpr inline syntax_option_type
  operator~(syntax_option_type __a)
  { return (syntax_option_type)(~static_cast<unsigned int>(__a)); }

  inline syntax_option_type&
  operator&=(syntax_option_type& __a, syntax_option_type __b)
  { return __a = __a & __b; }

  inline syntax_option_type&
  operator|=(syntax_option_type& __a, syntax_option_type __b)
  { return __a = __a | __b; }

  inline syntax_option_type&
  operator^=(syntax_option_type& __a, syntax_option_type __b)
  { return __a = __a ^ __b; }

  //@}

  /**
   * @name 5.2 Matching Rules
   *
   * Matching a regular expression against a sequence of characters [first,
   * last) proceeds according to the rules of the grammar specified for the
   * regular expression object, modified according to the effects listed
   * below for any bitmask elements set.
   *
   */
  //@{

  enum __match_flag
  {
    _S_not_bol,
    _S_not_eol,
    _S_not_bow,
    _S_not_eow,
    _S_any,
    _S_not_null,
    _S_continuous,
    _S_prev_avail,
    _S_sed,
    _S_no_copy,
    _S_first_only,
    _S_match_flag_last
  };

  /**
   * @brief This is a bitmask type indicating regex matching rules.
   *
   * The @c match_flag_type is implementation defined but it is valid to
   * perform bitwise operations on these values and expect the right thing to
   * happen.
   */
  enum match_flag_type : unsigned int { };

  /**
   * The default matching rules.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_default =
    static_cast<match_flag_type>(0);

  /**
   * The first character in the sequence [first, last) is treated as though it
   * is not at the beginning of a line, so the character (^) in the regular
   * expression shall not match [first, first).
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_not_bol =
    static_cast<match_flag_type>(1 << _S_not_bol);

  /**
   * The last character in the sequence [first, last) is treated as though it
   * is not at the end of a line, so the character ($) in the regular
   * expression shall not match [last, last).
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_not_eol =
    static_cast<match_flag_type>(1 << _S_not_eol);

  /**
   * The expression \\b is not matched against the sub-sequence
   * [first,first).
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_not_bow =
    static_cast<match_flag_type>(1 << _S_not_bow);

  /**
   * The expression \\b should not be matched against the sub-sequence
   * [last,last).
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_not_eow =
    static_cast<match_flag_type>(1 << _S_not_eow);

  /**
   * If more than one match is possible then any match is an acceptable
   * result.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_any =
    static_cast<match_flag_type>(1 << _S_any);

  /**
   * The expression does not match an empty sequence.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_not_null =
    static_cast<match_flag_type>(1 << _S_not_null);

  /**
   * The expression only matches a sub-sequence that begins at first .
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_continuous =
    static_cast<match_flag_type>(1 << _S_continuous);

  /**
   * --first is a valid iterator position.  When this flag is set then the
   * flags match_not_bol and match_not_bow are ignored by the regular
   * expression algorithms 28.11 and iterators 28.12.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type match_prev_avail =
    static_cast<match_flag_type>(1 << _S_prev_avail);

  /**
   * When a regular expression match is to be replaced by a new string, the
   * new string is constructed using the rules used by the ECMAScript replace
   * function in ECMA- 262 [Ecma International, ECMAScript Language
   * Specification, Standard Ecma-262, third edition, 1999], part 15.5.4.11
   * String.prototype.replace. In addition, during search and replace
   * operations all non-overlapping occurrences of the regular expression
   * are located and replaced, and sections of the input that did not match
   * the expression are copied unchanged to the output string.
   *
   * Format strings (from ECMA-262 [15.5.4.11]):
   * @li $$  The dollar-sign itself ($)
   * @li $&  The matched substring.
   * @li $`  The portion of @a string that precedes the matched substring.
   *         This would be match_results::prefix().
   * @li $'  The portion of @a string that follows the matched substring.
   *         This would be match_results::suffix().
   * @li $n  The nth capture, where n is in [1,9] and $n is not followed by a
   *         decimal digit.  If n <= match_results::size() and the nth capture
   *         is undefined, use the empty string instead.  If n >
   *         match_results::size(), the result is implementation-defined.
   * @li $nn The nnth capture, where nn is a two-digit decimal number on
   *         [01, 99].  If nn <= match_results::size() and the nth capture is
   *         undefined, use the empty string instead. If
   *         nn > match_results::size(), the result is implementation-defined.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type format_default =
    static_cast<match_flag_type>(0);

  /**
   * When a regular expression match is to be replaced by a new string, the
   * new string is constructed using the rules used by the POSIX sed utility
   * in IEEE Std 1003.1- 2001 [IEEE, Information Technology -- Portable
   * Operating System Interface (POSIX), IEEE Standard 1003.1-2001].
   */
  _GLIBCXX17_INLINE constexpr match_flag_type format_sed =
    static_cast<match_flag_type>(1 << _S_sed);

  /**
   * During a search and replace operation, sections of the character
   * container sequence being searched that do not match the regular
   * expression shall not be copied to the output string.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type format_no_copy =
    static_cast<match_flag_type>(1 << _S_no_copy);

  /**
   * When specified during a search and replace operation, only the first
   * occurrence of the regular expression shall be replaced.
   */
  _GLIBCXX17_INLINE constexpr match_flag_type format_first_only =
    static_cast<match_flag_type>(1 << _S_first_only);

  constexpr inline match_flag_type
  operator&(match_flag_type __a, match_flag_type __b)
  {
    return (match_flag_type)(static_cast<unsigned int>(__a)
				& static_cast<unsigned int>(__b));
  }

  constexpr inline match_flag_type
  operator|(match_flag_type __a, match_flag_type __b)
  {
    return (match_flag_type)(static_cast<unsigned int>(__a)
				| static_cast<unsigned int>(__b));
  }

  constexpr inline match_flag_type
  operator^(match_flag_type __a, match_flag_type __b)
  {
    return (match_flag_type)(static_cast<unsigned int>(__a)
				^ static_cast<unsigned int>(__b));
  }

  constexpr inline match_flag_type
  operator~(match_flag_type __a)
  { return (match_flag_type)(~static_cast<unsigned int>(__a)); }

  inline match_flag_type&
  operator&=(match_flag_type& __a, match_flag_type __b)
  { return __a = __a & __b; }

  inline match_flag_type&
  operator|=(match_flag_type& __a, match_flag_type __b)
  { return __a = __a | __b; }

  inline match_flag_type&
  operator^=(match_flag_type& __a, match_flag_type __b)
  { return __a = __a ^ __b; }

  //@}
} // namespace regex_constants
/* @} */ // group regex

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

