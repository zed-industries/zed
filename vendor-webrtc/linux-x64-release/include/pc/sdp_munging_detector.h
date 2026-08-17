/*
 *  Copyright 2025 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SDP_MUNGING_DETECTOR_H_
#define PC_SDP_MUNGING_DETECTOR_H_

#include "api/jsep.h"
#include "api/uma_metrics.h"

namespace webrtc {
// Determines if and how the SDP was modified.
SdpMungingType DetermineSdpMungingType(
    const SessionDescriptionInterface* sdesc,
    const SessionDescriptionInterface* last_created_desc);

// Determines if the ICE ufrag or pwd of the SDP were modified.
bool HasUfragSdpMunging(const SessionDescriptionInterface* sdesc,
                        const SessionDescriptionInterface* last_created_desc);

}  // namespace webrtc

#endif  // PC_SDP_MUNGING_DETECTOR_H_
