// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_INPUT_EVENT_RESULT_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_INPUT_EVENT_RESULT_H_

namespace blink {

enum class WebInputEventResult {
  // Event was not consumed by application or system.
  kNotHandled,
  // Event was consumed but suppressed before dispatched to application.
  kHandledSuppressed,
  // Event was consumed by application itself; ie. a script handler calling
  // preventDefault.
  kHandledApplication,
  // Event was consumed by the system; ie. executing the default action.
  kHandledSystem,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_INPUT_EVENT_RESULT_H_
