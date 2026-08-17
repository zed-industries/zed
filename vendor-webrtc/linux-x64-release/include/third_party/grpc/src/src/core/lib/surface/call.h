//
//
// Copyright 2015 gRPC authors.
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
//

#ifndef GRPC_SRC_CORE_LIB_SURFACE_CALL_H
#define GRPC_SRC_CORE_LIB_SURFACE_CALL_H

#include <grpc/grpc.h>
#include <grpc/impl/compression_types.h>
#include <grpc/support/atm.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "absl/functional/any_invocable.h"
#include "absl/functional/function_ref.h"
#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/debug/trace.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/promise/arena_promise.h"
#include "src/core/lib/promise/context.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/surface/channel.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/server/server_interface.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"
#include "src/core/util/time_precise.h"

typedef void (*grpc_ioreq_completion_func)(grpc_call* call, int success,
                                           void* user_data);

typedef struct grpc_call_create_args {
  grpc_core::RefCountedPtr<grpc_core::Channel> channel;
  grpc_core::ServerInterface* server;

  grpc_call* parent;
  uint32_t propagation_mask;

  grpc_completion_queue* cq;
  // if not NULL, it'll be used in lieu of cq
  grpc_pollset_set* pollset_set_alternative;

  const void* server_transport_data;

  std::optional<grpc_core::Slice> path;
  std::optional<grpc_core::Slice> authority;

  grpc_core::Timestamp send_deadline;
  bool registered_method;  // client_only
} grpc_call_create_args;

namespace grpc_core {

template <>
struct ArenaContextType<census_context> {
  static void Destroy(census_context*) {}
};

class Call : public CppImplOf<Call, grpc_call>,
             public grpc_event_engine::experimental::EventEngine::
                 Closure /* for deadlines */ {
 public:
  Arena* arena() const { return arena_.get(); }
  bool is_client() const { return is_client_; }

  virtual bool Completed() = 0;
  void CancelWithStatus(grpc_status_code status, const char* description);
  virtual void CancelWithError(grpc_error_handle error) = 0;
  virtual void SetCompletionQueue(grpc_completion_queue* cq) = 0;
  virtual char* GetPeer() = 0;
  virtual grpc_call_error StartBatch(const grpc_op* ops, size_t nops,
                                     void* notify_tag,
                                     bool is_notify_tag_closure) = 0;
  virtual bool failed_before_recv_message() const = 0;
  virtual bool is_trailers_only() const = 0;
  virtual absl::string_view GetServerAuthority() const = 0;
  virtual void ExternalRef() = 0;
  virtual void ExternalUnref() = 0;
  virtual void InternalRef(const char* reason) = 0;
  virtual void InternalUnref(const char* reason) = 0;

  void UpdateDeadline(Timestamp deadline) ABSL_LOCKS_EXCLUDED(deadline_mu_);
  void ResetDeadline() ABSL_LOCKS_EXCLUDED(deadline_mu_);
  Timestamp deadline() {
    MutexLock lock(&deadline_mu_);
    return deadline_;
  }

  virtual uint32_t test_only_message_flags() = 0;
  CompressionAlgorithmSet encodings_accepted_by_peer() {
    return encodings_accepted_by_peer_;
  }

  // This should return nullptr for the promise stack (and alternative means
  // for that functionality be invented)
  virtual grpc_call_stack* call_stack() = 0;

  // Implementation of EventEngine::Closure, called when deadline expires
  void Run() final;

  gpr_cycle_counter start_time() const { return start_time_; }

  void set_traced(bool traced) { traced_ = traced; }
  bool traced() const { return traced_; }

  virtual grpc_compression_algorithm incoming_compression_algorithm() = 0;

 protected:
  // The maximum number of concurrent batches possible.
  // Based upon the maximum number of individually queueable ops in the batch
  // api:
  //    - initial metadata send
  //    - message send
  //    - status/close send (depending on client/server)
  //    - initial metadata recv
  //    - message recv
  //    - status/close recv (depending on client/server)
  static constexpr size_t kMaxConcurrentBatches = 6;

  struct ParentCall {
    Mutex child_list_mu;
    Call* first_child ABSL_GUARDED_BY(child_list_mu) = nullptr;
  };

  struct ChildCall {
    explicit ChildCall(Call* parent) : parent(parent) {}
    Call* parent;
    /// siblings: children of the same parent form a list, and this list is
    /// protected under
    /// parent->mu
    Call* sibling_next = nullptr;
    Call* sibling_prev = nullptr;
  };

  Call(bool is_client, Timestamp send_deadline, RefCountedPtr<Arena> arena);
  ~Call() override = default;

  ParentCall* GetOrCreateParentCall();
  ParentCall* parent_call();

  absl::Status InitParent(Call* parent, uint32_t propagation_mask);
  void PublishToParent(Call* parent);
  void MaybeUnpublishFromParent();
  void PropagateCancellationToChildren();

  Timestamp send_deadline() const { return send_deadline_; }
  void set_send_deadline(Timestamp send_deadline) {
    send_deadline_ = send_deadline;
  }

  Slice GetPeerString() const {
    MutexLock lock(&peer_mu_);
    return peer_string_.Ref();
  }

  void SetPeerString(Slice peer_string) {
    MutexLock lock(&peer_mu_);
    peer_string_ = std::move(peer_string);
  }

  void ClearPeerString() { SetPeerString(Slice(grpc_empty_slice())); }

  // TODO(ctiller): cancel_func is for cancellation of the call - filter stack
  // holds no mutexes here, promise stack does, and so locking is different.
  // Remove this and cancel directly once promise conversion is done.
  void ProcessIncomingInitialMetadata(grpc_metadata_batch& md);
  // Fixup outgoing metadata before sending - adds compression, protects
  // internal headers against external modification.
  void PrepareOutgoingInitialMetadata(const grpc_op& op,
                                      grpc_metadata_batch& md);

  void HandleCompressionAlgorithmDisabled(
      grpc_compression_algorithm compression_algorithm) GPR_ATTRIBUTE_NOINLINE;
  void HandleCompressionAlgorithmNotAccepted(
      grpc_compression_algorithm compression_algorithm) GPR_ATTRIBUTE_NOINLINE;

  virtual grpc_compression_options compression_options() = 0;

  virtual void SetIncomingCompressionAlgorithm(
      grpc_compression_algorithm algorithm) = 0;

 private:
  const RefCountedPtr<Arena> arena_;
  std::atomic<ParentCall*> parent_call_{nullptr};
  ChildCall* child_ = nullptr;
  Timestamp send_deadline_;
  const bool is_client_;
  // flag indicating that cancellation is inherited
  bool cancellation_is_inherited_ = false;
  // Is this call traced?
  bool traced_ = false;
  // Supported encodings (compression algorithms), a bitset.
  // Always support no compression.
  CompressionAlgorithmSet encodings_accepted_by_peer_{GRPC_COMPRESS_NONE};
  // Peer name is protected by a mutex because it can be accessed by the
  // application at the same moment as it is being set by the completion
  // of the recv_initial_metadata op.  The mutex should be mostly uncontended.
  mutable Mutex peer_mu_;
  Slice peer_string_;
  // Current deadline.
  Mutex deadline_mu_;
  Timestamp deadline_ ABSL_GUARDED_BY(deadline_mu_) = Timestamp::InfFuture();
  grpc_event_engine::experimental::EventEngine::TaskHandle ABSL_GUARDED_BY(
      deadline_mu_) deadline_task_;
  gpr_cycle_counter start_time_ = gpr_get_cycle_counter();
};

template <>
struct ArenaContextType<Call> {
  static void Destroy(Call*) {}
};

}  // namespace grpc_core

