// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_PARTITION_ALLOC_FEATURES_H_
#define BASE_ALLOCATOR_PARTITION_ALLOC_FEATURES_H_

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/feature_list.h"
#include "base/metrics/field_trial_params.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/time/time.h"
#include "partition_alloc/partition_root.h"

namespace base::features {

namespace internal {

enum class PAFeatureEnabledProcesses {
  // Enabled only in the browser process.
  kBrowserOnly,
  // Enabled only in the browser and renderer processes.
  kBrowserAndRenderer,
  // Enabled in all processes, except renderer.
  kNonRenderer,
  // Enabled only in renderer processes.
  kRendererOnly,
  // Enabled in all child processes, except zygote.
  kAllChildProcesses,
  // Enabled in all processes.
  kAllProcesses,
};

enum class SchedulerLoopQuarantineBranchType {
  // The global quarantine branch, shared across threads.
  kGlobal,
  // Default configuration for thread-local branches on new threads.
  kThreadLocalDefault,
  // Specialized configuration for the main thread of a process.
  kMain,
};

}  // namespace internal

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocUnretainedDanglingPtr);
enum class UnretainedDanglingPtrMode {
  kCrash,
  kDumpWithoutCrashing,
};
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(UnretainedDanglingPtrMode,
                                       kUnretainedDanglingPtrModeParam);

// See /docs/dangling_ptr.md
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocDanglingPtr);
enum class DanglingPtrMode {
  // Crash immediately after detecting a dangling raw_ptr.
  kCrash,  // (default)

  // Log the signature of every occurrences without crashing. It is used by
  // bots.
  // Format "[DanglingSignature]\t<1>\t<2>\t<3>\t<4>"
  // 1. The function which freed the memory while it was still referenced.
  // 2. The task in which the memory was freed.
  // 3. The function which released the raw_ptr reference.
  // 4. The task in which the raw_ptr was released.
  kLogOnly,

  // Note: This will be extended with a single shot DumpWithoutCrashing.
};
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(DanglingPtrMode, kDanglingPtrModeParam);
enum class DanglingPtrType {
  // Act on any dangling raw_ptr released after being freed.
  kAll,  // (default)

  // Detect when freeing memory and releasing the dangling raw_ptr happens in
  // a different task. Those are more likely to cause use after free.
  kCrossTask,

  // Note: This will be extended with LongLived
};
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(DanglingPtrType, kDanglingPtrTypeParam);

using PartitionAllocWithAdvancedChecksEnabledProcesses =
    internal::PAFeatureEnabledProcesses;

#if PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocLargeThreadCacheSize);
BASE_EXPORT int GetPartitionAllocLargeThreadCacheSizeValue();
BASE_EXPORT int GetPartitionAllocLargeThreadCacheSizeValueForLowRAMAndroid();

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocLargeEmptySlotSpanRing);

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocWithAdvancedChecks);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    PartitionAllocWithAdvancedChecksEnabledProcesses,
    kPartitionAllocWithAdvancedChecksEnabledProcessesParam);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocSchedulerLoopQuarantine);
// Scheduler Loop Quarantine's per-thread capacity in bytes.
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    int,
    kPartitionAllocSchedulerLoopQuarantineBranchCapacity);
// Scheduler Loop Quarantine's capacity for the UI thread in bytes.
// TODO(https://crbug.com/387470567): Support more thread types.
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    int,
    kPartitionAllocSchedulerLoopQuarantineBrowserUICapacity);

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocZappingByFreeFlags);

// Eventually zero out most PartitionAlloc memory. This is not meant as a
// security guarantee, but to increase the compression ratio of PartitionAlloc's
// fragmented super pages.
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocEventuallyZeroFreedMemory);

// Whether to make PartitionAlloc use fewer memory regions. This matters on
// Linux-based systems, where there is a per-process limit that we hit in some
// cases. See the comment in PartitionBucket::SlotSpanCOmmitedSize() for detail.
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocFewerMemoryRegions);
#endif  // PA_BUILDFLAG(USE_PARTITION_ALLOC_AS_MALLOC)

using BackupRefPtrEnabledProcesses = internal::PAFeatureEnabledProcesses;

enum class BackupRefPtrMode {
  // BRP is disabled across all partitions. Equivalent to the Finch flag being
  // disabled.
  kDisabled,

  // BRP is enabled in the main partition, as well as certain Renderer-only
  // partitions (if enabled in Renderer at all).
  kEnabled,
};

enum class MemtagMode {
  // memtagMode will be SYNC.
  kSync,
  // memtagMode will be ASYNC.
  kAsync,
};

