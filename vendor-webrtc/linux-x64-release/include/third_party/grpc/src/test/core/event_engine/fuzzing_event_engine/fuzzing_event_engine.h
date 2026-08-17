// Copyright 2022 gRPC authors.
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

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_FUZZING_EVENT_ENGINE_FUZZING_EVENT_ENGINE_H
#define GRPC_TEST_CORE_EVENT_ENGINE_FUZZING_EVENT_ENGINE_FUZZING_EVENT_ENGINE_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/support/time.h>
#include <stddef.h>

#include <atomic>
#include <chrono>
#include <cstdint>
#include <map>
#include <memory>
#include <optional>
#include <queue>
#include <set>
#include <thread>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/event_engine/query_extensions.h"
#include "src/core/lib/event_engine/time_util.h"
#include "src/core/lib/experiments/experiments.h"
#include "src/core/util/no_destruct.h"
#include "src/core/util/sync.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"
#include "test/core/test_util/port.h"

namespace grpc_event_engine {
namespace experimental {

// EventEngine implementation to be used by fuzzers.
// It's only allowed to have one FuzzingEventEngine instantiated at a time.
class FuzzingEventEngine : public EventEngine {
 public:
  struct Options {
    Duration max_delay_run_after = std::chrono::seconds(30);
    Duration max_delay_write = std::chrono::seconds(30);
  };
  explicit FuzzingEventEngine(Options options,
                              const fuzzing_event_engine::Actions& actions);
  ~FuzzingEventEngine() override { UnsetGlobalHooks(); }

  using Time = std::chrono::time_point<FuzzingEventEngine, Duration>;

  // Once the fuzzing work is completed, this method should be called to speed
  // quiescence.
  void FuzzingDone() ABSL_LOCKS_EXCLUDED(mu_);
  // Increment time once and perform any scheduled work.
  void Tick(Duration max_time = std::chrono::seconds(600))
      ABSL_LOCKS_EXCLUDED(mu_);
  // Repeatedly call Tick() until there is no more work to do.
  void TickUntilIdle() ABSL_LOCKS_EXCLUDED(mu_);
  // Returns true if idle.
  bool IsIdle() ABSL_LOCKS_EXCLUDED(mu_);
  // Tick until some time
  void TickUntil(Time t) ABSL_LOCKS_EXCLUDED(mu_);
  // Tick for some duration
  void TickForDuration(Duration d) ABSL_LOCKS_EXCLUDED(mu_);

  // Sets a callback to be invoked any time RunAfter() is called.
  // Allows tests to verify the specified duration.
  void SetRunAfterDurationCallback(absl::AnyInvocable<void(Duration)> callback);

  absl::StatusOr<std::unique_ptr<Listener>> CreateListener(
      Listener::AcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const EndpointConfig& config,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory)
      ABSL_LOCKS_EXCLUDED(mu_) override;

  ConnectionHandle Connect(OnConnectCallback on_connect,
                           const ResolvedAddress& addr,
                           const EndpointConfig& args,
                           MemoryAllocator memory_allocator, Duration timeout)
      ABSL_LOCKS_EXCLUDED(mu_) override;

  bool CancelConnect(ConnectionHandle handle) ABSL_LOCKS_EXCLUDED(mu_) override;

  bool IsWorkerThread() override;

  absl::StatusOr<std::unique_ptr<DNSResolver>> GetDNSResolver(
      const DNSResolver::ResolverOptions& options) override;

  void Run(Closure* closure) ABSL_LOCKS_EXCLUDED(mu_) override;
  void Run(absl::AnyInvocable<void()> closure)
      ABSL_LOCKS_EXCLUDED(mu_) override;
  TaskHandle RunAfter(Duration when, Closure* closure)
      ABSL_LOCKS_EXCLUDED(mu_) override;
  TaskHandle RunAfter(Duration when, absl::AnyInvocable<void()> closure)
      ABSL_LOCKS_EXCLUDED(mu_) override;
  bool Cancel(TaskHandle handle) ABSL_LOCKS_EXCLUDED(mu_) override;

  TaskHandle RunAfterExactly(Duration when, absl::AnyInvocable<void()> closure)
      ABSL_LOCKS_EXCLUDED(mu_);

  Time Now() ABSL_LOCKS_EXCLUDED(mu_);

