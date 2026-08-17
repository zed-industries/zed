/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTP_TRANSCEIVER_H_
#define PC_RTP_TRANSCEIVER_H_

#include <stddef.h>

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/audio_options.h"
#include "api/crypto/crypto_options.h"
#include "api/jsep.h"
#include "api/media_types.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/rtp_transceiver_direction.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "media/base/media_channel.h"
#include "media/base/media_config.h"
#include "media/base/media_engine.h"
#include "pc/channel_interface.h"
#include "pc/codec_vendor.h"
#include "pc/connection_context.h"
#include "pc/proxy.h"
#include "pc/rtp_receiver.h"
#include "pc/rtp_receiver_proxy.h"
#include "pc/rtp_sender.h"
#include "pc/rtp_sender_proxy.h"
#include "pc/rtp_transport_internal.h"
#include "pc/session_description.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class PeerConnectionSdpMethods;

// Implementation of the public RtpTransceiverInterface.
//
// The RtpTransceiverInterface is only intended to be used with a PeerConnection
// that enables Unified Plan SDP. Thus, the methods that only need to implement
// public API features and are not used internally can assume exactly one sender
// and receiver.
//
// Since the RtpTransceiver is used internally by PeerConnection for tracking
// RtpSenders, RtpReceivers, and BaseChannels, and PeerConnection needs to be
// backwards compatible with Plan B SDP, this implementation is more flexible
// than that required by the WebRTC specification.
//
// With Plan B SDP, an RtpTransceiver can have any number of senders and
// receivers which map to a=ssrc lines in the m= section.
// With Unified Plan SDP, an RtpTransceiver will have exactly one sender and one
// receiver which are encapsulated by the m= section.
//
// This class manages the RtpSenders, RtpReceivers, and BaseChannel associated
// with this m= section. Since the transceiver, senders, and receivers are
// reference counted and can be referenced from JavaScript (in Chromium), these
// objects must be ready to live for an arbitrary amount of time. The
// BaseChannel is not reference counted, so
// the PeerConnection must take care of creating/deleting the BaseChannel.
//
// The RtpTransceiver is specialized to either audio or video according to the
// MediaType specified in the constructor. Audio RtpTransceivers will have
// AudioRtpSenders, AudioRtpReceivers, and a VoiceChannel. Video RtpTransceivers
// will have VideoRtpSenders, VideoRtpReceivers, and a VideoChannel.
class RtpTransceiver : public RtpTransceiverInterface {
 public:
  // Construct a Plan B-style RtpTransceiver with no senders, receivers, or
  // channel set.
  // `media_type` specifies the type of RtpTransceiver (and, by transitivity,
  // the type of senders, receivers, and channel). Can either by audio or video.
  RtpTransceiver(webrtc::MediaType media_type,
                 ConnectionContext* context,
                 CodecLookupHelper* codec_lookup_helper);
  // Construct a Unified Plan-style RtpTransceiver with the given sender and
  // receiver. The media type will be derived from the media types of the sender
  // and receiver. The sender and receiver should have the same media type.
  // `HeaderExtensionsToNegotiate` is used for initializing the return value of
  // HeaderExtensionsToNegotiate().
  RtpTransceiver(
      scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>> sender,
      scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>> receiver,
      ConnectionContext* context,
      CodecLookupHelper* codec_lookup_helper,
      std::vector<RtpHeaderExtensionCapability> HeaderExtensionsToNegotiate,
      std::function<void()> on_negotiation_needed);
  ~RtpTransceiver() override;

  // Not copyable or movable.
  RtpTransceiver(const RtpTransceiver&) = delete;
  RtpTransceiver& operator=(const RtpTransceiver&) = delete;
  RtpTransceiver(RtpTransceiver&&) = delete;
  RtpTransceiver& operator=(RtpTransceiver&&) = delete;

  // Returns the Voice/VideoChannel set for this transceiver. May be null if
  // the transceiver is not in the currently set local/remote description.
  ChannelInterface* channel() const { return channel_.get(); }

