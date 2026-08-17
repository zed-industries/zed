// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_PROCESS_MEMORY_DUMP_H_
#define BASE_TRACE_EVENT_PROCESS_MEMORY_DUMP_H_

#include <stddef.h>

#include <map>
#include <optional>
#include <unordered_map>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/gtest_prod_util.h"
#include "base/trace_event/heap_profiler_allocation_context.h"
#include "base/trace_event/memory_allocator_dump.h"
#include "base/trace_event/memory_allocator_dump_guid.h"
#include "base/trace_event/memory_dump_request_args.h"
#include "build/build_config.h"

// Define COUNT_RESIDENT_BYTES_SUPPORTED if platform supports counting of the
// resident memory.
#if !BUILDFLAG(IS_NACL)
#define COUNT_RESIDENT_BYTES_SUPPORTED
#endif

namespace perfetto {
namespace protos {
namespace pbzero {
class MemoryTrackerSnapshot;
}
}  // namespace protos
}  // namespace perfetto

namespace base {

class UnguessableToken;

namespace trace_event {

class TracedValue;

// ProcessMemoryDump is as a strongly typed container which holds the dumps
// produced by the MemoryDumpProvider(s) for a specific process.
class BASE_EXPORT ProcessMemoryDump {
 public:
  struct BASE_EXPORT MemoryAllocatorDumpEdge {
    friend bool operator==(const MemoryAllocatorDumpEdge&,
                           const MemoryAllocatorDumpEdge&) = default;

    MemoryAllocatorDumpGuid source;
    MemoryAllocatorDumpGuid target;
    int importance = 0;
    bool overridable = false;
  };

  // Maps allocator dumps absolute names (allocator_name/heap/subheap) to
  // MemoryAllocatorDump instances.
  using AllocatorDumpsMap =
      std::map<std::string, std::unique_ptr<MemoryAllocatorDump>>;

  // Stores allocator dump edges indexed by source allocator dump GUID.
  using AllocatorDumpEdgesMap =
      std::map<MemoryAllocatorDumpGuid, MemoryAllocatorDumpEdge>;

#if defined(COUNT_RESIDENT_BYTES_SUPPORTED)
  // Returns the number of bytes in a kernel memory page. Some platforms may
  // have a different value for kernel page sizes from user page sizes. It is
  // important to use kernel memory page sizes for resident bytes calculation.
  // In most cases, the two are the same.
  static size_t GetSystemPageSize();

  // Returns the total bytes resident for a virtual address range, with given
  // |start_address| and |mapped_size|. |mapped_size| is specified in bytes. The
  // value returned is valid only if the given range is currently mmapped by the
  // process. The |start_address| must be page-aligned.
  static std::optional<size_t> CountResidentBytes(void* start_address,
                                                  size_t mapped_size);

  // The same as above, but the given mapped range should belong to the
  // shared_memory's mapped region.
  static std::optional<size_t> CountResidentBytesInSharedMemory(
      void* start_address,
      size_t mapped_size);
#endif

  explicit ProcessMemoryDump(const MemoryDumpArgs& dump_args);
  ProcessMemoryDump(ProcessMemoryDump&&);

  ProcessMemoryDump(const ProcessMemoryDump&) = delete;
  ProcessMemoryDump& operator=(const ProcessMemoryDump&) = delete;

  ~ProcessMemoryDump();

  ProcessMemoryDump& operator=(ProcessMemoryDump&&);

  // Creates a new MemoryAllocatorDump with the given name and returns the
  // empty object back to the caller.
  // Arguments:
  //   absolute_name: a name that uniquely identifies allocator dumps produced
  //       by this provider. It is possible to specify nesting by using a
  //       path-like string (e.g., v8/isolate1/heap1, v8/isolate1/heap2).
  //       Leading or trailing slashes are not allowed.
  //   guid: an optional identifier, unique among all processes within the
  //       scope of a global dump. This is only relevant when using
  //       AddOwnershipEdge() to express memory sharing. If omitted,
  //       it will be automatically generated.
  // ProcessMemoryDump handles the memory ownership of its MemoryAllocatorDumps.
  MemoryAllocatorDump* CreateAllocatorDump(const std::string& absolute_name);
  MemoryAllocatorDump* CreateAllocatorDump(const std::string& absolute_name,
                                           const MemoryAllocatorDumpGuid& guid);

