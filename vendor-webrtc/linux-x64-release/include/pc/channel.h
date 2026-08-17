/*
 *  Copyright 2004 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_CHANNEL_H_
#define PC_CHANNEL_H_

#include <stdint.h>

#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/crypto/crypto_options.h"
#include "api/jsep.h"
#include "api/media_types.h"
#include "api/rtp_parameters.h"
#include "api/rtp_transceiver_direction.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/task_queue/task_queue_base.h"
#include "call/rtp_demuxer.h"
#include "call/rtp_packet_sink_interface.h"
#include "media/base/media_channel.h"
#include "media/base/stream_params.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "pc/channel_interface.h"
#include "pc/rtp_transport_internal.h"
#include "pc/session_description.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/checks.h"
#include "rtc_base/containers/flat_set.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/network/sent_packet.h"
#include "rtc_base/network_route.h"
#include "rtc_base/socket.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/unique_id_generator.h"

namespace webrtc {

// BaseChannel contains logic common to voice and video, including enable,
// marshaling calls to a worker and network threads, and connection and media
// monitors.
//
// BaseChannel assumes signaling and other threads are allowed to make
// synchronous calls to the worker thread, the worker thread makes synchronous
// calls only to the network thread, and the network thread can't be blocked by
// other threads.
// All methods with _n suffix must be called on network thread,
//     methods with _w suffix on worker thread
// and methods with _s suffix on signaling thread.
// Network and worker threads may be the same thread.
//
class VideoChannel;
class VoiceChannel;

class BaseChannel : public ChannelInterface,
                    // TODO(tommi): Consider implementing these interfaces
                    // via composition.
                    public MediaChannelNetworkInterface,
                    public RtpPacketSinkInterface {
 public:
  // If `srtp_required` is true, the channel will not send or receive any
  // RTP/RTCP packets without using SRTP (either using SDES or DTLS-SRTP).
  // The BaseChannel does not own the UniqueRandomIdGenerator so it is the
  // responsibility of the user to ensure it outlives this object.
  // TODO(zhihuang:) Create a BaseChannel::Config struct for the parameter lists
  // which will make it easier to change the constructor.

  // Constructor for use when the MediaChannels are split
  BaseChannel(
      TaskQueueBase* worker_thread,
      Thread* network_thread,
      TaskQueueBase* signaling_thread,
      std::unique_ptr<MediaSendChannelInterface> media_send_channel,
      std::unique_ptr<MediaReceiveChannelInterface> media_receive_channel,
      absl::string_view mid,
      bool srtp_required,
      CryptoOptions crypto_options,
      UniqueRandomIdGenerator* ssrc_generator);
  virtual ~BaseChannel();

  TaskQueueBase* worker_thread() const { return worker_thread_; }
  Thread* network_thread() const { return network_thread_; }
  const std::string& mid() const override { return demuxer_criteria_.mid(); }
  // TODO(deadbeef): This is redundant; remove this.
  absl::string_view transport_name() const override {
    RTC_DCHECK_RUN_ON(network_thread());
    if (rtp_transport_)
      return rtp_transport_->transport_name();
    return "";
  }

  // This function returns true if using SRTP (DTLS-based keying or SDES).
  bool srtp_active() const {
    RTC_DCHECK_RUN_ON(network_thread());
    return rtp_transport_ && rtp_transport_->IsSrtpActive();
  }

  // Set an RTP level transport which could be an RtpTransport without
  // encryption, an SrtpTransport for SDES or a DtlsSrtpTransport for DTLS-SRTP.
  // This can be called from any thread and it hops to the network thread
  // internally. It would replace the `SetTransports` and its variants.
  bool SetRtpTransport(RtpTransportInternal* rtp_transport) override;

  RtpTransportInternal* rtp_transport() const {
    RTC_DCHECK_RUN_ON(network_thread());
    return rtp_transport_;
  }

  // Channel control
  bool SetLocalContent(const MediaContentDescription* content,
                       SdpType type,
                       std::string& error_desc) override;
  bool SetRemoteContent(const MediaContentDescription* content,
                        SdpType type,
                        std::string& error_desc) override;
  // Controls whether this channel will receive packets on the basis of
  // matching payload type alone. This is needed for legacy endpoints that
  // don't signal SSRCs or use MID/RID, but doesn't make sense if there is
  // more than channel of specific media type, As that creates an ambiguity.
  //
  // This method will also remove any existing streams that were bound to this
  // channel on the basis of payload type, since one of these streams might
  // actually belong to a new channel. See: crbug.com/webrtc/11477
  bool SetPayloadTypeDemuxingEnabled(bool enabled) override;

  void Enable(bool enable) override;

  const std::vector<StreamParams>& local_streams() const override {
    return local_streams_;
  }
  const std::vector<StreamParams>& remote_streams() const override {
    return remote_streams_;
  }

  // Used for latency measurements.
  void SetFirstPacketReceivedCallback(std::function<void()> callback) override;
  void SetFirstPacketSentCallback(std::function<void()> callback) override;

  // From RtpTransport - public for testing only
  void OnTransportReadyToSend(bool ready);

  // Only public for unit tests.  Otherwise, consider protected.
  int SetOption(SocketType type, Socket::Option o, int val) override;

  // RtpPacketSinkInterface overrides.
  void OnRtpPacket(const RtpPacketReceived& packet) override;

  VideoMediaSendChannelInterface* video_media_send_channel() override {
    RTC_CHECK(false) << "Attempt to fetch video channel from non-video";
    return nullptr;
  }
  VoiceMediaSendChannelInterface* voice_media_send_channel() override {
    RTC_CHECK(false) << "Attempt to fetch voice channel from non-voice";
    return nullptr;
  }
  VideoMediaReceiveChannelInterface* video_media_receive_channel() override {
    RTC_CHECK(false) << "Attempt to fetch video channel from non-video";
    return nullptr;
  }
  VoiceMediaReceiveChannelInterface* voice_media_receive_channel() override {
    RTC_CHECK(false) << "Attempt to fetch voice channel from non-voice";
    return nullptr;
  }

 protected:
  void set_local_content_direction(RtpTransceiverDirection direction)
      RTC_RUN_ON(worker_thread()) {
    local_content_direction_ = direction;
  }

  RtpTransceiverDirection local_content_direction() const
      RTC_RUN_ON(worker_thread()) {
    return local_content_direction_;
  }

  void set_remote_content_direction(RtpTransceiverDirection direction)
      RTC_RUN_ON(worker_thread()) {
    remote_content_direction_ = direction;
  }

  RtpTransceiverDirection remote_content_direction() const
      RTC_RUN_ON(worker_thread()) {
    return remote_content_direction_;
  }

  RtpExtension::Filter extensions_filter() const { return extensions_filter_; }

  bool network_initialized() RTC_RUN_ON(network_thread()) {
    return media_send_channel()->HasNetworkInterface();
  }

  bool enabled() const RTC_RUN_ON(worker_thread()) { return enabled_; }
  TaskQueueBase* signaling_thread() const { return signaling_thread_; }

  // Call to verify that:
  // * The required content description directions have been set.
  // * The channel is enabled.
  // * The SRTP filter is active if it's needed.
  // * The transport has been writable before, meaning it should be at least
  //   possible to succeed in sending a packet.
  //
  // When any of these properties change, UpdateMediaSendRecvState_w should be
  // called.
  bool IsReadyToSendMedia_w() const RTC_RUN_ON(worker_thread());

  // NetworkInterface implementation, called by MediaEngine
  bool SendPacket(CopyOnWriteBuffer* packet,
                  const AsyncSocketPacketOptions& options) override;
  bool SendRtcp(CopyOnWriteBuffer* packet,
                const AsyncSocketPacketOptions& options) override;

  // From RtpTransportInternal
  void OnWritableState(bool writable);

  void OnNetworkRouteChanged(std::optional<NetworkRoute> network_route);

  bool SendPacket(bool rtcp,
                  CopyOnWriteBuffer* packet,
                  const AsyncSocketPacketOptions& options);

  void EnableMedia_w() RTC_RUN_ON(worker_thread());
  void DisableMedia_w() RTC_RUN_ON(worker_thread());

  // Performs actions if the RTP/RTCP writable state changed. This should
  // be called whenever a channel's writable state changes or when RTCP muxing
  // becomes active/inactive.
  void UpdateWritableState_n() RTC_RUN_ON(network_thread());
  void ChannelWritable_n() RTC_RUN_ON(network_thread());
  void ChannelNotWritable_n() RTC_RUN_ON(network_thread());

  bool SetPayloadTypeDemuxingEnabled_w(bool enabled)
      RTC_RUN_ON(worker_thread());

  // Should be called whenever the conditions for
  // IsReadyToReceiveMedia/IsReadyToSendMedia are satisfied (or unsatisfied).
  // Updates the send/recv state of the media channel.
  virtual void UpdateMediaSendRecvState_w() RTC_RUN_ON(worker_thread()) = 0;

  bool UpdateLocalStreams_w(const std::vector<StreamParams>& streams,
                            SdpType type,
                            std::string& error_desc)
      RTC_RUN_ON(worker_thread());
  bool UpdateRemoteStreams_w(const MediaContentDescription* content,
                             SdpType type,
                             std::string& error_desc)
      RTC_RUN_ON(worker_thread());
  virtual bool SetLocalContent_w(const MediaContentDescription* content,
                                 SdpType type,
                                 std::string& error_desc)
      RTC_RUN_ON(worker_thread()) = 0;
  virtual bool SetRemoteContent_w(const MediaContentDescription* content,
                                  SdpType type,
                                  std::string& error_desc)
      RTC_RUN_ON(worker_thread()) = 0;

  // Returns a list of RTP header extensions where any extension URI is unique.
  // Encrypted extensions will be either preferred or discarded, depending on
  // the current crypto_options_.
  RtpHeaderExtensions GetDeduplicatedRtpHeaderExtensions(
      const RtpHeaderExtensions& extensions);

  // Add `payload_type` to `demuxer_criteria_` if payload type demuxing is
  // enabled.
  // Returns true if the demuxer payload type changed and a re-registration
  // is needed.
  bool MaybeAddHandledPayloadType(int payload_type) RTC_RUN_ON(worker_thread());

  // Returns true if the demuxer payload type criteria was non-empty before
  // clearing.
  bool ClearHandledPayloadTypes() RTC_RUN_ON(worker_thread());

  // Hops to the network thread to update the transport if an update is
  // requested. If `update_demuxer` is false and `extensions` is not set, the
  // function simply returns. If either of these is set, the function updates
  // the transport with either or both of the demuxer criteria and the supplied
  // rtp header extensions.
  // Returns `true` if either an update wasn't needed or one was successfully
  // applied. If the return value is `false`, then updating the demuxer criteria
  // failed, which needs to be treated as an error.
  bool MaybeUpdateDemuxerAndRtpExtensions_w(
      bool update_demuxer,
      std::optional<RtpHeaderExtensions> extensions,
      std::string& error_desc) RTC_RUN_ON(worker_thread());

  bool RegisterRtpDemuxerSink_w() RTC_RUN_ON(worker_thread());

  // Return description of media channel to facilitate logging
  std::string ToString() const;

  const std::unique_ptr<MediaSendChannelInterface> media_send_channel_;
  const std::unique_ptr<MediaReceiveChannelInterface> media_receive_channel_;

 private:
  bool ConnectToRtpTransport_n() RTC_RUN_ON(network_thread());
  void DisconnectFromRtpTransport_n() RTC_RUN_ON(network_thread());
  void SignalSentPacket_n(const SentPacketInfo& sent_packet);

  TaskQueueBase* const worker_thread_;
  Thread* const network_thread_;
  TaskQueueBase* const signaling_thread_;
  scoped_refptr<PendingTaskSafetyFlag> alive_;

  // The functions are deleted after they have been called.
  std::function<void()> on_first_packet_received_
      RTC_GUARDED_BY(network_thread());
  std::function<void()> on_first_packet_sent_ RTC_GUARDED_BY(network_thread());

  RtpTransportInternal* rtp_transport_ RTC_GUARDED_BY(network_thread()) =
      nullptr;

  std::vector<std::pair<Socket::Option, int> > socket_options_
      RTC_GUARDED_BY(network_thread());
  std::vector<std::pair<Socket::Option, int> > rtcp_socket_options_
      RTC_GUARDED_BY(network_thread());
  bool writable_ RTC_GUARDED_BY(network_thread()) = false;
  bool was_ever_writable_n_ RTC_GUARDED_BY(network_thread()) = false;
  bool was_ever_writable_ RTC_GUARDED_BY(worker_thread()) = false;
  const bool srtp_required_ = true;

  // Set to either kPreferEncryptedExtension or kDiscardEncryptedExtension
  // based on the supplied CryptoOptions.
  const RtpExtension::Filter extensions_filter_;

  // Currently the `enabled_` flag is accessed from the signaling thread as
  // well, but it can be changed only when signaling thread does a synchronous
  // call to the worker thread, so it should be safe.
  bool enabled_ RTC_GUARDED_BY(worker_thread()) = false;
  bool enabled_s_ RTC_GUARDED_BY(signaling_thread()) = false;
  bool payload_type_demuxing_enabled_ RTC_GUARDED_BY(worker_thread()) = true;
  std::vector<StreamParams> local_streams_ RTC_GUARDED_BY(worker_thread());
  std::vector<StreamParams> remote_streams_ RTC_GUARDED_BY(worker_thread());
  RtpTransceiverDirection local_content_direction_
      RTC_GUARDED_BY(worker_thread()) = RtpTransceiverDirection::kInactive;
  RtpTransceiverDirection remote_content_direction_
      RTC_GUARDED_BY(worker_thread()) = RtpTransceiverDirection::kInactive;

  // Cached list of payload types, used if payload type demuxing is re-enabled.
  flat_set<uint8_t> payload_types_ RTC_GUARDED_BY(worker_thread());
  // A stored copy of the rtp header extensions as applied to the transport.
  RtpHeaderExtensions rtp_header_extensions_ RTC_GUARDED_BY(worker_thread());
  // TODO(bugs.webrtc.org/12239): Modified on worker thread, accessed
  // on network thread in RegisterRtpDemuxerSink_n (called from Init_w)
  RtpDemuxerCriteria demuxer_criteria_;
  // This generator is used to generate SSRCs for local streams.
  // This is needed in cases where SSRCs are not negotiated or set explicitly
  // like in Simulcast.
  // This object is not owned by the channel so it must outlive it.
  UniqueRandomIdGenerator* const ssrc_generator_;
};

// VoiceChannel is a specialization that adds support for early media, DTMF,
// and input/output level monitoring.
class VoiceChannel : public BaseChannel {
 public:
  VoiceChannel(
      TaskQueueBase* worker_thread,
      Thread* network_thread,
      TaskQueueBase* signaling_thread,
      std::unique_ptr<VoiceMediaSendChannelInterface> send_channel_impl,
      std::unique_ptr<VoiceMediaReceiveChannelInterface> receive_channel_impl,
      absl::string_view mid,
      bool srtp_required,
      CryptoOptions crypto_options,
      UniqueRandomIdGenerator* ssrc_generator);

  ~VoiceChannel();

  VideoChannel* AsVideoChannel() override {
    RTC_CHECK_NOTREACHED();
    return nullptr;
  }
  VoiceChannel* AsVoiceChannel() override { return this; }

  VoiceMediaSendChannelInterface* send_channel() {
    return media_send_channel_->AsVoiceSendChannel();
  }

  VoiceMediaReceiveChannelInterface* receive_channel() {
    return media_receive_channel_->AsVoiceReceiveChannel();
  }

  VoiceMediaSendChannelInterface* media_send_channel() override {
    return send_channel();
  }

  VoiceMediaSendChannelInterface* voice_media_send_channel() override {
    return send_channel();
  }

  VoiceMediaReceiveChannelInterface* media_receive_channel() override {
    return receive_channel();
  }

  VoiceMediaReceiveChannelInterface* voice_media_receive_channel() override {
    return receive_channel();
  }

  MediaType media_type() const override { return MediaType::AUDIO; }

 private:
  // overrides from BaseChannel
  void UpdateMediaSendRecvState_w() RTC_RUN_ON(worker_thread()) override;
  bool SetLocalContent_w(const MediaContentDescription* content,
                         SdpType type,
                         std::string& error_desc)
      RTC_RUN_ON(worker_thread()) override;
  bool SetRemoteContent_w(const MediaContentDescription* content,
                          SdpType type,
                          std::string& error_desc)
      RTC_RUN_ON(worker_thread()) override;

  // Last AudioSenderParameter sent down to the media_channel() via
  // SetSenderParameters.
  AudioSenderParameter last_send_params_ RTC_GUARDED_BY(worker_thread());
  // Last AudioReceiverParameters sent down to the media_channel() via
  // SetReceiverParameters.
  AudioReceiverParameters last_recv_params_ RTC_GUARDED_BY(worker_thread());
};

// VideoChannel is a specialization for video.
class VideoChannel : public BaseChannel {
 public:
  VideoChannel(
      TaskQueueBase* worker_thread,
      Thread* network_thread,
      TaskQueueBase* signaling_thread,
      std::unique_ptr<VideoMediaSendChannelInterface> media_send_channel,
      std::unique_ptr<VideoMediaReceiveChannelInterface> media_receive_channel,
      absl::string_view mid,
      bool srtp_required,
      CryptoOptions crypto_options,
      UniqueRandomIdGenerator* ssrc_generator);
  ~VideoChannel();

  VideoChannel* AsVideoChannel() override { return this; }
  VoiceChannel* AsVoiceChannel() override {
    RTC_CHECK_NOTREACHED();
    return nullptr;
  }

  VideoMediaSendChannelInterface* send_channel() {
    return media_send_channel_->AsVideoSendChannel();
  }

  VideoMediaReceiveChannelInterface* receive_channel() {
    return media_receive_channel_->AsVideoReceiveChannel();
  }

  VideoMediaSendChannelInterface* media_send_channel() override {
    return send_channel();
  }

  VideoMediaSendChannelInterface* video_media_send_channel() override {
    return send_channel();
  }

  VideoMediaReceiveChannelInterface* media_receive_channel() override {
    return receive_channel();
  }

  VideoMediaReceiveChannelInterface* video_media_receive_channel() override {
    return receive_channel();
  }

  MediaType media_type() const override { return MediaType::VIDEO; }

 private:
  // overrides from BaseChannel
  void UpdateMediaSendRecvState_w() RTC_RUN_ON(worker_thread()) override;
  bool SetLocalContent_w(const MediaContentDescription* content,
                         SdpType type,
                         std::string& error_desc)
      RTC_RUN_ON(worker_thread()) override;
  bool SetRemoteContent_w(const MediaContentDescription* content,
                          SdpType type,
                          std::string& error_desc)
      RTC_RUN_ON(worker_thread()) override;

  // Last VideoSenderParameters sent down to the media_channel() via
  // SetSenderParameters.
  VideoSenderParameters last_send_params_ RTC_GUARDED_BY(worker_thread());
  // Last VideoReceiverParameters sent down to the media_channel() via
  // SetReceiverParameters.
  VideoReceiverParameters last_recv_params_ RTC_GUARDED_BY(worker_thread());
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::BaseChannel;
using ::webrtc::VideoChannel;
using ::webrtc::VoiceChannel;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // PC_CHANNEL_H_
