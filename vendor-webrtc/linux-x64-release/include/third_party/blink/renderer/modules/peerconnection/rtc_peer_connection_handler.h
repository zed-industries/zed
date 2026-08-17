// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_H_

#include <stddef.h>

#include <array>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/public/mojom/peerconnection/peer_connection_tracker.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/media_stream_track_metrics.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_receiver_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sender_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/thermal_resource.h"
#include "third_party/blink/renderer/modules/peerconnection/transceiver_state_surfacer.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_peer_connection_handler_client.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_session_description_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_session_description_request.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/peer_connection_interface.h"
#include "third_party/webrtc/api/rtc_error.h"
#include "third_party/webrtc/api/rtp_transceiver_interface.h"
#include "third_party/webrtc/api/stats/rtc_stats.h"
#include "third_party/webrtc/api/stats/rtc_stats_collector_callback.h"

namespace blink {
class PeerConnectionDependencyFactory;
class PeerConnectionTracker;
class RTCAnswerOptionsPlatform;
class RTCOfferOptionsPlatform;
class RTCPeerConnectionHandlerClient;
class RTCSessionDescriptionInit;
class RTCVoidRequest;
class SetLocalDescriptionRequest;
class WebLocalFrame;

// Helper class for passing pre-parsed session descriptions to functions.
// Create a ParsedSessionDescription by calling one of the Parse functions.
// The function allows you to access its input SDP string, as well as the
// parsed form. If parse failed, description() returns null, and error()
// returns an error description. The class can't be modified or reused,
// but can be neutered by calling release().
class MODULES_EXPORT ParsedSessionDescription {
 public:
  static ParsedSessionDescription Parse(const RTCSessionDescriptionInit*);
  static ParsedSessionDescription Parse(const RTCSessionDescriptionPlatform*);
  static ParsedSessionDescription Parse(const String& type, const String& sdp);

  // Moveable, but not copyable.
  ParsedSessionDescription(ParsedSessionDescription&& other) = default;
  ParsedSessionDescription& operator=(ParsedSessionDescription&& other) =
      default;
  ParsedSessionDescription(const ParsedSessionDescription&) = delete;
  ParsedSessionDescription operator=(const ParsedSessionDescription&) = delete;

  const webrtc::SessionDescriptionInterface* description() const {
    return description_.get();
  }
  const webrtc::SdpParseError& error() const { return error_; }
  const String& type() const { return type_; }
  const String& sdp() const { return sdp_; }

  std::unique_ptr<webrtc::SessionDescriptionInterface> release() {
    return std::unique_ptr<webrtc::SessionDescriptionInterface>(
        description_.release());
  }

 protected:
  // The constructor will not parse the SDP.
  // It is protected, not private, in order to allow it to be used to construct
  // mock objects.
  ParsedSessionDescription(const String& type, const String& sdp);
  // The mock object also needs access to the description.
  std::unique_ptr<webrtc::SessionDescriptionInterface> description_;

 private:
  // Does the actual parsing. Only called from the static Parse methods.
  void DoParse();

  String type_;
  String sdp_;

  webrtc::SdpParseError error_;
};

// RTCPeerConnectionHandler is a delegate for the RTCPeerConnection API
// messages going between WebKit and native PeerConnection in WebRTC. It's
// owned by Blink.
// Blink calls all of these methods on the main render thread.
// Callbacks to the webrtc::PeerConnectionObserver implementation also occur on
// the main render thread.
class MODULES_EXPORT RTCPeerConnectionHandler {
 public:
  RTCPeerConnectionHandler(
      RTCPeerConnectionHandlerClient* client,
      blink::PeerConnectionDependencyFactory* dependency_factory,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      bool encoded_insertable_streams);

  RTCPeerConnectionHandler(const RTCPeerConnectionHandler&) = delete;
  RTCPeerConnectionHandler& operator=(const RTCPeerConnectionHandler&) = delete;

  virtual ~RTCPeerConnectionHandler();

  base::WeakPtr<RTCPeerConnectionHandler> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }
  // Initialize method only used for unit test.
  bool InitializeForTest(
      const webrtc::PeerConnectionInterface::RTCConfiguration&
          server_configuration,
      PeerConnectionTracker* peer_connection_tracker,
      ExceptionState& exception_state,
      RTCRtpTransport* rtp_transport);

  virtual bool Initialize(
      ExecutionContext* context,
      const webrtc::PeerConnectionInterface::RTCConfiguration&
          server_configuration,
      WebLocalFrame* web_frame,
      ExceptionState& exception_state,
      RTCRtpTransport* rtp_transport);

