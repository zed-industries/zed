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

#ifndef GRPC_SRC_CORE_LIB_SURFACE_FILTER_STACK_CALL_H
#define GRPC_SRC_CORE_LIB_SURFACE_FILTER_STACK_CALL_H

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
#include <vector>

#include "absl/log/check.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_join.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/surface/call.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/surface/completion_queue.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/server/server_interface.h"
#include "src/core/telemetry/call_tracer.h"
#include "src/core/util/alloc.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"

namespace grpc_core {

///////////////////////////////////////////////////////////////////////////////
// FilterStackCall
// To be removed once promise conversion is complete

class FilterStackCall final : public Call {
 public:
  ~FilterStackCall() override {
    gpr_free(static_cast<void*>(const_cast<char*>(final_info_.error_string)));
  }

  bool Completed() override {
    return gpr_atm_acq_load(&received_final_op_atm_) != 0;
  }

  // TODO(ctiller): return absl::StatusOr<SomeSmartPointer<Call>>?
  static grpc_error_handle Create(grpc_call_create_args* args,
                                  grpc_call** out_call);

  static Call* FromTopElem(grpc_call_element* elem) {
    return FromCallStack(grpc_call_stack_from_top_element(elem));
  }

  grpc_call_stack* call_stack() override {
    return reinterpret_cast<grpc_call_stack*>(
        reinterpret_cast<char*>(this) +
        GPR_ROUND_UP_TO_ALIGNMENT_SIZE(sizeof(*this)));
  }

  grpc_call_element* call_elem(size_t idx) {
    return grpc_call_stack_element(call_stack(), idx);
  }

  CallCombiner* call_combiner() { return &call_combiner_; }

  void CancelWithError(grpc_error_handle error) override;
  void SetCompletionQueue(grpc_completion_queue* cq) override;
  grpc_call_error StartBatch(const grpc_op* ops, size_t nops, void* notify_tag,
                             bool is_notify_tag_closure) override;
  void ExternalRef() override { ext_ref_.Ref(); }
  void ExternalUnref() override;
  void InternalRef(const char* reason) override {
    GRPC_CALL_STACK_REF(call_stack(), reason);
  }
  void InternalUnref(const char* reason) override {
    GRPC_CALL_STACK_UNREF(call_stack(), reason);
  }

  bool is_trailers_only() const override {
    bool result = is_trailers_only_;
    DCHECK(!result || recv_initial_metadata_.TransportSize() == 0);
    return result;
  }

  bool failed_before_recv_message() const override {
    return call_failed_before_recv_message_;
  }

  uint32_t test_only_message_flags() override {
    return test_only_last_message_flags_;
  }

  absl::string_view GetServerAuthority() const override {
    const Slice* authority_metadata =
        recv_initial_metadata_.get_pointer(HttpAuthorityMetadata());
    if (authority_metadata == nullptr) return "";
    return authority_metadata->as_string_view();
  }

  static size_t InitialSizeEstimate() {
    return sizeof(FilterStackCall) +
           (sizeof(BatchControl) * kMaxConcurrentBatches);
  }

  char* GetPeer() final;

  grpc_compression_options compression_options() override {
    return channel_->compression_options();
  }

  void DeleteThis() {
    auto arena = this->arena()->Ref();
    this->~FilterStackCall();
  }

  Channel* channel() const { return channel_.get(); }

 private:
  class ScopedContext : public promise_detail::Context<Arena> {
   public:
    explicit ScopedContext(FilterStackCall* call)
        : promise_detail::Context<Arena>(call->arena()) {}
  };

  static constexpr gpr_atm kRecvNone = 0;
  static constexpr gpr_atm kRecvInitialMetadataFirst = 1;

