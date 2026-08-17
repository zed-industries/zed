/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_WINDOW_FINDER_MAC_H_
#define MODULES_DESKTOP_CAPTURE_WINDOW_FINDER_MAC_H_

#include "api/scoped_refptr.h"
#include "modules/desktop_capture/window_finder.h"

namespace webrtc {

class DesktopConfigurationMonitor;

// The implementation of WindowFinder for Mac OSX.
class WindowFinderMac final : public WindowFinder {
 public:
  explicit WindowFinderMac(
      webrtc::scoped_refptr<DesktopConfigurationMonitor> configuration_monitor);
  ~WindowFinderMac() override;

  // WindowFinder implementation.
  WindowId GetWindowUnderPoint(DesktopVector point) override;

 private:
  const webrtc::scoped_refptr<DesktopConfigurationMonitor>
      configuration_monitor_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_WINDOW_FINDER_MAC_H_
