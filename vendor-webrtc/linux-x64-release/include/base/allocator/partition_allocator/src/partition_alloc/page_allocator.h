// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PAGE_ALLOCATOR_H_
#define PARTITION_ALLOC_PAGE_ALLOCATOR_H_

#include <cstddef>
#include <cstdint>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/page_allocator_constants.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/thread_isolation/thread_isolation.h"

namespace partition_alloc {

struct PageAccessibilityConfiguration {
  enum Permissions {
    kInaccessible,
    // This flag is valid only with AllocPages(), where in creates kInaccessible
    // pages that may later be re-mapped as executable, on platforms which
    // distinguish never-executable and maybe-executable pages.
    kInaccessibleWillJitLater,
    kRead,
    kReadWrite,
    // This flag is mapped to kReadWrite on systems that
    // don't support MTE.
    kReadWriteTagged,
    // This flag is mapped to kReadExecute on systems
    // that don't support Arm's BTI.
    kReadExecuteProtected,
    kReadExecute,
    // This flag is mapped to `kReadWriteExecute` on systems that do not support
    // Arm's BTI.
    kReadWriteExecuteProtected,
    // This flag is deprecated and will go away soon.
    // TODO(bbudge) Remove this as soon as V8 doesn't need RWX pages.
    kReadWriteExecute,
  };

#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  constexpr explicit PageAccessibilityConfiguration(Permissions permissions)
      : permissions(permissions) {}
  constexpr PageAccessibilityConfiguration(
      Permissions permissions,
      ThreadIsolationOption thread_isolation)
      : permissions(permissions), thread_isolation(thread_isolation) {}
#else
  constexpr explicit PageAccessibilityConfiguration(Permissions permissions)
      : permissions(permissions) {}
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)

  Permissions permissions;
#if PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
  // Tag the page with a Memory Protection Key. Use 0 for none.
  ThreadIsolationOption thread_isolation;
#endif  // PA_BUILDFLAG(ENABLE_THREAD_ISOLATION)
};

// Use for De/RecommitSystemPages API.
enum class PageAccessibilityDisposition {
  // Enforces permission update (Decommit will set to
  // PageAccessibilityConfiguration::kInaccessible;
  // Recommit will set to whatever was requested, other than
  // PageAccessibilityConfiguration::kInaccessible).
  kRequireUpdate,
  // Will not update permissions, if the platform supports that (POSIX & Fuchsia
  // only).
  kAllowKeepForPerf,
};

// Some platforms (including macOS and some Linux-based ones) support tagged
// memory regions, to help in debugging. On Android, these tags are used to name
// anonymous mappings.
//
// kChromium is the default value, used to distinguish general
// Chromium-originated allocations from other ones (e.g. from platform
// libraries).
enum class PageTag {
  kSimulation = 251,      // Memory simulator tool.
  kBlinkGC = 252,         // Blink GC pages.
  kPartitionAlloc = 253,  // PartitionAlloc, no matter the partition.
  kChromium = 254,        // Chromium page.
  kV8 = 255,              // V8 heap pages.

  kFirst = kSimulation,  // Minimum tag value.
  kLast = kV8            // Maximum tag value.
};

// See
// https://github.com/apple-oss-distributions/xnu/blob/5c2921b07a2480ab43ec66f5b9e41cb872bc554f/osfmk/mach/vm_statistics.h#L687
static_assert(static_cast<int>(PageTag::kLast) >= 240,
              "The first application-reserved tag on macOS is 240, see "
              "vm_statistics.h in XNU.");
static_assert(
    static_cast<int>(PageTag::kLast) < 256,
    "Tags are only 1 byte long on macOS, see vm_statistics.h in XNU.");

PA_COMPONENT_EXPORT(PARTITION_ALLOC)
uintptr_t NextAlignedWithOffset(uintptr_t ptr,
                                uintptr_t alignment,
                                uintptr_t requested_offset);

