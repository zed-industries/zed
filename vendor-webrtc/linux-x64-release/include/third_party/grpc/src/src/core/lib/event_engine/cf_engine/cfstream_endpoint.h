// Copyright 2023 The gRPC Authors
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
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFSTREAM_ENDPOINT_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFSTREAM_ENDPOINT_H
#include <grpc/support/port_platform.h>

#ifdef GPR_APPLE
#include <AvailabilityMacros.h>
#ifdef AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER

#include <CoreFoundation/CoreFoundation.h>
#include <grpc/event_engine/event_engine.h>

#include "absl/strings/str_format.h"
#include "src/core/lib/address_utils/sockaddr_utils.h"
#include "src/core/lib/event_engine/cf_engine/cf_engine.h"
#include "src/core/lib/event_engine/cf_engine/cftype_unique_ref.h"
#include "src/core/lib/event_engine/posix_engine/lockfree_event.h"
#include "src/core/lib/event_engine/tcp_socket_utils.h"
#include "src/core/util/host_port.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_event_engine::experimental {

class CFStreamEndpointImpl
    : public grpc_core::RefCounted<CFStreamEndpointImpl> {
 public:
  CFStreamEndpointImpl(std::shared_ptr<CFEventEngine> engine,
                       MemoryAllocator memory_allocator);
  ~CFStreamEndpointImpl();

  void Shutdown();

  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            EventEngine::Endpoint::ReadArgs args);
  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, EventEngine::Endpoint::WriteArgs args);

  const EventEngine::ResolvedAddress& GetPeerAddress() const {
    return peer_address_;
  }
  const EventEngine::ResolvedAddress& GetLocalAddress() const {
    return local_address_;
  }

 public:
  void Connect(absl::AnyInvocable<void(absl::Status)> on_connect,
               EventEngine::ResolvedAddress addr);
  bool CancelConnect(absl::Status status);

 private:
  void DoWrite(absl::AnyInvocable<void(absl::Status)> on_writable,
               SliceBuffer* data);
  void DoRead(absl::AnyInvocable<void(absl::Status)> on_read,
              SliceBuffer* buffer);

 private:
  static void* Retain(void* info) {
    auto that = static_cast<CFStreamEndpointImpl*>(info);
    return that->Ref().release();
  }

  static void Release(void* info) {
    auto that = static_cast<CFStreamEndpointImpl*>(info);
    that->Unref();
  }

  static void ReadCallback(CFReadStreamRef stream, CFStreamEventType type,
                           void* client_callback_info);
  static void WriteCallback(CFWriteStreamRef stream, CFStreamEventType type,
                            void* client_callback_info);

 private:
  CFTypeUniqueRef<CFReadStreamRef> cf_read_stream_;
  CFTypeUniqueRef<CFWriteStreamRef> cf_write_stream_;

  std::shared_ptr<CFEventEngine> engine_;

  EventEngine::ResolvedAddress peer_address_;
  EventEngine::ResolvedAddress local_address_;
  std::string peer_address_string_;
  std::string local_address_string_;
  MemoryAllocator memory_allocator_;

  LockfreeEvent open_event_;
  LockfreeEvent read_event_;
  LockfreeEvent write_event_;
};

class CFStreamEndpoint : public EventEngine::Endpoint {
 public:
  CFStreamEndpoint(std::shared_ptr<CFEventEngine> engine,
                   MemoryAllocator memory_allocator) {
    impl_ = grpc_core::MakeRefCounted<CFStreamEndpointImpl>(
        std::move(engine), std::move(memory_allocator));
  }
  ~CFStreamEndpoint() override { impl_->Shutdown(); }

  bool Read(absl::AnyInvocable<void(absl::Status)> on_read, SliceBuffer* buffer,
            ReadArgs args) override {
    return impl_->Read(std::move(on_read), buffer, std::move(args));
  }
  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             SliceBuffer* data, WriteArgs args) override {
    return impl_->Write(std::move(on_writable), data, std::move(args));
  }

  const EventEngine::ResolvedAddress& GetPeerAddress() const override {
    return impl_->GetPeerAddress();
  }
  const EventEngine::ResolvedAddress& GetLocalAddress() const override {
    return impl_->GetLocalAddress();
  }

  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

 public:
  void Connect(absl::AnyInvocable<void(absl::Status)> on_connect,
               EventEngine::ResolvedAddress addr) {
    impl_->Connect(std::move(on_connect), std::move(addr));
  }
  bool CancelConnect(absl::Status status) {
    return impl_->CancelConnect(std::move(status));
  }

 private:
  grpc_core::RefCountedPtr<CFStreamEndpointImpl> impl_;
};

}  // namespace grpc_event_engine::experimental

#endif  // AVAILABLE_MAC_OS_X_VERSION_10_12_AND_LATER
#endif  // GPR_APPLE

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_CF_ENGINE_CFSTREAM_ENDPOINT_H
