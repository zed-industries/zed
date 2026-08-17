/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_NETWORK_EMULATION_CREATE_CROSS_TRAFFIC_H_
#define API_TEST_NETWORK_EMULATION_CREATE_CROSS_TRAFFIC_H_

#include <memory>

#include "api/test/network_emulation/cross_traffic.h"
#include "api/test/network_emulation_manager.h"

namespace webrtc {

// This API is still in development and can be changed without prior notice.

std::unique_ptr<CrossTrafficGenerator> CreateRandomWalkCrossTraffic(
    CrossTrafficRoute* traffic_route,
    RandomWalkConfig config);

std::unique_ptr<CrossTrafficGenerator> CreatePulsedPeaksCrossTraffic(
    CrossTrafficRoute* traffic_route,
    PulsedPeaksConfig config);

std::unique_ptr<CrossTrafficGenerator> CreateFakeTcpCrossTraffic(
    EmulatedRoute* send_route,
    EmulatedRoute* ret_route,
    FakeTcpConfig config);

}  // namespace webrtc

#endif  // API_TEST_NETWORK_EMULATION_CREATE_CROSS_TRAFFIC_H_
