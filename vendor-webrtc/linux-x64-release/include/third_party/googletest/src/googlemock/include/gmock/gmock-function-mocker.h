// Copyright 2007, Google Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google Inc. nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// Google Mock - a framework for writing C++ mock classes.
//
// This file implements MOCK_METHOD.

// IWYU pragma: private, include "gmock/gmock.h"
// IWYU pragma: friend gmock/.*

#ifndef GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_FUNCTION_MOCKER_H_
#define GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_FUNCTION_MOCKER_H_

#include <cstddef>
#include <type_traits>  // IWYU pragma: keep
#include <utility>      // IWYU pragma: keep

#include "gmock/gmock-spec-builders.h"
#include "gmock/internal/gmock-internal-utils.h"
#include "gmock/internal/gmock-pp.h"

namespace testing {
namespace internal {
template <typename T>
using identity_t = T;

template <typename Pattern>
struct ThisRefAdjuster {
  template <typename T>
  using AdjustT = typename std::conditional<
      std::is_const<typename std::remove_reference<Pattern>::type>::value,
      typename std::conditional<std::is_lvalue_reference<Pattern>::value,
                                const T&, const T&&>::type,
      typename std::conditional<std::is_lvalue_reference<Pattern>::value, T&,
                                T&&>::type>::type;

  template <typename MockType>
  static AdjustT<MockType> Adjust(const MockType& mock) {
    return static_cast<AdjustT<MockType>>(const_cast<MockType&>(mock));
  }
};

constexpr bool PrefixOf(const char* a, const char* b) {
  return *a == 0 || (*a == *b && internal::PrefixOf(a + 1, b + 1));
}

template <size_t N, size_t M>
constexpr bool StartsWith(const char (&prefix)[N], const char (&str)[M]) {
  return N <= M && internal::PrefixOf(prefix, str);
}

template <size_t N, size_t M>
constexpr bool EndsWith(const char (&suffix)[N], const char (&str)[M]) {
  return N <= M && internal::PrefixOf(suffix, str + M - N);
}

template <size_t N, size_t M>
constexpr bool Equals(const char (&a)[N], const char (&b)[M]) {
  return N == M && internal::PrefixOf(a, b);
}

template <size_t N>
constexpr bool ValidateSpec(const char (&spec)[N]) {
  return internal::Equals("const", spec) ||
         internal::Equals("override", spec) ||
         internal::Equals("final", spec) ||
         internal::Equals("noexcept", spec) ||
         (internal::StartsWith("noexcept(", spec) &&
          internal::EndsWith(")", spec)) ||
         internal::Equals("ref(&)", spec) ||
         internal::Equals("ref(&&)", spec) ||
         (internal::StartsWith("Calltype(", spec) &&
          internal::EndsWith(")", spec));
}

}  // namespace internal

// The style guide prohibits "using" statements in a namespace scope
// inside a header file.  However, the FunctionMocker class template
// is meant to be defined in the ::testing namespace.  The following
// line is just a trick for working around a bug in MSVC 8.0, which
// cannot handle it if we define FunctionMocker in ::testing.
using internal::FunctionMocker;
}  // namespace testing

#define MOCK_METHOD(...)                                               \
  GMOCK_INTERNAL_WARNING_PUSH()                                        \
  GMOCK_INTERNAL_WARNING_CLANG(ignored, "-Wunused-member-function")    \
  GMOCK_PP_VARIADIC_CALL(GMOCK_INTERNAL_MOCK_METHOD_ARG_, __VA_ARGS__) \
  GMOCK_INTERNAL_WARNING_POP()

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_1(...) \
  GMOCK_INTERNAL_WRONG_ARITY(__VA_ARGS__)

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_2(...) \
  GMOCK_INTERNAL_WRONG_ARITY(__VA_ARGS__)

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_3(_Ret, _MethodName, _Args) \
  GMOCK_INTERNAL_MOCK_METHOD_ARG_4(_Ret, _MethodName, _Args, ())

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_4(_Ret, _MethodName, _Args, _Spec)  \
  GMOCK_INTERNAL_ASSERT_PARENTHESIS(_Args);                                \
  GMOCK_INTERNAL_ASSERT_PARENTHESIS(_Spec);                                \
  GMOCK_INTERNAL_ASSERT_VALID_SIGNATURE(                                   \
      GMOCK_PP_NARG0 _Args, GMOCK_INTERNAL_SIGNATURE(_Ret, _Args));        \
  GMOCK_INTERNAL_ASSERT_VALID_SPEC(_Spec)                                  \
  GMOCK_INTERNAL_MOCK_METHOD_IMPL(                                         \
      GMOCK_PP_NARG0 _Args, _MethodName, GMOCK_INTERNAL_HAS_CONST(_Spec),  \
      GMOCK_INTERNAL_HAS_OVERRIDE(_Spec), GMOCK_INTERNAL_HAS_FINAL(_Spec), \
      GMOCK_INTERNAL_GET_NOEXCEPT_SPEC(_Spec),                             \
      GMOCK_INTERNAL_GET_CALLTYPE_SPEC(_Spec),                             \
      GMOCK_INTERNAL_GET_REF_SPEC(_Spec),                                  \
      (GMOCK_INTERNAL_SIGNATURE(_Ret, _Args)))

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_5(...) \
  GMOCK_INTERNAL_WRONG_ARITY(__VA_ARGS__)

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_6(...) \
  GMOCK_INTERNAL_WRONG_ARITY(__VA_ARGS__)