  enum class PendingOp {
    kRecvMessage,
    kRecvInitialMetadata,
    kRecvTrailingMetadata,
    kSends
  };
  static intptr_t PendingOpMask(PendingOp op) {
    return static_cast<intptr_t>(1) << static_cast<intptr_t>(op);
  }
  static std::string PendingOpString(intptr_t pending_ops) {
    std::vector<absl::string_view> pending_op_strings;
    if (pending_ops & PendingOpMask(PendingOp::kRecvMessage)) {
      pending_op_strings.push_back("kRecvMessage");
    }
    if (pending_ops & PendingOpMask(PendingOp::kRecvInitialMetadata)) {
      pending_op_strings.push_back("kRecvInitialMetadata");
    }
    if (pending_ops & PendingOpMask(PendingOp::kRecvTrailingMetadata)) {
      pending_op_strings.push_back("kRecvTrailingMetadata");
    }
    if (pending_ops & PendingOpMask(PendingOp::kSends)) {
      pending_op_strings.push_back("kSends");
    }
    return absl::StrCat("{", absl::StrJoin(pending_op_strings, ","), "}");
  }
  struct BatchControl {
    FilterStackCall* call_ = nullptr;
    CallTracerAnnotationInterface* call_tracer_ = nullptr;
    grpc_transport_stream_op_batch op_;
    // Share memory for cq_completion and notify_tag as they are never needed
    // simultaneously. Each byte used in this data structure count as six bytes
    // per call, so any savings we can make are worthwhile,

    // We use notify_tag to determine whether or not to send notification to the
    // completion queue. Once we've made that determination, we can reuse the
    // memory for cq_completion.
    union {
      grpc_cq_completion cq_completion;
      struct {
        // Any given op indicates completion by either (a) calling a closure or
        // (b) sending a notification on the call's completion queue.  If
        // \a is_closure is true, \a tag indicates a closure to be invoked;
        // otherwise, \a tag indicates the tag to be used in the notification to
        // be sent to the completion queue.
        void* tag;
        bool is_closure;
      } notify_tag;
    } completion_data_;
    grpc_closure start_batch_;
    grpc_closure finish_batch_;
    std::atomic<intptr_t> ops_pending_{0};
    AtomicError batch_error_;
    void set_pending_ops(uintptr_t ops) {
      ops_pending_.store(ops, std::memory_order_release);
    }
    bool completed_batch_step(PendingOp op) {
      auto mask = PendingOpMask(op);
      auto r = ops_pending_.fetch_sub(mask, std::memory_order_acq_rel);
      GRPC_TRACE_VLOG(call, 2)
          << "BATCH:" << this << " COMPLETE:" << PendingOpString(mask)
          << " REMAINING:" << PendingOpString(r & ~mask)
          << " (tag:" << completion_data_.notify_tag.tag << ")";
      CHECK_NE((r & mask), 0);
      return r == mask;
    }

    void PostCompletion();
    void FinishStep(PendingOp op);
    void ProcessDataAfterMetadata();
    void ReceivingStreamReady(grpc_error_handle error);
    void ReceivingInitialMetadataReady(grpc_error_handle error);
    void ReceivingTrailingMetadataReady(grpc_error_handle error);
    void FinishBatch(grpc_error_handle error);
  };

  FilterStackCall(RefCountedPtr<Arena> arena,
                  const grpc_call_create_args& args);

  static void ReleaseCall(void* call, grpc_error_handle);
  static void DestroyCall(void* call, grpc_error_handle);

  static FilterStackCall* FromCallStack(grpc_call_stack* call_stack) {
    return reinterpret_cast<FilterStackCall*>(
        reinterpret_cast<char*>(call_stack) -
        GPR_ROUND_UP_TO_ALIGNMENT_SIZE(sizeof(FilterStackCall)));
  }

  void ExecuteBatch(grpc_transport_stream_op_batch* batch,
                    grpc_closure* start_batch_closure);
  void SetFinalStatus(grpc_error_handle error);
  BatchControl* ReuseOrAllocateBatchControl(const grpc_op* ops);
  bool PrepareApplicationMetadata(size_t count, grpc_metadata* metadata,
                                  bool is_trailing);
  void PublishAppMetadata(grpc_metadata_batch* b, bool is_trailing);
  void RecvInitialFilter(grpc_metadata_batch* b);
  void RecvTrailingFilter(grpc_metadata_batch* b,
                          grpc_error_handle batch_error);

