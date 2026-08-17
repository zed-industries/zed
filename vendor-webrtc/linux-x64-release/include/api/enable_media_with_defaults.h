/*
 *  Copyright 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_ENABLE_MEDIA_WITH_DEFAULTS_H_
#define API_ENABLE_MEDIA_WITH_DEFAULTS_H_

#include "api/peer_connection_interface.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Fills unset media related dependencies in `deps` and enables media support
// for a PeerConnectionFactory created from `deps`.
// This function is located in its own build target as it pulls additional
// dependencies compared to `EnableMedia`, and thus may add extra binary size.
RTC_EXPORT void EnableMediaWithDefaults(
    PeerConnectionFactoryDependencies& deps);

}  // namespace webrtc

#endif  // API_ENABLE_MEDIA_WITH_DEFAULTS_H_
