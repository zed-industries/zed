// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_THREAD_CACHE_H_
#define PARTITION_ALLOC_THREAD_CACHE_H_

#include <atomic>
#include <cstdint>
#include <limits>
#include <memory>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/lightweight_quarantine.h"
#include "partition_alloc/partition_alloc-inl.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx_wrapper/optional.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_base/time/time.h"
#include "partition_alloc/partition_alloc_config.h"
#include "partition_alloc/partition_alloc_constants.h"
#include "partition_alloc/partition_alloc_forward.h"
#include "partition_alloc/partition_bucket_lookup.h"
#include "partition_alloc/partition_freelist_entry.h"
#include "partition_alloc/partition_lock.h"
#include "partition_alloc/partition_stats.h"
#include "partition_alloc/partition_tls.h"

#if PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && PA_BUILDFLAG(HAS_64_BIT_POINTERS)
#include "partition_alloc/partition_alloc_base/cxx_wrapper/algorithm.h"
#endif

namespace partition_alloc {

class ThreadCache;

namespace tools {

// This is used from ThreadCacheInspector, which runs in a different process. It
// scans the process memory looking for the two needles, to locate the thread
// cache registry instance.
//
// These two values were chosen randomly, and in particular neither is a valid
// pointer on most 64 bit architectures.
#if PA_BUILDFLAG(HAS_64_BIT_POINTERS)
constexpr uintptr_t kNeedle1 = 0xe69e32f3ad9ea63;
constexpr uintptr_t kNeedle2 = 0x9615ee1c5eb14caf;
#else
constexpr uintptr_t kNeedle1 = 0xe69e32f3;
constexpr uintptr_t kNeedle2 = 0x9615ee1c;
#endif  // PA_BUILDFLAG(HAS_64_BIT_POINTERS)

// This array contains, in order:
// - kNeedle1
// - &ThreadCacheRegistry::Instance()
// - kNeedle2
//
// It is refererenced in the thread cache constructor to make sure it is not
// removed by the compiler. It is also not const to make sure it ends up in
// .data.
constexpr size_t kThreadCacheNeedleArraySize = 4;
extern uintptr_t kThreadCacheNeedleArray[kThreadCacheNeedleArraySize];

class HeapDumper;
class ThreadCacheInspector;

}  // namespace tools

namespace internal {

extern PA_COMPONENT_EXPORT(PARTITION_ALLOC) PartitionTlsKey g_thread_cache_key;

#if PA_CONFIG(THREAD_CACHE_FAST_TLS)
extern PA_COMPONENT_EXPORT(
    PARTITION_ALLOC) thread_local ThreadCache* g_thread_cache;
#endif

}  // namespace internal

constexpr internal::base::TimeDelta kMinPurgeInterval =
    internal::base::Seconds(1);
constexpr internal::base::TimeDelta kMaxPurgeInterval =
    internal::base::Minutes(1);
constexpr internal::base::TimeDelta kDefaultPurgeInterval =
    2 * kMinPurgeInterval;
constexpr size_t kMinCachedMemoryForPurgingBytes = 500 * 1024;

// Global registry of all ThreadCache instances.
//
// This class cannot allocate in the (Un)registerThreadCache() functions, as
// they are called from ThreadCache constructor, which is from within the
// allocator. However the other members can allocate.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) ThreadCacheRegistry {
 public:
  static ThreadCacheRegistry& Instance();
  // Do not instantiate.
  //
  // Several things are surprising here:
  // - The constructor is public even though this is intended to be a singleton:
  //   we cannot use a "static local" variable in |Instance()| as this is
  //   reached too early during CRT initialization on Windows, meaning that
  //   static local variables don't work (as they call into the uninitialized
  //   runtime). To sidestep that, we use a regular global variable in the .cc,
  //   which is fine as this object's constructor is constexpr.
  // - Marked inline so that the chromium style plugin doesn't complain that a
  //   "complex constructor" has an inline body. This warning is disabled when
  //   the constructor is explicitly marked "inline". Note that this is a false
  //   positive of the plugin, since constexpr implies inline.
  inline constexpr ThreadCacheRegistry();

