// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_TRACKER_H_

#include "base/gtest_prod_util.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/types/pass_key.h"
#include "base/values.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/receiver.h"
#include "third_party/blink/public/mojom/peerconnection/peer_connection_tracker.mojom-blink.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_peer_connection_handler_client.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_session_description_platform.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/peer_connection_interface.h"

namespace webrtc {
class DataChannelInterface;
}  // namespace webrtc

namespace blink {
class LocalFrame;
class MockPeerConnectionTracker;
class PeerConnectionTrackerTest;
class RTCAnswerOptionsPlatform;
class RTCIceCandidatePlatform;
class RTCOfferOptionsPlatform;
class RTCPeerConnectionHandler;
class UserMediaRequest;
class WebLocalFrame;

// This class collects data about each peer connection,
// sends it to the browser process, and handles messages
// from the browser process.
class MODULES_EXPORT PeerConnectionTracker
    : public GarbageCollected<PeerConnectionTracker>,
      public Supplement<LocalDOMWindow>,
      public blink::mojom::blink::PeerConnectionManager {
 public:
  static const char kSupplementName[];

  static PeerConnectionTracker& From(LocalDOMWindow& window);
  static PeerConnectionTracker* From(LocalFrame& frame);
  static PeerConnectionTracker* From(WebLocalFrame& frame);
  PeerConnectionTracker(
      LocalDOMWindow& window,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner,
      base::PassKey<PeerConnectionTracker>);

  PeerConnectionTracker(const PeerConnectionTracker&) = delete;
  PeerConnectionTracker& operator=(const PeerConnectionTracker&) = delete;

  ~PeerConnectionTracker() override;

  // Ctors for tests.
  PeerConnectionTracker(
      mojo::PendingRemote<mojom::blink::PeerConnectionTrackerHost> host,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner,
      base::PassKey<PeerConnectionTrackerTest> key)
      : PeerConnectionTracker(std::move(host), main_thread_task_runner) {}
  PeerConnectionTracker(
      mojo::PendingRemote<mojom::blink::PeerConnectionTrackerHost> host,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner,
      base::PassKey<MockPeerConnectionTracker> key)
      : PeerConnectionTracker(std::move(host), main_thread_task_runner) {}

  static void BindToFrame(
      LocalFrame* frame,
      mojo::PendingReceiver<mojom::blink::PeerConnectionManager> receiver);

  enum Source { kSourceLocal, kSourceRemote };

  enum Action {
    kActionSetLocalDescription,
    kActionSetLocalDescriptionImplicit,
    kActionSetRemoteDescription,
    kActionCreateOffer,
    kActionCreateAnswer
  };

  enum class TransceiverUpdatedReason {
    kAddTransceiver,
    kAddTrack,
    kRemoveTrack,
    kSetLocalDescription,
    kSetRemoteDescription,
  };

  // The following methods send an update to the browser process when a
  // PeerConnection update happens. The caller should call the Track* methods
  // after calling RegisterPeerConnection and before calling
  // UnregisterPeerConnection, otherwise the Track* call has no effect.

  // Sends an update when a PeerConnection has been created in Javascript. This
  // should be called once and only once for each PeerConnection. The
  // `pc_handler` is the handler object associated with the PeerConnection,
  // the `config` is used to initialize the PeerConnection and the `frame` is
  // the WebLocalFrame object representing the page in which the PeerConnection
  // is created.
  void RegisterPeerConnection(
      RTCPeerConnectionHandler* pc_handler,
      const webrtc::PeerConnectionInterface::RTCConfiguration& config,
      const blink::WebLocalFrame* frame);

  // Sends an update when a PeerConnection has been destroyed.
  virtual void UnregisterPeerConnection(RTCPeerConnectionHandler* pc_handler);

  // Sends an update when createOffer/createAnswer has been called.
  // The |pc_handler| is the handler object associated with the PeerConnection,
  // the |constraints| is the media constraints used to create the offer/answer.
  virtual void TrackCreateOffer(RTCPeerConnectionHandler* pc_handler,
                                RTCOfferOptionsPlatform* options);
  virtual void TrackCreateAnswer(RTCPeerConnectionHandler* pc_handler,
                                 blink::RTCAnswerOptionsPlatform* options);

  // Sends an update when setLocalDescription or setRemoteDescription is called.
  virtual void TrackSetSessionDescription(RTCPeerConnectionHandler* pc_handler,
                                          const String& sdp,
                                          const String& type,
                                          Source source);
  virtual void TrackSetSessionDescriptionImplicit(
      RTCPeerConnectionHandler* pc_handler);

  // Sends an update when setConfiguration is called.
  virtual void TrackSetConfiguration(
      RTCPeerConnectionHandler* pc_handler,
      const webrtc::PeerConnectionInterface::RTCConfiguration& config);

  // Sends an update when an Ice candidate is added.
  virtual void TrackAddIceCandidate(RTCPeerConnectionHandler* pc_handler,
                                    RTCIceCandidatePlatform* candidate,
                                    Source source,
                                    bool succeeded);
  // Sends an update when an Ice candidate error is receiver.
  virtual void TrackIceCandidateError(RTCPeerConnectionHandler* pc_handler,
                                      const String& address,
                                      std::optional<uint16_t> port,
                                      const String& host_candidate,
                                      const String& url,
                                      int error_code,
                                      const String& error_text);

  // Sends an update when a transceiver is added, modified or removed. This can
  // happen as a result of any of the methods indicated by |reason|.
  // Example events: "transceiverAdded", "transceiverModified".
  // See peer_connection_tracker_unittest.cc for expected resulting event
  // strings.
  virtual void TrackAddTransceiver(RTCPeerConnectionHandler* pc_handler,
                                   TransceiverUpdatedReason reason,
                                   const RTCRtpTransceiverPlatform& transceiver,
                                   size_t transceiver_index);
  virtual void TrackModifyTransceiver(
      RTCPeerConnectionHandler* pc_handler,
      TransceiverUpdatedReason reason,
      const RTCRtpTransceiverPlatform& transceiver,
      size_t transceiver_index);

  // Sends an update when a DataChannel is created.
  virtual void TrackCreateDataChannel(
      RTCPeerConnectionHandler* pc_handler,
      const webrtc::DataChannelInterface* data_channel,
      Source source);

  // Sends an update when a PeerConnection has been closed.
  virtual void TrackClose(RTCPeerConnectionHandler* pc_handler);

  // Sends an update when the signaling state of a PeerConnection has changed.
  virtual void TrackSignalingStateChange(
      RTCPeerConnectionHandler* pc_handler,
      webrtc::PeerConnectionInterface::SignalingState state);

  // Sends an update when the ICE connection state of a PeerConnection has
  // changed.
  virtual void TrackIceConnectionStateChange(
      RTCPeerConnectionHandler* pc_handler,
      webrtc::PeerConnectionInterface::IceConnectionState state);

  // Sends an update when the connection state
  // of a PeerConnection has changed.
  virtual void TrackConnectionStateChange(
      RTCPeerConnectionHandler* pc_handler,
      webrtc::PeerConnectionInterface::PeerConnectionState state);

  // Sends an update when the Ice gathering state
  // of a PeerConnection has changed.
  virtual void TrackIceGatheringStateChange(
      RTCPeerConnectionHandler* pc_handler,
      webrtc::PeerConnectionInterface::IceGatheringState state);

  // Sends an update when the SetSessionDescription or CreateOffer or
  // CreateAnswer callbacks are called.
  virtual void TrackSessionDescriptionCallback(
      RTCPeerConnectionHandler* pc_handler,
      Action action,
      const String& type,
      const String& value);

  // Sends an update when the session description's ID is set.
  virtual void TrackSessionId(RTCPeerConnectionHandler* pc_handler,
                              const String& session_id);

  // Sends an update when onRenegotiationNeeded is called.
  virtual void TrackOnRenegotiationNeeded(RTCPeerConnectionHandler* pc_handler);

  // Sends an update when getUserMedia is called.
  virtual void TrackGetUserMedia(UserMediaRequest* user_media_request);
  // Sends an update when getUserMedia resolveѕ with a stream.
  virtual void TrackGetUserMediaSuccess(UserMediaRequest* user_media_request,
                                        const MediaStream* stream);
  // Sends an update when getUserMedia fails with an error.
  virtual void TrackGetUserMediaFailure(UserMediaRequest* user_media_request,
                                        const String& error,
                                        const String& error_message);

  // Sends an update when getDisplayMedia is called.
  virtual void TrackGetDisplayMedia(UserMediaRequest* user_media_request);
  // Sends an update when getDisplayMedia resolveѕ with a stream.
  virtual void TrackGetDisplayMediaSuccess(UserMediaRequest* user_media_request,
                                           MediaStream* stream);
  // Sends an update when getDisplayMedia fails with an error.
  virtual void TrackGetDisplayMediaFailure(UserMediaRequest* user_media_request,
                                           const String& error,
                                           const String& error_message);
  // Sends a new fragment on an RtcEventLog.
  virtual void TrackRtcEventLogWrite(RTCPeerConnectionHandler* pc_handler,
                                     const WTF::Vector<uint8_t>& output);

  void Trace(Visitor* visitor) const override {
    visitor->Trace(peer_connection_tracker_host_);
    visitor->Trace(receiver_);
    Supplement<LocalDOMWindow>::Trace(visitor);
  }

 private:
  FRIEND_TEST_ALL_PREFIXES(PeerConnectionTrackerTest, OnSuspend);
  FRIEND_TEST_ALL_PREFIXES(PeerConnectionTrackerTest, OnThermalStateChange);
  FRIEND_TEST_ALL_PREFIXES(PeerConnectionTrackerTest,
                           ReportInitialThermalState);

  PeerConnectionTracker(
      mojo::PendingRemote<mojom::blink::PeerConnectionTrackerHost> host,
      scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner);

  void Bind(mojo::PendingReceiver<blink::mojom::blink::PeerConnectionManager>
                receiver);

  // Assign a local ID to a peer connection so that the browser process can
  // uniquely identify a peer connection in the renderer process.
  // The return value will always be positive.
  int GetNextLocalID();

  // Looks up a handler in our map and if found, returns its ID. If the handler
  // is not registered, the return value will be -1.
  int GetLocalIDForHandler(RTCPeerConnectionHandler* handler) const;

  void TrackTransceiver(const char* callback_type_ending,
                        RTCPeerConnectionHandler* pc_handler,
                        PeerConnectionTracker::TransceiverUpdatedReason reason,
                        const RTCRtpTransceiverPlatform& transceiver,
                        size_t transceiver_index);

  // PeerConnectionTracker implementation.
  void OnSuspend() override;
  void OnThermalStateChange(
      mojom::blink::DeviceThermalState thermal_state) override;
  void StartEventLog(int peer_connection_local_id,
                     int output_period_ms) override;
  void StopEventLog(int peer_connection_local_id) override;
  void GetStandardStats() override;
  void GetCurrentState() override;

  // Called to deliver an update to the host (PeerConnectionTrackerHost).
  // |local_id| - The id of the registered RTCPeerConnectionHandler.
  //              Using an id instead of the hander pointer is done on purpose
  //              to force doing the lookup before building the callback data
  //              in case the handler isn't registered.
  // |callback_type| - A string, most often static, that represents the type
  //                   of operation that the data stored in |value| comes from.
  //                   E.g. "createOffer", "createAnswer",
  //                   "setRemoteDescription" etc.
  // |value| - A json serialized string containing all the information for the
  //           update event.
  void SendPeerConnectionUpdate(int local_id,
                                const String& callback_type,
                                const String& value);

  void AddStandardStats(int lid, base::Value::List value);

  // This map stores the local ID assigned to each RTCPeerConnectionHandler.
  typedef WTF::HashMap<RTCPeerConnectionHandler*, int> PeerConnectionLocalIdMap;
  PeerConnectionLocalIdMap peer_connection_local_id_map_;
  mojom::blink::DeviceThermalState current_thermal_state_ =
      mojom::blink::DeviceThermalState::kUnknown;
  int32_t current_speed_limit_ = mojom::blink::kSpeedLimitMax;

  THREAD_CHECKER(main_thread_);
  HeapMojoRemote<blink::mojom::blink::PeerConnectionTrackerHost>
      peer_connection_tracker_host_;
  HeapMojoReceiver<blink::mojom::blink::PeerConnectionManager,
                   PeerConnectionTracker>
      receiver_;

  scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_TRACKER_H_
