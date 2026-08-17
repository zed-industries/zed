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
// This file tests some commonly used argument matchers.

#ifndef GOOGLEMOCK_TEST_GMOCK_MATCHERS_TEST_H_
#define GOOGLEMOCK_TEST_GMOCK_MATCHERS_TEST_H_

#include <string.h>
#include <time.h>

#include <array>
#include <cstdint>
#include <deque>
#include <forward_list>
#include <functional>
#include <iostream>
#include <iterator>
#include <limits>
#include <list>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <string>
#include <type_traits>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "gmock/gmock-matchers.h"
#include "gmock/gmock-more-matchers.h"
#include "gmock/gmock.h"
#include "gtest/gtest-spi.h"
#include "gtest/gtest.h"

namespace testing {
namespace gmock_matchers_test {

using std::greater;
using std::less;
using std::list;
using std::make_pair;
using std::map;
using std::multimap;
using std::multiset;
using std::ostream;
using std::pair;
using std::set;
using std::stringstream;
using std::vector;
using testing::internal::DummyMatchResultListener;
using testing::internal::ElementMatcherPair;
using testing::internal::ElementMatcherPairs;
using testing::internal::ElementsAreArrayMatcher;
using testing::internal::ExplainMatchFailureTupleTo;
using testing::internal::FloatingEqMatcher;
using testing::internal::FormatMatcherDescription;
using testing::internal::IsReadableTypeName;
using testing::internal::MatchMatrix;
using testing::internal::PredicateFormatterFromMatcher;
using testing::internal::RE;
using testing::internal::StreamMatchResultListener;
using testing::internal::Strings;

// Helper for testing container-valued matchers in mock method context. It is
// important to test matchers in this context, since it requires additional type
// deduction beyond what EXPECT_THAT does, thus making it more restrictive.
struct ContainerHelper {
  MOCK_METHOD1(Call, void(std::vector<std::unique_ptr<int>>));
};

// For testing ExplainMatchResultTo().
template <typename T>
struct GtestGreaterThanMatcher {
  using is_gtest_matcher = void;

  void DescribeTo(ostream* os) const { *os << "is > " << rhs; }
  void DescribeNegationTo(ostream* os) const { *os << "is <= " << rhs; }

  bool MatchAndExplain(T lhs, MatchResultListener* listener) const {
    if (lhs > rhs) {
      *listener << "which is " << (lhs - rhs) << " more than " << rhs;
    } else if (lhs == rhs) {
      *listener << "which is the same as " << rhs;
    } else {
      *listener << "which is " << (rhs - lhs) << " less than " << rhs;
    }

    return lhs > rhs;
  }

  T rhs;
};

template <typename T>
GtestGreaterThanMatcher<typename std::decay<T>::type> GtestGreaterThan(
    T&& rhs) {
  return {rhs};
}

// As the matcher above, but using the base class with virtual functions.
template <typename T>
class GreaterThanMatcher : public MatcherInterface<T> {
 public:
  explicit GreaterThanMatcher(T rhs) : impl_{rhs} {}

  void DescribeTo(ostream* os) const override { impl_.DescribeTo(os); }
  void DescribeNegationTo(ostream* os) const override {
    impl_.DescribeNegationTo(os);
  }

  bool MatchAndExplain(T lhs, MatchResultListener* listener) const override {
    return impl_.MatchAndExplain(lhs, listener);
  }

 private:
  const GtestGreaterThanMatcher<T> impl_;
};

// Names and instantiates a new instance of GTestMatcherTestP.
#define INSTANTIATE_GTEST_MATCHER_TEST_P(TestSuite)                        \
  using TestSuite##P = GTestMatcherTestP;                                  \
  INSTANTIATE_TEST_SUITE_P(MatcherInterface, TestSuite##P, Values(false)); \
  INSTANTIATE_TEST_SUITE_P(GtestMatcher, TestSuite##P, Values(true))

class GTestMatcherTestP : public testing::TestWithParam<bool> {
 public:
  template <typename T>
  Matcher<T> GreaterThan(T n) {
    if (use_gtest_matcher_) {
      return GtestGreaterThan(n);
    } else {
      return MakeMatcher(new GreaterThanMatcher<T>(n));
    }
  }
  const bool use_gtest_matcher_ = GetParam();
};

// Returns the description of the given matcher.
template <typename T>
std::string Describe(const Matcher<T>& m) {
  return DescribeMatcher<T>(m);
}

// Returns the description of the negation of the given matcher.
template <typename T>
std::string DescribeNegation(const Matcher<T>& m) {
  return DescribeMatcher<T>(m, true);
}

// Returns the reason why x matches, or doesn't match, m.
template <typename MatcherType, typename Value>
std::string Explain(const MatcherType& m, const Value& x) {
  StringMatchResultListener listener;
  ExplainMatchResult(m, x, &listener);
  return listener.str();
}

}  // namespace gmock_matchers_test
}  // namespace testing

#endif  // GOOGLEMOCK_TEST_GMOCK_MATCHERS_TEST_H_
