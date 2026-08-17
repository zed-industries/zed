// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ADDRESS_SPACE_H_
#define PARTITION_ALLOC_PARTITION_ADDRESS_SPACE_H_

#include <cstddef>
#include <utility>

#include "partition_alloc/address_pool_manager_types.h"
#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/page_allocator_constants.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/files/platform_file.h"
#include "partition_alloc/partition_alloc_base/notreached.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/thread_isolation/alignment.h"

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
#include "partition_alloc/thread_isolation/thread_isolation.h"
#endif

// The feature is not applicable to 32-bit address space.
#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)

namespace partition_alloc {

namespace internal {

// Manages PartitionAlloc address space, which is split into pools.
// See `glossary.md`.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) PartitionAddressSpace {
 public:
  // Represents pool-specific information about a given address.
  struct PoolInfo {
    pool_handle handle;
    uintptr_t base;
    uintptr_t base_mask;
    uintptr_t offset;
  };

#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
  PA_ALWAYS_INLINE static uintptr_t CorePoolBaseMask() {
    return setup_.core_pool_base_mask_;
  }
#else
  PA_ALWAYS_INLINE static constexpr uintptr_t CorePoolBaseMask() {
    return kCorePoolBaseMask;
  }
#endif

  PA_ALWAYS_INLINE static PoolInfo GetPoolInfo(uintptr_t address) {
    // When USE_BACKUP_REF_PTR is off, BRP pool isn't used.
#if !PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
    PA_DCHECK(!IsInBRPPool(address));
#endif
    pool_handle pool = kNullPoolHandle;
    uintptr_t base = 0;
    uintptr_t base_mask = 0;
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
    if (IsInBRPPool(address)) {
      pool = kBRPPoolHandle;
      base = setup_.brp_pool_base_address_;
      base_mask = CorePoolBaseMask();
    } else
#endif  // PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
      if (IsInRegularPool(address)) {
        pool = kRegularPoolHandle;
        base = setup_.regular_pool_base_address_;
        base_mask = CorePoolBaseMask();
      } else if (IsInConfigurablePool(address)) {
        PA_DCHECK(IsConfigurablePoolInitialized());
        pool = kConfigurablePoolHandle;
        base = setup_.configurable_pool_base_address_;
        base_mask = setup_.configurable_pool_base_mask_;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
      } else if (IsInThreadIsolatedPool(address)) {
        pool = kThreadIsolatedPoolHandle;
        base = setup_.thread_isolated_pool_base_address_;
        base_mask = kThreadIsolatedPoolBaseMask;
#endif
      } else {
        PA_NOTREACHED();
      }
    return PoolInfo{.handle = pool,
                    .base = base,
                    .base_mask = base_mask,
                    .offset = address - base};
  }
  PA_ALWAYS_INLINE static constexpr size_t ConfigurablePoolMaxSize() {
    return kConfigurablePoolMaxSize;
  }
  PA_ALWAYS_INLINE static constexpr size_t ConfigurablePoolMinSize() {
    return kConfigurablePoolMinSize;
  }

  // Initialize pools (except for the configurable one).
  //
  // This function must only be called from the main thread.
  static void Init();
  // Initialize the ConfigurablePool at the given address |pool_base|. It must
  // be aligned to the size of the pool. The size must be a power of two and
  // must be within [ConfigurablePoolMinSize(), ConfigurablePoolMaxSize()].
  //
  // This function must only be called from the main thread.
  static void InitConfigurablePool(uintptr_t pool_base, size_t size);
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  static void InitThreadIsolatedPool(ThreadIsolationOption thread_isolation);
  static void UninitThreadIsolatedPoolForTesting();
#endif
  static void UninitForTesting();
  static void UninitConfigurablePoolForTesting();

  PA_ALWAYS_INLINE static bool IsInitialized() {
    // Either neither or both regular and BRP pool are initialized. The
    // configurable and thread isolated pool are initialized separately.
    if (setup_.regular_pool_base_address_ != kUninitializedPoolBaseAddress) {
      PA_DCHECK(setup_.brp_pool_base_address_ != kUninitializedPoolBaseAddress);
      return true;
    }

    PA_DCHECK(setup_.brp_pool_base_address_ == kUninitializedPoolBaseAddress);
    return false;
  }

