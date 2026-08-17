// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_STATS_H_
#define PARTITION_ALLOC_PARTITION_STATS_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"

namespace partition_alloc {

// Most of these are not populated if PA_THREAD_CACHE_ENABLE_STATISTICS is not
// defined.
struct ThreadCacheStats {
  uint64_t alloc_count;   // Total allocation requests.
  uint64_t alloc_hits;    // Thread cache hits.
  uint64_t alloc_misses;  // Thread cache misses.

  // Allocation failure details:
  uint64_t alloc_miss_empty;
  uint64_t alloc_miss_too_large;

  // Cache fill details:
  uint64_t cache_fill_count;
  uint64_t cache_fill_hits;
  uint64_t cache_fill_misses;  // Object too large.

  uint64_t batch_fill_count;  // Number of central allocator requests.

  // Memory cost:
  uint32_t bucket_total_memory;
  uint32_t metadata_overhead;

#if PA_CONFIG(THREAD_CACHE_ALLOC_STATS)
  uint64_t allocs_per_bucket_[internal::kNumBuckets + 1];
#endif  // PA_CONFIG(THREAD_CACHE_ALLOC_STATS)
};

// Per-thread allocation statistics. Only covers allocations made through the
// partition linked to the thread cache. As the allocator doesn't record
// requested sizes in most cases, the data there will be an overestimate of the
// actually requested sizes. It is also not expected to sum up to anything
// meaningful across threads, due to the lack of synchronization. Figures there
// are cumulative, not net. Since the data below is per-thread, note a thread
// can deallocate more than it allocated.
struct ThreadAllocStats {
  uint64_t alloc_count;
  uint64_t alloc_total_size;
  uint64_t dealloc_count;
  uint64_t dealloc_total_size;
};

struct LightweightQuarantineStats {
  size_t size_in_bytes;
  size_t count;
  size_t cumulative_size_in_bytes;
  size_t cumulative_count;
  size_t quarantine_miss_count;  // Object too large.
};

// Struct used to retrieve total memory usage of a partition. Used by
// PartitionStatsDumper implementation.
struct PartitionMemoryStats {
  size_t total_mmapped_bytes;    // Total bytes mmap()-ed from the system.
  size_t total_committed_bytes;  // Total size of committed pages.
  size_t max_committed_bytes;    // Max size of committed pages.
  size_t total_allocated_bytes;  // Total size of allcoations.
  size_t max_allocated_bytes;    // Max size of allocations.
  size_t total_resident_bytes;   // Total bytes provisioned by the partition.
  size_t total_active_bytes;     // Total active bytes in the partition.
  size_t total_active_count;  // Total count of active objects in the partition.
  size_t total_decommittable_bytes;  // Total bytes that could be decommitted.
  size_t total_discardable_bytes;    // Total bytes that could be discarded.
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
  size_t
      total_brp_quarantined_bytes;  // Total bytes that are quarantined by BRP.
  size_t total_brp_quarantined_count;       // Total number of slots that are
                                            // quarantined by BRP.
  size_t cumulative_brp_quarantined_bytes;  // Cumulative bytes that are
                                            // quarantined by BRP.
  size_t cumulative_brp_quarantined_count;  // Cumulative number of slots that
                                            // are quarantined by BRP.
#endif

  bool has_thread_cache;
  ThreadCacheStats current_thread_cache_stats;
  ThreadCacheStats all_thread_caches_stats;

  bool has_scheduler_loop_quarantine;
  LightweightQuarantineStats scheduler_loop_quarantine_stats_total;

  // Count and total duration of system calls made since process start. May not
  // be reported on all platforms.
  uint64_t syscall_count;
  uint64_t syscall_total_time_ns;
};

// Struct used to retrieve memory statistics about a partition bucket. Used by
// PartitionStatsDumper implementation.
struct PartitionBucketMemoryStats {
  bool is_valid;       // Used to check if the stats is valid.
  bool is_direct_map;  // True if this is a direct mapping; size will not be
                       // unique.
  uint32_t bucket_slot_size;          // The size of the slot in bytes.
  uint32_t allocated_slot_span_size;  // Total size the slot span allocated
                                      // from the system (committed pages).
  uint32_t active_bytes;              // Total active bytes used in the bucket.
  uint32_t active_count;    // Total active objects allocated in the bucket.
  uint32_t resident_bytes;  // Total bytes provisioned in the bucket.
  uint32_t decommittable_bytes;    // Total bytes that could be decommitted.
  uint32_t discardable_bytes;      // Total bytes that could be discarded.
  uint32_t num_full_slot_spans;    // Number of slot spans with all slots
                                   // allocated.
  uint32_t num_active_slot_spans;  // Number of slot spans that have at least
                                   // one provisioned slot.
  uint32_t num_empty_slot_spans;   // Number of slot spans that are empty
                                   // but not decommitted.
  uint32_t num_decommitted_slot_spans;  // Number of slot spans that are empty
                                        // and decommitted.
};

// Interface that is passed to PartitionDumpStats and
// PartitionDumpStats for using the memory statistics.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) PartitionStatsDumper {
 public:
  virtual ~PartitionStatsDumper() = default;

  // Called to dump total memory used by partition, once per partition.
  virtual void PartitionDumpTotals(const char* partition_name,
                                   const PartitionMemoryStats*) = 0;

  // Called to dump stats about buckets, for each bucket.
  virtual void PartitionsDumpBucketStats(const char* partition_name,
                                         const PartitionBucketMemoryStats*) = 0;
};

// Simple version of PartitionStatsDumper, storing the returned stats in stats_.
// Does not handle per-bucket stats.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) SimplePartitionStatsDumper
    : public PartitionStatsDumper {
 public:
  SimplePartitionStatsDumper();
  ~SimplePartitionStatsDumper() override = default;

  void PartitionDumpTotals(const char* partition_name,
                           const PartitionMemoryStats* memory_stats) override;

  void PartitionsDumpBucketStats(const char* partition_name,
                                 const PartitionBucketMemoryStats*) override {}

  const PartitionMemoryStats& stats() const { return stats_; }

 private:
  PartitionMemoryStats stats_;
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PARTITION_STATS_H_
