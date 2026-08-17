// Copyright 2022 gRPC Authors
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

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENDPOINT_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENDPOINT_H

#include <grpc/support/port_platform.h>

// IWYU pragma: no_include <bits/types/struct_iovec.h>

#include <grpc/event_engine/event_engine.h>
#include <grpc/event_engine/memory_allocator.h>
#include <grpc/event_engine/slice_buffer.h>
#include <grpc/support/alloc.h>

#include <atomic>
#include <cstdint>
#include <memory>
#include <new>
#include <optional>
#include <utility>

#include "absl/base/thread_annotations.h"
#include "absl/container/flat_hash_map.h"
#include "absl/functional/any_invocable.h"
#include "absl/hash/hash.h"
#include "absl/log/check.h"
#include "absl/log/log.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "src/core/lib/event_engine/extensions/supports_fd.h"
#include "src/core/lib/event_engine/posix.h"
#include "src/core/lib/event_engine/posix_engine/event_poller.h"
#include "src/core/lib/event_engine/posix_engine/posix_engine_closure.h"
#include "src/core/lib/event_engine/posix_engine/tcp_socket_utils.h"
#include "src/core/lib/event_engine/posix_engine/traced_buffer_list.h"
#include "src/core/lib/iomgr/port.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/util/crash.h"
#include "src/core/util/ref_counted.h"
#include "src/core/util/sync.h"

#ifdef GRPC_POSIX_SOCKET_TCP

#include <sys/socket.h>  // IWYU pragma: keep
#include <sys/types.h>   // IWYU pragma: keep

#ifdef GRPC_MSG_IOVLEN_TYPE
typedef GRPC_MSG_IOVLEN_TYPE msg_iovlen_type;
#else
typedef size_t msg_iovlen_type;
#endif

#endif  //  GRPC_POSIX_SOCKET_TCP

namespace grpc_event_engine::experimental {

#ifdef GRPC_POSIX_SOCKET_TCP

class TcpZerocopySendRecord {
 public:
  TcpZerocopySendRecord() { buf_.Clear(); };

  ~TcpZerocopySendRecord() { DebugAssertEmpty(); }

  // TcpZerocopySendRecord contains a slice buffer holding the slices to be
  // sent. Given the slices that we wish to send, and the current offset into
  // the slice buffer (indicating which have already been sent), populate an
  // iovec array that will be used for a zerocopy enabled sendmsg().
  //   unwind_slice_idx - input/output parameter. It indicates the index of last
  //   slice whose contents were partially sent in the previous sendmsg. After
  //   this function returns, it gets updated to to a new offset
  //   depending on the number of bytes which are decided to be sent in the
  //   current sendmsg.
  //   unwind_byte_idx - input/output parameter. It indicates the byte offset
  //   within the last slice whose contents were partially sent in the previous
  //   sendmsg. After this function returns, it gets updated to a new offset
  //   depending on the number of bytes which are decided to be sent in the
  //   current sendmsg.
  //   sending_length - total number of bytes to be sent in the current sendmsg.
  //   iov - An iovec array containing the bytes to be sent in the current
  //   sendmsg.
  //  Returns: the number of entries in the iovec array.
  //
  msg_iovlen_type PopulateIovs(size_t* unwind_slice_idx,
                               size_t* unwind_byte_idx, size_t* sending_length,
                               iovec* iov);

  // A sendmsg() may not be able to send the bytes that we requested at this
  // time, returning EAGAIN (possibly due to backpressure). In this case,
  // unwind the offset into the slice buffer so we retry sending these bytes.
  void UnwindIfThrottled(size_t unwind_slice_idx, size_t unwind_byte_idx) {
    out_offset_.byte_idx = unwind_byte_idx;
    out_offset_.slice_idx = unwind_slice_idx;
  }

  // Update the offset into the slice buffer based on how much we wanted to sent
  // vs. what sendmsg() actually sent (which may be lower, possibly due to
  // backpressure).
  void UpdateOffsetForBytesSent(size_t sending_length, size_t actually_sent);

  // Indicates whether all underlying data has been sent or not.
  bool AllSlicesSent() { return out_offset_.slice_idx == buf_.Count(); }

  // Reset this structure for a new tcp_write() with zerocopy.
  void PrepareForSends(
      grpc_event_engine::experimental::SliceBuffer& slices_to_send) {
    DebugAssertEmpty();
    out_offset_.slice_idx = 0;
    out_offset_.byte_idx = 0;
    buf_.Swap(slices_to_send);
    Ref();
  }

