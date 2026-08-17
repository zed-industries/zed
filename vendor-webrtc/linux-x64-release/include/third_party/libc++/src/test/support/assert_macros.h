//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_SUPPORT_ASSERT_MACROS_H
#define TEST_SUPPORT_ASSERT_MACROS_H

// Contains a set of validation macros.
//
// Note these test were added after C++20 was well supported by the compilers
// used. To make the implementation simple the macros require C++20 or newer.
// It's not expected that existing tests start to use these new macros.
//
// These macros are an alternative to using assert. The differences are:
// - The assert message isn't localized.
// - It's possible to log additional information. This is useful when the
//   function asserting is a helper function. In these cases the assertion
//   failure contains to little information to find the issue. For example, in
//   the format functions, the really useful information is the actual output,
//   the expected output, and the format string used. These macros allow
//   logging additional arguments.

#include "test_macros.h"

#include <cstdio>
#include <cstdlib>

// This function prints the given arguments to standard error.
//
// Keeping this as a separate function is important since it provides a single point for
// downstreams to customize how errors are printed on exotic targets, if needed.
template <class ...Args>
void test_eprintf(char const* fmt, Args const& ...args) {
  std::fprintf(stderr, fmt, args...);
}

void test_log(const char* condition, const char* file, int line, const char* message) {
  const char* msg = condition ? "Assertion failure: " : "Unconditional failure:";
  test_eprintf("%s%s %s %d\n%s", msg, condition, file, line, message);
}

template <class F>
void test_log(const char* condition, const char* file, int line, const F& functor) {
  test_eprintf("Assertion failure: %s %s %d\n", condition, file, line);
  functor();
}

template <class Arg>
[[noreturn]] void test_fail(const char* file, int line, const Arg& arg) {
  test_log("", file, line, arg);
  std::abort();
}

template <class Arg>
void test_require(bool condition, const char* condition_str, const char* file, int line, const Arg& arg) {
  if (condition)
    return;

  test_log(condition_str, file, line, arg);
  std::abort();
}

// assert(false) replacement
// The ARG is either a
// - c-string or std::string, in which case the string is printed to stderr,
// - an invocable object, which will be invoked.
#define TEST_FAIL(ARG) ::test_fail(__FILE__, __LINE__, ARG)

// assert replacement.
// ARG is the same as for TEST_FAIL
#define TEST_REQUIRE(CONDITION, ARG) ::test_require(CONDITION, #CONDITION, __FILE__, __LINE__, ARG)

// LIBCPP_ASSERT replacement
//
// This requirement is only tested when the test suite is used for libc++.
// This allows checking libc++ specific requirements, for example the error
// messages of exceptions.
// ARG is the same as for TEST_FAIL
#if defined(_LIBCPP_VERSION)
#  define TEST_LIBCPP_REQUIRE(CONDITION, ARG) ::test_require(CONDITION, #CONDITION, __FILE__, __LINE__, ARG)
#else
#  define TEST_LIBCPP_REQUIRE(...) /* DO NOTHING */
#endif

// Helper macro to test an expression does not throw any exception.
#ifndef TEST_HAS_NO_EXCEPTIONS
#  define TEST_DOES_NOT_THROW(EXPR)                                                                                    \
    do {                                                                                                               \
      try {                                                                                                            \
        static_cast<void>(EXPR);                                                                                       \
      } catch (...) {                                                                                                  \
        ::test_log(#EXPR, __FILE__, __LINE__, "no exception was expected\n");                                          \
        ::std::abort();                                                                                                \
      }                                                                                                                \
    } while (false) /* */

// Helper macro to test an expression throws an exception of the expected type.
#  define TEST_THROWS_TYPE(TYPE, EXPR)                                                                                 \
    do {                                                                                                               \
      try {                                                                                                            \
        static_cast<void>(EXPR);                                                                                       \
        ::test_log(nullptr,                                                                                            \
                   __FILE__,                                                                                           \
                   __LINE__,                                                                                           \
                   "no exception is thrown while an exception of type " #TYPE " was expected\n");                      \
        ::std::abort();                                                                                                \
      } catch (const TYPE&) {                                                                                          \
        /* DO NOTHING */                                                                                               \
      } catch (...) {                                                                                                  \
        ::test_log(nullptr,                                                                                            \
                   __FILE__,                                                                                           \
                   __LINE__,                                                                                           \
                   "the type of the exception caught differs from the expected type " #TYPE "\n");                     \
        ::std::abort();                                                                                                \
      }                                                                                                                \
    } while (false) /* */

// Helper macro to test an expression throws an exception of the expected type and satisfies a predicate.
//
// In order to log additional information the predicate can use log macros.
// The exception caught is used as argument to the predicate.
#  define TEST_VALIDATE_EXCEPTION(TYPE, PRED, EXPR)                                                                    \
    do {                                                                                                               \
      try {                                                                                                            \
        static_cast<void>(EXPR);                                                                                       \
        ::test_log(nullptr,                                                                                            \
                   __FILE__,                                                                                           \
                   __LINE__,                                                                                           \
                   "no exception is thrown while an exception of type " #TYPE " was expected\n");                      \
        ::std::abort();                                                                                                \
      } catch (const TYPE& EXCEPTION) {                                                                                \
        PRED(EXCEPTION);                                                                                               \
      } catch (...) {                                                                                                  \
        ::test_log(nullptr,                                                                                            \
                   __FILE__,                                                                                           \
                   __LINE__,                                                                                           \
                   "the type of the exception caught differs from the expected type " #TYPE "\n");                     \
        ::std::abort();                                                                                                \
      }                                                                                                                \
    } while (false)                    /* */

#else                                  // TEST_HAS_NO_EXCEPTIONS
#  define TEST_DOES_NOT_THROW(EXPR) static_cast<void>(EXPR);
#  define TEST_THROWS_TYPE(...)        /* DO NOTHING */
#  define TEST_VALIDATE_EXCEPTION(...) /* DO NOTHING */
#endif                                 // TEST_HAS_NO_EXCEPTIONS

#endif // TEST_SUPPORT_ASSERT_MACROS_H
