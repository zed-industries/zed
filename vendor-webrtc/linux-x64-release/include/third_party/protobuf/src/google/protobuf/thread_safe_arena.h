// Protocol Buffers - Google's data interchange format
// Copyright 2022 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
//
// This file defines the internal class ThreadSafeArena

#ifndef GOOGLE_PROTOBUF_THREAD_SAFE_ARENA_H__
#define GOOGLE_PROTOBUF_THREAD_SAFE_ARENA_H__

#include <atomic>
#include <cstddef>
#include <cstdint>
#include <type_traits>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/synchronization/mutex.h"
#include "google/protobuf/arena_align.h"
#include "google/protobuf/arena_allocation_policy.h"
#include "google/protobuf/arena_cleanup.h"
#include "google/protobuf/arenaz_sampler.h"
#include "google/protobuf/port.h"
#include "google/protobuf/serial_arena.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// This class provides the core Arena memory allocation library. Different
// implementations only need to implement the public interface below.
// Arena is not a template type as that would only be useful if all protos
// in turn would be templates, which will/cannot happen. However separating
// the memory allocation part from the cruft of the API users expect we can
// use #ifdef the select the best implementation based on hardware / OS.
class PROTOBUF_EXPORT ThreadSafeArena {
 public:
  ThreadSafeArena();

  ThreadSafeArena(char* mem, size_t size);

  explicit ThreadSafeArena(void* mem, size_t size,
                           const AllocationPolicy& policy);

  // All protos have pointers back to the arena hence Arena must have
  // pointer stability.
  ThreadSafeArena(const ThreadSafeArena&) = delete;
  ThreadSafeArena& operator=(const ThreadSafeArena&) = delete;
  ThreadSafeArena(ThreadSafeArena&&) = delete;
  ThreadSafeArena& operator=(ThreadSafeArena&&) = delete;

  // Destructor deletes all owned heap allocated objects, and destructs objects
  // that have non-trivial destructors, except for proto2 message objects whose
  // destructors can be skipped. Also, frees all blocks except the initial block
  // if it was passed in.
  ~ThreadSafeArena();

  uint64_t Reset();

  uint64_t SpaceAllocated() const;
  uint64_t SpaceUsed() const;

  template <AllocationClient alloc_client = AllocationClient::kDefault>
  void* AllocateAligned(size_t n) {
    SerialArena* arena;
    if (ABSL_PREDICT_TRUE(GetSerialArenaFast(&arena))) {
      return arena->AllocateAligned<alloc_client>(n);
    } else {
      return AllocateAlignedFallback<alloc_client>(n);
    }
  }

  void ReturnArrayMemory(void* p, size_t size) {
    SerialArena* arena = nullptr;
    if (ABSL_PREDICT_TRUE(GetSerialArenaFast(&arena))) {
      arena->ReturnArrayMemory(p, size);
    }
  }

  // This function allocates n bytes if the common happy case is true and
  // returns true. Otherwise does nothing and returns false. This strange
  // semantics is necessary to allow callers to program functions that only
  // have fallback function calls in tail position. This substantially improves
  // code for the happy path.
  PROTOBUF_NDEBUG_INLINE bool MaybeAllocateAligned(size_t n, void** out) {
    SerialArena* arena = nullptr;
    if (ABSL_PREDICT_TRUE(GetSerialArenaFast(&arena))) {
      return arena->MaybeAllocateAligned(n, out);
    }
    return false;
  }

  void* AllocateAlignedWithCleanup(size_t n, size_t align,
                                   void (*destructor)(void*));

  // Add object pointer and cleanup function pointer to the list.
  void AddCleanup(void* elem, void (*cleanup)(void*));

  void* AllocateFromStringBlock();

  std::vector<void*> PeekCleanupListForTesting();

 private:
  friend class ArenaBenchmark;
  friend class TcParser;
  friend class SerialArena;
  friend struct SerialArenaChunkHeader;
  friend class cleanup::ChunkList;
  static uint64_t GetNextLifeCycleId();

  class SerialArenaChunk;

  // Returns a new SerialArenaChunk that has {id, serial} at slot 0. It may
  // grow based on "prev_num_slots".
  static SerialArenaChunk* NewSerialArenaChunk(uint32_t prev_capacity, void* id,
                                               SerialArena* serial);
  static SerialArenaChunk* SentrySerialArenaChunk();

  // Returns the first ArenaBlock* for the first SerialArena. If users provide
  // one, use it if it's acceptable. Otherwise returns a sentry block.
  ArenaBlock* FirstBlock(void* buf, size_t size);
  // Same as the above but returns a valid block if "policy" is not default.
  ArenaBlock* FirstBlock(void* buf, size_t size,
                         const AllocationPolicy& policy);

  // Adds SerialArena to the chunked list. May create a new chunk.
  void AddSerialArena(void* id, SerialArena* serial);

  void UnpoisonAllArenaBlocks() const;

  // Members are declared here to track sizeof(ThreadSafeArena) and hotness
  // centrally.

  // Unique for each arena. Changes on Reset().
  uint64_t tag_and_id_ = 0;

  TaggedAllocationPolicyPtr alloc_policy_;  // Tagged pointer to AllocPolicy.
  ThreadSafeArenaStatsHandle arena_stats_;

  // Adding a new chunk to head_ must be protected by mutex_.
  absl::Mutex mutex_;
  // Pointer to a linked list of SerialArenaChunk.
  std::atomic<SerialArenaChunk*> head_{nullptr};

