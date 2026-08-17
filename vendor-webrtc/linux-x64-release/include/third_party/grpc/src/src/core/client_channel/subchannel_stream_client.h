//
// Copyright 2018 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_STREAM_CLIENT_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_STREAM_CLIENT_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>

#include <atomic>
#include <memory>
#include <optional>

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/strings/string_view.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/client_channel/subchannel.h"
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/slice/slice.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/backoff.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/sync.h"

namespace grpc_core {

// Represents a streaming call on a subchannel that should be maintained
// open at all times.
// If the call fails with UNIMPLEMENTED, no further attempts are made.
// If the call fails with any other status (including OK), we retry the
// call with appropriate backoff.
// The backoff state is reset when we receive a message on a stream.
//
// Currently, this assumes server-side streaming, but it could be extended
// to support full bidi streaming if there is a need in the future.
class SubchannelStreamClient final
    : public InternallyRefCounted<SubchannelStreamClient> {
 public:
  // Interface implemented by caller.  Thread safety is provided for the
  // implementation; only one method will be called by any thread at any
  // one time (including destruction).
  //
  // The address of the SubchannelStreamClient object is passed to most
  // methods for logging purposes.
  class CallEventHandler {
   public:
    virtual ~CallEventHandler() = default;

    // Returns the path for the streaming call.
    virtual Slice GetPathLocked()
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
    // Called when a new call attempt is being started.
    virtual void OnCallStartLocked(SubchannelStreamClient* client)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
    // Called when a previous call attempt has failed and the retry
    // timer is started before the next attempt.
    virtual void OnRetryTimerStartLocked(SubchannelStreamClient* client)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
    // Returns the message payload to send from the client.
    virtual grpc_slice EncodeSendMessageLocked()
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
    // Called whenever a message is received from the server.
    virtual absl::Status RecvMessageReadyLocked(
        SubchannelStreamClient* client, absl::string_view serialized_message)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
    // Called when a stream fails.
    virtual void RecvTrailingMetadataReadyLocked(SubchannelStreamClient* client,
                                                 grpc_status_code status)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_) = 0;
  };

  // If tracer is non-null, it enables trace logging, with the specified
  // string being the first part of the log message.
  // Does not take ownership of interested_parties; the caller is responsible
  // for ensuring that it will outlive the SubchannelStreamClient.
  SubchannelStreamClient(
      RefCountedPtr<ConnectedSubchannel> connected_subchannel,
      grpc_pollset_set* interested_parties,
      std::unique_ptr<CallEventHandler> event_handler, const char* tracer);

  ~SubchannelStreamClient() override;

  void Orphan() override;

 private:
  // Contains a call to the backend and all the data related to the call.
  class CallState final : public Orphanable {
   public:
    CallState(RefCountedPtr<SubchannelStreamClient> client,
              grpc_pollset_set* interested_parties);
    ~CallState() override;

    void Orphan() override;

    void StartCallLocked()
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&SubchannelStreamClient::mu_);

   private:
    void Cancel();

    void StartBatch(grpc_transport_stream_op_batch* batch);
    static void StartBatchInCallCombiner(void* arg, grpc_error_handle error);

    void CallEndedLocked(bool retry)
        ABSL_EXCLUSIVE_LOCKS_REQUIRED(&subchannel_stream_client_->mu_);

    void RecvMessageReady();

    static void OnComplete(void* arg, grpc_error_handle error);
    static void RecvInitialMetadataReady(void* arg, grpc_error_handle error);
    static void RecvMessageReady(void* arg, grpc_error_handle error);
    static void RecvTrailingMetadataReady(void* arg, grpc_error_handle error);
    static void StartCancel(void* arg, grpc_error_handle error);
    static void OnCancelComplete(void* arg, grpc_error_handle error);

    static void AfterCallStackDestruction(void* arg, grpc_error_handle error);

    RefCountedPtr<SubchannelStreamClient> subchannel_stream_client_;
    grpc_polling_entity pollent_;

    RefCountedPtr<Arena> arena_;
    CallCombiner call_combiner_;

    // The streaming call to the backend. Always non-null.
    // Refs are tracked manually; when the last ref is released, the
    // CallState object will be automatically destroyed.
    SubchannelCall* call_;

    grpc_transport_stream_op_batch_payload payload_;
    grpc_transport_stream_op_batch batch_;
    grpc_transport_stream_op_batch recv_message_batch_;
    grpc_transport_stream_op_batch recv_trailing_metadata_batch_;

    grpc_closure on_complete_;

    // send_initial_metadata
    grpc_metadata_batch send_initial_metadata_;

    // send_message
    SliceBuffer send_message_;

    // send_trailing_metadata
    grpc_metadata_batch send_trailing_metadata_;

    // recv_initial_metadata
    grpc_metadata_batch recv_initial_metadata_;
    grpc_closure recv_initial_metadata_ready_;

    // recv_message
    std::optional<SliceBuffer> recv_message_;
    grpc_closure recv_message_ready_;
    std::atomic<bool> seen_response_{false};

    // True if the cancel_stream batch has been started.
    std::atomic<bool> cancelled_{false};

    // recv_trailing_metadata
    grpc_metadata_batch recv_trailing_metadata_;
    grpc_transport_stream_stats collect_stats_;
    grpc_closure recv_trailing_metadata_ready_;

    // Closure for call stack destruction.
    grpc_closure after_call_stack_destruction_;
  };

  void StartCall();
  void StartCallLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);

  void StartRetryTimerLocked() ABSL_EXCLUSIVE_LOCKS_REQUIRED(&mu_);
  void OnRetryTimer() ABSL_LOCKS_EXCLUDED(mu_);

  RefCountedPtr<ConnectedSubchannel> connected_subchannel_;
  grpc_pollset_set* interested_parties_;  // Do not own.
  const char* tracer_;
  RefCountedPtr<CallArenaAllocator> call_allocator_;

  Mutex mu_;
  std::unique_ptr<CallEventHandler> event_handler_ ABSL_GUARDED_BY(mu_);

  // The data associated with the current call.  It holds a ref
  // to this SubchannelStreamClient object.
  OrphanablePtr<CallState> call_state_ ABSL_GUARDED_BY(mu_);

  // Call retry state.
  BackOff retry_backoff_ ABSL_GUARDED_BY(mu_);
  std::optional<grpc_event_engine::experimental::EventEngine::TaskHandle>
      retry_timer_handle_ ABSL_GUARDED_BY(mu_);
  // A raw pointer will suffice since connected_subchannel_ holds a copy of the
  // ChannelArgs which holds an std::shared_ptr of the EventEngine.
  grpc_event_engine::experimental::EventEngine* event_engine_
      ABSL_GUARDED_BY(mu_);
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_SUBCHANNEL_STREAM_CLIENT_H