  // Looks up a MemoryAllocatorDump given its allocator and heap names, or
  // nullptr if not found.
  MemoryAllocatorDump* GetAllocatorDump(const std::string& absolute_name) const;

  // Do NOT use this method. All dump providers should use
  // CreateAllocatorDump(). Tries to create a new MemoryAllocatorDump only if it
  // doesn't already exist. Creating multiple dumps with same name using
  // GetOrCreateAllocatorDump() would override the existing scalars in MAD and
  // cause misreporting. This method is used only in rare cases multiple
  // components create allocator dumps with same name and only one of them adds
  // size.
  MemoryAllocatorDump* GetOrCreateAllocatorDump(
      const std::string& absolute_name);

  // Creates a shared MemoryAllocatorDump, to express cross-process sharing.
  // Shared allocator dumps are allowed to have duplicate guids within the
  // global scope, in order to reference the same dump from multiple processes.
  // See the design doc goo.gl/keU6Bf for reference usage patterns.
  MemoryAllocatorDump* CreateSharedGlobalAllocatorDump(
      const MemoryAllocatorDumpGuid& guid);

  // Creates a shared MemoryAllocatorDump as CreateSharedGlobalAllocatorDump,
  // but with a WEAK flag. A weak dump will be discarded unless a non-weak dump
  // is created using CreateSharedGlobalAllocatorDump by at least one process.
  // The WEAK flag does not apply if a non-weak dump with the same GUID already
  // exists or is created later. All owners and children of the discarded dump
  // will also be discarded transitively.
  MemoryAllocatorDump* CreateWeakSharedGlobalAllocatorDump(
      const MemoryAllocatorDumpGuid& guid);

  // Looks up a shared MemoryAllocatorDump given its guid.
  MemoryAllocatorDump* GetSharedGlobalAllocatorDump(
      const MemoryAllocatorDumpGuid& guid) const;

  // Returns the map of the MemoryAllocatorDumps added to this dump.
  const AllocatorDumpsMap& allocator_dumps() const LIFETIME_BOUND {
    return allocator_dumps_;
  }

  AllocatorDumpsMap* mutable_allocator_dumps_for_serialization() const {
    // Mojo takes a const input argument even for move-only types that can be
    // mutate while serializing (like this one). Hence the const_cast.
    return const_cast<AllocatorDumpsMap*>(&allocator_dumps_);
  }
  void SetAllocatorDumpsForSerialization(
      std::vector<std::unique_ptr<MemoryAllocatorDump>>);

  // Only for mojo serialization.
  std::vector<MemoryAllocatorDumpEdge> GetAllEdgesForSerialization() const;
  void SetAllEdgesForSerialization(const std::vector<MemoryAllocatorDumpEdge>&);

  // Adds an ownership relationship between two MemoryAllocatorDump(s) with the
  // semantics: |source| owns |target|, and has the effect of attributing
  // the memory usage of |target| to |source|. |importance| is optional and
  // relevant only for the cases of co-ownership, where it acts as a z-index:
  // the owner with the highest importance will be attributed |target|'s memory.
  // If an edge is present, its importance will not be updated unless
  // |importance| is larger.
  void AddOwnershipEdge(const MemoryAllocatorDumpGuid& source,
                        const MemoryAllocatorDumpGuid& target,
                        int importance);
  void AddOwnershipEdge(const MemoryAllocatorDumpGuid& source,
                        const MemoryAllocatorDumpGuid& target);

  // Adds edges that can be overriden by a later or earlier call to
  // AddOwnershipEdge() with the same source and target with a different
  // |importance| value.
  void AddOverridableOwnershipEdge(const MemoryAllocatorDumpGuid& source,
                                   const MemoryAllocatorDumpGuid& target,
                                   int importance);