  // Clear any global hooks installed by this event engine. Call prior to
  // destruction to ensure no overlap between tests if constructing/destructing
  // each test.
  void UnsetGlobalHooks() ABSL_LOCKS_EXCLUDED(mu_);

  Duration max_delay_write() const {
    return max_delay_[static_cast<int>(RunType::kWrite)];
  }

  std::pair<std::unique_ptr<Endpoint>, std::unique_ptr<Endpoint>>
  CreateEndpointPair();

 private:
  class IoToken {
   public:
    struct Manifest {
      absl::string_view operation = "NOTHING";
      void* whom = nullptr;
      int part = 0;
      std::atomic<size_t>* refs = nullptr;
    };

    IoToken() : manifest_{} {}
    explicit IoToken(Manifest manifest) : manifest_(manifest) {
      manifest_.refs->fetch_add(1, std::memory_order_relaxed);
      GRPC_TRACE_LOG(fuzzing_ee_writes, INFO)
          << "START_" << manifest_.operation << " " << manifest_.whom << ":"
          << manifest_.part;
    }
    ~IoToken() {
      if (manifest_.refs != nullptr) {
        GRPC_TRACE_LOG(fuzzing_ee_writes, INFO)
            << "STOP_" << manifest_.operation << " " << manifest_.whom << ":"
            << manifest_.part;
        manifest_.refs->fetch_sub(1, std::memory_order_relaxed);
      }
    }
    IoToken(const IoToken&) = delete;
    IoToken& operator=(const IoToken&) = delete;
    IoToken(IoToken&& other) noexcept
        : manifest_(std::exchange(other.manifest_, Manifest{})) {}
    IoToken& operator=(IoToken&& other) noexcept {
      if (manifest_.refs != nullptr) {
        manifest_.refs->fetch_sub(1, std::memory_order_relaxed);
      }
      manifest_ = std::exchange(other.manifest_, Manifest{});
      return *this;
    }
    void Swap(IoToken& other) { std::swap(manifest_, other.manifest_); }

   private:
    Manifest manifest_;
  };

  enum class RunType {
    kWrite,
    kRunAfter,
    kExact,
  };

  // One pending task to be run.
  struct Task {
    Task(intptr_t id, absl::AnyInvocable<void()> closure)
        : id(id), closure(std::move(closure)) {}
    intptr_t id;
    absl::AnyInvocable<void()> closure;
  };

  // Per listener information.
  // We keep a shared_ptr to this, one reference held by the FuzzingListener
  // Listener implementation, and one reference in the event engine state, so it
  // may be iterated through and inspected - principally to discover the ports
  // on which this listener is listening.
  struct ListenerInfo {
    ListenerInfo(
        Listener::AcceptCallback on_accept,
        absl::AnyInvocable<void(absl::Status)> on_shutdown,
        std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory)
        : on_accept(std::move(on_accept)),
          on_shutdown(std::move(on_shutdown)),
          memory_allocator_factory(std::move(memory_allocator_factory)),
          started(false) {}
    ~ListenerInfo() ABSL_LOCKS_EXCLUDED(mu_);
    // The callback to invoke when a new connection is accepted.
    Listener::AcceptCallback on_accept;
    // The callback to invoke when the listener is shut down.
    absl::AnyInvocable<void(absl::Status)> on_shutdown;
    // The memory allocator factory to use for this listener.
    const std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory;
    // The ports on which this listener is listening.
    std::vector<int> ports ABSL_GUARDED_BY(mu_);
    // Has start been called on the listener?
    // Used to emulate the Bind/Start semantics demanded by the API.
    bool started ABSL_GUARDED_BY(mu_);
    // The status to return via on_shutdown.
    absl::Status shutdown_status ABSL_GUARDED_BY(mu_);
  };

  // Implementation of Listener.
  class FuzzingListener final : public Listener {
   public:
    explicit FuzzingListener(std::shared_ptr<ListenerInfo> info)
        : info_(std::move(info)) {}
    ~FuzzingListener() override;
    absl::StatusOr<int> Bind(const ResolvedAddress& addr) override;
    absl::Status Start() override;

   private:
    std::shared_ptr<ListenerInfo> info_;
  };

