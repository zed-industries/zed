/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_NETWORK_CROSS_TRAFFIC_H_
#define TEST_NETWORK_CROSS_TRAFFIC_H_

#include <cmath>
#include <cstddef>
#include <deque>
#include <list>
#include <map>
#include <set>

#include "absl/functional/any_invocable.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/network_emulation/cross_traffic.h"
#include "api/test/network_emulation/network_emulation_interfaces.h"
#include "api/test/network_emulation_manager.h"
#include "api/units/data_rate.h"
#include "api/units/data_size.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "rtc_base/random.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"
#include "test/network/network_emulation.h"
#include "test/scenario/column_printer.h"

namespace webrtc {
namespace test {

class RandomWalkCrossTraffic final : public CrossTrafficGenerator {
 public:
  RandomWalkCrossTraffic(RandomWalkConfig config,
                         CrossTrafficRoute* traffic_route);
  ~RandomWalkCrossTraffic();

  void Process(Timestamp at_time) override;
  TimeDelta GetProcessInterval() const override;
  DataRate TrafficRate() const;
  ColumnPrinter StatsPrinter();

 private:
  SequenceChecker sequence_checker_;
  const RandomWalkConfig config_;
  CrossTrafficRoute* const traffic_route_ RTC_PT_GUARDED_BY(sequence_checker_);
  webrtc::Random random_ RTC_GUARDED_BY(sequence_checker_);

  Timestamp last_process_time_ RTC_GUARDED_BY(sequence_checker_) =
      Timestamp::MinusInfinity();
  Timestamp last_update_time_ RTC_GUARDED_BY(sequence_checker_) =
      Timestamp::MinusInfinity();
  Timestamp last_send_time_ RTC_GUARDED_BY(sequence_checker_) =
      Timestamp::MinusInfinity();
  double intensity_ RTC_GUARDED_BY(sequence_checker_) = 0;
  DataSize pending_size_ RTC_GUARDED_BY(sequence_checker_) = DataSize::Zero();
};

class PulsedPeaksCrossTraffic final : public CrossTrafficGenerator {
 public:
  PulsedPeaksCrossTraffic(PulsedPeaksConfig config,
                          CrossTrafficRoute* traffic_route);
  ~PulsedPeaksCrossTraffic();

  void Process(Timestamp at_time) override;
  TimeDelta GetProcessInterval() const override;
  DataRate TrafficRate() const;
  ColumnPrinter StatsPrinter();

 private:
  SequenceChecker sequence_checker_;
  const PulsedPeaksConfig config_;
  CrossTrafficRoute* const traffic_route_ RTC_PT_GUARDED_BY(sequence_checker_);

  Timestamp last_update_time_ RTC_GUARDED_BY(sequence_checker_) =
      Timestamp::MinusInfinity();
  Timestamp last_send_time_ RTC_GUARDED_BY(sequence_checker_) =
      Timestamp::MinusInfinity();
  bool sending_ RTC_GUARDED_BY(sequence_checker_) = false;
};

class TcpMessageRouteImpl final : public TcpMessageRoute {
 public:
  TcpMessageRouteImpl(Clock* clock,
                      TaskQueueBase* task_queue,
                      EmulatedRoute* send_route,
                      EmulatedRoute* ret_route);

  // Sends a TCP message of the given `size` over the route, `on_received` is
  // called when the message has been delivered. Note that the connection
  // parameters are reset iff there's no currently pending message on the route.
  void SendMessage(size_t size,
                   absl::AnyInvocable<void()> on_received) override;

 private:
  // Represents a message sent over the route. When all fragments has been
  // delivered, the message is considered delivered and the handler is
  // triggered. This only happen once.
  struct Message {
    absl::AnyInvocable<void()> handler;
    std::set<int> pending_fragment_ids;
  };
  // Represents a piece of a message that fit into a TCP packet.
  struct MessageFragment {
    int fragment_id;
    size_t size;
  };
  // Represents a packet sent on the wire.
  struct TcpPacket {
    int sequence_number;
    Timestamp send_time = Timestamp::MinusInfinity();
    MessageFragment fragment;
  };

  void OnRequest(TcpPacket packet_info);
  void OnResponse(TcpPacket packet_info, Timestamp at_time);
  void HandleLoss(Timestamp at_time);
  void SendPackets(Timestamp at_time);
  void HandlePacketTimeout(int seq_num, Timestamp at_time);

  Clock* const clock_;
  TaskQueueBase* const task_queue_;
  FakePacketRoute<TcpPacket> request_route_;
  FakePacketRoute<TcpPacket> response_route_;

  std::deque<MessageFragment> pending_;
  std::map<int, TcpPacket> in_flight_;
  std::list<Message> messages_;

  double cwnd_;
  double ssthresh_;

  int last_acked_seq_num_ = 0;
  int next_sequence_number_ = 0;
  int next_fragment_id_ = 0;
  Timestamp last_reduction_time_ = Timestamp::MinusInfinity();
  TimeDelta last_rtt_ = TimeDelta::Zero();
};

class FakeTcpCrossTraffic
    : public TwoWayFakeTrafficRoute<int, int>::TrafficHandlerInterface,
      public CrossTrafficGenerator {
 public:
  FakeTcpCrossTraffic(FakeTcpConfig config,
                      EmulatedRoute* send_route,
                      EmulatedRoute* ret_route);

  TimeDelta GetProcessInterval() const override;
  void Process(Timestamp at_time) override;

  void OnRequest(int sequence_number, Timestamp at_time) override;
  void OnResponse(int sequence_number, Timestamp at_time) override;

  void HandleLoss(Timestamp at_time);

  void SendPackets(Timestamp at_time);

 private:
  const FakeTcpConfig conf_;
  TwoWayFakeTrafficRoute<int, int> route_;

  std::map<int, Timestamp> in_flight_;
  double cwnd_ = 10;
  double ssthresh_ = INFINITY;
  bool ack_received_ = false;
  int last_acked_seq_num_ = 0;
  int next_sequence_number_ = 0;
  Timestamp last_reduction_time_ = Timestamp::MinusInfinity();
  TimeDelta last_rtt_ = TimeDelta::Zero();
  DataSize total_sent_ = DataSize::Zero();
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_NETWORK_CROSS_TRAFFIC_H_
