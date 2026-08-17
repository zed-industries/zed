/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_MOCK_CHANNEL_INTERFACE_H_
#define PC_TEST_MOCK_CHANNEL_INTERFACE_H_

#include <functional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/jsep.h"
#include "api/media_types.h"
#include "media/base/media_channel.h"
#include "media/base/stream_params.h"
#include "pc/channel_interface.h"
#include "pc/rtp_transport_internal.h"
#include "test/gmock.h"

namespace webrtc {

// Mock class for BaseChannel.
// Use this class in unit tests to avoid dependency on a specific
// implementation of BaseChannel.
class MockChannelInterface : public ChannelInterface {
 public:
  MOCK_METHOD(MediaType, media_type, (), (const, override));
  MOCK_METHOD(VideoChannel*, AsVideoChannel, (), (override));
  MOCK_METHOD(VoiceChannel*, AsVoiceChannel, (), (override));
  MOCK_METHOD(MediaSendChannelInterface*, media_send_channel, (), (override));
  MOCK_METHOD(VoiceMediaSendChannelInterface*,
              voice_media_send_channel,
              (),
              (override));
  MOCK_METHOD(VideoMediaSendChannelInterface*,
              video_media_send_channel,
              (),
              (override));
  MOCK_METHOD(MediaReceiveChannelInterface*,
              media_receive_channel,
              (),
              (override));
  MOCK_METHOD(VoiceMediaReceiveChannelInterface*,
              voice_media_receive_channel,
              (),
              (override));
  MOCK_METHOD(VideoMediaReceiveChannelInterface*,
              video_media_receive_channel,
              (),
              (override));
  MOCK_METHOD(absl::string_view, transport_name, (), (const, override));
  MOCK_METHOD(const std::string&, mid, (), (const, override));
  MOCK_METHOD(void, Enable, (bool), (override));
  MOCK_METHOD(void,
              SetFirstPacketReceivedCallback,
              (std::function<void()>),
              (override));
  MOCK_METHOD(void,
              SetFirstPacketSentCallback,
              (std::function<void()>),
              (override));
  MOCK_METHOD(bool,
              SetLocalContent,
              (const webrtc::MediaContentDescription*, SdpType, std::string&),
              (override));
  MOCK_METHOD(bool,
              SetRemoteContent,
              (const webrtc::MediaContentDescription*, SdpType, std::string&),
              (override));
  MOCK_METHOD(bool, SetPayloadTypeDemuxingEnabled, (bool), (override));
  MOCK_METHOD(const std::vector<StreamParams>&,
              local_streams,
              (),
              (const, override));
  MOCK_METHOD(const std::vector<StreamParams>&,
              remote_streams,
              (),
              (const, override));
  MOCK_METHOD(bool, SetRtpTransport, (RtpTransportInternal*), (override));
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::MockChannelInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_TEST_MOCK_CHANNEL_INTERFACE_H_
