// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EXCEPTION_STATE_MATCHERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EXCEPTION_STATE_MATCHERS_H_

#include <string>

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"

namespace blink {

// Teach gTest how to print the actual value of the exception state.
void PrintTo(const DummyExceptionStateForTesting& exception_state,
             std::ostream* os);

namespace internal {

std::string ExceptionCodeToString(ExceptionCode code);

}  // namespace internal

MATCHER(HadException, "") {
  return arg.HadException();
}

MATCHER_P(HadException,
          code,
          internal::ExceptionCodeToString(ToExceptionCode(code))) {
  return arg.HadException() && arg.Code() == ToExceptionCode(code);
}

MATCHER_P2(HadException,
           code,
           message,
           internal::ExceptionCodeToString(ToExceptionCode(code)) + ": " +
               testing::PrintToString(message)) {
  return arg.HadException() && arg.Code() == ToExceptionCode(code) &&
         arg.Message() == message;
}

MATCHER(HadNoException, "") {
  return !arg.HadException();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_EXCEPTION_STATE_MATCHERS_H_