  PA_ALWAYS_INLINE static bool IsConfigurablePoolInitialized() {
    return setup_.configurable_pool_base_address_ !=
           kUninitializedPoolBaseAddress;
  }

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  PA_ALWAYS_INLINE static bool IsThreadIsolatedPoolInitialized() {
    return setup_.thread_isolated_pool_base_address_ !=
           kUninitializedPoolBaseAddress;
  }
#endif

  // Returns false for nullptr.
  PA_ALWAYS_INLINE static bool IsInRegularPool(uintptr_t address) {
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    const uintptr_t regular_pool_base_mask = setup_.core_pool_base_mask_;
#else
    constexpr uintptr_t regular_pool_base_mask = kCorePoolBaseMask;
#endif
    return (address & regular_pool_base_mask) ==
           setup_.regular_pool_base_address_;
  }

  PA_ALWAYS_INLINE static uintptr_t RegularPoolBase() {
    return setup_.regular_pool_base_address_;
  }

  // Returns false for nullptr.
  PA_ALWAYS_INLINE static bool IsInBRPPool(uintptr_t address) {
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    const uintptr_t brp_pool_base_mask = setup_.core_pool_base_mask_;
#else
    constexpr uintptr_t brp_pool_base_mask = kCorePoolBaseMask;
#endif
    return (address & brp_pool_base_mask) == setup_.brp_pool_base_address_;
  }

#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  PA_ALWAYS_INLINE static uintptr_t BRPPoolBase() {
    return RegularPoolBase() + CorePoolSize();
  }
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)

  // Checks whether the address belongs to either regular or BRP pool.
  // Returns false for nullptr.
  PA_ALWAYS_INLINE static bool IsInCorePools(uintptr_t address) {
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    const uintptr_t core_pools_base_mask = setup_.glued_pools_base_mask_;
#else
    // The BRP pool is placed at the end of the regular pool, effectively
    // forming one virtual pool of a twice bigger size. Adjust the mask
    // appropriately.
    constexpr uintptr_t core_pools_base_mask = kCorePoolBaseMask << 1;
#endif  // PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    bool ret =
        (address & core_pools_base_mask) == setup_.regular_pool_base_address_;
    PA_DCHECK(ret == (IsInRegularPool(address) || IsInBRPPool(address)));
    return ret;
  }
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
  PA_ALWAYS_INLINE static size_t CorePoolsSize() { return CorePoolSize() * 2; }
#else
  PA_ALWAYS_INLINE static constexpr size_t CorePoolsSize() {
    return CorePoolSize() * 2;
  }
#endif  // PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)

  PA_ALWAYS_INLINE static uintptr_t OffsetInBRPPool(uintptr_t address) {
    PA_DCHECK(IsInBRPPool(address));
    return address - setup_.brp_pool_base_address_;
  }

  // Returns false for nullptr.
  PA_ALWAYS_INLINE static bool IsInConfigurablePool(uintptr_t address) {
    return (address & setup_.configurable_pool_base_mask_) ==
           setup_.configurable_pool_base_address_;
  }

  PA_ALWAYS_INLINE static uintptr_t ConfigurablePoolBase() {
    return setup_.configurable_pool_base_address_;
  }

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  // Returns false for nullptr.
  PA_ALWAYS_INLINE static bool IsInThreadIsolatedPool(uintptr_t address) {
    return (address & kThreadIsolatedPoolBaseMask) ==
           setup_.thread_isolated_pool_base_address_;
  }
#endif

