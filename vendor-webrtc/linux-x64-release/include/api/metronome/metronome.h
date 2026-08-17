/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_METRONOME_METRONOME_H_
#define API_METRONOME_METRONOME_H_

#include "absl/functional/any_invocable.h"
#include "api/units/time_delta.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// The Metronome posts OnTick() calls requested with RequestCallOnNextTick.
// The API is designed to be fully used from a single task queue. Scheduled
// callbacks are executed on the same sequence as they were requested on. There
// are no features implemented for cancellation. When that's needed, use e.g.
// ScopedTaskSafety from the client.
//
// The metronome concept is still under experimentation, and may not be availble
// in all platforms or applications. See https://crbug.com/1253787 for more
// details.
//
// Metronome implementations must be thread-compatible.
class RTC_EXPORT Metronome {
 public:
  virtual ~Metronome() = default;

  // Requests a call to `callback` on the next tick. Scheduled callbacks are
  // executed on the same sequence as they were requested on. There are no
  // features for cancellation. When that's needed, use e.g. ScopedTaskSafety
  // from the client.
  virtual void RequestCallOnNextTick(
      absl::AnyInvocable<void() &&> /* callback */) {}

  // Returns the current tick period of the metronome.
  virtual TimeDelta TickPeriod() const = 0;
};

}  // namespace webrtc

#endif  // API_METRONOME_METRONOME_H_
