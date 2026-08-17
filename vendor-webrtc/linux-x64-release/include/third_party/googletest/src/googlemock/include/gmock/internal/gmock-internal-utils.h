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
// This file defines some utilities useful for implementing Google
// Mock.  They are subject to change without notice, so please DO NOT
// USE THEM IN USER CODE.

// IWYU pragma: private, include "gmock/gmock.h"
// IWYU pragma: friend gmock/.*

#ifndef GOOGLEMOCK_INCLUDE_GMOCK_INTERNAL_GMOCK_INTERNAL_UTILS_H_
#define GOOGLEMOCK_INCLUDE_GMOCK_INTERNAL_GMOCK_INTERNAL_UTILS_H_

#include <stdio.h>

#include <ostream>  // NOLINT
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "gmock/internal/gmock-port.h"
#include "gtest/gtest.h"

namespace testing {

template <typename>
class Matcher;

namespace internal {

// Silence MSVC C4100 (unreferenced formal parameter) and
// C4805('==': unsafe mix of type 'const int' and type 'const bool')
GTEST_DISABLE_MSC_WARNINGS_PUSH_(4100 4805)

// Joins a vector of strings as if they are fields of a tuple; returns
// the joined string.
GTEST_API_ std::string JoinAsKeyValueTuple(
    const std::vector<const char*>& names, const Strings& values);

// Converts an identifier name to a space-separated list of lower-case
// words.  Each maximum substring of the form [A-Za-z][a-z]*|\d+ is
// treated as one word.  For example, both "FooBar123" and
// "foo_bar_123" are converted to "foo bar 123".
GTEST_API_ std::string ConvertIdentifierNameToWords(const char* id_name);

// GetRawPointer(p) returns the raw pointer underlying p when p is a
// smart pointer, or returns p itself when p is already a raw pointer.
// The following default implementation is for the smart pointer case.
template <typename Pointer>
inline const typename Pointer::element_type* GetRawPointer(const Pointer& p) {
  return p.get();
}
// This overload version is for std::reference_wrapper, which does not work with
// the overload above, as it does not have an `element_type`.
template <typename Element>
inline const Element* GetRawPointer(const std::reference_wrapper<Element>& r) {
  return &r.get();
}

// This overloaded version is for the raw pointer case.
template <typename Element>
inline Element* GetRawPointer(Element* p) {
  return p;
}

// Default definitions for all compilers.
// NOTE: If you implement support for other compilers, make sure to avoid
// unexpected overlaps.
// (e.g., Clang also processes #pragma GCC, and clang-cl also handles _MSC_VER.)
#define GMOCK_INTERNAL_WARNING_PUSH()
#define GMOCK_INTERNAL_WARNING_CLANG(Level, Name)
#define GMOCK_INTERNAL_WARNING_POP()

#if defined(__clang__)
#undef GMOCK_INTERNAL_WARNING_PUSH
#define GMOCK_INTERNAL_WARNING_PUSH() _Pragma("clang diagnostic push")
#undef GMOCK_INTERNAL_WARNING_CLANG
#define GMOCK_INTERNAL_WARNING_CLANG(Level, Warning) \
  _Pragma(GMOCK_PP_INTERNAL_STRINGIZE(clang diagnostic Level Warning))
#undef GMOCK_INTERNAL_WARNING_POP
#define GMOCK_INTERNAL_WARNING_POP() _Pragma("clang diagnostic pop")
#endif

// MSVC treats wchar_t as a native type usually, but treats it as the
// same as unsigned short when the compiler option /Zc:wchar_t- is
// specified.  It defines _NATIVE_WCHAR_T_DEFINED symbol when wchar_t
// is a native type.
#if defined(_MSC_VER) && !defined(_NATIVE_WCHAR_T_DEFINED)
// wchar_t is a typedef.
#else
#define GMOCK_WCHAR_T_IS_NATIVE_ 1
#endif

// In what follows, we use the term "kind" to indicate whether a type
// is bool, an integer type (excluding bool), a floating-point type,
// or none of them.  This categorization is useful for determining
// when a matcher argument type can be safely converted to another
// type in the implementation of SafeMatcherCast.
enum TypeKind { kBool, kInteger, kFloatingPoint, kOther };

// KindOf<T>::value is the kind of type T.
template <typename T>
struct KindOf {
  enum { value = kOther };  // The default kind.
};

// This macro declares that the kind of 'type' is 'kind'.
#define GMOCK_DECLARE_KIND_(type, kind) \
  template <>                           \
  struct KindOf<type> {                 \
    enum { value = kind };              \
  }

GMOCK_DECLARE_KIND_(bool, kBool);

// All standard integer types.
GMOCK_DECLARE_KIND_(char, kInteger);
GMOCK_DECLARE_KIND_(signed char, kInteger);
GMOCK_DECLARE_KIND_(unsigned char, kInteger);
GMOCK_DECLARE_KIND_(short, kInteger);           // NOLINT
GMOCK_DECLARE_KIND_(unsigned short, kInteger);  // NOLINT
GMOCK_DECLARE_KIND_(int, kInteger);
GMOCK_DECLARE_KIND_(unsigned int, kInteger);
GMOCK_DECLARE_KIND_(long, kInteger);                // NOLINT
GMOCK_DECLARE_KIND_(unsigned long, kInteger);       // NOLINT
GMOCK_DECLARE_KIND_(long long, kInteger);           // NOLINT
GMOCK_DECLARE_KIND_(unsigned long long, kInteger);  // NOLINT

#if GMOCK_WCHAR_T_IS_NATIVE_
GMOCK_DECLARE_KIND_(wchar_t, kInteger);
#endif

// All standard floating-point types.
GMOCK_DECLARE_KIND_(float, kFloatingPoint);
GMOCK_DECLARE_KIND_(double, kFloatingPoint);
GMOCK_DECLARE_KIND_(long double, kFloatingPoint);

#undef GMOCK_DECLARE_KIND_

// Evaluates to the kind of 'type'.
#define GMOCK_KIND_OF_(type)                   \
  static_cast< ::testing::internal::TypeKind>( \
      ::testing::internal::KindOf<type>::value)

// LosslessArithmeticConvertibleImpl<kFromKind, From, kToKind, To>::value
// is true if and only if arithmetic type From can be losslessly converted to
// arithmetic type To.
//
// It's the user's responsibility to ensure that both From and To are
// raw (i.e. has no CV modifier, is not a pointer, and is not a
// reference) built-in arithmetic types, kFromKind is the kind of
// From, and kToKind is the kind of To; the value is
// implementation-defined when the above pre-condition is violated.
template <TypeKind kFromKind, typename From, TypeKind kToKind, typename To>
using LosslessArithmeticConvertibleImpl = std::integral_constant<
    bool,
    // clang-format off
      // Converting from bool is always lossless
      (kFromKind == kBool) ? true
      // Converting between any other type kinds will be lossy if the type
      // kinds are not the same.
    : (kFromKind != kToKind) ? false
    : (kFromKind == kInteger &&
       // Converting between integers of different widths is allowed so long
       // as the conversion does not go from signed to unsigned.
      (((sizeof(From) < sizeof(To)) &&
        !(std::is_signed<From>::value && !std::is_signed<To>::value)) ||
       // Converting between integers of the same width only requires the
       // two types to have the same signedness.
       ((sizeof(From) == sizeof(To)) &&
        (std::is_signed<From>::value == std::is_signed<To>::value)))
       ) ? true
      // Floating point conversions are lossless if and only if `To` is at least
      // as wide as `From`.
    : (kFromKind == kFloatingPoint && (sizeof(From) <= sizeof(To))) ? true
    : false
    // clang-format on
    >;

// LosslessArithmeticConvertible<From, To>::value is true if and only if
// arithmetic type From can be losslessly converted to arithmetic type To.
//
// It's the user's responsibility to ensure that both From and To are
// raw (i.e. has no CV modifier, is not a pointer, and is not a
// reference) built-in arithmetic types; the value is
// implementation-defined when the above pre-condition is violated.
template <typename From, typename To>
using LosslessArithmeticConvertible =
    LosslessArithmeticConvertibleImpl<GMOCK_KIND_OF_(From), From,
                                      GMOCK_KIND_OF_(To), To>;

// This interface knows how to report a Google Mock failure (either
// non-fatal or fatal).
class FailureReporterInterface {
 public:
  // The type of a failure (either non-fatal or fatal).
  enum FailureType { kNonfatal, kFatal };