// Create a new call based on \a args.
// Regardless of success or failure, always returns a valid new call into *call
//
grpc_error_handle grpc_call_create(grpc_call_create_args* args,
                                   grpc_call** call);

void grpc_call_set_completion_queue(grpc_call* call, grpc_completion_queue* cq);

grpc_core::Arena* grpc_call_get_arena(grpc_call* call);

grpc_call_stack* grpc_call_get_call_stack(grpc_call* call);

grpc_call_error grpc_call_start_batch_and_execute(grpc_call* call,
                                                  const grpc_op* ops,
                                                  size_t nops,
                                                  grpc_closure* closure);

// gRPC core internal version of grpc_call_cancel that does not create
// exec_ctx.
void grpc_call_cancel_internal(grpc_call* call);

// Given the top call_element, get the call object.
grpc_call* grpc_call_from_top_element(grpc_call_element* surface_element);

void grpc_call_log_batch(const char* file, int line, const grpc_op* ops,
                         size_t nops);

void grpc_call_tracer_set(grpc_call* call, grpc_core::ClientCallTracer* tracer);

// Sets call tracer on the call and manages its life by using the call's arena.
// When using this API, the tracer will be destroyed by grpc_call arena when
// grpc_call is about to be destroyed. The caller of this API SHOULD NOT
// manually destroy the tracer. This API is used by Python as a way of using
// Arena to manage the lifetime of the call tracer. Python needs this API
// because the tracer was created within a separate shared object library which
// doesn't have access to core functions like arena->ManagedNew<>.
void grpc_call_tracer_set_and_manage(grpc_call* call,
                                     grpc_core::ClientCallTracer* tracer);

void* grpc_call_tracer_get(grpc_call* call);

#define GRPC_CALL_LOG_BATCH(ops, nops)                    \
  do {                                                    \
    if (GRPC_TRACE_FLAG_ENABLED(api)) {                   \
      grpc_call_log_batch(__FILE__, __LINE__, ops, nops); \
    }                                                     \
  } while (0)

uint8_t grpc_call_is_client(grpc_call* call);

class ClientCallTracerWrapper {
 public:
  explicit ClientCallTracerWrapper(grpc_core::ClientCallTracer* tracer)
      : tracer_(tracer) {}

 private:
  std::unique_ptr<grpc_core::ClientCallTracer> tracer_;
};

// Return an appropriate compression algorithm for the requested compression \a
// level in the context of \a call.
grpc_compression_algorithm grpc_call_compression_for_level(
    grpc_call* call, grpc_compression_level level);

// Did this client call receive a trailers-only response
// TODO(markdroth): This is currently available only to the C++ API.
//                  Move to surface API if requested by other languages.
bool grpc_call_is_trailers_only(const grpc_call* call);

// Returns the authority for the call, as seen on the server side.
absl::string_view grpc_call_server_authority(const grpc_call* call);

#endif  // GRPC_SRC_CORE_LIB_SURFACE_CALL_H
