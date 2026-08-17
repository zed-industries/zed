// Protocol Buffers - Google's data interchange format
// Copyright 2022 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd
//
// This file defines the internal class SerialArena

#ifndef GOOGLE_PROTOBUF_SERIAL_ARENA_H__
#define GOOGLE_PROTOBUF_SERIAL_ARENA_H__

#include <algorithm>
#include <atomic>
#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/base/prefetch.h"
#include "absl/log/absl_check.h"
#include "absl/numeric/bits.h"
#include "google/protobuf/arena_align.h"
#include "google/protobuf/arena_cleanup.h"
#include "google/protobuf/port.h"
#include "google/protobuf/string_block.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

// Arena blocks are variable length malloc-ed objects.  The following structure
// describes the common header for all blocks.
struct ArenaBlock {
  // For the sentry block with zero-size where ptr_/limit_ both point to `this`.
  constexpr ArenaBlock() : next(nullptr), size(0) {}

  ArenaBlock(ArenaBlock* next, size_t size) : next(next), size(size) {
    ABSL_DCHECK_GT(size, sizeof(ArenaBlock));
  }

  char* Pointer(size_t n) {
    ABSL_DCHECK_LE(n, size);
    return reinterpret_cast<char*>(this) + n;
  }
  char* Limit() { return Pointer(size & static_cast<size_t>(-8)); }

  bool IsSentry() const { return size == 0; }

  ArenaBlock* const next;
  const size_t size;
  // data follows
};

enum class AllocationClient { kDefault, kArray };

class ThreadSafeArena;

// Tag type used to invoke the constructor of the first SerialArena.
struct FirstSerialArena {
  explicit FirstSerialArena() = default;
};

// A simple arena allocator. Calls to allocate functions must be properly
// serialized by the caller, hence this class cannot be used as a general
// purpose allocator in a multi-threaded program. It serves as a building block
// for ThreadSafeArena, which provides a thread-safe arena allocator.
//
// This class manages
// 1) Arena bump allocation + owning memory blocks.
// 2) Maintaining a cleanup list.
// It delegates the actual memory allocation back to ThreadSafeArena, which
// contains the information on block growth policy and backing memory allocation
// used.
class PROTOBUF_EXPORT SerialArena {
 public:
  static constexpr size_t kBlockHeaderSize =
      ArenaAlignDefault::Ceil(sizeof(ArenaBlock));

  void CleanupList() { cleanup_list_.Cleanup(*this); }
  uint64_t SpaceAllocated() const {
    return space_allocated_.load(std::memory_order_relaxed);
  }
  uint64_t SpaceUsed() const;

  // See comments on `cached_blocks_` member for details.
  PROTOBUF_ALWAYS_INLINE void* TryAllocateFromCachedBlock(size_t size) {
    if (ABSL_PREDICT_FALSE(size < 16)) return nullptr;
    // We round up to the next larger block in case the memory doesn't match
    // the pattern we are looking for.
    const size_t index = absl::bit_width(size - 1) - 4;

    if (ABSL_PREDICT_FALSE(index >= cached_block_length_)) return nullptr;
    auto& cached_head = cached_blocks_[index];
    if (cached_head == nullptr) return nullptr;

    void* ret = cached_head;
    internal::UnpoisonMemoryRegion(ret, size);
    cached_head = cached_head->next;
    return ret;
  }

  // In kArray mode we look through cached blocks.
  // We do not do this by default because most non-array allocations will not
  // have the right size and will fail to find an appropriate cached block.
  //
  // TODO: Evaluate if we should use cached blocks for message types of
  // the right size. We can statically know if the allocation size can benefit
  // from it.
  template <AllocationClient alloc_client = AllocationClient::kDefault>
  void* AllocateAligned(size_t n) {
    ABSL_DCHECK(internal::ArenaAlignDefault::IsAligned(n));
    ABSL_DCHECK_GE(limit_, ptr());

    if (alloc_client == AllocationClient::kArray) {
      if (void* res = TryAllocateFromCachedBlock(n)) {
        return res;
      }
    }

    void* ptr;
    if (ABSL_PREDICT_TRUE(MaybeAllocateAligned(n, &ptr))) {
      return ptr;
    }
    return AllocateAlignedFallback(n);
  }

