// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CHECK_OP_H_
#define BASE_CHECK_OP_H_

#include <cstddef>
#include <string>
#include <string_view>
#include <type_traits>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/types/is_arc_pointer.h"
#include "base/types/supports_ostream_operator.h"
#include "base/types/supports_to_string.h"

// This header defines the (DP)CHECK_EQ etc. macros.
//
// (DP)CHECK_EQ(x, y) is similar to (DP)CHECK(x == y) but will also log the
// values of x and y if the condition doesn't hold. This works for basic types
// and types with an operator<< or .ToString() method.
//
// The operands are evaluated exactly once, and even in build modes where e.g.
// DCHECK is disabled, the operands and their stringification methods are still
// referenced to avoid warnings about unused variables or functions.
//
// Like (D)CHECK (D)CHECK_EQ also supports an optional base::NotFatalUntil
// parameter. See base/check.h.
//
// To support the stringification of the check operands, this header is
// *significantly* larger than base/check.h, so it should be avoided in common
// headers.
//
// This header also provides the (DP)CHECK macros (by including check.h), so if
// you use e.g. both CHECK_EQ and CHECK, including this header is enough. If you
// only use CHECK however, please include the smaller check.h instead.

namespace base {
template <class Char>
class basic_cstring_view;
}

namespace logging {

// Functions for turning check operand values into NUL-terminated C strings.
// Caller takes ownership of the result and must release it with `free`.
// This would normally be defined by <ostream>, but this header tries to avoid
// including <ostream> to reduce compile-time. See https://crrev.com/c/2128112.
BASE_EXPORT char* CheckOpValueStr(int v);
BASE_EXPORT char* CheckOpValueStr(unsigned v);
BASE_EXPORT char* CheckOpValueStr(long v);
BASE_EXPORT char* CheckOpValueStr(unsigned long v);
BASE_EXPORT char* CheckOpValueStr(long long v);
BASE_EXPORT char* CheckOpValueStr(unsigned long long v);
BASE_EXPORT char* CheckOpValueStr(const void* v);
BASE_EXPORT char* CheckOpValueStr(std::nullptr_t v);
BASE_EXPORT char* CheckOpValueStr(double v);
// Although the standard defines operator<< for std::string and std::string_view
// in their respective headers, libc++ requires <ostream> for them. See
// https://github.com/llvm/llvm-project/issues/61070. So we define non-<ostream>
// versions here too.
BASE_EXPORT char* CheckOpValueStr(const std::string& v);
BASE_EXPORT char* CheckOpValueStr(std::string_view v);
BASE_EXPORT char* CheckOpValueStr(base::basic_cstring_view<char> v);

// Convert a streamable value to string out-of-line to avoid <sstream>.
BASE_EXPORT char* StreamValToStr(const void* v,
                                 void (*stream_func)(std::ostream&,
                                                     const void*));

#ifdef __has_builtin
#define SUPPORTS_BUILTIN_ADDRESSOF (__has_builtin(__builtin_addressof))
#else
#define SUPPORTS_BUILTIN_ADDRESSOF 0
#endif

template <typename T>
  requires(base::internal::SupportsOstreamOperator<const T&> &&
           !std::is_function_v<T> && !std::is_pointer_v<T>)
inline char* CheckOpValueStr(const T& v) {
  auto f = [](std::ostream& s, const void* p) {
    s << *reinterpret_cast<const T*>(p);
  };

  // operator& might be overloaded, so do the std::addressof dance.
  // __builtin_addressof is preferred since it also handles Obj-C ARC pointers.
  // Some casting is still needed, because T might be volatile.
#if SUPPORTS_BUILTIN_ADDRESSOF
  const void* vp = const_cast<const void*>(
      reinterpret_cast<const volatile void*>(__builtin_addressof(v)));
#else
  const void* vp = reinterpret_cast<const void*>(
      const_cast<const char*>(&reinterpret_cast<const volatile char&>(v)));
#endif
  return StreamValToStr(vp, f);
}

#undef SUPPORTS_BUILTIN_ADDRESSOF

// Even if the pointer type supports operator<<, print the pointer by
// value. This is especially useful for `char*` and `unsigned char*`,
// which would otherwise print the pointed-to data.
template <typename T>
  requires(std::is_pointer_v<T> &&
           !std::is_function_v<std::remove_pointer_t<T>>)
inline char* CheckOpValueStr(const T& v) {
#if defined(__OBJC__)
  const void* vp;
  if constexpr (base::IsArcPointer<T>) {
    vp = const_cast<const void*>((__bridge const volatile void*)(v));
  } else {
    vp = const_cast<const void*>(reinterpret_cast<const volatile void*>(v));
  }
#else
  const void* vp =
      const_cast<const void*>(reinterpret_cast<const volatile void*>(v));
#endif
  return CheckOpValueStr(vp);
}

// Overload for types that have no operator<< but do have .ToString() defined.
template <typename T>
  requires(!base::internal::SupportsOstreamOperator<const T&> &&
           base::internal::SupportsToString<const T&>)
inline char* CheckOpValueStr(const T& v) {
  // .ToString() may not return a std::string, e.g. blink::WTF::String.
  return CheckOpValueStr(v.ToString());
}

// Provide an overload for functions and function pointers. Function pointers
// don't implicitly convert to void* but do implicitly convert to bool, so
// without this function pointers are always printed as 1 or 0. (MSVC isn't
// standards-conforming here and converts function pointers to regular
// pointers, so this is a no-op for MSVC.)
template <typename T>
  requires(std::is_function_v<std::remove_pointer_t<T>>)
inline char* CheckOpValueStr(const T& v) {
  return CheckOpValueStr(reinterpret_cast<const void*>(v));
}

// We need overloads for enums that don't support operator<<.
// (i.e. scoped enums where no operator<< overload was declared).
template <typename T>
  requires(!base::internal::SupportsOstreamOperator<const T&> &&
           std::is_enum_v<T>)
inline char* CheckOpValueStr(const T& v) {
  return CheckOpValueStr(static_cast<std::underlying_type_t<T>>(v));
}

// Takes ownership of `v1_str` and `v2_str`, destroying them with free(). For
// use with CheckOpValueStr() which allocates these strings using strdup().
// Returns allocated string (with strdup) for passing into
// ::logging::CheckError::(D)CheckOp methods.
BASE_EXPORT char* CreateCheckOpLogMessageString(const char* expr_str,
                                                char* v1_str,
                                                char* v2_str);

// Helper macro for binary operators.
// The 'switch' is used to prevent the 'else' from being ambiguous when the
// macro is used in an 'if' clause such as:
// if (a == 1)
//   CHECK_EQ(2, a);
#define CHECK_OP_FUNCTION_IMPL(check_failure_type, check_log_message_function, \
                               name, op, val1, val2, ...)                      \
  switch (0)                                                                   \
  case 0:                                                                      \
  default:                                                                     \
    if (::logging::LogMessage *const message_on_fail =                         \
            ::logging::Check##name##Impl(                                      \
                (val1), (val2),                                                \
                [](char* str1, char* str2) {                                   \
                  return check_log_message_function(                           \
                      ::logging::CreateCheckOpLogMessageString(                \
                          #val1 " " #op " " #val2, str1, str2) __VA_OPT__(, )  \
                          __VA_ARGS__);                                        \
                });                                                            \
        !message_on_fail)                                                      \
      ;                                                                        \
    else                                                                       \
      (check_failure_type)(message_on_fail)

