/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_CONFIGURATION_MONITOR_H_
#define MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_CONFIGURATION_MONITOR_H_

#include <ApplicationServices/ApplicationServices.h>

#include <memory>
#include <set>

#include "api/ref_counted_base.h"
#include "modules/desktop_capture/mac/desktop_configuration.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// The class provides functions to synchronize capturing and display
// reconfiguring across threads, and the up-to-date MacDesktopConfiguration.
class DesktopConfigurationMonitor final
    : public webrtc::RefCountedNonVirtual<DesktopConfigurationMonitor> {
 public:
  DesktopConfigurationMonitor();
  ~DesktopConfigurationMonitor();

  DesktopConfigurationMonitor(const DesktopConfigurationMonitor&) = delete;
  DesktopConfigurationMonitor& operator=(const DesktopConfigurationMonitor&) =
      delete;

  // Returns the current desktop configuration.
  MacDesktopConfiguration desktop_configuration();

 private:
  static void DisplaysReconfiguredCallback(CGDirectDisplayID display,
                                           CGDisplayChangeSummaryFlags flags,
                                           void* user_parameter);
  void DisplaysReconfigured(CGDirectDisplayID display,
                            CGDisplayChangeSummaryFlags flags);

  Mutex desktop_configuration_lock_;
  MacDesktopConfiguration desktop_configuration_
      RTC_GUARDED_BY(&desktop_configuration_lock_);
  std::set<CGDirectDisplayID> reconfiguring_displays_;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_MAC_DESKTOP_CONFIGURATION_MONITOR_H_
