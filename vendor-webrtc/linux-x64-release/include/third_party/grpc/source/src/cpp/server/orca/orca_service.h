//
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
//

#ifndef GRPC_SRC_CPP_SERVER_ORCA_ORCA_SERVICE_H
#define GRPC_SRC_CPP_SERVER_ORCA_ORCA_SERVICE_H

#include <grpc/event_engine/event_engine.h>
#include <grpcpp/ext/orca_service.h>
#include <grpcpp/impl/sync.h>
#include <grpcpp/support/byte_buffer.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/support/status.h>

#include <atomic>
#include <memory>
#include <optional>

#include "absl/base/thread_annotations.h"
#include "absl/strings/string_view.h"
#include "src/core/util/ref_counted.h"

namespace grpc {
namespace experimental {

class OrcaService::Reactor
    : public ServerWriteReactor<ByteBuffer>,
      public grpc_core::RefCounted<OrcaService::Reactor> {
 public:
  explicit Reactor(OrcaService* service, absl::string_view peer,
                   const ByteBuffer* request_buffer,
                   std::shared_ptr<OrcaService::ReactorHook> hook);

  void OnWriteDone(bool ok) override;

  void OnCancel() override;

  void OnDone() override;

 private:
  void FinishRpc(grpc::Status status);

  void SendResponse();

  bool MaybeScheduleTimer();

  bool MaybeCancelTimer();

  void OnTimer();

  OrcaService* service_;

  grpc::internal::Mutex timer_mu_;
  std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
      timer_handle_ ABSL_GUARDED_BY(&timer_mu_);
  bool cancelled_ ABSL_GUARDED_BY(&timer_mu_) = false;

  grpc_event_engine::experimental::EventEngine::Duration report_interval_;
  ByteBuffer response_;
  std::shared_ptr<ReactorHook> hook_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine_;
};

}  // namespace experimental
}  // namespace grpc

#endif  // GRPC_SRC_CPP_SERVER_ORCA_ORCA_SERVICE_H