  // Creates the Voice/VideoChannel and sets it.
  RTCError CreateChannel(
      absl::string_view mid,
      Call* call_ptr,
      const MediaConfig& media_config,
      bool srtp_required,
      CryptoOptions crypto_options,
      const AudioOptions& audio_options,
      const VideoOptions& video_options,
      VideoBitrateAllocatorFactory* video_bitrate_allocator_factory,
      std::function<RtpTransportInternal*(absl::string_view)> transport_lookup);

  // Sets the Voice/VideoChannel. The caller must pass in the correct channel
  // implementation based on the type of the transceiver.  The call must
  // furthermore be made on the signaling thread.
  //
  // `channel`: The channel instance to be associated with the transceiver.
  //     This must be a valid pointer.
  //     The state of the object
  //     is expected to be newly constructed and not initalized for network
  //     activity (see next parameter for more).
  //
  //     The transceiver takes ownership of `channel`.
  //
  // `transport_lookup`: This
  //     callback function will be used to look up the `RtpTransport` object
  //     to associate with the channel via `BaseChannel::SetRtpTransport`.
  //     The lookup function will be called on the network thread, synchronously
  //     during the call to `SetChannel`.  This means that the caller of
  //     `SetChannel()` may provide a callback function that references state
  //     that exists within the calling scope of SetChannel (e.g. a variable
  //     on the stack).
  //     The reason for this design is to limit the number of times we jump
  //     synchronously to the network thread from the signaling thread.
  //     The callback allows us to combine the transport lookup with network
  //     state initialization of the channel object.
  // ClearChannel() must be used before calling SetChannel() again.
  void SetChannel(std::unique_ptr<ChannelInterface> channel,
                  std::function<RtpTransportInternal*(const std::string&)>
                      transport_lookup);

  // Clear the association between the transceiver and the channel.
  void ClearChannel();

  // Adds an RtpSender of the appropriate type to be owned by this transceiver.
  // Must not be null.
  void AddSender(
      scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>> sender);

  // Removes the given RtpSender. Returns false if the sender is not owned by
  // this transceiver.
  bool RemoveSender(RtpSenderInterface* sender);

  // Returns a vector of the senders owned by this transceiver.
  std::vector<scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>>>
  senders() const {
    return senders_;
  }

  // Adds an RtpReceiver of the appropriate type to be owned by this
  // transceiver. Must not be null.
  void AddReceiver(
      scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>
          receiver);

  // Removes the given RtpReceiver. Returns false if the receiver is not owned
  // by this transceiver.
  bool RemoveReceiver(RtpReceiverInterface* receiver);

  // Returns a vector of the receivers owned by this transceiver.
  std::vector<scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>>
  receivers() const {
    return receivers_;
  }

  // Returns the backing object for the transceiver's Unified Plan sender.
  scoped_refptr<RtpSenderInternal> sender_internal() const;

  // Returns the backing object for the transceiver's Unified Plan receiver.
  scoped_refptr<RtpReceiverInternal> receiver_internal() const;

  // RtpTransceivers are not associated until they have a corresponding media
  // section set in SetLocalDescription or SetRemoteDescription. Therefore,
  // when setting a local offer we need a way to remember which transceiver was
  // used to create which media section in the offer. Storing the mline index
  // in CreateOffer is specified in JSEP to allow us to do that.
  std::optional<size_t> mline_index() const { return mline_index_; }
  void set_mline_index(std::optional<size_t> mline_index) {
    mline_index_ = mline_index;
  }

  // Sets the MID for this transceiver. If the MID is not null, then the
  // transceiver is considered "associated" with the media section that has the
  // same MID.
  void set_mid(const std::optional<std::string>& mid) { mid_ = mid; }

  // Sets the intended direction for this transceiver. Intended to be used
  // internally over SetDirection since this does not trigger a negotiation
  // needed callback.
  void set_direction(RtpTransceiverDirection direction) {
    direction_ = direction;
  }

  // Sets the current direction for this transceiver as negotiated in an offer/
  // answer exchange. The current direction is null before an answer with this
  // transceiver has been set.
  void set_current_direction(RtpTransceiverDirection direction);

