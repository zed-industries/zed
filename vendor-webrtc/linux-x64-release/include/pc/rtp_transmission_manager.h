/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_RTP_TRANSMISSION_MANAGER_H_
#define PC_RTP_TRANSMISSION_MANAGER_H_

#include <stdint.h>

#include <functional>
#include <string>
#include <vector>

#include "api/environment/environment.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_parameters.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "media/base/media_channel.h"
#include "pc/codec_vendor.h"
#include "pc/connection_context.h"
#include "pc/legacy_stats_collector_interface.h"
#include "pc/rtp_receiver.h"
#include "pc/rtp_receiver_proxy.h"
#include "pc/rtp_sender.h"
#include "pc/rtp_sender_proxy.h"
#include "pc/rtp_transceiver.h"
#include "pc/transceiver_list.h"
#include "pc/usage_pattern.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/unique_id_generator.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

// This class contains information about
// an RTPSender, used for things like looking it up by SSRC.
struct RtpSenderInfo {
  RtpSenderInfo() : first_ssrc(0) {}
  RtpSenderInfo(const std::string& stream_id,
                const std::string& sender_id,
                uint32_t ssrc)
      : stream_id(stream_id), sender_id(sender_id), first_ssrc(ssrc) {}
  bool operator==(const RtpSenderInfo& other) {
    return this->stream_id == other.stream_id &&
           this->sender_id == other.sender_id &&
           this->first_ssrc == other.first_ssrc;
  }
  std::string stream_id;
  std::string sender_id;
  // An RtpSender can have many SSRCs. The first one is used as a sort of ID
  // for communicating with the lower layers.
  uint32_t first_ssrc;
};

// The RtpTransmissionManager class is responsible for managing the lifetime
// and relationships between objects of type RtpSender, RtpReceiver and
// RtpTransceiver.
class RtpTransmissionManager : public RtpSenderBase::SetStreamsObserver {
 public:
  RtpTransmissionManager(const Environment& env,
                         bool is_unified_plan,
                         ConnectionContext* context,
                         CodecLookupHelper* codec_lookup_helper,
                         UsagePattern* usage_pattern,
                         PeerConnectionObserver* observer,
                         LegacyStatsCollectorInterface* legacy_stats,
                         std::function<void()> on_negotiation_needed);

  // No move or copy permitted.
  RtpTransmissionManager(const RtpTransmissionManager&) = delete;
  RtpTransmissionManager& operator=(const RtpTransmissionManager&) = delete;

  // Stop activity. In particular, don't call observer_ any more.
  void Close();

  // RtpSenderBase::SetStreamsObserver override.
  void OnSetStreams() override;

