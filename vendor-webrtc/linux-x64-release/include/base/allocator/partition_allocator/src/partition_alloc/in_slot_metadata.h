// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_IN_SLOT_METADATA_H_
#define PARTITION_ALLOC_IN_SLOT_METADATA_H_

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <limits>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/dangling_raw_ptr_checks.h"
#include "partition_alloc/partition_alloc_base/bits.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/immediate_crash.h"
#include "partition_alloc/partition_alloc_check.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/tagging.h"

namespace partition_alloc::internal {

// Aligns up (on 8B boundary) `in_slot_metadata_size` on Mac as a workaround for
// crash. Workaround was introduced for MacOS 13: https://crbug.com/1378822. But
// it has been enabled by default because MacOS 14 and later seems to need it
// too. https://crbug.com/1457756
// Enabled on iOS as a workaround for a speculative bug in Swift's
// __StringStorage.create https://crbug.com/327804972
//
// Placed outside `PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)`
// intentionally to accommodate usage in contexts also outside
// this gating.
PA_ALWAYS_INLINE constexpr size_t AlignUpInSlotMetadataSizeForApple(
    size_t in_slot_metadata_size) {
#if PA_BUILDFLAG(IS_APPLE)
  return base::bits::AlignUp<size_t>(in_slot_metadata_size, 8);
#else
  return in_slot_metadata_size;
#endif  // PA_BUILDFLAG(IS_APPLE)
}

#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)

// Utility functions to define a bit field.
template <typename CountType>
static constexpr CountType SafeShift(CountType lhs, int rhs) {
  return rhs >= std::numeric_limits<CountType>::digits ? 0 : lhs << rhs;
}
template <typename CountType>
struct BitField {
  static constexpr CountType None() { return CountType(0); }
  static constexpr CountType Bit(int n_th) {
    return SafeShift<CountType>(1, n_th);
  }
  // Mask with bits between `lo` and `hi` (both inclusive) set.
  static constexpr CountType Mask(int lo, int hi) {
    return (SafeShift<CountType>(1, hi + 1) - 1) &
           ~(SafeShift<CountType>(1, lo) - 1);
  }
};

// Special-purpose atomic bit field class mainly used by RawPtrBackupRefImpl.
// Formerly known as `PartitionRefCount`, but renamed to support usage that is
// unrelated to BRP.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) InSlotMetadata {
 public:
  // This class holds an atomic 32 bits field: `count_`. It holds 4 values:
  //
  // bits   name                   description
  // -----  ---------------------  ----------------------------------------
  // 0      is_allocated           Whether or not the memory is held by the
  //                               allocator.
  //                               - 1 at construction time.
  //                               - Decreased in ReleaseFromAllocator();
  //                               - We check whether this bit is set in
  //                                 `ReleaseFromAllocator()`, and if not we
  //                                 have a double-free.
  //
  // 1-29   ptr_count              Number of raw_ptr<T>.
  //                               - Increased in Acquire()
  //                               - Decreased in Release()
  //
  // 30     request_quarantine     When set, PA will quarantine the memory in
  //                               Scheduler-Loop quarantine.
  //                               It also extends quarantine duration when
  //                               set after being quarantined.
  // 31     needs_mac11_malloc_    Whether malloc_size() return value needs to
  //          size_hack            be adjusted for this allocation.
  //
  // On `PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)` builds, it holds two more
  // entries in total of 64 bits.
  //
  // bits   name                   description
  // -----  ---------------------  ----------------------------------------
  // 0      is_allocated
  // 1-31   ptr_count
  //
  // 32     dangling_detected      A dangling raw_ptr<> has been detected.
  // 33     needs_mac11_malloc_
  //          size_hack
  // 34     request_quarantine
  //
  // 35-63  unprotected_ptr_count  Number of
  //                               raw_ptr<T, DisableDanglingPtrDetection>
  //                               - Increased in AcquireFromUnprotectedPtr().
  //                               - Decreased in ReleaseFromUnprotectedPtr().
  //
  // The allocation is reclaimed if all of:
  // - |is_allocated|
  // - |ptr_count|
  // - |unprotected_ptr_count|
  // are zero.
  //
  // During ReleaseFromAllocator(), if |ptr_count| is not zero,
  // |dangling_detected| is set and the error is reported via
  // DanglingRawPtrDetected(id). The matching DanglingRawPtrReleased(id) will be
  // called when the last raw_ptr<> is released.
#if !PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
  using CountType = uint32_t;
  static constexpr CountType kMemoryHeldByAllocatorBit =
      BitField<CountType>::Bit(0);
  static constexpr CountType kPtrCountMask = BitField<CountType>::Mask(1, 29);
  // The most significant bit of the refcount is reserved to prevent races with
  // overflow detection.
  static constexpr CountType kMaxPtrCount = BitField<CountType>::Mask(1, 28);
  static constexpr CountType kRequestQuarantineBit =
      BitField<CountType>::Bit(30);
  static constexpr CountType kNeedsMac11MallocSizeHackBit =
      BitField<CountType>::Bit(31);
  static constexpr CountType kDanglingRawPtrDetectedBit =
      BitField<CountType>::None();
  static constexpr CountType kUnprotectedPtrCountMask =
      BitField<CountType>::None();
#else   // !PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
  using CountType = uint64_t;
  static constexpr auto kMemoryHeldByAllocatorBit = BitField<CountType>::Bit(0);
  static constexpr auto kPtrCountMask = BitField<CountType>::Mask(1, 31);
  // The most significant bit of the refcount is reserved to prevent races with
  // overflow detection.
  static constexpr auto kMaxPtrCount = BitField<CountType>::Mask(1, 30);
  static constexpr auto kDanglingRawPtrDetectedBit =
      BitField<CountType>::Bit(32);
  static constexpr auto kNeedsMac11MallocSizeHackBit =
      BitField<CountType>::Bit(33);
  static constexpr CountType kRequestQuarantineBit =
      BitField<CountType>::Bit(34);
  static constexpr auto kUnprotectedPtrCountMask =
      BitField<CountType>::Mask(35, 63);
  // The most significant bit of the refcount is reserved to prevent races with
  // overflow detection.
  static constexpr auto kMaxUnprotectedPtrCount =
      BitField<CountType>::Mask(35, 62);
#endif  // !PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)

