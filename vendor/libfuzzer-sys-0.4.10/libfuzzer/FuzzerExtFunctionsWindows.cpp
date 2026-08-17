//=== FuzzerExtWindows.cpp - Interface to external functions --------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
// Implementation of FuzzerExtFunctions for Windows. Uses alternatename when
// compiled with MSVC. Uses weak aliases when compiled with clang. Unfortunately
// the method each compiler supports is not supported by the other.
//===----------------------------------------------------------------------===//
#include "FuzzerPlatform.h"
#if LIBFUZZER_WINDOWS

#include "FuzzerExtFunctions.h"
#include "FuzzerIO.h"
#include <stdlib.h>

using namespace fuzzer;

// Intermediate macro to ensure the parameter is expanded before stringified.
#define STRINGIFY_(A) #A
#define STRINGIFY(A) STRINGIFY_(A)

#if LIBFUZZER_MSVC
#define GET_FUNCTION_ADDRESS(fn) &fn
#else
#define GET_FUNCTION_ADDRESS(fn) __builtin_function_start(fn)
#endif // LIBFUZER_MSVC

// Copied from compiler-rt/lib/sanitizer_common/sanitizer_win_defs.h
#if defined(_M_IX86) || defined(__i386__)
#define WIN_SYM_PREFIX "_"
#else
#define WIN_SYM_PREFIX
#endif

// Declare external functions as having alternativenames, so that we can
// determine if they are not defined.
#define EXTERNAL_FUNC(Name, Default)                                           \
  __pragma(comment(linker, "/alternatename:" WIN_SYM_PREFIX STRINGIFY(         \
                               Name) "=" WIN_SYM_PREFIX STRINGIFY(Default)))

extern "C" {
#define EXT_FUNC(NAME, RETURN_TYPE, FUNC_SIG, WARN)         \
  RETURN_TYPE NAME##Def FUNC_SIG {                          \
    Printf("ERROR: Function \"%s\" not defined.\n", #NAME); \
    exit(1);                                                \
  }                                                         \
  EXTERNAL_FUNC(NAME, NAME##Def) RETURN_TYPE NAME FUNC_SIG

#include "FuzzerExtFunctions.def"

#undef EXT_FUNC
}

template <typename T>
static T *GetFnPtr(void *Fun, void *FunDef, const char *FnName,
                   bool WarnIfMissing) {
  if (Fun == FunDef) {
    if (WarnIfMissing)
      Printf("WARNING: Failed to find function \"%s\".\n", FnName);
    return nullptr;
  }
  return (T *)Fun;
}

namespace fuzzer {

ExternalFunctions::ExternalFunctions() {
#define EXT_FUNC(NAME, RETURN_TYPE, FUNC_SIG, WARN)                            \
  this->NAME = GetFnPtr<decltype(::NAME)>(GET_FUNCTION_ADDRESS(::NAME),        \
                                          GET_FUNCTION_ADDRESS(::NAME##Def),   \
                                          #NAME, WARN);

#include "FuzzerExtFunctions.def"

#undef EXT_FUNC
}

}  // namespace fuzzer

#endif // LIBFUZZER_WINDOWS
