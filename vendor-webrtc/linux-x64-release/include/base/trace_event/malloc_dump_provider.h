// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_MALLOC_DUMP_PROVIDER_H_
#define BASE_TRACE_EVENT_MALLOC_DUMP_PROVIDER_H_

#include "base/allocator/buildflags.h"
#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/singleton.h"
#include "base/synchronization/lock.h"
#include "base/time/time.h"
#include "base/trace_event/memory_dump_provider.h"
#include "build/build_config.h"
#include "partition_alloc/buildflags.h"

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
#include "partition_alloc/partition_stats.h"  // nogncheck
#endif

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID) || \
    BUILDFLAG(IS_WIN) || BUILDFLAG(IS_MAC)
#define MALLOC_MEMORY_TRACING_SUPPORTED
#endif

namespace base {
namespace trace_event {

class MemoryAllocatorDump;

// Dump provider which collects process-wide memory stats.
class BASE_EXPORT MallocDumpProvider : public MemoryDumpProvider {
 public:
  // Name of the allocated_objects dump. Use this to declare suballocator dumps
  // from other dump providers.
  static const char kAllocatedObjects[];

  static MallocDumpProvider* GetInstance();

  // The Extreme LUD is implemented in //components/gwp_asan, which //base
  // cannot depend on. The following API allows an injection of stats-report
  // function of the Extreme LUD.
  struct ExtremeLUDStats {
#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
    // This default-constructs to be zero'ed.
    partition_alloc::LightweightQuarantineStats lq_stats{0};
#endif
    size_t capacity_in_bytes = 0;
  };
  struct ExtremeLUDStatsSet {
    ExtremeLUDStats for_small_objects{};
    ExtremeLUDStats for_large_objects{};
  };
#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  using ExtremeLUDGetStatsCallback = RepeatingCallback<ExtremeLUDStatsSet()>;
  static void SetExtremeLUDGetStatsCallback(
      ExtremeLUDGetStatsCallback callback);
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

  MallocDumpProvider(const MallocDumpProvider&) = delete;
  MallocDumpProvider& operator=(const MallocDumpProvider&) = delete;

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const MemoryDumpArgs& args,
                    ProcessMemoryDump* pmd) override;

 private:
  struct CumulativeEludStats {
    size_t quarantined_bytes = 0;
    size_t quarantined_count = 0;
    size_t miss_count = 0;
  };

  friend struct DefaultSingletonTraits<MallocDumpProvider>;

  MallocDumpProvider();
  ~MallocDumpProvider() override;

  void ReportPerMinuteStats(uint64_t syscall_count,
                            size_t cumulative_brp_quarantined_bytes,
                            size_t cumulative_brp_quarantined_count,
                            const ExtremeLUDStats& elud_stats_for_small_objects,
                            const ExtremeLUDStats& elud_stats_for_large_objects,
                            MemoryAllocatorDump* malloc_dump,
                            MemoryAllocatorDump* partition_alloc_dump,
                            MemoryAllocatorDump* elud_dump_for_small_objects,
                            MemoryAllocatorDump* elud_dump_for_large_objects);

  bool emit_metrics_on_memory_dump_
      GUARDED_BY(emit_metrics_on_memory_dump_lock_) = true;
  base::Lock emit_metrics_on_memory_dump_lock_;

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
  // Returns a reference to the injected stats-report function of the Extreme
  // LUD. The returned callback is_null() if the Extreme LUD is not enabled.
  static ExtremeLUDGetStatsCallback& GetExtremeLUDGetStatsCallback();
  // To be accurate, this requires the dump provider to be created very early,
  // which is the case. The alternative would be to drop the first data point,
  // which is not desirable as early process activity is highly relevant.
  base::TimeTicks last_memory_dump_time_ = base::TimeTicks::Now();
  uint64_t last_syscall_count_ = 0;
  size_t last_cumulative_brp_quarantined_bytes_ = 0;
  size_t last_cumulative_brp_quarantined_count_ = 0;
  CumulativeEludStats last_cumulative_elud_stats_for_small_objects_{0};
  CumulativeEludStats last_cumulative_elud_stats_for_large_objects_{0};
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
};

#if PA_BUILDFLAG(USE_PARTITION_ALLOC)
// This class is used to invert the dependency of PartitionAlloc on the
// PartitionAllocMemoryDumpProvider. This implements an interface that will
// be called with memory statistics for each bucket in the allocator.
class BASE_EXPORT MemoryDumpPartitionStatsDumper final
    : public partition_alloc::PartitionStatsDumper {
 public:
  MemoryDumpPartitionStatsDumper(const char* root_name,
                                 ProcessMemoryDump* memory_dump,
                                 MemoryDumpLevelOfDetail level_of_detail);

  static constexpr char kPartitionsDumpName[] = "partitions";

  // PartitionStatsDumper implementation.
  void PartitionDumpTotals(
      const char* partition_name,
      const partition_alloc::PartitionMemoryStats*) override;
  void PartitionsDumpBucketStats(
      const char* partition_name,
      const partition_alloc::PartitionBucketMemoryStats*) override;

  size_t total_mmapped_bytes() const { return total_mmapped_bytes_; }
  size_t total_resident_bytes() const { return total_resident_bytes_; }
  size_t total_active_bytes() const { return total_active_bytes_; }
  size_t total_active_count() const { return total_active_count_; }
  uint64_t syscall_count() const { return syscall_count_; }
  size_t cumulative_brp_quarantined_bytes() const {
    return cumulative_brp_quarantined_bytes_;
  }
  size_t cumulative_brp_quarantined_count() const {
    return cumulative_brp_quarantined_count_;
  }

 private:
  const char* root_name_;
  raw_ptr<base::trace_event::ProcessMemoryDump> memory_dump_;
  uint64_t uid_ = 0;
  size_t total_mmapped_bytes_ = 0;
  size_t total_resident_bytes_ = 0;
  size_t total_active_bytes_ = 0;
  size_t total_active_count_ = 0;
  uint64_t syscall_count_ = 0;
  size_t cumulative_brp_quarantined_bytes_ = 0;
  size_t cumulative_brp_quarantined_count_ = 0;
  bool detailed_;
};

#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC)

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_MALLOC_DUMP_PROVIDER_H_
