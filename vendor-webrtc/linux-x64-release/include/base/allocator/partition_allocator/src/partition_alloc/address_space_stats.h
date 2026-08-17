// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_ADDRESS_SPACE_STATS_H_
#define PARTITION_ALLOC_ADDRESS_SPACE_STATS_H_

#include <cstddef>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc {

// All members are measured in super pages.
struct PoolStats {
  size_t usage = 0;

  // On 32-bit, pools are mainly logical entities, intermingled with
  // allocations not managed by PartitionAlloc. The "largest available
  // reservation" is not possible to measure in that case.
#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  size_t largest_available_reservation = 0;
#endif
};

struct AddressSpaceStats {
  PoolStats regular_pool_stats;
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
  PoolStats brp_pool_stats;
#endif  // PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  PoolStats configurable_pool_stats;
#else
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
  size_t blocklist_size;  // measured in super pages
  size_t blocklist_hit_count;
#endif  // PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  PoolStats thread_isolated_pool_stats;
#endif
};

// Interface passed to `AddressPoolManager::DumpStats()` to mediate
// for `AddressSpaceDumpProvider`.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) AddressSpaceStatsDumper {
 public:
  virtual void DumpStats(const AddressSpaceStats* address_space_stats) = 0;
  virtual ~AddressSpaceStatsDumper() = default;
};

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_ADDRESS_SPACE_STATS_H_
