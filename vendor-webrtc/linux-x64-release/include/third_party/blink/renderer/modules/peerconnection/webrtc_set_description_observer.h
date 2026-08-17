// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_SET_DESCRIPTION_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_SET_DESCRIPTION_OBSERVER_H_

#include <memory>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_peer_connection_handler.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_receiver_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sender_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_transceiver_impl.h"
#include "third_party/blink/renderer/modules/peerconnection/transceiver_state_surfacer.h"
#include "third_party/blink/renderer/modules/peerconnection/webrtc_media_stream_track_adapter_map.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/api/jsep.h"
#include "third_party/webrtc/api/peer_connection_interface.h"
#include "third_party/webrtc/api/rtc_error.h"
#include "third_party/webrtc/api/rtp_receiver_interface.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/set_remote_description_observer_interface.h"
#include "third_party/webrtc/rtc_base/ref_count.h"
#include "third_party/webrtc/rtc_base/ref_counted_object.h"

namespace blink {

// Copies the session description.
// Note: At the time of writing, third_party/webrtc/pc/sdp_utils.h's
// webrtc::CloneSessionDescription() creates a copy that does not include
// candidates added with AddIceCandidate. This is why we need our own copy
// function, which copies everything.
std::unique_ptr<webrtc::SessionDescriptionInterface> CopySessionDescription(
    const webrtc::SessionDescriptionInterface* description);

// The blink layer correspondent of the setLocalDescription() observer
// (webrtc::SetSessionDescriptionObserver) and setRemoteDescription() observer
// (webrtc::SetRemoteDescriptionObserverInterface). The implementation should
// process the state changes of the Set[Local/Remote]Description() by inspecting
// the updated States.
class MODULES_EXPORT WebRtcSetDescriptionObserver
    : public WTF::ThreadSafeRefCounted<WebRtcSetDescriptionObserver> {
 public:
  // The states as they were when the operation finished on the webrtc signaling
  // thread. Note that other operations may have occurred while jumping back to
  // the main thread, but these must be handled separately.
  struct MODULES_EXPORT States {
    States();
    States(States&& other);

    States(const States&) = delete;
    States& operator=(const States&) = delete;

    ~States();

    States& operator=(States&& other);

    webrtc::PeerConnectionInterface::SignalingState signaling_state;
    blink::WebRTCSctpTransportSnapshot sctp_transport_state;
    std::vector<blink::RtpTransceiverState> transceiver_states
        ALLOW_DISCOURAGED_TYPE(
            "Avoids conversions when passed from/to webrtc API");
    std::unique_ptr<webrtc::SessionDescriptionInterface>
        pending_local_description;
    std::unique_ptr<webrtc::SessionDescriptionInterface>
        current_local_description;
    std::unique_ptr<webrtc::SessionDescriptionInterface>
        pending_remote_description;
    std::unique_ptr<webrtc::SessionDescriptionInterface>
        current_remote_description;
  };

  WebRtcSetDescriptionObserver();

  WebRtcSetDescriptionObserver(const WebRtcSetDescriptionObserver&) = delete;
  WebRtcSetDescriptionObserver& operator=(const WebRtcSetDescriptionObserver&) =
      delete;

  // Invoked in a PostTask() on the main thread after the SetLocalDescription()
  // or SetRemoteDescription() operation completed on the webrtc signaling
  // thread.
  virtual void OnSetDescriptionComplete(webrtc::RTCError error,
                                        States states) = 0;

 protected:
  friend class WTF::ThreadSafeRefCounted<WebRtcSetDescriptionObserver>;
  virtual ~WebRtcSetDescriptionObserver();
};

// Takes care of surfacing WebRtcSetDescriptionObserver::State information from
// the webrtc signaling thread to the main thread. With the state information
// obtained, invokes |observer_|'s
// WebRtcSetDescriptionObserver::OnSetDescriptionComplete() on the main thread.
//
// This implements the behavior
// of both WebRtcSetLocalDescriptionObserverHandler and
// WebRtcSetRemoteDescriptionObserverHandler, but these are put in different
// classes because local and remote description observers have different
// interfaces in webrtc.
class MODULES_EXPORT WebRtcSetDescriptionObserverHandlerImpl
    : public WTF::ThreadSafeRefCounted<
          WebRtcSetDescriptionObserverHandlerImpl> {
 public:
  WebRtcSetDescriptionObserverHandlerImpl(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      scoped_refptr<WebRtcSetDescriptionObserver> observer);

  WebRtcSetDescriptionObserverHandlerImpl(
      const WebRtcSetDescriptionObserverHandlerImpl&) = delete;
  WebRtcSetDescriptionObserverHandlerImpl& operator=(
      const WebRtcSetDescriptionObserverHandlerImpl&) = delete;

  // Must be called on the webrtc signaling thread internally by the handler
  // when the Set[Local/Remote]Description() operation finishes.
  void OnSetDescriptionComplete(webrtc::RTCError error);

 private:
  friend class WTF::ThreadSafeRefCounted<
      WebRtcSetDescriptionObserverHandlerImpl>;
  virtual ~WebRtcSetDescriptionObserverHandlerImpl();

  void OnSetDescriptionCompleteOnMainThread(
      webrtc::RTCError error,
      webrtc::PeerConnectionInterface::SignalingState signaling_state,
      blink::TransceiverStateSurfacer transceiver_state_surfacer,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          pending_local_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          current_local_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          pending_remote_description,
      std::unique_ptr<webrtc::SessionDescriptionInterface>
          current_remote_description);

  scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner_;
  webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc_;
  scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map_;
  scoped_refptr<WebRtcSetDescriptionObserver> observer_;
};

// An implementation of webrtc::SetLocalDescriptionObserverInterface for
// performing the operations of WebRtcSetDescriptionObserverHandlerImpl.
class MODULES_EXPORT WebRtcSetLocalDescriptionObserverHandler
    : public webrtc::SetLocalDescriptionObserverInterface {
 public:
  static scoped_refptr<WebRtcSetLocalDescriptionObserverHandler> Create(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      scoped_refptr<WebRtcSetDescriptionObserver> observer);

  WebRtcSetLocalDescriptionObserverHandler(
      const WebRtcSetLocalDescriptionObserverHandler&) = delete;
  WebRtcSetLocalDescriptionObserverHandler& operator=(
      const WebRtcSetLocalDescriptionObserverHandler&) = delete;

  // webrtc::SetLocalDescriptionObserverInterface implementation. Implementation
  // calls WebRtcSetDescriptionObserverHandlerImpl::OnSetDescriptionComplete().
  void OnSetLocalDescriptionComplete(webrtc::RTCError error) override;

 protected:
  WebRtcSetLocalDescriptionObserverHandler(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      scoped_refptr<WebRtcSetDescriptionObserver> observer);
  ~WebRtcSetLocalDescriptionObserverHandler() override;

  scoped_refptr<WebRtcSetDescriptionObserverHandlerImpl> handler_impl_;
};

// An implementation of webrtc::SetRemoteDescriptionObserverInterface for
// performing the operations of WebRtcSetDescriptionObserverHandlerImpl.
class MODULES_EXPORT WebRtcSetRemoteDescriptionObserverHandler
    : public webrtc::SetRemoteDescriptionObserverInterface {
 public:
  static scoped_refptr<WebRtcSetRemoteDescriptionObserverHandler> Create(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      scoped_refptr<WebRtcSetDescriptionObserver> observer);

  WebRtcSetRemoteDescriptionObserverHandler(
      const WebRtcSetRemoteDescriptionObserverHandler&) = delete;
  WebRtcSetRemoteDescriptionObserverHandler& operator=(
      const WebRtcSetRemoteDescriptionObserverHandler&) = delete;

  // webrtc::SetRemoteDescriptionObserverInterface implementation.
  // Implementation calls
  // WebRtcSetDescriptionObserverHandlerImpl::OnSetDescriptionComplete().
  void OnSetRemoteDescriptionComplete(webrtc::RTCError error) override;

 protected:
  WebRtcSetRemoteDescriptionObserverHandler(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> signaling_task_runner,
      webrtc::scoped_refptr<webrtc::PeerConnectionInterface> pc,
      scoped_refptr<blink::WebRtcMediaStreamTrackAdapterMap> track_adapter_map,
      scoped_refptr<WebRtcSetDescriptionObserver> observer);
  ~WebRtcSetRemoteDescriptionObserverHandler() override;

  scoped_refptr<WebRtcSetDescriptionObserverHandlerImpl> handler_impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_WEBRTC_SET_DESCRIPTION_OBSERVER_H_
