/*
 *  Copyright 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_MOCK_RTP_RECEIVER_INTERNAL_H_
#define PC_TEST_MOCK_RTP_RECEIVER_INTERNAL_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "api/crypto/frame_decryptor_interface.h"
#include "api/dtls_transport_interface.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"
#include "media/base/media_channel.h"
#include "pc/rtp_receiver.h"
#include "test/gmock.h"

namespace webrtc {

// The definition of MockRtpReceiver is copied in to avoid multiple inheritance.
class MockRtpReceiverInternal : public RtpReceiverInternal {
 public:
  // RtpReceiverInterface methods.
  MOCK_METHOD(scoped_refptr<MediaStreamTrackInterface>,
              track,
              (),
              (const, override));
  MOCK_METHOD(scoped_refptr<DtlsTransportInterface>,
              dtls_transport,
              (),
              (const, override));
  MOCK_METHOD(std::vector<std::string>, stream_ids, (), (const, override));
  MOCK_METHOD(std::vector<scoped_refptr<MediaStreamInterface>>,
              streams,
              (),
              (const, override));
  MOCK_METHOD(webrtc::MediaType, media_type, (), (const, override));
  MOCK_METHOD(std::string, id, (), (const, override));
  MOCK_METHOD(RtpParameters, GetParameters, (), (const, override));
  MOCK_METHOD(void, SetObserver, (RtpReceiverObserverInterface*), (override));
  MOCK_METHOD(void,
              SetJitterBufferMinimumDelay,
              (std::optional<double>),
              (override));
  MOCK_METHOD(std::vector<RtpSource>, GetSources, (), (const, override));
  MOCK_METHOD(void,
              SetFrameDecryptor,
              (webrtc::scoped_refptr<FrameDecryptorInterface>),
              (override));
  MOCK_METHOD(scoped_refptr<FrameDecryptorInterface>,
              GetFrameDecryptor,
              (),
              (const, override));

  // RtpReceiverInternal methods.
  MOCK_METHOD(void, Stop, (), (override));
  MOCK_METHOD(void,
              SetMediaChannel,
              (webrtc::MediaReceiveChannelInterface*),
              (override));
  MOCK_METHOD(void, SetupMediaChannel, (uint32_t), (override));
  MOCK_METHOD(void, SetupUnsignaledMediaChannel, (), (override));
  MOCK_METHOD(std::optional<uint32_t>, ssrc, (), (const, override));
  MOCK_METHOD(void, NotifyFirstPacketReceived, (), (override));
  MOCK_METHOD(void, set_stream_ids, (std::vector<std::string>), (override));
  MOCK_METHOD(void,
              set_transport,
              (webrtc::scoped_refptr<DtlsTransportInterface>),
              (override));
  MOCK_METHOD(void,
              SetStreams,
              (const std::vector<webrtc::scoped_refptr<MediaStreamInterface>>&),
              (override));
  MOCK_METHOD(int, AttachmentId, (), (const, override));
};

}  // namespace webrtc

#endif  // PC_TEST_MOCK_RTP_RECEIVER_INTERNAL_H_
