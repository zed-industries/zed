/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_LINK_RTCP_OBSERVER_H_
#define MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_LINK_RTCP_OBSERVER_H_

#include "api/array_view.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/rtp_rtcp/include/report_block_data.h"
#include "modules/rtp_rtcp/include/rtp_rtcp_defines.h"
#include "modules/rtp_rtcp/source/rtcp_packet/congestion_control_feedback.h"
#include "modules/rtp_rtcp/source/rtcp_packet/transport_feedback.h"
#include "test/gmock.h"

namespace webrtc {

class MockNetworkLinkRtcpObserver : public NetworkLinkRtcpObserver {
 public:
  MOCK_METHOD(void,
              OnRttUpdate,
              (Timestamp receive_time, TimeDelta rtt),
              (override));
  MOCK_METHOD(void,
              OnTransportFeedback,
              (Timestamp receive_time, const rtcp::TransportFeedback& feedback),
              (override));
  MOCK_METHOD(void,
              OnCongestionControlFeedback,
              (Timestamp receive_time,
               const rtcp::CongestionControlFeedback& feedback),
              (override));
  MOCK_METHOD(void,
              OnReceiverEstimatedMaxBitrate,
              (Timestamp receive_time, DataRate bitrate),
              (override));
  MOCK_METHOD(void,
              OnReport,
              (Timestamp receive_time,
               ArrayView<const ReportBlockData> report_blocks),
              (override));
};

}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_MOCKS_MOCK_NETWORK_LINK_RTCP_OBSERVER_H_