  void RegisterThreadCache(ThreadCache* cache);
  void UnregisterThreadCache(ThreadCache* cache);
  // Prints statistics for all thread caches, or this thread's only.
  void DumpStats(bool my_thread_only, ThreadCacheStats* stats);
  // Purge() this thread's cache, and asks the other ones to trigger Purge() at
  // a later point (during a deallocation).
  void PurgeAll();

  // Runs `PurgeAll` and updates the next interval which
  // `GetPeriodicPurgeNextIntervalInMicroseconds` returns.
  //
  // Note that it's a caller's responsibility to invoke this member function
  // periodically with an appropriate interval. This function does not schedule
  // any task nor timer.
  void RunPeriodicPurge();
  // Returns the appropriate interval to invoke `RunPeriodicPurge` next time.
  int64_t GetPeriodicPurgeNextIntervalInMicroseconds() const;

  // Controls the thread cache size, by setting the multiplier to a value above
  // or below |ThreadCache::kDefaultMultiplier|.
  void SetThreadCacheMultiplier(float multiplier);
  void SetLargestActiveBucketIndex(uint16_t largest_active_bucket_index);

  // Controls the thread cache purging configuration.
  void SetPurgingConfiguration(
      const internal::base::TimeDelta min_purge_interval,
      const internal::base::TimeDelta max_purge_interval,
      const internal::base::TimeDelta default_purge_interval,
      size_t min_cached_memory_for_purging_bytes);
  internal::base::TimeDelta min_purge_interval() const {
    return min_purge_interval_;
  }
  internal::base::TimeDelta max_purge_interval() const {
    return max_purge_interval_;
  }
  internal::base::TimeDelta default_purge_interval() const {
    return default_purge_interval_;
  }
  size_t min_cached_memory_for_purging_bytes() const {
    return min_cached_memory_for_purging_bytes_;
  }
  bool is_purging_configured() const { return is_purging_configured_; }

  static internal::Lock& GetLock() { return Instance().lock_; }
  // Purges all thread caches *now*. This is completely thread-unsafe, and
  // should only be called in a post-fork() handler.
  void ForcePurgeAllThreadAfterForkUnsafe();

  void ResetForTesting();

 private:
  friend class tools::ThreadCacheInspector;
  friend class tools::HeapDumper;

  // Not using base::Lock as the object's constructor must be constexpr.
  internal::Lock lock_;
  ThreadCache* list_head_ PA_GUARDED_BY(GetLock()) = nullptr;
  bool periodic_purge_is_initialized_ = false;
  internal::base::TimeDelta min_purge_interval_;
  internal::base::TimeDelta max_purge_interval_;
  internal::base::TimeDelta default_purge_interval_;
  size_t min_cached_memory_for_purging_bytes_ = 0u;
  internal::base::TimeDelta periodic_purge_next_interval_;
  bool is_purging_configured_ = false;

  uint16_t largest_active_bucket_index_ =
      internal::BucketIndexLookup::GetIndexForNeutralBuckets(
          kThreadCacheDefaultSizeThreshold);
};

constexpr ThreadCacheRegistry::ThreadCacheRegistry() = default;

#if PA_CONFIG(THREAD_CACHE_ENABLE_STATISTICS)
#define PA_INCREMENT_COUNTER(counter) ++counter
#else
#define PA_INCREMENT_COUNTER(counter) \
  do {                                \
  } while (0)
#endif  // PA_CONFIG(THREAD_CACHE_ENABLE_STATISTICS)

#if PA_BUILDFLAG(DCHECKS_ARE_ON)

namespace internal {

class ReentrancyGuard {
 public:
  explicit ReentrancyGuard(bool& flag) : flag_(flag) {
    PA_CHECK(!flag_);
    flag_ = true;
  }