#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  PA_ALWAYS_INLINE static bool IsShadowMetadataEnabledOnRegularPool() {
    return regular_pool_fd_ != base::kInvalidPlatformFile;
  }

  PA_ALWAYS_INLINE static bool IsShadowMetadataEnabledOnBRPPool() {
    return brp_pool_fd_ != base::kInvalidPlatformFile;
  }

  PA_ALWAYS_INLINE static bool IsShadowMetadataEnabledOnConfigurablePool() {
    return configurable_pool_fd_ != base::kInvalidPlatformFile;
  }

  PA_ALWAYS_INLINE static bool IsShadowMetadataEnabled(pool_handle pool) {
    switch (pool) {
      case kRegularPoolHandle:
        return IsShadowMetadataEnabledOnRegularPool();
      case kBRPPoolHandle:
        return IsShadowMetadataEnabledOnBRPPool();
      case kConfigurablePoolHandle:
        return IsShadowMetadataEnabledOnConfigurablePool();
      default:
        return false;
    }
  }

  // To reduce the cost of address conversion (metadata address inside Regular
  // Pool to its shadow metadata address), we will make the size of the address
  // space of shadow metadata the same as `max(regular pool size, brp
  // pool size, configurable pool size)` (only 1 shadow address space. Not 3)
  // So we need to use different offset for metadata of the regular pool's
  // SuperPages and for the brp pool's SuperPages.
  // i.e. |kSystemPageOffsetOfRegularPoolShadow| and
  // |kSystemPageOffsetOfBRPPoolShadow|.
  //
  // i: the index of SystemPage for metadata inside the regular pool's
  // SuperPage.
  //    (currently, the index is 1.)
  //
  //     i-th
  // +------------+
  // | SystemPage | (regular pool)
  // +------------+
  //       \
  //        \ mapping
  //         \
  //      (i+kSystemPageOffsetOfRegularPoolShadow)-th
  //     +------------+
  //     | SystemPage | (shadow)
  //     +------------+
  //
  // (i + kSystemPageOffsetOfRegularPoolShadow)-th SystemPage inside the matched
  // SuperPage inside the shadow pool is used for the metadata.
  static constexpr size_t kSystemPageOffsetOfRegularPoolShadow = 0u;
  static constexpr size_t kSystemPageOffsetOfBRPPoolShadow = 2u;
  static constexpr size_t kSystemPageOffsetOfConfigurablePoolShadow = 4u;

  static size_t CorePoolShadowSize();
  static size_t ConfigurablePoolShadowSize();

  PA_ALWAYS_INLINE static std::ptrdiff_t RegularPoolShadowOffset() {
    return regular_pool_shadow_offset_;
  }

  PA_ALWAYS_INLINE static std::ptrdiff_t BRPPoolShadowOffset() {
    return brp_pool_shadow_offset_;
  }

  PA_ALWAYS_INLINE static std::ptrdiff_t ConfigurablePoolShadowOffset() {
    return configurable_pool_shadow_offset_;
  }

  // TODO(crbug.com/40238514): Confirm we can use kConfigurablePoolMaxSize/4
  // for iOS and confirm iOS EarlyGrey tests pass when the shadow metadata
  // is enabled, since IIRC iOS limits virtual address space too.
  static_assert(
      !PA_BUILDFLAG(IS_IOS),
      "kConfigurablePoolMaxSize is too large to run iOS EarlyGrey tests, "
      "because the test process cannot use an extended virtual address space. "
      "Temporarily disable ShadowMetadata feature on iOS");

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  // Check whether the given |ptr| points to an address inside the address space
  // reserved for the regular and brp shadow. However the result |true| doesn't
  // mean the given |ptr| is valid. Because we don't use the entire address
  // space for the shadow. We only use 2 SystemPageSize() / kSuperPageSize(%)
  // of the space.
  //
  // TODO(crbug.com/40238514) This is an unused function. Start using it in
  // tests and/or in production code.
  PA_ALWAYS_INLINE static bool IsInPoolShadow(const void* ptr) {
    uintptr_t ptr_as_uintptr = reinterpret_cast<uintptr_t>(ptr);
    return (pool_shadow_address_ <= ptr_as_uintptr &&
            (ptr_as_uintptr < pool_shadow_address_ + CorePoolSize() ||
             ptr_as_uintptr < pool_shadow_address_ + kConfigurablePoolMaxSize));
  }
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

  static void InitShadowMetadata(PoolHandleMask pool);
  static void MapMetadata(uintptr_t super_page, bool copy_metadata);
  static void UnmapShadowMetadata(uintptr_t super_page, pool_handle pool);
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)

  // PartitionAddressSpace is static_only class.
  PartitionAddressSpace() = delete;
  PartitionAddressSpace(const PartitionAddressSpace&) = delete;
  void* operator new(size_t) = delete;
  void* operator new(size_t, void*) = delete;

 private:
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
  static bool IsIOSTestProcess();

  PA_ALWAYS_INLINE static size_t CorePoolSize() {
    return IsIOSTestProcess() ? kCorePoolSizeForIOSTestProcess : kCorePoolSize;
  }
