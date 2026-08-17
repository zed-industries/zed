/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WIN_DISPLAY_CONFIGURATION_MONITOR_H_
#define MODULES_DESKTOP_CAPTURE_WIN_DISPLAY_CONFIGURATION_MONITOR_H_

#include "modules/desktop_capture/desktop_capturer.h"
#include "modules/desktop_capture/desktop_geometry.h"
#include "rtc_base/containers/flat_map.h"

namespace webrtc {

// A passive monitor to detect the change of display configuration on a Windows
// system.
// TODO(zijiehe): Also check for pixel format changes.
class DisplayConfigurationMonitor {
 public:
  // Checks whether the display configuration has changed since the last time
  // IsChanged() was called. |source_id| is used to observe changes for a
  // specific display or all displays if kFullDesktopScreenId is passed in.
  // Returns false if object was Reset() or if IsChanged() has not been called.
  bool IsChanged(DesktopCapturer::SourceId source_id);

  // Resets to the initial state.
  void Reset();

 private:
  DesktopVector GetDpiForSourceId(DesktopCapturer::SourceId source_id);

  // Represents the size of the desktop which includes all displays.
  DesktopRect rect_;

  // Tracks the DPI for each display being captured. We need to track for each
  // display as each one can be configured to use a different DPI which will not
  // be reflected in calls to get the system DPI.
  flat_map<DesktopCapturer::SourceId, DesktopVector> source_dpis_;

  // Indicates whether |rect_| and |source_dpis_| have been initialized. This is
  // used to prevent the monitor instance from signaling 'IsChanged()' before
  // the initial values have been set.
  bool initialized_ = false;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WIN_DISPLAY_CONFIGURATION_MONITOR_H_
