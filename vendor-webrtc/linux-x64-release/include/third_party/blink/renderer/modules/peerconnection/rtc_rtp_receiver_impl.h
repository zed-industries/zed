// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_IMPL_H_

#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_receiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/api/peer_connection_interface.h"
#include "third_party/webrtc/api/rtp_receiver_interface.h"
#include "third_party/webrtc/api/stats/rtc_stats.h"

namespace blink {

class RTCEncodedAudioStreamTransformer;
class RTCEncodedVideoStreamTransformer;

// This class represents the state of a receiver; a snapshot of what a
// webrtc-layer receiver looked like when it was inspected on the signaling
// thread such that this information can be moved to the main thread in a single
// PostTask. It is used to surface state changes to make the blink-layer
// receiver up-to-date.
//
// Blink objects live on the main thread and webrtc objects live on the
// signaling thread. If multiple asynchronous operations begin execution on the
// main thread they are posted and executed in order on the signaling thread.
// For example, operation A and operation B are called in JavaScript. When A is
// done on the signaling thread, webrtc object states will be updated. A
// callback is posted to the main thread so that blink objects can be updated to
// match the result of operation A. But if callback A tries to inspect the
// webrtc objects from the main thread this requires posting back to the
// signaling thread and waiting, which also includes waiting for the previously
// posted task: operation B. Inspecting the webrtc object like this does not
// guarantee you to get the state of operation A.
//
// As such, all state changes associated with an operation have to be surfaced
// in the same callback. This includes copying any states into a separate object
// so that it can be inspected on the main thread without any additional thread
// hops.
//
// The RtpReceiverState is a snapshot of what the webrtc::RtpReceiverInterface
// looked like when the RtpReceiverState was created on the signaling thread. It
// also takes care of initializing track adapters, such that we have access to a
// blink track corresponding to the webrtc track of the receiver.
//
// Except for initialization logic and operator=(), the RtpReceiverState is
// immutable and only accessible on the main thread.
//
// TODO(crbug.com/787254): Consider merging RTCRtpReceiverImpl and
// RTCRtpReceiver, and removing RTCRtpReceiverPlatform when all its clients are
// Onion soup'ed. Also, move away from using std::vector.
class MODULES_EXPORT RtpReceiverState {
 public:
  RtpReceiverState(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      scoped_refptr<webrtc::RtpReceiverInterface> webrtc_receiver,
      std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>
          track_ref,
      std::vector<std::string> stream_ids);
  RtpReceiverState(RtpReceiverState&&);
  RtpReceiverState(const RtpReceiverState&) = delete;
  ~RtpReceiverState();

  // This is intended to be used for moving the object from the signaling thread
  // to the main thread and as such has no thread checks. Once moved to the main
  // this should only be invoked on the main thread.
  RtpReceiverState& operator=(RtpReceiverState&&);
  RtpReceiverState& operator=(const RtpReceiverState&) = delete;

  bool is_initialized() const;
  void Initialize();

  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner() const;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner() const;
  scoped_refptr<webrtc::RtpReceiverInterface> webrtc_receiver() const;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> webrtc_dtls_transport()
      const;
  webrtc::DtlsTransportInformation webrtc_dtls_transport_information() const;

  const std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>&
  track_ref() const;
  const std::vector<std::string>& stream_ids() const;

 private:
  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;
  scoped_refptr<webrtc::RtpReceiverInterface> webrtc_receiver_;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> webrtc_dtls_transport_;
  webrtc::DtlsTransportInformation webrtc_dtls_transport_information_;
  bool is_initialized_;
  std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>
      track_ref_;
  std::vector<std::string> stream_ids_ ALLOW_DISCOURAGED_TYPE(
      "Avoids conversions when passed from/to webrtc API");
};

// Used to surface |webrtc::RtpReceiverInterface| to blink. Multiple
// |RTCRtpReceiverImpl|s could reference the same webrtc receiver; |id| is the
// value of the pointer to the webrtc receiver.
class MODULES_EXPORT RTCRtpReceiverImpl : public RTCRtpReceiverPlatform {
 public:
  static uintptr_t getId(
      const webrtc::RtpReceiverInterface* webrtc_rtp_receiver);

  RTCRtpReceiverImpl(webrtc::scoped_refptr<webrtc::PeerConnectionInterface>
                         native_peer_connection,
                     RtpReceiverState state,
                     bool require_encoded_insertable_streams,
                     std::unique_ptr<webrtc::Metronome> decode_metronome);
  RTCRtpReceiverImpl(const RTCRtpReceiverImpl& other);
  ~RTCRtpReceiverImpl() override;

  RTCRtpReceiverImpl& operator=(const RTCRtpReceiverImpl& other);

  const RtpReceiverState& state() const;
  void set_state(RtpReceiverState state);

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
  RTCEncodedAudioStreamTransformer* GetEncodedAudioStreamTransformer()
      const override;
  RTCEncodedVideoStreamTransformer* GetEncodedVideoStreamTransformer()
      const override;

 private:
  class RTCRtpReceiverInternal;
  struct RTCRtpReceiverInternalTraits;

  scoped_refptr<RTCRtpReceiverInternal> internal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_RECEIVER_IMPL_H_
