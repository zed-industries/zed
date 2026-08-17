// Copyright 2023 gRPC authors.
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

#ifndef GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_FILTER_LEGACY_CALL_DATA_H
#define GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_FILTER_LEGACY_CALL_DATA_H

#include <grpc/event_engine/event_engine.h>
#include <grpc/slice.h>
#include <grpc/status.h>
#include <grpc/support/port_platform.h>
#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <utility>

#include "absl/container/inlined_vector.h"
#include "absl/functional/any_invocable.h"
#include "src/core/call/metadata_batch.h"
#include "src/core/client_channel/client_channel_filter.h"
#include "src/core/client_channel/retry_filter.h"
#include "src/core/client_channel/retry_service_config.h"
#include "src/core/client_channel/retry_throttle.h"
#include "src/core/lib/channel/channel_fwd.h"
#include "src/core/lib/channel/channel_stack.h"
#include "src/core/lib/iomgr/call_combiner.h"
#include "src/core/lib/iomgr/closure.h"
#include "src/core/lib/iomgr/error.h"
#include "src/core/lib/iomgr/polling_entity.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/lib/transport/transport.h"
#include "src/core/util/backoff.h"
#include "src/core/util/debug_location.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/ref_counted_ptr.h"
#include "src/core/util/time.h"

namespace grpc_core {

class RetryFilter::LegacyCallData final {
 public:
  static grpc_error_handle Init(grpc_call_element* elem,
                                const grpc_call_element_args* args);
  static void Destroy(grpc_call_element* elem,
                      const grpc_call_final_info* /*final_info*/,
                      grpc_closure* then_schedule_closure);
  static void StartTransportStreamOpBatch(
      grpc_call_element* elem, grpc_transport_stream_op_batch* batch);
  static void SetPollent(grpc_call_element* elem, grpc_polling_entity* pollent);

 private:
  class CallStackDestructionBarrier;

  // Pending batches stored in call data.
  struct PendingBatch {
    // The pending batch.  If nullptr, this slot is empty.
    grpc_transport_stream_op_batch* batch = nullptr;
    // Indicates whether payload for send ops has been cached in
    // LegacyCallData.
    bool send_ops_cached = false;
  };

  // State associated with each call attempt.
  class CallAttempt final : public RefCounted<CallAttempt> {
   public:
    CallAttempt(LegacyCallData* calld, bool is_transparent_retry);
    ~CallAttempt() override;

    bool lb_call_committed() const { return lb_call_committed_; }

    // Constructs and starts whatever batches are needed on this call
    // attempt.
    void StartRetriableBatches();

    // Frees cached send ops that have already been completed after
    // committing the call.
    void FreeCachedSendOpDataAfterCommit();

    // Cancels the call attempt.
    void CancelFromSurface(grpc_transport_stream_op_batch* cancel_batch);

