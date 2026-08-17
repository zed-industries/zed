//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___LOCALE_DIR_PAD_AND_OUTPUT_H
#define _LIBCPP___LOCALE_DIR_PAD_AND_OUTPUT_H

#include <__config>

#if _LIBCPP_HAS_LOCALIZATION

#  include <ios>

#  if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#    pragma GCC system_header
#  endif

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _CharT, class _OutputIterator>
_LIBCPP_HIDE_FROM_ABI _OutputIterator __pad_and_output(
    _OutputIterator __s, const _CharT* __ob, const _CharT* __op, const _CharT* __oe, ios_base& __iob, _CharT __fl) {
  streamsize __sz = __oe - __ob;
  streamsize __ns = __iob.width();
  if (__ns > __sz)
    __ns -= __sz;
  else
    __ns = 0;
  for (; __ob < __op; ++__ob, ++__s)
    *__s = *__ob;
  for (; __ns; --__ns, ++__s)
    *__s = __fl;
  for (; __ob < __oe; ++__ob, ++__s)
    *__s = *__ob;
  __iob.width(0);
  return __s;
}

template <class _CharT, class _Traits>
_LIBCPP_HIDE_FROM_ABI ostreambuf_iterator<_CharT, _Traits> __pad_and_output(
    ostreambuf_iterator<_CharT, _Traits> __s,
    const _CharT* __ob,
    const _CharT* __op,
    const _CharT* __oe,
    ios_base& __iob,
    _CharT __fl) {
  if (__s.__sbuf_ == nullptr)
    return __s;
  streamsize __sz = __oe - __ob;
  streamsize __ns = __iob.width();
  if (__ns > __sz)
    __ns -= __sz;
  else
    __ns = 0;
  streamsize __np = __op - __ob;
  if (__np > 0) {
    if (__s.__sbuf_->sputn(__ob, __np) != __np) {
      __s.__sbuf_ = nullptr;
      return __s;
    }
  }
  if (__ns > 0) {
    basic_string<_CharT, _Traits> __sp(__ns, __fl);
    if (__s.__sbuf_->sputn(__sp.data(), __ns) != __ns) {
      __s.__sbuf_ = nullptr;
      return __s;
    }
  }
  __np = __oe - __op;
  if (__np > 0) {
    if (__s.__sbuf_->sputn(__op, __np) != __np) {
      __s.__sbuf_ = nullptr;
      return __s;
    }
  }
  __iob.width(0);
  return __s;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_HAS_LOCALIZATION

#endif // _LIBCPP___LOCALE_DIR_PAD_AND_OUTPUT_H
