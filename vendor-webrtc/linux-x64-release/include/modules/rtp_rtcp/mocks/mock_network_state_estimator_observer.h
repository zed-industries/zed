/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_STATE_ESTIMATOR_OBSERVER_H_
#define MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_STATE_ESTIMATOR_OBSERVER_H_

#include "api/transport/network_types.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "test/gmock.h"

namespace webrtc {

class MockNetworkStateEstimateObserver : public NetworkStateEstimateObserver {
 public:
  MOCK_METHOD(void,
              OnRemoteNetworkEstimate,
              (NetworkStateEstimate estimate),
              (override));
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_STATE_ESTIMATOR_OBSERVER_H_