   private:
    // State used for starting a retryable batch on the call attempt's LB call.
    // This provides its own grpc_transport_stream_op_batch and other data
    // structures needed to populate the ops in the batch.
    // We allocate one struct on the arena for each attempt at starting a
    // batch on a given LB call.
    class BatchData final
        : public RefCounted<BatchData, PolymorphicRefCount, UnrefCallDtor> {
     public:
      BatchData(RefCountedPtr<CallAttempt> call_attempt, int refcount,
                bool set_on_complete);
      ~BatchData() override;

      grpc_transport_stream_op_batch* batch() { return &batch_; }

      // Adds retriable send_initial_metadata op.
      void AddRetriableSendInitialMetadataOp();
      // Adds retriable send_message op.
      void AddRetriableSendMessageOp();
      // Adds retriable send_trailing_metadata op.
      void AddRetriableSendTrailingMetadataOp();
      // Adds retriable recv_initial_metadata op.
      void AddRetriableRecvInitialMetadataOp();
      // Adds retriable recv_message op.
      void AddRetriableRecvMessageOp();
      // Adds retriable recv_trailing_metadata op.
      void AddRetriableRecvTrailingMetadataOp();
      // Adds cancel_stream op.
      void AddCancelStreamOp(grpc_error_handle error);

     private:
      // Frees cached send ops that were completed by the completed batch in
      // batch_data.  Used when batches are completed after the call is
      // committed.
      void FreeCachedSendOpDataForCompletedBatch();

      // If there is a pending recv_initial_metadata op, adds a closure
      // to closures for recv_initial_metadata_ready.
      void MaybeAddClosureForRecvInitialMetadataCallback(
          grpc_error_handle error, CallCombinerClosureList* closures);
      // Intercepts recv_initial_metadata_ready callback for retries.
      // Commits the call and returns the initial metadata up the stack.
      static void RecvInitialMetadataReady(void* arg, grpc_error_handle error);

      // If there is a pending recv_message op, adds a closure to closures
      // for recv_message_ready.
      void MaybeAddClosureForRecvMessageCallback(
          grpc_error_handle error, CallCombinerClosureList* closures);
      // Intercepts recv_message_ready callback for retries.
      // Commits the call and returns the message up the stack.
      static void RecvMessageReady(void* arg, grpc_error_handle error);

      // If there is a pending recv_trailing_metadata op, adds a closure to
      // closures for recv_trailing_metadata_ready.
      void MaybeAddClosureForRecvTrailingMetadataReady(
          grpc_error_handle error, CallCombinerClosureList* closures);
      // Adds any necessary closures for deferred batch completion
      // callbacks to closures.
      void AddClosuresForDeferredCompletionCallbacks(
          CallCombinerClosureList* closures);
      // For any pending batch containing an op that has not yet been started,
      // adds the pending batch's completion closures to closures.
      void AddClosuresToFailUnstartedPendingBatches(
          grpc_error_handle error, CallCombinerClosureList* closures);
      // Runs necessary closures upon completion of a call attempt.
      void RunClosuresForCompletedCall(grpc_error_handle error);
      // Intercepts recv_trailing_metadata_ready callback for retries.
      // Commits the call and returns the trailing metadata up the stack.
      static void RecvTrailingMetadataReady(void* arg, grpc_error_handle error);

      // Adds the on_complete closure for the pending batch completed in
      // batch_data to closures.
      void AddClosuresForCompletedPendingBatch(
          grpc_error_handle error, CallCombinerClosureList* closures);

      // If there are any cached ops to replay or pending ops to start on the
      // LB call, adds them to closures.
      void AddClosuresForReplayOrPendingSendOps(
          CallCombinerClosureList* closures);

      // Callback used to intercept on_complete from LB calls.
      static void OnComplete(void* arg, grpc_error_handle error);

      // Callback used to handle on_complete for internally generated
      // cancel_stream op.
      static void OnCompleteForCancelOp(void* arg, grpc_error_handle error);

      // This DOES hold a ref, but it cannot be a RefCountedPtr<>, because
      // our dtor unrefs the owning call, which may delete the arena in
      // which we are allocated, which means that running the dtor of any
      // data members after that would cause a crash.
      CallAttempt* call_attempt_;
      // The batch to use in the LB call.
      // Its payload field points to CallAttempt::batch_payload_.
      grpc_transport_stream_op_batch batch_;
      // For intercepting on_complete.
      grpc_closure on_complete_;
    };

    // Creates a BatchData object on the call's arena with the
    // specified refcount.  If set_on_complete is true, the batch's
    // on_complete callback will be set to point to on_complete();
    // otherwise, the batch's on_complete callback will be null.
    BatchData* CreateBatch(int refcount, bool set_on_complete) {
      return calld_->arena_->New<BatchData>(Ref(DEBUG_LOCATION, "CreateBatch"),
                                            refcount, set_on_complete);
    }

    // If there are any cached send ops that need to be replayed on this
    // call attempt, creates and returns a new batch to replay those ops.
    // Otherwise, returns nullptr.
    BatchData* MaybeCreateBatchForReplay();

    // Adds a closure to closures that will execute batch in the call combiner.
    void AddClosureForBatch(grpc_transport_stream_op_batch* batch,
                            const char* reason,
                            CallCombinerClosureList* closures);

    // Helper function used to start a recv_trailing_metadata batch.  This
    // is used in the case where a recv_initial_metadata or recv_message
    // op fails in a way that we know the call is over but when the application
    // has not yet started its own recv_trailing_metadata op.
    void AddBatchForInternalRecvTrailingMetadata(
        CallCombinerClosureList* closures);

    // Adds a batch to closures to cancel this call attempt, if
    // cancellation has not already been sent on the LB call.
    void MaybeAddBatchForCancelOp(grpc_error_handle error,
                                  CallCombinerClosureList* closures);

    // Adds batches for pending batches to closures.
    void AddBatchesForPendingBatches(CallCombinerClosureList* closures);

    // Adds whatever batches are needed on this attempt to closures.
    void AddRetriableBatches(CallCombinerClosureList* closures);

    // Returns true if any send op in the batch was not yet started on this
    // attempt.
    bool PendingBatchContainsUnstartedSendOps(PendingBatch* pending);

    // Returns true if there are cached send ops to replay.
    bool HaveSendOpsToReplay();

