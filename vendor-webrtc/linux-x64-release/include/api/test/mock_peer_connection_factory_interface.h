/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_MOCK_PEER_CONNECTION_FACTORY_INTERFACE_H_
#define API_TEST_MOCK_PEER_CONNECTION_FACTORY_INTERFACE_H_

#include <cstdint>
#include <cstdio>
#include <string>

#include "absl/strings/string_view.h"
#include "api/audio_options.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "rtc_base/ref_counted_object.h"
#include "test/gmock.h"

namespace webrtc {

class MockPeerConnectionFactoryInterface
    : public RefCountedObject<webrtc::PeerConnectionFactoryInterface> {
 public:
  static scoped_refptr<MockPeerConnectionFactoryInterface> Create() {
    return scoped_refptr<MockPeerConnectionFactoryInterface>(
        new MockPeerConnectionFactoryInterface());
  }

  MOCK_METHOD(void, SetOptions, (const Options&), (override));
  MOCK_METHOD(RTCErrorOr<scoped_refptr<PeerConnectionInterface>>,
              CreatePeerConnectionOrError,
              (const PeerConnectionInterface::RTCConfiguration&,
               PeerConnectionDependencies),
              (override));
  MOCK_METHOD(RtpCapabilities,
              GetRtpSenderCapabilities,
              (webrtc::MediaType),
              (const, override));
  MOCK_METHOD(RtpCapabilities,
              GetRtpReceiverCapabilities,
              (webrtc::MediaType),
              (const, override));
  MOCK_METHOD(scoped_refptr<MediaStreamInterface>,
              CreateLocalMediaStream,
              (const std::string&),
              (override));
  MOCK_METHOD(scoped_refptr<AudioSourceInterface>,
              CreateAudioSource,
              (const webrtc::AudioOptions&),
              (override));
  MOCK_METHOD(scoped_refptr<VideoTrackInterface>,
              CreateVideoTrack,
              (const std::string&, VideoTrackSourceInterface*),
              (override));
  MOCK_METHOD(scoped_refptr<VideoTrackInterface>,
              CreateVideoTrack,
              (webrtc::scoped_refptr<VideoTrackSourceInterface>,
               absl::string_view),
              (override));
  MOCK_METHOD(scoped_refptr<AudioTrackInterface>,
              CreateAudioTrack,
              (const std::string&, AudioSourceInterface*),
              (override));
  MOCK_METHOD(bool, StartAecDump, (FILE*, int64_t), (override));
  MOCK_METHOD(void, StopAecDump, (), (override));

 protected:
  MockPeerConnectionFactoryInterface() = default;
};

}  // namespace webrtc

#endif  // API_TEST_MOCK_PEER_CONNECTION_FACTORY_INTERFACE_H_