  virtual ~FailureReporterInterface() = default;

  // Reports a failure that occurred at the given source file location.
  virtual void ReportFailure(FailureType type, const char* file, int line,
                             const std::string& message) = 0;
};

// Returns the failure reporter used by Google Mock.
GTEST_API_ FailureReporterInterface* GetFailureReporter();

// Asserts that condition is true; aborts the process with the given
// message if condition is false.  We cannot use LOG(FATAL) or CHECK()
// as Google Mock might be used to mock the log sink itself.  We
// inline this function to prevent it from showing up in the stack
// trace.
inline void Assert(bool condition, const char* file, int line,
                   const std::string& msg) {
  if (!condition) {
    GetFailureReporter()->ReportFailure(FailureReporterInterface::kFatal, file,
                                        line, msg);
  }
}
inline void Assert(bool condition, const char* file, int line) {
  Assert(condition, file, line, "Assertion failed.");
}

// Verifies that condition is true; generates a non-fatal failure if
// condition is false.
inline void Expect(bool condition, const char* file, int line,
                   const std::string& msg) {
  if (!condition) {
    GetFailureReporter()->ReportFailure(FailureReporterInterface::kNonfatal,
                                        file, line, msg);
  }
}
inline void Expect(bool condition, const char* file, int line) {
  Expect(condition, file, line, "Expectation failed.");
}

// Severity level of a log.
enum LogSeverity { kInfo = 0, kWarning = 1 };

// Valid values for the --gmock_verbose flag.

// All logs (informational and warnings) are printed.
const char kInfoVerbosity[] = "info";
// Only warnings are printed.
const char kWarningVerbosity[] = "warning";
// No logs are printed.
const char kErrorVerbosity[] = "error";

// Returns true if and only if a log with the given severity is visible
// according to the --gmock_verbose flag.
GTEST_API_ bool LogIsVisible(LogSeverity severity);

// Prints the given message to stdout if and only if 'severity' >= the level
// specified by the --gmock_verbose flag.  If stack_frames_to_skip >=
// 0, also prints the stack trace excluding the top
// stack_frames_to_skip frames.  In opt mode, any positive
// stack_frames_to_skip is treated as 0, since we don't know which
// function calls will be inlined by the compiler and need to be
// conservative.
GTEST_API_ void Log(LogSeverity severity, const std::string& message,
                    int stack_frames_to_skip);

// A marker class that is used to resolve parameterless expectations to the
// correct overload. This must not be instantiable, to prevent client code from
// accidentally resolving to the overload; for example:
//
//    ON_CALL(mock, Method({}, nullptr))...
//
class WithoutMatchers {
 private:
  WithoutMatchers() {}
  friend GTEST_API_ WithoutMatchers GetWithoutMatchers();
};

// Internal use only: access the singleton instance of WithoutMatchers.
GTEST_API_ WithoutMatchers GetWithoutMatchers();

// Invalid<T>() is usable as an expression of type T, but will terminate
// the program with an assertion failure if actually run.  This is useful
// when a value of type T is needed for compilation, but the statement
// will not really be executed (or we don't care if the statement
// crashes).
template <typename T>
inline T Invalid() {
  Assert(/*condition=*/false, /*file=*/"", /*line=*/-1,
         "Internal error: attempt to return invalid value");
#if defined(__GNUC__) || defined(__clang__)
  __builtin_unreachable();
#elif defined(_MSC_VER)
  __assume(0);
#else
  return Invalid<T>();
#endif
}

// Given a raw type (i.e. having no top-level reference or const
// modifier) RawContainer that's either an STL-style container or a
// native array, class StlContainerView<RawContainer> has the
// following members:
//
//   - type is a type that provides an STL-style container view to
//     (i.e. implements the STL container concept for) RawContainer;
//   - const_reference is a type that provides a reference to a const
//     RawContainer;
//   - ConstReference(raw_container) returns a const reference to an STL-style
//     container view to raw_container, which is a RawContainer.
//   - Copy(raw_container) returns an STL-style container view of a
//     copy of raw_container, which is a RawContainer.
//
// This generic version is used when RawContainer itself is already an
// STL-style container.
template <class RawContainer>
class StlContainerView {
 public:
  typedef RawContainer type;
  typedef const type& const_reference;

