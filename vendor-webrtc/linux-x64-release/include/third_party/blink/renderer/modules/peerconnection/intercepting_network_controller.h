// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_INTERCEPTING_NETWORK_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_INTERCEPTING_NETWORK_CONTROLLER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_transport.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_transport_processor.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cross_thread_task.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier_base.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/api/transport/network_control.h"

namespace blink {

// Implementation of NetworkControllerInterface intercepting calls to methods
// we're interested in, forwarding the rest on to a supplied
// fallback_controller instance.
class InterceptingNetworkController
    : public webrtc::NetworkControllerInterface {
 public:
  explicit InterceptingNetworkController(
      std::unique_ptr<webrtc::NetworkControllerInterface> fallback_controller,
      CrossThreadWeakHandle<RTCRtpTransport> rtp_transport_handle,
      scoped_refptr<base::SequencedTaskRunner> task_runner);

  // Implementation of NetworkControllerInterface.

  // Called when network availability changes.
  webrtc::NetworkControlUpdate OnNetworkAvailability(
      webrtc::NetworkAvailability na) override;
  // Called when the receiving or sending endpoint changes address.
  webrtc::NetworkControlUpdate OnNetworkRouteChange(
      webrtc::NetworkRouteChange nrc) override;
  // Called periodically with a periodicy as specified by
  // NetworkControllerFactoryInterface::GetProcessInterval.
  webrtc::NetworkControlUpdate OnProcessInterval(
      webrtc::ProcessInterval pi) override;

  // Called when remotely calculated bitrate is received.
  webrtc::NetworkControlUpdate OnRemoteBitrateReport(
      webrtc::RemoteBitrateReport rbr) override;
  // Called round trip time has been calculated by protocol specific mechanisms.
  webrtc::NetworkControlUpdate OnRoundTripTimeUpdate(
      webrtc::RoundTripTimeUpdate rttu) override;
  // Called when a packet is sent on the network.
  webrtc::NetworkControlUpdate OnSentPacket(webrtc::SentPacket sp) override;
  // Called when a packet is received from the remote client.
  webrtc::NetworkControlUpdate OnReceivedPacket(
      webrtc::ReceivedPacket rp) override;
  // Called when the stream specific configuration has been updated.
  webrtc::NetworkControlUpdate OnStreamsConfig(
      webrtc::StreamsConfig sc) override;
  // Called when target transfer rate constraints has been changed.
  webrtc::NetworkControlUpdate OnTargetRateConstraints(
      webrtc::TargetRateConstraints trc) override;
  // Called when a protocol specific calculation of packet loss has been made.
  webrtc::NetworkControlUpdate OnTransportLossReport(
      webrtc::TransportLossReport tlr) override;
  // Called with per packet feedback regarding receive time.
  webrtc::NetworkControlUpdate OnTransportPacketsFeedback(
      webrtc::TransportPacketsFeedback tpf) override;
  // Called with network state estimate updates.
  webrtc::NetworkControlUpdate OnNetworkStateEstimate(
      webrtc::NetworkStateEstimate nse) override;

 private:
  // Ref counted object which is given a reference to the
  // RTCRtpTransportProcessor once it's created on a worker, then takes BWE
  // signals from an InterceptingNetworkController and post them to the
  // processor on the JS Worker thread.
  class FeedbackProviderImpl : public FeedbackProvider {
   public:
    FeedbackProviderImpl() = default;
    ~FeedbackProviderImpl() override = default;

    // Impl of FeedbackProvider.
    void SetProcessor(CrossThreadWeakHandle<RTCRtpTransportProcessor>
                          rtp_transport_processor_handle,
                      scoped_refptr<base::SequencedTaskRunner>
                          rtp_transport_processor_task_runner) override;
    void SetCustomMaxBitrateBps(uint64_t custom_max_bitrate_bps) override {
      // Called on the RTCRtpTransportProcessor's JS thread.
      base::AutoLock mutex(custom_bitrate_lock_);
      custom_max_bitrate_bps_ = custom_max_bitrate_bps;
    }

    // Methods called by InterceptingNetworkController.
    void OnFeedback(webrtc::TransportPacketsFeedback feedback);
    void OnSentPacket(webrtc::SentPacket sp);

    std::optional<uint64_t> CustomMaxBitrateBps() {
      // Called on a WebRTC thread.
      base::AutoLock mutex(custom_bitrate_lock_);
      return custom_max_bitrate_bps_;
    }

   private:
    void OnFeedbackOnDestinationTaskRunner(
        webrtc::TransportPacketsFeedback feedback,
        RTCRtpTransportProcessor* rtp_transport);
    void OnSentPacketOnDestinationTaskRunner(
        webrtc::SentPacket sp,
        RTCRtpTransportProcessor* rtp_transport);

    base::Lock processor_lock_;
    // Store just a CrossThreadWeakHandle pointing at an RTCRtpTransport, as
    // we're constructed on a WebRTC thread, only unwrapping in tasks posted to
    // the blink task runner which owns the RTCRtpTransport object.
    std::optional<CrossThreadWeakHandle<RTCRtpTransportProcessor>>
        rtp_transport_processor_handle_ GUARDED_BY(processor_lock_);
    scoped_refptr<base::SequencedTaskRunner>
        rtp_transport_processor_task_runner_ GUARDED_BY(processor_lock_);

    base::Lock custom_bitrate_lock_;
    std::optional<uint64_t> custom_max_bitrate_bps_
        GUARDED_BY(custom_bitrate_lock_);
  };

  const std::unique_ptr<webrtc::NetworkControllerInterface>
      fallback_controller_;
  const scoped_refptr<FeedbackProviderImpl> feedback_provider_;
};
}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_INTERCEPTING_NETWORK_CONTROLLER_H_
