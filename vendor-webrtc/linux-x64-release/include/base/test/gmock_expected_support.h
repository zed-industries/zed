// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GMOCK_EXPECTED_SUPPORT_H_
#define BASE_TEST_GMOCK_EXPECTED_SUPPORT_H_

#include <ostream>
#include <string>
#include <type_traits>
#include <utility>

#include "base/strings/strcat.h"
#include "base/strings/to_string.h"
#include "base/types/expected.h"
#include "base/types/expected_internal.h"
#include "base/types/expected_macros.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base::test {

namespace internal {

// `HasVoidValueType<T>` is true iff `T` satisfies
// `base::internal::IsExpected<T>` and `T`'s `value_type` is `void`.
template <typename T>
concept HasVoidValueType =
    base::internal::IsExpected<T> &&
    std::is_void_v<typename std::remove_cvref_t<T>::value_type>;

// Implementation for matcher `HasValue`.
class HasValueMatcher {
 public:
  HasValueMatcher() = default;

  template <typename T>
  operator ::testing::Matcher<T>() const {  // NOLINT
    return ::testing::Matcher<T>(new Impl<const T&>());
  }

 private:
  template <typename T>
    requires(base::internal::IsExpected<T>)
  class Impl : public ::testing::MatcherInterface<T> {
   public:
    Impl() = default;

    void DescribeTo(std::ostream* os) const override {
      *os << "is an 'expected' type with a value";
    }

    void DescribeNegationTo(std::ostream* os) const override {
      *os << "is an 'expected' type with an error";
    }

    bool MatchAndExplain(
        T actual_value,
        ::testing::MatchResultListener* listener) const override {
      if (!actual_value.has_value()) {
        *listener << "which has the error " << ToString(actual_value.error());
      }
      return actual_value.has_value();
    }
  };
};

// Implementation for matcher `ValueIs`.
template <typename T>
class ValueIsMatcher {
 public:
  explicit ValueIsMatcher(T matcher) : matcher_(std::move(matcher)) {}

  template <typename U>
  operator ::testing::Matcher<U>() const {  // NOLINT
    return ::testing::Matcher<U>(new Impl<const U&>(matcher_));
  }

 private:
  template <typename U>
    requires(base::internal::IsExpected<U> && !HasVoidValueType<U>)
  class Impl : public ::testing::MatcherInterface<U> {
   public:
    explicit Impl(const T& matcher)
        : matcher_(::testing::SafeMatcherCast<const V&>(matcher)) {}

    void DescribeTo(std::ostream* os) const override {
      *os << "is an 'expected' type with a value which ";
      matcher_.DescribeTo(os);
    }

    void DescribeNegationTo(std::ostream* os) const override {
      *os << "is an 'expected' type with an error or a value which ";
      matcher_.DescribeNegationTo(os);
    }

    bool MatchAndExplain(
        U actual_value,
        ::testing::MatchResultListener* listener) const override {
      if (!actual_value.has_value()) {
        *listener << "which has the error " << ToString(actual_value.error());
        return false;
      }

      ::testing::StringMatchResultListener inner_listener;
      const bool match =
          matcher_.MatchAndExplain(actual_value.value(), &inner_listener);
      const std::string explanation = inner_listener.str();
      if (!explanation.empty()) {
        *listener << "which has the value " << ToString(actual_value.value())
                  << ", " << explanation;
      }
      return match;
    }

   private:
    using V = typename std::remove_cvref_t<U>::value_type;

    const ::testing::Matcher<const V&> matcher_;
  };

  const T matcher_;
};

// Implementation for matcher `ErrorIs`.
template <typename T>
class ErrorIsMatcher {
 public:
  explicit ErrorIsMatcher(T matcher) : matcher_(std::move(matcher)) {}

  template <typename U>
  operator ::testing::Matcher<U>() const {  // NOLINT
    return ::testing::Matcher<U>(new Impl<const U&>(matcher_));
  }

