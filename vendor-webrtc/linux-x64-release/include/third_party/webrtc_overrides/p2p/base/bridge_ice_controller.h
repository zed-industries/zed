// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_CONTROLLER_H_
#define THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_CONTROLLER_H_

#include <cstdint>
#include <memory>
#include <optional>

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/webrtc/api/rtc_error.h"
#include "third_party/webrtc/p2p/base/active_ice_controller_interface.h"
#include "third_party/webrtc/p2p/base/connection.h"
#include "third_party/webrtc/p2p/base/ice_agent_interface.h"
#include "third_party/webrtc/p2p/base/ice_controller_interface.h"
#include "third_party/webrtc/p2p/base/ice_switch_reason.h"
#include "third_party/webrtc/p2p/base/ice_transport_internal.h"
#include "third_party/webrtc/p2p/base/transport_description.h"
#include "third_party/webrtc/rtc_base/system/rtc_export.h"
#include "third_party/webrtc_overrides/p2p/base/ice_connection.h"
#include "third_party/webrtc_overrides/p2p/base/ice_controller_observer.h"
#include "third_party/webrtc_overrides/p2p/base/ice_interaction_interface.h"
#include "third_party/webrtc_overrides/p2p/base/ice_ping_proposal.h"
#include "third_party/webrtc_overrides/p2p/base/ice_prune_proposal.h"
#include "third_party/webrtc_overrides/p2p/base/ice_switch_proposal.h"

