/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VOIP_TEST_MOCK_VOIP_ENGINE_H_
#define API_VOIP_TEST_MOCK_VOIP_ENGINE_H_

#include <cstdint>
#include <map>
#include <optional>

#include "api/array_view.h"
#include "api/audio_codecs/audio_format.h"
#include "api/voip/voip_base.h"
#include "api/voip/voip_codec.h"
#include "api/voip/voip_dtmf.h"
#include "api/voip/voip_engine.h"
#include "api/voip/voip_network.h"
#include "api/voip/voip_statistics.h"
#include "api/voip/voip_volume_control.h"
#include "test/gmock.h"

namespace webrtc {

class MockVoipBase : public VoipBase {
 public:
  MOCK_METHOD(ChannelId,
              CreateChannel,
              (Transport*, std::optional<uint32_t>),
              (override));
  MOCK_METHOD(VoipResult, ReleaseChannel, (ChannelId), (override));
  MOCK_METHOD(VoipResult, StartSend, (ChannelId), (override));
  MOCK_METHOD(VoipResult, StopSend, (ChannelId), (override));
  MOCK_METHOD(VoipResult, StartPlayout, (ChannelId), (override));
  MOCK_METHOD(VoipResult, StopPlayout, (ChannelId), (override));
};

class MockVoipCodec : public VoipCodec {
 public:
  MOCK_METHOD(VoipResult,
              SetSendCodec,
              (ChannelId, int, const SdpAudioFormat&),
              (override));
  MOCK_METHOD(VoipResult,
              SetReceiveCodecs,
              (ChannelId, (const std::map<int, SdpAudioFormat>&)),
              (override));
};

class MockVoipDtmf : public VoipDtmf {
 public:
  MOCK_METHOD(VoipResult,
              RegisterTelephoneEventType,
              (ChannelId, int, int),
              (override));
  MOCK_METHOD(VoipResult,
              SendDtmfEvent,
              (ChannelId, DtmfEvent, int),
              (override));
};

class MockVoipNetwork : public VoipNetwork {
 public:
  MOCK_METHOD(VoipResult,
              ReceivedRTPPacket,
              (ChannelId channel_id,
               webrtc::ArrayView<const uint8_t> rtp_packet),
              (override));
  MOCK_METHOD(VoipResult,
              ReceivedRTCPPacket,
              (ChannelId channel_id,
               webrtc::ArrayView<const uint8_t> rtcp_packet),
              (override));
};

class MockVoipStatistics : public VoipStatistics {
 public:
  MOCK_METHOD(VoipResult,
              GetIngressStatistics,
              (ChannelId, IngressStatistics&),
              (override));
  MOCK_METHOD(VoipResult,
              GetChannelStatistics,
              (ChannelId channel_id, ChannelStatistics&),
              (override));
};

class MockVoipVolumeControl : public VoipVolumeControl {
 public:
  MOCK_METHOD(VoipResult, SetInputMuted, (ChannelId, bool), (override));

  MOCK_METHOD(VoipResult,
              GetInputVolumeInfo,
              (ChannelId, VolumeInfo&),
              (override));
  MOCK_METHOD(VoipResult,
              GetOutputVolumeInfo,
              (ChannelId, VolumeInfo&),
              (override));
};

class MockVoipEngine : public VoipEngine {
 public:
  VoipBase& Base() override { return base_; }
  VoipNetwork& Network() override { return network_; }
  VoipCodec& Codec() override { return codec_; }
  VoipDtmf& Dtmf() override { return dtmf_; }
  VoipStatistics& Statistics() override { return statistics_; }
  VoipVolumeControl& VolumeControl() override { return volume_; }

  // Direct access to underlying members are required for testing.
  MockVoipBase base_;
  MockVoipNetwork network_;
  MockVoipCodec codec_;
  MockVoipDtmf dtmf_;
  MockVoipStatistics statistics_;
  MockVoipVolumeControl volume_;
};

}  // namespace webrtc

#endif  // API_VOIP_TEST_MOCK_VOIP_ENGINE_H_