  // References: 1 reference per sendmsg(), and 1 for the tcp_write().
  void Ref() { ref_.fetch_add(1, std::memory_order_relaxed); }

  // Unref: called when we get an error queue notification for a sendmsg(), if a
  //  sendmsg() failed or when tcp_write() is done.
  bool Unref() {
    const intptr_t prior = ref_.fetch_sub(1, std::memory_order_acq_rel);
    DCHECK_GT(prior, 0);
    if (prior == 1) {
      AllSendsComplete();
      return true;
    }
    return false;
  }

 private:
  struct OutgoingOffset {
    size_t slice_idx = 0;
    size_t byte_idx = 0;
  };

  void DebugAssertEmpty() {
    DCHECK_EQ(buf_.Count(), 0u);
    DCHECK_EQ(buf_.Length(), 0u);
    DCHECK_EQ(ref_.load(std::memory_order_relaxed), 0);
  }

  // When all sendmsg() calls associated with this tcp_write() have been
  // completed (ie. we have received the notifications for each sequence number
  // for each sendmsg()) and all reference counts have been dropped, drop our
  // reference to the underlying data since we no longer need it.
  void AllSendsComplete() {
    DCHECK_EQ(ref_.load(std::memory_order_relaxed), 0);
    buf_.Clear();
  }

  grpc_event_engine::experimental::SliceBuffer buf_;
  std::atomic<intptr_t> ref_{0};
  OutgoingOffset out_offset_;
};

class TcpZerocopySendCtx {
 public:
  static constexpr int kDefaultMaxSends = 4;
  static constexpr size_t kDefaultSendBytesThreshold = 16 * 1024;  // 16KB

  explicit TcpZerocopySendCtx(
      bool zerocopy_enabled, int max_sends = kDefaultMaxSends,
      size_t send_bytes_threshold = kDefaultSendBytesThreshold)
      : max_sends_(max_sends),
        free_send_records_size_(max_sends),
        threshold_bytes_(send_bytes_threshold) {
    send_records_ = static_cast<TcpZerocopySendRecord*>(
        gpr_malloc(max_sends * sizeof(*send_records_)));
    free_send_records_ = static_cast<TcpZerocopySendRecord**>(
        gpr_malloc(max_sends * sizeof(*free_send_records_)));
    if (send_records_ == nullptr || free_send_records_ == nullptr) {
      gpr_free(send_records_);
      gpr_free(free_send_records_);
      VLOG(2) << "Disabling TCP TX zerocopy due to memory pressure.\n";
      memory_limited_ = true;
      enabled_ = false;
    } else {
      for (int idx = 0; idx < max_sends_; ++idx) {
        new (send_records_ + idx) TcpZerocopySendRecord();
        free_send_records_[idx] = send_records_ + idx;
      }
      enabled_ = zerocopy_enabled;
    }
  }

  ~TcpZerocopySendCtx() {
    if (send_records_ != nullptr) {
      for (int idx = 0; idx < max_sends_; ++idx) {
        send_records_[idx].~TcpZerocopySendRecord();
      }
    }
    gpr_free(send_records_);
    gpr_free(free_send_records_);
  }

  // True if we were unable to allocate the various bookkeeping structures at
  // transport initialization time. If memory limited, we do not zerocopy.
  bool MemoryLimited() const { return memory_limited_; }

  // TCP send zerocopy maintains an implicit sequence number for every
  // successful sendmsg() with zerocopy enabled; the kernel later gives us an
  // error queue notification with this sequence number indicating that the
  // underlying data buffers that we sent can now be released. Once that
  // notification is received, we can release the buffers associated with this
  // zerocopy send record. Here, we associate the sequence number with the data
  // buffers that were sent with the corresponding call to sendmsg().
  void NoteSend(TcpZerocopySendRecord* record) {
    record->Ref();
    {
      grpc_core::MutexLock lock(&mu_);
      is_in_write_ = true;
      AssociateSeqWithSendRecordLocked(last_send_, record);
    }
    ++last_send_;
  }

