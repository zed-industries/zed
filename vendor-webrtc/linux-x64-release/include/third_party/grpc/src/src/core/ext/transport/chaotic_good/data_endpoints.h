// Copyright 2024 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_DATA_ENDPOINTS_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_DATA_ENDPOINTS_H

#include <atomic>
#include <chrono>
#include <cstdint>
#include <memory>

#include "src/core/ext/transport/chaotic_good/pending_connection.h"
#include "src/core/ext/transport/chaotic_good/tcp_ztrace_collector.h"
#include "src/core/ext/transport/chaotic_good/transport_context.h"
#include "src/core/lib/promise/party.h"
#include "src/core/lib/promise/promise.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/promise_endpoint.h"
#include "src/core/telemetry/metrics.h"
#include "src/core/util/seq_bit_set.h"

namespace grpc_core {
namespace chaotic_good {

namespace data_endpoints_detail {

class Clock {
 public:
  virtual uint64_t Now() = 0;

 protected:
  ~Clock() = default;
};

class SendRate {
 public:
  explicit SendRate(
      double initial_rate = 0 /* <=0 ==> not set, bytes per nanosecond */)
      : current_rate_(initial_rate) {}
  void StartSend(uint64_t current_time, uint64_t send_size);
  void MaybeCompleteSend(uint64_t current_time);
  void SetCurrentRate(double bytes_per_nanosecond);
  bool IsRateMeasurementStale() const;
  // Returns double nanoseconds from now.
  double DeliveryTime(uint64_t current_time, size_t bytes);

 private:
  uint64_t send_start_time_ = 0;
  uint64_t send_size_ = 0;
  double current_rate_;  // bytes per nanosecond
  Timestamp last_rate_measurement_ = Timestamp::ProcessEpoch();
};

struct NextWrite {
  SliceBuffer bytes;
  bool trace;
};

// Buffered writes for one data endpoint
class OutputBuffer {
 public:
  std::optional<double> DeliveryTime(uint64_t current_time, size_t bytes);
  SliceBuffer& pending() { return pending_; }
  Waker TakeWaker() { return std::move(flush_waker_); }
  void SetWaker() {
    flush_waker_ = GetContext<Activity>()->MakeNonOwningWaker();
  }
  bool HavePending() const { return pending_.Length() > 0; }
  NextWrite TakePendingAndStartWrite(uint64_t current_time);
  void MaybeCompleteSend(uint64_t current_time);
  void UpdateSendRate(double bytes_per_nanosecond) {
    rate_probe_outstanding_ = false;
    send_rate_.SetCurrentRate(bytes_per_nanosecond);
  }

 private:
  Waker flush_waker_;
  SliceBuffer pending_;
  SendRate send_rate_;
  bool rate_probe_outstanding_ = false;
};

// The set of output buffers for all connected data endpoints
class OutputBuffers : public RefCounted<OutputBuffers> {
 public:
  OutputBuffers(Clock* clock,
                std::shared_ptr<TcpZTraceCollector> ztrace_collector)
      : ztrace_collector_(std::move(ztrace_collector)), clock_(clock) {}

  auto Write(uint64_t payload_tag, SliceBuffer output_buffer,
             std::shared_ptr<TcpCallTracer> call_tracer) {
    return [payload_tag, send_time = clock_->Now(),
            output_buffer = std::move(output_buffer),
            call_tracer = std::move(call_tracer), this]() mutable {
      return PollWrite(payload_tag, send_time, output_buffer, call_tracer);
    };
  }

  auto Next(uint32_t connection_id) {
    return [this, connection_id]() { return PollNext(connection_id); };
  }

  void AddEndpoint(uint32_t connection_id);

  uint32_t ReadyEndpoints() const {
    return ready_endpoints_.load(std::memory_order_relaxed);
  }

  void UpdateSendRate(uint32_t connection_id, double bytes_per_nanosecond);

 private:
  Poll<Empty> PollWrite(uint64_t payload_tag, uint64_t send_time,
                        SliceBuffer& output_buffer,
                        std::shared_ptr<TcpCallTracer>& call_tracer);
  Poll<NextWrite> PollNext(uint32_t connection_id);
  void UpdateMetrics(size_t output_buffer, const TcpConnectionMetrics& metrics);

  const std::shared_ptr<TcpZTraceCollector> ztrace_collector_;
  Mutex mu_;
  std::vector<std::optional<OutputBuffer>> buffers_ ABSL_GUARDED_BY(mu_);
  Waker write_waker_ ABSL_GUARDED_BY(mu_);
  std::atomic<uint32_t> ready_endpoints_{0};
  Clock* const clock_;
};

class InputQueue : public RefCounted<InputQueue> {
 public:
  // One outstanding read.
  // ReadTickets get filed by read requests, and all tickets are fullfilled
  // by an endpoint.
  // A call may Await a ticket to get the bytes back later (or it may skip that
  // step - in which case the bytes are thrown away after reading).
  // This decoupling is necessary to ensure that cancelled reads by calls do not
  // cause data corruption for other calls.
  class ReadTicket {
   public:
    ReadTicket(ValueOrFailure<uint64_t> payload_tag,
               RefCountedPtr<InputQueue> input_queues)
        : payload_tag_(payload_tag), input_queues_(std::move(input_queues)) {}