 private:
  template <typename U>
    requires(base::internal::IsExpected<U>)
  class Impl : public ::testing::MatcherInterface<U> {
   public:
    explicit Impl(const T& matcher)
        : matcher_(::testing::SafeMatcherCast<const E&>(matcher)) {}

    void DescribeTo(std::ostream* os) const override {
      *os << "is an 'expected' type with an error which ";
      matcher_.DescribeTo(os);
    }

    void DescribeNegationTo(std::ostream* os) const override {
      *os << "is an 'expected' type with a value or an error which ";
      matcher_.DescribeNegationTo(os);
    }

    bool MatchAndExplain(
        U actual_value,
        ::testing::MatchResultListener* listener) const override {
      if (actual_value.has_value()) {
        if constexpr (HasVoidValueType<U>) {
          *listener << "which has a value";
        } else {
          *listener << "which has the value " << ToString(actual_value.value());
        }
        return false;
      }

      ::testing::StringMatchResultListener inner_listener;
      const bool match =
          matcher_.MatchAndExplain(actual_value.error(), &inner_listener);
      const std::string explanation = inner_listener.str();
      if (!explanation.empty()) {
        *listener << "which has the error " << ToString(actual_value.error())
                  << ", " << explanation;
      }
      return match;
    }

   private:
    using E = typename std::remove_cvref_t<U>::error_type;

    const ::testing::Matcher<const E&> matcher_;
  };

 private:
  const T matcher_;
};

}  // namespace internal

// Returns a gMock matcher that matches an `expected<T, E>` which has a value.
inline internal::HasValueMatcher HasValue() {
  return internal::HasValueMatcher();
}

// Returns a gMock matcher that matches an `expected<T, E>` which has a non-void
// value which matches the inner matcher.
template <typename T>
inline internal::ValueIsMatcher<typename std::decay_t<T>> ValueIs(T&& matcher) {
  return internal::ValueIsMatcher<typename std::decay_t<T>>(
      std::forward<T>(matcher));
}

// Returns a gMock matcher that matches an `expected<T, E>` which has an error
// which matches the inner matcher.
template <typename T>
inline internal::ErrorIsMatcher<typename std::decay_t<T>> ErrorIs(T&& matcher) {
  return internal::ErrorIsMatcher<typename std::decay_t<T>>(
      std::forward<T>(matcher));
}

}  // namespace base::test

// Executes an expression that returns an `expected<T, E>` or
// `std::optional<T>`, and assigns the contained `T` to `lhs` if the result is a
// value. If the result is an error, generates a test failure and returns from
// the current function, which must have a `void` return type. For more usage
// examples and caveats, see the documentation for `ASSIGN_OR_RETURN`.
//
// Example: Declaring and initializing a new value:
//   ASSERT_OK_AND_ASSIGN(ValueType value, MaybeGetValue(arg));
//
// Example: Assigning to an existing value:
//   ValueType value;
//   ASSERT_OK_AND_ASSIGN(value, MaybeGetValue(arg));
#define ASSERT_OK_AND_ASSIGN(lhs, rexpr)                             \
  ASSIGN_OR_RETURN(lhs, rexpr, []<typename... Ts>(const Ts&... e) {  \
    std::string message;                                             \
    if constexpr (sizeof...(Ts) > 0) {                               \
      message = base::StrCat(                                        \
          {#rexpr, " returned error: ", (..., base::ToString(e))});  \
    } else {                                                         \
      message = base::StrCat({#rexpr, " returned nullopt"});         \
    }                                                                \
    return GTEST_MESSAGE_(message.c_str(),                           \
                          ::testing::TestPartResult::kFatalFailure); \
  })

namespace base {
template <typename T, typename E>
void PrintTo(const expected<T, E>& expected, ::std::ostream* os) {
  *os << expected.ToString();
}

template <typename T>
void PrintTo(const ok<T>& a, ::std::ostream* os) {
  *os << a.ToString();
}

template <typename T>
void PrintTo(const unexpected<T>& a, ::std::ostream* os) {
  *os << a.ToString();
}
}  // namespace base

#endif  // BASE_TEST_GMOCK_EXPECTED_SUPPORT_H_