  // One read that's outstanding.
  struct PendingRead {
    // The associated io token
    IoToken io_token;
    // Callback to invoke when the read completes.
    absl::AnyInvocable<void(absl::Status)> on_read;
    // The buffer to read into.
    SliceBuffer* buffer;
  };

  // The join between two Endpoint instances.
  struct EndpointMiddle {
    EndpointMiddle(int listener_port, int client_port)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
    // Address of each side of the endpoint.
    const ResolvedAddress addrs[2];
    // Is the endpoint closed?
    bool closed[2] ABSL_GUARDED_BY(mu_) = {false, false};
    // Is the endpoint writing?
    bool writing[2] ABSL_GUARDED_BY(mu_) = {false, false};
    // Bytes written into each endpoint and awaiting a read.
    std::vector<uint8_t> pending[2] ABSL_GUARDED_BY(mu_);
    // The sizes of each accepted write, as determined by the fuzzer actions.
    std::queue<size_t> write_sizes[2] ABSL_GUARDED_BY(mu_);
    // The next read that's pending (or nullopt).
    std::optional<PendingRead> pending_read[2] ABSL_GUARDED_BY(mu_);

    // Helper to take some bytes from data and queue them into pending[index].
    // Returns true if all bytes were consumed, false if more writes are needed.
    bool Write(SliceBuffer* data, int index) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
  };

  // Implementation of Endpoint.
  // When a connection is formed, we create two of these - one with index 0, the
  // other index 1, both pointing to the same EndpointMiddle.
  class FuzzingEndpoint final : public Endpoint {
   public:
    FuzzingEndpoint(std::shared_ptr<EndpointMiddle> middle, int index)
        : middle_(std::move(middle)), index_(index) {}
    ~FuzzingEndpoint() override;

    bool Read(absl::AnyInvocable<void(absl::Status)> on_read,
              SliceBuffer* buffer, ReadArgs args) override;
    bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
               SliceBuffer* data, WriteArgs args) override;
    const ResolvedAddress& GetPeerAddress() const override {
      return middle_->addrs[peer_index()];
    }
    const ResolvedAddress& GetLocalAddress() const override {
      return middle_->addrs[my_index()];
    }
    std::vector<size_t> AllWriteMetrics() override;
    std::optional<absl::string_view> GetMetricName(size_t key) override;
    std::optional<size_t> GetMetricKey(absl::string_view name) override;

   private:
    int my_index() const { return index_; }
    int peer_index() const { return 1 - index_; }
    // Schedule additional writes to be performed later.
    // Takes a ref to middle instead of holding this, so that should the
    // endpoint be destroyed we don't have to worry about use-after-free.
    // Instead that scheduled callback will see the middle is closed and finally
    // report completion to the caller.
    // Since there is no timeliness contract for the completion of writes after
    // endpoint shutdown, it's believed this is a legal implementation.
    static void ScheduleDelayedWrite(
        std::shared_ptr<EndpointMiddle> middle, int index,
        absl::AnyInvocable<void(absl::Status)> on_writable, SliceBuffer* data,
        IoToken write_token) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
    const std::shared_ptr<EndpointMiddle> middle_;
    const int index_;
  };

  void RunLocked(RunType run_type, absl::AnyInvocable<void()> closure)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    RunAfterLocked(run_type, Duration::zero(), std::move(closure));
  }

