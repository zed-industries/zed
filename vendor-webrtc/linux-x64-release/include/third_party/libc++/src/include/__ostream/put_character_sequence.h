//===---------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===---------------------------------------------------------------------===//

#ifndef _LIBCPP___OSTREAM_PUT_CHARACTER_SEQUENCE_H
#define _LIBCPP___OSTREAM_PUT_CHARACTER_SEQUENCE_H

#include <__config>

#if _LIBCPP_HAS_LOCALIZATION

#  include <__cstddef/size_t.h>
#  include <__fwd/ostream.h>
#  include <__iterator/ostreambuf_iterator.h>
#  include <__locale_dir/pad_and_output.h>
#  include <ios>

#  if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#    pragma GCC system_header
#  endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _CharT, class _Traits>
_LIBCPP_HIDE_FROM_ABI basic_ostream<_CharT, _Traits>&
__put_character_sequence(basic_ostream<_CharT, _Traits>& __os, const _CharT* __str, size_t __len) {
#  if _LIBCPP_HAS_EXCEPTIONS
  try {
#  endif // _LIBCPP_HAS_EXCEPTIONS
    typename basic_ostream<_CharT, _Traits>::sentry __s(__os);
    if (__s) {
      typedef ostreambuf_iterator<_CharT, _Traits> _Ip;
      if (std::__pad_and_output(
              _Ip(__os),
              __str,
              (__os.flags() & ios_base::adjustfield) == ios_base::left ? __str + __len : __str,
              __str + __len,
              __os,
              __os.fill())
              .failed())
        __os.setstate(ios_base::badbit | ios_base::failbit);
    }
#  if _LIBCPP_HAS_EXCEPTIONS
  } catch (...) {
    __os.__set_badbit_and_consider_rethrow();
  }
#  endif // _LIBCPP_HAS_EXCEPTIONS
  return __os;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_HAS_LOCALIZATION

#endif // _LIBCPP___OSTREAM_PUT_CHARACTER_SEQUENCE_H
