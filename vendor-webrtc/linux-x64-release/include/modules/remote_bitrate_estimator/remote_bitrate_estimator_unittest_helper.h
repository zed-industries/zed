/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_UNITTEST_HELPER_H_
#define MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_UNITTEST_HELPER_H_

#include <cstddef>
#include <cstdint>
#include <list>
#include <map>
#include <memory>
#include <utility>
#include <vector>

#include "modules/remote_bitrate_estimator/include/remote_bitrate_estimator.h"
#include "system_wrappers/include/clock.h"
#include "test/gtest.h"

namespace webrtc {
namespace testing {

class TestBitrateObserver : public RemoteBitrateObserver {
 public:
  TestBitrateObserver() : updated_(false), latest_bitrate_(0) {}
  virtual ~TestBitrateObserver() {}

  void OnReceiveBitrateChanged(const std::vector<uint32_t>& ssrcs,
                               uint32_t bitrate) override;

  void Reset() { updated_ = false; }

  bool updated() const { return updated_; }

  uint32_t latest_bitrate() const { return latest_bitrate_; }

 private:
  bool updated_;
  uint32_t latest_bitrate_;
};

class RtpStream {
 public:
  struct RtpPacket {
    int64_t send_time;
    int64_t arrival_time;
    uint32_t rtp_timestamp;
    size_t size;
    uint32_t ssrc;
  };

  struct RtcpPacket {
    uint32_t ntp_secs;
    uint32_t ntp_frac;
    uint32_t timestamp;
    uint32_t ssrc;
  };

  typedef std::list<RtpPacket*> PacketList;

  enum { kSendSideOffsetUs = 1000000 };

  RtpStream(int fps,
            int bitrate_bps,
            uint32_t ssrc,
            uint32_t frequency,
            uint32_t timestamp_offset,
            int64_t rtcp_receive_time);

  RtpStream(const RtpStream&) = delete;
  RtpStream& operator=(const RtpStream&) = delete;

  void set_rtp_timestamp_offset(uint32_t offset);

  // Generates a new frame for this stream. If called too soon after the
  // previous frame, no frame will be generated. The frame is split into
  // packets.
  int64_t GenerateFrame(int64_t time_now_us, PacketList* packets);

  // The send-side time when the next frame can be generated.
  int64_t next_rtp_time() const;

  // Generates an RTCP packet.
  RtcpPacket* Rtcp(int64_t time_now_us);

  void set_bitrate_bps(int bitrate_bps);

  int bitrate_bps() const;

  uint32_t ssrc() const;

  static bool Compare(const std::pair<uint32_t, RtpStream*>& left,
                      const std::pair<uint32_t, RtpStream*>& right);

 private:
  enum { kRtcpIntervalUs = 1000000 };

  int fps_;
  int bitrate_bps_;
  uint32_t ssrc_;
  uint32_t frequency_;
  int64_t next_rtp_time_;
  int64_t next_rtcp_time_;
  uint32_t rtp_timestamp_offset_;
  const double kNtpFracPerMs;
};

class StreamGenerator {
 public:
  typedef std::list<RtpStream::RtcpPacket*> RtcpList;

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
  int64_t GenerateFrame(RtpStream::PacketList* packets, int64_t time_now_us);

 private:
  typedef std::map<uint32_t, RtpStream*> StreamMap;

  // Capacity of the simulated channel in bits per second.
  int capacity_;
  // The time when the last packet arrived.
  int64_t prev_arrival_time_us_;
  // All streams being transmitted on this simulated channel.
  StreamMap streams_;
};
}  // namespace testing

class RemoteBitrateEstimatorTest : public ::testing::Test {
 public:
  RemoteBitrateEstimatorTest();
  virtual ~RemoteBitrateEstimatorTest();

  RemoteBitrateEstimatorTest(const RemoteBitrateEstimatorTest&) = delete;
  RemoteBitrateEstimatorTest& operator=(const RemoteBitrateEstimatorTest&) =
      delete;

 protected:
  virtual void SetUp() = 0;

  void AddDefaultStream();

  // Helper to convert some time format to resolution used in absolute send time
  // header extension, rounded upwards. `t` is the time to convert, in some
  // resolution. `denom` is the value to divide `t` by to get whole seconds,
  // e.g. `denom` = 1000 if `t` is in milliseconds.
  static uint32_t AbsSendTime(int64_t t, int64_t denom);

  // Helper to add two absolute send time values and keep it less than 1<<24.
  static uint32_t AddAbsSendTime(uint32_t t1, uint32_t t2);

  // Helper to create an RTPHeader containing the relevant data for the
  // estimator (all other fields are cleared) and call IncomingPacket on the
  // estimator.
  void IncomingPacket(uint32_t ssrc,
                      size_t payload_size,
                      int64_t arrival_time,
                      uint32_t rtp_timestamp,
                      uint32_t absolute_send_time);

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

  SimulatedClock clock_;  // Time at the receiver.
  std::unique_ptr<testing::TestBitrateObserver> bitrate_observer_;
  std::unique_ptr<RemoteBitrateEstimator> bitrate_estimator_;
  std::unique_ptr<testing::StreamGenerator> stream_generator_;
  int64_t arrival_time_offset_ms_;
};
}  // namespace webrtc

#endif  // MODULES_REMOTE_BITRATE_ESTIMATOR_REMOTE_BITRATE_ESTIMATOR_UNITTEST_HELPER_H_