  static const_reference ConstReference(const RawContainer& container) {
    static_assert(!std::is_const<RawContainer>::value,
                  "RawContainer type must not be const");
    return container;
  }
  static type Copy(const RawContainer& container) { return container; }
};

// This specialization is used when RawContainer is a native array type.
template <typename Element, size_t N>
class StlContainerView<Element[N]> {
 public:
  typedef typename std::remove_const<Element>::type RawElement;
  typedef internal::NativeArray<RawElement> type;
  // NativeArray<T> can represent a native array either by value or by
  // reference (selected by a constructor argument), so 'const type'
  // can be used to reference a const native array.  We cannot
  // 'typedef const type& const_reference' here, as that would mean
  // ConstReference() has to return a reference to a local variable.
  typedef const type const_reference;

  static const_reference ConstReference(const Element (&array)[N]) {
    static_assert(std::is_same<Element, RawElement>::value,
                  "Element type must not be const");
    return type(array, N, RelationToSourceReference());
  }
  static type Copy(const Element (&array)[N]) {
    return type(array, N, RelationToSourceCopy());
  }
};

// This specialization is used when RawContainer is a native array
// represented as a (pointer, size) tuple.
template <typename ElementPointer, typename Size>
class StlContainerView< ::std::tuple<ElementPointer, Size> > {
 public:
  typedef typename std::remove_const<
      typename std::pointer_traits<ElementPointer>::element_type>::type
      RawElement;
  typedef internal::NativeArray<RawElement> type;
  typedef const type const_reference;