#else
  // The pool sizes should be as large as maximum whenever possible.
  PA_ALWAYS_INLINE static constexpr size_t CorePoolSize() {
    return kCorePoolSize;
  }
#endif  // PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  PA_ALWAYS_INLINE static constexpr size_t ThreadIsolatedPoolSize() {
    return kThreadIsolatedPoolSize;
  }
#endif

  // On 64-bit systems, PA allocates from several contiguous, mutually disjoint
  // pools. The BRP pool is where all allocations have a BRP ref-count, thus
  // pointers pointing there can use a BRP protection against UaF. Allocations
  // in the other pools don't have that.
  //
  // Pool sizes have to be the power of two. Each pool will be aligned at its
  // own size boundary.
  //
  // NOTE! The BRP pool must be preceded by an inaccessible region. This is to
  // prevent a pointer to the end of a non-BRP-pool allocation from falling into
  // the BRP pool, thus triggering BRP mechanism and likely crashing. This
  // "forbidden zone" can be as small as 1B, but it's simpler to just reserve an
  // allocation granularity unit.
  //
  // The ConfigurablePool is an optional Pool that can be created inside an
  // existing mapping provided by the embedder. This Pool can be used when
  // certain PA allocations must be located inside a given virtual address
  // region. One use case for this Pool is V8 Sandbox, which requires that
  // ArrayBuffers be located inside of it.
  static constexpr size_t kCorePoolSize = kPoolMaxSize;
  static_assert(base::bits::HasSingleBit(kCorePoolSize));
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  static constexpr size_t kThreadIsolatedPoolSize = kGiB / 4;
  static_assert(base::bits::HasSingleBit(kThreadIsolatedPoolSize));
#endif
  static constexpr size_t kConfigurablePoolMaxSize = kPoolMaxSize;
  static constexpr size_t kConfigurablePoolMinSize = 1 * kGiB;
  static_assert(kConfigurablePoolMinSize <= kConfigurablePoolMaxSize);
  static_assert(base::bits::HasSingleBit(kConfigurablePoolMaxSize));
  static_assert(base::bits::HasSingleBit(kConfigurablePoolMinSize));

#if PA_BUILDFLAG(IS_IOS)

#if !PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
#error iOS is only supported with a dynamically sized GigaCase.
#endif

  // We can't afford pool sizes as large as kPoolMaxSize in iOS EarlGrey tests,
  // since the test process cannot use an extended virtual address space (see
  // crbug.com/1250788).
  static constexpr size_t kCorePoolSizeForIOSTestProcess = kGiB / 4;
  static_assert(kCorePoolSizeForIOSTestProcess < kCorePoolSize);
  static_assert(base::bits::HasSingleBit(kCorePoolSizeForIOSTestProcess));
#endif  // PA_BUILDFLAG(IOS_IOS)

#if !PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
  // Masks used to easy determine belonging to a pool.
  static constexpr uintptr_t kCorePoolOffsetMask =
      static_cast<uintptr_t>(kCorePoolSize) - 1;
  static constexpr uintptr_t kCorePoolBaseMask = ~kCorePoolOffsetMask;
#endif  // !PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  static constexpr uintptr_t kThreadIsolatedPoolOffsetMask =
      static_cast<uintptr_t>(kThreadIsolatedPoolSize) - 1;
  static constexpr uintptr_t kThreadIsolatedPoolBaseMask =
      ~kThreadIsolatedPoolOffsetMask;