  // Creates ownership edges for shared memory. Handles the case of cross
  // process sharing and importance of ownership for the case with and without
  // the shared memory dump provider. This handles both shared memory from both
  // legacy base::SharedMemory as well as current base::SharedMemoryMapping. The
  // weak version creates a weak global dump.
  // |client_local_dump_guid| The guid of the local dump created by the client
  // of base::SharedMemory.
  // |shared_memory_guid| The ID of the shared memory that is assigned globally,
  // used to create global dump edges in the new model.
  // |importance| Importance of the global dump edges to say if the current
  // process owns the memory segment.
  void CreateSharedMemoryOwnershipEdge(
      const MemoryAllocatorDumpGuid& client_local_dump_guid,
      const UnguessableToken& shared_memory_guid,
      int importance);
  void CreateWeakSharedMemoryOwnershipEdge(
      const MemoryAllocatorDumpGuid& client_local_dump_guid,
      const UnguessableToken& shared_memory_guid,
      int importance);

  const AllocatorDumpEdgesMap& allocator_dumps_edges() const LIFETIME_BOUND {
    return allocator_dumps_edges_;
  }

  // Utility method to add a suballocation relationship with the following
  // semantics: |source| is suballocated from |target_node_name|.
  // This creates a child node of |target_node_name| and adds an ownership edge
  // between |source| and the new child node. As a result, the UI will not
  // account the memory of |source| in the target node.
  void AddSuballocation(const MemoryAllocatorDumpGuid& source,
                        const std::string& target_node_name);

  // Removes all the MemoryAllocatorDump(s) contained in this instance. This
  // ProcessMemoryDump can be safely reused as if it was new once this returns.
  void Clear();

  // Merges all MemoryAllocatorDump(s) contained in |other| inside this
  // ProcessMemoryDump, transferring their ownership to this instance.
  // |other| will be an empty ProcessMemoryDump after this method returns.
  // This is to allow dump providers to pre-populate ProcessMemoryDump instances
  // and later move their contents into the ProcessMemoryDump passed as argument
  // of the MemoryDumpProvider::OnMemoryDump(ProcessMemoryDump*) callback.
  void TakeAllDumpsFrom(ProcessMemoryDump* other);

  // Populate the traced value with information about the memory allocator
  // dumps.
  void SerializeAllocatorDumpsInto(TracedValue* value) const;

  void SerializeAllocatorDumpsInto(
      perfetto::protos::pbzero::MemoryTrackerSnapshot* memory_snapshot,
      const base::ProcessId pid) const;

  const MemoryDumpArgs& dump_args() const LIFETIME_BOUND { return dump_args_; }

 private:
  FRIEND_TEST_ALL_PREFIXES(ProcessMemoryDumpTest, BackgroundModeTest);
  FRIEND_TEST_ALL_PREFIXES(ProcessMemoryDumpTest, SharedMemoryOwnershipTest);
  FRIEND_TEST_ALL_PREFIXES(ProcessMemoryDumpTest, GuidsTest);

  MemoryAllocatorDump* AddAllocatorDumpInternal(
      std::unique_ptr<MemoryAllocatorDump> mad);

  // A per-process token, valid throughout all the lifetime of the current
  // process, used to disambiguate dumps with the same name generated in
  // different processes.
  const UnguessableToken& process_token() const LIFETIME_BOUND {
    return process_token_;
  }
  void set_process_token_for_testing(UnguessableToken token) {
    process_token_ = token;
  }

  // Returns the Guid of the dump for the given |absolute_name| for
  // for the given process' token. |process_token| is used to disambiguate GUIDs
  // derived from the same name under different processes.
  MemoryAllocatorDumpGuid GetDumpId(const std::string& absolute_name);

  void CreateSharedMemoryOwnershipEdgeInternal(
      const MemoryAllocatorDumpGuid& client_local_dump_guid,
      const UnguessableToken& shared_memory_guid,
      int importance,
      bool is_weak);

  MemoryAllocatorDump* GetBlackHoleMad(const std::string& absolute_name);

  UnguessableToken process_token_;
  AllocatorDumpsMap allocator_dumps_;

  // Keeps track of relationships between MemoryAllocatorDump(s).
  AllocatorDumpEdgesMap allocator_dumps_edges_;

  // Level of detail of the current dump.
  MemoryDumpArgs dump_args_;

  // This allocator dump is returned when an invalid dump is created in
  // background mode. The attributes of the dump are ignored and not added to
  // the trace.
  std::unique_ptr<MemoryAllocatorDump> black_hole_mad_;

  // When set to true, the DCHECK(s) for invalid dump creations on the
  // background mode are disabled for testing.
  static bool is_black_hole_non_fatal_for_testing_;
};

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_PROCESS_MEMORY_DUMP_H_
