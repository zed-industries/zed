// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_RECEIVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_RECEIVER_H_

#include <optional>

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/api/rtp_parameters.h"
#include "third_party/webrtc/api/rtp_receiver_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/test/mock_rtpreceiver.h"
#include "third_party/webrtc/rtc_base/ref_count.h"

namespace blink {

class MockRtpReceiver
    : public webrtc::RefCountedObject<webrtc::MockRtpReceiver> {};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_RECEIVER_H_
