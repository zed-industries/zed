// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SHARED_MEMORY_TRACKER_H_
#define BASE_MEMORY_SHARED_MEMORY_TRACKER_H_

#include <map>
#include <string>

#include "base/base_export.h"
#include "base/memory/shared_memory_mapping.h"
#include "base/synchronization/lock.h"
#include "base/trace_event/base_tracing.h"

namespace base {

namespace trace_event {
class MemoryAllocatorDump;
class MemoryAllocatorDumpGuid;
class ProcessMemoryDump;
}  // namespace trace_event

// SharedMemoryTracker tracks shared memory usage.
class BASE_EXPORT SharedMemoryTracker : public trace_event::MemoryDumpProvider {
 public:
  // Returns a singleton instance.
  static SharedMemoryTracker* GetInstance();

  SharedMemoryTracker(const SharedMemoryTracker&) = delete;
  SharedMemoryTracker& operator=(const SharedMemoryTracker&) = delete;

  static std::string GetDumpNameForTracing(const UnguessableToken& id);

  static trace_event::MemoryAllocatorDumpGuid GetGlobalDumpIdForTracing(
      const UnguessableToken& id);

  // Gets or creates if non-existant, a memory dump for the |shared_memory|
  // inside the given |pmd|. Also adds the necessary edges for the dump when
  // creating the dump.
  static const trace_event::MemoryAllocatorDump* GetOrCreateSharedMemoryDump(
      const SharedMemoryMapping& shared_memory,
      trace_event::ProcessMemoryDump* pmd);

  // Records shared memory usage on valid mapping.
  void IncrementMemoryUsage(const SharedMemoryMapping& mapping);

  // Records shared memory usage on unmapping.
  void DecrementMemoryUsage(const SharedMemoryMapping& mapping);

  // Root dump name for all shared memory dumps.
  static const char kDumpRootName[];

 private:
  SharedMemoryTracker();
  ~SharedMemoryTracker() override;

  // trace_event::MemoryDumpProvider implementation.
  bool OnMemoryDump(const trace_event::MemoryDumpArgs& args,
                    trace_event::ProcessMemoryDump* pmd) override;

  static const trace_event::MemoryAllocatorDump*
  GetOrCreateSharedMemoryDumpInternal(void* mapped_memory,
                                      size_t mapped_size,
                                      const UnguessableToken& mapped_id,
                                      trace_event::ProcessMemoryDump* pmd);

  // Information associated with each mapped address.
  struct UsageInfo {
    UsageInfo(size_t size, const UnguessableToken& id)
        : mapped_size(size), mapped_id(id) {}

    size_t mapped_size;
    UnguessableToken mapped_id;
  };

  Lock usages_lock_;
  std::map<void*, UsageInfo> usages_ GUARDED_BY(usages_lock_);
};

}  // namespace base

#endif  // BASE_MEMORY_SHARED_MEMORY_TRACKER_H_
