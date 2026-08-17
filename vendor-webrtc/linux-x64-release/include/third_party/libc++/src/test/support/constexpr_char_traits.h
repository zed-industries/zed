// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _CONSTEXPR_CHAR_TRAITS
#define _CONSTEXPR_CHAR_TRAITS

#include <string>
#include <cassert>
#include <cstddef>

#include "test_macros.h"

// Tests whether the pointer p is in the range [first, last).
//
// Precondition: The range [first, last) is a valid range.
//
// Typically the pointers are compared with less than. This is not allowed when
// the pointers belong to different ranges, which is UB. Typically, this is
// benign at run-time, however since UB is not allowed during constant
// evaluation this does not compile. This function does the validation without
// UB.
//
// When p is in the range [first, last) the data can be copied from the
// beginning to the end. Otherwise it needs to be copied from the end to the
// beginning.
template <class CharT>
TEST_CONSTEXPR_CXX14 bool is_pointer_in_range(const CharT* first, const CharT* last, const CharT* p) {
  if (first == p) // Needed when n == 0
    return true;

  for (; first != last; ++first)
    if (first == p)
      return true;

  return false;
}

template <class CharT>
struct constexpr_char_traits
{
    typedef CharT     char_type;
    typedef int       int_type;
    typedef std::streamoff off_type;
    typedef std::streampos pos_type;
    typedef std::mbstate_t state_type;
    // The comparison_category is omitted so the class will have weak_ordering
    // in C++20. This is intentional.

    static TEST_CONSTEXPR_CXX14 void assign(char_type& c1, const char_type& c2) TEST_NOEXCEPT
        {c1 = c2;}

    static TEST_CONSTEXPR bool eq(char_type c1, char_type c2) TEST_NOEXCEPT
        {return c1 == c2;}

    static TEST_CONSTEXPR  bool lt(char_type c1, char_type c2) TEST_NOEXCEPT
        {return c1 < c2;}

    static TEST_CONSTEXPR_CXX14 int              compare(const char_type* s1, const char_type* s2, std::size_t n);
    static TEST_CONSTEXPR_CXX14 std::size_t           length(const char_type* s);
    static TEST_CONSTEXPR_CXX14 const char_type* find(const char_type* s, std::size_t n, const char_type& a);
    static TEST_CONSTEXPR_CXX14 char_type*       move(char_type* s1, const char_type* s2, std::size_t n);
    static TEST_CONSTEXPR_CXX14 char_type*       copy(char_type* s1, const char_type* s2, std::size_t n);
    static TEST_CONSTEXPR_CXX14 char_type*       assign(char_type* s, std::size_t n, char_type a);

    static TEST_CONSTEXPR int_type  not_eof(int_type c) TEST_NOEXCEPT
        {return eq_int_type(c, eof()) ? ~eof() : c;}

    static TEST_CONSTEXPR char_type to_char_type(int_type c) TEST_NOEXCEPT
        {return char_type(c);}

    static TEST_CONSTEXPR int_type  to_int_type(char_type c) TEST_NOEXCEPT
        {return int_type(c);}

    static TEST_CONSTEXPR bool      eq_int_type(int_type c1, int_type c2) TEST_NOEXCEPT
        {return c1 == c2;}

    static TEST_CONSTEXPR int_type  eof() TEST_NOEXCEPT
        {return int_type(EOF);}
};


template <class CharT>
TEST_CONSTEXPR_CXX14 int
constexpr_char_traits<CharT>::compare(const char_type* s1, const char_type* s2, std::size_t n)
{
    for (; n; --n, ++s1, ++s2)
    {
        if (lt(*s1, *s2))
            return -1;
        if (lt(*s2, *s1))
            return 1;
    }
    return 0;
}

template <class CharT>
TEST_CONSTEXPR_CXX14 std::size_t
constexpr_char_traits<CharT>::length(const char_type* s)
{
    std::size_t len = 0;
    for (; !eq(*s, char_type(0)); ++s)
        ++len;
    return len;
}

template <class CharT>
TEST_CONSTEXPR_CXX14 const CharT*
constexpr_char_traits<CharT>::find(const char_type* s, std::size_t n, const char_type& a)
{
    for (; n; --n)
    {
        if (eq(*s, a))
            return s;
        ++s;
    }
    return 0;
}

template <class CharT>
TEST_CONSTEXPR_CXX14 CharT* constexpr_char_traits<CharT>::move(char_type* s1, const char_type* s2, std::size_t n) {
  if (s1 == s2)
    return s1;

  char_type* r = s1;
  if (is_pointer_in_range(s1, s1 + n, s2)) {
    for (; n; --n)
      assign(*s1++, *s2++);
  } else {
    s1 += n;
    s2 += n;
    for (; n; --n)
      assign(*--s1, *--s2);
  }
  return r;
}

template <class CharT>
TEST_CONSTEXPR_CXX14 CharT*
constexpr_char_traits<CharT>::copy(char_type* s1, const char_type* s2, std::size_t n)
{
    if (!TEST_IS_CONSTANT_EVALUATED) // fails in constexpr because we might be comparing unrelated pointers
        assert(s2 < s1 || s2 >= s1+n);
    char_type* r = s1;
    for (; n; --n, ++s1, ++s2)
        assign(*s1, *s2);
    return r;
}

template <class CharT>
TEST_CONSTEXPR_CXX14 CharT*
constexpr_char_traits<CharT>::assign(char_type* s, std::size_t n, char_type a)
{
    char_type* r = s;
    for (; n; --n, ++s)
        assign(*s, a);
    return r;
}

#endif // _CONSTEXPR_CHAR_TRAITS
