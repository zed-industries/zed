// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_ADDRESS_POOL_MANAGER_H_
#define PARTITION_ALLOC_ADDRESS_POOL_MANAGER_H_

#include <bitset>
#include <limits>

#include "partition_alloc/address_pool_manager_types.h"
#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_address_space.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_lock.h"
#include "partition_alloc/thread_isolation/alignment.h"
#include "partition_alloc/thread_isolation/thread_isolation.h"

#if !PA_BUILDFLAG(HAS_64_BIT_POINTERS)
#include "partition_alloc/address_pool_manager_bitmap.h"
#endif

namespace partition_alloc {

class AddressSpaceStatsDumper;
struct AddressSpaceStats;
struct PoolStats;

}  // namespace partition_alloc

namespace partition_alloc::internal {

// (64bit version)
// AddressPoolManager takes a reserved virtual address space and manages address
// space allocation.
//
// AddressPoolManager (currently) supports up to 4 pools. Each pool manages a
// contiguous reserved address space. Alloc() takes a pool_handle and returns
// address regions from the specified pool. Free() also takes a pool_handle and
// returns the address region back to the manager.
//
// (32bit version)
// AddressPoolManager wraps AllocPages and FreePages and remembers allocated
// address regions using bitmaps. IsManagedByPartitionAlloc*Pool use the bitmaps
// to judge whether a given address is in a pool that supports BackupRefPtr or
// in a pool that doesn't. All PartitionAlloc allocations must be in either of
// the pools.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC)
    PA_THREAD_ISOLATED_ALIGN AddressPoolManager {
 public:
  static AddressPoolManager& GetInstance();

  AddressPoolManager(const AddressPoolManager&) = delete;
  AddressPoolManager& operator=(const AddressPoolManager&) = delete;

#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  void Add(pool_handle handle, uintptr_t address, size_t length);
  void Remove(pool_handle handle);

  // Populate a |used| bitset of superpages currently in use.
  void GetPoolUsedSuperPages(pool_handle handle,
                             std::bitset<kMaxSuperPagesInPool>& used);

  // Return the base address of a pool.
  uintptr_t GetPoolBaseAddress(pool_handle handle);
#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  // Reserves address space from the pool.
  uintptr_t Reserve(pool_handle handle,
                    uintptr_t requested_address,
                    size_t length);

  // Frees address space back to the pool and decommits underlying system pages.
  void UnreserveAndDecommit(pool_handle handle,
                            uintptr_t address,
                            size_t length);
  void ResetForTesting();

#if !PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  void MarkUsed(pool_handle handle, uintptr_t address, size_t size);
  void MarkUnused(pool_handle handle, uintptr_t address, size_t size);

  static bool IsManagedByRegularPool(uintptr_t address) {
    return AddressPoolManagerBitmap::IsManagedByRegularPool(address);
  }

  static bool IsManagedByBRPPool(uintptr_t address) {
    return AddressPoolManagerBitmap::IsManagedByBRPPool(address);
  }
#endif  // !PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  void DumpStats(AddressSpaceStatsDumper* dumper);

 private:
  friend class AddressPoolManagerForTesting;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  // If we use a thread isolated pool, we need to write-protect its metadata.
  // Allow the function to get access to the pool pointer.
  friend void WriteProtectThreadIsolatedGlobals(ThreadIsolationOption);
#endif

  constexpr AddressPoolManager() = default;
  ~AddressPoolManager() = default;

  // Populates `stats` if applicable.
  // Returns whether `stats` was populated. (They might not be, e.g.
  // if PartitionAlloc is wholly unused in this process.)
  bool GetStats(AddressSpaceStats* stats);

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  // This function just exists to static_assert the layout of the private fields
  // in Pool. It is never called.
  static void AssertThreadIsolatedLayout();
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  class Pool {
   public:
    constexpr Pool() = default;
    ~Pool() = default;

    Pool(const Pool&) = delete;
    Pool& operator=(const Pool&) = delete;

    void Initialize(uintptr_t ptr, size_t length);
    bool IsInitialized();
    void Reset();

    uintptr_t FindChunk(size_t size);
    void FreeChunk(uintptr_t address, size_t size);

    bool TryReserveChunk(uintptr_t address, size_t size);

    void GetUsedSuperPages(std::bitset<kMaxSuperPagesInPool>& used);
    uintptr_t GetBaseAddress();

    void GetStats(PoolStats* stats);

   private:
    // The lock needs to be the first field in this class.
    // We write-protect the pool in the ThreadIsolated case, except that the
    // lock can be used without acquiring write-permission first (via
    // DumpStats()). So instead of protecting the whole variable, we only
    // protect the memory after the lock.
    // See the alignment of ` below.
    Lock lock_;

    // The bitset stores the allocation state of the address pool. 1 bit per
    // super-page: 1 = allocated, 0 = free.
    std::bitset<kMaxSuperPagesInPool> alloc_bitset_ PA_GUARDED_BY(lock_);

    // An index of a bit in the bitset before which we know for sure there all
    // 1s. This is a best-effort hint in the sense that there still may be lots
    // of 1s after this index, but at least we know there is no point in
    // starting the search before it.
    size_t bit_hint_ PA_GUARDED_BY(lock_) = 0;

    size_t total_bits_ = 0;
    uintptr_t address_begin_ = 0;
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
    uintptr_t address_end_ = 0;
#endif

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    friend class AddressPoolManager;
    friend void WriteProtectThreadIsolatedGlobals(ThreadIsolationOption);
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  };

  PA_ALWAYS_INLINE Pool* GetPool(pool_handle handle) {
    PA_DCHECK(kNullPoolHandle < handle && handle <= kNumPools);
    return &pools_[handle - 1];
  }

  // Gets the stats for the pool identified by `handle`, if
  // initialized.
  void GetPoolStats(pool_handle handle, PoolStats* stats);

  // If thread isolation support is enabled, we need to write-protect the
  // isolated pool (which needs to be last). For this, we need to add padding in
  // front of the pools so that the isolated one starts on a page boundary.
  // We also skip the Lock at the beginning of the pool since it needs to be
  // used in contexts where we didn't enable write access to the pool memory.
#if defined(__clang__)
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wzero-length-array"
#endif
  char pad_[PA_THREAD_ISOLATED_ARRAY_PAD_SZ_WITH_OFFSET(
      Pool,
      kNumPools,
      offsetof(Pool, alloc_bitset_))] = {};
#if defined(__clang__)
#pragma clang diagnostic pop
#endif
  Pool pools_[kNumPools];

#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  PA_CONSTINIT static AddressPoolManager singleton_;
};

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_ADDRESS_POOL_MANAGER_H_