 private:
  static PROTOBUF_ALWAYS_INLINE constexpr size_t AlignUpTo(size_t n, size_t a) {
    // We are wasting space by over allocating align - 8 bytes. Compared to a
    // dedicated function that takes current alignment in consideration.  Such a
    // scheme would only waste (align - 8)/2 bytes on average, but requires a
    // dedicated function in the outline arena allocation functions. Possibly
    // re-evaluate tradeoffs later.
    return a <= 8 ? ArenaAlignDefault::Ceil(n) : ArenaAlignAs(a).Padded(n);
  }

  static PROTOBUF_ALWAYS_INLINE void* AlignTo(void* p, size_t a) {
    return (a <= ArenaAlignDefault::align)
               ? ArenaAlignDefault::CeilDefaultAligned(p)
               : ArenaAlignAs(a).CeilDefaultAligned(p);
  }

  // See comments on `cached_blocks_` member for details.
  void ReturnArrayMemory(void* p, size_t size) {
    // We only need to check for 32-bit platforms.
    // In 64-bit platforms the minimum allocation size from Repeated*Field will
    // be 16 guaranteed.
    if (sizeof(void*) < 8) {
      if (ABSL_PREDICT_FALSE(size < 16)) return;
    } else {
      PROTOBUF_ASSUME(size >= 16);
    }

    // We round down to the next smaller block in case the memory doesn't match
    // the pattern we are looking for. eg, someone might have called Reserve()
    // on the repeated field.
    const size_t index = absl::bit_width(size) - 5;

    if (ABSL_PREDICT_FALSE(index >= cached_block_length_)) {
      // We can't put this object on the freelist so make this object the
      // freelist. It is guaranteed it is larger than the one we have, and
      // large enough to hold another allocation of `size`.
      CachedBlock** new_list = static_cast<CachedBlock**>(p);
      size_t new_size = size / sizeof(CachedBlock*);

      std::copy(cached_blocks_, cached_blocks_ + cached_block_length_,
                new_list);

      // We need to unpoison this memory before filling it in case it has been
      // poisoned by another sanitizer client.
      internal::UnpoisonMemoryRegion(
          new_list + cached_block_length_,
          (new_size - cached_block_length_) * sizeof(CachedBlock*));

      std::fill(new_list + cached_block_length_, new_list + new_size, nullptr);

      cached_blocks_ = new_list;
      // Make the size fit in uint8_t. This is the power of two, so we don't
      // need anything larger.
      cached_block_length_ =
          static_cast<uint8_t>(std::min(size_t{64}, new_size));

      return;
    }

    auto& cached_head = cached_blocks_[index];
    auto* new_node = static_cast<CachedBlock*>(p);
    new_node->next = cached_head;
    cached_head = new_node;
    internal::PoisonMemoryRegion(p, size);
  }

 public:
  // Allocate space if the current region provides enough space.
  bool MaybeAllocateAligned(size_t n, void** out) {
    ABSL_DCHECK(internal::ArenaAlignDefault::IsAligned(n));
    ABSL_DCHECK_GE(limit_, ptr());
    char* ret = ptr();
    // ret + n may point out of the block bounds, or ret may be nullptr.
    // Both computations have undefined behavior when done on pointers,
    // so do them on uintptr_t instead.
    if (ABSL_PREDICT_FALSE(reinterpret_cast<uintptr_t>(ret) + n >
                           reinterpret_cast<uintptr_t>(limit_))) {
      return false;
    }
    internal::UnpoisonMemoryRegion(ret, n);
    *out = ret;
    char* next = ret + n;
    set_ptr(next);
    MaybePrefetchData(next);
    return true;
  }

  // If there is enough space in the current block, allocate space for one
  // std::string object and register for destruction. The object has not been
  // constructed and the memory returned is uninitialized.
  PROTOBUF_ALWAYS_INLINE void* MaybeAllocateStringWithCleanup() {
    void* p;
    return MaybeAllocateString(p) ? p : nullptr;
  }

  PROTOBUF_ALWAYS_INLINE
  void* AllocateAlignedWithCleanup(size_t n, size_t align,
                                   void (*destructor)(void*)) {
    n = ArenaAlignDefault::Ceil(n);
    char* ret = ArenaAlignAs(align).CeilDefaultAligned(ptr());
    // See the comment in MaybeAllocateAligned re uintptr_t.
    if (ABSL_PREDICT_FALSE(reinterpret_cast<uintptr_t>(ret) + n >
                           reinterpret_cast<uintptr_t>(limit_))) {
      return AllocateAlignedWithCleanupFallback(n, align, destructor);
    }
    internal::UnpoisonMemoryRegion(ret, n);
    char* next = ret + n;
    set_ptr(next);
    AddCleanup(ret, destructor);
    ABSL_DCHECK_GE(limit_, ptr());
    MaybePrefetchData(next);
    return ret;
  }