  // Sets the fired direction for this transceiver. The fired direction is null
  // until SetRemoteDescription is called or an answer is set (either local or
  // remote) after which the only valid reason to go back to null is rollback.
  void set_fired_direction(std::optional<RtpTransceiverDirection> direction);

  // According to JSEP rules for SetRemoteDescription, RtpTransceivers can be
  // reused only if they were added by AddTrack.
  void set_created_by_addtrack(bool created_by_addtrack) {
    created_by_addtrack_ = created_by_addtrack;
  }
  // If AddTrack has been called then transceiver can't be removed during
  // rollback.
  void set_reused_for_addtrack(bool reused_for_addtrack) {
    reused_for_addtrack_ = reused_for_addtrack;
  }

  bool created_by_addtrack() const { return created_by_addtrack_; }

  bool reused_for_addtrack() const { return reused_for_addtrack_; }

  // Returns true if this transceiver has ever had the current direction set to
  // sendonly or sendrecv.
  bool has_ever_been_used_to_send() const {
    return has_ever_been_used_to_send_;
  }

  // Informs the transceiver that its owning
  // PeerConnection is closed.
  void SetPeerConnectionClosed();

  // Executes the "stop the RTCRtpTransceiver" procedure from
  // the webrtc-pc specification, described under the stop() method.
  void StopTransceiverProcedure();

  // RtpTransceiverInterface implementation.
  webrtc::MediaType media_type() const override;
  std::optional<std::string> mid() const override;
  scoped_refptr<RtpSenderInterface> sender() const override;
  scoped_refptr<RtpReceiverInterface> receiver() const override;
  bool stopped() const override;
  bool stopping() const override;
  RtpTransceiverDirection direction() const override;
  RTCError SetDirectionWithError(
      RtpTransceiverDirection new_direction) override;
  std::optional<RtpTransceiverDirection> current_direction() const override;
  std::optional<RtpTransceiverDirection> fired_direction() const override;
  RTCError StopStandard() override;
  void StopInternal() override;
  RTCError SetCodecPreferences(ArrayView<RtpCodecCapability> codecs) override;
  // TODO(https://crbug.com/webrtc/391275081): Delete codec_preferences() in
  // favor of filtered_codec_preferences() because it's not used anywhere.
  std::vector<RtpCodecCapability> codec_preferences() const override;
  // A direction()-filtered view of codec_preferences(). If this filtering
  // results in not having any media codecs, an empty list is returned to mean
  // "no preferences".
  std::vector<RtpCodecCapability> filtered_codec_preferences() const;
  std::vector<RtpHeaderExtensionCapability> GetHeaderExtensionsToNegotiate()
      const override;
  std::vector<RtpHeaderExtensionCapability> GetNegotiatedHeaderExtensions()
      const override;
  RTCError SetHeaderExtensionsToNegotiate(
      ArrayView<const RtpHeaderExtensionCapability> header_extensions) override;

  // Called on the signaling thread when the local or remote content description
  // is updated. Used to update the negotiated header extensions.
  // TODO(tommi): The implementation of this method is currently very simple and
  // only used for updating the negotiated headers. However, we're planning to
  // move all the updates done on the channel from the transceiver into this
  // method. This will happen with the ownership of the channel object being
  // moved into the transceiver.
  void OnNegotiationUpdate(SdpType sdp_type,
                           const MediaContentDescription* content);

 private:
  MediaEngineInterface* media_engine() const {
    return context_->media_engine();
  }
  ConnectionContext* context() const { return context_; }
  CodecVendor& codec_vendor() {
    return *codec_lookup_helper_->GetCodecVendor();
  }
  void OnFirstPacketReceived();
  void OnFirstPacketSent();
  void StopSendingAndReceiving();
  // Tell the senders and receivers about possibly-new media channels
  // in a newly created `channel_`.
  void PushNewMediaChannel();
  // Delete `channel_`, and ensure that references to its media channels
  // are updated before deleting it.
  void DeleteChannel();