    // If our retry state is no longer needed, switch to fast path by moving
    // our LB call into calld_->committed_call_ and having calld_ drop
    // its ref to us.
    void MaybeSwitchToFastPath();

    // Returns true if the call should be retried.
    bool ShouldRetry(std::optional<grpc_status_code> status,
                     std::optional<Duration> server_pushback_ms);

    // Abandons the call attempt.  Unrefs any deferred batches.
    void Abandon();

    void OnPerAttemptRecvTimer();
    static void OnPerAttemptRecvTimerLocked(void* arg, grpc_error_handle error);
    void MaybeCancelPerAttemptRecvTimer();

    LegacyCallData* calld_;
    OrphanablePtr<ClientChannelFilter::FilterBasedLoadBalancedCall> lb_call_;
    bool lb_call_committed_ = false;

    grpc_closure on_per_attempt_recv_timer_;
    grpc_event_engine::experimental::EventEngine::TaskHandle
        per_attempt_recv_timer_handle_ =
            grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;

    // BatchData.batch.payload points to this.
    grpc_transport_stream_op_batch_payload batch_payload_;
    // For send_initial_metadata.
    grpc_metadata_batch send_initial_metadata_;
    // For send_message.
    SliceBuffer send_message_;
    // For send_trailing_metadata.
    grpc_metadata_batch send_trailing_metadata_;
    // For intercepting recv_initial_metadata.
    grpc_metadata_batch recv_initial_metadata_;
    grpc_closure recv_initial_metadata_ready_;
    bool trailing_metadata_available_ = false;
    // For intercepting recv_message.
    grpc_closure recv_message_ready_;
    std::optional<SliceBuffer> recv_message_;
    uint32_t recv_message_flags_;
    // For intercepting recv_trailing_metadata.
    grpc_metadata_batch recv_trailing_metadata_;
    grpc_transport_stream_stats collect_stats_;
    grpc_closure recv_trailing_metadata_ready_;
    // These fields indicate which ops have been started and completed on
    // this call attempt.
    size_t started_send_message_count_ = 0;
    size_t completed_send_message_count_ = 0;
    size_t started_recv_message_count_ = 0;
    size_t completed_recv_message_count_ = 0;
    bool started_send_initial_metadata_ : 1;
    bool completed_send_initial_metadata_ : 1;
    bool started_send_trailing_metadata_ : 1;
    bool completed_send_trailing_metadata_ : 1;
    bool started_recv_initial_metadata_ : 1;
    bool completed_recv_initial_metadata_ : 1;
    bool started_recv_trailing_metadata_ : 1;
    bool completed_recv_trailing_metadata_ : 1;
    bool sent_cancel_stream_ : 1;
    // State for callback processing.
    RefCountedPtr<BatchData> recv_initial_metadata_ready_deferred_batch_;
    grpc_error_handle recv_initial_metadata_error_;
    RefCountedPtr<BatchData> recv_message_ready_deferred_batch_;
    grpc_error_handle recv_message_error_;
    struct OnCompleteDeferredBatch {
      OnCompleteDeferredBatch(RefCountedPtr<BatchData> batch,
                              grpc_error_handle error)
          : batch(std::move(batch)), error(error) {}
      RefCountedPtr<BatchData> batch;
      grpc_error_handle error;
    };
    // There cannot be more than 3 pending send op batches at a time.
    absl::InlinedVector<OnCompleteDeferredBatch, 3>
        on_complete_deferred_batches_;
    RefCountedPtr<BatchData> recv_trailing_metadata_internal_batch_;
    grpc_error_handle recv_trailing_metadata_error_;
    bool seen_recv_trailing_metadata_from_surface_ : 1;
    // NOTE: Do not move this next to the metadata bitfields above. That would
    //       save space but will also result in a data race because compiler
    //       will generate a 2 byte store which overwrites the meta-data
    //       fields upon setting this field.
    bool abandoned_ : 1;
  };

  LegacyCallData(RetryFilter* chand, const grpc_call_element_args& args);
  ~LegacyCallData();

  void StartTransportStreamOpBatch(grpc_transport_stream_op_batch* batch);

  // Returns the index into pending_batches_ to be used for batch.
  static size_t GetBatchIndex(grpc_transport_stream_op_batch* batch);
  PendingBatch* PendingBatchesAdd(grpc_transport_stream_op_batch* batch);
  void PendingBatchClear(PendingBatch* pending);
  void MaybeClearPendingBatch(PendingBatch* pending);
  static void FailPendingBatchInCallCombiner(void* arg,
                                             grpc_error_handle error);
  // Fails all pending batches.  Does NOT yield call combiner.
  void PendingBatchesFail(grpc_error_handle error);
  // Returns a pointer to the first pending batch for which predicate(batch)
  // returns true, or null if not found.
  template <typename Predicate>
  PendingBatch* PendingBatchFind(const char* log_message, Predicate predicate);