  // Quick check to assert these masks do not overlap.
  static_assert((kMemoryHeldByAllocatorBit + kPtrCountMask +
                 kUnprotectedPtrCountMask + kDanglingRawPtrDetectedBit +
                 kRequestQuarantineBit + kNeedsMac11MallocSizeHackBit) ==
                std::numeric_limits<CountType>::max());

  static constexpr auto kPtrInc =
      SafeShift<CountType>(1, base::bits::CountrZero(kPtrCountMask));
  static constexpr auto kUnprotectedPtrInc =
      SafeShift<CountType>(1, base::bits::CountrZero(kUnprotectedPtrCountMask));

  PA_ALWAYS_INLINE explicit InSlotMetadata(bool needs_mac11_malloc_size_hack);

  // Incrementing the counter doesn't imply any visibility about modified
  // memory, hence relaxed atomics. For decrement, visibility is required before
  // the memory gets freed, necessitating an acquire/release barrier before
  // freeing the memory.
  //
  // For details, see base::AtomicRefCount, which has the same constraints and
  // characteristics.
  //
  // FYI: The assembly produced by the compiler on every platform, in particular
  // the uint64_t fetch_add on 32bit CPU.
  // https://docs.google.com/document/d/1cSTVDVEE-8l2dXLPcfyN75r6ihMbeiSp1ncL9ae3RZE
  PA_ALWAYS_INLINE void Acquire() {
    CheckCookieIfSupported();

    CountType old_count = count_.fetch_add(kPtrInc, std::memory_order_relaxed);
    // Check overflow.
    PA_CHECK((old_count & kPtrCountMask) != kMaxPtrCount);
  }

  // Similar to |Acquire()|, but for raw_ptr<T, DisableDanglingPtrDetection>
  // instead of raw_ptr<T>.
  PA_ALWAYS_INLINE void AcquireFromUnprotectedPtr() {
#if PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
    CheckCookieIfSupported();
    CountType old_count =
        count_.fetch_add(kUnprotectedPtrInc, std::memory_order_relaxed);
    // Check overflow.
    PA_CHECK((old_count & kUnprotectedPtrCountMask) != kMaxUnprotectedPtrCount);
#else
    Acquire();
#endif
  }