  // If sendmsg() actually failed, though, we need to revert the sequence number
  // that we speculatively bumped before calling sendmsg(). Note that we bump
  // this sequence number and perform relevant bookkeeping (see: NoteSend())
  // *before* calling sendmsg() since, if we called it *after* sendmsg(), then
  // there is a possible race with the release notification which could occur on
  // another thread before we do the necessary bookkeeping. Hence, calling
  // NoteSend() *before* sendmsg() and implementing an undo function is needed.
  void UndoSend() {
    --last_send_;
    if (ReleaseSendRecord(last_send_)->Unref()) {
      // We should still be holding the ref taken by tcp_write().
      DCHECK(0);
    }
  }

  // Simply associate this send record (and the underlying sent data buffers)
  // with the implicit sequence number for this zerocopy sendmsg().
  void AssociateSeqWithSendRecordLocked(uint32_t seq,
                                        TcpZerocopySendRecord* record)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    ctx_lookup_.emplace(seq, record);
  }

  // Get a send record for a send that we wish to do with zerocopy.
  TcpZerocopySendRecord* GetSendRecord() {
    grpc_core::MutexLock lock(&mu_);
    return TryGetSendRecordLocked();
  }

  // A given send record corresponds to a single tcp_write() with zerocopy
  // enabled. This can result in several sendmsg() calls to flush all of the
  // data to wire. Each sendmsg() takes a reference on the
  // TcpZerocopySendRecord, and corresponds to a single sequence number.
  // ReleaseSendRecord releases a reference on TcpZerocopySendRecord for a
  // single sequence number. This is called either when we receive the relevant
  // error queue notification (saying that we can discard the underlying
  // buffers for this sendmsg()) is received from the kernel - or, in case
  // sendmsg() was unsuccessful to begin with.
  TcpZerocopySendRecord* ReleaseSendRecord(uint32_t seq) {
    grpc_core::MutexLock lock(&mu_);
    return ReleaseSendRecordLocked(seq);
  }

  // After all the references to a TcpZerocopySendRecord are released, we can
  // add it back to the pool (of size max_sends_). Note that we can only have
  // max_sends_ tcp_write() instances with zerocopy enabled in flight at the
  // same time.
  void PutSendRecord(TcpZerocopySendRecord* record) {
    grpc_core::MutexLock lock(&mu_);
    DCHECK(record >= send_records_ && record < send_records_ + max_sends_);
    PutSendRecordLocked(record);
  }

  // Indicate that we are disposing of this zerocopy context. This indicator
  // will prevent new zerocopy writes from being issued.
  void Shutdown() { shutdown_.store(true, std::memory_order_release); }

  // Indicates that there are no inflight tcp_write() instances with zerocopy
  // enabled.
  bool AllSendRecordsEmpty() {
    grpc_core::MutexLock lock(&mu_);
    return free_send_records_size_ == max_sends_;
  }

  bool Enabled() const { return enabled_; }

  // Only use zerocopy if we are sending at least this many bytes. The
  // additional overhead of reading the error queue for notifications means that
  // zerocopy is not useful for small transfers.
  size_t ThresholdBytes() const { return threshold_bytes_; }

  // Expected to be called by handler reading messages from the err queue.
  // It is used to indicate that some optmem memory is now available. It returns
  // true to tell the caller to mark the file descriptor as immediately
  // writable.
  //
  // OptMem (controlled by the kernel option optmem_max) refers to the memory
  // allocated to the cmsg list maintained by the kernel that contains "extra"
  // packet information like SCM_RIGHTS or IP_TTL. Increasing this option allows
  // the kernel to allocate more memory as needed for more control messages that
  // need to be sent for each socket connected.
  //
  // If a write is currently in progress on the socket (ie. we have issued a
  // sendmsg() and are about to check its return value) then we set omem state
  // to CHECK to make the sending thread know that some tcp_omem was
  // concurrently freed even if sendmsg() returns ENOBUFS. In this case, since
  // there is already an active send thread, we do not need to mark the
  // socket writeable, so we return false.
  //
  // If there was no write in progress on the socket, and the socket was not
  // marked as FULL, then we need not mark the socket writeable now that some
  // tcp_omem memory is freed since it was not considered as blocked on
  // tcp_omem to begin with. So in this case, return false.
  //
  // But, if a write was not in progress and the omem state was FULL, then we
  // need to mark the socket writeable since it is no longer blocked by
  // tcp_omem. In this case, return true.
  //
  // Please refer to the STATE TRANSITION DIAGRAM below for more details.
  //
  bool UpdateZeroCopyOptMemStateAfterFree() {
    grpc_core::MutexLock lock(&mu_);
    if (is_in_write_) {
      zcopy_enobuf_state_ = OptMemState::kCheck;
      return false;
    }
    DCHECK(zcopy_enobuf_state_ != OptMemState::kCheck);
    if (zcopy_enobuf_state_ == OptMemState::kFull) {
      // A previous sendmsg attempt was blocked by ENOBUFS. Return true to
      // mark the fd as writable so the next write attempt could be made.
      zcopy_enobuf_state_ = OptMemState::kOpen;
      return true;
    } else if (zcopy_enobuf_state_ == OptMemState::kOpen) {
      // No need to mark the fd as writable because the previous write
      // attempt did not encounter ENOBUFS.
      return false;
    } else {
      // This state should never be reached because it implies that the previous
      // state was CHECK and is_in_write is false. This means that after the
      // previous sendmsg returned and set is_in_write to false, it did
      // not update the z-copy change from CHECK to OPEN.
      grpc_core::Crash("OMem state error!");
    }
  }