  ~ReentrancyGuard() { flag_ = false; }

 private:
  bool& flag_;
};

}  // namespace internal

#define PA_REENTRANCY_GUARD(x)      \
  internal::ReentrancyGuard guard { \
    x                               \
  }

#else  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_REENTRANCY_GUARD(x) \
  do {                         \
  } while (0)

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

// Per-thread cache. *Not* threadsafe, must only be accessed from a single
// thread.
//
// In practice, this is easily enforced as long as only |instance| is
// manipulated, as it is a thread_local member. As such, any
// |ThreadCache::instance->*()| call will necessarily be done from a single
// thread.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC) ThreadCache {
 public:
  struct Bucket {
    internal::PartitionFreelistEntry* freelist_head = nullptr;
    // Want to keep sizeof(Bucket) small, using small types.
    uint8_t count = 0;
    std::atomic<uint8_t> limit{};  // Can be changed from another thread.
    uint16_t slot_size = 0;

    Bucket();
  };

  // Initializes the thread cache for |root|. May allocate, so should be called
  // with the thread cache disabled on the partition side, and without the
  // partition lock held.
  //
  // May only be called by a single PartitionRoot.
  static void Init(PartitionRoot* root);

  static void DeleteForTesting(ThreadCache* tcache);

  // Deletes existing thread cache and creates a new one for |root|.
  static void SwapForTesting(PartitionRoot* root);

  // Removes the tombstone marker that would be returned by Get() otherwise.
  static void RemoveTombstoneForTesting();

  // Can be called several times, must be called before any ThreadCache
  // interactions.
  static void EnsureThreadSpecificDataInitialized();

  static ThreadCache* Get() {
#if PA_CONFIG(THREAD_CACHE_FAST_TLS)
    return internal::g_thread_cache;
#else
    // This region isn't MTE-tagged.
    return reinterpret_cast<ThreadCache*>(
        internal::PartitionTlsGet(internal::g_thread_cache_key));
#endif
  }

  static bool IsValid(ThreadCache* tcache) {
    // Do not MTE-untag, as it'd mess up the sentinel value.
    return reinterpret_cast<uintptr_t>(tcache) & kTombstoneMask;
  }

  static bool IsTombstone(ThreadCache* tcache) {
    // Do not MTE-untag, as it'd mess up the sentinel value.
    return reinterpret_cast<uintptr_t>(tcache) == kTombstone;
  }

  // Create a new ThreadCache associated with |root|.
  // Must be called without the partition locked, as this may allocate.
  static ThreadCache* Create(PartitionRoot* root);

  const internal::PartitionFreelistDispatcher*
  get_freelist_dispatcher_from_root();

  ~ThreadCache();

  // Disallow copy and move.
  ThreadCache(const ThreadCache&) = delete;
  ThreadCache(const ThreadCache&&) = delete;
  ThreadCache& operator=(const ThreadCache&) = delete;

  // Tries to put a slot at |slot_start| into the cache.
  // The slot comes from the bucket at index |bucket_index| from the partition
  // this cache is for.
  //
  // Returns the slot size if the insertion succeeds, `nullopt` otherwise.
  // Insertion can fail either because the cache is full or the
  // allocation was too large.
  PA_ALWAYS_INLINE std::optional<size_t> MaybePutInCache(uintptr_t slot_start,
                                                         size_t bucket_index);

  // Tries to allocate a memory slot from the cache.
  // Returns 0 on failure.
  //
  // Has the same behavior as RawAlloc(), that is: no cookie nor ref-count
  // handling. Sets |slot_size| to the allocated size upon success.
  PA_ALWAYS_INLINE uintptr_t GetFromCache(size_t bucket_index,
                                          size_t* slot_size);

  // Asks this cache to trigger |Purge()| at a later point. Can be called from
  // any thread.
  void SetShouldPurge();
  // Empties the cache.
  // The Partition lock must *not* be held when calling this.
  // Must be called from the thread this cache is for.
  void Purge();
  // |TryPurge| is the same as |Purge|, except that |TryPurge| will
  // not crash if the thread cache is inconsistent. Normally inconsistency
  // is a sign of a bug somewhere, so |Purge| should be preferred in most cases.
  void TryPurge();
  // Amount of cached memory for this thread's cache, in bytes.
  size_t CachedMemory() const;
  void AccumulateStats(ThreadCacheStats* stats) const;

  // Purge the thread cache of the current thread, if one exists.
  static void PurgeCurrentThread();

  const ThreadAllocStats& thread_alloc_stats() const {
    return thread_alloc_stats_;
  }
  size_t bucket_count_for_testing(size_t index) const {
    return buckets_[index].count;
  }

  internal::base::PlatformThreadId thread_id() const { return thread_id_; }

  // Sets the maximum size of allocations that may be cached by the thread
  // cache. This applies to all threads. However, the maximum size is bounded by
  // |kLargeSizeThreshold|.
  static void SetLargestCachedSize(size_t size);

  // Cumulative stats about *all* allocations made on the `root_` partition on
  // this thread, that is not only the allocations serviced by the thread cache,
  // but all allocations, including large and direct-mapped ones. This should in
  // theory be split into a separate PerThread data structure, but the thread
  // cache is the only per-thread data we have as of now.
  //
  // TODO(lizeb): Investigate adding a proper per-thread data structure.
  PA_ALWAYS_INLINE void RecordAllocation(size_t size);
  PA_ALWAYS_INLINE void RecordDeallocation(size_t size);
  void ResetPerThreadAllocationStatsForTesting();

  // Fill 1 / kBatchFillRatio * bucket.limit slots at a time.
  static constexpr uint16_t kBatchFillRatio = 8;

  // Limit for the smallest bucket will be kDefaultMultiplier *
  // kSmallBucketBaseCount by default.
  static constexpr float kDefaultMultiplier = 2.;
  static constexpr uint8_t kSmallBucketBaseCount = 64;

  static constexpr size_t kDefaultSizeThreshold =
      kThreadCacheDefaultSizeThreshold;
  static constexpr size_t kLargeSizeThreshold = kThreadCacheLargeSizeThreshold;
  static constexpr uint16_t kBucketCount =
      internal::BucketIndexLookup::GetIndexForNeutralBuckets(
          kThreadCacheLargeSizeThreshold) +
      1;
  static_assert(
      kBucketCount < internal::kNumBuckets,
      "Cannot have more cached buckets than what the allocator supports");

  const ThreadCache* prev_for_testing() const
      PA_EXCLUSIVE_LOCKS_REQUIRED(ThreadCacheRegistry::GetLock()) {
    return prev_;
  }
  const ThreadCache* next_for_testing() const
      PA_EXCLUSIVE_LOCKS_REQUIRED(ThreadCacheRegistry::GetLock()) {
    return next_;
  }

  ThreadCacheStats& stats_for_testing() { return stats_; }

  Bucket& bucket_for_testing(size_t index) { return buckets_[index]; }
  void ClearBucketForTesting(Bucket& bucket, size_t limit) {
    ClearBucket(bucket, limit);
  }

  internal::SchedulerLoopQuarantineBranch& GetSchedulerLoopQuarantineBranch() {
    return scheduler_loop_quarantine_branch_;
  }

  // Returns true if the given address is in the thread cache's freelist.
  // Otherwise, returns false.
  bool IsInFreelist(uintptr_t address, size_t bucket_index, size_t& position);

 private:
  friend class tools::HeapDumper;
  friend class tools::ThreadCacheInspector;

  static_assert(sizeof(Bucket) <= 2 * sizeof(void*), "Keep Bucket small.");

  explicit ThreadCache(PartitionRoot* root);
  static void Delete(void* thread_cache_ptr);

  static void* operator new(size_t count);
  static void operator delete(void* ptr);

  void PurgeInternal();
  template <bool crash_on_corruption>
  void PurgeInternalHelper();

  // Fills a bucket from the central allocator.
  void FillBucket(size_t bucket_index);
  // Empties the |bucket| until there are at most |limit| objects in it.
  template <bool crash_on_corruption>
  void ClearBucketHelper(Bucket& bucket, size_t limit);
  void ClearBucket(Bucket& bucket, size_t limit);
  PA_ALWAYS_INLINE void PutInBucket(Bucket& bucket, uintptr_t slot_start);
  void ResetForTesting();
  // Releases the entire freelist starting at |head| to the root.
  template <bool crash_on_corruption>
  void FreeAfter(internal::PartitionFreelistEntry* head, size_t slot_size);
  static void SetGlobalLimits(PartitionRoot* root, float multiplier);

  // On some architectures, ThreadCache::Get() can be called and return
  // something after the thread cache has been destroyed. In this case, we set
  // it to this value, to signal that the thread is being terminated, and the
  // thread cache should not be used.
  //
  // This happens in particular on Windows, during program termination.
  //
  // We choose 0x1 as the value as it is an invalid pointer value, since it is
  // not aligned, and too low. Also, checking !(ptr & kTombstoneMask) checks for
  // nullptr and kTombstone at the same time.
  static constexpr uintptr_t kTombstone = 0x1;
  static constexpr uintptr_t kTombstoneMask = ~kTombstone;

  static uint8_t global_limits_[kBucketCount];
  // Index of the largest active bucket. Not all processes/platforms will use
  // all buckets, as using larger buckets increases the memory footprint.
  //
  // TODO(lizeb): Investigate making this per-thread rather than static, to
  // improve locality, and open the door to per-thread settings.
  static uint16_t largest_active_bucket_index_;

  // These are at the beginning as they're accessed for each allocation.
  uint32_t cached_memory_ = 0;
  std::atomic<bool> should_purge_;
  ThreadCacheStats stats_;
  ThreadAllocStats thread_alloc_stats_;

  // Buckets are quite big, though each is only 2 pointers.
  Bucket buckets_[kBucketCount];

  // Cold data below.
  PartitionRoot* const root_;

  const internal::base::PlatformThreadId thread_id_;