namespace blink {

// BridgeIceController allows criculating ICE controller requests through blink
// before taking the necessary action. This enables blink to consult with the
// application before manipulating the ICE transport.
//
// BridgeIceController is constructed and owned for the entirety of its lifetime
// by the native ICE transport (i.e. P2PTransportChannel). It must be called on
// the same sequence (or thread) on which the ICE agent expects to be invoked.
class RTC_EXPORT BridgeIceController
    : public webrtc::ActiveIceControllerInterface {
 public:
  // Constructs an ICE controller wrapping an already constructed native webrtc
  // ICE controller. Does not take ownership of the ICE agent, which must
  // already exist and outlive the ICE controller. Task runner should be the
  // sequence on which the ICE agent expects to be invoked.
  BridgeIceController(
      scoped_refptr<base::SequencedTaskRunner> network_task_runner,
      IceControllerObserverInterface* observer,
      webrtc::IceAgentInterface* ice_agent,
      std::unique_ptr<webrtc::IceControllerInterface> native_controller);
  BridgeIceController(
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      webrtc::IceAgentInterface* ice_agent,
      std::unique_ptr<webrtc::IceControllerInterface> native_controller);
  ~BridgeIceController() override;

  void AttachObserver(IceControllerObserverInterface* observer);

  // ActiveIceControllerInterface overrides.

  void SetIceConfig(const webrtc::IceConfig& config) override;
  bool GetUseCandidateAttribute(const webrtc::Connection* connection,
                                webrtc::NominationMode mode,
                                webrtc::IceMode remote_ice_mode) const override;

  void OnConnectionAdded(const webrtc::Connection* connection) override;
  void OnConnectionPinged(const webrtc::Connection* connection) override;
  void OnConnectionUpdated(const webrtc::Connection* connection) override;
  void OnConnectionSwitched(const webrtc::Connection* connection) override;
  void OnConnectionDestroyed(const webrtc::Connection* connection) override;

  void OnSortAndSwitchRequest(webrtc::IceSwitchReason reason) override;
  void OnImmediateSortAndSwitchRequest(webrtc::IceSwitchReason reason) override;
  bool OnImmediateSwitchRequest(webrtc::IceSwitchReason reason,
                                const webrtc::Connection* selected) override;

  // Only for unit tests
  const webrtc::Connection* FindNextPingableConnection() override;

 private:
  void MaybeStartPinging();
  void SelectAndPingConnection();

  void DoPerformPing(const webrtc::IceControllerInterface::PingResult result);
  void DoPerformPing(const webrtc::Connection* connection,
                     std::optional<int> recheck_delay_ms);
  void DoSchedulePingRecheck(std::optional<int> recheck_delay_ms);

  void SortAndSwitchToBestConnection(webrtc::IceSwitchReason reason);

  void DoSortAndSwitchToBestConnection(webrtc::IceSwitchReason reason);
  void DoPerformSwitch(
      webrtc::IceSwitchReason reason_for_switch,
      const webrtc::IceControllerInterface::SwitchResult result);
  void DoPerformSwitch(webrtc::IceSwitchReason reason_for_switch,
                       const webrtc::Connection* connection,
                       std::optional<webrtc::IceRecheckEvent> recheck_event,
                       base::span<const webrtc::Connection* const>
                           connections_to_forget_state_on);
  void DoScheduleSwitchRecheck(
      std::optional<webrtc::IceRecheckEvent> recheck_event);

  void UpdateStateOnConnectionsResorted();
  void UpdateStateOnPrune();

  void PruneConnections();

  // Callbacks from ICE interaction proxy.

  void OnPingProposalAccepted(const IcePingProposal& proposal);
  void OnPingProposalRejected(const IcePingProposal& proposal);

  void OnSwitchProposalAccepted(const IceSwitchProposal& proposal);
  void OnSwitchProposalRejected(const IceSwitchProposal& proposal);

  void OnPruneProposalAccepted(const IcePruneProposal& proposal);
  void OnPruneProposalRejected(const IcePruneProposal& proposal);

  webrtc::RTCError OnPingRequested(const IceConnection& connection);
  webrtc::RTCError OnSwitchRequested(const IceConnection& connection);
  webrtc::RTCError OnPruneRequested(
      base::span<const IceConnection> connections_to_prune);

  // Receives ICE interaction requests and delegates them to the ICE controller
  // to act on as appropriate.
  class IceInteractionProxy : public IceInteractionInterface {
   public:
    IceInteractionProxy(BridgeIceController* controller,
                        scoped_refptr<base::SequencedTaskRunner> task_runner);
    ~IceInteractionProxy() override = default;

    void AcceptPingProposal(const IcePingProposal& proposal) override;
    void RejectPingProposal(const IcePingProposal& proposal) override;

    void AcceptSwitchProposal(const IceSwitchProposal& proposal) override;
    void RejectSwitchProposal(const IceSwitchProposal& proposal) override;

    void AcceptPruneProposal(const IcePruneProposal& proposal) override;
    void RejectPruneProposal(const IcePruneProposal& proposal) override;

    webrtc::RTCError PingIceConnection(
        const IceConnection& connection) override;
    webrtc::RTCError SwitchToIceConnection(
        const IceConnection& connection) override;
    webrtc::RTCError PruneIceConnections(
        base::span<const IceConnection> connections_to_prune) override;

   private:
    BridgeIceController* controller_;
    scoped_refptr<base::SequencedTaskRunner> task_runner_;
  };

  void DoPerformPrune(base::span<const webrtc::Connection* const> connections);

  const webrtc::Connection* FindConnection(uint32_t id) const;

  const scoped_refptr<base::SequencedTaskRunner> network_task_runner_;

  bool started_pinging_ = false;
  bool sort_pending_ = false;
  const webrtc::Connection* selected_connection_ = nullptr;

  scoped_refptr<IceInteractionProxy> interaction_proxy_;
  const std::unique_ptr<webrtc::IceControllerInterface> native_controller_;
  webrtc::IceAgentInterface& agent_;
  IceControllerObserverInterface* observer_ = nullptr;

  base::WeakPtrFactory<BridgeIceController> weak_factory_;
};

}  // namespace blink

#endif  // THIRD_PARTY_WEBRTC_OVERRIDES_P2P_BASE_BRIDGE_ICE_CONTROLLER_H_