enum class RetagMode {
  // Allocations are retagged by incrementing the current tag.
  kIncrement,

  // Allocations are retagged with a random tag.
  kRandom,
};

using MemoryTaggingEnabledProcesses = internal::PAFeatureEnabledProcesses;

enum class BucketDistributionMode : uint8_t {
  kDefault,
  kDenser,
};

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocBackupRefPtr);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(BackupRefPtrEnabledProcesses,
                                       kBackupRefPtrEnabledProcessesParam);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(BackupRefPtrMode,
                                       kBackupRefPtrModeParam);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(int,
                                       kBackupRefPtrExtraExtrasSizeParam);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocMemoryTagging);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(MemtagMode, kMemtagModeParam);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(RetagMode, kRetagModeParam);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(MemoryTaggingEnabledProcesses,
                                       kMemoryTaggingEnabledProcessesParam);
// Kill switch for memory tagging. Skips any code related to memory tagging when
// enabled.
BASE_EXPORT BASE_DECLARE_FEATURE(kKillPartitionAllocMemoryTagging);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocPermissiveMte);
BASE_EXPORT BASE_DECLARE_FEATURE(kAsanBrpDereferenceCheck);
BASE_EXPORT BASE_DECLARE_FEATURE(kAsanBrpExtractionCheck);
BASE_EXPORT BASE_DECLARE_FEATURE(kAsanBrpInstantiationCheck);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(BucketDistributionMode,
                                       kPartitionAllocBucketDistributionParam);

BASE_EXPORT BASE_DECLARE_FEATURE(kLowerPAMemoryLimitForNonMainRenderers);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocUseDenserDistribution);

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocMemoryReclaimer);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(TimeDelta,
                                       kPartitionAllocMemoryReclaimerInterval);
BASE_EXPORT BASE_DECLARE_FEATURE(
    kPartitionAllocStraightenLargerSlotSpanFreeLists);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    partition_alloc::StraightenLargerSlotSpanFreeListsMode,
    kPartitionAllocStraightenLargerSlotSpanFreeListsMode);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocSortSmallerSlotSpanFreeLists);
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocSortActiveSlotSpans);

#if BUILDFLAG(IS_WIN)
BASE_EXPORT BASE_DECLARE_FEATURE(kPageAllocatorRetryOnCommitFailure);
#endif

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_CHROMEOS)
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(
    bool,
    kPartialLowEndModeExcludePartitionAllocSupport);
#endif

BASE_EXPORT BASE_DECLARE_FEATURE(kEnableConfigurableThreadCacheMultiplier);
BASE_EXPORT double GetThreadCacheMultiplier();
BASE_EXPORT double GetThreadCacheMultiplierForAndroid();

BASE_EXPORT BASE_DECLARE_FEATURE(kEnableConfigurableThreadCachePurgeInterval);
extern const partition_alloc::internal::base::TimeDelta
GetThreadCacheMinPurgeInterval();
extern const partition_alloc::internal::base::TimeDelta
GetThreadCacheMaxPurgeInterval();
extern const partition_alloc::internal::base::TimeDelta
GetThreadCacheDefaultPurgeInterval();

BASE_EXPORT BASE_DECLARE_FEATURE(
    kEnableConfigurableThreadCacheMinCachedMemoryForPurging);
BASE_EXPORT int GetThreadCacheMinCachedMemoryForPurgingBytes();

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocDisableBRPInBufferPartition);

// When set, partitions use a larger ring buffer and free memory less
// aggressively when in the foreground.
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocAdjustSizeWhenInForeground);

// When enabled, uses a more nuanced heuristic to determine if slot
// spans can be treated as "single-slot."
//
// See also: https://crbug.com/333443437
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocUseSmallSingleSlotSpans);

#if PA_CONFIG(ENABLE_SHADOW_METADATA)
using ShadowMetadataEnabledProcesses = internal::PAFeatureEnabledProcesses;

BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocShadowMetadata);
BASE_EXPORT BASE_DECLARE_FEATURE_PARAM(ShadowMetadataEnabledProcesses,
                                       kShadowMetadataEnabledProcessesParam);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)

#if PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)
BASE_EXPORT BASE_DECLARE_FEATURE(kPartitionAllocUsePriorityInheritanceLocks);
#endif  // PA_BUILDFLAG(ENABLE_PARTITION_LOCK_PRIORITY_INHERITANCE)

}  // namespace base::features

#endif  // BASE_ALLOCATOR_PARTITION_ALLOC_FEATURES_H_
