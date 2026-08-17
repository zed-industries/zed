// Copyright 2013, Google Inc.
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
// This file implements some matchers that depend on gmock-matchers.h.
//
// Note that tests are implemented in gmock-matchers_test.cc rather than
// gmock-more-matchers-test.cc.

// IWYU pragma: private, include "gmock/gmock.h"
// IWYU pragma: friend gmock/.*

#ifndef GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_MORE_MATCHERS_H_
#define GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_MORE_MATCHERS_H_

#include <ostream>
#include <string>

#include "gmock/gmock-matchers.h"

namespace testing {

// Silence C4100 (unreferenced formal
// parameter) for MSVC
GTEST_DISABLE_MSC_WARNINGS_PUSH_(4100)
#if defined(_MSC_VER) && (_MSC_VER == 1900)
// and silence C4800 (C4800: 'int *const ': forcing value
// to bool 'true' or 'false') for MSVC 14
GTEST_DISABLE_MSC_WARNINGS_PUSH_(4800)
#endif

namespace internal {

// Implements the polymorphic IsEmpty matcher, which
// can be used as a Matcher<T> as long as T is either a container that defines
// empty() and size() (e.g. std::vector or std::string), or a C-style string.
class IsEmptyMatcher {
 public:
  // Matches anything that defines empty() and size().
  template <typename MatcheeContainerType>
  bool MatchAndExplain(const MatcheeContainerType& c,
                       MatchResultListener* listener) const {
    if (c.empty()) {
      return true;
    }
    *listener << "whose size is " << c.size();
    return false;
  }

  // Matches C-style strings.
  bool MatchAndExplain(const char* s, MatchResultListener* listener) const {
    return MatchAndExplain(std::string(s), listener);
  }

  // Describes what this matcher matches.
  void DescribeTo(std::ostream* os) const { *os << "is empty"; }

  void DescribeNegationTo(std::ostream* os) const { *os << "isn't empty"; }
};

}  // namespace internal

// Creates a polymorphic matcher that matches an empty container or C-style
// string. The container must support both size() and empty(), which all
// STL-like containers provide.
inline PolymorphicMatcher<internal::IsEmptyMatcher> IsEmpty() {
  return MakePolymorphicMatcher(internal::IsEmptyMatcher());
}

// Define a matcher that matches a value that evaluates in boolean
// context to true.  Useful for types that define "explicit operator
// bool" operators and so can't be compared for equality with true
// and false.
MATCHER(IsTrue, negation ? "is false" : "is true") {
  return static_cast<bool>(arg);
}

// Define a matcher that matches a value that evaluates in boolean
// context to false.  Useful for types that define "explicit operator
// bool" operators and so can't be compared for equality with true
// and false.
MATCHER(IsFalse, negation ? "is true" : "is false") {
  return !static_cast<bool>(arg);
}

#if defined(_MSC_VER) && (_MSC_VER == 1900)
GTEST_DISABLE_MSC_WARNINGS_POP_()  // 4800
#endif
GTEST_DISABLE_MSC_WARNINGS_POP_()  // 4100

}  // namespace testing

#endif  // GOOGLEMOCK_INCLUDE_GMOCK_GMOCK_MORE_MATCHERS_H_
