// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_SENDER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_SENDER_H_

#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/api/rtp_parameters.h"
#include "third_party/webrtc/api/rtp_sender_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/test/mock_rtpsender.h"
#include "third_party/webrtc/rtc_base/ref_count.h"

namespace blink {

class MockRtpSender : public webrtc::RefCountedObject<webrtc::MockRtpSender> {};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_TESTING_MOCK_RTP_SENDER_H_
