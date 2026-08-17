/*
 *  Copyright 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef PC_TEST_MOCK_VOICE_MEDIA_RECEIVE_CHANNEL_INTERFACE_H_
#define PC_TEST_MOCK_VOICE_MEDIA_RECEIVE_CHANNEL_INTERFACE_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <set>
#include <type_traits>
#include <vector>

#include "api/call/audio_sink.h"
#include "api/crypto/frame_decryptor_interface.h"
#include "api/frame_transformer_interface.h"
#include "api/media_types.h"
#include "api/rtp_headers.h"
#include "api/rtp_parameters.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/rtp_source.h"
#include "media/base/media_channel.h"
#include "media/base/stream_params.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "test/gmock.h"

namespace webrtc {

class MockVoiceMediaReceiveChannelInterface
    : public VoiceMediaReceiveChannelInterface {
 public:
  MockVoiceMediaReceiveChannelInterface() {
    ON_CALL(*this, AsVoiceReceiveChannel).WillByDefault(testing::Return(this));
  }

  // VoiceMediaReceiveChannelInterface
  MOCK_METHOD(bool,
              SetReceiverParameters,
              (const webrtc::AudioReceiverParameters& params),
              (override));
  MOCK_METHOD(RtpParameters,
              GetRtpReceiverParameters,
              (uint32_t ssrc),
              (const, override));
  MOCK_METHOD(std::vector<RtpSource>,
              GetSources,
              (uint32_t ssrc),
              (const, override));
  MOCK_METHOD(RtpParameters,
              GetDefaultRtpReceiveParameters,
              (),
              (const, override));
  MOCK_METHOD(void, SetPlayout, (bool playout), (override));
  MOCK_METHOD(bool,
              SetOutputVolume,
              (uint32_t ssrc, double volume),
              (override));
  MOCK_METHOD(bool, SetDefaultOutputVolume, (double volume), (override));
  MOCK_METHOD(void,
              SetRawAudioSink,
              (uint32_t ssrc, std::unique_ptr<AudioSinkInterface> sink),
              (override));
  MOCK_METHOD(void,
              SetDefaultRawAudioSink,
              (std::unique_ptr<AudioSinkInterface> sink),
              (override));
  MOCK_METHOD(bool,
              GetStats,
              (webrtc::VoiceMediaReceiveInfo * stats, bool reset_legacy),
              (override));
  MOCK_METHOD(::webrtc::RtcpMode, RtcpMode, (), (const, override));
  MOCK_METHOD(void, SetRtcpMode, (::webrtc::RtcpMode mode), (override));
  MOCK_METHOD(void, SetReceiveNackEnabled, (bool enabled), (override));
  MOCK_METHOD(void, SetReceiveNonSenderRttEnabled, (bool enabled), (override));

  // MediaReceiveChannelInterface
  MOCK_METHOD(VideoMediaReceiveChannelInterface*,
              AsVideoReceiveChannel,
              (),
              (override));
  MOCK_METHOD(VoiceMediaReceiveChannelInterface*,
              AsVoiceReceiveChannel,
              (),
              (override));
  MOCK_METHOD(MediaType, media_type, (), (const, override));
  MOCK_METHOD(bool,
              AddRecvStream,
              (const webrtc::StreamParams& sp),
              (override));
  MOCK_METHOD(bool, RemoveRecvStream, (uint32_t ssrc), (override));
  MOCK_METHOD(void, ResetUnsignaledRecvStream, (), (override));
  MOCK_METHOD(void,
              SetInterface,
              (webrtc::MediaChannelNetworkInterface * iface),
              (override));
  MOCK_METHOD(void,
              OnPacketReceived,
              (const RtpPacketReceived& packet),
              (override));
  MOCK_METHOD(std::optional<uint32_t>,
              GetUnsignaledSsrc,
              (),
              (const, override));
  MOCK_METHOD(void,
              ChooseReceiverReportSsrc,
              (const std::set<uint32_t>& choices),
              (override));
  MOCK_METHOD(void, OnDemuxerCriteriaUpdatePending, (), (override));
  MOCK_METHOD(void, OnDemuxerCriteriaUpdateComplete, (), (override));
  MOCK_METHOD(void,
              SetFrameDecryptor,
              (uint32_t ssrc,
               scoped_refptr<FrameDecryptorInterface> frame_decryptor),
              (override));
  MOCK_METHOD(void,
              SetDepacketizerToDecoderFrameTransformer,
              (uint32_t ssrc,
               scoped_refptr<FrameTransformerInterface> frame_transformer),
              (override));
  MOCK_METHOD(bool,
              SetBaseMinimumPlayoutDelayMs,
              (uint32_t ssrc, int delay_ms),
              (override));
  MOCK_METHOD(std::optional<int>,
              GetBaseMinimumPlayoutDelayMs,
              (uint32_t ssrc),
              (const, override));
};

static_assert(!std::is_abstract_v<MockVoiceMediaReceiveChannelInterface>, "");

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::MockVoiceMediaReceiveChannelInterface;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_TEST_MOCK_VOICE_MEDIA_RECEIVE_CHANNEL_INTERFACE_H_