#define GMOCK_INTERNAL_MOCK_METHOD_ARG_7(...) \
  GMOCK_INTERNAL_WRONG_ARITY(__VA_ARGS__)

#define GMOCK_INTERNAL_WRONG_ARITY(...)                                      \
  static_assert(                                                             \
      false,                                                                 \
      "MOCK_METHOD must be called with 3 or 4 arguments. _Ret, "             \
      "_MethodName, _Args and optionally _Spec. _Args and _Spec must be "    \
      "enclosed in parentheses. If _Ret is a type with unprotected commas, " \
      "it must also be enclosed in parentheses.")

#define GMOCK_INTERNAL_ASSERT_PARENTHESIS(_Tuple) \
  static_assert(                                  \
      GMOCK_PP_IS_ENCLOSED_PARENS(_Tuple),        \
      GMOCK_PP_STRINGIZE(_Tuple) " should be enclosed in parentheses.")

#define GMOCK_INTERNAL_ASSERT_VALID_SIGNATURE(_N, ...)                 \
  static_assert(                                                       \
      std::is_function<__VA_ARGS__>::value,                            \
      "Signature must be a function type, maybe return type contains " \
      "unprotected comma.");                                           \
  static_assert(                                                       \
      ::testing::tuple_size<typename ::testing::internal::Function<    \
              __VA_ARGS__>::ArgumentTuple>::value == _N,               \
      "This method does not take " GMOCK_PP_STRINGIZE(                 \
          _N) " arguments. Parenthesize all types with unprotected commas.")

#define GMOCK_INTERNAL_ASSERT_VALID_SPEC(_Spec) \
  GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_ASSERT_VALID_SPEC_ELEMENT, ~, _Spec)

#define GMOCK_INTERNAL_MOCK_METHOD_IMPL(_N, _MethodName, _Constness,           \
                                        _Override, _Final, _NoexceptSpec,      \
                                        _CallType, _RefSpec, _Signature)       \
  typename ::testing::internal::Function<GMOCK_PP_REMOVE_PARENS(               \
      _Signature)>::Result                                                     \
  GMOCK_INTERNAL_EXPAND(_CallType)                                             \
      _MethodName(GMOCK_PP_REPEAT(GMOCK_INTERNAL_PARAMETER, _Signature, _N))   \
          GMOCK_PP_IF(_Constness, const, )                                     \
              _RefSpec _NoexceptSpec GMOCK_PP_IF(_Override, override, )        \
                  GMOCK_PP_IF(_Final, final, ) {                               \
    GMOCK_MOCKER_(_N, _Constness, _MethodName)                                 \
        .SetOwnerAndName(this, #_MethodName);                                  \
    return GMOCK_MOCKER_(_N, _Constness, _MethodName)                          \
        .Invoke(GMOCK_PP_REPEAT(GMOCK_INTERNAL_FORWARD_ARG, _Signature, _N));  \
  }                                                                            \
  ::testing::MockSpec<GMOCK_PP_REMOVE_PARENS(_Signature)> gmock_##_MethodName( \
      GMOCK_PP_REPEAT(GMOCK_INTERNAL_MATCHER_PARAMETER, _Signature, _N))       \
      GMOCK_PP_IF(_Constness, const, ) _RefSpec {                              \
    GMOCK_MOCKER_(_N, _Constness, _MethodName).RegisterOwner(this);            \
    return GMOCK_MOCKER_(_N, _Constness, _MethodName)                          \
        .With(GMOCK_PP_REPEAT(GMOCK_INTERNAL_MATCHER_ARGUMENT, , _N));         \
  }                                                                            \
  ::testing::MockSpec<GMOCK_PP_REMOVE_PARENS(_Signature)> gmock_##_MethodName( \
      const ::testing::internal::WithoutMatchers&,                             \
      GMOCK_PP_IF(_Constness, const, )::testing::internal::Function<           \
          GMOCK_PP_REMOVE_PARENS(_Signature)>*) const _RefSpec _NoexceptSpec { \
    return ::testing::internal::ThisRefAdjuster<GMOCK_PP_IF(                   \
        _Constness, const, ) int _RefSpec>::Adjust(*this)                      \
        .gmock_##_MethodName(GMOCK_PP_REPEAT(                                  \
            GMOCK_INTERNAL_A_MATCHER_ARGUMENT, _Signature, _N));               \
  }                                                                            \
  mutable ::testing::FunctionMocker<GMOCK_PP_REMOVE_PARENS(_Signature)>        \
  GMOCK_MOCKER_(_N, _Constness, _MethodName)

