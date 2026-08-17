/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_FUZZERS_DCSCTP_FUZZERS_H_
#define NET_DCSCTP_FUZZERS_DCSCTP_FUZZERS_H_

#include <deque>
#include <memory>
#include <set>
#include <vector>

#include "api/array_view.h"
#include "api/task_queue/task_queue_base.h"
#include "net/dcsctp/public/dcsctp_socket.h"

namespace dcsctp {
namespace dcsctp_fuzzers {

// A fake timeout used during fuzzing.
class FuzzerTimeout : public Timeout {
 public:
  explicit FuzzerTimeout(std::set<TimeoutID>& active_timeouts)
      : active_timeouts_(active_timeouts) {}

  void Start(DurationMs /* duration_ms */, TimeoutID timeout_id) override {
    // Start is only allowed to be called on stopped or expired timeouts.
    if (timeout_id_.has_value()) {
      // It has been started before, but maybe it expired. Ensure that it's not
      // running at least.
      RTC_DCHECK(active_timeouts_.find(*timeout_id_) == active_timeouts_.end());
    }
    timeout_id_ = timeout_id;
    RTC_DCHECK(active_timeouts_.insert(timeout_id).second);
  }

  void Stop() override {
    // Stop is only allowed to be called on active timeouts. Not stopped or
    // expired.
    RTC_DCHECK(timeout_id_.has_value());
    RTC_DCHECK(active_timeouts_.erase(*timeout_id_) == 1);
    timeout_id_ = std::nullopt;
  }

  // A set of all active timeouts, managed by `FuzzerCallbacks`.
  std::set<TimeoutID>& active_timeouts_;
  // If present, the timout is active and will expire reported as `timeout_id`.
  std::optional<TimeoutID> timeout_id_;
};

class FuzzerCallbacks : public DcSctpSocketCallbacks {
 public:
  static constexpr int kRandomValue = 42;
  void SendPacket(webrtc::ArrayView<const uint8_t> data) override {
    sent_packets_.emplace_back(std::vector<uint8_t>(data.begin(), data.end()));
  }
  std::unique_ptr<Timeout> CreateTimeout(
      webrtc::TaskQueueBase::DelayPrecision /* precision */) override {
    // The fuzzer timeouts don't implement |precision|.
    return std::make_unique<FuzzerTimeout>(active_timeouts_);
  }
  webrtc::Timestamp Now() override { return webrtc::Timestamp::Millis(42); }
  uint32_t GetRandomInt(uint32_t /* low */, uint32_t /* high */) override {
    return kRandomValue;
  }
  void OnMessageReceived(DcSctpMessage /* message */) override {}
  void OnError(ErrorKind /* error */,
               absl::string_view /* message */) override {}
  void OnAborted(ErrorKind /* error */,
                 absl::string_view /* message */) override {}
  void OnConnected() override {}
  void OnClosed() override {}
  void OnConnectionRestarted() override {}
  void OnStreamsResetFailed(
      webrtc::ArrayView<const StreamID> /* outgoing_streams */,
      absl::string_view /* reason */) override {}
  void OnStreamsResetPerformed(
      webrtc::ArrayView<const StreamID> outgoing_streams) override {}
  void OnIncomingStreamsReset(
      webrtc::ArrayView<const StreamID> incoming_streams) override {}

  std::vector<uint8_t> ConsumeSentPacket() {
    if (sent_packets_.empty()) {
      return {};
    }
    std::vector<uint8_t> ret = sent_packets_.front();
    sent_packets_.pop_front();
    return ret;
  }

  // Given an index among the active timeouts, will expire that one.
  std::optional<TimeoutID> ExpireTimeout(size_t index) {
    if (index < active_timeouts_.size()) {
      auto it = active_timeouts_.begin();
      std::advance(it, index);
      TimeoutID timeout_id = *it;
      active_timeouts_.erase(it);
      return timeout_id;
    }
    return std::nullopt;
  }

 private:
  // Needs to be ordered, to allow fuzzers to expire timers.
  std::set<TimeoutID> active_timeouts_;
  std::deque<std::vector<uint8_t>> sent_packets_;
};

// Given some fuzzing `data` will send packets to the socket as well as calling
// API methods.
void FuzzSocket(DcSctpSocketInterface& socket,
                FuzzerCallbacks& cb,
                webrtc::ArrayView<const uint8_t> data);

}  // namespace dcsctp_fuzzers
}  // namespace dcsctp
#endif  // NET_DCSCTP_FUZZERS_DCSCTP_FUZZERS_H_