#if !CHECK_WILL_STREAM()

// Discard log strings to reduce code bloat.
#define CHECK_OP_INTERNAL_IMPL(name, op, val1, val2) CHECK((val1)op(val2))

#else

#define CHECK_OP_INTERNAL_IMPL(name, op, val1, val2)                       \
  CHECK_OP_FUNCTION_IMPL(::logging::CheckNoreturnError,                    \
                         ::logging::CheckNoreturnError::CheckOp, name, op, \
                         val1, val2)

#endif

#define CHECK_OP(name, op, val1, val2, ...)                                \
  BASE_IF(BASE_IS_EMPTY(__VA_ARGS__),                                      \
          CHECK_OP_INTERNAL_IMPL(name, op, val1, val2),                    \
          CHECK_OP_FUNCTION_IMPL(::logging::CheckError,                    \
                                 ::logging::CheckError::CheckOp, name, op, \
                                 val1, val2, __VA_ARGS__))

// The second overload avoids address-taking of static members for
// fundamental types.
#define DEFINE_CHECK_OP_IMPL(name, op)                                         \
  template <typename T, typename U, typename F>                                \
    requires(!std::is_fundamental_v<T> || !std::is_fundamental_v<U>)           \
  constexpr ::logging::LogMessage* Check##name##Impl(const T& v1, const U& v2, \
                                                     F on_failure) {           \
    if (ANALYZER_ASSUME_TRUE(v1 op v2)) [[likely]]                             \
      return nullptr;                                                          \
    return on_failure(CheckOpValueStr(v1), CheckOpValueStr(v2));               \
  }                                                                            \
  template <typename T, typename U, typename F>                                \
    requires(std::is_fundamental_v<T> && std::is_fundamental_v<U>)             \
  constexpr ::logging::LogMessage* Check##name##Impl(T v1, U v2,               \
                                                     F on_failure) {           \
    if (ANALYZER_ASSUME_TRUE(v1 op v2)) [[likely]]                             \
      return nullptr;                                                          \
    return on_failure(CheckOpValueStr(v1), CheckOpValueStr(v2));               \
  }

