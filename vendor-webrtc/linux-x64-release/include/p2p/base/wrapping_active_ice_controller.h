/*
 *  Copyright 2022 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_WRAPPING_ACTIVE_ICE_CONTROLLER_H_
#define P2P_BASE_WRAPPING_ACTIVE_ICE_CONTROLLER_H_

#include <memory>

#include "api/task_queue/pending_task_safety_flag.h"
#include "p2p/base/active_ice_controller_interface.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_agent_interface.h"
#include "p2p/base/ice_controller_factory_interface.h"
#include "p2p/base/ice_controller_interface.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// WrappingActiveIceController provides the functionality of a legacy passive
// ICE controller but packaged as an active ICE Controller.
class WrappingActiveIceController : public ActiveIceControllerInterface {
 public:
  // Constructs an active ICE controller wrapping an already constructed legacy
  // ICE controller. Does not take ownership of the ICE agent, which must
  // already exist and outlive the ICE controller.
  WrappingActiveIceController(IceAgentInterface* ice_agent,
                              std::unique_ptr<IceControllerInterface> wrapped);
  // Constructs an active ICE controller that wraps over a legacy ICE
  // controller. The legacy ICE controller is constructed through a factory, if
  // one is supplied. If not, a default BasicIceController is wrapped instead.
  // Does not take ownership of the ICE agent, which must already exist and
  // outlive the ICE controller.
  WrappingActiveIceController(
      IceAgentInterface* ice_agent,
      IceControllerFactoryInterface* wrapped_factory,
      const IceControllerFactoryArgs& wrapped_factory_args);
  virtual ~WrappingActiveIceController();

  void SetIceConfig(const IceConfig& config) override;
  bool GetUseCandidateAttribute(const Connection* connection,
                                NominationMode mode,
                                IceMode remote_ice_mode) const override;

  void OnConnectionAdded(const Connection* connection) override;
  void OnConnectionPinged(const Connection* connection) override;
  void OnConnectionUpdated(const Connection* connection) override;
  void OnConnectionSwitched(const Connection* connection) override;
  void OnConnectionDestroyed(const Connection* connection) override;

  void OnSortAndSwitchRequest(IceSwitchReason reason) override;
  void OnImmediateSortAndSwitchRequest(IceSwitchReason reason) override;
  bool OnImmediateSwitchRequest(IceSwitchReason reason,
                                const Connection* selected) override;

  // Only for unit tests
  const Connection* FindNextPingableConnection() override;

 private:
  void MaybeStartPinging();
  void SelectAndPingConnection();
  void HandlePingResult(IceControllerInterface::PingResult result);

  void SortAndSwitchToBestConnection(IceSwitchReason reason);
  void HandleSwitchResult(IceSwitchReason reason_for_switch,
                          IceControllerInterface::SwitchResult result);
  void UpdateStateOnConnectionsResorted();

  void PruneConnections();

  Thread* const network_thread_;
  ScopedTaskSafety task_safety_;

  bool started_pinging_ RTC_GUARDED_BY(network_thread_) = false;
  bool sort_pending_ RTC_GUARDED_BY(network_thread_) = false;
  const Connection* selected_connection_ RTC_GUARDED_BY(network_thread_) =
      nullptr;

  std::unique_ptr<IceControllerInterface> wrapped_
      RTC_GUARDED_BY(network_thread_);
  IceAgentInterface& agent_ RTC_GUARDED_BY(network_thread_);
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::WrappingActiveIceController;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_WRAPPING_ACTIVE_ICE_CONTROLLER_H_
