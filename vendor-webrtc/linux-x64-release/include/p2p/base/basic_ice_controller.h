/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_BASIC_ICE_CONTROLLER_H_
#define P2P_BASE_BASIC_ICE_CONTROLLER_H_

#include <algorithm>
#include <cstdint>
#include <functional>
#include <map>
#include <optional>
#include <set>
#include <vector>

#include "api/array_view.h"
#include "p2p/base/connection.h"
#include "p2p/base/ice_controller_factory_interface.h"
#include "p2p/base/ice_controller_interface.h"
#include "p2p/base/ice_switch_reason.h"
#include "p2p/base/ice_transport_internal.h"
#include "p2p/base/p2p_constants.h"
#include "p2p/base/transport_description.h"
#include "rtc_base/network.h"
#include "rtc_base/network_constants.h"

namespace webrtc {

class BasicIceController : public IceControllerInterface {
 public:
  explicit BasicIceController(const IceControllerFactoryArgs& args);
  virtual ~BasicIceController();

  void SetIceConfig(const IceConfig& config) override;
  void SetSelectedConnection(const Connection* selected_connection) override;
  void AddConnection(const Connection* connection) override;
  void OnConnectionDestroyed(const Connection* connection) override;
  ArrayView<const Connection* const> GetConnections() const override {
    return connections_;
  }
  ArrayView<const Connection*> connections() const override {
    return ArrayView<const Connection*>(
        const_cast<const Connection**>(connections_.data()),
        connections_.size());
  }

  bool HasPingableConnection() const override;

  PingResult SelectConnectionToPing(int64_t last_ping_sent_ms) override;

  bool GetUseCandidateAttr(const Connection* conn,
                           NominationMode mode,
                           IceMode remote_ice_mode) const override;

  SwitchResult ShouldSwitchConnection(IceSwitchReason reason,
                                      const Connection* connection) override;
  SwitchResult SortAndSwitchConnection(IceSwitchReason reason) override;

  std::vector<const Connection*> PruneConnections() override;

  // These methods are only for tests.
  const Connection* FindNextPingableConnection() override;
  void MarkConnectionPinged(const Connection* conn) override;

 private:
  // A transport channel is weak if the current best connection is either
  // not receiving or not writable, or if there is no best connection at all.
  bool weak() const {
    return !selected_connection_ || selected_connection_->weak();
  }

  int weak_ping_interval() const {
    return std::max(config_.ice_check_interval_weak_connectivity_or_default(),
                    config_.ice_check_min_interval_or_default());
  }

  int strong_ping_interval() const {
    return std::max(config_.ice_check_interval_strong_connectivity_or_default(),
                    config_.ice_check_min_interval_or_default());
  }

  int check_receiving_interval() const {
    return std::max(webrtc::MIN_CHECK_RECEIVING_INTERVAL,
                    config_.receiving_timeout_or_default() / 10);
  }

  const Connection* FindOldestConnectionNeedingTriggeredCheck(int64_t now);
  // Between `conn1` and `conn2`, this function returns the one which should
  // be pinged first.
  const Connection* MorePingable(const Connection* conn1,
                                 const Connection* conn2);
  // Select the connection which is Relay/Relay. If both of them are,
  // UDP relay protocol takes precedence.
  const Connection* MostLikelyToWork(const Connection* conn1,
                                     const Connection* conn2);
  // Compare the last_ping_sent time and return the one least recently pinged.
  const Connection* LeastRecentlyPinged(const Connection* conn1,
                                        const Connection* conn2);

  bool IsPingable(const Connection* conn, int64_t now) const;
  bool IsBackupConnection(const Connection* conn) const;
  // Whether a writable connection is past its ping interval and needs to be
  // pinged again.
  bool WritableConnectionPastPingInterval(const Connection* conn,
                                          int64_t now) const;
  int CalculateActiveWritablePingInterval(const Connection* conn,
                                          int64_t now) const;

  std::map<const Network*, const Connection*> GetBestConnectionByNetwork()
      const;
  std::vector<const Connection*> GetBestWritableConnectionPerNetwork() const;

  bool ReadyToSend(const Connection* connection) const;
  bool PresumedWritable(const Connection* conn) const;

  int CompareCandidatePairNetworks(
      const Connection* a,
      const Connection* b,
      std::optional<AdapterType> network_preference) const;

  // The methods below return a positive value if `a` is preferable to `b`,
  // a negative value if `b` is preferable, and 0 if they're equally preferable.
  // If `receiving_unchanged_threshold` is set, then when `b` is receiving and
  // `a` is not, returns a negative value only if `b` has been in receiving
  // state and `a` has been in not receiving state since
  // `receiving_unchanged_threshold` and sets
  // `missed_receiving_unchanged_threshold` to true otherwise.
  int CompareConnectionStates(
      const Connection* a,
      const Connection* b,
      std::optional<int64_t> receiving_unchanged_threshold,
      bool* missed_receiving_unchanged_threshold) const;
  int CompareConnectionCandidates(const Connection* a,
                                  const Connection* b) const;
  // Compares two connections based on the connection states
  // (writable/receiving/connected), nomination states, last data received time,
  // and static preferences. Does not include latency. Used by both sorting
  // and ShouldSwitchSelectedConnection().
  // Returns a positive value if `a` is better than `b`.
  int CompareConnections(const Connection* a,
                         const Connection* b,
                         std::optional<int64_t> receiving_unchanged_threshold,
                         bool* missed_receiving_unchanged_threshold) const;

  SwitchResult HandleInitialSelectDampening(IceSwitchReason reason,
                                            const Connection* new_connection);

  std::function<IceTransportStateInternal()> ice_transport_state_func_;
  std::function<IceRole()> ice_role_func_;
  std::function<bool(const Connection*)> is_connection_pruned_func_;

  IceConfig config_;
  const IceFieldTrials* field_trials_;

  // `connections_` is a sorted list with the first one always be the
  // `selected_connection_` when it's not nullptr. The combination of
  // `pinged_connections_` and `unpinged_connections_` has the same
  // connections as `connections_`. These 2 sets maintain whether a
  // connection should be pinged next or not.
  const Connection* selected_connection_ = nullptr;
  std::vector<const Connection*> connections_;
  std::set<const Connection*> pinged_connections_;
  std::set<const Connection*> unpinged_connections_;

  // Timestamp for when we got the first selectable connection.
  int64_t initial_select_timestamp_ms_ = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::BasicIceController;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_BASIC_ICE_CONTROLLER_H_