    ReadTicket(const ReadTicket&) = delete;
    ReadTicket& operator=(const ReadTicket&) = delete;
    ReadTicket(ReadTicket&& other) noexcept
        : payload_tag_(std::exchange(other.payload_tag_, 0)),
          input_queues_(std::move(other.input_queues_)) {}
    ReadTicket& operator=(ReadTicket&& other) noexcept {
      payload_tag_ = std::exchange(other.payload_tag_, 0);
      input_queues_ = std::move(other.input_queues_);
      return *this;
    }

    ~ReadTicket() {
      if (input_queues_ != nullptr && payload_tag_.ok() && *payload_tag_ != 0) {
        input_queues_->Cancel(*payload_tag_);
      }
    }

    auto Await() {
      return If(
          payload_tag_.ok(),
          [this]() {
            return [input_queues = input_queues_,
                    payload_tag = std::exchange(*payload_tag_, 0)]() {
              return input_queues->PollRead(payload_tag);
            };
          },
          []() {
            return []() -> absl::StatusOr<SliceBuffer> {
              return absl::InternalError("Duplicate read of tagged payload");
            };
          });
    }

   private:
    ValueOrFailure<uint64_t> payload_tag_;
    RefCountedPtr<InputQueue> input_queues_;
  };

  InputQueue() {
    read_requested_.Set(0);
    read_completed_.Set(0);
  }

  ReadTicket Read(uint64_t payload_tag);
  void CompleteRead(uint64_t payload_tag, absl::StatusOr<SliceBuffer> buffer);
  void Cancel(uint64_t payload_tag);

 private:
  struct Cancelled {};

  Poll<absl::StatusOr<SliceBuffer>> PollRead(uint64_t payload_tag);

  Mutex mu_;
  SeqBitSet read_requested_ ABSL_GUARDED_BY(mu_);
  SeqBitSet read_completed_ ABSL_GUARDED_BY(mu_);
  absl::flat_hash_map<uint64_t, Waker> read_wakers_ ABSL_GUARDED_BY(mu_);
  absl::flat_hash_map<uint64_t, absl::StatusOr<SliceBuffer>> read_buffers_
      ABSL_GUARDED_BY(mu_);
};

class Endpoint final {
 public:
  Endpoint(uint32_t id, RefCountedPtr<OutputBuffers> output_buffers,
           RefCountedPtr<InputQueue> input_queues,
           PendingConnection pending_connection, bool enable_tracing,
           TransportContextPtr ctx,
           std::shared_ptr<TcpZTraceCollector> ztrace_collector);

 private:
  static auto WriteLoop(uint32_t id,
                        RefCountedPtr<OutputBuffers> output_buffers,
                        std::shared_ptr<PromiseEndpoint> endpoint,
                        std::shared_ptr<TcpZTraceCollector> ztrace_collector);
  static auto ReadLoop(uint32_t id, RefCountedPtr<InputQueue> input_queues,
                       std::shared_ptr<PromiseEndpoint> endpoint,
                       std::shared_ptr<TcpZTraceCollector> ztrace_collector);

  RefCountedPtr<Party> party_;
};

}  // namespace data_endpoints_detail

// Collection of data connections.
class DataEndpoints {
 public:
  using ReadTicket = data_endpoints_detail::InputQueue::ReadTicket;

  explicit DataEndpoints(std::vector<PendingConnection> endpoints,
                         TransportContextPtr ctx,
                         std::shared_ptr<TcpZTraceCollector> ztrace_collector,
                         bool enable_tracing,
                         data_endpoints_detail::Clock* clock = DefaultClock());

  // Try to queue output_buffer against a data endpoint.
  // Returns a promise that resolves to the data endpoint connection id
  // selected.
  // Connection ids returned by this class are 0 based (which is different
  // to how chaotic good communicates them on the wire - those are 1 based
  // to allow for the control channel identification)
  auto Write(uint64_t tag, SliceBuffer output_buffer,
             std::shared_ptr<TcpCallTracer> call_tracer) {
    return output_buffers_->Write(tag, std::move(output_buffer),
                                  std::move(call_tracer));
  }

  ReadTicket Read(uint64_t tag) { return input_queues_->Read(tag); }

  bool empty() const { return output_buffers_->ReadyEndpoints() == 0; }

 private:
  static data_endpoints_detail::Clock* DefaultClock() {
    class ClockImpl final : public data_endpoints_detail::Clock {
     public:
      uint64_t Now() override {
        return std::chrono::steady_clock::now().time_since_epoch().count();
      }
    };
    static ClockImpl clock;
    return &clock;
  }

  RefCountedPtr<data_endpoints_detail::OutputBuffers> output_buffers_;
  RefCountedPtr<data_endpoints_detail::InputQueue> input_queues_;
  Mutex mu_;
  std::vector<data_endpoints_detail::Endpoint> endpoints_ ABSL_GUARDED_BY(mu_);
};

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_DATA_ENDPOINTS_H