  // Returns true if the allocation should be reclaimed.
  PA_ALWAYS_INLINE bool Release() {
    CheckCookieIfSupported();

    CountType old_count = count_.fetch_sub(kPtrInc, std::memory_order_release);
    // Check underflow.
    PA_DCHECK(old_count & kPtrCountMask);

#if PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
    // If a dangling raw_ptr<> was detected, report it.
    if ((old_count & kDanglingRawPtrDetectedBit) == kDanglingRawPtrDetectedBit)
        [[unlikely]] {
      partition_alloc::internal::DanglingRawPtrReleased(
          reinterpret_cast<uintptr_t>(this));
    }
#endif

    return ReleaseCommon(old_count - kPtrInc);
  }

  // Similar to |Release()|, but for raw_ptr<T, DisableDanglingPtrDetection>
  // instead of raw_ptr<T>.
  PA_ALWAYS_INLINE bool ReleaseFromUnprotectedPtr() {
#if PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
    CheckCookieIfSupported();

    CountType old_count =
        count_.fetch_sub(kUnprotectedPtrInc, std::memory_order_release);
    // Check underflow.
    PA_DCHECK(old_count & kUnprotectedPtrCountMask);

    return ReleaseCommon(old_count - kUnprotectedPtrInc);
#else
    return Release();
#endif
  }

  // `PreReleaseFromAllocator()` performs what `ReleaseFromAllocator()` does
  // partially in a way that supports multiple calls.
  // This function can be used when allocation is sent to quarantine to perform
  // dangling `raw_ptr` checks before quarantine, not after.
  PA_ALWAYS_INLINE void PreReleaseFromAllocator() {
    CheckCookieIfSupported();
    CheckDanglingPointersOnFree(count_.load(std::memory_order_relaxed));
  }

  // Returns true if the allocation should be reclaimed.
  // This function should be called by the allocator during Free().
  PA_ALWAYS_INLINE bool ReleaseFromAllocator(
      uintptr_t slot_start,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span) {
    CheckCookieIfSupported();

    CountType old_count =
        count_.fetch_and(~kMemoryHeldByAllocatorBit, std::memory_order_release);

    // If kMemoryHeldByAllocatorBit was already unset, it indicates a double
    // free, but it could also be caused by a memory corruption. Note, this
    // detection mechanism isn't perfect, because in-slot-metadata can be
    // overwritten by the freelist pointer (or its shadow) for very small slots,
    // thus masking the error away.
    if (!(old_count & kMemoryHeldByAllocatorBit)) [[unlikely]] {
      DoubleFreeOrCorruptionDetected(old_count, slot_start, slot_span);
    }

    // Release memory when no raw_ptr<> exists anymore:
    static constexpr CountType mask = kPtrCountMask | kUnprotectedPtrCountMask;
    if ((old_count & mask) == 0) [[likely]] {
      std::atomic_thread_fence(std::memory_order_acquire);
      // The allocation is about to get freed, so clear the cookie.
      ClearCookieIfSupported();
      return true;
    }

    CheckDanglingPointersOnFree(old_count);
    return false;
  }

  // "IsAlive" means is allocated and not freed. "KnownRefs" refers to
  // raw_ptr<T> references. There may be other references from raw pointers or
  // unique_ptr, but we have no way of tracking them, so we hope for the best.
  // To summarize, the function returns whether we believe the allocation can be
  // safely freed.
  PA_ALWAYS_INLINE bool IsAliveWithNoKnownRefs() {
    CheckCookieIfSupported();
    static constexpr CountType mask =
        kMemoryHeldByAllocatorBit | kPtrCountMask | kUnprotectedPtrCountMask;
    return (count_.load(std::memory_order_acquire) & mask) ==
           kMemoryHeldByAllocatorBit;
  }

  PA_ALWAYS_INLINE bool IsAlive() {
    bool alive =
        count_.load(std::memory_order_relaxed) & kMemoryHeldByAllocatorBit;
    if (alive) {
      CheckCookieIfSupported();
    }
    return alive;
  }