#define GMOCK_INTERNAL_EXPAND(...) __VA_ARGS__

// Valid modifiers.
#define GMOCK_INTERNAL_HAS_CONST(_Tuple) \
  GMOCK_PP_HAS_COMMA(GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_DETECT_CONST, ~, _Tuple))

#define GMOCK_INTERNAL_HAS_OVERRIDE(_Tuple) \
  GMOCK_PP_HAS_COMMA(                       \
      GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_DETECT_OVERRIDE, ~, _Tuple))

#define GMOCK_INTERNAL_HAS_FINAL(_Tuple) \
  GMOCK_PP_HAS_COMMA(GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_DETECT_FINAL, ~, _Tuple))

#define GMOCK_INTERNAL_GET_NOEXCEPT_SPEC(_Tuple) \
  GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_NOEXCEPT_SPEC_IF_NOEXCEPT, ~, _Tuple)

#define GMOCK_INTERNAL_NOEXCEPT_SPEC_IF_NOEXCEPT(_i, _, _elem)          \
  GMOCK_PP_IF(                                                          \
      GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_NOEXCEPT(_i, _, _elem)), \
      _elem, )

#define GMOCK_INTERNAL_GET_CALLTYPE_SPEC(_Tuple) \
  GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_CALLTYPE_SPEC_IF_CALLTYPE, ~, _Tuple)

#define GMOCK_INTERNAL_CALLTYPE_SPEC_IF_CALLTYPE(_i, _, _elem)          \
  GMOCK_PP_IF(                                                          \
      GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_CALLTYPE(_i, _, _elem)), \
      GMOCK_PP_CAT(GMOCK_INTERNAL_UNPACK_, _elem), )

#define GMOCK_INTERNAL_GET_REF_SPEC(_Tuple) \
  GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_REF_SPEC_IF_REF, ~, _Tuple)

#define GMOCK_INTERNAL_REF_SPEC_IF_REF(_i, _, _elem)                       \
  GMOCK_PP_IF(GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_REF(_i, _, _elem)), \
              GMOCK_PP_CAT(GMOCK_INTERNAL_UNPACK_, _elem), )

#ifdef GMOCK_INTERNAL_STRICT_SPEC_ASSERT
#define GMOCK_INTERNAL_ASSERT_VALID_SPEC_ELEMENT(_i, _, _elem) \
  static_assert(                                                     \
      ::testing::internal::ValidateSpec(GMOCK_PP_STRINGIZE(_elem)),  \
      "Token \'" GMOCK_PP_STRINGIZE(                                 \
          _elem) "\' cannot be recognized as a valid specification " \
                 "modifier. Is a ',' missing?");