  // Expected to be called by the thread calling sendmsg after the syscall
  // invocation. is complete. If an ENOBUF is seen, it checks if the error
  // handler (Tx0cp completions) has already run and free'ed up some OMem. It
  // returns true indicating that the write can be attempted again immediately.
  // If ENOBUFS was seen but no Tx0cp completions have been received between the
  // sendmsg() and us taking this lock, then tcp_omem is still full from our
  // point of view. Therefore, we do not signal that the socket is writeable
  // with respect to the availability of tcp_omem. Therefore the function
  // returns false. This indicates that another write should not be attempted
  // immediately and the calling thread should wait until the socket is writable
  // again. If ENOBUFS was not seen, then again return false because the next
  // write should be attempted only when the socket is writable again.
  //
  // Please refer to the STATE TRANSITION DIAGRAM below for more details.
  //
  bool UpdateZeroCopyOptMemStateAfterSend(bool seen_enobuf, bool& constrained) {
    grpc_core::MutexLock lock(&mu_);
    is_in_write_ = false;
    constrained = false;
    if (seen_enobuf) {
      if (ctx_lookup_.size() == 1) {
        // There is no un-acked z-copy record. Set constrained to true to
        // indicate that we are re-source constrained because we're seeing
        // ENOBUFS even for the first record. This indicates that either
        // the process does not have hard memlock ulimit or RLIMIT_MEMLOCK
        // configured correctly.
        constrained = true;
      }
      if (zcopy_enobuf_state_ == OptMemState::kCheck) {
        zcopy_enobuf_state_ = OptMemState::kOpen;
        return true;
      } else {
        zcopy_enobuf_state_ = OptMemState::kFull;
      }
    } else if (zcopy_enobuf_state_ != OptMemState::kOpen) {
      zcopy_enobuf_state_ = OptMemState::kOpen;
    }
    return false;
  }

 private:
  //                      STATE TRANSITION DIAGRAM
  //
  // sendmsg succeeds       Tx-zero copy succeeds and there is no active sendmsg
  //      ----<<--+  +------<<-------------------------------------+
  //      |       |  |                                             |
  //      |       |  v       sendmsg returns ENOBUFS               |
  //      +-----> OPEN  ------------->>-------------------------> FULL
  //                ^                                              |
  //                |                                              |
  //                | sendmsg completes                            |
  //                +----<<---------- CHECK <-------<<-------------+
  //                                        Tx-zero copy succeeds and there is
  //                                        an active sendmsg
  //
  // OptMem (controlled by the kernel option optmem_max) refers to the memory
  // allocated to the cmsg list maintained by the kernel that contains "extra"
  // packet information like SCM_RIGHTS or IP_TTL. Increasing this option allows
  // the kernel to allocate more memory as needed for more control messages that
  // need to be sent for each socket connected. Each tx zero copy sendmsg has
  // a corresponding entry added into the Optmem queue. The entry is popped
  // from the Optmem queue when the zero copy send is complete.
  enum class OptMemState : int8_t {
    kOpen,   // Everything is clear and omem is not full.
    kFull,   // The last sendmsg() has returned with an errno of ENOBUFS.
    kCheck,  // Error queue is read while is_in_write_ was true, so we should
             // check this state after the sendmsg.
  };

