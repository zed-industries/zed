/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_ABS_SEND_TIME_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_ABS_SEND_TIME_H_

#include <stddef.h>
#include <stdint.h>

#include <list>
#include <map>
#include <memory>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/remote_bitrate_estimator/aimd_rate_control.h"
#include "modules/remote_bitrate_estimator/include/bwe_defines.h"
#include "modules/remote_bitrate_estimator/include/remote_bitrate_estimator.h"
#include "modules/remote_bitrate_estimator/inter_arrival.h"
#include "modules/remote_bitrate_estimator/overuse_detector.h"
#include "modules/remote_bitrate_estimator/overuse_estimator.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/bitrate_tracker.h"

namespace webrtc {

class RemoteBitrateEstimatorAbsSendTime : public RemoteBitrateEstimator {
 public:
  RemoteBitrateEstimatorAbsSendTime(const Environment& env,
                                    RemoteBitrateObserver* absl_nonnull
                                        observer);

  RemoteBitrateEstimatorAbsSendTime() = delete;
  RemoteBitrateEstimatorAbsSendTime(const RemoteBitrateEstimatorAbsSendTime&) =
      delete;
  RemoteBitrateEstimatorAbsSendTime& operator=(
      const RemoteBitrateEstimatorAbsSendTime&) = delete;

  ~RemoteBitrateEstimatorAbsSendTime() override;

  void IncomingPacket(const RtpPacketReceived& rtp_packet) override;
  TimeDelta Process() override;
  void OnRttUpdate(int64_t avg_rtt_ms, int64_t max_rtt_ms) override;
  void RemoveStream(uint32_t ssrc) override;
  DataRate LatestEstimate() const override;

 private:
  struct Probe {
    Probe(Timestamp send_time, Timestamp recv_time, DataSize payload_size)
        : send_time(send_time),
          recv_time(recv_time),
          payload_size(payload_size) {}

    Timestamp send_time;
    Timestamp recv_time;
    DataSize payload_size;
  };

  struct Cluster {
    DataRate SendBitrate() const { return mean_size / send_mean; }
    DataRate RecvBitrate() const { return mean_size / recv_mean; }

    TimeDelta send_mean = TimeDelta::Zero();
    TimeDelta recv_mean = TimeDelta::Zero();
    // TODO(holmer): Add some variance metric as well?
    DataSize mean_size = DataSize::Zero();
    int count = 0;
    int num_above_min_delta = 0;
  };

  enum class ProbeResult { kBitrateUpdated, kNoUpdate };

  static bool IsWithinClusterBounds(TimeDelta send_delta,
                                    const Cluster& cluster_aggregate);

  static void MaybeAddCluster(const Cluster& cluster_aggregate,
                              std::list<Cluster>& clusters);

  std::list<Cluster> ComputeClusters() const;

  const Cluster* FindBestProbe(const std::list<Cluster>& clusters) const;

  // Returns true if a probe which changed the estimate was detected.
  ProbeResult ProcessClusters(Timestamp now);

  bool IsBitrateImproving(DataRate probe_bitrate) const;

  void TimeoutStreams(Timestamp now);

  const Environment env_;
  RemoteBitrateObserver* absl_nonnull const observer_;
  std::unique_ptr<InterArrival> inter_arrival_;
  std::unique_ptr<OveruseEstimator> estimator_;
  OveruseDetector detector_;
  BitrateTracker incoming_bitrate_{kBitrateWindow};
  bool incoming_bitrate_initialized_ = false;
  std::list<Probe> probes_;
  size_t total_probes_received_ = 0;
  Timestamp first_packet_time_ = Timestamp::MinusInfinity();
  Timestamp last_update_ = Timestamp::MinusInfinity();
  bool uma_recorded_ = false;

  std::map<uint32_t, Timestamp> ssrcs_;
  AimdRateControl remote_rate_;
};

}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_ABS_SEND_TIME_H_