  PROTOBUF_ALWAYS_INLINE
  void AddCleanup(void* elem, void (*destructor)(void*)) {
    cleanup_list_.Add(elem, destructor, *this);
    MaybePrefetchCleanup();
  }

  ABSL_ATTRIBUTE_RETURNS_NONNULL void* AllocateFromStringBlock();

  std::vector<void*> PeekCleanupListForTesting();

 private:
  friend class ThreadSafeArena;
  friend class cleanup::ChunkList;

  // See comments for cached_blocks_.
  struct CachedBlock {
    // Simple linked list.
    CachedBlock* next;
  };

  static constexpr ptrdiff_t kPrefetchDataDegree = ABSL_CACHELINE_SIZE * 16;
  static constexpr ptrdiff_t kPrefetchCleanupDegree = ABSL_CACHELINE_SIZE * 6;

  // Constructor is private as only New() should be used.
  inline SerialArena(ArenaBlock* b, ThreadSafeArena& parent);

  // Constructors to handle the first SerialArena.
  inline explicit SerialArena(ThreadSafeArena& parent);
  inline SerialArena(FirstSerialArena, ArenaBlock* b, ThreadSafeArena& parent);

  bool MaybeAllocateString(void*& p);
  ABSL_ATTRIBUTE_RETURNS_NONNULL void* AllocateFromStringBlockFallback();

  // Prefetch the next prefetch_degree bytes after `prefetch_ptr` and
  // up to `limit`, if `next` is within prefetch_degree bytes of `prefetch_ptr`.
  PROTOBUF_ALWAYS_INLINE
  static const char* MaybePrefetchImpl(const ptrdiff_t prefetch_degree,
                                       const char* next, const char* limit,
                                       const char* prefetch_ptr) {
    if (ABSL_PREDICT_TRUE(prefetch_ptr - next > prefetch_degree))
      return prefetch_ptr;
    if (ABSL_PREDICT_TRUE(prefetch_ptr < limit)) {
      prefetch_ptr = std::max(next, prefetch_ptr);
      ABSL_DCHECK(prefetch_ptr != nullptr);
      const char* end = std::min(limit, prefetch_ptr + prefetch_degree);
      for (; prefetch_ptr < end; prefetch_ptr += ABSL_CACHELINE_SIZE) {
        absl::PrefetchToLocalCacheForWrite(prefetch_ptr);
      }
    }
    return prefetch_ptr;
  }
  PROTOBUF_ALWAYS_INLINE
  void MaybePrefetchData(const char* next) {
    ABSL_DCHECK(static_cast<const void*>(prefetch_ptr_) == nullptr ||
                static_cast<const void*>(prefetch_ptr_) >= head());
    prefetch_ptr_ =
        MaybePrefetchImpl(kPrefetchDataDegree, next, limit_, prefetch_ptr_);
  }
  PROTOBUF_ALWAYS_INLINE
  void MaybePrefetchCleanup() {
    ABSL_DCHECK(static_cast<const void*>(cleanup_list_.prefetch_ptr_) ==
                    nullptr ||
                static_cast<const void*>(cleanup_list_.prefetch_ptr_) >=
                    cleanup_list_.head_);
    cleanup_list_.prefetch_ptr_ = MaybePrefetchImpl(
        kPrefetchCleanupDegree, reinterpret_cast<char*>(cleanup_list_.next_),
        reinterpret_cast<char*>(cleanup_list_.limit_),
        cleanup_list_.prefetch_ptr_);
  }

  // Creates a new SerialArena inside mem using the remaining memory as for
  // future allocations.
  // The `parent` arena must outlive the serial arena, which is guaranteed
  // because the parent manages the lifetime of the serial arenas.
  static SerialArena* New(SizedPtr mem, ThreadSafeArena& parent);
  // Free SerialArena returning the memory passed in to New.
  template <typename Deallocator>
  SizedPtr Free(Deallocator deallocator);

  size_t FreeStringBlocks() {
    // On the active block delete all strings skipping the unused instances.
    size_t unused_bytes = string_block_unused_.load(std::memory_order_relaxed);
    if (StringBlock* sb = string_block_.load(std::memory_order_relaxed)) {
      return FreeStringBlocks(sb, unused_bytes);
    }
    return 0;
  }
  static size_t FreeStringBlocks(StringBlock* string_block, size_t unused);