  void* first_owner_;
  // Must be declared after alloc_policy_; otherwise, it may lose info on
  // user-provided initial block.
  SerialArena first_arena_;

  static_assert(std::is_trivially_destructible<SerialArena>{},
                "SerialArena needs to be trivially destructible.");

  const AllocationPolicy* AllocPolicy() const { return alloc_policy_.get(); }
  void InitializeWithPolicy(const AllocationPolicy& policy);
  void* AllocateAlignedWithCleanupFallback(size_t n, size_t align,
                                           void (*destructor)(void*));

  void Init();

  // Delete or Destruct all objects owned by the arena.
  void CleanupList();

  inline void CacheSerialArena(SerialArena* serial) {
    thread_cache().last_serial_arena = serial;
    thread_cache().last_lifecycle_id_seen = tag_and_id_;
  }

  PROTOBUF_NDEBUG_INLINE bool GetSerialArenaFast(SerialArena** arena) {
    // If this thread already owns a block in this arena then try to use that.
    // This fast path optimizes the case where multiple threads allocate from
    // the same arena.
    ThreadCache* tc = &thread_cache();
    if (ABSL_PREDICT_TRUE(tc->last_lifecycle_id_seen == tag_and_id_)) {
      *arena = tc->last_serial_arena;
      return true;
    }
    return false;
  }

  // Finds SerialArena or creates one if not found. When creating a new one,
  // create a big enough block to accommodate n bytes.
  SerialArena* GetSerialArenaFallback(size_t n);

  SerialArena* GetSerialArena();

  template <AllocationClient alloc_client = AllocationClient::kDefault>
  void* AllocateAlignedFallback(size_t n);

  // Executes callback function over SerialArenaChunk. Passes const
  // SerialArenaChunk*.
  template <typename Callback>
  void WalkConstSerialArenaChunk(Callback fn) const;

  // Executes callback function over SerialArenaChunk.
  template <typename Callback>
  void WalkSerialArenaChunk(Callback fn);

  // Visits SerialArena and calls "fn", including "first_arena" and ones on
  // chunks. Do not rely on the order of visit. The callback function should
  // accept `const SerialArena*`.
  template <typename Callback>
  void VisitSerialArena(Callback fn) const;

  // Releases all memory except the first block which it returns. The first
  // block might be owned by the user and thus need some extra checks before
  // deleting.
  SizedPtr Free();

  // ThreadCache is accessed very frequently, so we align it such that it's
  // located within a single cache line.
  static constexpr size_t kThreadCacheAlignment = 32;

#ifdef _MSC_VER
#pragma warning(disable : 4324)
#endif
  struct alignas(kThreadCacheAlignment) ThreadCache {
    // Number of per-thread lifecycle IDs to reserve. Must be power of two.
    // To reduce contention on a global atomic, each thread reserves a batch of
    // IDs.  The following number is calculated based on a stress test with
    // ~6500 threads all frequently allocating a new arena.
    static constexpr size_t kPerThreadIds = 256;
    // Next lifecycle ID available to this thread. We need to reserve a new
    // batch, if `next_lifecycle_id & (kPerThreadIds - 1) == 0`.
    uint64_t next_lifecycle_id{0};
    // The ThreadCache is considered valid as long as this matches the
    // lifecycle_id of the arena being used.
    uint64_t last_lifecycle_id_seen{static_cast<uint64_t>(-1)};
    SerialArena* last_serial_arena{nullptr};
  };
  static_assert(sizeof(ThreadCache) <= kThreadCacheAlignment,
                "ThreadCache may span several cache lines");

  // Lifecycle_id can be highly contended variable in a situation of lots of
  // arena creation. Make sure that other global variables are not sharing the
  // cacheline.
#ifdef _MSC_VER
#pragma warning(disable : 4324)
#endif
  using LifecycleId = uint64_t;
  alignas(kCacheAlignment) ABSL_CONST_INIT
      static std::atomic<LifecycleId> lifecycle_id_;
#if defined(PROTOBUF_NO_THREADLOCAL)
  // iOS does not support __thread keyword so we use a custom thread local
  // storage class we implemented.
  static ThreadCache& thread_cache();
#elif defined(PROTOBUF_USE_DLLS) && defined(_WIN32)
  // Thread local variables cannot be exposed through MSVC DLL interface but we
  // can wrap them in static functions.
  static ThreadCache& thread_cache();
#else
  PROTOBUF_CONSTINIT static PROTOBUF_THREAD_LOCAL ThreadCache thread_cache_;
  static ThreadCache& thread_cache() { return thread_cache_; }
#endif

 public:
  // kBlockHeaderSize is sizeof(ArenaBlock), aligned up to the nearest multiple
  // of 8 to protect the invariant that pos is always at a multiple of 8.
  static constexpr size_t kBlockHeaderSize = SerialArena::kBlockHeaderSize;
  static constexpr size_t kSerialArenaSize =
      (sizeof(SerialArena) + 7) & static_cast<size_t>(-8);
  static constexpr size_t kAllocPolicySize =
      ArenaAlignDefault::Ceil(sizeof(AllocationPolicy));
  static constexpr size_t kMaxCleanupNodeSize = 16;
  static_assert(kBlockHeaderSize % 8 == 0,
                "kBlockHeaderSize must be a multiple of 8.");
  static_assert(kSerialArenaSize % 8 == 0,
                "kSerialArenaSize must be a multiple of 8.");
};

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_THREAD_SAFE_ARENA_H__
