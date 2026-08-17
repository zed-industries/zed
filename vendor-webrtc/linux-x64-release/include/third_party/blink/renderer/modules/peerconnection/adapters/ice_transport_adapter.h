// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_H_

#include <vector>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/p2p/base/p2p_transport_channel.h"

namespace blink {

// Defines the ICE candidate policy the browser uses to surface the permitted
// candidates to the application.
// https://w3c.github.io/webrtc-pc/#dom-rtcicetransportpolicy
enum class IceTransportPolicy {
  // The ICE Agent uses only media relay candidates.
  kRelay,
  // The ICE Agent can use any type of candidate.
  kAll
};

// The IceTransportAdapter is the API used by the RTCIceTransport Blink binding
// to interact with the ICE implementation. It exactly mirrors the requirements
// of the RTCIceTransport: each JavaScript method that must interact with the
// ICE implementation should map to exactly one method call on this interface.
// This interface is designed to be fully asynchronous; all methods are void and
// callbacks occur via the Delegate (implemented by the client).
//
// The ICE Agent is immediately active once this object has been constructed. It
// can be stopped by deleting the IceTransportAdapter.
class IceTransportAdapter {
  USING_FAST_MALLOC(IceTransportAdapter);

 public:
  // Delegate to receive callbacks from the IceTransportAdapter. The Delegate
  // must outlive the IceTransportAdapter.
  class Delegate {
   public:
    virtual ~Delegate() = default;

    // Called asynchronously when the ICE gathering state changes.
    virtual void OnGatheringStateChanged(webrtc::IceGatheringState new_state) {}

    // Called asynchronously when a new ICE candidate has been gathered.
    virtual void OnCandidateGathered(const webrtc::Candidate& candidate) {}

    // Called asynchronously when the ICE connection state has changed.
    virtual void OnStateChanged(webrtc::IceTransportState new_state) {}

    // Called asynchronously when the ICE agent selects a different candidate
    // pair for the active connection.
    virtual void OnSelectedCandidatePairChanged(
        const std::pair<webrtc::Candidate, webrtc::Candidate>&
            selected_candidate_pair) {}
  };

  virtual ~IceTransportAdapter() = default;

  // Start ICE candidate gathering.
  virtual void StartGathering(
      const webrtc::IceParameters& local_parameters,
      const webrtc::ServerAddresses& stun_servers,
      const std::vector<webrtc::RelayServerConfig>& turn_servers,
      IceTransportPolicy policy) = 0;

  // Start ICE connectivity checks with the given initial remote candidates.
  virtual void Start(
      const webrtc::IceParameters& remote_parameters,
      webrtc::IceRole role,
      const Vector<webrtc::Candidate>& initial_remote_candidates) = 0;

  // Handle a remote ICE restart. This changes the remote parameters and clears
  // all remote candidates.
  virtual void HandleRemoteRestart(
      const webrtc::IceParameters& new_remote_parameters) = 0;

  // Adds a remote candidate to potentially start connectivity checks with.
  // The caller must ensure Start() has already bene called.
  virtual void AddRemoteCandidate(const webrtc::Candidate& candidate) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_ADAPTERS_ICE_TRANSPORT_ADAPTER_H_
