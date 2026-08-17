/*
 * Copyright (C) 2017 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACING_CORE_SHARED_MEMORY_ARBITER_IMPL_H_
#define SRC_TRACING_CORE_SHARED_MEMORY_ARBITER_IMPL_H_

#include <stdint.h>

#include <functional>
#include <map>
#include <memory>
#include <mutex>
#include <vector>

#include "perfetto/ext/base/weak_ptr.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "perfetto/ext/tracing/core/shared_memory_abi.h"
#include "perfetto/ext/tracing/core/shared_memory_arbiter.h"
#include "perfetto/tracing/core/forward_decls.h"
#include "src/tracing/core/id_allocator.h"

namespace perfetto {

class PatchList;
class Patch;
class TraceWriter;
class TraceWriterImpl;

namespace base {
class TaskRunner;
}  // namespace base

// This class handles the shared memory buffer on the producer side. It is used
// to obtain thread-local chunks and to partition pages from several threads.
// There is one arbiter instance per Producer.
// This class is thread-safe and uses locks to do so. Data sources are supposed
// to interact with this sporadically, only when they run out of space on their
// current thread-local chunk.
//
// The arbiter can become "unbound" as a consequence of:
//  (a) being created without an endpoint
//  (b) CreateStartupTraceWriter calls after creation (whether created with or
//      without endpoint).
//
// Entering the unbound state is only supported if all trace writers are created
// in kDrop mode. In the unbound state, the arbiter buffers commit messages
// until all trace writers are bound to a target buffer.
//
// The following state transitions are possible:
//
//   CreateInstance()
//    |
//    |  CreateUnboundInstance()
//    |    |
//    |    |
//    |    V
//    |  [ !fully_bound_, !endpoint_, 0 unbound buffer reservations ]
//    |      |     |
//    |      |     | CreateStartupTraceWriter(buf)
//    |      |     |  buffer reservations += buf
//    |      |     |
//    |      |     |             ----
//    |      |     |            |    | CreateStartupTraceWriter(buf)
//    |      |     |            |    |  buffer reservations += buf
//    |      |     V            |    V
//    |      |   [ !fully_bound_, !endpoint_, >=1 unbound buffer reservations ]
//    |      |                                                |
//    |      |                       BindToProducerEndpoint() |
//    |      |                                                |
//    |      | BindToProducerEndpoint()                       |
//    |      |                                                V
//    |      |   [ !fully_bound_, endpoint_, >=1 unbound buffer reservations ]
//    |      |   A    |    A                               |     A
//    |      |   |    |    |                               |     |
//    |      |   |     ----                                |     |
//    |      |   |    CreateStartupTraceWriter(buf)        |     |
//    |      |   |     buffer reservations += buf          |     |
//    |      |   |                                         |     |
//    |      |   | CreateStartupTraceWriter(buf)           |     |
//    |      |   |  where buf is not yet bound             |     |
//    |      |   |  buffer reservations += buf             |     | (yes)
//    |      |   |                                         |     |
//    |      |   |        BindStartupTargetBuffer(buf, id) |-----
//    |      |   |           buffer reservations -= buf    | reservations > 0?
//    |      |   |                                         |
//    |      |   |                                         | (no)
//    |      V   |                                         V
//     --> [ fully_bound_, endpoint_, 0 unbound buffer reservations ]
//              |    A
//              |    | CreateStartupTraceWriter(buf)
//              |    |  where buf is already bound
//               ----
class SharedMemoryArbiterImpl : public SharedMemoryArbiter {
 public:
  // See SharedMemoryArbiter::CreateInstance(). |start|, |size| define the
  // boundaries of the shared memory buffer. ProducerEndpoint and TaskRunner may
  // be |nullptr| if created unbound, see
  // SharedMemoryArbiter::CreateUnboundInstance().

  // SharedMemoryArbiterImpl(void* start,
  //                         size_t size,
  //                         size_t page_size,
  //                         TracingService::ProducerEndpoint*
  //                         producer_endpoint, base::TaskRunner* task_runner) :
  //   SharedMemoryArbiterImpl(start, size, page_size, false, producer_endpoint,
  //   task_runner) {
  // }

  SharedMemoryArbiterImpl(void* start,
                          size_t size,
                          ShmemMode mode,
                          size_t page_size,
                          TracingService::ProducerEndpoint*,
                          base::TaskRunner*);

  // Returns a new Chunk to write tracing data. Depending on the provided
  // BufferExhaustedPolicy, this may return an invalid chunk if no valid free
  // chunk could be found in the SMB.
  SharedMemoryABI::Chunk GetNewChunk(const SharedMemoryABI::ChunkHeader&,
                                     BufferExhaustedPolicy);

  // Puts back a Chunk that has been completed and sends a request to the
  // service to move it to the central tracing buffer. |target_buffer| is the
  // absolute trace buffer ID where the service should move the chunk onto (the
  // producer is just to copy back the same number received in the
  // DataSourceConfig upon the StartDataSource() request).
  // PatchList is a pointer to the list of patches for previous chunks. The
  // first patched entries will be removed from the patched list and sent over
  // to the service in the same CommitData() IPC request.
  void ReturnCompletedChunk(SharedMemoryABI::Chunk,
                            MaybeUnboundBufferID target_buffer,
                            PatchList*);

  // Send a request to the service to apply completed patches from |patch_list|.
  // |writer_id| is the ID of the TraceWriter that calls this method,
  // |target_buffer| is the global trace buffer ID of its target buffer.
  void SendPatches(WriterID writer_id,
                   MaybeUnboundBufferID target_buffer,
                   PatchList* patch_list);

  SharedMemoryABI* shmem_abi_for_testing() { return &shmem_abi_; }

  static void set_default_layout_for_testing(SharedMemoryABI::PageLayout l) {
    default_page_layout = l;
  }

  static SharedMemoryABI::PageLayout default_page_layout_for_testing() {
    return default_page_layout;
  }

  // SharedMemoryArbiter implementation.
  // See include/perfetto/tracing/core/shared_memory_arbiter.h for comments.
  std::unique_ptr<TraceWriter> CreateTraceWriter(
      BufferID target_buffer,
      BufferExhaustedPolicy) override;
  std::unique_ptr<TraceWriter> CreateStartupTraceWriter(
      uint16_t target_buffer_reservation_id) override;
  void BindToProducerEndpoint(TracingService::ProducerEndpoint*,
                              base::TaskRunner*) override;
  void BindStartupTargetBuffer(uint16_t target_buffer_reservation_id,
                               BufferID target_buffer_id) override;
  void AbortStartupTracingForReservation(
      uint16_t target_buffer_reservation_id) override;
  void NotifyFlushComplete(FlushRequestID) override;

  void SetBatchCommitsDuration(uint32_t batch_commits_duration_ms) override;

  bool EnableDirectSMBPatching() override;

  void SetDirectSMBPatchingSupportedByService() override;

  void FlushPendingCommitDataRequests(
      std::function<void()> callback = {}) override;
  bool TryShutdown() override;

  base::TaskRunner* task_runner() const { return task_runner_; }
  size_t page_size() const { return shmem_abi_.page_size(); }
  size_t num_pages() const { return shmem_abi_.num_pages(); }

  base::WeakPtr<SharedMemoryArbiterImpl> GetWeakPtr() const {
    return weak_ptr_factory_.GetWeakPtr();
  }

 private:
  friend class TraceWriterImpl;
  friend class StartupTraceWriterTest;
  friend class SharedMemoryArbiterImplTest;

  struct TargetBufferReservation {
    bool resolved = false;
    BufferID target_buffer = kInvalidBufferId;
  };

  // Placeholder for the actual target buffer ID of a startup target buffer
  // reservation ID in |target_buffer_reservations_|.
  static constexpr BufferID kInvalidBufferId = 0;

  static SharedMemoryABI::PageLayout default_page_layout;

  SharedMemoryArbiterImpl(const SharedMemoryArbiterImpl&) = delete;
  SharedMemoryArbiterImpl& operator=(const SharedMemoryArbiterImpl&) = delete;

  void UpdateCommitDataRequest(SharedMemoryABI::Chunk chunk,
                               WriterID writer_id,
                               MaybeUnboundBufferID target_buffer,
                               PatchList* patch_list);

  // Search the chunks that are being batched in |commit_data_req_| for a chunk
  // that needs patching and that matches the provided |writer_id| and
  // |patch.chunk_id|. If found, apply |patch| to that chunk, and if
  // |chunk_needs_more_patching| is true, clear the needs patching flag of the
  // chunk and mark it as complete - to allow the service to read it (and other
  // chunks after it) during scraping. Returns true if the patch was applied,
  // false otherwise.
  //
  // Note: the caller must be holding |lock_| for the duration of the call.
  bool TryDirectPatchLocked(WriterID writer_id,
                            const Patch& patch,
                            bool chunk_needs_more_patching);
  std::unique_ptr<TraceWriter> CreateTraceWriterInternal(
      MaybeUnboundBufferID target_buffer,
      BufferExhaustedPolicy);

  // Called by the TraceWriter destructor.
  void ReleaseWriterID(WriterID);

  void BindStartupTargetBufferImpl(std::unique_lock<std::mutex> scoped_lock,
                                   uint16_t target_buffer_reservation_id,
                                   BufferID target_buffer_id);

  // Returns some statistics about chunks/pages in the shared memory buffer.
  struct Stats {
    size_t chunks_free = 0;
    size_t chunks_being_written = 0;
    size_t chunks_being_read = 0;
    size_t chunks_complete = 0;

    // No chunks are included from free/malformed pages.
    size_t pages_free = 0;
    size_t pages_unexpected = 0;
  };
  Stats GetStats();

  // If any flush callbacks were queued up while the arbiter or any target
  // buffer reservation was unbound, this wraps the pending callbacks into a new
  // std::function and returns it. Otherwise returns an invalid std::function.
  std::function<void()> TakePendingFlushCallbacksLocked();

  // Replace occurrences of target buffer reservation IDs in |commit_data_req_|
  // with their respective actual BufferIDs if they were already bound. Returns
  // true iff all occurrences were replaced.
  bool ReplaceCommitPlaceholderBufferIdsLocked();

  // Update and return |fully_bound_| based on the arbiter's |pending_writers_|
  // state.
  bool UpdateFullyBoundLocked();

  // Only accessed on |task_runner_| after the producer endpoint was bound.
  TracingService::ProducerEndpoint* producer_endpoint_ = nullptr;

  // Set to true when this instance runs in a emulation mode for a producer
  // endpoint that doesn't support shared memory (e.g. vsock).
  const bool use_shmem_emulation_ = false;

  // --- Begin lock-protected members ---

  std::mutex lock_;

  base::TaskRunner* task_runner_ = nullptr;
  SharedMemoryABI shmem_abi_;
  size_t page_idx_ = 0;
  std::unique_ptr<CommitDataRequest> commit_data_req_;
  size_t bytes_pending_commit_ = 0;  // SUM(chunk.size() : commit_data_req_).
  IdAllocator<WriterID> active_writer_ids_;
  bool did_shutdown_ = false;

  // Whether the arbiter itself and all startup target buffer reservations are
  // bound. Note that this can become false again later if a new target buffer
  // reservation is created by calling CreateStartupTraceWriter() with a new
  // reservation id.
  bool fully_bound_;

  // Whether the arbiter was always bound. If false, the arbiter was unbound at
  // one point in time.
  bool was_always_bound_;

  // Whether all created trace writers were created with kDrop policy.
  bool all_writers_have_drop_policy_ = true;

  // IDs of writers and their assigned target buffers that should be registered
  // with the service after the arbiter and/or their startup target buffer is
  // bound.
  std::map<WriterID, MaybeUnboundBufferID> pending_writers_;

  // Callbacks for flush requests issued while the arbiter or a target buffer
  // reservation was unbound.
  std::vector<std::function<void()>> pending_flush_callbacks_;

  // See SharedMemoryArbiter::SetBatchCommitsDuration.
  uint32_t batch_commits_duration_ms_ = 0;

  // See SharedMemoryArbiter::EnableDirectSMBPatching.
  bool direct_patching_enabled_ = false;

  // See SharedMemoryArbiter::SetDirectSMBPatchingSupportedByService.
  bool direct_patching_supported_by_service_ = false;

  // Indicates whether we have already scheduled a delayed flush for the
  // purposes of batching. Set to true at the beginning of a batching period and
  // cleared at the end of the period. Immediate flushes that happen during a
  // batching period will empty the |commit_data_req| (triggering an immediate
  // IPC to the service), but will not clear this flag and the
  // previously-scheduled delayed flush will still occur at the end of the
  // batching period.
  bool delayed_flush_scheduled_ = false;

  // Stores target buffer reservations for writers created via
  // CreateStartupTraceWriter(). A bound reservation sets
  // TargetBufferReservation::resolved to true and is associated with the actual
  // BufferID supplied in BindStartupTargetBuffer().
  //
  // TODO(eseckler): Clean up entries from this map. This would probably require
  // a method in SharedMemoryArbiter that allows a producer to invalidate a
  // reservation ID.
  std::map<MaybeUnboundBufferID, TargetBufferReservation>
      target_buffer_reservations_;

  // --- End lock-protected members ---

  // Keep at the end.
  base::WeakPtrFactory<SharedMemoryArbiterImpl> weak_ptr_factory_;
};

}  // namespace perfetto

#endif  // SRC_TRACING_CORE_SHARED_MEMORY_ARBITER_IMPL_H_
