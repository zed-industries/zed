// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

export namespace std {
  using std::streampos;
#if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wstreampos;
#endif
  using std::u16streampos;
  using std::u32streampos;
#if _LIBCPP_HAS_CHAR8_T
  using std::u8streampos;
#endif

#if _LIBCPP_HAS_EXPERIMENTAL_SYNCSTREAM
  using std::basic_osyncstream;
  using std::basic_syncbuf;
#endif

  using std::istreambuf_iterator;
  using std::ostreambuf_iterator;

#if _LIBCPP_HAS_EXPERIMENTAL_SYNCSTREAM
  using std::osyncstream;
  using std::syncbuf;

#  if _LIBCPP_HAS_WIDE_CHARACTERS
  using std::wosyncstream;
  using std::wsyncbuf;
#  endif
#endif // _LIBCPP_HAS_EXPERIMENTAL_SYNCSTREAM

  using std::fpos;
} // namespace std