#endif

  // This must be set to such a value that IsIn*Pool() always returns false when
  // the pool isn't initialized.
  static constexpr uintptr_t kUninitializedPoolBaseAddress =
      static_cast<uintptr_t>(-1);

  struct alignas(kPartitionCachelineSize) PA_THREAD_ISOLATED_ALIGN PoolSetup {
    // Before PartitionAddressSpace::Init(), no allocation are allocated from a
    // reserved address space. Therefore, set *_pool_base_address_ initially to
    // -1, so that PartitionAddressSpace::IsIn*Pool() always returns false.
    constexpr PoolSetup() = default;

    // Using a struct to enforce alignment and padding
    uintptr_t regular_pool_base_address_ = kUninitializedPoolBaseAddress;
    uintptr_t brp_pool_base_address_ = kUninitializedPoolBaseAddress;
    uintptr_t configurable_pool_base_address_ = kUninitializedPoolBaseAddress;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    uintptr_t thread_isolated_pool_base_address_ =
        kUninitializedPoolBaseAddress;
#endif
#if PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    uintptr_t core_pool_base_mask_ = 0;
    uintptr_t glued_pools_base_mask_ = 0;
#endif  // PA_CONFIG(DYNAMICALLY_SELECT_POOL_SIZE)
    uintptr_t configurable_pool_base_mask_ = 0;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
    ThreadIsolationOption thread_isolation_;
#endif
  };
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  static_assert(sizeof(PoolSetup) % SystemPageSize() == 0,
                "PoolSetup has to fill a page(s)");
#else
  static_assert(sizeof(PoolSetup) % kPartitionCachelineSize == 0,
                "PoolSetup has to fill a cacheline(s)");
#endif

  // See the comment describing the address layout above.
  //
  // These are write-once fields, frequently accessed thereafter. Make sure they
  // don't share a cacheline with other, potentially writeable data, through
  // alignment and padding.
  PA_CONSTINIT static PoolSetup setup_;

#if PA_CONFIG(ENABLE_SHADOW_METADATA)
  static std::ptrdiff_t regular_pool_shadow_offset_;
  static std::ptrdiff_t brp_pool_shadow_offset_;
  static std::ptrdiff_t configurable_pool_shadow_offset_;
  static base::PlatformFile regular_pool_fd_;
  static base::PlatformFile brp_pool_fd_;
  static base::PlatformFile configurable_pool_fd_;
  static uintptr_t pool_shadow_address_;
#endif  // PA_CONFIG(ENABLE_SHADOW_METADATA)

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  // If we use thread isolation, we need to write-protect its metadata.
  // Allow the function to get access to the PoolSetup.
  friend void WriteProtectThreadIsolatedGlobals(ThreadIsolationOption);
#endif
};

PA_ALWAYS_INLINE PartitionAddressSpace::PoolInfo GetPoolInfo(
    uintptr_t address) {
  return PartitionAddressSpace::GetPoolInfo(address);
}

PA_ALWAYS_INLINE pool_handle GetPool(uintptr_t address) {
  return GetPoolInfo(address).handle;
}

PA_ALWAYS_INLINE uintptr_t OffsetInBRPPool(uintptr_t address) {
  return PartitionAddressSpace::OffsetInBRPPool(address);
}

}  // namespace internal

// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAlloc(uintptr_t address) {
  // When ENABLE_BACKUP_REF_PTR_SUPPORT is off, BRP pool isn't used.
#if !PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
  PA_DCHECK(!internal::PartitionAddressSpace::IsInBRPPool(address));
#endif

  return internal::PartitionAddressSpace::IsInCorePools(address)
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
         || internal::PartitionAddressSpace::IsInThreadIsolatedPool(address)
#endif
         || internal::PartitionAddressSpace::IsInConfigurablePool(address);
}

// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAllocRegularPool(uintptr_t address) {
  return internal::PartitionAddressSpace::IsInRegularPool(address);
}

// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAllocBRPPool(uintptr_t address) {
  return internal::PartitionAddressSpace::IsInBRPPool(address);
}

// Checks whether the address belongs to either regular or BRP pool.
// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAllocCorePools(uintptr_t address) {
  return internal::PartitionAddressSpace::IsInCorePools(address);
}

// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAllocConfigurablePool(
    uintptr_t address) {
  return internal::PartitionAddressSpace::IsInConfigurablePool(address);
}

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
// Returns false for nullptr.
PA_ALWAYS_INLINE bool IsManagedByPartitionAllocThreadIsolatedPool(
    uintptr_t address) {
  return internal::PartitionAddressSpace::IsInThreadIsolatedPool(address);
}
#endif

PA_ALWAYS_INLINE bool IsConfigurablePoolAvailable() {
  return internal::PartitionAddressSpace::IsConfigurablePoolInitialized();
}

}  // namespace partition_alloc

#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS)

#endif  // PARTITION_ALLOC_PARTITION_ADDRESS_SPACE_H_