  RTCError UpdateCodecPreferencesCaches(
      const std::vector<RtpCodecCapability>& codecs);

  // Enforce that this object is created, used and destroyed on one thread.
  TaskQueueBase* const thread_;
  const bool unified_plan_;
  const webrtc::MediaType media_type_;
  scoped_refptr<PendingTaskSafetyFlag> signaling_thread_safety_;
  std::vector<scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>>>
      senders_;
  std::vector<scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>>
      receivers_;

  bool stopped_ RTC_GUARDED_BY(thread_) = false;
  bool stopping_ RTC_GUARDED_BY(thread_) = false;
  bool is_pc_closed_ = false;
  RtpTransceiverDirection direction_ = RtpTransceiverDirection::kInactive;
  std::optional<RtpTransceiverDirection> current_direction_;
  std::optional<RtpTransceiverDirection> fired_direction_;
  std::optional<std::string> mid_;
  std::optional<size_t> mline_index_;
  bool created_by_addtrack_ = false;
  bool reused_for_addtrack_ = false;
  bool has_ever_been_used_to_send_ = false;

  // Accessed on both thread_ and the network thread. Considered safe
  // because all access on the network thread is within an invoke()
  // from thread_.
  std::unique_ptr<ChannelInterface> channel_ = nullptr;
  ConnectionContext* const context_;
  CodecLookupHelper* const codec_lookup_helper_;
  std::vector<RtpCodecCapability> codec_preferences_;
  std::vector<RtpCodecCapability> sendrecv_codec_preferences_;
  std::vector<RtpCodecCapability> sendonly_codec_preferences_;
  std::vector<RtpCodecCapability> recvonly_codec_preferences_;
  std::vector<RtpHeaderExtensionCapability> header_extensions_to_negotiate_;

  // `negotiated_header_extensions_` is read and written to on the signaling
  // thread from the SdpOfferAnswerHandler class (e.g.
  // PushdownMediaDescription().
  RtpHeaderExtensions negotiated_header_extensions_ RTC_GUARDED_BY(thread_);

  const std::function<void()> on_negotiation_needed_;
};

BEGIN_PRIMARY_PROXY_MAP(RtpTransceiver)

PROXY_PRIMARY_THREAD_DESTRUCTOR()
BYPASS_PROXY_CONSTMETHOD0(webrtc::MediaType, media_type)
PROXY_CONSTMETHOD0(std::optional<std::string>, mid)
PROXY_CONSTMETHOD0(scoped_refptr<RtpSenderInterface>, sender)
PROXY_CONSTMETHOD0(scoped_refptr<RtpReceiverInterface>, receiver)
PROXY_CONSTMETHOD0(bool, stopped)
PROXY_CONSTMETHOD0(bool, stopping)
PROXY_CONSTMETHOD0(RtpTransceiverDirection, direction)
PROXY_METHOD1(RTCError, SetDirectionWithError, RtpTransceiverDirection)
PROXY_CONSTMETHOD0(std::optional<RtpTransceiverDirection>, current_direction)
PROXY_CONSTMETHOD0(std::optional<RtpTransceiverDirection>, fired_direction)
PROXY_METHOD0(RTCError, StopStandard)
PROXY_METHOD0(void, StopInternal)
PROXY_METHOD1(RTCError, SetCodecPreferences, ArrayView<RtpCodecCapability>)
PROXY_CONSTMETHOD0(std::vector<RtpCodecCapability>, codec_preferences)
PROXY_CONSTMETHOD0(std::vector<RtpHeaderExtensionCapability>,
                   GetHeaderExtensionsToNegotiate)
PROXY_CONSTMETHOD0(std::vector<RtpHeaderExtensionCapability>,
                   GetNegotiatedHeaderExtensions)
PROXY_METHOD1(RTCError,
              SetHeaderExtensionsToNegotiate,
              ArrayView<const RtpHeaderExtensionCapability>)
END_PROXY_MAP(RtpTransceiver)

}  // namespace webrtc

#endif  // PC_RTP_TRANSCEIVER_H_