#else
#define GMOCK_INTERNAL_ASSERT_VALID_SPEC_ELEMENT(_i, _, _elem)                 \
  static_assert(                                                               \
      (GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_CONST(_i, _, _elem)) +         \
       GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_OVERRIDE(_i, _, _elem)) +      \
       GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_FINAL(_i, _, _elem)) +         \
       GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_NOEXCEPT(_i, _, _elem)) +      \
       GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_REF(_i, _, _elem)) +           \
       GMOCK_PP_HAS_COMMA(GMOCK_INTERNAL_DETECT_CALLTYPE(_i, _, _elem))) == 1, \
      GMOCK_PP_STRINGIZE(                                                      \
          _elem) " cannot be recognized as a valid specification modifier.");
#endif  // GMOCK_INTERNAL_STRICT_SPEC_ASSERT

// Modifiers implementation.
#define GMOCK_INTERNAL_DETECT_CONST(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_CONST_I_, _elem)

#define GMOCK_INTERNAL_DETECT_CONST_I_const ,

#define GMOCK_INTERNAL_DETECT_OVERRIDE(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_OVERRIDE_I_, _elem)

#define GMOCK_INTERNAL_DETECT_OVERRIDE_I_override ,

#define GMOCK_INTERNAL_DETECT_FINAL(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_FINAL_I_, _elem)

#define GMOCK_INTERNAL_DETECT_FINAL_I_final ,

#define GMOCK_INTERNAL_DETECT_NOEXCEPT(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_NOEXCEPT_I_, _elem)

#define GMOCK_INTERNAL_DETECT_NOEXCEPT_I_noexcept ,

#define GMOCK_INTERNAL_DETECT_REF(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_REF_I_, _elem)

#define GMOCK_INTERNAL_DETECT_REF_I_ref ,

#define GMOCK_INTERNAL_UNPACK_ref(x) x

#define GMOCK_INTERNAL_DETECT_CALLTYPE(_i, _, _elem) \
  GMOCK_PP_CAT(GMOCK_INTERNAL_DETECT_CALLTYPE_I_, _elem)

#define GMOCK_INTERNAL_DETECT_CALLTYPE_I_Calltype ,

#define GMOCK_INTERNAL_UNPACK_Calltype(...) __VA_ARGS__

// Note: The use of `identity_t` here allows _Ret to represent return types that
// would normally need to be specified in a different way. For example, a method
// returning a function pointer must be written as
//
// fn_ptr_return_t (*method(method_args_t...))(fn_ptr_args_t...)
//
// But we only support placing the return type at the beginning. To handle this,
// we wrap all calls in identity_t, so that a declaration will be expanded to
//
// identity_t<fn_ptr_return_t (*)(fn_ptr_args_t...)> method(method_args_t...)
//
// This allows us to work around the syntactic oddities of function/method
// types.
#define GMOCK_INTERNAL_SIGNATURE(_Ret, _Args)                                 \
  ::testing::internal::identity_t<GMOCK_PP_IF(GMOCK_PP_IS_BEGIN_PARENS(_Ret), \
                                              GMOCK_PP_REMOVE_PARENS,         \
                                              GMOCK_PP_IDENTITY)(_Ret)>(      \
      GMOCK_PP_FOR_EACH(GMOCK_INTERNAL_GET_TYPE, _, _Args))

#define GMOCK_INTERNAL_GET_TYPE(_i, _, _elem)                          \
  GMOCK_PP_COMMA_IF(_i)                                                \
  GMOCK_PP_IF(GMOCK_PP_IS_BEGIN_PARENS(_elem), GMOCK_PP_REMOVE_PARENS, \
              GMOCK_PP_IDENTITY)                                       \
  (_elem)

#define GMOCK_INTERNAL_PARAMETER(_i, _Signature, _)            \
  GMOCK_PP_COMMA_IF(_i)                                        \
  GMOCK_INTERNAL_ARG_O(_i, GMOCK_PP_REMOVE_PARENS(_Signature)) \
  gmock_a##_i

