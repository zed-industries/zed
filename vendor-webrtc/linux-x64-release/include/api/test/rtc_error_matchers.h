/*
 *  Copyright 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_RTC_ERROR_MATCHERS_H_
#define API_TEST_RTC_ERROR_MATCHERS_H_

#include <string>

#include "absl/strings/str_cat.h"
#include "api/rtc_error.h"
#include "test/gmock.h"

namespace webrtc {

MATCHER(IsRtcOk, "") {
  if (!arg.ok()) {
    *result_listener << "Expected OK, got " << absl::StrCat(arg);
    return false;
  }
  return true;
}

MATCHER_P(IsRtcOkAndHolds,
          matcher,
          "RtcErrorOr that is holding an OK status and ") {
  if (!arg.ok()) {
    *result_listener << "Expected OK, got " << absl::StrCat(arg);
    return false;
  }
  return testing::ExplainMatchResult(matcher, arg.value(), result_listener);
}

MATCHER_P(IsRtcErrorWithType, error_type, ToString(error_type)) {
  if (arg.ok()) {
    *result_listener << "Expected " << ToString(error_type) << ", got OK.";
    return false;
  }
  if (arg.type() != error_type) {
    *result_listener << "Expected " << ToString(error_type) << ", got "
                     << ToString(arg.type());
    return false;
  }
  return true;
}

MATCHER_P2(IsRtcErrorWithTypeAndMessage,
           error_type,
           message,
           ToString(error_type)) {
  if (arg.ok()) {
    *result_listener << "Expected " << ToString(error_type) << ", got OK.";
    return false;
  }
  if (arg.type() != error_type) {
    *result_listener << "Expected " << ToString(error_type) << ", got "
                     << ToString(arg.type());
    return false;
  }
  if (std::string(arg.message()) != message) {
    *result_listener << "Expected message \"" << message << "\", got \""
                     << arg.message() << "\"";
    return false;
  }
  return true;
}

MATCHER_P2(IsRtcErrorOrWithMessage,
           error_matcher,
           message_matcher,
           "RtcErrorOr that is holding an error that " +
               testing::DescribeMatcher<RTCError>(error_matcher, negation) +
               (negation ? " or " : " and ") + " with a message that " +
               testing::DescribeMatcher<std::string>(message_matcher,
                                                     negation)) {
  if (arg.ok()) {
    *result_listener << "Expected error, got " << absl::StrCat(arg);
    return false;
  }
  return testing::ExplainMatchResult(error_matcher, arg.error(),
                                     result_listener) &&
         testing::ExplainMatchResult(message_matcher, arg.error().message(),
                                     result_listener);
}

}  // namespace webrtc

#endif  // API_TEST_RTC_ERROR_MATCHERS_H_
