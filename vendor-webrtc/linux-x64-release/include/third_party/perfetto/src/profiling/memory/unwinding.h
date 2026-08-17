/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_PROFILING_MEMORY_UNWINDING_H_
#define SRC_PROFILING_MEMORY_UNWINDING_H_

#include <unwindstack/Regs.h>

#include "perfetto/base/time.h"
#include "perfetto/ext/base/scoped_file.h"
#include "perfetto/ext/base/thread_task_runner.h"
#include "perfetto/ext/tracing/core/basic_types.h"
#include "src/profiling/common/unwind_support.h"
#include "src/profiling/memory/bookkeeping.h"
#include "src/profiling/memory/unwound_messages.h"
#include "src/profiling/memory/wire_protocol.h"

namespace perfetto {
namespace profiling {

std::unique_ptr<unwindstack::Regs> CreateRegsFromRawData(
    unwindstack::ArchEnum arch,
    void* raw_data);

bool DoUnwind(WireMessage*, UnwindingMetadata* metadata, AllocRecord* out);

// AllocRecords are expensive to construct and destruct. We have seen up to
// 10 % of total CPU of heapprofd being used to destruct them. That is why
// we reuse them to cut CPU usage significantly.
class AllocRecordArena {
 public:
  AllocRecordArena() : alloc_records_mutex_(new std::mutex()) {}

  void ReturnAllocRecord(std::unique_ptr<AllocRecord>);
  std::unique_ptr<AllocRecord> BorrowAllocRecord();

  void Enable();
  void Disable();

 private:
  std::unique_ptr<std::mutex> alloc_records_mutex_;
  std::vector<std::unique_ptr<AllocRecord>> alloc_records_;
  bool enabled_ = true;
};

class UnwindingWorker : public base::UnixSocket::EventListener {
 public:
  class Delegate {
   public:
    virtual void PostAllocRecord(UnwindingWorker*,
                                 std::unique_ptr<AllocRecord>) = 0;
    virtual void PostFreeRecord(UnwindingWorker*, std::vector<FreeRecord>) = 0;
    virtual void PostHeapNameRecord(UnwindingWorker*, HeapNameRecord rec) = 0;
    virtual void PostSocketDisconnected(UnwindingWorker*,
                                        DataSourceInstanceID,
                                        pid_t pid,
                                        SharedRingBuffer::Stats stats) = 0;
    virtual void PostDrainDone(UnwindingWorker*, DataSourceInstanceID) = 0;
    virtual ~Delegate();
  };

  struct HandoffData {
    DataSourceInstanceID data_source_instance_id;
    base::UnixSocketRaw sock;
    base::ScopedFile maps_fd;
    base::ScopedFile mem_fd;
    SharedRingBuffer shmem;
    ClientConfiguration client_config;
    bool stream_allocations;
  };

  UnwindingWorker(Delegate* delegate, base::ThreadTaskRunner thread_task_runner)
      : delegate_(delegate),
        thread_task_runner_(std::move(thread_task_runner)) {}

  ~UnwindingWorker() override;
  UnwindingWorker(UnwindingWorker&&) = default;

  // Public API safe to call from other threads.
  void PostDisconnectSocket(pid_t pid);
  void PostPurgeProcess(pid_t pid);
  void PostHandoffSocket(HandoffData);
  void PostDrainFree(DataSourceInstanceID, pid_t pid);
  void ReturnAllocRecord(std::unique_ptr<AllocRecord> record) {
    alloc_record_arena_.ReturnAllocRecord(std::move(record));
  }

  // Implementation of UnixSocket::EventListener.
  // Do not call explicitly.
  void OnDisconnect(base::UnixSocket* self) override;
  void OnNewIncomingConnection(base::UnixSocket*,
                               std::unique_ptr<base::UnixSocket>) override {
    PERFETTO_DFATAL_OR_ELOG("This should not happen.");
  }
  void OnDataAvailable(base::UnixSocket* self) override;

 public:
  // public for testing/fuzzer
  struct ClientData {
    DataSourceInstanceID data_source_instance_id;
    std::unique_ptr<base::UnixSocket> sock;
    UnwindingMetadata metadata;
    SharedRingBuffer shmem;
    ClientConfiguration client_config;
    bool stream_allocations = false;
    size_t drain_bytes = 0;
    std::vector<FreeRecord> free_records;
  };

  // public for testing/fuzzing
  static void HandleBuffer(UnwindingWorker* self,
                           AllocRecordArena* alloc_record_arena,
                           const SharedRingBuffer::Buffer& buf,
                           ClientData* client_data,
                           pid_t peer_pid,
                           Delegate* delegate);

 private:
  void HandleHandoffSocket(HandoffData data);
  void HandleDisconnectSocket(pid_t pid);
  void HandleDrainFree(DataSourceInstanceID, pid_t);
  void RemoveClientData(
      std::map<pid_t, ClientData>::iterator client_data_iterator);
  void FinishDisconnect(
      std::map<pid_t, ClientData>::iterator client_data_iterator);
  std::unique_ptr<AllocRecord> BorrowAllocRecord();

  struct ReadAndUnwindBatchResult {
    enum class Status {
      kHasMore,
      kReadSome,
      kReadNone,
    };
    size_t bytes_read = 0;
    Status status;
  };
  ReadAndUnwindBatchResult ReadAndUnwindBatch(ClientData* client_data);
  void BatchUnwindJob(pid_t);
  void DrainJob(pid_t);

  AllocRecordArena alloc_record_arena_;
  std::map<pid_t, ClientData> client_data_;
  Delegate* delegate_;

  // Task runner with a dedicated thread. Keep last. By destroying this task
  // runner first, we ensure that the UnwindingWorker is not active while the
  // rest of its state is being destroyed. Additionally this ensures that the
  // destructing thread sees a consistent view of the memory due to the
  // ThreadTaskRunner's destructor joining a thread.
  base::ThreadTaskRunner thread_task_runner_;
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_UNWINDING_H_