// clang-format off
DEFINE_CHECK_OP_IMPL(EQ, ==)
DEFINE_CHECK_OP_IMPL(NE, !=)
DEFINE_CHECK_OP_IMPL(LE, <=)
DEFINE_CHECK_OP_IMPL(LT, < )
DEFINE_CHECK_OP_IMPL(GE, >=)
DEFINE_CHECK_OP_IMPL(GT, > )
#undef DEFINE_CHECK_OP_IMPL
#define CHECK_EQ(val1, val2, ...) \
  CHECK_OP(EQ, ==, val1, val2 __VA_OPT__(, ) __VA_ARGS__)
#define CHECK_NE(val1, val2, ...) \
  CHECK_OP(NE, !=, val1, val2 __VA_OPT__(, ) __VA_ARGS__)
#define CHECK_LE(val1, val2, ...) \
  CHECK_OP(LE, <=, val1, val2 __VA_OPT__(, ) __VA_ARGS__)
#define CHECK_LT(val1, val2, ...) \
  CHECK_OP(LT, < , val1, val2 __VA_OPT__(, ) __VA_ARGS__)
#define CHECK_GE(val1, val2, ...) \
  CHECK_OP(GE, >=, val1, val2 __VA_OPT__(, ) __VA_ARGS__)
#define CHECK_GT(val1, val2, ...) \
  CHECK_OP(GT, > , val1, val2 __VA_OPT__(, ) __VA_ARGS__)
// clang-format on

#if DCHECK_IS_ON()

#define DCHECK_OP(name, op, val1, val2)                                   \
  CHECK_OP_FUNCTION_IMPL(::logging::CheckError,                           \
                         ::logging::CheckError::DCheckOp, name, op, val1, \
                         val2)

#else

// Don't do any evaluation but still reference the same stuff as when enabled.
#define DCHECK_OP(name, op, val1, val2)                      \
  EAT_CHECK_STREAM_PARAMS((::logging::CheckOpValueStr(val1), \
                           ::logging::CheckOpValueStr(val2), (val1)op(val2)))

#endif

// clang-format off
#define DCHECK_EQ(val1, val2) DCHECK_OP(EQ, ==, val1, val2)
#define DCHECK_NE(val1, val2) DCHECK_OP(NE, !=, val1, val2)
#define DCHECK_LE(val1, val2) DCHECK_OP(LE, <=, val1, val2)
#define DCHECK_LT(val1, val2) DCHECK_OP(LT, < , val1, val2)
#define DCHECK_GE(val1, val2) DCHECK_OP(GE, >=, val1, val2)
#define DCHECK_GT(val1, val2) DCHECK_OP(GT, > , val1, val2)
// clang-format on

#define DUMP_WILL_BE_CHECK_OP(name, op, val1, val2)                          \
  CHECK_OP_FUNCTION_IMPL(::logging::CheckError,                              \
                         ::logging::CheckError::DumpWillBeCheckOp, name, op, \
                         val1, val2)

#define DUMP_WILL_BE_CHECK_EQ(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(EQ, ==, val1, val2)
#define DUMP_WILL_BE_CHECK_NE(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(NE, !=, val1, val2)
#define DUMP_WILL_BE_CHECK_LE(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(LE, <=, val1, val2)
#define DUMP_WILL_BE_CHECK_LT(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(LT, <, val1, val2)
#define DUMP_WILL_BE_CHECK_GE(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(GE, >=, val1, val2)
#define DUMP_WILL_BE_CHECK_GT(val1, val2) \
  DUMP_WILL_BE_CHECK_OP(GT, >, val1, val2)

}  // namespace logging

#endif  // BASE_CHECK_OP_H_