  TcpZerocopySendRecord* ReleaseSendRecordLocked(uint32_t seq)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    auto iter = ctx_lookup_.find(seq);
    DCHECK(iter != ctx_lookup_.end());
    TcpZerocopySendRecord* record = iter->second;
    ctx_lookup_.erase(iter);
    return record;
  }

  TcpZerocopySendRecord* TryGetSendRecordLocked()
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    if (shutdown_.load(std::memory_order_acquire)) {
      return nullptr;
    }
    if (free_send_records_size_ == 0) {
      return nullptr;
    }
    free_send_records_size_--;
    return free_send_records_[free_send_records_size_];
  }

  void PutSendRecordLocked(TcpZerocopySendRecord* record)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(mu_) {
    DCHECK(free_send_records_size_ < max_sends_);
    free_send_records_[free_send_records_size_] = record;
    free_send_records_size_++;
  }

  TcpZerocopySendRecord* send_records_ ABSL_GUARDED_BY(mu_);
  TcpZerocopySendRecord** free_send_records_ ABSL_GUARDED_BY(mu_);
  int max_sends_;
  int free_send_records_size_ ABSL_GUARDED_BY(mu_);
  grpc_core::Mutex mu_;
  uint32_t last_send_ = 0;
  std::atomic<bool> shutdown_{false};
  bool enabled_ = false;
  size_t threshold_bytes_ = kDefaultSendBytesThreshold;
  absl::flat_hash_map<uint32_t, TcpZerocopySendRecord*> ctx_lookup_
      ABSL_GUARDED_BY(mu_);
  bool memory_limited_ = false;
  bool is_in_write_ ABSL_GUARDED_BY(mu_) = false;
  OptMemState zcopy_enobuf_state_ ABSL_GUARDED_BY(mu_) = OptMemState::kOpen;
};

class PosixEndpointImpl : public grpc_core::RefCounted<PosixEndpointImpl> {
 public:
  PosixEndpointImpl(
      EventHandle* handle, PosixEngineClosure* on_done,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine,
      grpc_event_engine::experimental::MemoryAllocator&& allocator,
      const PosixTcpOptions& options);
  ~PosixEndpointImpl() override;
  bool Read(
      absl::AnyInvocable<void(absl::Status)> on_read,
      grpc_event_engine::experimental::SliceBuffer* buffer,
      grpc_event_engine::experimental::EventEngine::Endpoint::ReadArgs args);
  bool Write(
      absl::AnyInvocable<void(absl::Status)> on_writable,
      grpc_event_engine::experimental::SliceBuffer* data,
      grpc_event_engine::experimental::EventEngine::Endpoint::WriteArgs args);
  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetPeerAddress() const {
    return peer_address_;
  }
  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetLocalAddress() const {
    return local_address_;
  }

  int GetWrappedFd() { return fd_; }

  bool CanTrackErrors() const { return poller_->CanTrackErrors(); }

  void MaybeShutdown(
      absl::Status why,
      absl::AnyInvocable<void(absl::StatusOr<int> release_fd)> on_release_fd);

 private:
  void UpdateRcvLowat() ABSL_EXCLUSIVE_LOCKS_REQUIRED(read_mu_);
  void HandleWrite(absl::Status status);
  void HandleError(absl::Status status);
  void HandleRead(absl::Status status) ABSL_NO_THREAD_SAFETY_ANALYSIS;
  bool HandleReadLocked(absl::Status& status)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(read_mu_);
  void MaybeMakeReadSlices() ABSL_EXCLUSIVE_LOCKS_REQUIRED(read_mu_);
  bool TcpDoRead(absl::Status& status) ABSL_EXCLUSIVE_LOCKS_REQUIRED(read_mu_);
  void FinishEstimate();
  void AddToEstimate(size_t bytes);
  void MaybePostReclaimer() ABSL_EXCLUSIVE_LOCKS_REQUIRED(read_mu_);
  void PerformReclamation() ABSL_LOCKS_EXCLUDED(read_mu_);
  // Zero copy related helper methods.
  TcpZerocopySendRecord* TcpGetSendZerocopyRecord(
      grpc_event_engine::experimental::SliceBuffer& buf);
  bool DoFlushZerocopy(TcpZerocopySendRecord* record, absl::Status& status);
  bool TcpFlushZerocopy(TcpZerocopySendRecord* record, absl::Status& status);
  bool TcpFlush(absl::Status& status);
  void TcpShutdownTracedBufferList();
  void UnrefMaybePutZerocopySendRecord(TcpZerocopySendRecord* record);
  void ZerocopyDisableAndWaitForRemaining();
  bool WriteWithTimestamps(struct msghdr* msg, size_t sending_length,
                           ssize_t* sent_length, int* saved_errno,
                           int additional_flags);
  absl::Status TcpAnnotateError(absl::Status src_error) const;
#ifdef GRPC_LINUX_ERRQUEUE
  bool ProcessErrors();
  // Reads a cmsg to process zerocopy control messages.
  void ProcessZerocopy(struct cmsghdr* cmsg);
  // Reads a cmsg to derive timestamps from the control messages.
  struct cmsghdr* ProcessTimestamp(msghdr* msg, struct cmsghdr* cmsg);
#endif  // GRPC_LINUX_ERRQUEUE
  grpc_core::Mutex read_mu_;
  PosixSocketWrapper sock_;
  int fd_;
  bool is_first_read_ = true;
  bool has_posted_reclaimer_ ABSL_GUARDED_BY(read_mu_) = false;
  double target_length_;
  int min_read_chunk_size_;
  int max_read_chunk_size_;
  int set_rcvlowat_ = 0;
  double bytes_read_this_round_ = 0;
  std::atomic<int> ref_count_{1};