  // Assertion to allocation which ought to be alive.
  PA_ALWAYS_INLINE void EnsureAlive(
      uintptr_t slot_start,
      SlotSpanMetadata<MetadataKind::kReadOnly>* slot_span) {
    CountType count = count_.load(std::memory_order_relaxed);
    if (!(count & kMemoryHeldByAllocatorBit)) {
      DoubleFreeOrCorruptionDetected(count, slot_start, slot_span);
    }
    CheckCookieIfSupported();
  }

  // Called when a raw_ptr is not banning dangling ptrs, but the user still
  // wants to ensure the pointer is not currently dangling. This is currently
  // used in UnretainedWrapper to make sure callbacks are not invoked with
  // dangling pointers. If such a raw_ptr exists but the allocation is no longer
  // alive, then we have a dangling pointer to a dead object.
  PA_ALWAYS_INLINE void ReportIfDangling() {
    if (!IsAlive()) {
      partition_alloc::internal::UnretainedDanglingRawPtrDetected(
          reinterpret_cast<uintptr_t>(this));
    }
  }

  // Request to quarantine this allocation. The request might be ignored if
  // the allocation is already freed.
  // TODO(crbug.com/329027914) This is an unused function. Start using it in
  // tests and/or in production code.
  PA_ALWAYS_INLINE void SetQuarantineRequest() {
    CountType old_count =
        count_.fetch_or(kRequestQuarantineBit, std::memory_order_relaxed);
    // This bit cannot be used after the memory is freed.
    PA_DCHECK(old_count & kMemoryHeldByAllocatorBit);
  }

  // Get and clear out quarantine request.
  // TODO(crbug.com/329027914) This is an unused function. Start using it in
  // tests and/or in production code.
  PA_ALWAYS_INLINE bool PopQuarantineRequest() {
    CountType old_count =
        count_.fetch_and(~kRequestQuarantineBit, std::memory_order_acq_rel);
    // This bit cannot be used after the memory is freed.
    PA_DCHECK(old_count & kMemoryHeldByAllocatorBit);
    return old_count & kRequestQuarantineBit;
  }

  // GWP-ASan slots are assigned an extra reference (note `kPtrInc` below) to
  // make sure the `raw_ptr<T>` release operation will never attempt to call the
  // PA `free` on such a slot. GWP-ASan takes the extra reference into account
  // when determining whether the slot can be reused.
  PA_ALWAYS_INLINE void InitializeForGwpAsan() {
#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
    brp_cookie_ = CalculateCookie();
#endif
    count_.store(kPtrInc | kMemoryHeldByAllocatorBit,
                 std::memory_order_release);
  }

  PA_ALWAYS_INLINE bool CanBeReusedByGwpAsan() {
    static constexpr CountType mask = kPtrCountMask | kUnprotectedPtrCountMask;
    return (count_.load(std::memory_order_acquire) & mask) == kPtrInc;
  }

  bool NeedsMac11MallocSizeHack() {
    return count_.load(std::memory_order_relaxed) &
           kNeedsMac11MallocSizeHackBit;
  }

#if PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)
  PA_ALWAYS_INLINE void SetRequestedSize(size_t size) {
    requested_size_ = static_cast<uint32_t>(size);
  }
  PA_ALWAYS_INLINE uint32_t requested_size() const { return requested_size_; }
#endif  // PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)

 private:
  // If there are some dangling raw_ptr<>. Turn on the error flag, and
  // emit the `DanglingPtrDetected` once to embedders.
  PA_ALWAYS_INLINE void CheckDanglingPointersOnFree(CountType count) {
#if PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
    // The `kPtrCountMask` counts the number of raw_ptr<T>. It is expected to be
    // zero when there are no unexpected dangling pointers.
    if ((count & kPtrCountMask) == 0) [[likely]] {
      return;
    }

    // Two events are sent to embedders:
    // 1. `DanglingRawPtrDetected` - Here
    // 2. `DanglingRawPtrReleased` - In Release().
    //
    // The `dangling_detected` bit signals we must emit the second during
    // `Release().
    CountType old_count =
        count_.fetch_or(kDanglingRawPtrDetectedBit, std::memory_order_relaxed);

    // This function supports multiple calls. `DanglingRawPtrDetected` must be
    // called only once. So only the first caller setting the bit can continue.
    if ((old_count & kDanglingRawPtrDetectedBit) ==
        kDanglingRawPtrDetectedBit) {
      return;
    }

    partition_alloc::internal::DanglingRawPtrDetected(
        reinterpret_cast<uintptr_t>(this));
#endif  // PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)
  }

