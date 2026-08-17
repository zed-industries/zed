/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_RTP_TRANSCEIVER_H_
#define API_TEST_MOCK_RTP_TRANSCEIVER_H_

#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "api/make_ref_counted.h"
#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_direction.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "test/gmock.h"

namespace webrtc {

class MockRtpTransceiver : public RtpTransceiverInterface {
 public:
  MockRtpTransceiver() = default;

  static scoped_refptr<MockRtpTransceiver> Create() {
    return make_ref_counted<MockRtpTransceiver>();
  }

  MOCK_METHOD(webrtc::MediaType, media_type, (), (const, override));
  MOCK_METHOD(std::optional<std::string>, mid, (), (const, override));
  MOCK_METHOD(scoped_refptr<RtpSenderInterface>, sender, (), (const, override));
  MOCK_METHOD(scoped_refptr<RtpReceiverInterface>,
              receiver,
              (),
              (const, override));
  MOCK_METHOD(bool, stopped, (), (const, override));
  MOCK_METHOD(bool, stopping, (), (const, override));
  MOCK_METHOD(RtpTransceiverDirection, direction, (), (const, override));
  MOCK_METHOD(void,
              SetDirection,
              (RtpTransceiverDirection new_direction),
              (override));
  MOCK_METHOD(RTCError,
              SetDirectionWithError,
              (RtpTransceiverDirection new_direction),
              (override));
  MOCK_METHOD(std::optional<RtpTransceiverDirection>,
              current_direction,
              (),
              (const, override));
  MOCK_METHOD(std::optional<RtpTransceiverDirection>,
              fired_direction,
              (),
              (const, override));
  MOCK_METHOD(RTCError, StopStandard, (), (override));
  MOCK_METHOD(void, StopInternal, (), (override));
  MOCK_METHOD(void, Stop, (), (override));
  MOCK_METHOD(RTCError,
              SetCodecPreferences,
              (webrtc::ArrayView<RtpCodecCapability> codecs),
              (override));
  MOCK_METHOD(std::vector<RtpCodecCapability>,
              codec_preferences,
              (),
              (const, override));
  MOCK_METHOD(std::vector<RtpHeaderExtensionCapability>,
              GetHeaderExtensionsToNegotiate,
              (),
              (const, override));
  MOCK_METHOD(std::vector<RtpHeaderExtensionCapability>,
              GetNegotiatedHeaderExtensions,
              (),
              (const, override));
  MOCK_METHOD(
      webrtc::RTCError,
      SetHeaderExtensionsToNegotiate,
      (webrtc::ArrayView<const RtpHeaderExtensionCapability> header_extensions),
      (override));
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_RTP_TRANSCEIVER_H_
