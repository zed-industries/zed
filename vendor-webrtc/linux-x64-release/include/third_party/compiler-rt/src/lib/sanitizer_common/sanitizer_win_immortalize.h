//===-- sanitizer_win_immortalize.h ---------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file is shared between AddressSanitizer, and interception.
//
// Windows-specific thread-safe and pre-CRT global initialization safe
// infrastructure to create an object whose destructor is never called.
//===----------------------------------------------------------------------===//
#if SANITIZER_WINDOWS
#  pragma once
// Requires including sanitizer_placement_new.h (which is not allowed to be
// included in headers).

#  include "sanitizer_win_defs.h"
// These types are required to satisfy XFG which requires that the names of the
// types for indirect calls to be correct as well as the name of the original
// type for any typedefs.

// TODO: There must be a better way to do this
#  ifndef _WINDOWS_
typedef void* PVOID;
typedef int BOOL;
typedef union _RTL_RUN_ONCE {
  PVOID ptr;
} INIT_ONCE, *PINIT_ONCE;

extern "C" {
__declspec(dllimport) int WINAPI InitOnceExecuteOnce(
    PINIT_ONCE, BOOL(WINAPI*)(PINIT_ONCE, PVOID, PVOID*), void*, void*);
}
#  endif

namespace __sanitizer {
template <class Ty>
BOOL WINAPI immortalize_impl(PINIT_ONCE, PVOID storage_ptr, PVOID*) noexcept {
  // Ty must provide a placement new operator
  new (storage_ptr) Ty();
  return 1;
}

template <class Ty, typename Arg>
BOOL WINAPI immortalize_impl(PINIT_ONCE, PVOID storage_ptr,
                             PVOID* param) noexcept {
  // Ty must provide a placement new operator
  new (storage_ptr) Ty(*((Arg*)param));
  return 1;
}

template <class Ty>
Ty& immortalize() {  // return a reference to an object that will live forever
  static INIT_ONCE flag;
  alignas(Ty) static unsigned char storage[sizeof(Ty)];
  InitOnceExecuteOnce(&flag, immortalize_impl<Ty>, &storage, nullptr);
  return reinterpret_cast<Ty&>(storage);
}

template <class Ty, typename Arg>
Ty& immortalize(
    Arg arg) {  // return a reference to an object that will live forever
  static INIT_ONCE flag;
  alignas(Ty) static unsigned char storage[sizeof(Ty)];
  InitOnceExecuteOnce(&flag, immortalize_impl<Ty, Arg>, &storage, &arg);
  return reinterpret_cast<Ty&>(storage);
}
}  // namespace __sanitizer
#endif  // SANITIZER_WINDOWS