  // The common parts shared by Release() and ReleaseFromUnprotectedPtr().
  // Called after updating the ref counts, |count| is the new value of |count_|
  // set by fetch_sub. Returns true if memory can be reclaimed.
  PA_ALWAYS_INLINE bool ReleaseCommon(CountType count) {
    // Do not release memory, if it is still held by any of:
    // - The allocator
    // - A raw_ptr<T>
    // - A raw_ptr<T, DisableDanglingPtrDetection>
    //
    // Assuming this raw_ptr is not dangling, the memory must still be held at
    // least by the allocator, so this is `[[likely]]`.
    if ((count & (kMemoryHeldByAllocatorBit | kPtrCountMask |
                  kUnprotectedPtrCountMask))) [[likely]] {
      return false;  // Do not release the memory.
    }

    // In most thread-safe reference count implementations, an acquire
    // barrier is required so that all changes made to an object from other
    // threads are visible to its destructor. In our case, the destructor
    // finishes before the final `Release` call, so it shouldn't be a problem.
    // However, we will keep it as a precautionary measure.
    std::atomic_thread_fence(std::memory_order_acquire);

    // The allocation is about to get freed, so clear the cookie.
    ClearCookieIfSupported();
    return true;
  }

  // The cookie helps us ensure that:
  // 1) The reference count pointer calculation is correct.
  // 2) The returned allocation slot is not freed.
  PA_ALWAYS_INLINE void CheckCookieIfSupported() {
#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
    PA_CHECK(brp_cookie_ == CalculateCookie());
#endif
  }

  PA_ALWAYS_INLINE void ClearCookieIfSupported() {
#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
    brp_cookie_ = 0;
#endif
  }

#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
  PA_ALWAYS_INLINE uint32_t CalculateCookie() {
    return static_cast<uint32_t>(reinterpret_cast<uintptr_t>(this)) ^
           kCookieSalt;
  }
#endif  // PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)

  [[noreturn]] PA_NOINLINE PA_NOT_TAIL_CALLED static void
  DoubleFreeOrCorruptionDetected(CountType count,
                                 uintptr_t slot_start,
                                 SlotSpanMetadata<MetadataKind::kReadOnly>*);

  // Note that in free slots, this is overwritten by encoded freelist
  // pointer(s). The way the pointers are encoded on 64-bit little-endian
  // architectures, count_ happens stay even, which works well with the
  // double-free-detection in ReleaseFromAllocator(). Don't change the layout of
  // this class, to preserve this functionality.
  std::atomic<CountType> count_;

#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
  static constexpr uint32_t kCookieSalt = 0xc01dbeef;
  volatile uint32_t brp_cookie_;
#endif

#if PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)
  uint32_t requested_size_;
#endif
};

PA_ALWAYS_INLINE InSlotMetadata::InSlotMetadata(
    bool needs_mac11_malloc_size_hack)
    : count_(kMemoryHeldByAllocatorBit |
             (needs_mac11_malloc_size_hack ? kNeedsMac11MallocSizeHackBit : 0))
#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE)
      ,
      brp_cookie_(CalculateCookie())
#endif
{
}

static_assert(kAlignment % alignof(InSlotMetadata) == 0,
              "kAlignment must be multiples of alignof(InSlotMetadata).");

#if PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)

#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE) || \
    PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)
static constexpr size_t kInSlotMetadataSizeShift = 4;
#else
static constexpr size_t kInSlotMetadataSizeShift = 3;
#endif

#else  // PA_BUILDFLAG(ENABLE_DANGLING_RAW_PTR_CHECKS)

#if PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE) && \
    PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)