// Allocates one or more pages.
//
// The requested |address| is just a hint; the actual address returned may
// differ. The returned address will be aligned to |align_offset| modulo |align|
// bytes.
//
// |length|, |align| and |align_offset| are in bytes, and must be a multiple of
// |PageAllocationGranularity()|. |length| and |align| must be non-zero.
// |align_offset| must be less than |align|. |align| must be a power of two.
//
// If |address| is 0/nullptr, then a suitable and randomized address will be
// chosen automatically.
//
// |accessibility| controls the permission of the allocated pages.
// PageAccessibilityConfiguration::kInaccessible means uncommitted.
//
// |page_tag| is used on some platforms to identify the source of the
// allocation.
//
// |file_descriptor_for_shared_alloc| is only used in mapping the shadow
// pools to the same physical address as the real one in
// PartitionAddressSpace::Init(). It should be ignored in other cases.
//
// This call will return 0/nullptr if the allocation cannot be satisfied.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
uintptr_t AllocPages(size_t length,
                     size_t align,
                     PageAccessibilityConfiguration accessibility,
                     PageTag page_tag = PageTag::kChromium,
                     int file_descriptor_for_shared_alloc = -1);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
uintptr_t AllocPages(uintptr_t address,
                     size_t length,
                     size_t align,
                     PageAccessibilityConfiguration accessibility,
                     PageTag page_tag = PageTag::kChromium);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void* AllocPages(void* address,
                 size_t length,
                 size_t align,
                 PageAccessibilityConfiguration accessibility,
                 PageTag page_tag = PageTag::kChromium);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
uintptr_t AllocPagesWithAlignOffset(
    uintptr_t address,
    size_t length,
    size_t align,
    size_t align_offset,
    PageAccessibilityConfiguration page_accessibility,
    PageTag page_tag = PageTag::kChromium,
    int file_descriptor_for_shared_alloc = -1);

// Frees one or more pages starting at |address| and continuing for |length|
// bytes.
//
// |address| and |length| must match a previous call to |AllocPages|. Therefore,
// |address| must be aligned to |PageAllocationGranularity()| bytes, and
// |length| must be a multiple of |PageAllocationGranularity()|.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void FreePages(uintptr_t address, size_t length);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void FreePages(void* address, size_t length);

// Marks one or more system pages, starting at |address| with the given
// |page_accessibility|. |length| must be a multiple of |SystemPageSize()|
// bytes.
//
// Returns true if the permission change succeeded. In most cases you must
// |CHECK| the result.
//
// Note: On Windows, setting permissions to `PAGE_NOACCESS` will also decommit
// pages. This is desirable because clients assume that pages with no access
// rights should be "free" from a resource standpoint. In particular this allows
// clients to map a large amount of memory, set its access rights to
// `PAGE_NOACCESS` and not worry about commit limit exhaustion.
// On the flip side, this means that changing permissions can often fail on this
// platform.
[[nodiscard]] PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool TrySetSystemPagesAccess(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration page_accessibility);
[[nodiscard]] PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool TrySetSystemPagesAccess(
    void* address,
    size_t length,
    PageAccessibilityConfiguration page_accessibility);

// Marks one or more system pages, starting at |address| with the given
// |page_accessibility|. |length| must be a multiple of |SystemPageSize()|
// bytes.
//
// Performs a CHECK that the operation succeeds.
//
// See the note above for Windows-specific behavior.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetSystemPagesAccess(uintptr_t address,
                          size_t length,
                          PageAccessibilityConfiguration page_accessibility);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetSystemPagesAccess(void* address,
                          size_t length,
                          PageAccessibilityConfiguration page_accessibility);