#if PA_BUILDFLAG(DCHECKS_ARE_ON)
  bool is_in_thread_cache_ = false;
#endif

  // Intrusive list since ThreadCacheRegistry::RegisterThreadCache() cannot
  // allocate.
  ThreadCache* next_ PA_GUARDED_BY(ThreadCacheRegistry::GetLock());
  ThreadCache* prev_ PA_GUARDED_BY(ThreadCacheRegistry::GetLock());

  // Thread-Local version of `PartitionRoot::scheduler_loop_quarantine_branch_`.
  internal::SchedulerLoopQuarantineBranch scheduler_loop_quarantine_branch_;

  friend class ThreadCacheRegistry;
  friend class PartitionAllocThreadCacheTest;
  friend class tools::ThreadCacheInspector;
};

PA_ALWAYS_INLINE std::optional<size_t> ThreadCache::MaybePutInCache(
    uintptr_t slot_start,
    size_t bucket_index) {
  PA_REENTRANCY_GUARD(is_in_thread_cache_);
  PA_INCREMENT_COUNTER(stats_.cache_fill_count);

  if (bucket_index > largest_active_bucket_index_) [[unlikely]] {
    PA_INCREMENT_COUNTER(stats_.cache_fill_misses);
    return std::nullopt;
  }

  auto& bucket = buckets_[bucket_index];

  PA_DCHECK(bucket.count != 0 || bucket.freelist_head == nullptr);

  PutInBucket(bucket, slot_start);
  cached_memory_ += bucket.slot_size;
  PA_INCREMENT_COUNTER(stats_.cache_fill_hits);

  // Relaxed ordering: we don't care about having an up-to-date or consistent
  // value, just want it to not change while we are using it, hence using
  // relaxed ordering, and loading into a local variable. Without it, we are
  // gambling that the compiler would not issue multiple loads.
  uint8_t limit = bucket.limit.load(std::memory_order_relaxed);
  // Batched deallocation, amortizing lock acquisitions.
  if (bucket.count > limit) [[unlikely]] {
    ClearBucket(bucket, limit / 2);
  }

  if (should_purge_.load(std::memory_order_relaxed)) [[unlikely]] {
    PurgeInternal();
  }

  return bucket.slot_size;
}