  // Add a new track, creating transceiver if required.
  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>* init_send_encodings);

  // Create a new RTP sender. Does not associate with a transceiver.
  scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>> CreateSender(
      webrtc::MediaType media_type,
      const std::string& id,
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>& send_encodings);

  // Create a new RTP receiver. Does not associate with a transceiver.
  scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>
  CreateReceiver(webrtc::MediaType media_type, const std::string& receiver_id);

  // Create a new RtpTransceiver of the given type and add it to the list of
  // registered transceivers.
  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  CreateAndAddTransceiver(
      scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>> sender,
      scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>
          receiver);

  // Returns the first RtpTransceiver suitable for a newly added track, if such
  // transceiver is available.
  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  FindFirstTransceiverForAddedTrack(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<RtpEncodingParameters>* init_send_encodings);

  // Returns the list of senders currently associated with some
  // registered transceiver
  std::vector<scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>>>
  GetSendersInternal() const;

  // Returns the list of receivers currently associated with a transceiver
  std::vector<scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>>
  GetReceiversInternal() const;

  // Plan B: Get the transceiver containing all audio senders and receivers
  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  GetAudioTransceiver() const;
  // Plan B: Get the transceiver containing all video senders and receivers
  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  GetVideoTransceiver() const;

  // Add an audio track, reusing or creating the sender.
  void AddAudioTrack(AudioTrackInterface* track, MediaStreamInterface* stream);
  // Plan B: Remove an audio track, removing the sender.
  void RemoveAudioTrack(AudioTrackInterface* track,
                        MediaStreamInterface* stream);
  // Add a video track, reusing or creating the sender.
  void AddVideoTrack(VideoTrackInterface* track, MediaStreamInterface* stream);
  // Plan B: Remove a video track, removing the sender.
  void RemoveVideoTrack(VideoTrackInterface* track,
                        MediaStreamInterface* stream);

  // Triggered when a remote sender has been seen for the first time in a remote
  // session description. It creates a remote MediaStreamTrackInterface
  // implementation and triggers CreateAudioReceiver or CreateVideoReceiver.
  void OnRemoteSenderAdded(const RtpSenderInfo& sender_info,
                           MediaStreamInterface* stream,
                           webrtc::MediaType media_type);

  // Triggered when a remote sender has been removed from a remote session
  // description. It removes the remote sender with id `sender_id` from a remote
  // MediaStream and triggers DestroyAudioReceiver or DestroyVideoReceiver.
  void OnRemoteSenderRemoved(const RtpSenderInfo& sender_info,
                             MediaStreamInterface* stream,
                             webrtc::MediaType media_type);

  // Triggered when a local sender has been seen for the first time in a local
  // session description.
  // This method triggers CreateAudioSender or CreateVideoSender if the rtp
  // streams in the local SessionDescription can be mapped to a MediaStreamTrack
  // in a MediaStream in `local_streams_`
  void OnLocalSenderAdded(const RtpSenderInfo& sender_info,
                          webrtc::MediaType media_type);

  // Triggered when a local sender has been removed from a local session
  // description.
  // This method triggers DestroyAudioSender or DestroyVideoSender if a stream
  // has been removed from the local SessionDescription and the stream can be
  // mapped to a MediaStreamTrack in a MediaStream in `local_streams_`.
  void OnLocalSenderRemoved(const RtpSenderInfo& sender_info,
                            webrtc::MediaType media_type);

  std::vector<RtpSenderInfo>* GetRemoteSenderInfos(
      webrtc::MediaType media_type);
  std::vector<RtpSenderInfo>* GetLocalSenderInfos(webrtc::MediaType media_type);
  const RtpSenderInfo* FindSenderInfo(const std::vector<RtpSenderInfo>& infos,
                                      const std::string& stream_id,
                                      const std::string& sender_id) const;

  // Return the RtpSender with the given track attached.
  scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>>
  FindSenderForTrack(MediaStreamTrackInterface* track) const;

  // Return the RtpSender with the given id, or null if none exists.
  scoped_refptr<RtpSenderProxyWithInternal<RtpSenderInternal>> FindSenderById(
      const std::string& sender_id) const;

  // Return the RtpReceiver with the given id, or null if none exists.
  scoped_refptr<RtpReceiverProxyWithInternal<RtpReceiverInternal>>
  FindReceiverById(const std::string& receiver_id) const;

  TransceiverList* transceivers() { return &transceivers_; }
  const TransceiverList* transceivers() const { return &transceivers_; }

  // Plan B helpers for getting the voice/video media channels for the single
  // audio/video transceiver, if it exists.
  VoiceMediaSendChannelInterface* voice_media_send_channel() const;
  VideoMediaSendChannelInterface* video_media_send_channel() const;
  VoiceMediaReceiveChannelInterface* voice_media_receive_channel() const;
  VideoMediaReceiveChannelInterface* video_media_receive_channel() const;

 private:
  Thread* signaling_thread() const { return context_->signaling_thread(); }
  Thread* worker_thread() const { return context_->worker_thread(); }
  bool IsUnifiedPlan() const { return is_unified_plan_; }
  void NoteUsageEvent(UsageEvent event) {
    usage_pattern_->NoteUsageEvent(event);
  }

  // AddTrack implementation when Unified Plan is specified.
  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrackUnifiedPlan(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>* init_send_encodings);
  // AddTrack implementation when Plan B is specified.
  RTCErrorOr<scoped_refptr<RtpSenderInterface>> AddTrackPlanB(
      scoped_refptr<MediaStreamTrackInterface> track,
      const std::vector<std::string>& stream_ids,
      const std::vector<RtpEncodingParameters>* init_send_encodings);

  // Create an RtpReceiver that sources an audio track.
  void CreateAudioReceiver(MediaStreamInterface* stream,
                           const RtpSenderInfo& remote_sender_info)
      RTC_RUN_ON(signaling_thread());

  // Create an RtpReceiver that sources a video track.
  void CreateVideoReceiver(MediaStreamInterface* stream,
                           const RtpSenderInfo& remote_sender_info)
      RTC_RUN_ON(signaling_thread());
  scoped_refptr<RtpReceiverInterface> RemoveAndStopReceiver(
      const RtpSenderInfo& remote_sender_info) RTC_RUN_ON(signaling_thread());

  PeerConnectionObserver* Observer() const;
  void OnNegotiationNeeded();

  MediaEngineInterface* media_engine() const;

  UniqueRandomIdGenerator* ssrc_generator() const {
    return context_->ssrc_generator();
  }

  const Environment env_;
  TransceiverList transceivers_;

  // These lists store sender info seen in local/remote descriptions.
  std::vector<RtpSenderInfo> remote_audio_sender_infos_
      RTC_GUARDED_BY(signaling_thread());
  std::vector<RtpSenderInfo> remote_video_sender_infos_
      RTC_GUARDED_BY(signaling_thread());
  std::vector<RtpSenderInfo> local_audio_sender_infos_
      RTC_GUARDED_BY(signaling_thread());
  std::vector<RtpSenderInfo> local_video_sender_infos_
      RTC_GUARDED_BY(signaling_thread());

  bool closed_ = false;
  bool const is_unified_plan_;
  ConnectionContext* context_;
  CodecLookupHelper* codec_lookup_helper_;
  UsagePattern* usage_pattern_;
  PeerConnectionObserver* observer_;
  LegacyStatsCollectorInterface* const legacy_stats_;
  std::function<void()> on_negotiation_needed_;
  WeakPtrFactory<RtpTransmissionManager> weak_ptr_factory_
      RTC_GUARDED_BY(signaling_thread());
};

}  // namespace webrtc

#endif  // PC_RTP_TRANSMISSION_MANAGER_H_