  // garbage after the last read.
  grpc_event_engine::experimental::SliceBuffer last_read_buffer_;

  grpc_event_engine::experimental::SliceBuffer* incoming_buffer_
      ABSL_GUARDED_BY(read_mu_) = nullptr;
  // bytes pending on the socket from the last read.
  int inq_ = 1;
  // cache whether kernel supports inq.
  bool inq_capable_ = false;

  grpc_event_engine::experimental::SliceBuffer* outgoing_buffer_ = nullptr;
  // byte within outgoing_buffer's slices[0] to write next.
  size_t outgoing_byte_idx_ = 0;

  PosixEngineClosure* on_read_ = nullptr;
  PosixEngineClosure* on_write_ = nullptr;
  PosixEngineClosure* on_error_ = nullptr;
  PosixEngineClosure* on_done_ = nullptr;
  absl::AnyInvocable<void(absl::Status)> read_cb_ ABSL_GUARDED_BY(read_mu_);
  absl::AnyInvocable<void(absl::Status)> write_cb_;

  grpc_event_engine::experimental::EventEngine::ResolvedAddress peer_address_;
  grpc_event_engine::experimental::EventEngine::ResolvedAddress local_address_;

  // Maintain a shared_ptr to mem_quota_ to ensure the underlying basic memory
  // quota is not deleted until the endpoint is destroyed.
  grpc_core::MemoryQuotaRefPtr mem_quota_;
  grpc_core::MemoryOwner memory_owner_;
  grpc_core::MemoryAllocator::Reservation self_reservation_;

  void* outgoing_buffer_arg_ = nullptr;

  absl::AnyInvocable<void(absl::StatusOr<int>)> on_release_fd_ = nullptr;

  // A counter which starts at 0. It is initialized the first time the
  // socket options for collecting timestamps are set, and is incremented
  // with each byte sent.
  int bytes_counter_ = -1;
  // True if timestamping options are set on the socket.
#ifdef GRPC_LINUX_ERRQUEUE
  bool socket_ts_enabled_ = false;
#endif  // GRPC_LINUX_ERRQUEUE
  // Cache whether we can set timestamping options
  bool ts_capable_ = true;
  // Set to 1 if we do not want to be notified on errors anymore.
  std::atomic<bool> stop_error_notification_{false};
  std::unique_ptr<TcpZerocopySendCtx> tcp_zerocopy_send_ctx_;
  TcpZerocopySendRecord* current_zerocopy_send_ = nullptr;
  // A hint from upper layers specifying the minimum number of bytes that need
  // to be read to make meaningful progress.
  int min_progress_size_ = 1;
  TracedBufferList traced_buffers_;
  // The handle is owned by the PosixEndpointImpl object.
  EventHandle* handle_;
  PosixEventPoller* poller_;
  std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine_;
};

class PosixEndpoint : public PosixEndpointWithFdSupport {
 public:
  PosixEndpoint(
      EventHandle* handle, PosixEngineClosure* on_shutdown,
      std::shared_ptr<grpc_event_engine::experimental::EventEngine> engine,
      grpc_event_engine::experimental::MemoryAllocator&& allocator,
      const PosixTcpOptions& options)
      : impl_(new PosixEndpointImpl(handle, on_shutdown, std::move(engine),
                                    std::move(allocator), options)) {}

