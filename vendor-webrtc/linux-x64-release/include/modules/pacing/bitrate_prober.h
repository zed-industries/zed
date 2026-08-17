/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PACING_BITRATE_PROBER_H_
#define MODULES_PACING_BITRATE_PROBER_H_

#include <stddef.h>

#include <optional>
#include <queue>

#include "api/field_trials_view.h"
#include "api/transport/network_types.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/experiments/field_trial_parser.h"

namespace webrtc {
class RtcEventLog;

struct BitrateProberConfig {
  explicit BitrateProberConfig(const FieldTrialsView* key_value_config);
  BitrateProberConfig(const BitrateProberConfig&) = default;
  BitrateProberConfig& operator=(const BitrateProberConfig&) = default;
  ~BitrateProberConfig() = default;

  // Maximum amount of time each probe can be delayed.
  FieldTrialParameter<TimeDelta> max_probe_delay;
  // This is used to start sending a probe after a large enough packet.
  // The min packet size is scaled with the bitrate we're probing at.
  // This defines the max min packet size, meaning that on high bitrates
  // a packet of at least this size is needed to trigger sending a probe.
  FieldTrialParameter<DataSize> min_packet_size;

  // If true, `min_packet_size` is ignored.
  bool allow_start_probing_immediately = false;
};

// Note that this class isn't thread-safe by itself and therefore relies
// on being protected by the caller.
class BitrateProber {
 public:
  explicit BitrateProber(const FieldTrialsView& field_trials);
  ~BitrateProber() = default;

  void SetEnabled(bool enable);
  void SetAllowProbeWithoutMediaPacket(bool allow);

  // Returns true if the prober is in a probing session, i.e., it currently
  // wants packets to be sent out according to the time returned by
  // TimeUntilNextProbe().
  bool is_probing() const { return probing_state_ == ProbingState::kActive; }

  // Initializes a new probing session if the prober is allowed to probe. Does
  // not initialize the prober unless the packet size is large enough to probe
  // with.
  void OnIncomingPacket(DataSize packet_size);

  // Create a cluster used to probe.
  void CreateProbeCluster(const ProbeClusterConfig& cluster_config);
  // Returns the time at which the next probe should be sent to get accurate
  // probing. If probing is not desired at this time, Timestamp::PlusInfinity()
  // will be returned.
  // TODO(bugs.webrtc.org/11780): Remove `now` argument when old mode is gone.
  Timestamp NextProbeTime(Timestamp now) const;

  // Information about the current probing cluster.
  std::optional<PacedPacketInfo> CurrentCluster(Timestamp now);

  // Returns the minimum number of bytes that the prober recommends for
  // the next probe, or zero if not probing. A probe can consist of multiple
  // packets that are sent back to back.
  DataSize RecommendedMinProbeSize() const;

  // Called to report to the prober that a probe has been sent. In case of
  // multiple packets per probe, this call would be made at the end of sending
  // the last packet in probe. `size` is the total size of all packets in probe.
  void ProbeSent(Timestamp now, DataSize size);

 private:
  enum class ProbingState {
    // Probing will not be triggered in this state at all times.
    kDisabled,
    // Probing is enabled and ready to trigger on the first packet arrival if
    // there is a probe cluster.
    kInactive,
    // Probe cluster is filled with the set of data rates to be probed and
    // probes are being sent.
    kActive,
  };

  // A probe cluster consists of a set of probes. Each probe in turn can be
  // divided into a number of packets to accommodate the MTU on the network.
  struct ProbeCluster {
    PacedPacketInfo pace_info;

    int sent_probes = 0;
    int sent_bytes = 0;
    TimeDelta min_probe_delta = TimeDelta::Zero();
    Timestamp requested_at = Timestamp::MinusInfinity();
    Timestamp started_at = Timestamp::MinusInfinity();
  };

  Timestamp CalculateNextProbeTime(const ProbeCluster& cluster) const;

  void MaybeSetActiveState(DataSize packet_size);
  bool ReadyToSetActiveState(DataSize packet_size) const;

  ProbingState probing_state_;

  // Probe bitrate per packet. These are used to compute the delta relative to
  // the previous probe packet based on the size and time when that packet was
  // sent.
  std::queue<ProbeCluster> clusters_;

  // Time the next probe should be sent when in kActive state.
  Timestamp next_probe_time_;

  BitrateProberConfig config_;
};

}  // namespace webrtc

#endif  // MODULES_PACING_BITRATE_PROBER_H_
