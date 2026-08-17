/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_CONTROLLER_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_CONTROLLER_H_

#include <stdint.h>

#include <optional>
#include <vector>

#include "absl/base/attributes.h"
#include "api/field_trials_view.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/transport/network_types.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/experiments/field_trial_parser.h"

namespace webrtc {

struct ProbeControllerConfig {
  explicit ProbeControllerConfig(const FieldTrialsView* key_value_config);
  ProbeControllerConfig(const ProbeControllerConfig&);
  ProbeControllerConfig& operator=(const ProbeControllerConfig&) = default;
  ~ProbeControllerConfig();

  // These parameters configure the initial probes. First we send one or two
  // probes of sizes p1 * start_bitrate_ and p2 * start_bitrate_.
  // Then whenever we get a bitrate estimate of at least further_probe_threshold
  // times the size of the last sent probe we'll send another one of size
  // step_size times the new estimate.
  FieldTrialParameter<double> first_exponential_probe_scale;
  FieldTrialOptional<double> second_exponential_probe_scale;
  FieldTrialParameter<double> further_exponential_probe_scale;
  FieldTrialParameter<double> further_probe_threshold;
  FieldTrialParameter<bool> abort_further_probe_if_max_lower_than_current;
  // Duration of time from the first initial probe where repeated initial probes
  // are sent if repeated initial probing is enabled.
  FieldTrialParameter<TimeDelta> repeated_initial_probing_time_period;
  // The minimum probing duration of an individual probe during
  // the repeated_initial_probing_time_period.
  FieldTrialParameter<TimeDelta> initial_probe_duration;
  // Delta time between sent bursts of packets in a probe during
  // the repeated_initial_probing_time_period.
  FieldTrialParameter<TimeDelta> initial_min_probe_delta;
  // Configures how often we send ALR probes and how big they are.
  FieldTrialParameter<TimeDelta> alr_probing_interval;
  FieldTrialParameter<double> alr_probe_scale;
  // Configures how often we send probes if NetworkStateEstimate is available.
  FieldTrialParameter<TimeDelta> network_state_estimate_probing_interval;
  // Periodically probe as long as the ratio between current estimate and
  // NetworkStateEstimate is lower then this.
  FieldTrialParameter<double>
      probe_if_estimate_lower_than_network_state_estimate_ratio;
  FieldTrialParameter<TimeDelta>
      estimate_lower_than_network_state_estimate_probing_interval;
  FieldTrialParameter<double> network_state_probe_scale;
  // Overrides min_probe_duration if network_state_estimate_probing_interval
  // is set and a network state estimate is known and equal or higher than the
  // probe target.
  FieldTrialParameter<TimeDelta> network_state_probe_duration;
  // Overrides min_probe_delta if network_state_estimate_probing_interval
  // is set and a network state estimate is known and equal or higher than the
  // probe target.
  FieldTrialParameter<TimeDelta> network_state_min_probe_delta;

  // Configures the probes emitted by changed to the allocated bitrate.
  FieldTrialParameter<bool> probe_on_max_allocated_bitrate_change;
  FieldTrialOptional<double> first_allocation_probe_scale;
  FieldTrialOptional<double> second_allocation_probe_scale;
  FieldTrialParameter<double> allocation_probe_limit_by_current_scale;

  // The minimum number probing packets used.
  FieldTrialParameter<int> min_probe_packets_sent;
  // The minimum probing duration.
  FieldTrialParameter<TimeDelta> min_probe_duration;
  // Delta time between sent bursts of packets in a probe.
  FieldTrialParameter<TimeDelta> min_probe_delta;
  FieldTrialParameter<double> loss_limited_probe_scale;
  // Don't send a probe if min(estimate, network state estimate) is larger than
  // this fraction of the set max or max allocated bitrate.
  FieldTrialParameter<double> skip_if_estimate_larger_than_fraction_of_max;
  // Scale factor of the max allocated bitrate. Used when deciding if a probe
  // can be skiped due to that the estimate is already high enough.
  FieldTrialParameter<double> skip_probe_max_allocated_scale;
};

// Reason that bandwidth estimate is limited. Bandwidth estimate can be limited
// by either delay based bwe, or loss based bwe when it increases/decreases the
// estimate.
enum class BandwidthLimitedCause : int {
  kLossLimitedBweIncreasing = 0,
  kLossLimitedBwe = 1,
  kDelayBasedLimited = 2,
  kDelayBasedLimitedDelayIncreased = 3,
  kRttBasedBackOffHighRtt = 4
};

// This class controls initiation of probing to estimate initial channel
// capacity. There is also support for probing during a session when max
// bitrate is adjusted by an application.
class ProbeController {
 public:
  explicit ProbeController(const FieldTrialsView* key_value_config,
                           RtcEventLog* event_log);
  ~ProbeController();