  // Caches data for send ops so that it can be retried later, if not
  // already cached.
  void MaybeCacheSendOpsForBatch(PendingBatch* pending);
  void FreeCachedSendInitialMetadata();
  // Frees cached send_message at index idx.
  void FreeCachedSendMessage(size_t idx);
  void FreeCachedSendTrailingMetadata();
  void FreeAllCachedSendOpData();

  // Commits the call so that no further retry attempts will be performed.
  void RetryCommit(CallAttempt* call_attempt);

  // Starts a timer to retry after appropriate back-off.
  // If server_pushback is nullopt, retry_backoff_ is used.
  void StartRetryTimer(std::optional<Duration> server_pushback);

  void OnRetryTimer();
  static void OnRetryTimerLocked(void* arg, grpc_error_handle /*error*/);

  // Adds a closure to closures to start a transparent retry.
  void AddClosureToStartTransparentRetry(CallCombinerClosureList* closures);
  static void StartTransparentRetry(void* arg, grpc_error_handle error);

  OrphanablePtr<ClientChannelFilter::FilterBasedLoadBalancedCall>
  CreateLoadBalancedCall(absl::AnyInvocable<void()> on_commit,
                         bool is_transparent_retry);

  void CreateCallAttempt(bool is_transparent_retry);

  RetryFilter* chand_;
  grpc_polling_entity* pollent_;
  RefCountedPtr<internal::ServerRetryThrottleData> retry_throttle_data_;
  const internal::RetryMethodConfig* retry_policy_ = nullptr;
  BackOff retry_backoff_;

  Timestamp deadline_;
  Arena* arena_;
  grpc_call_stack* owning_call_;
  CallCombiner* call_combiner_;

  grpc_error_handle cancelled_from_surface_;

  RefCountedPtr<CallStackDestructionBarrier> call_stack_destruction_barrier_;

  // TODO(roth): As part of implementing hedging, we will need to maintain a
  // list of all pending attempts, so that we can cancel them all if the call
  // gets cancelled.
  RefCountedPtr<CallAttempt> call_attempt_;

  // LB call used when we've committed to a call attempt and the retry
  // state for that attempt is no longer needed.  This provides a fast
  // path for long-running streaming calls that minimizes overhead.
  OrphanablePtr<ClientChannelFilter::FilterBasedLoadBalancedCall>
      committed_call_;

  // When are are not yet fully committed to a particular call (i.e.,
  // either we might still retry or we have committed to the call but
  // there are still some cached ops to be replayed on the call),
  // batches received from above will be added to this list, and they
  // will not be removed until we have invoked their completion callbacks.
  size_t bytes_buffered_for_retry_ = 0;
  PendingBatch pending_batches_[MAX_PENDING_BATCHES];
  bool pending_send_initial_metadata_ : 1;
  bool pending_send_message_ : 1;
  bool pending_send_trailing_metadata_ : 1;

  // Retry state.
  bool retry_committed_ : 1;
  bool retry_codepath_started_ : 1;
  bool sent_transparent_retry_not_seen_by_server_ : 1;
  int num_attempts_completed_ = 0;
  grpc_event_engine::experimental::EventEngine::TaskHandle retry_timer_handle_ =
      grpc_event_engine::experimental::EventEngine::TaskHandle::kInvalid;
  grpc_closure retry_closure_;

  // Cached data for retrying send ops.
  // send_initial_metadata
  bool seen_send_initial_metadata_ = false;
  grpc_metadata_batch send_initial_metadata_;
  // send_message
  // When we get a send_message op, we replace the original byte stream
  // with a CachingByteStream that caches the slices to a local buffer for
  // use in retries.
  // Note: We inline the cache for the first 3 send_message ops and use
  // dynamic allocation after that.  This number was essentially picked
  // at random; it could be changed in the future to tune performance.
  struct CachedSendMessage {
    SliceBuffer* slices;
    uint32_t flags;
  };
  absl::InlinedVector<CachedSendMessage, 3> send_messages_;
  // send_trailing_metadata
  bool seen_send_trailing_metadata_ = false;
  grpc_metadata_batch send_trailing_metadata_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CLIENT_CHANNEL_RETRY_FILTER_LEGACY_CALL_DATA_H