// Decommits one or more system pages starting at |address| and continuing for
// |length| bytes. |address| and |length| must be aligned to a system page
// boundary.
//
// This API will crash if the operation cannot be performed!
//
// If disposition is PageAccessibilityDisposition::kRequireUpdate (recommended),
// the decommitted pages will be made inaccessible before the call returns.
// While it is always a programming error to access decommitted pages without
// first recommitting them, callers may use
// PageAccessibilityDisposition::kAllowKeepForPerf to allow the implementation
// to skip changing permissions (use with care), for performance reasons (see
// crrev.com/c/2567282 and crrev.com/c/2563038 for perf regressions encountered
// in the past). Implementations may choose to always modify permissions, hence
// accessing those pages may or may not trigger a fault.
//
// Decommitting means that physical resources (RAM or swap/pagefile) backing the
// allocated virtual address range may be released back to the system, but the
// address space is still allocated to the process (possibly using up page table
// entries or other accounting resources). There is no guarantee that the pages
// are zeroed, unless |DecommittedMemoryIsAlwaysZeroed()| is true.
//
// This operation may not be atomic on some platforms.
//
// Note: "Committed memory" is a Windows Memory Subsystem concept that ensures
// processes will not fault when touching a committed memory region. There is
// no analogue in the POSIX & Fuchsia memory API where virtual memory pages are
// best-effort allocated resources on the first touch. If
// PageAccessibilityDisposition::kRequireUpdate disposition is used, this API
// behaves in a platform-agnostic way by simulating the Windows "decommit" state
// by both discarding the region (allowing the OS to avoid swap operations)
// *and* changing the page protections so accesses fault.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void DecommitSystemPages(
    uintptr_t address,
    size_t length,
    PageAccessibilityDisposition accessibility_disposition);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void DecommitSystemPages(
    void* address,
    size_t length,
    PageAccessibilityDisposition accessibility_disposition);

// Decommits one or more system pages starting at |address| and continuing for
// |length| bytes. |address| and |length| must be aligned to a system page
// boundary.
//
// In contrast to |DecommitSystemPages|, this API guarantees that the pages are
// zeroed and will always mark the region as inaccessible (the equivalent of
// setting them to PageAccessibilityConfiguration::kInaccessible).
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool DecommitAndZeroSystemPages(uintptr_t address,
                                size_t length,
                                PageTag page_tag = PageTag::kChromium);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool DecommitAndZeroSystemPages(void* address,
                                size_t length,
                                PageTag page_tag = PageTag::kChromium);

// Whether decommitted memory is guaranteed to be zeroed when it is
// recommitted. Do not assume that this will not change over time.
constexpr PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) bool DecommittedMemoryIsAlwaysZeroed() {
#if PA_BUILDFLAG(IS_APPLE)
  return false;
#else
  return true;
#endif
}

// (Re)Commits one or more system pages, starting at |address| and continuing
// for |length| bytes with the given |page_accessibility| (must not be
// PageAccessibilityConfiguration::kInaccessible). |address| and |length|
// must be aligned to a system page boundary.
//
// This API will crash if the operation cannot be performed!
//
// If disposition is PageAccessibilityConfiguration::kRequireUpdate, the calls
// updates the pages to |page_accessibility|. This can be used regardless of
// what disposition was used to decommit the pages.
// PageAccessibilityConfiguration::kAllowKeepForPerf allows the implementation
// to leave the page permissions, if that improves performance. This option can
// only be used if the pages were previously accessible and decommitted with
// that same option.
//
// The memory will be zeroed when it is committed for the first time. However,
// there is no such guarantee when memory is recommitted, unless
// |DecommittedMemoryIsAlwaysZeroed()| is true.
//
// This operation may not be atomic on some platforms.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void RecommitSystemPages(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration page_accessibility,
    PageAccessibilityDisposition accessibility_disposition);

// Like RecommitSystemPages(), but returns false instead of crashing.
[[nodiscard]] PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool TryRecommitSystemPages(
    uintptr_t address,
    size_t length,
    PageAccessibilityConfiguration page_accessibility,
    PageAccessibilityDisposition accessibility_disposition);