  static const_reference ConstReference(
      const ::std::tuple<ElementPointer, Size>& array) {
    return type(std::get<0>(array), std::get<1>(array),
                RelationToSourceReference());
  }
  static type Copy(const ::std::tuple<ElementPointer, Size>& array) {
    return type(std::get<0>(array), std::get<1>(array), RelationToSourceCopy());
  }
};

// The following specialization prevents the user from instantiating
// StlContainer with a reference type.
template <typename T>
class StlContainerView<T&>;

// A type transform to remove constness from the first part of a pair.
// Pairs like that are used as the value_type of associative containers,
// and this transform produces a similar but assignable pair.
template <typename T>
struct RemoveConstFromKey {
  typedef T type;
};

// Partially specialized to remove constness from std::pair<const K, V>.
template <typename K, typename V>
struct RemoveConstFromKey<std::pair<const K, V> > {
  typedef std::pair<K, V> type;
};

// Emit an assertion failure due to incorrect DoDefault() usage. Out-of-lined to
// reduce code size.
GTEST_API_ void IllegalDoDefault(const char* file, int line);

template <typename F, typename Tuple, size_t... Idx>
auto ApplyImpl(F&& f, Tuple&& args, std::index_sequence<Idx...>)
    -> decltype(std::forward<F>(f)(
        std::get<Idx>(std::forward<Tuple>(args))...)) {
  return std::forward<F>(f)(std::get<Idx>(std::forward<Tuple>(args))...);
}

// Apply the function to a tuple of arguments.
template <typename F, typename Tuple>
auto Apply(F&& f, Tuple&& args)
    -> decltype(ApplyImpl(
        std::forward<F>(f), std::forward<Tuple>(args),
        std::make_index_sequence<std::tuple_size<
            typename std::remove_reference<Tuple>::type>::value>())) {
  return ApplyImpl(std::forward<F>(f), std::forward<Tuple>(args),
                   std::make_index_sequence<std::tuple_size<
                       typename std::remove_reference<Tuple>::type>::value>());
}

// Template struct Function<F>, where F must be a function type, contains
// the following typedefs:
//
//   Result:               the function's return type.
//   Arg<N>:               the type of the N-th argument, where N starts with 0.
//   ArgumentTuple:        the tuple type consisting of all parameters of F.
//   ArgumentMatcherTuple: the tuple type consisting of Matchers for all
//                         parameters of F.
//   MakeResultVoid:       the function type obtained by substituting void
//                         for the return type of F.
//   MakeResultIgnoredValue:
//                         the function type obtained by substituting Something
//                         for the return type of F.
template <typename T>
struct Function;

template <typename R, typename... Args>
struct Function<R(Args...)> {
  using Result = R;
  static constexpr size_t ArgumentCount = sizeof...(Args);
  template <size_t I>
  using Arg = ElemFromList<I, Args...>;
  using ArgumentTuple = std::tuple<Args...>;
  using ArgumentMatcherTuple = std::tuple<Matcher<Args>...>;
  using MakeResultVoid = void(Args...);
  using MakeResultIgnoredValue = IgnoredValue(Args...);
};

// Workaround for MSVC error C2039: 'type': is not a member of 'std'
// when std::tuple_element is used.
// See: https://github.com/google/googletest/issues/3931
// Can be replaced with std::tuple_element_t in C++14.
template <size_t I, typename T>
using TupleElement = typename std::tuple_element<I, T>::type;

bool Base64Unescape(const std::string& encoded, std::string* decoded);

GTEST_DISABLE_MSC_WARNINGS_POP_()  // 4100 4805

}  // namespace internal
}  // namespace testing

#endif  // GOOGLEMOCK_INCLUDE_GMOCK_INTERNAL_GMOCK_INTERNAL_UTILS_H_
