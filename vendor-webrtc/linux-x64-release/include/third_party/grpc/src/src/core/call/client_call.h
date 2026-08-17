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

#ifndef GRPC_SRC_CORE_CALL_CLIENT_CALL_H
#define GRPC_SRC_CORE_CALL_CLIENT_CALL_H

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
#include <string>

#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata.h"
#include "src/core/lib/promise/status_flag.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/surface/call.h"
#include "src/core/lib/surface/call_utils.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/single_set_ptr.h"

namespace grpc_core {

class ClientCall final
    : public Call,
      public DualRefCounted<ClientCall, NonPolymorphicRefCount,
                            UnrefCallDestroy> {
 public:
  ClientCall(grpc_call* parent_call, uint32_t propagation_mask,
             grpc_completion_queue* cq, Slice path,
             std::optional<Slice> authority, bool registered_method,
             Timestamp deadline, grpc_compression_options compression_options,
             RefCountedPtr<Arena> arena,
             RefCountedPtr<UnstartedCallDestination> destination);

  void CancelWithError(grpc_error_handle error) override;
  bool is_trailers_only() const override { return is_trailers_only_; }
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
    if (!saw_trailing_metadata_.load(std::memory_order_relaxed)) {
      CancelWithError(absl::CancelledError());
    }
  }

  void SetCompletionQueue(grpc_completion_queue*) override {
    Crash("unimplemented");
  }

  grpc_compression_options compression_options() override {
    return compression_options_;
  }

  grpc_call_stack* call_stack() override { return nullptr; }

  char* GetPeer() override;

  bool Completed() final { Crash("unimplemented"); }
  bool failed_before_recv_message() const final {
    return started_call_initiator_.WasCancelledPushed();
  }

  grpc_compression_algorithm incoming_compression_algorithm() override {
    return message_receiver_.incoming_compression_algorithm();
  }

  void SetIncomingCompressionAlgorithm(
      grpc_compression_algorithm algorithm) override {
    message_receiver_.SetIncomingCompressionAlgorithm(algorithm);
  }

  uint32_t test_only_message_flags() override {
    return message_receiver_.last_message_flags();
  }

  void Destroy() {
    auto arena = this->arena()->Ref();
    this->~ClientCall();
  }

 private:
  struct UnorderedStart {
    absl::AnyInvocable<void()> start_pending_batch;
    UnorderedStart* next;
  };

  void CommitBatch(const grpc_op* ops, size_t nops, void* notify_tag,
                   bool is_notify_tag_closure);
  template <typename Batch>
  void ScheduleCommittedBatch(Batch batch);
  Party::WakeupHold StartCall(const grpc_op& send_initial_metadata_op);
  // Attempt to start the call and send handler down the stack; returns true if
  // state was updated, false otherwise (with cur_state updated to the new
  // current state).
  // If this function returns false, it's guaranteed that handler is not
  // touched.
  // Should be called repeatedly until it returns true.
  bool StartCallMaybeUpdateState(uintptr_t& cur_state,
                                 UnstartedCallHandler& handler);

  std::string DebugTag() { return absl::StrFormat("CLIENT_CALL[%p]: ", this); }
  void OnReceivedStatus(ServerMetadataHandle server_trailing_metadata,
                        grpc_status_code* out_status,
                        grpc_slice* out_status_details,
                        const char** out_error_string,
                        grpc_metadata_array* out_trailing_metadata);

  // call_state_ is one of:
  // 1. kUnstarted - call has not yet been started
  // 2. pointer to an UnorderedStart - call has ops started, but no send initial
  //    metadata yet
  // 3. kStarted - call has been started and call_initiator_ is ready
  // 4. kCancelled - call was cancelled before starting
  // In cases (1) and (2) send_initial_metadata_ is used to store the initial
  // but unsent metadata.
  // In case (3) started_call_initiator_ is used to store the call initiator.
  // In case (4) no other state is used.
  enum CallState : uintptr_t {
    kUnstarted = 0,
    kStarted = 1,
    kCancelled = 2,
  };
  std::atomic<uintptr_t> call_state_{kUnstarted};
  ClientMetadataHandle send_initial_metadata_{
      Arena::MakePooledForOverwrite<ClientMetadata>()};
  CallInitiator started_call_initiator_;
  // Status passed to CancelWithError;
  // if call_state_ == kCancelled then this is the authoritative status,
  // otherwise the server trailing metadata from started_call_initiator_ is
  // authoritative.
  SingleSetPtr<absl::Status> cancel_status_;
  MessageReceiver message_receiver_;
  grpc_completion_queue* const cq_;
  const RefCountedPtr<UnstartedCallDestination> call_destination_;
  const grpc_compression_options compression_options_;
  ServerMetadataHandle received_initial_metadata_;
  ServerMetadataHandle received_trailing_metadata_;
  bool is_trailers_only_;
  std::atomic<bool> saw_trailing_metadata_{false};
};

grpc_call* MakeClientCall(grpc_call* parent_call, uint32_t propagation_mask,
                          grpc_completion_queue* cq, Slice path,
                          std::optional<Slice> authority,
                          bool registered_method, Timestamp deadline,
                          grpc_compression_options compression_options,
                          RefCountedPtr<Arena> arena,
                          RefCountedPtr<UnstartedCallDestination> destination);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_CLIENT_CALL_H
