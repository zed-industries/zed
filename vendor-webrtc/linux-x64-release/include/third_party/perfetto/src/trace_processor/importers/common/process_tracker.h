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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_PROCESS_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_PROCESS_TRACKER_H_

#include <cstdint>
#include <optional>
#include <unordered_map>
#include <unordered_set>
#include <utility>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

// Thread names can come from different sources, and we don't always want to
// overwrite the previously set name. This enum determines the priority of
// different sources.
enum class ThreadNamePriority {
  kOther = 0,
  kFtrace = 1,
  kEtwTrace = 1,
  kProcessTree = 2,
  kTrackDescriptorThreadType = 3,
  kTrackDescriptor = 4,

  // Priority when trace processor hardcodes a name for a process (e.g. calling
  // the idle thread "swapper" when parsing ftrace).
  // Keep this last.
  kTraceProcessorConstant = 5,
};

class ProcessTracker {
 public:
  explicit ProcessTracker(TraceProcessorContext*);
  ProcessTracker(const ProcessTracker&) = delete;
  ProcessTracker& operator=(const ProcessTracker&) = delete;
  virtual ~ProcessTracker();

  using UniqueThreadIterator = std::vector<UniqueTid>::const_iterator;
  using UniqueThreadBounds =
      std::pair<UniqueThreadIterator, UniqueThreadIterator>;

  // TODO(b/110409911): Invalidation of process and threads is yet to be
  // implemented. This will include passing timestamps into the below methods
  // to ensure the correct upid/utid is found.

  // Called when a task_newtask is observed. This force the tracker to start
  // a new UTID for the thread, which is needed for TID-recycling resolution.
  UniqueTid StartNewThread(std::optional<int64_t> timestamp, uint32_t tid);

  // Returns whether a thread is considered alive by the process tracker.
  bool IsThreadAlive(UniqueTid utid);

  // Called when sched_process_exit is observed. This forces the tracker to
  // end the thread lifetime for the utid associated with the given tid.
  void EndThread(int64_t timestamp, uint32_t tid);

  // Returns the thread utid or std::nullopt if it doesn't exist.
  std::optional<UniqueTid> GetThreadOrNull(uint32_t tid);

  // Returns the thread utid (or creates a new entry if not present)
  UniqueTid GetOrCreateThread(uint32_t tid);

  // Assigns the given name to the thread if the new name has a higher priority
  // than the existing one. Returns the utid of the thread.
  virtual UniqueTid UpdateThreadName(uint32_t tid,
                                     StringId thread_name_id,
                                     ThreadNamePriority priority);

  // Assigns the given name to the thread if the new name has a higher priority
  // than the existing one. The thread is identified by utid.
  virtual void UpdateThreadNameByUtid(UniqueTid utid,
                                      StringId thread_name_id,
                                      ThreadNamePriority priority);

  // Called when a thread is seen the process tree. Retrieves the matching utid
  // for the tid and the matching upid for the tgid and stores both.
  // Virtual for testing.
  virtual UniqueTid UpdateThread(uint32_t tid, uint32_t pid);

  // Associates trusted_pid with track UUID.
  void UpdateTrustedPid(uint32_t trusted_pid, uint64_t uuid);

  // Returns the trusted_pid associated with the track UUID, or std::nullopt if
  // not found.
  std::optional<uint32_t> GetTrustedPid(uint64_t uuid);

  // Performs namespace-local to root-level resolution of thread or process id,
  // given tid (can be root-level or namespace-local, but we don't know
  // beforehand) and root-level pid/tgid that the thread belongs to.
  // Returns the root-level thread id for tid on successful resolution;
  // otherwise, returns std::nullopt on resolution failure, or the thread of
  // tid isn't running in a pid namespace.
  std::optional<uint32_t> ResolveNamespacedTid(uint32_t root_level_pid,
                                               uint32_t tid);

  // Called when a task_newtask without the CLONE_THREAD flag is observed.
  // This force the tracker to start both a new UTID and a new UPID.
  UniquePid StartNewProcess(std::optional<int64_t> timestamp,
                            std::optional<uint32_t> parent_tid,
                            uint32_t pid,
                            StringId main_thread_name,
                            ThreadNamePriority priority);

  // Called when a process is seen in a process tree. Retrieves the UniquePid
  // for that pid or assigns a new one.
  // Virtual for testing.
  virtual UniquePid SetProcessMetadata(uint32_t pid,
                                       std::optional<uint32_t> ppid,
                                       base::StringView name,
                                       base::StringView cmdline);

  // Sets the process user id.
  void SetProcessUid(UniquePid upid, uint32_t uid);

  // Assigns the given name to the process identified by |upid| if it does not
  // have a name yet.
  virtual void SetProcessNameIfUnset(UniquePid upid, StringId process_name_id);

  // Sets the start timestamp to the process identified by |upid| if it doesn't
  // have a timestamp yet.
  void SetStartTsIfUnset(UniquePid upid, int64_t start_ts_nanoseconds);

  // Called on a task rename event to set the thread name and possibly process
  // name (if the tid provided is the main thread of the process).
  void UpdateThreadNameAndMaybeProcessName(uint32_t tid,
                                           StringId thread_name,
                                           ThreadNamePriority priority);

