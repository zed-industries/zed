//===-- size_class_allocator.h ----------------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef SCUDO_SIZE_CLASS_ALLOCATOR_H_
#define SCUDO_SIZE_CLASS_ALLOCATOR_H_

#include "internal_defs.h"
#include "list.h"
#include "platform.h"
#include "report.h"
#include "stats.h"
#include "string_utils.h"

namespace scudo {

template <class SizeClassAllocator> struct SizeClassAllocatorLocalCache {
  typedef typename SizeClassAllocator::SizeClassMap SizeClassMap;
  typedef typename SizeClassAllocator::CompactPtrT CompactPtrT;

  void init(GlobalStats *S, SizeClassAllocator *A) {
    DCHECK(isEmpty());
    Stats.init();
    if (LIKELY(S))
      S->link(&Stats);
    Allocator = A;
    initAllocator();
  }

  void destroy(GlobalStats *S) {
    drain();
    if (LIKELY(S))
      S->unlink(&Stats);
  }

  void *allocate(uptr ClassId) {
    DCHECK_LT(ClassId, NumClasses);
    PerClass *C = &PerClassArray[ClassId];
    if (C->Count == 0) {
      // Refill half of the number of max cached.
      DCHECK_GT(C->MaxCount / 2, 0U);
      if (UNLIKELY(!refill(C, ClassId, C->MaxCount / 2)))
        return nullptr;
      DCHECK_GT(C->Count, 0);
    }
    // We read ClassSize first before accessing Chunks because it's adjacent to
    // Count, while Chunks might be further off (depending on Count). That keeps
    // the memory accesses in close quarters.
    const uptr ClassSize = C->ClassSize;
    CompactPtrT CompactP = C->Chunks[--C->Count];
    Stats.add(StatAllocated, ClassSize);
    Stats.sub(StatFree, ClassSize);
    return Allocator->decompactPtr(ClassId, CompactP);
  }

  bool deallocate(uptr ClassId, void *P) {
    CHECK_LT(ClassId, NumClasses);
    PerClass *C = &PerClassArray[ClassId];

    // If the cache is full, drain half of blocks back to the main allocator.
    const bool NeedToDrainCache = C->Count == C->MaxCount;
    if (NeedToDrainCache)
      drain(C, ClassId);
    // See comment in allocate() about memory accesses.
    const uptr ClassSize = C->ClassSize;
    C->Chunks[C->Count++] =
        Allocator->compactPtr(ClassId, reinterpret_cast<uptr>(P));
    Stats.sub(StatAllocated, ClassSize);
    Stats.add(StatFree, ClassSize);

    return NeedToDrainCache;
  }

  bool isEmpty() const {
    for (uptr I = 0; I < NumClasses; ++I)
      if (PerClassArray[I].Count)
        return false;
    return true;
  }

  void drain() {
    // Drain BatchClassId last as it may be needed while draining normal blocks.
    for (uptr I = 0; I < NumClasses; ++I) {
      if (I == BatchClassId)
        continue;
      while (PerClassArray[I].Count > 0)
        drain(&PerClassArray[I], I);
    }
    while (PerClassArray[BatchClassId].Count > 0)
      drain(&PerClassArray[BatchClassId], BatchClassId);
    DCHECK(isEmpty());
  }

  void *getBatchClassBlock() {
    void *B = allocate(BatchClassId);
    if (UNLIKELY(!B))
      reportOutOfMemory(SizeClassAllocator::getSizeByClassId(BatchClassId));
    return B;
  }

  LocalStats &getStats() { return Stats; }

  void getStats(ScopedString *Str) {
    bool EmptyCache = true;
    for (uptr I = 0; I < NumClasses; ++I) {
      if (PerClassArray[I].Count == 0)
        continue;

      EmptyCache = false;
      // The size of BatchClass is set to 0 intentionally. See the comment in
      // initAllocator() for more details.
      const uptr ClassSize = I == BatchClassId
                                 ? SizeClassAllocator::getSizeByClassId(I)
                                 : PerClassArray[I].ClassSize;
      // Note that the string utils don't support printing u16 thus we cast it
      // to a common use type uptr.
      Str->append("    %02zu (%6zu): cached: %4zu max: %4zu\n", I, ClassSize,
                  static_cast<uptr>(PerClassArray[I].Count),
                  static_cast<uptr>(PerClassArray[I].MaxCount));
    }

    if (EmptyCache)
      Str->append("    No block is cached.\n");
  }

  static u16 getMaxCached(uptr Size) {
    return Min(SizeClassMap::MaxNumCachedHint,
               SizeClassMap::getMaxCachedHint(Size));
  }

private:
  static const uptr NumClasses = SizeClassMap::NumClasses;
  static const uptr BatchClassId = SizeClassMap::BatchClassId;
  struct alignas(SCUDO_CACHE_LINE_SIZE) PerClass {
    u16 Count;
    u16 MaxCount;
    // Note: ClassSize is zero for the transfer batch.
    uptr ClassSize;
    CompactPtrT Chunks[2 * SizeClassMap::MaxNumCachedHint];
  };
  PerClass PerClassArray[NumClasses] = {};
  LocalStats Stats;
  SizeClassAllocator *Allocator = nullptr;

  NOINLINE void initAllocator() {
    for (uptr I = 0; I < NumClasses; I++) {
      PerClass *P = &PerClassArray[I];
      const uptr Size = SizeClassAllocator::getSizeByClassId(I);
      P->MaxCount = static_cast<u16>(2 * getMaxCached(Size));
      if (I != BatchClassId) {
        P->ClassSize = Size;
      } else {
        // ClassSize in this struct is only used for malloc/free stats, which
        // should only track user allocations, not internal movements.
        P->ClassSize = 0;
      }
    }
  }

  NOINLINE bool refill(PerClass *C, uptr ClassId, u16 MaxRefill) {
    const u16 NumBlocksRefilled =
        Allocator->popBlocks(this, ClassId, C->Chunks, MaxRefill);
    DCHECK_LE(NumBlocksRefilled, MaxRefill);
    C->Count = static_cast<u16>(C->Count + NumBlocksRefilled);
    return NumBlocksRefilled != 0;
  }

  NOINLINE void drain(PerClass *C, uptr ClassId) {
    const u16 Count = Min(static_cast<u16>(C->MaxCount / 2), C->Count);
    Allocator->pushBlocks(this, ClassId, &C->Chunks[0], Count);
    // u16 will be promoted to int by arithmetic type conversion.
    C->Count = static_cast<u16>(C->Count - Count);
    for (u16 I = 0; I < C->Count; I++)
      C->Chunks[I] = C->Chunks[I + Count];
  }
};

template <class SizeClassAllocator> struct SizeClassAllocatorNoCache {
  typedef typename SizeClassAllocator::SizeClassMap SizeClassMap;
  typedef typename SizeClassAllocator::CompactPtrT CompactPtrT;

  void init(GlobalStats *S, SizeClassAllocator *A) {
    Stats.init();
    if (LIKELY(S))
      S->link(&Stats);
    Allocator = A;
    initAllocator();
  }

  void destroy(GlobalStats *S) {
    if (LIKELY(S))
      S->unlink(&Stats);
  }

  void *allocate(uptr ClassId) {
    CompactPtrT CompactPtr;
    uptr NumBlocksPopped = Allocator->popBlocks(this, ClassId, &CompactPtr, 1U);
    if (NumBlocksPopped == 0)
      return nullptr;
    DCHECK_EQ(NumBlocksPopped, 1U);
    const PerClass *C = &PerClassArray[ClassId];
    Stats.add(StatAllocated, C->ClassSize);
    Stats.sub(StatFree, C->ClassSize);
    return Allocator->decompactPtr(ClassId, CompactPtr);
  }

  bool deallocate(uptr ClassId, void *P) {
    CHECK_LT(ClassId, NumClasses);

    if (ClassId == BatchClassId)
      return deallocateBatchClassBlock(P);

    CompactPtrT CompactPtr =
        Allocator->compactPtr(ClassId, reinterpret_cast<uptr>(P));
    Allocator->pushBlocks(this, ClassId, &CompactPtr, 1U);
    PerClass *C = &PerClassArray[ClassId];
    Stats.sub(StatAllocated, C->ClassSize);
    Stats.add(StatFree, C->ClassSize);

    // The following adopts the same strategy of allocator draining as used
    // in SizeClassAllocatorLocalCache so that use the same hint when doing
    // a page release.
    ++C->Count;
    const bool SuggestDraining = C->Count >= C->MaxCount;
    if (SuggestDraining)
      C->Count = 0;
    return SuggestDraining;
  }

  void *getBatchClassBlock() {
    PerClass *C = &PerClassArray[BatchClassId];
    if (C->Count == 0) {
      const u16 NumBlocksRefilled = Allocator->popBlocks(
          this, BatchClassId, BatchClassStorage, C->MaxCount);
      if (NumBlocksRefilled == 0)
        reportOutOfMemory(SizeClassAllocator::getSizeByClassId(BatchClassId));
      DCHECK_LE(NumBlocksRefilled, SizeClassMap::MaxNumCachedHint);
      C->Count = NumBlocksRefilled;
    }

    const uptr ClassSize = C->ClassSize;
    CompactPtrT CompactP = BatchClassStorage[--C->Count];
    Stats.add(StatAllocated, ClassSize);
    Stats.sub(StatFree, ClassSize);

    return Allocator->decompactPtr(BatchClassId, CompactP);
  }

  LocalStats &getStats() { return Stats; }

  void getStats(ScopedString *Str) { Str->append("    No block is cached.\n"); }

  bool isEmpty() const {
    const PerClass *C = &PerClassArray[BatchClassId];
    return C->Count == 0;
  }
  void drain() {
    PerClass *C = &PerClassArray[BatchClassId];
    if (C->Count > 0) {
      Allocator->pushBlocks(this, BatchClassId, BatchClassStorage, C->Count);
      C->Count = 0;
    }
  }

  static u16 getMaxCached(uptr Size) {
    return Min(SizeClassMap::MaxNumCachedHint,
               SizeClassMap::getMaxCachedHint(Size));
  }

private:
  static const uptr NumClasses = SizeClassMap::NumClasses;
  static const uptr BatchClassId = SizeClassMap::BatchClassId;
  struct alignas(SCUDO_CACHE_LINE_SIZE) PerClass {
    u16 Count = 0;
    u16 MaxCount;
    // Note: ClassSize is zero for the transfer batch.
    uptr ClassSize;
  };
  PerClass PerClassArray[NumClasses] = {};
  // Popping BatchClass blocks requires taking a certain amount of blocks at
  // once. This restriction comes from how we manage the storing of BatchClass
  // in the primary allocator. See more details in `popBlocksImpl` in the
  // primary allocator.
  CompactPtrT BatchClassStorage[SizeClassMap::MaxNumCachedHint] = {};
  LocalStats Stats;
  SizeClassAllocator *Allocator = nullptr;

  bool deallocateBatchClassBlock(void *P) {
    PerClass *C = &PerClassArray[BatchClassId];
    // Drain all the blocks.
    if (C->Count >= C->MaxCount) {
      Allocator->pushBlocks(this, BatchClassId, BatchClassStorage, C->Count);
      C->Count = 0;
    }
    BatchClassStorage[C->Count++] =
        Allocator->compactPtr(BatchClassId, reinterpret_cast<uptr>(P));

    // Currently, BatchClass doesn't support page releasing, so we always return
    // false.
    return false;
  }

  NOINLINE void initAllocator() {
    for (uptr I = 0; I < NumClasses; I++) {
      PerClass *P = &PerClassArray[I];
      const uptr Size = SizeClassAllocator::getSizeByClassId(I);
      if (I != BatchClassId) {
        P->ClassSize = Size;
        P->MaxCount = static_cast<u16>(2 * getMaxCached(Size));
      } else {
        // ClassSize in this struct is only used for malloc/free stats, which
        // should only track user allocations, not internal movements.
        P->ClassSize = 0;
        P->MaxCount = SizeClassMap::MaxNumCachedHint;
      }
    }
  }
};

} // namespace scudo

#endif // SCUDO_SIZE_CLASS_ALLOCATOR_H_
