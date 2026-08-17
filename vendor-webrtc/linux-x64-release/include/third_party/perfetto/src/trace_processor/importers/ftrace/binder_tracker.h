/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_BINDER_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_BINDER_TRACKER_H_

#include <stdint.h>
#include <optional>
#include <stack>

#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class BinderTracker : public Destructible {
 public:
  // Commands sent from userspace to the kernel binder driver.
  enum : uint32_t {
    kBC_TRANSACTION = 0x40406300,
    kBC_REPLY = 0x40406301,
    kBC_TRANSACTION_SG = 0x40486311,
    kBC_REPLY_SG = 0x40486312,
  };

  // Return commands sent from the kernel binder driver to userspace.
  enum : uint32_t {
    kBR_TRANSACTION_SEC_CTX = 0x80487202,
    kBR_TRANSACTION = 0x80407202,
    kBR_REPLY = 0x80407203,
    kBR_DEAD_REPLY = 0x7205,
    kBR_TRANSACTION_COMPLETE = 0x7206,
    kBR_FAILED_REPLY = 0x7211,
    kBR_FROZEN_REPLY = 0x7212,
    kBR_TRANSACTION_PENDING_FROZEN = 0x7214,
    kBR_ONEWAY_SPAM_SUSPECT = 0x7213,
  };

  using SetArgsCallback = std::function<void(ArgsTracker::BoundInserter*)>;
  // Declared public for testing only.
  explicit BinderTracker(TraceProcessorContext*);
  BinderTracker(const BinderTracker&) = delete;
  BinderTracker& operator=(const BinderTracker&) = delete;
  ~BinderTracker() override;
  static BinderTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->binder_tracker) {
      context->binder_tracker.reset(new BinderTracker(context));
    }
    return static_cast<BinderTracker*>(context->binder_tracker.get());
  }

  void Transaction(int64_t timestamp,
                   uint32_t tid,
                   int32_t transaction_id,
                   int32_t dest_node,
                   uint32_t dest_tgid,
                   uint32_t dest_tid,
                   bool is_reply,
                   uint32_t flags,
                   StringId code);
  void Locked(int64_t timestamp, uint32_t pid);
  void Lock(int64_t timestamp, uint32_t dest_pid);
  void Unlock(int64_t timestamp, uint32_t pid);
  void TransactionReceived(int64_t timestamp,
                           uint32_t tid,
                           int32_t transaction_id);
  void CommandToKernel(int64_t timestamp, uint32_t tid, uint32_t cmd);
  void ReturnFromKernel(int64_t timestamp, uint32_t tid, uint32_t cmd);
  void TransactionAllocBuf(int64_t timestamp,
                           uint32_t pid,
                           uint64_t data_size,
                           uint64_t offsets_size);

  // For testing
  bool utid_stacks_empty() const { return utid_stacks_.size() == 0; }

 private:
  TraceProcessorContext* const context_;

  struct OutstandingTransaction {
    bool is_reply = false;
    bool is_oneway = false;
    SetArgsCallback args_inserter;
    std::optional<TrackId> send_track_id;
    std::optional<SliceId> send_slice_id;
  };
  // TODO(rsavitski): switch back to FlatHashMap once the latter's perf is fixed
  // for insert+erase heavy workfloads.
  std::unordered_map<int32_t, OutstandingTransaction> outstanding_transactions_;

  struct TxnFrame {
    // The state of this thread at this stack level.
    enum State : uint32_t;
    State state;
    struct TxnInfo {
      bool is_oneway;
      bool is_reply;
    };
    std::optional<TxnInfo> txn_info;
  };
  // Each thread can have a stack of multiple transactions.
  base::FlatHashMap<UniqueTid, std::stack<TxnFrame>> utid_stacks_;

  // Returns the current state of this thread or nullptr, if the thread doesn't
  // have a binder state.
  TxnFrame* GetTidTopFrame(uint32_t tid);
  // Creates a new frame in the stack for this thread. Note: this might
  // invalidate previously returned TxnFrame*.
  TxnFrame* PushTidFrame(uint32_t tid);
  // Removes the current frame for this thread. It's an error to call this if
  // the thread didn't have a frame. Note: this might invalidate previously
  // returned TxnFrame*.
  void PopTidFrame(uint32_t tid);

  base::FlatHashMap<uint32_t, int64_t> attempt_lock_;
  base::FlatHashMap<uint32_t, int64_t> lock_acquired_;

  const StringId binder_category_id_;
  const StringId lock_waiting_id_;
  const StringId lock_held_id_;
  const StringId transaction_slice_id_;
  const StringId transaction_async_id_;
  const StringId reply_id_;
  const StringId async_rcv_id_;
  const StringId transaction_id_;
  const StringId dest_node_;
  const StringId dest_process_;
  const StringId dest_thread_;
  const StringId dest_name_;
  const StringId is_reply_;
  const StringId flags_;
  const StringId code_;
  const StringId calling_tid_;
  const StringId data_size_;
  const StringId offsets_size_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_BINDER_TRACKER_H_
