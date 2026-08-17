/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_SINGLE_STREAM_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_SINGLE_STREAM_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <optional>
#include <vector>

#include "absl/base/nullability.h"
#include "api/environment/environment.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/remote_bitrate_estimator/aimd_rate_control.h"
#include "modules/remote_bitrate_estimator/include/remote_bitrate_estimator.h"
#include "modules/remote_bitrate_estimator/inter_arrival.h"
#include "modules/remote_bitrate_estimator/overuse_detector.h"
#include "modules/remote_bitrate_estimator/overuse_estimator.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/bitrate_tracker.h"

namespace webrtc {

class RemoteBitrateEstimatorSingleStream : public RemoteBitrateEstimator {
 public:
  RemoteBitrateEstimatorSingleStream(const Environment& env,
                                     RemoteBitrateObserver* absl_nonnull
                                         observer);

  RemoteBitrateEstimatorSingleStream() = delete;
  RemoteBitrateEstimatorSingleStream(
      const RemoteBitrateEstimatorSingleStream&) = delete;
  RemoteBitrateEstimatorSingleStream& operator=(
      const RemoteBitrateEstimatorSingleStream&) = delete;

  ~RemoteBitrateEstimatorSingleStream() override;

  void IncomingPacket(const RtpPacketReceived& rtp_packet) override;
  TimeDelta Process() override;
  void OnRttUpdate(int64_t avg_rtt_ms, int64_t max_rtt_ms) override;
  void RemoveStream(uint32_t ssrc) override;
  DataRate LatestEstimate() const override;

 private:
  struct Detector {
    Detector();

    Timestamp last_packet_time;
    InterArrival inter_arrival;
    OveruseEstimator estimator;
    OveruseDetector detector;
  };

  // Triggers a new estimate calculation.
  void UpdateEstimate(Timestamp now);

  std::vector<uint32_t> GetSsrcs() const;

  const Environment env_;
  RemoteBitrateObserver* absl_nonnull const observer_;
  std::map<uint32_t, Detector> overuse_detectors_;
  BitrateTracker incoming_bitrate_;
  DataRate last_valid_incoming_bitrate_;
  AimdRateControl remote_rate_;
  std::optional<Timestamp> last_process_time_;
  TimeDelta process_interval_;
  bool uma_recorded_;
};

}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_SINGLE_STREAM_H_
