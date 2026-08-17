//===-- sanitizer_win_thunk_interception.h -------------------------  -----===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// This header provide helper macros and functions to delegate calls to the
// shared runtime that lives in the sanitizer DLL.
//===----------------------------------------------------------------------===//

#ifndef SANITIZER_WIN_THUNK_INTERCEPTION_H
#define SANITIZER_WIN_THUNK_INTERCEPTION_H
#include <stdint.h>

#include "sanitizer_internal_defs.h"

extern "C" {
__declspec(dllimport) bool __cdecl __sanitizer_override_function(
    const char *export_name, __sanitizer::uptr user_function,
    __sanitizer::uptr *old_function = nullptr);
__declspec(dllimport) bool __cdecl __sanitizer_override_function_by_addr(
    __sanitizer::uptr source_function, __sanitizer::uptr target_function,
    __sanitizer::uptr *old_target_function = nullptr);
__declspec(dllimport) bool __cdecl __sanitizer_register_weak_function(
    const char *export_name, __sanitizer::uptr user_function,
    __sanitizer::uptr *old_function = nullptr);
}

using sanitizer_thunk = int (*)();

namespace __sanitizer {
int override_function(const char *export_name, uptr user_function);
int register_weak(const char *export_name, uptr user_function);
void initialize_thunks(const sanitizer_thunk *begin,
                       const sanitizer_thunk *end);
}  // namespace __sanitizer

// -------------------- Function interception macros ------------------------ //
// We can't define our own version of strlen etc. because that would lead to
// link-time or even type mismatch errors.  Instead, we can declare a function
// just to be able to get its address.  Me may miss the first few calls to the
// functions since it can be called before __dll_thunk_init, but that would lead
// to false negatives in the startup code before user's global initializers,
// which isn't a big deal.
// Use .INTR segment to register function pointers that are iterated over during
// startup that will replace local_function with sanitizer_export.

#define INTERCEPT_LIBRARY_FUNCTION(local_function, sanitizer_export)   \
  extern "C" void local_function();                                    \
  static int intercept_##local_function() {                            \
    return __sanitizer::override_function(                             \
        sanitizer_export,                                              \
        reinterpret_cast<__sanitizer::uptr>(local_function));          \
  }                                                                    \
  __pragma(section(".INTR$M", long, read)) __declspec(allocate(        \
      ".INTR$M")) int (*__sanitizer_static_thunk_##local_function)() = \
      intercept_##local_function;

// ------------------ Weak symbol registration macros ---------------------- //
// Use .WEAK segment to register function pointers that are iterated over during
// startup that will replace sanitizer_export with local_function
#ifdef __clang__
#  define REGISTER_WEAK_OPTNONE __attribute__((optnone))
#  define REGISTER_WEAK_FUNCTION_ADDRESS(fn) __builtin_function_start(fn)
#else
#  define REGISTER_WEAK_OPTNONE
#  define REGISTER_WEAK_FUNCTION_ADDRESS(fn) &fn
#endif

#define REGISTER_WEAK_FUNCTION(local_function)                          \
  extern "C" void local_function();                                     \
  extern "C" void WEAK_EXPORT_NAME(local_function)();                   \
  WIN_WEAK_IMPORT_DEF(local_function)                                   \
  REGISTER_WEAK_OPTNONE static int register_weak_##local_function() {   \
    if ((uintptr_t)REGISTER_WEAK_FUNCTION_ADDRESS(local_function) !=    \
        (uintptr_t)REGISTER_WEAK_FUNCTION_ADDRESS(                      \
            WEAK_EXPORT_NAME(local_function))) {                        \
      return __sanitizer::register_weak(                                \
          SANITIZER_STRINGIFY(WEAK_EXPORT_NAME(local_function)),        \
          reinterpret_cast<__sanitizer::uptr>(local_function));         \
    }                                                                   \
    return 0;                                                           \
  }                                                                     \
  __pragma(section(".WEAK$M", long, read)) __declspec(allocate(         \
      ".WEAK$M")) int (*__sanitizer_register_weak_##local_function)() = \
      register_weak_##local_function;
#endif  // SANITIZER_WIN_STATIC_RUNTIME_THUNK_H
