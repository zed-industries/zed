//===-- Common internal contructs -------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC___SUPPORT_COMMON_H
#define LLVM_LIBC_SRC___SUPPORT_COMMON_H

#ifndef LIBC_NAMESPACE
#error "LIBC_NAMESPACE macro is not defined."
#endif

#include "src/__support/macros/attributes.h"
#include "src/__support/macros/config.h"
#include "src/__support/macros/properties/architectures.h"

#ifndef LLVM_LIBC_FUNCTION_ATTR
#define LLVM_LIBC_FUNCTION_ATTR
#endif

// clang-format off
// Allow each function `func` to have extra attributes specified by defining:
// `LLVM_LIBC_FUNCTION_ATTR_func` macro, which should always start with
// "LLVM_LIBC_EMPTY, "
//
// For examples:
// #define LLVM_LIBC_FUNCTION_ATTR_memcpy LLVM_LIBC_EMPTY, [[gnu::weak]]
// #define LLVM_LIBC_FUNCTION_ATTR_memchr LLVM_LIBC_EMPTY, [[gnu::weak]] [[gnu::visibility("default")]]
// clang-format on
#define LLVM_LIBC_EMPTY

#define GET_SECOND(first, second, ...) second
#define EXPAND_THEN_SECOND(name) GET_SECOND(name, LLVM_LIBC_EMPTY)

#define LLVM_LIBC_ATTR(name) EXPAND_THEN_SECOND(LLVM_LIBC_FUNCTION_ATTR_##name)

// At the moment, [[gnu::alias()]] is not supported on MacOS, and it is needed
// to cleanly export and alias the C++ symbol `LIBC_NAMESPACE::func` with the C
// symbol `func`.  So for public packaging on MacOS, we will only export the C
// symbol.  Moreover, a C symbol `func` in macOS is mangled as `_func`.
#if defined(LIBC_COPT_PUBLIC_PACKAGING)
#ifndef __APPLE__
#define LLVM_LIBC_FUNCTION_IMPL(type, name, arglist)                           \
  LLVM_LIBC_ATTR(name)                                                         \
  LLVM_LIBC_FUNCTION_ATTR decltype(LIBC_NAMESPACE::name)                       \
      __##name##_impl__ __asm__(#name);                                        \
  decltype(LIBC_NAMESPACE::name) name [[gnu::alias(#name)]];                   \
  type __##name##_impl__ arglist
#else // __APPLE__
#define LLVM_LIBC_FUNCTION_IMPL(type, name, arglist)                           \
  LLVM_LIBC_ATTR(name)                                                         \
  LLVM_LIBC_FUNCTION_ATTR decltype(LIBC_NAMESPACE::name) name asm("_" #name);  \
  type name arglist
#endif // __APPLE__
#else  // LIBC_COPT_PUBLIC_PACKAGING
#define LLVM_LIBC_FUNCTION_IMPL(type, name, arglist) type name arglist
#endif // LIBC_COPT_PUBLIC_PACKAGING

// This extra layer of macro allows `name` to be a macro to rename a function.
#define LLVM_LIBC_FUNCTION(type, name, arglist)                                \
  LLVM_LIBC_FUNCTION_IMPL(type, name, arglist)

namespace LIBC_NAMESPACE_DECL {
namespace internal {
LIBC_INLINE constexpr bool same_string(char const *lhs, char const *rhs) {
  for (; *lhs || *rhs; ++lhs, ++rhs)
    if (*lhs != *rhs)
      return false;
  return true;
}
} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#define __LIBC_MACRO_TO_STRING(str) #str
#define LIBC_MACRO_TO_STRING(str) __LIBC_MACRO_TO_STRING(str)

// LLVM_LIBC_IS_DEFINED checks whether a particular macro is defined.
// Usage: constexpr bool kUseAvx = LLVM_LIBC_IS_DEFINED(__AVX__);
//
// This works by comparing the stringified version of the macro with and without
// evaluation. If FOO is not undefined both stringifications yield "FOO". If FOO
// is defined, one stringification yields "FOO" while the other yields its
// stringified value "1".
#define LLVM_LIBC_IS_DEFINED(macro)                                            \
  !LIBC_NAMESPACE::internal::same_string(                                      \
      LLVM_LIBC_IS_DEFINED__EVAL_AND_STRINGIZE(macro), #macro)
#define LLVM_LIBC_IS_DEFINED__EVAL_AND_STRINGIZE(s) #s

#endif // LLVM_LIBC_SRC___SUPPORT_COMMON_H
