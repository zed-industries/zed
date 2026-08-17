// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
#if _LIBCPP_HAS_LOCALIZATION && _LIBCPP_HAS_EXPERIMENTAL_SYNCSTREAM
  using std::basic_syncbuf;

  // [syncstream.syncbuf.special], specialized algorithms
  using std::swap;

  using std::syncbuf;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wsyncbuf;
#  endif
  using std::basic_osyncstream;

  using std::osyncstream;
#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wosyncstream;
#  endif
#endif // _LIBCPP_HAS_LOCALIZATION && _LIBCPP_HAS_EXPERIMENTAL_SYNCSTREAM
} // namespace std
