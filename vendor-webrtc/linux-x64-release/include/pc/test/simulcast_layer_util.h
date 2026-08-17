/*
 *  Copyright 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_SIMULCAST_LAYER_UTIL_H_
#define PC_TEST_SIMULCAST_LAYER_UTIL_H_

#include <string>
#include <vector>

#include "api/jsep.h"
#include "api/rtp_transceiver_interface.h"
#include "pc/simulcast_description.h"

namespace webrtc {

std::vector<SimulcastLayer> CreateLayers(const std::vector<std::string>& rids,
                                         const std::vector<bool>& active);

std::vector<SimulcastLayer> CreateLayers(const std::vector<std::string>& rids,
                                         bool active);

RtpTransceiverInit CreateTransceiverInit(
    const std::vector<SimulcastLayer>& layers);

SimulcastDescription RemoveSimulcast(SessionDescriptionInterface* sd);

}  // namespace webrtc

#endif  // PC_TEST_SIMULCAST_LAYER_UTIL_H_