  // Called when a process is seen in a process tree. Retrieves the UniquePid
  // for that pid or assigns a new one.
  // Virtual for testing.
  virtual UniquePid GetOrCreateProcess(uint32_t pid);

  // Returns the upid for a given pid.
  std::optional<UniquePid> UpidForPidForTesting(uint32_t pid) {
    auto it = pids_.Find(pid);
    return it ? std::make_optional(*it) : std::nullopt;
  }

  // Returns the bounds of a range that includes all UniqueTids that have the
  // requested tid.
  UniqueThreadBounds UtidsForTidForTesting(uint32_t tid) {
    const auto& deque = tids_[tid];
    return std::make_pair(deque.begin(), deque.end());
  }

  // Marks the two threads as belonging to the same process, even if we don't
  // know which one yet. If one of the two threads is later mapped to a process,
  // the other will be mapped to the same process. The order of the two threads
  // is irrelevant, Associate(A, B) has the same effect of Associate(B, A).
  void AssociateThreads(UniqueTid, UniqueTid);

  // Creates the mapping from tid 0 <-> utid 0 and pid 0 <-> upid 0. This is
  // done for Linux-based system traces (proto or ftrace format) as for these
  // traces, we always have the "swapper" (idle) process having tid/pid 0.
  void SetPidZeroIsUpidZeroIdleProcess();

  // Returns a BoundInserter to add arguments to the arg set of a process.
  // Arguments are flushed into trace storage only after the trace was loaded in
  // its entirety.
  ArgsTracker::BoundInserter AddArgsTo(UniquePid upid);

  // Called when the trace was fully loaded.
  void NotifyEndOfFile();

  // Tracks the namespace-local pids for a process running in a pid namespace.
  void UpdateNamespacedProcess(uint32_t pid, std::vector<uint32_t> nspid);

  // Tracks the namespace-local thread ids for a thread running in a pid
  // namespace.
  void UpdateNamespacedThread(uint32_t pid,
                              uint32_t tid,
                              std::vector<uint32_t> nstid);

  // The UniqueTid of the swapper thread, is 0 for the default machine and is
  // > 0 for remote machines.
  UniqueTid swapper_utid() const { return swapper_utid_; }

 private:
  // Returns the utid of a thread having |tid| and |pid| as the parent process.
  // pid == std::nullopt matches all processes.
  // Returns std::nullopt if such a thread doesn't exist.
  std::optional<uint32_t> GetThreadOrNull(uint32_t tid,
                                          std::optional<uint32_t> pid);

  // Called whenever we discover that the passed thread belongs to the passed
  // process. The |pending_assocs_| vector is scanned to see if there are any
  // other threads associated to the passed thread.
  void ResolvePendingAssociations(UniqueTid, UniquePid);

  // Writes the association that the passed thread belongs to the passed
  // process.
  void AssociateThreadToProcess(UniqueTid, UniquePid);

  TraceProcessorContext* const context_;

  ArgsTracker args_tracker_;

  // Mapping for tid to the vector of possible UniqueTids.
  // TODO(lalitm): this is a one-many mapping because this code was written
  // before global sorting was a thing so multiple threads could be "active"
  // simultaneously. This is no longer the case so this should be removed
  // (though it seems like there are subtle things which break in Chrome if this
  // changes).
  base::FlatHashMap<uint32_t /* tid */, std::vector<UniqueTid>> tids_;

  // Mapping of the most recently seen pid to the associated upid.
  base::FlatHashMap<uint32_t /* pid (aka tgid) */, UniquePid> pids_;

  // Pending thread associations. The meaning of a pair<ThreadA, ThreadB> in
  // this vector is: we know that A and B belong to the same process, but we
  // don't know yet which process. A and A are idempotent, as in, pair<A,B> is
  // equivalent to pair<B,A>.
  std::vector<std::pair<UniqueTid, UniqueTid>> pending_assocs_;

  // Pending parent process associations. The meaning of pair<ThreadA, ProcessB>
  // in this vector is: we know that A created process B but we don't know the
  // process of A. That is, we don't know the parent *process* of B.
  std::vector<std::pair<UniqueTid, UniquePid>> pending_parent_assocs_;

  // A mapping from utid to the priority of a thread name source.
  std::vector<ThreadNamePriority> thread_name_priorities_;

  // A mapping from track UUIDs to trusted pids.
  std::unordered_map<uint64_t, uint32_t> trusted_pids_;

  struct NamespacedThread {
    uint32_t pid;                 // Root-level pid.
    uint32_t tid;                 // Root-level tid.
    std::vector<uint32_t> nstid;  // Namespace-local tids.
  };
  // Keeps track of pid-namespaced threads, keyed by root-level thread ids.
  std::unordered_map<uint32_t /* tid */, NamespacedThread> namespaced_threads_;

  struct NamespacedProcess {
    uint32_t pid;                          // Root-level pid.
    std::vector<uint32_t> nspid;           // Namespace-local pids.
    std::unordered_set<uint32_t> threads;  // Root-level thread IDs.
  };
  // Keeps track pid-namespaced processes, keyed by root-level pids.
  std::unordered_map<uint32_t /* pid (aka tgid) */, NamespacedProcess>
      namespaced_processes_;

  UniquePid swapper_upid_ = 0;
  UniqueTid swapper_utid_ = 0;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_PROCESS_TRACKER_H_