  ProbeController(const ProbeController&) = delete;
  ProbeController& operator=(const ProbeController&) = delete;

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> SetBitrates(
      DataRate min_bitrate,
      DataRate start_bitrate,
      DataRate max_bitrate,
      Timestamp at_time);

  // The total bitrate, as opposed to the max bitrate, is the sum of the
  // configured bitrates for all active streams.
  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig>
  OnMaxTotalAllocatedBitrate(DataRate max_total_allocated_bitrate,
                             Timestamp at_time);

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> OnNetworkAvailability(
      NetworkAvailability msg);

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> SetEstimatedBitrate(
      DataRate bitrate,
      BandwidthLimitedCause bandwidth_limited_cause,
      Timestamp at_time);

  void EnablePeriodicAlrProbing(bool enable);

  // Probes are sent periodically every 1s during the first 5s after the network
  // becomes available or until OnMaxTotalAllocatedBitrate is invoked with a
  // none zero max_total_allocated_bitrate (there are active streams being
  // sent.) Probe rate is up to max configured bitrate configured via
  // SetBitrates.
  void EnableRepeatedInitialProbing(bool enable);

  void SetAlrStartTimeMs(std::optional<int64_t> alr_start_time);
  void SetAlrEndedTimeMs(int64_t alr_end_time);

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> RequestProbe(
      Timestamp at_time);

  void SetNetworkStateEstimate(webrtc::NetworkStateEstimate estimate);

  // Resets the ProbeController to a state equivalent to as if it was just
  // created EXCEPT for configuration settings like
  // `enable_periodic_alr_probing_` `network_available_` and
  // `max_total_allocated_bitrate_`.
  void Reset(Timestamp at_time);

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> Process(
      Timestamp at_time);

 private:
  enum class State {
    // Initial state where no probing has been triggered yet.
    kInit,
    // Waiting for probing results to continue further probing.
    kWaitingForProbingResult,
    // Probing is complete.
    kProbingComplete,
  };

  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig>
  InitiateExponentialProbing(Timestamp at_time);
  ABSL_MUST_USE_RESULT std::vector<ProbeClusterConfig> InitiateProbing(
      Timestamp now,
      std::vector<DataRate> bitrates_to_probe,
      bool probe_further);
  bool TimeForAlrProbe(Timestamp at_time) const;
  bool TimeForNetworkStateProbe(Timestamp at_time) const;
  bool TimeForNextRepeatedInitialProbe(Timestamp at_time) const;
  ProbeClusterConfig CreateProbeClusterConfig(Timestamp at_time,
                                              DataRate bitrate);

  bool network_available_;
  bool repeated_initial_probing_enabled_ = false;
  Timestamp last_allowed_repeated_initial_probe_ = Timestamp::MinusInfinity();
  BandwidthLimitedCause bandwidth_limited_cause_ =
      BandwidthLimitedCause::kDelayBasedLimited;
  State state_;
  DataRate min_bitrate_to_probe_further_ = DataRate::PlusInfinity();
  Timestamp time_last_probing_initiated_ = Timestamp::MinusInfinity();
  DataRate estimated_bitrate_ = DataRate::Zero();
  std::optional<webrtc::NetworkStateEstimate> network_estimate_;
  DataRate start_bitrate_ = DataRate::Zero();
  DataRate max_bitrate_ = DataRate::PlusInfinity();
  Timestamp last_bwe_drop_probing_time_ = Timestamp::Zero();
  std::optional<Timestamp> alr_start_time_;
  std::optional<Timestamp> alr_end_time_;
  bool enable_periodic_alr_probing_;
  Timestamp time_of_last_large_drop_ = Timestamp::MinusInfinity();
  DataRate bitrate_before_last_large_drop_ = DataRate::Zero();
  DataRate max_total_allocated_bitrate_ = DataRate::Zero();

  const bool in_rapid_recovery_experiment_;
  RtcEventLog* event_log_;

  int32_t next_probe_cluster_id_ = 1;

  ProbeControllerConfig config_;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_PROBE_CONTROLLER_H_
