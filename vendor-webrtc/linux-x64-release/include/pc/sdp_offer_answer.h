/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SDP_OFFER_ANSWER_H_
#define PC_SDP_OFFER_ANSWER_H_

#include <stddef.h>
#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <vector>

#include "api/audio_options.h"
#include "api/candidate.h"
#include "api/jsep.h"
#include "api/jsep_ice_candidate.h"
#include "api/media_stream_interface.h"
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "api/rtp_transceiver_direction.h"
#include "api/rtp_transceiver_interface.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/uma_metrics.h"
#include "api/video/video_bitrate_allocator_factory.h"
#include "media/base/media_channel.h"
#include "media/base/media_engine.h"
#include "media/base/stream_params.h"
#include "p2p/base/port_allocator.h"
#include "pc/codec_vendor.h"
#include "pc/connection_context.h"
#include "pc/data_channel_controller.h"
#include "pc/jsep_transport_controller.h"
#include "pc/media_options.h"
#include "pc/media_session.h"
#include "pc/media_stream_observer.h"
#include "pc/rtp_receiver.h"
#include "pc/rtp_transceiver.h"
#include "pc/rtp_transmission_manager.h"
#include "pc/sdp_state_provider.h"
#include "pc/session_description.h"
#include "pc/stream_collection.h"
#include "pc/transceiver_list.h"
#include "pc/webrtc_session_description_factory.h"
#include "rtc_base/operations_chain.h"
#include "rtc_base/rtc_certificate_generator.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/unique_id_generator.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

// SdpOfferAnswerHandler is a component
// of the PeerConnection object as defined
// by the PeerConnectionInterface API surface.
// The class is responsible for the following:
// - Parsing and interpreting SDP.
// - Generating offers and answers based on the current state.
// This class lives on the signaling thread.
class SdpOfferAnswerHandler : public SdpStateProvider {
 public:
  ~SdpOfferAnswerHandler();

  // Creates an SdpOfferAnswerHandler. Modifies dependencies.
  static std::unique_ptr<SdpOfferAnswerHandler> Create(
      PeerConnectionSdpMethods* pc,
      const PeerConnectionInterface::RTCConfiguration& configuration,
      std::unique_ptr<RTCCertificateGeneratorInterface> cert_generator,
      std::unique_ptr<webrtc::VideoBitrateAllocatorFactory>
          video_bitrate_allocator_factory,
      ConnectionContext* context,
      CodecLookupHelper* codec_lookup_helper);

  void ResetSessionDescFactory() {
    RTC_DCHECK_RUN_ON(signaling_thread());
    webrtc_session_desc_factory_.reset();
  }
  const WebRtcSessionDescriptionFactory* webrtc_session_desc_factory() const {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return webrtc_session_desc_factory_.get();
  }

  // Change signaling state to Closed, and perform appropriate actions.
  void Close();

  // Called as part of destroying the owning PeerConnection.
  void PrepareForShutdown();

  // Implementation of SdpStateProvider
  PeerConnectionInterface::SignalingState signaling_state() const override;

  const SessionDescriptionInterface* local_description() const override;
  const SessionDescriptionInterface* remote_description() const override;
  const SessionDescriptionInterface* current_local_description() const override;
  const SessionDescriptionInterface* current_remote_description()
      const override;
  const SessionDescriptionInterface* pending_local_description() const override;
  const SessionDescriptionInterface* pending_remote_description()
      const override;

  bool NeedsIceRestart(const std::string& content_name) const override;
  bool IceRestartPending(const std::string& content_name) const override;
  std::optional<SSLRole> GetDtlsRole(const std::string& mid) const override;

  void RestartIce();

  // JSEP01
  void CreateOffer(
      CreateSessionDescriptionObserver* observer,
      const PeerConnectionInterface::RTCOfferAnswerOptions& options);
  void CreateAnswer(
      CreateSessionDescriptionObserver* observer,
      const PeerConnectionInterface::RTCOfferAnswerOptions& options);