  virtual void Close();
  virtual void CloseAndUnregister();

  virtual Vector<std::unique_ptr<RTCRtpTransceiverPlatform>> CreateOffer(
      RTCSessionDescriptionRequest* request,
      RTCOfferOptionsPlatform* options);

  virtual void CreateAnswer(blink::RTCSessionDescriptionRequest* request,
                            blink::RTCAnswerOptionsPlatform* options);

  virtual void SetLocalDescription(blink::RTCVoidRequest* request);
  // Set local and remote description.
  // The parsed_sdp argument must be passed in with std::move.
  virtual void SetLocalDescription(blink::RTCVoidRequest* request,
                                   ParsedSessionDescription parsed_sdp);
  virtual void SetRemoteDescription(blink::RTCVoidRequest* request,
                                    ParsedSessionDescription parsed_sdp);

  virtual const webrtc::PeerConnectionInterface::RTCConfiguration&
  GetConfiguration() const;
  virtual webrtc::RTCErrorType SetConfiguration(
      const webrtc::PeerConnectionInterface::RTCConfiguration& configuration);
  virtual void AddIceCandidate(blink::RTCVoidRequest* request,
                               RTCIceCandidatePlatform* candidate);
  virtual void RestartIce();

  virtual void GetStats(RTCStatsReportCallback callback);
  virtual webrtc::RTCErrorOr<std::unique_ptr<RTCRtpTransceiverPlatform>>
  AddTransceiverWithTrack(MediaStreamComponent* component,
                          const webrtc::RtpTransceiverInit& init);
  virtual webrtc::RTCErrorOr<std::unique_ptr<RTCRtpTransceiverPlatform>>
  AddTransceiverWithKind(const String& kind,
                         const webrtc::RtpTransceiverInit& init);
  virtual webrtc::RTCErrorOr<std::unique_ptr<RTCRtpTransceiverPlatform>>
  AddTrack(MediaStreamComponent* component,
           const MediaStreamDescriptorVector& descriptors);
  virtual webrtc::RTCErrorOr<std::unique_ptr<RTCRtpTransceiverPlatform>>
  RemoveTrack(blink::RTCRtpSenderPlatform* web_sender);

  Vector<std::unique_ptr<blink::RTCRtpSenderPlatform>> GetPlatformSenders()
      const;

  virtual webrtc::scoped_refptr<webrtc::DataChannelInterface> CreateDataChannel(
      const String& label,
      const webrtc::DataChannelInit& init);
  virtual webrtc::PeerConnectionInterface* NativePeerConnection();
  virtual void RunSynchronousOnceClosureOnSignalingThread(
      base::OnceClosure closure,
      const char* trace_event_name);

  virtual void TrackIceConnectionStateChange(
      webrtc::PeerConnectionInterface::IceConnectionState state);

  // Asynchronously calls native_peer_connection_->getStats on the signaling
  // thread. (Future cleanup potential: just use the other GetStats() method?)
  void GetStandardStatsForTracker(
      webrtc::scoped_refptr<webrtc::RTCStatsCollectorCallback> observer);

  // Allows webrtc-internals to request a brief dump of the current state.
  void EmitCurrentStateForTracker();

  // Tells the |client_| to close RTCPeerConnection.
  // Make it virtual for testing purpose.
  virtual void CloseClientPeerConnection();
  // Invoked when a new thermal state is received from the OS.
  // Virtual for testing purposes.
  virtual void OnThermalStateChange(
      mojom::blink::DeviceThermalState thermal_state);

  // Start recording an event log.
  void StartEventLog(int output_period_ms);
  // Stop recording an event log.
  void StopEventLog();

  // WebRTC event log fragments sent back from PeerConnection land here.
  void OnWebRtcEventLogWrite(const WTF::Vector<uint8_t>& output);

  // Virtual for testing purposes.
  virtual scoped_refptr<base::SingleThreadTaskRunner> signaling_thread() const;

  bool encoded_insertable_streams() { return encoded_insertable_streams_; }
  blink::WebLocalFrame* frame() const { return frame_; }

