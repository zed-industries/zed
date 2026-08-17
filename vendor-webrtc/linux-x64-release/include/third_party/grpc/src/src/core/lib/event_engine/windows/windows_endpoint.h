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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_ENDPOINT_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_ENDPOINT_H
#include <grpc/support/port_platform.h>

#ifdef GPR_WINDOWS

#include <grpc/event_engine/event_engine.h>

#include "src/core/lib/event_engine/thread_pool/thread_pool.h"
#include "src/core/lib/event_engine/windows/win_socket.h"

namespace grpc_event_engine::experimental {

class WindowsEndpoint : public EventEngine::Endpoint {
 public:
  WindowsEndpoint(const EventEngine::ResolvedAddress& peer_address,
                  std::unique_ptr<WinSocket> socket,
                  MemoryAllocator&& allocator, const EndpointConfig& config,
                  ThreadPool* thread_pool, std::shared_ptr<EventEngine> engine);
  ~WindowsEndpoint() override;
  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs args) override;
  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, WriteArgs args) override;
  const EventEngine::ResolvedAddress& GetPeerAddress() const override;
  const EventEngine::ResolvedAddress& GetLocalAddress() const override;
  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

 private:
  struct AsyncIOState;

  // Permanent closure type for Read callbacks
  class HandleReadClosure : public EventEngine::Closure {
   public:
    void Run() override;
    void Prime(std::shared_ptr<AsyncIOState> io_state, SliceBuffer* buffer,
               absl::AnyInvocable<void(absl::Status)> cb);
    // Resets the per-request data, releasing the ref on io_state_.
    // Returns the previous callback.
    [[nodiscard]] absl::AnyInvocable<void(absl::Status)>
    ResetAndReturnCallback();
    // Run the callback with whatever data is available, and reset state.
    //
    // Returns true if the callback has been called with some data. Returns
    // false if no data has been read.
    bool MaybeFinishIfDataHasAlreadyBeenRead();
    // Swap any leftover slices into the provided buffer
    void DonateSpareSlices(SliceBuffer* buffer);

   private:
    std::shared_ptr<AsyncIOState> io_state_;
    absl::AnyInvocable<void(absl::Status)> cb_;
    SliceBuffer* buffer_ = nullptr;
    SliceBuffer last_read_buffer_;
  };

  // Permanent closure type for Write callbacks
  class HandleWriteClosure : public EventEngine::Closure {
   public:
    void Run() override;
    void Prime(std::shared_ptr<AsyncIOState> io_state, SliceBuffer* buffer,
               absl::AnyInvocable<void(absl::Status)> cb);
    // Resets the per-request data, releasing the ref on io_state_.
    // Returns the previous callback.
    [[nodiscard]] absl::AnyInvocable<void(absl::Status)>
    ResetAndReturnCallback();

   private:
    std::shared_ptr<AsyncIOState> io_state_;
    absl::AnyInvocable<void(absl::Status)> cb_;
    SliceBuffer* buffer_ = nullptr;
  };

  // A class to manage the data that must outlive the Endpoint.
  //
  // Once an endpoint is done and destroyed, there still may be overlapped
  // operations pending. To clean up safely, this data must outlive the
  // Endpoint, and be destroyed asynchronously when all pending overlapped
  // events are complete.
  struct AsyncIOState {
    AsyncIOState(WindowsEndpoint* endpoint, std::unique_ptr<WinSocket> socket,
                 std::shared_ptr<EventEngine> engine, ThreadPool* thread_pool);
    ~AsyncIOState();

    // Perform the low-level calls and execute the HandleReadClosure
    // asynchronously.
    void DoTcpRead(SliceBuffer* buffer);

    WindowsEndpoint* const endpoint;
    std::unique_ptr<WinSocket> socket;
    HandleReadClosure handle_read_event;
    HandleWriteClosure handle_write_event;
    std::shared_ptr<EventEngine> engine;
    ThreadPool* thread_pool;
  };

  EventEngine::ResolvedAddress peer_address_;
  std::string peer_address_string_;
  EventEngine::ResolvedAddress local_address_;
  std::string local_address_string_;
  MemoryAllocator allocator_;
  std::shared_ptr<AsyncIOState> io_state_;
};

}  // namespace grpc_event_engine::experimental

#endif

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_WINDOWS_WINDOWS_ENDPOINT_H