  void SetLocalDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetLocalDescriptionObserverInterface> observer);
  void SetLocalDescription(
      scoped_refptr<SetLocalDescriptionObserverInterface> observer);
  void SetLocalDescription(SetSessionDescriptionObserver* observer,
                           SessionDescriptionInterface* desc);
  void SetLocalDescription(SetSessionDescriptionObserver* observer);

  void SetRemoteDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetRemoteDescriptionObserverInterface> observer);
  void SetRemoteDescription(SetSessionDescriptionObserver* observer,
                            SessionDescriptionInterface* desc);

  PeerConnectionInterface::RTCConfiguration GetConfiguration();
  RTCError SetConfiguration(
      const PeerConnectionInterface::RTCConfiguration& configuration);
  bool AddIceCandidate(const IceCandidateInterface* candidate);
  void AddIceCandidate(std::unique_ptr<IceCandidateInterface> candidate,
                       std::function<void(RTCError)> callback);
  bool RemoveIceCandidates(const std::vector<Candidate>& candidates);
  // Adds a locally generated candidate to the local description.
  void AddLocalIceCandidate(const JsepIceCandidate* candidate);
  void RemoveLocalIceCandidates(const std::vector<Candidate>& candidates);
  bool ShouldFireNegotiationNeededEvent(uint32_t event_id);

  bool AddStream(MediaStreamInterface* local_stream);
  void RemoveStream(MediaStreamInterface* local_stream);

  std::optional<bool> is_caller() const;
  bool HasNewIceCredentials();
  void UpdateNegotiationNeeded();
  void AllocateSctpSids();
  // Based on the negotiation state, guess what the SSLRole might be without
  // directly getting the information from the transport.
  // This is used for allocating stream ids for data channels.
  // See also `InternalDataChannelInit::fallback_ssl_role`.
  std::optional<SSLRole> GuessSslRole() const;

  // Destroys all media BaseChannels.
  void DestroyMediaChannels();

  scoped_refptr<StreamCollectionInterface> local_streams();
  scoped_refptr<StreamCollectionInterface> remote_streams();

  bool initial_offerer() {
    RTC_DCHECK_RUN_ON(signaling_thread());
    if (initial_offerer_) {
      return *initial_offerer_;
    }
    return false;
  }

  SdpMungingType sdp_munging_type() const { return last_sdp_munging_type_; }
  void DisableSdpMungingChecksForTesting() {
    disable_sdp_munging_checks_ = true;
  }

 private:
  class RemoteDescriptionOperation;
  class ImplicitCreateSessionDescriptionObserver;

  friend class ImplicitCreateSessionDescriptionObserver;
  class SetSessionDescriptionObserverAdapter;

  friend class SetSessionDescriptionObserverAdapter;

  enum class SessionError {
    kNone,       // No error.
    kContent,    // Error in BaseChannel SetLocalContent/SetRemoteContent.
    kTransport,  // Error from the underlying transport.
  };

  // Represents the [[LocalIceCredentialsToReplace]] internal slot in the spec.
  // It makes the next CreateOffer() produce new ICE credentials even if
  // RTCOfferAnswerOptions::ice_restart is false.
  // https://w3c.github.io/webrtc-pc/#dfn-localufragstoreplace
  // TODO(hbos): When JsepTransportController/JsepTransport supports rollback,
  // move this type of logic to JsepTransportController/JsepTransport.
  class LocalIceCredentialsToReplace;

  // Only called by the Create() function.
  explicit SdpOfferAnswerHandler(PeerConnectionSdpMethods* pc,
                                 ConnectionContext* context);
  // Called from the `Create()` function. Can only be called
  // once. Modifies dependencies.
  void Initialize(
      const PeerConnectionInterface::RTCConfiguration& configuration,
      std::unique_ptr<RTCCertificateGeneratorInterface> cert_generator,
      std::unique_ptr<webrtc::VideoBitrateAllocatorFactory>
          video_bitrate_allocator_factory,
      ConnectionContext* context,
      CodecLookupHelper* codec_lookup_helper);

  Thread* signaling_thread() const;
  Thread* network_thread() const;
  // Non-const versions of local_description()/remote_description(), for use
  // internally.
  SessionDescriptionInterface* mutable_local_description()
      RTC_RUN_ON(signaling_thread()) {
    return pending_local_description_ ? pending_local_description_.get()
                                      : current_local_description_.get();
  }
  SessionDescriptionInterface* mutable_remote_description()
      RTC_RUN_ON(signaling_thread()) {
    return pending_remote_description_ ? pending_remote_description_.get()
                                       : current_remote_description_.get();
  }

  // Synchronous implementations of SetLocalDescription/SetRemoteDescription
  // that return an RTCError instead of invoking a callback.
  RTCError ApplyLocalDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid);
  void ApplyRemoteDescription(
      std::unique_ptr<RemoteDescriptionOperation> operation);

  RTCError ReplaceRemoteDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      SdpType sdp_type,
      std::unique_ptr<SessionDescriptionInterface>* replaced_description)
      RTC_RUN_ON(signaling_thread());

  // Part of ApplyRemoteDescription steps specific to Unified Plan.
  void ApplyRemoteDescriptionUpdateTransceiverState(SdpType sdp_type);

  // Part of ApplyRemoteDescription steps specific to plan b.
  void PlanBUpdateSendersAndReceivers(
      const ContentInfo* audio_content,
      const AudioContentDescription* audio_desc,
      const ContentInfo* video_content,
      const VideoContentDescription* video_desc);

  // Implementation of the offer/answer exchange operations. These are chained
  // onto the `operations_chain_` when the public CreateOffer(), CreateAnswer(),
  // SetLocalDescription() and SetRemoteDescription() methods are invoked.
  void DoCreateOffer(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options,
      scoped_refptr<CreateSessionDescriptionObserver> observer);
  void DoCreateAnswer(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options,
      scoped_refptr<CreateSessionDescriptionObserver> observer);
  void DoSetLocalDescription(
      std::unique_ptr<SessionDescriptionInterface> desc,
      scoped_refptr<SetLocalDescriptionObserverInterface> observer);
  void DoSetRemoteDescription(
      std::unique_ptr<RemoteDescriptionOperation> operation);

  // Called after a DoSetRemoteDescription operation completes.
  void SetRemoteDescriptionPostProcess(bool was_answer)
      RTC_RUN_ON(signaling_thread());

  // Update the state, signaling if necessary.
  void ChangeSignalingState(
      PeerConnectionInterface::SignalingState signaling_state);

  RTCError UpdateSessionState(
      SdpType type,
      ContentSource source,
      const SessionDescription* description,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid);

  bool IsUnifiedPlan() const;

  // Signals from MediaStreamObserver.
  void OnAudioTrackAdded(AudioTrackInterface* track,
                         MediaStreamInterface* stream)
      RTC_RUN_ON(signaling_thread());
  void OnAudioTrackRemoved(AudioTrackInterface* track,
                           MediaStreamInterface* stream)
      RTC_RUN_ON(signaling_thread());
  void OnVideoTrackAdded(VideoTrackInterface* track,
                         MediaStreamInterface* stream)
      RTC_RUN_ON(signaling_thread());
  void OnVideoTrackRemoved(VideoTrackInterface* track,
                           MediaStreamInterface* stream)
      RTC_RUN_ON(signaling_thread());

  // | desc_type | is the type of the description that caused the rollback.
  RTCError Rollback(SdpType desc_type);
  void OnOperationsChainEmpty();

  // Runs the algorithm **set the associated remote streams** specified in
  // https://w3c.github.io/webrtc-pc/#set-associated-remote-streams.
  void SetAssociatedRemoteStreams(
      scoped_refptr<RtpReceiverInternal> receiver,
      const std::vector<std::string>& stream_ids,
      std::vector<scoped_refptr<MediaStreamInterface>>* added_streams,
      std::vector<scoped_refptr<MediaStreamInterface>>* removed_streams);

  bool CheckIfNegotiationIsNeeded();
  void GenerateNegotiationNeededEvent();
  // Helper method which verifies SDP.
  RTCError ValidateSessionDescription(
      const SessionDescriptionInterface* sdesc,
      ContentSource source,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid)
      RTC_RUN_ON(signaling_thread());

  // Updates the local RtpTransceivers according to the JSEP rules. Called as
  // part of setting the local/remote description.
  RTCError UpdateTransceiversAndDataChannels(
      ContentSource source,
      const SessionDescriptionInterface& new_session,
      const SessionDescriptionInterface* old_local_description,
      const SessionDescriptionInterface* old_remote_description,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid);

  // Associate the given transceiver according to the JSEP rules.
  RTCErrorOr<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  AssociateTransceiver(ContentSource source,
                       SdpType type,
                       size_t mline_index,
                       const ContentInfo& content,
                       const ContentInfo* old_local_content,
                       const ContentInfo* old_remote_content)
      RTC_RUN_ON(signaling_thread());

  // Returns the media section in the given session description that is
  // associated with the RtpTransceiver. Returns null if none found or this
  // RtpTransceiver is not associated. Logic varies depending on the
  // SdpSemantics specified in the configuration.
  const ContentInfo* FindMediaSectionForTransceiver(
      const RtpTransceiver* transceiver,
      const SessionDescriptionInterface* sdesc) const;

  // Either creates or destroys the transceiver's BaseChannel according to the
  // given media section.
  RTCError UpdateTransceiverChannel(
      scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
          transceiver,
      const ContentInfo& content,
      const ContentGroup* bundle_group) RTC_RUN_ON(signaling_thread());

  // Either creates or destroys the local data channel according to the given
  // media section.
  RTCError UpdateDataChannelTransport(ContentSource source,
                                      const ContentInfo& content,
                                      const ContentGroup* bundle_group)
      RTC_RUN_ON(signaling_thread());
  // Check if a call to SetLocalDescription is acceptable with a session
  // description of the given type.
  bool ExpectSetLocalDescription(SdpType type);
  // Check if a call to SetRemoteDescription is acceptable with a session
  // description of the given type.
  bool ExpectSetRemoteDescription(SdpType type);

  // The offer/answer machinery assumes the media section MID is present and
  // unique. To support legacy end points that do not supply a=mid lines, this
  // method will modify the session description to add MIDs generated according
  // to the SDP semantics.
  void FillInMissingRemoteMids(SessionDescription* remote_description);

  // Returns an RtpTransceiver, if available, that can be used to receive the
  // given media type according to JSEP rules.
  scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
  FindAvailableTransceiverToReceive(webrtc::MediaType media_type) const;

  // Returns a MediaSessionOptions struct with options decided by `options`,
  // the local MediaStreams and DataChannels.
  void GetOptionsForOffer(const PeerConnectionInterface::RTCOfferAnswerOptions&
                              offer_answer_options,
                          MediaSessionOptions* session_options);
  void GetOptionsForPlanBOffer(
      const PeerConnectionInterface::RTCOfferAnswerOptions&
          offer_answer_options,
      MediaSessionOptions* session_options) RTC_RUN_ON(signaling_thread());
  void GetOptionsForUnifiedPlanOffer(
      const PeerConnectionInterface::RTCOfferAnswerOptions&
          offer_answer_options,
      MediaSessionOptions* session_options) RTC_RUN_ON(signaling_thread());

  // Returns a MediaSessionOptions struct with options decided by
  // `constraints`, the local MediaStreams and DataChannels.
  void GetOptionsForAnswer(const PeerConnectionInterface::RTCOfferAnswerOptions&
                               offer_answer_options,
                           MediaSessionOptions* session_options);
  void GetOptionsForPlanBAnswer(
      const PeerConnectionInterface::RTCOfferAnswerOptions&
          offer_answer_options,
      MediaSessionOptions* session_options) RTC_RUN_ON(signaling_thread());
  void GetOptionsForUnifiedPlanAnswer(
      const PeerConnectionInterface::RTCOfferAnswerOptions&
          offer_answer_options,
      MediaSessionOptions* session_options) RTC_RUN_ON(signaling_thread());

  const char* SessionErrorToString(SessionError error) const;
  std::string GetSessionErrorMsg();
  // Returns the last error in the session. See the enum above for details.
  SessionError session_error() const {
    RTC_DCHECK_RUN_ON(signaling_thread());
    return session_error_;
  }
  const std::string& session_error_desc() const { return session_error_desc_; }

  RTCError HandleLegacyOfferOptions(
      const PeerConnectionInterface::RTCOfferAnswerOptions& options);
  void RemoveRecvDirectionFromReceivingTransceiversOfType(
      webrtc::MediaType media_type) RTC_RUN_ON(signaling_thread());
  void AddUpToOneReceivingTransceiverOfType(webrtc::MediaType media_type);

  std::vector<scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>>
  GetReceivingTransceiversOfType(webrtc::MediaType media_type)
      RTC_RUN_ON(signaling_thread());

  // Runs the algorithm specified in
  // https://w3c.github.io/webrtc-pc/#process-remote-track-removal
  // This method will update the following lists:
  // `remove_list` is the list of transceivers for which the receiving track is
  //     being removed.
  // `removed_streams` is the list of streams which no longer have a receiving
  //     track so should be removed.
  void ProcessRemovalOfRemoteTrack(
      const scoped_refptr<RtpTransceiverProxyWithInternal<RtpTransceiver>>
          transceiver,
      std::vector<scoped_refptr<RtpTransceiverInterface>>* remove_list,
      std::vector<scoped_refptr<MediaStreamInterface>>* removed_streams);

  void RemoveRemoteStreamsIfEmpty(
      const std::vector<scoped_refptr<MediaStreamInterface>>& remote_streams,
      std::vector<scoped_refptr<MediaStreamInterface>>* removed_streams);

  // Remove all local and remote senders of type `media_type`.
  // Called when a media type is rejected (m-line set to port 0).
  void RemoveSenders(webrtc::MediaType media_type);

  // Loops through the vector of `streams` and finds added and removed
  // StreamParams since last time this method was called.
  // For each new or removed StreamParam, OnLocalSenderSeen or
  // OnLocalSenderRemoved is invoked.
  void UpdateLocalSenders(const std::vector<StreamParams>& streams,
                          webrtc::MediaType media_type);

  // Makes sure a MediaStreamTrack is created for each StreamParam in `streams`,
  // and existing MediaStreamTracks are removed if there is no corresponding
  // StreamParam. If `default_track_needed` is true, a default MediaStreamTrack
  // is created if it doesn't exist; if false, it's removed if it exists.
  // `media_type` is the type of the `streams` and can be either audio or video.
  // If a new MediaStream is created it is added to `new_streams`.
  void UpdateRemoteSendersList(const std::vector<StreamParams>& streams,
                               bool default_track_needed,
                               webrtc::MediaType media_type,
                               StreamCollection* new_streams);

  // Enables media channels to allow sending of media.
  // This enables media to flow on all configured audio/video channels.
  void EnableSending();
  // Push the media parts of the local or remote session description
  // down to all of the channels, and start SCTP if needed.
  RTCError PushdownMediaDescription(
      SdpType type,
      ContentSource source,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid);

  RTCError PushdownTransportDescription(ContentSource source, SdpType type);
  // Helper function to remove stopped transceivers.
  void RemoveStoppedTransceivers();
  // Deletes the corresponding channel of contents that don't exist in `desc`.
  // `desc` can be null. This means that all channels are deleted.
  void RemoveUnusedChannels(const SessionDescription* desc);

  // Finds remote MediaStreams without any tracks and removes them from
  // `remote_streams_` and notifies the observer that the MediaStreams no longer
  // exist.
  void UpdateEndedRemoteMediaStreams();

  // Uses all remote candidates in the currently set remote_description().
  // If no remote description is currently set (nullptr), the return value will
  // be true. If `UseCandidate()` fails for any candidate in the remote
  // description, the return value will be false.
  bool UseCandidatesInRemoteDescription();
  // Uses `candidate` in this session.
  bool UseCandidate(const IceCandidateInterface* candidate);
  // Returns true if we are ready to push down the remote candidate.
  // `remote_desc` is the new remote description, or NULL if the current remote
  // description should be used. Output `valid` is true if the candidate media
  // index is valid.
  bool ReadyToUseRemoteCandidate(const IceCandidateInterface* candidate,
                                 const SessionDescriptionInterface* remote_desc,
                                 bool* valid);

  RTCErrorOr<const ContentInfo*> FindContentInfo(
      const SessionDescriptionInterface* description,
      const IceCandidateInterface* candidate) RTC_RUN_ON(signaling_thread());

  // Functions for dealing with transports.
  // Note that cricket code uses the term "channel" for what other code
  // refers to as "transport".

  // Allocates media channels based on the `desc`. If `desc` doesn't have
  // the BUNDLE option, this method will disable BUNDLE in PortAllocator.
  // This method will also delete any existing media channels before creating.
  RTCError CreateChannels(const SessionDescription& desc);

  // Generates MediaDescriptionOptions for the `session_opts` based on existing
  // local description or remote description.
  void GenerateMediaDescriptionOptions(
      const SessionDescriptionInterface* session_desc,
      RtpTransceiverDirection audio_direction,
      RtpTransceiverDirection video_direction,
      std::optional<size_t>* audio_index,
      std::optional<size_t>* video_index,
      std::optional<size_t>* data_index,
      MediaSessionOptions* session_options);

  // Generates the active MediaDescriptionOptions for the local data channel
  // given the specified MID.
  MediaDescriptionOptions GetMediaDescriptionOptionsForActiveData(
      const std::string& mid) const;

  // Generates the rejected MediaDescriptionOptions for the local data channel
  // given the specified MID.
  MediaDescriptionOptions GetMediaDescriptionOptionsForRejectedData(
      const std::string& mid) const;

  // Based on number of transceivers per media type, enabled or disable
  // payload type based demuxing in the affected channels.
  bool UpdatePayloadTypeDemuxingState(
      ContentSource source,
      const std::map<std::string, const ContentGroup*>& bundle_groups_by_mid);

  // Updates the error state, signaling if necessary.
  void SetSessionError(SessionError error, const std::string& error_desc);

  // Implements AddIceCandidate without reporting usage, but returns the
  // particular success/error value that should be reported (and can be utilized
  // for other purposes).
  AddIceCandidateResult AddIceCandidateInternal(
      const IceCandidateInterface* candidate);

  void ReportInitialSdpMunging(bool had_local_description, SdpType type);

  // ==================================================================
  // Access to pc_ variables
  MediaEngineInterface* media_engine() const;
  TransceiverList* transceivers();
  const TransceiverList* transceivers() const;
  DataChannelController* data_channel_controller();
  const DataChannelController* data_channel_controller() const;
  PortAllocator* port_allocator();
  const PortAllocator* port_allocator() const;
  RtpTransmissionManager* rtp_manager();
  const RtpTransmissionManager* rtp_manager() const;
  JsepTransportController* transport_controller_s()
      RTC_RUN_ON(signaling_thread());
  const JsepTransportController* transport_controller_s() const
      RTC_RUN_ON(signaling_thread());
  JsepTransportController* transport_controller_n()
      RTC_RUN_ON(network_thread());
  const JsepTransportController* transport_controller_n() const
      RTC_RUN_ON(network_thread());
  // ===================================================================
  const AudioOptions& audio_options() { return audio_options_; }
  const VideoOptions& video_options() { return video_options_; }
  bool ConfiguredForMedia() const;

  PeerConnectionSdpMethods* const pc_;
  ConnectionContext* const context_;

  std::unique_ptr<WebRtcSessionDescriptionFactory> webrtc_session_desc_factory_
      RTC_GUARDED_BY(signaling_thread());

  std::unique_ptr<SessionDescriptionInterface> current_local_description_
      RTC_GUARDED_BY(signaling_thread());
  std::unique_ptr<SessionDescriptionInterface> pending_local_description_
      RTC_GUARDED_BY(signaling_thread());
  std::unique_ptr<SessionDescriptionInterface> current_remote_description_
      RTC_GUARDED_BY(signaling_thread());
  std::unique_ptr<SessionDescriptionInterface> pending_remote_description_
      RTC_GUARDED_BY(signaling_thread());
  std::unique_ptr<SessionDescriptionInterface> last_created_offer_
      RTC_GUARDED_BY(signaling_thread());
  std::unique_ptr<SessionDescriptionInterface> last_created_answer_
      RTC_GUARDED_BY(signaling_thread());
  SdpMungingType last_sdp_munging_type_ = SdpMungingType::kNoModification;

  PeerConnectionInterface::SignalingState signaling_state_
      RTC_GUARDED_BY(signaling_thread()) = PeerConnectionInterface::kStable;

  // Whether this peer is the caller. Set when the local description is applied.
  std::optional<bool> is_caller_ RTC_GUARDED_BY(signaling_thread());

  // Streams added via AddStream.
  const scoped_refptr<StreamCollection> local_streams_
      RTC_GUARDED_BY(signaling_thread());
  // Streams created as a result of SetRemoteDescription.
  const scoped_refptr<StreamCollection> remote_streams_
      RTC_GUARDED_BY(signaling_thread());

  std::vector<std::unique_ptr<MediaStreamObserver>> stream_observers_
      RTC_GUARDED_BY(signaling_thread());

  // The operations chain is used by the offer/answer exchange methods to ensure
  // they are executed in the right order. For example, if
  // SetRemoteDescription() is invoked while CreateOffer() is still pending, the
  // SRD operation will not start until CreateOffer() has completed. See
  // https://w3c.github.io/webrtc-pc/#dfn-operations-chain.
  scoped_refptr<OperationsChain> operations_chain_
      RTC_GUARDED_BY(signaling_thread());

  // One PeerConnection has only one RTCP CNAME.
  // https://tools.ietf.org/html/draft-ietf-rtcweb-rtp-usage-26#section-4.9
  const std::string rtcp_cname_;

  // MIDs will be generated using this generator which will keep track of
  // all the MIDs that have been seen over the life of the PeerConnection.
  UniqueStringGenerator mid_generator_ RTC_GUARDED_BY(signaling_thread());

  // List of content names for which the remote side triggered an ICE restart.
  std::set<std::string> pending_ice_restarts_
      RTC_GUARDED_BY(signaling_thread());

  std::unique_ptr<LocalIceCredentialsToReplace>
      local_ice_credentials_to_replace_ RTC_GUARDED_BY(signaling_thread());

  bool remote_peer_supports_msid_ RTC_GUARDED_BY(signaling_thread()) = false;
  bool is_negotiation_needed_ RTC_GUARDED_BY(signaling_thread()) = false;
  uint32_t negotiation_needed_event_id_ RTC_GUARDED_BY(signaling_thread()) = 0;
  bool update_negotiation_needed_on_empty_chain_
      RTC_GUARDED_BY(signaling_thread()) = false;
  // If PT demuxing is successfully negotiated one time we will allow PT
  // demuxing for the rest of the session so that PT-based apps default to PT
  // demuxing in follow-up O/A exchanges.
  bool pt_demuxing_has_been_used_audio_ RTC_GUARDED_BY(signaling_thread()) =
      false;
  bool pt_demuxing_has_been_used_video_ RTC_GUARDED_BY(signaling_thread()) =
      false;

  // In Unified Plan, if we encounter remote SDP that does not contain an a=msid
  // line we create and use a stream with a random ID for our receivers. This is
  // to support legacy endpoints that do not support the a=msid attribute (as
  // opposed to streamless tracks with "a=msid:-").
  scoped_refptr<MediaStreamInterface> missing_msid_default_stream_
      RTC_GUARDED_BY(signaling_thread());

  SessionError session_error_ RTC_GUARDED_BY(signaling_thread()) =
      SessionError::kNone;
  std::string session_error_desc_ RTC_GUARDED_BY(signaling_thread());

  // Member variables for caching global options.
  AudioOptions audio_options_ RTC_GUARDED_BY(signaling_thread());
  VideoOptions video_options_ RTC_GUARDED_BY(signaling_thread());

  // A video bitrate allocator factory.
  // This can be injected using the PeerConnectionDependencies,
  // or else the CreateBuiltinVideoBitrateAllocatorFactory() will be called.
  // Note that one can still choose to override this in a MediaEngine
  // if one wants too.
  std::unique_ptr<VideoBitrateAllocatorFactory> video_bitrate_allocator_factory_
      RTC_GUARDED_BY(signaling_thread());

  // Whether we are the initial offerer on the association. This
  // determines the SSL role.
  std::optional<bool> initial_offerer_ RTC_GUARDED_BY(signaling_thread());

  // Whether SDP munging checks are enabled or not.
  // Some tests will be detected as SDP munging, so offer the option
  // to disable.
  bool disable_sdp_munging_checks_ = false;
  CodecLookupHelper* codec_lookup_helper_ = nullptr;

  // Whether the username fragment or the password of the SDP was munged.
  bool has_sdp_munged_ufrag_ = false;

  WeakPtrFactory<SdpOfferAnswerHandler> weak_ptr_factory_
      RTC_GUARDED_BY(signaling_thread());
};

}  // namespace webrtc

#endif  // PC_SDP_OFFER_ANSWER_H_
