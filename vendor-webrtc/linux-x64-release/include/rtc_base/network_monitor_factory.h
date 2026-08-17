/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_NETWORK_MONITOR_FACTORY_H_
#define RTC_BASE_NETWORK_MONITOR_FACTORY_H_

#include "rtc_base/network_monitor.h"
namespace webrtc {
class FieldTrialsView;
}  // namespace webrtc

namespace webrtc {

// Forward declaring this so it's not part of the API surface; it's only
// expected to be used by Android/iOS SDK code.

/*
 * NetworkMonitorFactory creates NetworkMonitors.
 * Note that CreateNetworkMonitor is expected to be called on the network
 * thread with the returned object only being used on that thread thereafter.
 */
class NetworkMonitorFactory {
 public:
  virtual NetworkMonitorInterface* CreateNetworkMonitor(
      const FieldTrialsView& field_trials) = 0;

  virtual ~NetworkMonitorFactory();

 protected:
  NetworkMonitorFactory();
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::NetworkMonitorFactory;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NETWORK_MONITOR_FACTORY_H_
