// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_IMPL_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_receiver_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sender_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/webrtc/api/rtp_transceiver_interface.h"

namespace blink {

// This class represents the state of a transceiver; a snapshot of what a
// webrtc-layer transceiver looked like when it was inspected on the signaling
// thread such that this information can be moved to the main thread in a single
// PostTask. It is used to surface state changes to make the blink-layer
// transceiver up-to-date.
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
// The RtpTransceiverState is a snapshot of what the
// webrtc::RtpTransceiverInterface looked like when the RtpTransceiverState was
// created on the signaling thread. It also takes care of initializing sender
// and receiver states, including their track adapters such that we have access
// to a blink track corresponding to the webrtc tracks of the sender and
// receiver.
//
// Except for initialization logic and operator=(), the RtpTransceiverState is
// immutable and only accessible on the main thread.
//
// TODO(crbug.com/787254): Consider merging RTCRtpTransceiverImpl and
// RTCRtpTransceiver (requires coordination with senders and receivers) and
// removing RTCRtpTransceiverPlatform when all its clients are Onion soup'ed.
// Also, move away from using std::vector.
class MODULES_EXPORT RtpTransceiverState {
 public:
  RtpTransceiverState(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      scoped_refptr<webrtc::RtpTransceiverInterface> webrtc_transceiver,
      std::optional<blink::RtpSenderState> sender_state,
      std::optional<blink::RtpReceiverState> receiver_state,
      std::optional<std::string> mid,
      webrtc::RtpTransceiverDirection direction,
      std::optional<webrtc::RtpTransceiverDirection> current_direction,
      std::optional<webrtc::RtpTransceiverDirection> fired_direction,
      Vector<webrtc::RtpHeaderExtensionCapability>
          header_extensions_negotiated);
  RtpTransceiverState(RtpTransceiverState&&);
  RtpTransceiverState(const RtpTransceiverState&) = delete;
  ~RtpTransceiverState();

  // This is intended to be used for moving the object from the signaling thread
  // to the main thread and as such has no thread checks. Once moved to the main
  // this should only be invoked on the main thread.
  RtpTransceiverState& operator=(RtpTransceiverState&&);
  RtpTransceiverState& operator=(const RtpTransceiverState&) = delete;

  bool is_initialized() const;
  void Initialize();

  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner() const;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner() const;
  scoped_refptr<webrtc::RtpTransceiverInterface> webrtc_transceiver() const;
  const std::optional<blink::RtpSenderState>& sender_state() const;
  blink::RtpSenderState MoveSenderState();
  const std::optional<blink::RtpReceiverState>& receiver_state() const;
  blink::RtpReceiverState MoveReceiverState();
  std::optional<std::string> mid() const;
  void set_mid(std::optional<std::string>);
  webrtc::RtpTransceiverDirection direction() const;
  void set_direction(webrtc::RtpTransceiverDirection);
  std::optional<webrtc::RtpTransceiverDirection> current_direction() const;
  std::optional<webrtc::RtpTransceiverDirection> fired_direction() const;
  const Vector<webrtc::RtpHeaderExtensionCapability>&
  header_extensions_negotiated() const;

 private:
  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;
  scoped_refptr<webrtc::RtpTransceiverInterface> webrtc_transceiver_;
  bool is_initialized_;
  std::optional<blink::RtpSenderState> sender_state_;
  std::optional<blink::RtpReceiverState> receiver_state_;
  std::optional<std::string> mid_;
  webrtc::RtpTransceiverDirection direction_;
  std::optional<webrtc::RtpTransceiverDirection> current_direction_;
  std::optional<webrtc::RtpTransceiverDirection> fired_direction_;
  Vector<webrtc::RtpHeaderExtensionCapability> header_extensions_negotiated_;
};

// RTCRtpTransceiverImpl::set_state() performs differently depending on the
// update mode. The update mode exists to get around problems with the webrtc
// threading model: https://crbug.com/webrtc/8692.
//
// Transceiver state information can be surfaced as a result of invoking a
// number of different JavaScript APIs. The way states are surfaced from webrtc
// to blink fall into two categories:
//   Blocking operations and callback-based operations.
//
// When a blocking operation is invoked, the main thread is blocked on the
// webrtc signaling thread, and the state information is surfaced immediately
// - guaranteed to be up-to-date. An example of this is addTrack().
// Callback-based operations on the other hand will post a task from the
// signaling thread to the main thread, placing the task to update the state
// information in queue. There is no guarantee that something - such as
// addTrack() - doesn't happen in-between the posting of the task and the
// execution of it. In such cases, the state information surfaced might not be
// up-to-date (in edge cases). Examples of callback-based operations include
// setLocalDescripti.on() and setRemoteDescription().
enum class TransceiverStateUpdateMode {
  // In this mode, all state information is updated. Use this enum unless
  // a different update mode applies.
  kAll,
  // Use this enum when surfacing state information as a result of
  // setLocalDescription() or setRemoteDescription().
  // Behaves like "kAll" except "transceiver.sender.track" and
  // "transceiver.direction" are not updated.
  kSetDescription,
};

// Used to surface |webrtc::RtpTransceiverInterface| to blink. Multiple
// |RTCRtpTransceiverImpl|s could reference the same webrtc transceiver; |id| is
// unique per webrtc transceiver.
// Its methods are accessed on the main thread, internally also performs
// operations on the signaling thread.
class MODULES_EXPORT RTCRtpTransceiverImpl : public RTCRtpTransceiverPlatform {
 public:
  static uintptr_t GetId(
      const webrtc::RtpTransceiverInterface* webrtc_transceiver);

  RTCRtpTransceiverImpl(
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface>
          native_peer_connection,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_map,
      RtpTransceiverState state,
      bool encoded_insertable_streams,
      std::unique_ptr<webrtc::Metronome> decode_metronome);
  RTCRtpTransceiverImpl(const RTCRtpTransceiverImpl& other);
  ~RTCRtpTransceiverImpl() override;

  RTCRtpTransceiverImpl& operator=(const RTCRtpTransceiverImpl& other);
  std::unique_ptr<RTCRtpTransceiverImpl> ShallowCopy() const;

  const RtpTransceiverState& state() const;
  void set_state(RtpTransceiverState state,
                 TransceiverStateUpdateMode update_mode);
  blink::RTCRtpSenderImpl* content_sender();
  blink::RTCRtpReceiverImpl* content_receiver();

  uintptr_t Id() const override;
  String Mid() const override;
  std::unique_ptr<RTCRtpSenderPlatform> Sender() const override;
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
  class RTCRtpTransceiverInternal;
  struct RTCRtpTransceiverInternalTraits;

  scoped_refptr<RTCRtpTransceiverInternal> internal_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_IMPL_H_
