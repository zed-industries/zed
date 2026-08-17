// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_FAKE_RTC_RTP_TRANSCEIVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_FAKE_RTC_RTP_TRANSCEIVER_IMPL_H_

#include <memory>

#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_dtmf_sender_handler.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_receiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_sender_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_source.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// TODO(https://crbug.com/868868): Similar methods to this exist in many blink
// unittests. Move to a separate file and reuse it in all of them.
MediaStreamComponent* CreateMediaStreamComponent(
    const String& id,
    scoped_refptr<base::SingleThreadTaskRunner> task_runner);

class FakeRTCRtpSenderImpl : public blink::RTCRtpSenderPlatform {
 public:
  FakeRTCRtpSenderImpl(String track_id,
                       Vector<String> stream_ids,
                       scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  FakeRTCRtpSenderImpl(const FakeRTCRtpSenderImpl&);
  ~FakeRTCRtpSenderImpl() override;
  FakeRTCRtpSenderImpl& operator=(const FakeRTCRtpSenderImpl&);

  std::unique_ptr<blink::RTCRtpSenderPlatform> ShallowCopy() const override;
  uintptr_t Id() const override;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> DtlsTransport()
      override;
  webrtc::DtlsTransportInformation DtlsTransportInformation() override;
  MediaStreamComponent* Track() const override;
  Vector<String> StreamIds() const override;
  void ReplaceTrack(MediaStreamComponent* with_track,
                    blink::RTCVoidRequest* request) override;
  std::unique_ptr<blink::RtcDtmfSenderHandler> GetDtmfSender() const override;
  std::unique_ptr<webrtc::RtpParameters> GetParameters() const override;
  void SetParameters(Vector<webrtc::RtpEncodingParameters>,
                     std::optional<webrtc::DegradationPreference>,
                     blink::RTCVoidRequest*) override;
  void GetStats(RTCStatsReportCallback) override;
  void SetStreams(const Vector<String>& stream_ids) override;

 private:
  String track_id_;
  Vector<String> stream_ids_;
  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

class FakeRTCRtpReceiverImpl : public RTCRtpReceiverPlatform {
 public:
  FakeRTCRtpReceiverImpl(
      const String& track_id,
      Vector<String> stream_ids,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);
  FakeRTCRtpReceiverImpl(const FakeRTCRtpReceiverImpl&);
  ~FakeRTCRtpReceiverImpl() override;
  FakeRTCRtpReceiverImpl& operator=(const FakeRTCRtpReceiverImpl&);

  std::unique_ptr<RTCRtpReceiverPlatform> ShallowCopy() const override;
  uintptr_t Id() const override;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> DtlsTransport()
      override;
  webrtc::DtlsTransportInformation DtlsTransportInformation() override;
  MediaStreamComponent* Track() const override;
  Vector<String> StreamIds() const override;
  Vector<std::unique_ptr<RTCRtpSource>> GetSources() override;
  void GetStats(RTCStatsReportCallback) override;
  std::unique_ptr<webrtc::RtpParameters> GetParameters() const override;
  void SetJitterBufferMinimumDelay(
      std::optional<double> delay_seconds) override;

 private:
  Persistent<MediaStreamComponent> component_;
  Vector<String> stream_ids_;
};

class FakeRTCRtpTransceiverImpl : public RTCRtpTransceiverPlatform {
 public:
  FakeRTCRtpTransceiverImpl(
      const String& mid,
      FakeRTCRtpSenderImpl sender,
      FakeRTCRtpReceiverImpl receiver,
      webrtc::RtpTransceiverDirection direction,
      std::optional<webrtc::RtpTransceiverDirection> current_direction);
  ~FakeRTCRtpTransceiverImpl() override;

  uintptr_t Id() const override;
  String Mid() const override;
  std::unique_ptr<blink::RTCRtpSenderPlatform> Sender() const override;
  std::unique_ptr<RTCRtpReceiverPlatform> Receiver() const override;
  webrtc::RtpTransceiverDirection Direction() const override;
  webrtc::RTCError SetDirection(
      webrtc::RtpTransceiverDirection direction) override;
  std::optional<webrtc::RtpTransceiverDirection> CurrentDirection()
      const override;
  std::optional<webrtc::RtpTransceiverDirection> FiredDirection()
      const override;

  webrtc::RTCError Stop() override;
  webrtc::RTCError SetCodecPreferences(
      Vector<webrtc::RtpCodecCapability>) override;
  webrtc::RTCError SetHeaderExtensionsToNegotiate(
      Vector<webrtc::RtpHeaderExtensionCapability> header_extensions) override;
  Vector<webrtc::RtpHeaderExtensionCapability> GetNegotiatedHeaderExtensions()
      const override;
  Vector<webrtc::RtpHeaderExtensionCapability> GetHeaderExtensionsToNegotiate()
      const override;

 private:
  String mid_;
  FakeRTCRtpSenderImpl sender_;
  FakeRTCRtpReceiverImpl receiver_;
  webrtc::RtpTransceiverDirection direction_;
  std::optional<webrtc::RtpTransceiverDirection> current_direction_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_FAKE_RTC_RTP_TRANSCEIVER_IMPL_H_