  // Adds 'used` to space_used_ in relaxed atomic order.
  void AddSpaceUsed(size_t space_used) {
    space_used_.store(space_used_.load(std::memory_order_relaxed) + space_used,
                      std::memory_order_relaxed);
  }

  // Adds 'allocated` to space_allocated_ in relaxed atomic order.
  void AddSpaceAllocated(size_t space_allocated) {
    space_allocated_.store(
        space_allocated_.load(std::memory_order_relaxed) + space_allocated,
        std::memory_order_relaxed);
  }

  // Helper getters/setters to handle relaxed operations on atomic variables.
  ArenaBlock* head() { return head_.load(std::memory_order_relaxed); }
  const ArenaBlock* head() const {
    return head_.load(std::memory_order_relaxed);
  }

  char* ptr() { return ptr_.load(std::memory_order_relaxed); }
  const char* ptr() const { return ptr_.load(std::memory_order_relaxed); }
  void set_ptr(char* ptr) { return ptr_.store(ptr, std::memory_order_relaxed); }
  PROTOBUF_ALWAYS_INLINE void set_range(char* ptr, char* limit) {
    set_ptr(ptr);
    prefetch_ptr_ = ptr;
    limit_ = limit;
  }

  void* AllocateAlignedFallback(size_t n);
  void* AllocateAlignedWithCleanupFallback(size_t n, size_t align,
                                           void (*destructor)(void*));
  void AddCleanupFallback(void* elem, void (*destructor)(void*));
  inline void AllocateNewBlock(size_t n);
  inline void Init(ArenaBlock* b, size_t offset);

  // Members are declared here to track sizeof(SerialArena) and hotness
  // centrally. They are (roughly) laid out in descending order of hotness.

  // Next pointer to allocate from.  Always 8-byte aligned.  Points inside
  // head_ (and head_->pos will always be non-canonical).  We keep these
  // here to reduce indirection.
  std::atomic<char*> ptr_{nullptr};
  // Limiting address up to which memory can be allocated from the head block.
  char* limit_ = nullptr;
  // Current prefetch positions. Data from `ptr_` up to but not including
  // `prefetch_ptr_` is software prefetched.
  const char* prefetch_ptr_ = nullptr;

  // Chunked linked list for managing cleanup for arena elements.
  cleanup::ChunkList cleanup_list_;

  // The active string block.
  std::atomic<StringBlock*> string_block_{nullptr};

  // The number of unused bytes in string_block_.
  // We allocate from `effective_size()` down to 0 inside `string_block_`.
  // `unused  == 0` means that `string_block_` is exhausted. (or null).
  std::atomic<size_t> string_block_unused_{0};

  std::atomic<ArenaBlock*> head_{nullptr};  // Head of linked list of blocks.
  std::atomic<size_t> space_used_{0};       // Necessary for metrics.
  std::atomic<size_t> space_allocated_{0};
  ThreadSafeArena& parent_;

  // Repeated*Field and Arena play together to reduce memory consumption by
  // reusing blocks. Currently, natural growth of the repeated field types makes
  // them allocate blocks of size `8 + 2^N, N>=3`.
  // When the repeated field grows returns the previous block and we put it in
  // this free list.
  // `cached_blocks_[i]` points to the free list for blocks of size `8+2^(i+3)`.
  // The array of freelists is grown when needed in `ReturnArrayMemory()`.
  uint8_t cached_block_length_ = 0;
  CachedBlock** cached_blocks_ = nullptr;
};

PROTOBUF_ALWAYS_INLINE bool SerialArena::MaybeAllocateString(void*& p) {
  // Check how many unused instances are in the current block.
  size_t unused_bytes = string_block_unused_.load(std::memory_order_relaxed);
  if (ABSL_PREDICT_TRUE(unused_bytes != 0)) {
    unused_bytes -= sizeof(std::string);
    string_block_unused_.store(unused_bytes, std::memory_order_relaxed);
    p = string_block_.load(std::memory_order_relaxed)->AtOffset(unused_bytes);
    return true;
  }
  return false;
}

ABSL_ATTRIBUTE_RETURNS_NONNULL PROTOBUF_ALWAYS_INLINE void*
SerialArena::AllocateFromStringBlock() {
  void* p;
  if (ABSL_PREDICT_TRUE(MaybeAllocateString(p))) return p;
  return AllocateFromStringBlockFallback();
}

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_SERIAL_ARENA_H__
