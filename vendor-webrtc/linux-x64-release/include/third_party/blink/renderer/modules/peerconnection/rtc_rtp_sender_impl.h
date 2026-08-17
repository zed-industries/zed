// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENDER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENDER_IMPL_H_

#include <memory>
#include <string>
#include <vector>

#include "base/functional/callback.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_sender_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"
#include "third_party/webrtc/api/peer_connection_interface.h"
#include "third_party/webrtc/api/rtp_sender_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"

namespace blink {

class RTCEncodedAudioStreamTransformer;
class RTCEncodedVideoStreamTransformer;

// This class represents the state of a sender; a snapshot of what a
// webrtc-layer sender looked like when it was inspected on the signaling thread
// such that this information can be moved to the main thread in a single
// PostTask. It is used to surface state changes to make the blink-layer sender
// up-to-date.
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
// The RtpSenderState is a snapshot of what the webrtc::RtpSenderInterface
// looked like when the RtpSenderState was created on the signaling thread. It
// also takes care of initializing track adapters, such that we have access to a
// blink track corresponding to the webrtc track of the sender.
//
// Except for initialization logic and operator=(), the RtpSenderState is
// immutable and only accessible on the main thread.
//
// TODO(crbug.com/787254): Consider merging RTCRtpSenderImpl and RTCRtpReceiver,
// and removing RTCRtpSenderPlatform when all its clients are Onion soup'ed.
// Also, move away from using std::vector.
class MODULES_EXPORT RtpSenderState {
 public:
  RtpSenderState(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::RtpSenderInterface> webrtc_sender,
      std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>
          track_ref,
      std::vector<std::string> stream_ids);
  RtpSenderState(RtpSenderState&&);
  RtpSenderState(const RtpSenderState&) = delete;
  ~RtpSenderState();

  // This is intended to be used for moving the object from the signaling thread
  // to the main thread and as such has no thread checks. Once moved to the main
  // this should only be invoked on the main thread.
  RtpSenderState& operator=(RtpSenderState&&);
  RtpSenderState& operator=(const RtpSenderState&) = delete;

  bool is_initialized() const;
  void Initialize();

  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner() const;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner() const;
  webrtc::scoped_refptr<webrtc::RtpSenderInterface> webrtc_sender() const;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> webrtc_dtls_transport()
      const;
  webrtc::DtlsTransportInformation webrtc_dtls_transport_information() const;
  const std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>&
  track_ref() const;
  void set_track_ref(
      std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>
          track_ref);
  std::vector<std::string> stream_ids() const;

 private:
  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;
  webrtc::scoped_refptr<webrtc::RtpSenderInterface> webrtc_sender_;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> webrtc_dtls_transport_;
  webrtc::DtlsTransportInformation webrtc_dtls_transport_information_;
  bool is_initialized_;
  std::unique_ptr<blink::WebRtcMediaStreamTrackAdapterMap::AdapterRef>
      track_ref_;
  std::vector<std::string> stream_ids_ ALLOW_DISCOURAGED_TYPE(
      "Avoids conversions when passed from/to webrtc API");
};

// Used to surface |webrtc::RtpSenderInterface| to blink. Multiple
// |RTCRtpSenderImpl|s could reference the same webrtc sender; |id| is the value
// of the pointer to the webrtc sender.
// TODO(hbos): [Onion Soup] Move all of the implementation inside the blink
// object and remove this class and interface. The blink object is reference
// counted and we can get rid of this "Web"-copyable with "internal" nonsense,
// all the blink object will need is the RtpSenderState. Requires coordination
// with transceivers and receivers since these are tightly coupled.
// https://crbug.com/787254
class MODULES_EXPORT RTCRtpSenderImpl : public blink::RTCRtpSenderPlatform {
 public:
  static uintptr_t getId(const webrtc::RtpSenderInterface* webrtc_sender);

  RTCRtpSenderImpl(
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface>
          native_peer_connection,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_map,
      RtpSenderState state,
      bool encoded_insertable_streams);
  RTCRtpSenderImpl(const RTCRtpSenderImpl& other);
  ~RTCRtpSenderImpl() override;
  RTCRtpSenderImpl& operator=(const RTCRtpSenderImpl& other);

  const RtpSenderState& state() const;
  void set_state(RtpSenderState state);

  // blink::RTCRtpSenderPlatform.
  std::unique_ptr<blink::RTCRtpSenderPlatform> ShallowCopy() const override;
  uintptr_t Id() const override;
  webrtc::scoped_refptr<webrtc::DtlsTransportInterface> DtlsTransport()
      override;
  webrtc::DtlsTransportInformation DtlsTransportInformation() override;
  MediaStreamComponent* Track() const override;
  Vector<String> StreamIds() const override;
  void ReplaceTrack(MediaStreamComponent* with_track,
                    RTCVoidRequest* request) override;
  std::unique_ptr<blink::RtcDtmfSenderHandler> GetDtmfSender() const override;
  std::unique_ptr<webrtc::RtpParameters> GetParameters() const override;
  void SetParameters(Vector<webrtc::RtpEncodingParameters>,
                     std::optional<webrtc::DegradationPreference>,
                     blink::RTCVoidRequest*) override;
  void GetStats(RTCStatsReportCallback) override;
  void SetStreams(const Vector<String>& stream_ids) override;
  RTCEncodedAudioStreamTransformer* GetEncodedAudioStreamTransformer()
      const override;
  RTCEncodedVideoStreamTransformer* GetEncodedVideoStreamTransformer()
      const override;

  // The ReplaceTrack() that takes a blink::RTCVoidRequest is implemented on
  // top of this, which returns the result in a callback instead. Allows doing
  // ReplaceTrack() without having a blink::RTCVoidRequest, which can only be
  // constructed inside of blink.
  void ReplaceTrack(MediaStreamComponent* with_track,
                    base::OnceCallback<void(bool)> callback);
  // Removes this sender's track from its PeerConnection. Only used in Plan B.
  bool RemoveFromPeerConnection(webrtc::PeerConnectionInterface* pc);

 private:
  class RTCRtpSenderInternal;
  struct RTCRtpSenderInternalTraits;

  scoped_refptr<RTCRtpSenderInternal> internal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENDER_IMPL_H_
