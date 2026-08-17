/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_TURN_UTILS_H_
#define MEDIA_BASE_TURN_UTILS_H_

#include <cstddef>
#include <cstdint>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Finds data location within a TURN Channel Message or TURN Send Indication
// message.
bool RTC_EXPORT UnwrapTurnPacket(const uint8_t* packet,
                                 size_t packet_size,
                                 size_t* content_position,
                                 size_t* content_size);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::UnwrapTurnPacket;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_TURN_UTILS_H_
