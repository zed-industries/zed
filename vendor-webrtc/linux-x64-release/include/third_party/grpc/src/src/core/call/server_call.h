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

#ifndef GRPC_SRC_CORE_CALL_SERVER_CALL_H
#define GRPC_SRC_CORE_CALL_SERVER_CALL_H

#include <grpc/byte_buffer.h>
#include <grpc/compression.h>
#include <grpc/event_engine/event_engine.h>
#include <grpc/grpc.h>
#include <grpc/impl/call.h>
#include <grpc/impl/propagation_bits.h>
#include <grpc/slice.h>
#include <grpc/slice_buffer.h>
#include <grpc/status.h>
#include <grpc/support/alloc.h>
#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <grpc/support/string_util.h>
#include <inttypes.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>

#include <atomic>
#include <cstdint>
#include <memory>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/promise/poll.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/surface/call.h"
#include "src/core/lib/surface/call_utils.h"
#include "src/core/server/server_interface.h"
#include "src/core/telemetry/stats.h"
#include "src/core/telemetry/stats_data.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

class ServerCall final : public Call, public DualRefCounted<ServerCall> {
 public:
  ServerCall(ClientMetadataHandle client_initial_metadata,
             CallHandler call_handler, ServerInterface* server,
             grpc_completion_queue* cq)
      : Call(false,
             client_initial_metadata->get(GrpcTimeoutMetadata())
                 .value_or(Timestamp::InfFuture()),
             call_handler.arena()->Ref()),
        call_handler_(std::move(call_handler)),
        client_initial_metadata_stored_(std::move(client_initial_metadata)),
        cq_(cq),
        server_(server) {
    global_stats().IncrementServerCallsCreated();
  }

  void CancelWithError(grpc_error_handle error) override {
    call_handler_.SpawnInfallible(
        "CancelWithError",
        [self = WeakRefAsSubclass<ServerCall>(), error = std::move(error)] {
          self->call_handler_.PushServerTrailingMetadata(
              CancelledServerMetadataFromStatus(error));
        });
  }
  bool is_trailers_only() const override {
    Crash("is_trailers_only not implemented for server calls");
  }
  absl::string_view GetServerAuthority() const override {
    Crash("unimplemented");
  }
  grpc_call_error StartBatch(const grpc_op* ops, size_t nops, void* notify_tag,
                             bool is_notify_tag_closure) override;

  void ExternalRef() override { Ref().release(); }
  void ExternalUnref() override { Unref(); }
  void InternalRef(const char*) override { WeakRef().release(); }
  void InternalUnref(const char*) override { WeakUnref(); }

  void Orphaned() override {
    if (!saw_was_cancelled_.load(std::memory_order_relaxed)) {
      CancelWithError(absl::CancelledError());
    }
  }

  void SetCompletionQueue(grpc_completion_queue*) override {
    Crash("unimplemented");
  }

  grpc_compression_options compression_options() override {
    return server_->compression_options();
  }

  grpc_call_stack* call_stack() override { return nullptr; }

  char* GetPeer() override {
    Slice peer_slice = GetPeerString();
    if (!peer_slice.empty()) {
      absl::string_view peer_string_view = peer_slice.as_string_view();
      char* peer_string =
          static_cast<char*>(gpr_malloc(peer_string_view.size() + 1));
      memcpy(peer_string, peer_string_view.data(), peer_string_view.size());
      peer_string[peer_string_view.size()] = '\0';
      return peer_string;
    }
    return gpr_strdup("unknown");
  }

  bool Completed() final { Crash("unimplemented"); }
  bool failed_before_recv_message() const final {
    return call_handler_.WasCancelledPushed();
  }

  uint32_t test_only_message_flags() override {
    return message_receiver_.last_message_flags();
  }

  grpc_compression_algorithm incoming_compression_algorithm() override {
    return message_receiver_.incoming_compression_algorithm();
  }

  void SetIncomingCompressionAlgorithm(
      grpc_compression_algorithm algorithm) override {
    message_receiver_.SetIncomingCompressionAlgorithm(algorithm);
  }

 private:
  void CommitBatch(const grpc_op* ops, size_t nops, void* notify_tag,
                   bool is_notify_tag_closure);

  std::string DebugTag() { return absl::StrFormat("SERVER_CALL[%p]: ", this); }

  CallHandler call_handler_;
  std::atomic<bool> sent_server_initial_metadata_batch_{false};
  Latch<void> server_initial_metadata_scheduled_;
  MessageReceiver message_receiver_;
  ClientMetadataHandle client_initial_metadata_stored_;
  grpc_completion_queue* const cq_;
  ServerInterface* const server_;
  std::atomic<bool> saw_was_cancelled_{false};
};

grpc_call* MakeServerCall(CallHandler call_handler,
                          ClientMetadataHandle client_initial_metadata,
                          ServerInterface* server, grpc_completion_queue* cq,
                          grpc_metadata_array* publish_initial_metadata);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_SERVER_CALL_H