#define GMOCK_INTERNAL_FORWARD_ARG(_i, _Signature, _) \
  GMOCK_PP_COMMA_IF(_i)                               \
  ::std::forward<GMOCK_INTERNAL_ARG_O(                \
      _i, GMOCK_PP_REMOVE_PARENS(_Signature))>(gmock_a##_i)

#define GMOCK_INTERNAL_MATCHER_PARAMETER(_i, _Signature, _)        \
  GMOCK_PP_COMMA_IF(_i)                                            \
  GMOCK_INTERNAL_MATCHER_O(_i, GMOCK_PP_REMOVE_PARENS(_Signature)) \
  gmock_a##_i

#define GMOCK_INTERNAL_MATCHER_ARGUMENT(_i, _1, _2) \
  GMOCK_PP_COMMA_IF(_i)                             \
  gmock_a##_i

#define GMOCK_INTERNAL_A_MATCHER_ARGUMENT(_i, _Signature, _) \
  GMOCK_PP_COMMA_IF(_i)                                      \
  ::testing::A<GMOCK_INTERNAL_ARG_O(_i, GMOCK_PP_REMOVE_PARENS(_Signature))>()

#define GMOCK_INTERNAL_ARG_O(_i, ...) \
  typename ::testing::internal::Function<__VA_ARGS__>::template Arg<_i>::type

#define GMOCK_INTERNAL_MATCHER_O(_i, ...)                          \
  const ::testing::Matcher<typename ::testing::internal::Function< \
      __VA_ARGS__>::template Arg<_i>::type>&

#define MOCK_METHOD0(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 0, __VA_ARGS__)
#define MOCK_METHOD1(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 1, __VA_ARGS__)
#define MOCK_METHOD2(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 2, __VA_ARGS__)
#define MOCK_METHOD3(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 3, __VA_ARGS__)
#define MOCK_METHOD4(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 4, __VA_ARGS__)
#define MOCK_METHOD5(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 5, __VA_ARGS__)
#define MOCK_METHOD6(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 6, __VA_ARGS__)
#define MOCK_METHOD7(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 7, __VA_ARGS__)
#define MOCK_METHOD8(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 8, __VA_ARGS__)
#define MOCK_METHOD9(m, ...) GMOCK_INTERNAL_MOCK_METHODN(, , m, 9, __VA_ARGS__)
#define MOCK_METHOD10(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, , m, 10, __VA_ARGS__)

#define MOCK_CONST_METHOD0(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 0, __VA_ARGS__)
#define MOCK_CONST_METHOD1(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 1, __VA_ARGS__)
#define MOCK_CONST_METHOD2(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 2, __VA_ARGS__)
#define MOCK_CONST_METHOD3(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 3, __VA_ARGS__)
#define MOCK_CONST_METHOD4(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 4, __VA_ARGS__)
#define MOCK_CONST_METHOD5(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 5, __VA_ARGS__)
#define MOCK_CONST_METHOD6(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 6, __VA_ARGS__)
#define MOCK_CONST_METHOD7(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 7, __VA_ARGS__)
#define MOCK_CONST_METHOD8(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 8, __VA_ARGS__)
#define MOCK_CONST_METHOD9(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 9, __VA_ARGS__)
#define MOCK_CONST_METHOD10(m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, , m, 10, __VA_ARGS__)

#define MOCK_METHOD0_T(m, ...) MOCK_METHOD0(m, __VA_ARGS__)
#define MOCK_METHOD1_T(m, ...) MOCK_METHOD1(m, __VA_ARGS__)
#define MOCK_METHOD2_T(m, ...) MOCK_METHOD2(m, __VA_ARGS__)
#define MOCK_METHOD3_T(m, ...) MOCK_METHOD3(m, __VA_ARGS__)
#define MOCK_METHOD4_T(m, ...) MOCK_METHOD4(m, __VA_ARGS__)
#define MOCK_METHOD5_T(m, ...) MOCK_METHOD5(m, __VA_ARGS__)
#define MOCK_METHOD6_T(m, ...) MOCK_METHOD6(m, __VA_ARGS__)
#define MOCK_METHOD7_T(m, ...) MOCK_METHOD7(m, __VA_ARGS__)
#define MOCK_METHOD8_T(m, ...) MOCK_METHOD8(m, __VA_ARGS__)
#define MOCK_METHOD9_T(m, ...) MOCK_METHOD9(m, __VA_ARGS__)
#define MOCK_METHOD10_T(m, ...) MOCK_METHOD10(m, __VA_ARGS__)

#define MOCK_CONST_METHOD0_T(m, ...) MOCK_CONST_METHOD0(m, __VA_ARGS__)
#define MOCK_CONST_METHOD1_T(m, ...) MOCK_CONST_METHOD1(m, __VA_ARGS__)
#define MOCK_CONST_METHOD2_T(m, ...) MOCK_CONST_METHOD2(m, __VA_ARGS__)
#define MOCK_CONST_METHOD3_T(m, ...) MOCK_CONST_METHOD3(m, __VA_ARGS__)
#define MOCK_CONST_METHOD4_T(m, ...) MOCK_CONST_METHOD4(m, __VA_ARGS__)
#define MOCK_CONST_METHOD5_T(m, ...) MOCK_CONST_METHOD5(m, __VA_ARGS__)
#define MOCK_CONST_METHOD6_T(m, ...) MOCK_CONST_METHOD6(m, __VA_ARGS__)
#define MOCK_CONST_METHOD7_T(m, ...) MOCK_CONST_METHOD7(m, __VA_ARGS__)
#define MOCK_CONST_METHOD8_T(m, ...) MOCK_CONST_METHOD8(m, __VA_ARGS__)
#define MOCK_CONST_METHOD9_T(m, ...) MOCK_CONST_METHOD9(m, __VA_ARGS__)
#define MOCK_CONST_METHOD10_T(m, ...) MOCK_CONST_METHOD10(m, __VA_ARGS__)

#define MOCK_METHOD0_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 0, __VA_ARGS__)
#define MOCK_METHOD1_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 1, __VA_ARGS__)
#define MOCK_METHOD2_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 2, __VA_ARGS__)
#define MOCK_METHOD3_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 3, __VA_ARGS__)
#define MOCK_METHOD4_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 4, __VA_ARGS__)
#define MOCK_METHOD5_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 5, __VA_ARGS__)
#define MOCK_METHOD6_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 6, __VA_ARGS__)
#define MOCK_METHOD7_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 7, __VA_ARGS__)
#define MOCK_METHOD8_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 8, __VA_ARGS__)
#define MOCK_METHOD9_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 9, __VA_ARGS__)
#define MOCK_METHOD10_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(, ct, m, 10, __VA_ARGS__)

#define MOCK_CONST_METHOD0_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 0, __VA_ARGS__)
#define MOCK_CONST_METHOD1_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 1, __VA_ARGS__)
#define MOCK_CONST_METHOD2_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 2, __VA_ARGS__)
#define MOCK_CONST_METHOD3_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 3, __VA_ARGS__)
#define MOCK_CONST_METHOD4_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 4, __VA_ARGS__)
#define MOCK_CONST_METHOD5_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 5, __VA_ARGS__)
#define MOCK_CONST_METHOD6_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 6, __VA_ARGS__)
#define MOCK_CONST_METHOD7_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 7, __VA_ARGS__)
#define MOCK_CONST_METHOD8_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 8, __VA_ARGS__)
#define MOCK_CONST_METHOD9_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 9, __VA_ARGS__)
#define MOCK_CONST_METHOD10_WITH_CALLTYPE(ct, m, ...) \
  GMOCK_INTERNAL_MOCK_METHODN(const, ct, m, 10, __VA_ARGS__)

#define MOCK_METHOD0_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD0_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD1_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD1_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD2_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD2_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD3_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD3_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD4_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD4_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD5_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD5_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD6_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD6_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD7_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD7_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD8_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD8_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD9_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD9_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_METHOD10_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_METHOD10_WITH_CALLTYPE(ct, m, __VA_ARGS__)

#define MOCK_CONST_METHOD0_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD0_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD1_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD1_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD2_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD2_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD3_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD3_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD4_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD4_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD5_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD5_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD6_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD6_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD7_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD7_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD8_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD8_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD9_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD9_WITH_CALLTYPE(ct, m, __VA_ARGS__)
#define MOCK_CONST_METHOD10_T_WITH_CALLTYPE(ct, m, ...) \
  MOCK_CONST_METHOD10_WITH_CALLTYPE(ct, m, __VA_ARGS__)

#define GMOCK_INTERNAL_MOCK_METHODN(constness, ct, Method, args_num, ...) \
  GMOCK_INTERNAL_ASSERT_VALID_SIGNATURE(                                  \
      args_num, ::testing::internal::identity_t<__VA_ARGS__>);            \
  GMOCK_INTERNAL_MOCK_METHOD_IMPL(                                        \
      args_num, Method, GMOCK_PP_NARG0(constness), 0, 0, , ct, ,          \
      (::testing::internal::identity_t<__VA_ARGS__>))

#define GMOCK_MOCKER_(arity, constness, Method) \
  GTEST_CONCAT_TOKEN_(gmock##constness##arity##_##Method##_, __LINE__)

#endif  // GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_FUNCTION_MOCKER_H_
