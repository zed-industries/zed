/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TRANSPORT_BANDWIDTH_ESTIMATION_SETTINGS_H_
#define API_TRANSPORT_BANDWIDTH_ESTIMATION_SETTINGS_H_

#include "rtc_base/system/rtc_export.h"
namespace webrtc {
// Configuration settings affecting bandwidth estimation.
// These settings can be set and changed by an application.
struct RTC_EXPORT BandwidthEstimationSettings {
  // A bandwith estimation probe may be sent using a RtpTransceiver with
  // direction SendOnly or SendRecv that supports RTX. The probe can be sent
  // without first sending media packets in which case Rtp padding packets are
  // used.
  bool allow_probe_without_media = false;
};

}  // namespace webrtc
#endif  // API_TRANSPORT_BANDWIDTH_ESTIMATION_SETTINGS_H_