// Discard one or more system pages starting at |address| and continuing for
// |length| bytes. |length| must be a multiple of |SystemPageSize()|.
//
// Discarding is a hint to the system that the page is no longer required. The
// hint may:
//   - Do nothing.
//   - Discard the page immediately, freeing up physical pages.
//   - Discard the page at some time in the future in response to memory
//   pressure.
//
// Only committed pages should be discarded. Discarding a page does not decommit
// it, and it is valid to discard an already-discarded page. A read or write to
// a discarded page will not fault.
//
// Reading from a discarded page may return the original page content, or a page
// full of zeroes.
//
// Writing to a discarded page is the only guaranteed way to tell the system
// that the page is required again. Once written to, the content of the page is
// guaranteed stable once more. After being written to, the page content may be
// based on the original page content, or a page of zeroes.
//
// WARNING: Do not discard a large amount of pages, for a potentially long
// duration. Discarded pages are *not* decommitted on Windows, where total
// system-wide committed memory is limited. As most Chromium OOM crashes are
// commit limit related, this will both impact Private Memory Footprint (which
// reports committed memory) and stability (since we will bump into the limit
// more often).
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void DiscardSystemPages(uintptr_t address, size_t length);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void DiscardSystemPages(void* address, size_t length);

// Seal a number of system pages starting at |address|. Returns |true| on
// success.
//
// This blocks various modifications to the pages such as unmapping, remapping
// or changing page permissions. Note that it doesn't change the accessibility
// of the memory, sealed writable pages will still be writable.
//
// This is mainly useful for non-writable memory (either via page permissions or
// other hardware features like pkeys) that is bound to the process lifetime.
//
// While unmapping the pages gets blocked, it can still be possible to release
// the memory using |DiscardSystemPages()|, though note that at least on Linux,
// it requires write access to the page to succeed.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool SealSystemPages(uintptr_t address, size_t length);
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
bool SealSystemPages(void* address, size_t length);

// Rounds up |address| to the next multiple of |SystemPageSize()|. Returns
// 0 for an |address| of 0.
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
RoundUpToSystemPage(uintptr_t address) {
  return (address + internal::SystemPageOffsetMask()) &
         internal::SystemPageBaseMask();
}

// Rounds down |address| to the previous multiple of |SystemPageSize()|. Returns
// 0 for an |address| of 0.
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
RoundDownToSystemPage(uintptr_t address) {
  return address & internal::SystemPageBaseMask();
}

// Rounds up |address| to the next multiple of |PageAllocationGranularity()|.
// Returns 0 for an |address| of 0.
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
RoundUpToPageAllocationGranularity(uintptr_t address) {
  return (address + internal::PageAllocationGranularityOffsetMask()) &
         internal::PageAllocationGranularityBaseMask();
}

// Rounds down |address| to the previous multiple of
// |PageAllocationGranularity()|. Returns 0 for an |address| of 0.
PA_ALWAYS_INLINE PAGE_ALLOCATOR_CONSTANTS_DECLARE_CONSTEXPR uintptr_t
RoundDownToPageAllocationGranularity(uintptr_t address) {
  return address & internal::PageAllocationGranularityBaseMask();
}

// Reserves (at least) |size| bytes of address space, aligned to
// |PageAllocationGranularity()|. This can be called early on to make it more
// likely that large allocations will succeed. Returns true if the reservation
// succeeded, false if the reservation failed or a reservation was already made.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool ReserveAddressSpace(size_t size);

// Releases any reserved address space. |AllocPages| calls this automatically on
// an allocation failure. External allocators may also call this on failure.
//
// Returns true when an existing reservation was released.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool ReleaseReservation();

// Returns true if there is currently an address space reservation.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) bool HasReservationForTesting();

// Returns |errno| (POSIX) or the result of |GetLastError| (Windows) when |mmap|
// (POSIX) or |VirtualAlloc| (Windows) fails.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) uint32_t GetAllocPageErrorCode();

// Returns the total amount of mapped pages from all clients of
// PageAllocator. These pages may or may not be committed. This is mostly useful
// to assess address space pressure.
PA_COMPONENT_EXPORT(PARTITION_ALLOC) size_t GetTotalMappedSize();

#if PA_BUILDFLAG(IS_WIN)
// Sets whether to retry the allocation of pages when a commit failure
// happens. This doesn't cover cases where the system is out of address space,
// or reaches another limit.
PA_COMPONENT_EXPORT(PARTITION_ALLOC)
void SetRetryOnCommitFailure(bool retry_on_commit_failure);
bool GetRetryOnCommitFailure();
#endif  // PA_BUILDFLAG(IS_WIN)

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_PAGE_ALLOCATOR_H_
