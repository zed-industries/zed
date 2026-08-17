/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_INCLUDE_RECEIVE_SIDE_CONGESTION_CONTROLLER_H_
#define MODULES_CONGESTION_CONTROLLER_INCLUDE_RECEIVE_SIDE_CONGESTION_CONTROLLER_H_

#include <cstdint>
#include <memory>

#include "api/environment/environment.h"
#include "api/media_types.h"
#include "api/sequence_checker.h"
#include "api/units/data_rate.h"
#include "api/units/time_delta.h"
#include "modules/congestion_controller/remb_throttler.h"
#include "modules/include/module_common_types.h"
#include "modules/remote_bitrate_estimator/congestion_control_feedback_generator.h"
#include "modules/remote_bitrate_estimator/transport_sequence_number_feedback_generator.h"
#include "modules/rtp_rtcp/source/rtp_packet_received.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
class RemoteBitrateEstimator;

// This class represents the congestion control state for receive
// streams. For send side bandwidth estimation, this is simply
// relaying for each received RTP packet back to the sender. While for
// receive side bandwidth estimation, we do the estimation locally and
// send our results back to the sender.
class ReceiveSideCongestionController : public CallStatsObserver {
 public:
  ReceiveSideCongestionController(
      const Environment& env,
      TransportSequenceNumberFeedbackGenenerator::RtcpSender feedback_sender,
      RembThrottler::RembSender remb_sender);

  ~ReceiveSideCongestionController() override = default;

  void EnableSendCongestionControlFeedbackAccordingToRfc8888();

  void OnReceivedPacket(const RtpPacketReceived& packet, MediaType media_type);

  // Implements CallStatsObserver.
  void OnRttUpdate(int64_t avg_rtt_ms, int64_t max_rtt_ms) override;

  // This is send bitrate, used to control the rate of feedback messages.
  void OnBitrateChanged(int bitrate_bps);

  // Ensures the remote party is notified of the receive bitrate no larger than
  // `bitrate` using RTCP REMB.
  void SetMaxDesiredReceiveBitrate(DataRate bitrate);

  // Returns latest receive side bandwidth estimation.
  // Returns zero if receive side bandwidth estimation is unavailable.
  DataRate LatestReceiveSideEstimate() const;

  // Removes stream from receive side bandwidth estimation.
  // Noop if receive side bwe is not used or stream doesn't participate in it.
  void RemoveStream(uint32_t ssrc);

  // Runs periodic tasks if it is time to run them, returns time until next
  // call to `MaybeProcess` should be non idle.
  TimeDelta MaybeProcess();

 private:
  void PickEstimator(bool has_absolute_send_time)
      RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  const Environment env_;
  RembThrottler remb_throttler_;

  // TODO: bugs.webrtc.org/42224904 - Use sequence checker for all usage of
  // ReceiveSideCongestionController. At the time of
  // writing OnReceivedPacket and MaybeProcess can unfortunately be called on an
  // arbitrary thread by external projects.
  SequenceChecker sequence_checker_;

  bool send_rfc8888_congestion_feedback_ = false;
  TransportSequenceNumberFeedbackGenenerator
      transport_sequence_number_feedback_generator_;
  CongestionControlFeedbackGenerator congestion_control_feedback_generator_
      RTC_GUARDED_BY(sequence_checker_);

  mutable Mutex mutex_;
  std::unique_ptr<RemoteBitrateEstimator> rbe_ RTC_GUARDED_BY(mutex_);
  bool using_absolute_send_time_ RTC_GUARDED_BY(mutex_);
  uint32_t packets_since_absolute_send_time_ RTC_GUARDED_BY(mutex_);
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_INCLUDE_RECEIVE_SIDE_CONGESTION_CONTROLLER_H_