  bool Read(absl::AnyInvocable<void(absl::Status)> on_read,
            grpc_event_engine::experimental::SliceBuffer* buffer,
            grpc_event_engine::experimental::EventEngine::Endpoint::ReadArgs
                args) override {
    return impl_->Read(std::move(on_read), buffer, std::move(args));
  }

  bool Write(absl::AnyInvocable<void(absl::Status)> on_writable,
             grpc_event_engine::experimental::SliceBuffer* data,
             grpc_event_engine::experimental::EventEngine::Endpoint::WriteArgs
                 args) override {
    return impl_->Write(std::move(on_writable), data, std::move(args));
  }

  std::vector<size_t> AllWriteMetrics() override { return {}; }
  std::optional<absl::string_view> GetMetricName(size_t) override {
    return std::nullopt;
  }
  std::optional<size_t> GetMetricKey(absl::string_view) override {
    return std::nullopt;
  }

  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetPeerAddress() const override {
    return impl_->GetPeerAddress();
  }
  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetLocalAddress() const override {
    return impl_->GetLocalAddress();
  }

  int GetWrappedFd() override { return impl_->GetWrappedFd(); }

  bool CanTrackErrors() override { return impl_->CanTrackErrors(); }

  void Shutdown(absl::AnyInvocable<void(absl::StatusOr<int> release_fd)>
                    on_release_fd) override {
    if (!shutdown_.exchange(true, std::memory_order_acq_rel)) {
      impl_->MaybeShutdown(absl::FailedPreconditionError("Endpoint closing"),
                           std::move(on_release_fd));
    }
  }

  ~PosixEndpoint() override {
    if (!shutdown_.exchange(true, std::memory_order_acq_rel)) {
      impl_->MaybeShutdown(absl::FailedPreconditionError("Endpoint closing"),
                           nullptr);
    }
  }

 private:
  PosixEndpointImpl* impl_;
  std::atomic<bool> shutdown_{false};
};

#else  // GRPC_POSIX_SOCKET_TCP

class PosixEndpoint : public PosixEndpointWithFdSupport {
 public:
  PosixEndpoint() = default;

  bool Read(
      absl::AnyInvocable<void(absl::Status)> /*on_read*/,
      grpc_event_engine::experimental::SliceBuffer* /*buffer*/,
      grpc_event_engine::experimental::EventEngine::Endpoint::ReadArgs /*args*/)
      override {
    grpc_core::Crash("PosixEndpoint::Read not supported on this platform");
  }

  bool Write(absl::AnyInvocable<void(absl::Status)> /*on_writable*/,
             grpc_event_engine::experimental::SliceBuffer* /*data*/,
             grpc_event_engine::experimental::EventEngine::Endpoint::
                 WriteArgs /*args*/) override {
    grpc_core::Crash("PosixEndpoint::Write not supported on this platform");
  }

  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetPeerAddress() const override {
    grpc_core::Crash(
        "PosixEndpoint::GetPeerAddress not supported on this platform");
  }
  const grpc_event_engine::experimental::EventEngine::ResolvedAddress&
  GetLocalAddress() const override {
    grpc_core::Crash(
        "PosixEndpoint::GetLocalAddress not supported on this platform");
  }

  int GetWrappedFd() override {
    grpc_core::Crash(
        "PosixEndpoint::GetWrappedFd not supported on this platform");
  }

  bool CanTrackErrors() override {
    grpc_core::Crash(
        "PosixEndpoint::CanTrackErrors not supported on this platform");
  }

  void Shutdown(absl::AnyInvocable<void(absl::StatusOr<int> release_fd)>
                    on_release_fd) override {
    grpc_core::Crash("PosixEndpoint::Shutdown not supported on this platform");
  }

  ~PosixEndpoint() override = default;
};

#endif  // GRPC_POSIX_SOCKET_TCP

// Create a PosixEndpoint.
// A shared_ptr of the EventEngine is passed to the endpoint to ensure that
// the EventEngine is alive for the lifetime of the endpoint. The ownership
// of the EventHandle is transferred to the endpoint.
std::unique_ptr<PosixEndpoint> CreatePosixEndpoint(
    EventHandle* handle, PosixEngineClosure* on_shutdown,
    std::shared_ptr<EventEngine> engine,
    grpc_event_engine::experimental::MemoryAllocator&& allocator,
    const PosixTcpOptions& options);

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_POSIX_ENDPOINT_H
