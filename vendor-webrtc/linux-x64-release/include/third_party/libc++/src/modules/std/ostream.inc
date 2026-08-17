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
  using std::basic_ostream;

  using std::ostream;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wostream;
#  endif

  using std::endl;
  using std::ends;
  using std::flush;

#  if 0
  using std::emit_on_flush;
  using std::flush_emit;
  using std::noemit_on_flush;
#  endif
  using std::operator<<;

#  if _LIBCPP_STD_VER >= 23
  // [ostream.formatted.print], print functions
  using std::print;
  using std::println;

  using std::vprint_nonunicode;
#    if _LIBCPP_HAS_UNICODE
  using std::vprint_unicode;
#    endif // _LIBCPP_HAS_UNICODE
#  endif   // _LIBCPP_STD_VER >= 23

#endif // _LIBCPP_HAS_LOCALIZATION
} // namespace std