  TaskHandle RunAfterLocked(RunType run_type, Duration when,
                            absl::AnyInvocable<void()> closure)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
  TaskHandle RunAfterExactlyLocked(Duration when,
                                   absl::AnyInvocable<void()> closure)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  // Allocate a port. Considered fuzzer selected port orderings first, and then
  // falls back to an exhaustive incremental search from port #1.
  int AllocatePort() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);
  // Is the given port in use by any listener?
  bool IsPortUsed(int port) ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  bool IsIdleLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  // Called whenever the time is incremented, and used by
  // ThreadedFuzzingEventEngine to insert a sleep so that real time passes at
  // approximately the same rate as FuzzingEventEngine time.
  // TODO(ctiller): This is very approximate and unsound and we should probably
  // evaluate whether we want to continue supporting ThreadedFuzzingEventEngine
  // at all.
  virtual void OnClockIncremented(Duration) {}

  // For the next connection being built, query the list of fuzzer selected
  // write size limits.
  std::queue<size_t> WriteSizesForConnection()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_);

  gpr_timespec NowAsTimespec(gpr_clock_type clock_type)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(now_mu_);
  static gpr_timespec GlobalNowImpl(gpr_clock_type clock_type)
      ABSL_LOCKS_EXCLUDED(mu_);
  absl::Time NowAsAbslTime() ABSL_LOCKS_EXCLUDED(now_mu_);

  static grpc_core::NoDestruct<grpc_core::Mutex> mu_;
  static grpc_core::NoDestruct<grpc_core::Mutex> now_mu_
      ABSL_ACQUIRED_AFTER(mu_);

  const Duration max_delay_[2];
  intptr_t next_task_id_ ABSL_GUARDED_BY(mu_) = 1;
  // Start at 5 seconds after the epoch.
  // This needs to be more than 1, and otherwise is kind of arbitrary.
  // The grpc_core::Timer code special cases the zero second time period after
  // epoch to allow for some fancy atomic stuff.
  Time now_ ABSL_GUARDED_BY(now_mu_) = Time() + std::chrono::seconds(5);
  std::queue<Duration> task_delays_ ABSL_GUARDED_BY(mu_);
  const absl::Time epoch_ = absl::Now();
  std::map<int, std::vector<fuzzing_event_engine::ReturnedEndpointMetrics>>
      returned_endpoint_metrics_ ABSL_GUARDED_BY(mu_);
  std::map<size_t, std::string> endpoint_metrics_by_id_;
  std::map<std::string, size_t> endpoint_metrics_by_name_;
  std::map<intptr_t, std::shared_ptr<Task>> tasks_by_id_ ABSL_GUARDED_BY(mu_);
  std::multimap<Time, std::shared_ptr<Task>> tasks_by_time_
      ABSL_GUARDED_BY(mu_);
  std::set<std::shared_ptr<ListenerInfo>> listeners_ ABSL_GUARDED_BY(mu_);
  // Fuzzer selected port allocations.
  std::queue<int> free_ports_ ABSL_GUARDED_BY(mu_);
  // Next free port to allocate once fuzzer selections are exhausted.
  int next_free_port_ ABSL_GUARDED_BY(mu_) = 1;
  // Ports that were included in the fuzzer selected port orderings.
  std::set<int> fuzzer_mentioned_ports_ ABSL_GUARDED_BY(mu_);
  // Fuzzer selected write sizes for future connections - one picked off per
  // WriteSizesForConnection() call.
  std::queue<std::queue<size_t>> write_sizes_for_future_connections_
      ABSL_GUARDED_BY(mu_);
  grpc_pick_port_functions previous_pick_port_functions_;
  std::atomic<size_t> outstanding_writes_{0};
  std::atomic<size_t> outstanding_reads_{0};

  // TODO(ctiller): these can be removed when IsSaneTimerEnvironment() is
  // guaranteed to be true.
  Duration exponential_gate_time_increment_ ABSL_GUARDED_BY(mu_) =
      std::chrono::milliseconds(1);
  intptr_t current_tick_ ABSL_GUARDED_BY(now_mu_) = 0;

  grpc_core::Mutex run_after_duration_callback_mu_;
  absl::AnyInvocable<void(Duration)> run_after_duration_callback_
      ABSL_GUARDED_BY(run_after_duration_callback_mu_);
  int next_write_id_ ABSL_GUARDED_BY(mu_) = 1;
};

class ThreadedFuzzingEventEngine : public FuzzingEventEngine {
 public:
  ThreadedFuzzingEventEngine()
      : ThreadedFuzzingEventEngine(std::chrono::milliseconds(10)) {}

  explicit ThreadedFuzzingEventEngine(Duration max_time)
      : FuzzingEventEngine(FuzzingEventEngine::Options(),
                           fuzzing_event_engine::Actions()),
        main_([this, max_time]() {
          while (!done_.load()) {
            Tick(max_time);
          }
        }) {}

  ~ThreadedFuzzingEventEngine() override {
    done_.store(true);
    main_.join();
  }

 private:
  void OnClockIncremented(Duration duration) override {
    absl::SleepFor(absl::Milliseconds(
        1 + grpc_event_engine::experimental::Milliseconds(duration)));
  }

  std::atomic<bool> done_{false};
  std::thread main_;
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_FUZZING_EVENT_ENGINE_FUZZING_EVENT_ENGINE_H
