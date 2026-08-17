/*
 * Copyright (C) 2011 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_FREE_LIST_ARENA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_FREE_LIST_ARENA_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/pod_arena.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace WTF {

template <class T>
class PODFreeListArena : public RefCounted<PODFreeListArena<T>> {
  USING_FAST_MALLOC(PODFreeListArena);

 public:
  static scoped_refptr<PODFreeListArena> Create() {
    return base::AdoptRef(new PODFreeListArena);
  }

  // Creates a new PODFreeListArena configured with the given Allocator.
  static scoped_refptr<PODFreeListArena> Create(
      scoped_refptr<PODArena::Allocator> allocator) {
    return base::AdoptRef(new PODFreeListArena(std::move(allocator)));
  }

  // Allocates an object from the arena.
  T* AllocateObject() {
    void* ptr = AllocateFromFreeList();

    if (ptr) {
      // Use placement operator new to allocate a T at this location.
      new (ptr) T();
      return static_cast<T*>(ptr);
    }

    // PODArena::allocateObject calls T's constructor.
    return static_cast<T*>(arena_->AllocateObject<T>());
  }

  template <class Argument1Type>
  T* AllocateObject(const Argument1Type& argument1) {
    void* ptr = AllocateFromFreeList();

    if (ptr) {
      // Use placement operator new to allocate a T at this location.
      new (ptr) T(argument1);
      return static_cast<T*>(ptr);
    }

    // PODArena::allocateObject calls T's constructor.
    return static_cast<T*>(arena_->AllocateObject<T>(argument1));
  }

  void FreeObject(T* ptr) {
    FixedSizeMemoryChunk* old_free_list = free_list_;

    free_list_ = reinterpret_cast<FixedSizeMemoryChunk*>(ptr);
    free_list_->next = old_free_list;
  }

 private:
  PODFreeListArena() : arena_(PODArena::Create()), free_list_(nullptr) {}

  explicit PODFreeListArena(scoped_refptr<PODArena::Allocator> allocator)
      : arena_(PODArena::Create(std::move(allocator))), free_list_(nullptr) {}

  ~PODFreeListArena() = default;

  void* AllocateFromFreeList() {
    if (free_list_) {
      void* memory = free_list_;
      free_list_ = free_list_->next;
      return memory;
    }
    return nullptr;
  }

  int GetFreeListSizeForTesting() const {
    int total = 0;
    for (FixedSizeMemoryChunk* cur = free_list_; cur; cur = cur->next) {
      total++;
    }
    return total;
  }

  scoped_refptr<PODArena> arena_;

  // This free list contains pointers within every chunk that's been allocated
  // so far. None of the individual chunks can be freed until the arena is
  // destroyed.
  struct FixedSizeMemoryChunk {
    DISALLOW_NEW();
    FixedSizeMemoryChunk* next;
  };
  FixedSizeMemoryChunk* free_list_;

  static_assert(sizeof(T) >= sizeof(FixedSizeMemoryChunk),
                "PODFreeListArena type should be larger");

  friend class WTF::RefCounted<PODFreeListArena>;
  friend class PODFreeListArenaTest;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_FREE_LIST_ARENA_H_