PA_ALWAYS_INLINE uintptr_t ThreadCache::GetFromCache(size_t bucket_index,
                                                     size_t* slot_size) {
#if PA_CONFIG(THREAD_CACHE_ALLOC_STATS)
  stats_.allocs_per_bucket_[bucket_index]++;
#endif

  PA_REENTRANCY_GUARD(is_in_thread_cache_);
  PA_INCREMENT_COUNTER(stats_.alloc_count);
  // Only handle "small" allocations.
  if (bucket_index > largest_active_bucket_index_) [[unlikely]] {
    PA_INCREMENT_COUNTER(stats_.alloc_miss_too_large);
    PA_INCREMENT_COUNTER(stats_.alloc_misses);
    return 0;
  }

  auto& bucket = buckets_[bucket_index];
  if (bucket.freelist_head) [[likely]] {
    PA_INCREMENT_COUNTER(stats_.alloc_hits);
  } else {
    PA_DCHECK(bucket.count == 0);
    PA_INCREMENT_COUNTER(stats_.alloc_miss_empty);
    PA_INCREMENT_COUNTER(stats_.alloc_misses);

    FillBucket(bucket_index);

    // Very unlikely, means that the central allocator is out of memory. Let it
    // deal with it (may return 0, may crash).
    if (!bucket.freelist_head) [[unlikely]] {
      return 0;
    }
  }

  PA_DCHECK(bucket.count != 0);
  internal::PartitionFreelistEntry* entry = bucket.freelist_head;
  // TODO(lizeb): Consider removing once crbug.com/1382658 is fixed.
#if PA_BUILDFLAG(IS_CHROMEOS) && PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && \
    PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  // x86_64 architecture now supports 57 bits of address space, as of Ice Lake
  // for Intel. However Chrome OS systems do not ship with kernel support for
  // it, but with 48 bits, so all canonical addresses have the upper 16 bits
  // zeroed (17 in practice, since the upper half of address space is reserved
  // by the kernel).
  constexpr uintptr_t kCanonicalPointerMask = (1ULL << 48) - 1;
  PA_CHECK(!(reinterpret_cast<uintptr_t>(entry) & ~kCanonicalPointerMask));
#endif  // PA_BUILDFLAG(IS_CHROMEOS) && PA_BUILDFLAG(PA_ARCH_CPU_X86_64) &&
        // PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  // Passes the bucket size to |GetNext()|, so that in case of freelist
  // corruption, we know the bucket size that lead to the crash, helping to
  // narrow down the search for culprit. |bucket| was touched just now, so this
  // does not introduce another cache miss.
  const internal::PartitionFreelistDispatcher* freelist_dispatcher =
      get_freelist_dispatcher_from_root();
#if PA_BUILDFLAG(USE_FREELIST_DISPATCHER)
  internal::PartitionFreelistEntry* next =
      freelist_dispatcher->GetNextForThreadCacheTrue(entry, bucket.slot_size);
#else
  internal::PartitionFreelistEntry* next =
      freelist_dispatcher->GetNextForThreadCache<true>(entry, bucket.slot_size);
#endif  // PA_BUILDFLAG(USE_FREELIST_DISPATCHER)

  PA_DCHECK(entry != next);
  bucket.count--;
  PA_DCHECK(bucket.count != 0 || !next);
  bucket.freelist_head = next;
  *slot_size = bucket.slot_size;

  PA_DCHECK(cached_memory_ >= bucket.slot_size);
  cached_memory_ -= bucket.slot_size;

  return internal::SlotStartPtr2Addr(entry);
}

