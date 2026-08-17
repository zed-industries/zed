/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_NETWORK_MONITOR_OBSERVER_H_
#define SDK_OBJC_NATIVE_SRC_NETWORK_MONITOR_OBSERVER_H_

#include <map>
#include <string>

#include "absl/strings/string_view.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/string_utils.h"
#include "rtc_base/thread.h"

namespace webrtc {

// Observer interface for listening to NWPathMonitor updates.
class NetworkMonitorObserver {
 public:
  // Called when a path update occurs, on network monitor dispatch queue.
  //
  // `adapter_type_by_name` is a map from interface name (i.e. "pdp_ip0") to
  // adapter type, for all available interfaces on the current path. If an
  // interface name isn't present it can be assumed to be unavailable.
  virtual void OnPathUpdate(
      std::map<std::string, webrtc::AdapterType, webrtc::AbslStringViewCmp>
          adapter_type_by_name) = 0;

 protected:
  virtual ~NetworkMonitorObserver() {}
};

}  // namespace webrtc

#endif  //  SDK_OBJC_NATIVE_SRC_AUDIO_AUDIO_SESSION_OBSERVER_H_
