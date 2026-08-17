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

#ifndef GRPC_TEST_CORE_EVENT_ENGINE_EVENT_ENGINE_TEST_UTILS_H
#define GRPC_TEST_CORE_EVENT_ENGINE_EVENT_ENGINE_TEST_UTILS_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/slice_buffer.h>

#include <functional>
#include <map>
#include <memory>
#include <string>
#include <thread>
#include <tuple>
#include <utility>
#include <vector>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/util/notification.h"
#include "src/core/util/sync.h"

using EventEngineFactory = std::function<
    std::unique_ptr<grpc_event_engine::experimental::EventEngine>()>;

namespace grpc_event_engine {
namespace experimental {

std::string ExtractSliceBufferIntoString(SliceBuffer* buf);

// Returns a random message with bounded length.
std::string GetNextSendMessage();

// A helper method to exchange data between two endpoints. It is assumed
// that both endpoints are connected. The data (specified as a string) is
// written by the sender_endpoint and read by the receiver_endpoint. It
// returns OK status only if data written == data read. It also blocks the
// calling thread until said Write and Read operations are complete.
absl::Status SendValidatePayload(absl::string_view data,
                                 EventEngine::Endpoint* send_endpoint,
                                 EventEngine::Endpoint* receive_endpoint);

// A helper class to create clients/listeners and connections between them.
// The clients and listeners can be created by the oracle EventEngine
// or the EventEngine under test. The class provides handles into the
// connections that are created. Individual tests can test expected behavior by
// exchanging arbitrary data over these connections.
class ConnectionManager {
 public:
  ConnectionManager(std::unique_ptr<EventEngine> test_event_engine,
                    std::unique_ptr<EventEngine> oracle_event_engine)
      : memory_quota_(std::make_unique<grpc_core::MemoryQuota>("foo")),
        test_event_engine_(std::move(test_event_engine)),
        oracle_event_engine_(std::move(oracle_event_engine)) {}
  ~ConnectionManager() = default;

  // It creates and starts a listener bound to all the specified list of
  //  addresses.  If successful, return OK status. The type of the listener is
  //  determined by the 2nd argument.
  absl::Status BindAndStartListener(const std::vector<std::string>& addrs,
                                    bool listener_type_oracle = true);

  // If connection is successful, returns a tuple containing:
  //    1. a pointer to the client side endpoint of the connection.
  //    2. a pointer to the server side endpoint of the connection.
  // If un-successful it returns a non-OK  status containing the error
  // encountered.
  absl::StatusOr<std::tuple<std::unique_ptr<EventEngine::Endpoint>,
                            std::unique_ptr<EventEngine::Endpoint>>>
  CreateConnection(std::string target_addr, EventEngine::Duration timeout,
                   bool client_type_oracle);

 private:
  class Connection {
   public:
    Connection() = default;
    ~Connection() = default;

    void SetClientEndpoint(
        std::unique_ptr<EventEngine::Endpoint>&& client_endpoint) {
      client_endpoint_ = std::move(client_endpoint);
      client_signal_.Notify();
    }
    void SetServerEndpoint(
        std::unique_ptr<EventEngine::Endpoint>&& server_endpoint) {
      server_endpoint_ = std::move(server_endpoint);
      server_signal_.Notify();
    }
    std::unique_ptr<EventEngine::Endpoint> GetClientEndpoint() {
      auto client_endpoint = std::move(client_endpoint_);
      client_endpoint_.reset();
      return client_endpoint;
    }
    std::unique_ptr<EventEngine::Endpoint> GetServerEndpoint() {
      auto server_endpoint = std::move(server_endpoint_);
      server_endpoint_.reset();
      return server_endpoint;
    }

   private:
    std::unique_ptr<EventEngine::Endpoint> client_endpoint_;
    std::unique_ptr<EventEngine::Endpoint> server_endpoint_;
    grpc_core::Notification client_signal_;
    grpc_core::Notification server_signal_;
  };

  grpc_core::Mutex mu_;
  std::unique_ptr<grpc_core::MemoryQuota> memory_quota_;
  int num_processed_connections_ = 0;
  Connection last_in_progress_connection_;
  std::map<std::string, std::shared_ptr<EventEngine::Listener>> listeners_;
  std::unique_ptr<EventEngine> test_event_engine_;
  std::unique_ptr<EventEngine> oracle_event_engine_;
};

void AppendStringToSliceBuffer(SliceBuffer* buf, absl::string_view data);

class NotifyOnDelete {
 public:
  explicit NotifyOnDelete(grpc_core::Notification* signal) : signal_(signal) {}
  NotifyOnDelete(const NotifyOnDelete&) = delete;
  NotifyOnDelete& operator=(const NotifyOnDelete&) = delete;
  NotifyOnDelete(NotifyOnDelete&& other) noexcept {
    signal_ = other.signal_;
    other.signal_ = nullptr;
  }
  NotifyOnDelete& operator=(NotifyOnDelete&& other) noexcept {
    signal_ = other.signal_;
    other.signal_ = nullptr;
    return *this;
  }
  ~NotifyOnDelete() {
    if (signal_ != nullptr) {
      signal_->Notify();
    }
  }

 private:
  grpc_core::Notification* signal_;
};

// An endpoint implementation that supports Read and Write via std::threads.
// Passing a grpc_core::Notification will allow owners to know when all
// in-flight callbacks have been run, and all endpoint state has been destroyed.
class ThreadedNoopEndpoint : public EventEngine::Endpoint {
 public:
  explicit ThreadedNoopEndpoint(grpc_core::Notification* destroyed)
      : state_(std::make_shared<EndpointState>(destroyed)) {}
  ~ThreadedNoopEndpoint() override {
    std::thread deleter([state = state_]() {
      CleanupThread(state->read);
      CleanupThread(state->write);
    });
    deleter.detach();
  }

  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs /* args */) override {
    buffer->Clear();
    CleanupThread(state_->read);
    state_->read = new std::thread([cb = std::move(on_read)]() mutable {
      cb(absl::UnknownError("test"));
    });
    return false;
  }

  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, WriteArgs /* args */) override {
    data->Clear();
    CleanupThread(state_->write);
    state_->write = new std::thread([cb = std::move(on_writable)]() mutable {
      cb(absl::UnknownError("test"));
    });
    return false;
  }

  const EventEngine::ResolvedAddress& GetPeerAddress() const override {
    return peer_;
  }

  const EventEngine::ResolvedAddress& GetLocalAddress() const override {
    return local_;
  }

  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

 private:
  struct EndpointState {
    explicit EndpointState(grpc_core::Notification* deleter)
        : delete_notifier_(deleter) {}
    std::thread* read = nullptr;
    std::thread* write = nullptr;
    NotifyOnDelete delete_notifier_;
  };

  static void CleanupThread(std::thread* thd) {
    if (thd != nullptr) {
      thd->join();
      delete thd;
    }
  }

  std::shared_ptr<EndpointState> state_;
  EventEngine::ResolvedAddress peer_;
  EventEngine::ResolvedAddress local_;
};

// We need everything EventEngine to do reasonable timer steps -- without it
// we need to do a bunch of evil to make sure both timer systems are ticking
// each step.
bool IsSaneTimerEnvironment();

}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_TEST_CORE_EVENT_ENGINE_EVENT_ENGINE_TEST_UTILS_H
