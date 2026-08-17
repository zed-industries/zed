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
 * @file bits/regex_error.h
 * @brief Error and exception objects for the std regex library.
 *
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{regex}
 */

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

/**
 * @addtogroup regex
 * @{
 */

namespace regex_constants
{
  /**
   * @name 5.3 Error Types
   */
  //@{

  enum error_type
    {
      _S_error_collate,
      _S_error_ctype,
      _S_error_escape,
      _S_error_backref,
      _S_error_brack,
      _S_error_paren,
      _S_error_brace,
      _S_error_badbrace,
      _S_error_range,
      _S_error_space,
      _S_error_badrepeat,
      _S_error_complexity,
      _S_error_stack,
    };

  /** The expression contained an invalid collating element name. */
  constexpr error_type error_collate(_S_error_collate);

  /** The expression contained an invalid character class name. */
  constexpr error_type error_ctype(_S_error_ctype);

  /**
   * The expression contained an invalid escaped character, or a trailing
   * escape.
   */
  constexpr error_type error_escape(_S_error_escape);

  /** The expression contained an invalid back reference. */
  constexpr error_type error_backref(_S_error_backref);

  /** The expression contained mismatched [ and ]. */
  constexpr error_type error_brack(_S_error_brack);

  /** The expression contained mismatched ( and ). */
  constexpr error_type error_paren(_S_error_paren);

  /** The expression contained mismatched { and } */
  constexpr error_type error_brace(_S_error_brace);

  /** The expression contained an invalid range in a {} expression. */
  constexpr error_type error_badbrace(_S_error_badbrace);

  /**
   * The expression contained an invalid character range,
   * such as [b-a] in most encodings.
   */
  constexpr error_type error_range(_S_error_range);

  /**
   * There was insufficient memory to convert the expression into a
   * finite state machine.
   */
  constexpr error_type error_space(_S_error_space);

  /**
   * One of <em>*?+{</em> was not preceded by a valid regular expression.
   */
  constexpr error_type error_badrepeat(_S_error_badrepeat);

  /**
   * The complexity of an attempted match against a regular expression
   * exceeded a pre-set level.
   */
  constexpr error_type error_complexity(_S_error_complexity);

  /**
   * There was insufficient memory to determine whether the
   * regular expression could match the specified character sequence.
   */
  constexpr error_type error_stack(_S_error_stack);

  //@}
} // namespace regex_constants

  // [7.8] Class regex_error
  /**
   * @brief A regular expression exception class.
   * @ingroup exceptions
   *
   * The regular expression library throws objects of this class on error.
   */
  class regex_error : public std::runtime_error
  {
    regex_constants::error_type _M_code;

  public:
    /**
     * @brief Constructs a regex_error object.
     *
     * @param __ecode the regex error code.
     */
    explicit
    regex_error(regex_constants::error_type __ecode);

    virtual ~regex_error() throw();

    /**
     * @brief Gets the regex error code.
     *
     * @returns the regex error code.
     */
    regex_constants::error_type
    code() const
    { return _M_code; }

  private:
    regex_error(regex_constants::error_type __ecode, const char* __what)
    : std::runtime_error(__what), _M_code(__ecode)
    { }

    friend void __throw_regex_error(regex_constants::error_type, const char*);
  };

  //@} // group regex

  void
  __throw_regex_error(regex_constants::error_type __ecode);

  inline void
  __throw_regex_error(regex_constants::error_type __ecode, const char* __what)
  { _GLIBCXX_THROW_OR_ABORT(regex_error(__ecode, __what)); }

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