PA_ALWAYS_INLINE void ThreadCache::PutInBucket(Bucket& bucket,
                                               uintptr_t slot_start) {
#if PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY) && \
    PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && PA_BUILDFLAG(HAS_64_BIT_POINTERS)
  // We see freelist corruption crashes happening in the wild.  These are likely
  // due to out-of-bounds accesses in the previous slot, or to a Use-After-Free
  // somewhere in the code.
  //
  // The issue is that we detect the UaF far away from the place where it
  // happens. As a consequence, we should try to make incorrect code crash as
  // early as possible. Poisoning memory at free() time works for UaF, but it
  // was seen in the past to incur a high performance cost.
  //
  // Here, only poison the current cacheline, which we are touching anyway.
  // TODO(lizeb): Make sure this does not hurt performance.

  // Everything below requires this alignment.
  static_assert(internal::kAlignment == 16, "");

  // The pointer is always 16 bytes aligned, so its start address is always == 0
  // % 16. Its distance to the next cacheline is
  //   `64 - ((slot_start & 63) / 16) * 16`
  static_assert(
      internal::kPartitionCachelineSize == 64,
      "The computation below assumes that cache lines are 64 bytes long.");
  int distance_to_next_cacheline_in_16_bytes = 4 - ((slot_start >> 4) & 3);
  int slot_size_remaining_in_16_bytes = bucket.slot_size / 16;
  slot_size_remaining_in_16_bytes = std::min(
      slot_size_remaining_in_16_bytes, distance_to_next_cacheline_in_16_bytes);

  static const uint32_t poison_16_bytes[4] = {0xbadbad00, 0xbadbad00,
                                              0xbadbad00, 0xbadbad00};

#if !(PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)) && \
    PA_HAS_BUILTIN(__builtin_assume_aligned)
  void* slot_start_tagged = __builtin_assume_aligned(
      internal::SlotStartAddr2Ptr(slot_start), internal::kAlignment);
#else
  // TODO(crbug.com/40262684): std::assume_aligned introduce an additional
  // dependency: _libcpp_verbose_abort(const char*, ...).  It will cause
  // "undefined symbol" error when linking allocator_shim.dll.
  void* slot_start_tagged = internal::SlotStartAddr2Ptr(slot_start);
#endif

  uint32_t* address_aligned = static_cast<uint32_t*>(slot_start_tagged);
  for (int i = 0; i < slot_size_remaining_in_16_bytes; i++) {
    // Clang will expand the memcpy to a 16-byte write (movups on x86).
    memcpy(address_aligned, poison_16_bytes, sizeof(poison_16_bytes));
    address_aligned += 4;
  }
#endif  // PA_CONFIG(HAS_FREELIST_SHADOW_ENTRY) &&
        // PA_BUILDFLAG(PA_ARCH_CPU_X86_64) && PA_BUILDFLAG(HAS_64_BIT_POINTERS)

  auto* entry =
      get_freelist_dispatcher_from_root()->EmplaceAndInitForThreadCache(
          slot_start, bucket.freelist_head);
  bucket.freelist_head = entry;
  bucket.count++;
}

PA_ALWAYS_INLINE void ThreadCache::RecordAllocation(size_t size) {
  thread_alloc_stats_.alloc_count++;
  thread_alloc_stats_.alloc_total_size += size;
}

PA_ALWAYS_INLINE void ThreadCache::RecordDeallocation(size_t size) {
  thread_alloc_stats_.dealloc_count++;
  thread_alloc_stats_.dealloc_total_size += size;
}

}  // namespace partition_alloc

#endif  // PARTITION_ALLOC_THREAD_CACHE_H_