static constexpr size_t kInSlotMetadataSizeShift = 4;
#elif PA_CONFIG(IN_SLOT_METADATA_CHECK_COOKIE) || \
    PA_CONFIG(IN_SLOT_METADATA_STORE_REQUESTED_SIZE)
static constexpr size_t kInSlotMetadataSizeShift = 3;
#else
static constexpr size_t kInSlotMetadataSizeShift = 2;
#endif

#endif  // PA_CONFIG(ENABLE_DANGLING_RAW_PTR_CHECKS)
static_assert((1 << kInSlotMetadataSizeShift) == sizeof(InSlotMetadata));

// The in-slot metadata table is tucked in the metadata region of the super
// page, and spans a single system page.
//
// We need one InSlotMetadata for each data system page in a super page. They
// take `x = sizeof(InSlotMetadata) * (kSuperPageSize / SystemPageSize())`
// space. They need to fit into a system page of metadata as sparsely as
// possible to minimize cache line sharing, hence we calculate a multiplier as
// `SystemPageSize() / x` which is equal to
// `SystemPageSize()^2 / kSuperPageSize / sizeof(InSlotMetadata)`.
//
// The multiplier is expressed as a bitshift to optimize the code generation.
// SystemPageSize() isn't always a constrexpr, in which case the compiler
// wouldn't know it's a power of two. The equivalence of these calculations is
// checked in PartitionAllocGlobalInit().
PA_ALWAYS_INLINE static PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR size_t
GetInSlotMetadataIndexMultiplierShift() {
  return SystemPageShift() * 2 - kSuperPageShift - kInSlotMetadataSizeShift;
}

PA_ALWAYS_INLINE InSlotMetadata* InSlotMetadataPointer(uintptr_t slot_start,
                                                       size_t slot_size) {
  // In-slot metadata is typically put at the end of the slot. However, there
  // are a handful of issues that need to be considered:
  // 1. GWP-ASan uses 2-page slots and wants the 2nd page to be inaccissable, so
  //    putting an in-slot metadata there is a no-go.
  // 2. When direct map is reallocated in-place, it's `slot_size` may change and
  //    pages can be (de)committed. This would force in-slot metadata
  //    relocation, which could lead to a race with the metadata access.
  // 3. For single-slot spans, the unused pages between `GetUtilizedSlotSize()`
  //    and `slot_size` may be discarded thus interfering with the in-slot
  //    metadata.
  //
  // All of the above happen to have `slot_start` at the page boundary. We place
  // the InSlotMetadata object out-of-line in this case, specifically in a
  // special table after the super page metadata (see InSlotMetadataTable in
  // partition_alloc_constants.h).
  if (slot_start & SystemPageOffsetMask()) [[likely]] {
    uintptr_t refcount_address =
        slot_start + slot_size - sizeof(InSlotMetadata);
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SLOW_CHECKS)
    PA_CHECK(refcount_address % alignof(InSlotMetadata) == 0);
#endif
    // TODO(bartekn): Plumb the tag from the callers, so that MTE tag can be
    // included in the pointer arithmetic, and not re-read from memory.
    return static_cast<InSlotMetadata*>(TagAddr(refcount_address));
  } else {
    // No need to MTE-tag, as the metadata region isn't protected by MTE.
    InSlotMetadata* table_base = reinterpret_cast<InSlotMetadata*>(
        (slot_start & kSuperPageBaseMask) + SystemPageSize() * 2);
    size_t index = ((slot_start & kSuperPageOffsetMask) >> SystemPageShift())
                   << GetInSlotMetadataIndexMultiplierShift();
#if PA_BUILDFLAG(DCHECKS_ARE_ON) || \
    PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SLOW_CHECKS)
    PA_CHECK(sizeof(InSlotMetadata) * index <= SystemPageSize());
#endif
    return table_base + index;
  }
}

#endif  // PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)

static inline constexpr size_t kInSlotMetadataSizeAdjustment =
#if PA_BUILDFLAG(ENABLE_BACKUP_REF_PTR_SUPPORT)
    AlignUpInSlotMetadataSizeForApple(sizeof(InSlotMetadata));
#else
    0ul;
#endif

}  // namespace partition_alloc::internal

#endif  // PARTITION_ALLOC_IN_SLOT_METADATA_H_
