/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_SCOPED_THREAD_DESKTOP_H_
#define MODULES_DESKTOP_CAPTURE_WIN_SCOPED_THREAD_DESKTOP_H_

#include <windows.h>

#include <memory>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class Desktop;

class RTC_EXPORT ScopedThreadDesktop {
 public:
  ScopedThreadDesktop();
  ~ScopedThreadDesktop();

  ScopedThreadDesktop(const ScopedThreadDesktop&) = delete;
  ScopedThreadDesktop& operator=(const ScopedThreadDesktop&) = delete;

  // Returns true if `desktop` has the same desktop name as the currently
  // assigned desktop (if assigned) or as the initial desktop (if not assigned).
  // Returns false in any other case including failing Win32 APIs and
  // uninitialized desktop handles.
  bool IsSame(const Desktop& desktop);

  // Reverts the calling thread to use the initial desktop.
  void Revert();

  // Assigns `desktop` to be the calling thread. Returns true if the thread has
  // been switched to `desktop` successfully. Takes ownership of `desktop`.
  bool SetThreadDesktop(Desktop* desktop);

 private:
  // The desktop handle assigned to the calling thread by Set
  std::unique_ptr<Desktop> assigned_;

  // The desktop handle assigned to the calling thread at creation.
  std::unique_ptr<Desktop> initial_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_SCOPED_THREAD_DESKTOP_H_
