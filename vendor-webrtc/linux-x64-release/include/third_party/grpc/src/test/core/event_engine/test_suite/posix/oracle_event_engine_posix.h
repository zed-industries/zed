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

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_POSIX_ORACLE_EVENT_ENGINE_POSIX_H
#define GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_POSIX_ORACLE_EVENT_ENGINE_POSIX_H

#include <grpc/event_engine/endpoint_config.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/slice_buffer.h>

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/base/thread_annotations.h"
#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/util/crash.h"
#include "src/core/util/notification.h"
#include "src/core/util/sync.h"
#include "src/core/util/thd.h"
#include "test/core/event_engine/event_engine_test_utils.h"

namespace grpc_event_engine {
namespace experimental {

class PosixOracleEndpoint : public EventEngine::Endpoint {
 public:
  explicit PosixOracleEndpoint(int socket_fd);
  static std::unique_ptr<PosixOracleEndpoint> Create(int socket_fd);
  ~PosixOracleEndpoint() override;
  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs args) override;
  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, WriteArgs args) override;
  void Shutdown();
  EventEngine::ResolvedAddress& GetPeerAddress() const override {
    grpc_core::Crash("unimplemented");
  }
  EventEngine::ResolvedAddress& GetLocalAddress() const override {
    grpc_core::Crash("unimplemented");
  }
  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

 private:
  // An internal helper class definition of Read operations to be performed
  // by the TCPServerEndpoint.
  class ReadOperation {
   public:
    ReadOperation()
        : num_bytes_to_read_(-1), buffer_(nullptr), on_complete_(nullptr) {}
    ReadOperation(int num_bytes_to_read, SliceBuffer* buffer,
                  absl::AnyInvocable<void(absl::Status)>&& on_complete)
        : num_bytes_to_read_(num_bytes_to_read),
          buffer_(buffer),
          on_complete_(std::move(on_complete)) {}
    bool IsValid() { return num_bytes_to_read_ >= 0 && buffer_ != nullptr; }
    int GetNumBytesToRead() const { return num_bytes_to_read_; }
    void operator()(std::string read_data, absl::Status status) {
      if (on_complete_ != nullptr) {
        AppendStringToSliceBuffer(std::exchange(buffer_, nullptr), read_data);
        std::exchange(on_complete_, nullptr)(status);
      }
    }

   private:
    int num_bytes_to_read_;
    SliceBuffer* buffer_;
    absl::AnyInvocable<void(absl::Status)> on_complete_;
  };

  // An internal helper class definition of Write operations to be performed
  // by the TCPServerEndpoint.
  class WriteOperation {
   public:
    WriteOperation() : bytes_to_write_(std::string()), on_complete_(nullptr) {}
    WriteOperation(SliceBuffer* buffer,
                   absl::AnyInvocable<void(absl::Status)>&& on_complete)
        : bytes_to_write_(ExtractSliceBufferIntoString(buffer)),
          on_complete_(std::move(on_complete)) {}
    bool IsValid() { return !bytes_to_write_.empty(); }
    std::string GetBytesToWrite() const { return bytes_to_write_; }
    void operator()(absl::Status status) {
      if (on_complete_ != nullptr) {
        std::exchange(on_complete_, nullptr)(status);
      }
    }

   private:
    std::string bytes_to_write_;
    absl::AnyInvocable<void(absl::Status)> on_complete_;
  };

  void ProcessReadOperations();
  void ProcessWriteOperations();

  mutable grpc_core::Mutex mu_;
  bool is_shutdown_ = false;
  int socket_fd_;
  ReadOperation read_ops_channel_;
  WriteOperation write_ops_channel_;
  std::unique_ptr<grpc_core::Notification> read_op_signal_{
      new grpc_core::Notification()};
  std::unique_ptr<grpc_core::Notification> write_op_signal_{
      new grpc_core::Notification()};
  grpc_core::Thread read_ops_ ABSL_GUARDED_BY(mu_);
  grpc_core::Thread write_ops_ ABSL_GUARDED_BY(mu_);
};

class PosixOracleListener : public EventEngine::Listener {
 public:
  PosixOracleListener(
      EventEngine::Listener::AcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory);
  ~PosixOracleListener() override;
  absl::StatusOr<int> Bind(const EventEngine::ResolvedAddress& addr) override;
  absl::Status Start() override;

 private:
  void HandleIncomingConnections();

  mutable grpc_core::Mutex mu_;
  EventEngine::Listener::AcceptCallback on_accept_;
  absl::AnyInvocable<void(absl::Status)> on_shutdown_;
  std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory_;
  grpc_core::Thread serve_;
  int pipefd_[2];
  bool is_started_ = false;
  std::vector<int> listener_fds_;
};

// A posix based oracle EventEngine.
class PosixOracleEventEngine final : public EventEngine {
 public:
  PosixOracleEventEngine() = default;
  ~PosixOracleEventEngine() override = default;

  absl::StatusOr<std::unique_ptr<Listener>> CreateListener(
      Listener::AcceptCallback on_accept,
      absl::AnyInvocable<void(absl::Status)> on_shutdown,
      const EndpointConfig& /*config*/,
      std::unique_ptr<MemoryAllocatorFactory> memory_allocator_factory)
      override {
    return std::make_unique<PosixOracleListener>(
        std::move(on_accept), std::move(on_shutdown),
        std::move(memory_allocator_factory));
  }

  ConnectionHandle Connect(OnConnectCallback on_connect,
                           const ResolvedAddress& addr,
                           const EndpointConfig& args,
                           MemoryAllocator memory_allocator,
                           EventEngine::Duration timeout) override;

  bool CancelConnect(ConnectionHandle /*handle*/) override {
    grpc_core::Crash("unimplemented");
  }
  bool IsWorkerThread() override { return false; };
  absl::StatusOr<std::unique_ptr<DNSResolver>> GetDNSResolver(
      const DNSResolver::ResolverOptions& /*options*/) override {
    grpc_core::Crash("unimplemented");
  }
  void Run(Closure* /*closure*/) override { grpc_core::Crash("unimplemented"); }
  void Run(absl::AnyInvocable<void()> /*closure*/) override {
    grpc_core::Crash("unimplemented");
  }
  TaskHandle RunAfter(EventEngine::Duration /*duration*/,
                      Closure* /*closure*/) override {
    grpc_core::Crash("unimplemented");
  }
  TaskHandle RunAfter(EventEngine::Duration /*duration*/,
                      absl::AnyInvocable<void()> /*closure*/) override {
    grpc_core::Crash("unimplemented");
  }
  bool Cancel(TaskHandle /*handle*/) override {
    grpc_core::Crash("unimplemented");
  }
};

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_TEST_SUITE_POSIX_ORACLE_EVENT_ENGINE_POSIX_H
