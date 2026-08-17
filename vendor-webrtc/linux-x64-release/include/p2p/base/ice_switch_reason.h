/*
 *  Copyright 2022 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ICE_SWITCH_REASON_H_
#define P2P_BASE_ICE_SWITCH_REASON_H_

#include <string>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

enum class IceSwitchReason {
  UNKNOWN,
  REMOTE_CANDIDATE_GENERATION_CHANGE,
  NETWORK_PREFERENCE_CHANGE,
  NEW_CONNECTION_FROM_LOCAL_CANDIDATE,
  NEW_CONNECTION_FROM_REMOTE_CANDIDATE,
  NEW_CONNECTION_FROM_UNKNOWN_REMOTE_ADDRESS,
  NOMINATION_ON_CONTROLLED_SIDE,
  DATA_RECEIVED,
  CONNECT_STATE_CHANGE,
  SELECTED_CONNECTION_DESTROYED,
  // The ICE_CONTROLLER_RECHECK enum value lets an IceController request
  // P2PTransportChannel to recheck a switch periodically without an event
  // taking place.
  ICE_CONTROLLER_RECHECK,
  // The webrtc application requested a connection switch.
  APPLICATION_REQUESTED,
};

RTC_EXPORT std::string IceSwitchReasonToString(IceSwitchReason reason);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IceSwitchReason;
using ::webrtc::IceSwitchReasonToString;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ICE_SWITCH_REASON_H_