 protected:
  // Constructor to be used for constructing mocks only.
  explicit RTCPeerConnectionHandler(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  webrtc::PeerConnectionInterface* native_peer_connection() {
    return native_peer_connection_.get();
  }

  class Observer;
  friend class Observer;
  class WebRtcSetDescriptionObserverImpl;
  friend class WebRtcSetDescriptionObserverImpl;
  class SetLocalDescriptionRequest;
  friend class SetLocalDescriptionRequest;

  void OnSessionDescriptionsUpdated(
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          pending_local_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          current_local_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          pending_remote_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          current_remote_description);
  void TrackSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state);
  void OnIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state);
  void OnConnectionChange(
      webrtc::PeerConnectionInterface::PeerConnectionState new_state);
  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state);
  void OnNegotiationNeededEvent(uint32_t event_id);
  void OnModifySctpTransport(blink::WebRTCSctpTransportSnapshot state);
  void OnModifyTransceivers(
      webrtc::PeerConnectionInterface::SignalingState signaling_state,
      std::vector<blink::RtpTransceiverState> transceiver_states,
      bool is_remote_description,
      bool is_rollback);
  void OnDataChannel(
      webrtc::scoped_refptr<webrtc::DataChannelInterface> channel);
  void OnIceCandidate(const String& sdp,
                      const String& sdp_mid,
                      int sdp_mline_index,
                      int component,
                      int address_family,
                      const String& usernameFragment,
                      const String& url);
  void OnIceCandidateError(const String& address,
                           std::optional<uint16_t> port,
                           const String& host_candidate,
                           const String& url,
                           int error_code,
                           const String& error_text);
  void OnInterestingUsage(int usage_pattern);

 private:
  // Record info about the first SessionDescription from the local and
  // remote side to record UMA stats once both are set.
  struct FirstSessionDescription {
    explicit FirstSessionDescription(
        const webrtc::SessionDescriptionInterface* desc);

    bool audio = false;
    bool video = false;
    // If audio or video will use RTCP-mux (if there is no audio or
    // video, then false).
    bool rtcp_mux = false;
  };

  // Report to UMA whether an IceConnectionState has occurred. It only records
  // the first occurrence of a given state.
  void ReportICEState(
      webrtc::PeerConnectionInterface::IceConnectionState new_state);

  void ReportFirstSessionDescriptions(const FirstSessionDescription& local,
                                      const FirstSessionDescription& remote);

  void AddTransceiverWithTrackOnSignalingThread(
      webrtc::MediaStreamTrackInterface* webrtc_track,
      webrtc::RtpTransceiverInit init,
      blink::TransceiverStateSurfacer* transceiver_state_surfacer,
      webrtc::RTCErrorOr<
          webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>>*
          error_or_transceiver);
  void AddTransceiverWithMediaTypeOnSignalingThread(
      webrtc::MediaType media_type,
      webrtc::RtpTransceiverInit init,
      blink::TransceiverStateSurfacer* transceiver_state_surfacer,
      webrtc::RTCErrorOr<
          webrtc::scoped_refptr<webrtc::RtpTransceiverInterface>>*
          error_or_transceiver);
  void AddTrackOnSignalingThread(
      webrtc::MediaStreamTrackInterface* track,
      std::vector<std::string> stream_ids,
      blink::TransceiverStateSurfacer* transceiver_state_surfacer,
      webrtc::RTCErrorOr<webrtc::scoped_refptr<webrtc::RtpSenderInterface>>*
          error_or_sender);
  // Helper function to remove a track on the signaling thread.
  // Updates the entire transceiver state.
  // The result will be std::nullopt if the operation is cancelled,
  // and no change to the state will be made.
  void RemoveTrackOnSignalingThread(
      webrtc::RtpSenderInterface* sender,
      blink::TransceiverStateSurfacer* transceiver_state_surfacer,
      std::optional<webrtc::RTCError>* result);
  void CreateOfferOnSignalingThread(
      webrtc::CreateSessionDescriptionObserver* observer,
      webrtc::PeerConnectionInterface::RTCOfferAnswerOptions offer_options,
      blink::TransceiverStateSurfacer* transceiver_state_surfacer);
  Vector<std::unique_ptr<blink::RTCRtpSenderImpl>>::iterator FindSender(
      uintptr_t id);
  Vector<std::unique_ptr<blink::RTCRtpReceiverImpl>>::iterator FindReceiver(
      uintptr_t id);
  Vector<std::unique_ptr<blink::RTCRtpTransceiverImpl>>::iterator
  FindTransceiver(uintptr_t id);
  // For full transceiver implementations, returns the index of
  // |rtp_transceivers_| that correspond to |platform_transceiver|.
  // For sender-only transceiver implementations, returns the index of
  // |rtp_senders_| that correspond to |platform_transceiver.Sender()|.
  // For receiver-only transceiver implementations, returns the index of
  // |rtp_receivers_| that correspond to |platform_transceiver.Receiver()|.
  // NOTREACHED()-crashes if no correspondent is found.
  wtf_size_t GetTransceiverIndex(
      const RTCRtpTransceiverPlatform& platform_transceiver);
  std::unique_ptr<blink::RTCRtpTransceiverImpl> CreateOrUpdateTransceiver(
      blink::RtpTransceiverState transceiver_state,
      blink::TransceiverStateUpdateMode update_mode);
  void MaybeCreateThermalUmaListner();

  // Initialize() is never expected to be called more than once, even if the
  // first call fails.
  bool initialize_called_ = false;

  // |client_| points to the blink object (blink::RTCPeerConnection)
  // that owns this object.
  WeakPersistent<RTCPeerConnectionHandlerClient> client_;
  // True if this PeerConnection has been closed.
  // After the PeerConnection has been closed, this object may no longer
  // forward callbacks to blink.
  bool is_closed_ = false;
  // True if CloseAndUnregister has been called.
  bool is_unregistered_ = false;

  // Transition from kHaveLocalOffer to kHaveRemoteOffer indicates implicit
  // rollback in which case we need to also make visiting of kStable observable.
  webrtc::PeerConnectionInterface::SignalingState previous_signaling_state_ =
      webrtc::PeerConnectionInterface::kStable;

  // Will be reset to nullptr when the handler is `CloseAndUnregister()`-ed, so
  // it doesn't prevent the factory from being garbage-collected.
  Persistent<PeerConnectionDependencyFactory> dependency_factory_;
  // Reference that outlives `dependency_factory_`, used to free WebRTC
  // references on the signaling thread during GC.
  scoped_refptr<base::SingleThreadTaskRunner> signaling_thread_;

  raw_ptr<blink::WebLocalFrame> frame_ = nullptr;

  // Map and owners of track adapters. Every track that is in use by the peer
  // connection has an associated blink and webrtc layer representation of it.
  // The map keeps track of the relationship between
  // |MediaStreamComponent|s and |webrtc::MediaStreamTrackInterface|s.
  // Track adapters are created on the fly when a component (such as a stream)
  // needs to reference it, and automatically disposed when there are no longer
  // any components referencing it.
  scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map_;
  // Blink layer correspondents of |webrtc::RtpSenderInterface|.
  Vector<std::unique_ptr<blink::RTCRtpSenderImpl>> rtp_senders_;
  // Blink layer correspondents of |webrtc::RtpReceiverInterface|.
  Vector<std::unique_ptr<blink::RTCRtpReceiverImpl>> rtp_receivers_;
  // Blink layer correspondents of |webrtc::RtpTransceiverInterface|.
  Vector<std::unique_ptr<blink::RTCRtpTransceiverImpl>> rtp_transceivers_;
  // A snapshot of transceiver ids taken before the last transition, used to
  // detect any removals during rollback.
  Vector<uintptr_t> previous_transceiver_ids_;

  WeakPersistent<PeerConnectionTracker> peer_connection_tracker_;

  MediaStreamTrackMetrics track_metrics_;

  // To make sure the observers are released after native_peer_connection_,
  // they have to come first.
  CrossThreadPersistent<Observer> peer_connection_observer_;

  // |native_peer_connection_| is the WebRTC native PeerConnection object.
  webrtc::scoped_refptr<webrtc::PeerConnectionInterface>
      native_peer_connection_;

  // The last applied configuration. Used so that the constraints
  // used when constructing the PeerConnection carry over when
  // SetConfiguration is called.
  webrtc::PeerConnectionInterface::RTCConfiguration configuration_;
  bool encoded_insertable_streams_ = false;

  // Resources for Adaptation.
  // The Thermal Resource is lazily instantiated on platforms where thermal
  // signals are supported.
  scoped_refptr<ThermalResource> thermal_resource_;

  // Record info about the first SessionDescription from the local and
  // remote side to record UMA stats once both are set.  We only check
  // for the first offer or answer.  "pranswer"s and "unknown"s (from
  // unit tests) are ignored.
  std::unique_ptr<FirstSessionDescription> first_local_description_;
  std::unique_ptr<FirstSessionDescription> first_remote_description_;

  // Track which ICE Connection state that this PeerConnection has gone through.
  std::array<bool, webrtc::PeerConnectionInterface::kIceConnectionMax>
      ice_state_seen_ = {};

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  base::WeakPtrFactory<RTCPeerConnectionHandler> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_PEER_CONNECTION_HANDLER_H_
