/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_DEPRECATED_EVENT_WRAPPER_H_
#define MODULES_VIDEO_CODING_DEPRECATED_EVENT_WRAPPER_H_

namespace webrtc {
enum EventTypeWrapper { kEventSignaled = 1, kEventTimeout = 2 };

class EventWrapper {
 public:
  // Factory method. Constructor disabled.
  static EventWrapper* Create();

  virtual ~EventWrapper() {}

  // Releases threads who are calling Wait() and has started waiting. Please
  // note that a thread calling Wait() will not start waiting immediately.
  // assumptions to the contrary is a very common source of issues in
  // multithreaded programming.
  // Set is sticky in the sense that it will release at least one thread
  // either immediately or some time in the future.
  virtual bool Set() = 0;

  // Puts the calling thread into a wait state. The thread may be released
  // by a Set() call depending on if other threads are waiting and if so on
  // timing. The thread that was released will reset the event before leaving
  // preventing more threads from being released. If multiple threads
  // are waiting for the same Set(), only one (random) thread is guaranteed to
  // be released. It is possible that multiple (random) threads are released
  // Depending on timing.
  //
  // `max_time_ms` is the maximum time to wait in milliseconds.
  // TODO(bugs.webrtc.org/14366): Migrate to TimeDelta.
  virtual EventTypeWrapper Wait(int max_time_ms) = 0;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_EVENT_WRAPPER_H_
