// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
#if _LIBCPP_HAS_LOCALIZATION
  using std::basic_stringbuf;

  using std::swap;

  using std::stringbuf;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wstringbuf;
#  endif

  using std::basic_istringstream;

  using std::istringstream;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wistringstream;
#  endif

  using std::basic_ostringstream;

  using std::ostringstream;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wostringstream;
#  endif

  using std::basic_stringstream;

  using std::stringstream;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wstringstream;
#  endif
#endif // _LIBCPP_HAS_LOCALIZATION
} // namespace std
