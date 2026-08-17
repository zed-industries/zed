/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_ICE_CONTROLLER_INTERFACE_H_
#define P2P_BASE_ICE_CONTROLLER_INTERFACE_H_

#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "api/array_view.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/p2p_transport_channel_ice_field_trials.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Forward declaration to avoid circular dependency.

struct RTC_EXPORT IceRecheckEvent {
  IceRecheckEvent(IceSwitchReason _reason, int _recheck_delay_ms)
      : reason(_reason), recheck_delay_ms(_recheck_delay_ms) {}

  std::string ToString() const;

  IceSwitchReason reason;
  int recheck_delay_ms;
};

// Defines the interface for a module that control
// - which connection to ping
// - which connection to use
// - which connection to prune
// - which connection to forget learned state on
//
// The P2PTransportChannel owns (creates and destroys) Connections,
// but P2PTransportChannel gives const pointers to the the IceController using
// `AddConnection`, i.e the IceController should not call any non-const methods
// on a Connection but signal back in the interface if any mutable function
// shall be called.
//
// Current these are limited to:
// Connection::Ping               - returned in PingResult
// Connection::Prune              - retuned in PruneConnections
// Connection::ForgetLearnedState - return in SwitchResult
//
// The IceController shall keep track of all connections added
// (and not destroyed) and give them back using the GetConnections() function.
//
// When a Connection gets destroyed
// - signals on Connection::SignalDestroyed
// - P2PTransportChannel calls IceController::OnConnectionDestroyed
class IceControllerInterface {
 public:
  // This represents the result of a switch call.
  struct SwitchResult {
    // Connection that we should (optionally) switch to.
    std::optional<const Connection*> connection;

    // An optional recheck event for when a Switch() should be attempted again.
    std::optional<IceRecheckEvent> recheck_event;

    // A vector with connection to run ForgetLearnedState on.
    std::vector<const Connection*> connections_to_forget_state_on;
  };

  // This represents the result of a call to SelectConnectionToPing.
  struct PingResult {
    PingResult(const Connection* conn, int _recheck_delay_ms)
        : connection(conn ? std::optional<const Connection*>(conn)
                          : std::nullopt),
          recheck_delay_ms(_recheck_delay_ms) {}

    // Connection that we should (optionally) ping.
    const std::optional<const Connection*> connection;

    // The delay before P2PTransportChannel shall call SelectConnectionToPing()
    // again.
    //
    // Since the IceController determines which connection to ping and
    // only returns one connection at a time, the recheck_delay_ms does not have
    // any obvious implication on bitrate for pings. E.g the recheck_delay_ms
    // will be shorter if there are more connections available.
    const int recheck_delay_ms = 0;
  };

  virtual ~IceControllerInterface() = default;

  // These setters are called when the state of P2PTransportChannel is mutated.
  virtual void SetIceConfig(const IceConfig& config) = 0;
  virtual void SetSelectedConnection(const Connection* selected_connection) = 0;
  virtual void AddConnection(const Connection* connection) = 0;
  virtual void OnConnectionDestroyed(const Connection* connection) = 0;

  // These are all connections that has been added and not destroyed.
  virtual ArrayView<const Connection* const> GetConnections() const {
    // Stub implementation to simplify downstream roll.
    RTC_CHECK_NOTREACHED();
    return {};
  }
  // TODO(bugs.webrtc.org/15702): Remove this after downstream is cleaned up.
  virtual ArrayView<const Connection*> connections() const {
    // Stub implementation to simplify downstream removal.
    RTC_CHECK_NOTREACHED();
    return {};
  }

  // Is there a pingable connection ?
  // This function is used to boot-strap pinging, after this returns true
  // SelectConnectionToPing() will be called periodically.
  virtual bool HasPingableConnection() const = 0;

  // Select a connection to Ping, or nullptr if none.
  virtual PingResult SelectConnectionToPing(int64_t last_ping_sent_ms) = 0;

  // Compute the "STUN_ATTR_USE_CANDIDATE" for `conn`.
  virtual bool GetUseCandidateAttr(const Connection* conn,
                                   NominationMode mode,
                                   IceMode remote_ice_mode) const = 0;

  // These methods is only added to not have to change all unit tests
  // that simulate pinging by marking a connection pinged.
  virtual const Connection* FindNextPingableConnection() = 0;
  virtual void MarkConnectionPinged(const Connection* con) = 0;

  // Check if we should switch to `connection`.
  // This method is called for IceSwitchReasons that can switch directly
  // i.e without resorting.
  virtual SwitchResult ShouldSwitchConnection(IceSwitchReason reason,
                                              const Connection* connection) = 0;

  // Sort connections and check if we should switch.
  virtual SwitchResult SortAndSwitchConnection(IceSwitchReason reason) = 0;

  // Prune connections.
  virtual std::vector<const Connection*> PruneConnections() = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::IceControllerInterface;
using ::webrtc::IceRecheckEvent;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_ICE_CONTROLLER_INTERFACE_H_
