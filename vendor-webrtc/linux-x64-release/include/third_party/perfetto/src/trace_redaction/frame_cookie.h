/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_REDACTION_FRAME_COOKIE_H_
#define SRC_TRACE_REDACTION_FRAME_COOKIE_H_

#include <cstdint>

struct FrameCookie {
  // The timestamp from the trace packet.
  uint64_t ts;

  // The cookie value will be found inside of the start event (there are four
  // different start types). This is the app's pid (main thread id).

  // ExpectedSurfaceFrameStart: pid = app id
  // ActualSurfaceFrameStart: pid = app id

  // ExpectedDisplayFrameStart: pid = surface flinger
  // ActualDisplayFrameStart: pid = surface flinger
  int32_t pid;

  // The cookie value will be found inside of the start event (there are four
  // different start types). End events use the cookie to connect to the start
  // event. Therefore end events don't need a pid.
  int64_t cookie;
};

#endif  // SRC_TRACE_REDACTION_FRAME_COOKIE_H_
