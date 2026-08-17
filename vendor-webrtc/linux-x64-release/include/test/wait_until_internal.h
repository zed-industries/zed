/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_WAIT_UNTIL_INTERNAL_H_
#define TEST_WAIT_UNTIL_INTERNAL_H_

#include <string>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "test/gmock.h"
#include "test/gtest.h"

namespace webrtc {
namespace wait_until_internal {

// Explains the match result of `matcher` against `value` to `listener`.
// `value_name` is the name of the value to be used in the error message.
// This is inspired by testing::ExplainMatchResult and
// testing::internal::MatchPrintAndExplain.
template <typename T, typename M>
bool ExplainMatchResult(const M& matcher,
                        const T& value,
                        ::testing::StringMatchResultListener* absl_nonnull
                            listener,
                        absl::string_view value_name) {
  // SafeMatcherCast is required for matchers whose type does not match the
  // argument type.
  ::testing::Matcher<const T&> safe_matcher =
      ::testing::SafeMatcherCast<const T&>(matcher);

  auto* ss = listener->stream();
  *ss << "Value of: " << value_name << "\n";
  *ss << "Expected: ";
  safe_matcher.DescribeTo(ss);
  *ss << "\nActual: ";
  ::testing::StringMatchResultListener inner_listener;
  if (::testing::ExplainMatchResult(safe_matcher, value, &inner_listener)) {
    return true;
  }
  *ss << ::testing::PrintToString(value);
  if (const std::string& inner_message = inner_listener.str();
      !inner_message.empty()) {
    *ss << ", " << inner_message;
  }
  return false;
}

}  // namespace wait_until_internal
}  // namespace webrtc

#endif  // TEST_WAIT_UNTIL_INTERNAL_H_