  grpc_compression_algorithm incoming_compression_algorithm() override {
    return incoming_compression_algorithm_;
  }
  void SetIncomingCompressionAlgorithm(
      grpc_compression_algorithm algorithm) override {
    incoming_compression_algorithm_ = algorithm;
  }

  RefCountedPtr<Channel> channel_;
  RefCount ext_ref_;
  CallCombiner call_combiner_;
  grpc_completion_queue* cq_;
  grpc_polling_entity pollent_;

  /// has grpc_call_unref been called
  bool destroy_called_ = false;
  // Trailers-only response status
  bool is_trailers_only_ = false;
  /// which ops are in-flight
  bool sent_initial_metadata_ = false;
  bool sending_message_ = false;
  bool sent_final_op_ = false;
  bool received_initial_metadata_ = false;
  bool receiving_message_ = false;
  bool requested_final_op_ = false;
  gpr_atm received_final_op_atm_ = 0;

  BatchControl* active_batches_[kMaxConcurrentBatches] = {};
  grpc_transport_stream_op_batch_payload stream_op_payload_;

  // first idx: is_receiving, second idx: is_trailing
  grpc_metadata_batch send_initial_metadata_;
  grpc_metadata_batch send_trailing_metadata_;
  grpc_metadata_batch recv_initial_metadata_;
  grpc_metadata_batch recv_trailing_metadata_;

  // Buffered read metadata waiting to be returned to the application.
  // Element 0 is initial metadata, element 1 is trailing metadata.
  grpc_metadata_array* buffered_metadata_[2] = {};

  // Call data useful used for reporting. Only valid after the call has
  // completed
  grpc_call_final_info final_info_;

  SliceBuffer send_slice_buffer_;
  std::optional<SliceBuffer> receiving_slice_buffer_;
  uint32_t receiving_stream_flags_;
  uint32_t test_only_last_message_flags_ = 0;
  // Compression algorithm for *incoming* data
  grpc_compression_algorithm incoming_compression_algorithm_ =
      GRPC_COMPRESS_NONE;

  bool call_failed_before_recv_message_ = false;
  grpc_byte_buffer** receiving_buffer_ = nullptr;
  grpc_slice receiving_slice_ = grpc_empty_slice();
  grpc_closure receiving_stream_ready_;
  grpc_closure receiving_initial_metadata_ready_;
  grpc_closure receiving_trailing_metadata_ready_;
  // Status about operation of call
  bool sent_server_trailing_metadata_ = false;
  gpr_atm cancelled_with_error_ = 0;

  grpc_closure release_call_;

  union {
    struct {
      grpc_status_code* status;
      grpc_slice* status_details;
      const char** error_string;
    } client;
    struct {
      int* cancelled;
      // backpointer to owning server if this is a server side call.
      ServerInterface* core_server;
    } server;
  } final_op_;
  AtomicError status_error_;

  // recv_state can contain one of the following values:
  // RECV_NONE :                 :  no initial metadata and messages received
  // RECV_INITIAL_METADATA_FIRST :  received initial metadata first
  // a batch_control*            :  received messages first

  //             +------1------RECV_NONE------3-----+
  //             |                                  |
  //             |                                  |
  //             v                                  v
  // RECV_INITIAL_METADATA_FIRST        receiving_stream_ready_bctlp
  //       |           ^                      |           ^
  //       |           |                      |           |
  //       +-----2-----+                      +-----4-----+

  // For 1, 4: See receiving_initial_metadata_ready() function
  // For 2, 3: See receiving_stream_ready() function
  gpr_atm recv_state_ = 0;
};

// Create a new call based on \a args.
// Regardless of success or failure, always returns a valid new call into *call
//
grpc_error_handle grpc_call_create(grpc_call_create_args* args,
                                   grpc_call** call);

// Given the top call_element, get the call object.
grpc_call* grpc_call_from_top_element(grpc_call_element* surface_element);

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_LIB_SURFACE_FILTER_STACK_CALL_H
