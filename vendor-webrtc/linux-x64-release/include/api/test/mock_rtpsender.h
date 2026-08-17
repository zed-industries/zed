/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_RTPSENDER_H_
#define API_TEST_MOCK_RTPSENDER_H_

#include <cstdint>
#include <memory>
#include <string>
#include <type_traits>
#include <vector>

#include "api/crypto/frame_encryptor_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/dtmf_sender_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/make_ref_counted.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/video_codecs/video_encoder_factory.h"
#include "rtc_base/ref_counted_object.h"
#include "test/gmock.h"

namespace webrtc {

class MockRtpSender : public RtpSenderInterface {
 public:
  static scoped_refptr<MockRtpSender> Create() {
    return make_ref_counted<MockRtpSender>();
  }

  MOCK_METHOD(bool, SetTrack, (MediaStreamTrackInterface*), (override));
  MOCK_METHOD(scoped_refptr<MediaStreamTrackInterface>,
              track,
              (),
              (const, override));
  MOCK_METHOD(scoped_refptr<DtlsTransportInterface>,
              dtls_transport,
              (),
              (const, override));
  MOCK_METHOD(uint32_t, ssrc, (), (const, override));
  MOCK_METHOD(webrtc::MediaType, media_type, (), (const, override));
  MOCK_METHOD(std::string, id, (), (const, override));
  MOCK_METHOD(std::vector<std::string>, stream_ids, (), (const, override));
  MOCK_METHOD(void, SetStreams, (const std::vector<std::string>&), (override));
  MOCK_METHOD(std::vector<RtpEncodingParameters>,
              init_send_encodings,
              (),
              (const, override));
  MOCK_METHOD(RtpParameters, GetParameters, (), (const, override));
  MOCK_METHOD(RTCError, SetParameters, (const RtpParameters&), (override));
  MOCK_METHOD(void,
              SetParametersAsync,
              (const RtpParameters&, SetParametersCallback),
              (override));
  MOCK_METHOD(scoped_refptr<DtmfSenderInterface>,
              GetDtmfSender,
              (),
              (const, override));
  MOCK_METHOD(void,
              SetFrameEncryptor,
              (webrtc::scoped_refptr<FrameEncryptorInterface>),
              (override));
  MOCK_METHOD(scoped_refptr<FrameEncryptorInterface>,
              GetFrameEncryptor,
              (),
              (const, override));
  MOCK_METHOD(void,
              SetFrameTransformer,
              (webrtc::scoped_refptr<FrameTransformerInterface>),
              (override));
  MOCK_METHOD(void,
              SetEncoderSelector,
              (std::unique_ptr<VideoEncoderFactory::EncoderSelectorInterface>),
              (override));
  MOCK_METHOD(void, SetObserver, (RtpSenderObserverInterface*), (override));
};

static_assert(!std::is_abstract_v<webrtc::RefCountedObject<MockRtpSender>>, "");
}  // namespace webrtc

#endif  // API_TEST_MOCK_RTPSENDER_H_
