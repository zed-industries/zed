/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_UNITTEST_HELPER_H_
#define MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_UNITTEST_HELPER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <vector>

#include "api/transport/field_trial_based_config.h"
#include "api/transport/network_types.h"
#include "api/units/timestamp.h"
#include "modules/congestion_controller/goog_cc/acknowledged_bitrate_estimator_interface.h"
#include "modules/congestion_controller/goog_cc/delay_based_bwe.h"
#include "modules/congestion_controller/goog_cc/probe_bitrate_estimator.h"
#include "system_wrappers/include/clock.h"
#include "test/field_trial.h"
#include "test/gtest.h"

namespace webrtc {
namespace test {

class TestBitrateObserver {
 public:
  TestBitrateObserver() : updated_(false), latest_bitrate_(0) {}
  ~TestBitrateObserver() {}

  void OnReceiveBitrateChanged(uint32_t bitrate);

  void Reset() { updated_ = false; }

  bool updated() const { return updated_; }

  uint32_t latest_bitrate() const { return latest_bitrate_; }

 private:
  bool updated_;
  uint32_t latest_bitrate_;
};

class RtpStream {
 public:
  enum { kSendSideOffsetUs = 1000000 };

  RtpStream(int fps, int bitrate_bps);

  RtpStream(const RtpStream&) = delete;
  RtpStream& operator=(const RtpStream&) = delete;

  // Generates a new frame for this stream. If called too soon after the
  // previous frame, no frame will be generated. The frame is split into
  // packets.
  int64_t GenerateFrame(int64_t time_now_us,
                        int64_t* next_sequence_number,
                        std::vector<PacketResult>* packets);

  // The send-side time when the next frame can be generated.
  int64_t next_rtp_time() const;

  void set_bitrate_bps(int bitrate_bps);

  int bitrate_bps() const;

  static bool Compare(const std::unique_ptr<RtpStream>& lhs,
                      const std::unique_ptr<RtpStream>& rhs);

 private:
  int fps_;
  int bitrate_bps_;
  int64_t next_rtp_time_;
};

class StreamGenerator {
 public:
  StreamGenerator(int capacity, int64_t time_now);
  ~StreamGenerator();

  StreamGenerator(const StreamGenerator&) = delete;
  StreamGenerator& operator=(const StreamGenerator&) = delete;

  // Add a new stream.
  void AddStream(RtpStream* stream);

  // Set the link capacity.
  void set_capacity_bps(int capacity_bps);

  // Divides `bitrate_bps` among all streams. The allocated bitrate per stream
  // is decided by the initial allocation ratios.
  void SetBitrateBps(int bitrate_bps);

  // Set the RTP timestamp offset for the stream identified by `ssrc`.
  void set_rtp_timestamp_offset(uint32_t ssrc, uint32_t offset);

  // TODO(holmer): Break out the channel simulation part from this class to make
  // it possible to simulate different types of channels.
  int64_t GenerateFrame(int64_t time_now_us,
                        int64_t* next_sequence_number,
                        std::vector<PacketResult>* packets);

 private:
  // Capacity of the simulated channel in bits per second.
  int capacity_;
  // The time when the last packet arrived.
  int64_t prev_arrival_time_us_;
  // All streams being transmitted on this simulated channel.
  std::vector<std::unique_ptr<RtpStream>> streams_;
};
}  // namespace test

class DelayBasedBweTest : public ::testing::Test {
 public:
  DelayBasedBweTest();
  ~DelayBasedBweTest() override;

 protected:
  void AddDefaultStream();

  // Helpers to insert a single packet into the delay-based BWE.
  void IncomingFeedback(int64_t arrival_time_ms,
                        int64_t send_time_ms,
                        size_t payload_size);
  void IncomingFeedback(int64_t arrival_time_ms,
                        int64_t send_time_ms,
                        size_t payload_size,
                        const PacedPacketInfo& pacing_info);
  void IncomingFeedback(Timestamp receive_time,
                        Timestamp send_time,
                        size_t payload_size,
                        const PacedPacketInfo& pacing_info);

  // Generates a frame of packets belonging to a stream at a given bitrate and
  // with a given ssrc. The stream is pushed through a very simple simulated
  // network, and is then given to the receive-side bandwidth estimator.
  // Returns true if an over-use was seen, false otherwise.
  // The StreamGenerator::updated() should be used to check for any changes in
  // target bitrate after the call to this function.
  bool GenerateAndProcessFrame(uint32_t ssrc, uint32_t bitrate_bps);

  // Run the bandwidth estimator with a stream of `number_of_frames` frames, or
  // until it reaches `target_bitrate`.
  // Can for instance be used to run the estimator for some time to get it
  // into a steady state.
  uint32_t SteadyStateRun(uint32_t ssrc,
                          int number_of_frames,
                          uint32_t start_bitrate,
                          uint32_t min_bitrate,
                          uint32_t max_bitrate,
                          uint32_t target_bitrate);

  void TestTimestampGroupingTestHelper();

  void TestWrappingHelper(int silence_time_s);

  void InitialBehaviorTestHelper(uint32_t expected_converge_bitrate);
  void RateIncreaseReorderingTestHelper(uint32_t expected_bitrate);
  void RateIncreaseRtpTimestampsTestHelper(int expected_iterations);
  void CapacityDropTestHelper(int number_of_streams,
                              bool wrap_time_stamp,
                              uint32_t expected_bitrate_drop_delta,
                              int64_t receiver_clock_offset_change_ms);

  static const uint32_t kDefaultSsrc;
  FieldTrialBasedConfig field_trial_config_;

  std::unique_ptr<test::ScopedFieldTrials>
      field_trial;        // Must be initialized first.
  SimulatedClock clock_;  // Time at the receiver.
  test::TestBitrateObserver bitrate_observer_;
  std::unique_ptr<AcknowledgedBitrateEstimatorInterface>
      acknowledged_bitrate_estimator_;
  const std::unique_ptr<ProbeBitrateEstimator> probe_bitrate_estimator_;
  std::unique_ptr<DelayBasedBwe> bitrate_estimator_;
  std::unique_ptr<test::StreamGenerator> stream_generator_;
  int64_t arrival_time_offset_ms_;
  int64_t next_sequence_number_;
  bool first_update_;
};

}  // namespace webrtc

#endif  // MODULES_CONGESTION_CONTROLLER_GOOG_CC_DELAY_BASED_BWE_UNITTEST_HELPER_H_
