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
#  if _LIBCPP_STD_VER < 26 || defined(_LIBCPP_ENABLE_CXX26_REMOVED_STRSTREAM)
  using std::istrstream;
  using std::ostrstream;
  using std::strstream;
  using std::strstreambuf;
#  endif // _LIBCPP_STD_VER < 26 || defined(_LIBCPP_ENABLE_CXX26_REMOVED_STRSTREAM)
#endif   // _LIBCPP_HAS_LOCALIZATION
} // namespace std
