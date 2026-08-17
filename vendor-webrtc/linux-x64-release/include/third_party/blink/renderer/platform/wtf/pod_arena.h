/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifdef UNSAFE_BUFFERS_BUILD
// TODO(crbug.com/351564777): Remove this and convert code to safer constructs.
#pragma allow_unsafe_buffers
#endif

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_H_

#include <stdint.h>
#include <memory>
#include <utility>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {

// An arena which allocates only Plain Old Data (POD), or classes and
// structs bottoming out in Plain Old Data. NOTE: the constructors of
// the objects allocated in this arena are called, but _not_ their
// destructors.

class PODArena final : public RefCounted<PODArena> {
  USING_FAST_MALLOC(PODArena);

 public:
  // The arena is configured with an allocator, which is responsible
  // for allocating and freeing chunks of memory at a time.
  class Allocator : public RefCounted<Allocator> {
   public:
    virtual void* Allocate(size_t size) = 0;
    virtual void Free(void* ptr) = 0;

   protected:
    virtual ~Allocator() = default;
    friend class WTF::RefCounted<Allocator>;
  };

  // The Arena's default allocator, which uses fastMalloc and
  // fastFree to allocate chunks of storage.
  class FastMallocAllocator : public Allocator {
   public:
    static scoped_refptr<FastMallocAllocator> Create() {
      return base::AdoptRef(new FastMallocAllocator);
    }

    void* Allocate(size_t size) override {
      return WTF::Partitions::FastMalloc(size,
                                         WTF_HEAP_PROFILER_TYPE_NAME(PODArena));
    }
    void Free(void* ptr) override { WTF::Partitions::FastFree(ptr); }

   protected:
    FastMallocAllocator() = default;
  };

  // Creates a new PODArena configured with a FastMallocAllocator.
  static scoped_refptr<PODArena> Create() {
    return base::AdoptRef(new PODArena);
  }

  // Creates a new PODArena configured with the given Allocator.
  static scoped_refptr<PODArena> Create(scoped_refptr<Allocator> allocator) {
    return base::AdoptRef(new PODArena(std::move(allocator)));
  }

  // Allocates an object from the arena.
  template <class T>
  T* AllocateObject() {
    return new (AllocateBase<T>()) T();
  }

  // Allocates an object from the arena, calling a single-argument constructor.
  template <class T, class Argument1Type>
  T* AllocateObject(const Argument1Type& argument1) {
    return new (AllocateBase<T>()) T(argument1);
  }

  // The initial size of allocated chunks; increases as necessary to
  // satisfy large allocations. Mainly public for unit tests.
  enum { kDefaultChunkSize = 16384 };

 protected:
  friend class WTF::RefCounted<PODArena>;

  PODArena()
      : allocator_(FastMallocAllocator::Create()),
        current_(nullptr),
        current_chunk_size_(kDefaultChunkSize) {}

  explicit PODArena(scoped_refptr<Allocator> allocator)
      : allocator_(std::move(allocator)),
        current_(nullptr),
        current_chunk_size_(kDefaultChunkSize) {}

  template <class T>
  void* AllocateBase() {
    static_assert(
        sizeof(T) % alignof(T) == 0,
        "We are guaranteed that sizeof(T) is a multiple of alignof(T).");
    void* ptr = nullptr;
    size_t rounded_size = sizeof(T);
    if (current_)
      ptr = current_->Allocate(rounded_size);

    if (!ptr) {
      if (rounded_size > current_chunk_size_)
        current_chunk_size_ = rounded_size;
      chunks_.push_back(
          std::make_unique<Chunk>(allocator_.get(), current_chunk_size_));
      current_ = chunks_.back().get();
      ptr = current_->Allocate(rounded_size);
    }
    return ptr;
  }

  // Manages a chunk of memory and individual allocations out of it.
  class Chunk final {
    USING_FAST_MALLOC(Chunk);

   public:
    // Allocates a block of memory of the given size from the passed
    // Allocator.
    Chunk(Allocator* allocator, size_t size)
        : allocator_(allocator), size_(size), current_offset_(0) {
      base_ = static_cast<uint8_t*>(allocator_->Allocate(size));
    }

    Chunk(const Chunk&) = delete;
    Chunk& operator=(const Chunk&) = delete;

    // Frees the memory allocated from the Allocator in the
    // constructor.
    ~Chunk() { allocator_->Free(base_); }

    // Returns a pointer to "size" bytes of storage, or 0 if this
    // Chunk could not satisfy the allocation.
    void* Allocate(size_t size) {
      // Check for overflow
      if (current_offset_ + size < current_offset_)
        return nullptr;

      if (current_offset_ + size > size_)
        return nullptr;

      void* result = base_ + current_offset_;
      current_offset_ += size;
      return result;
    }

   protected:
    Allocator* allocator_;
    uint8_t* base_;
    size_t size_;
    size_t current_offset_;
  };

  scoped_refptr<Allocator> allocator_;
  Chunk* current_;
  size_t current_chunk_size_;
  Vector<std::unique_ptr<Chunk>> chunks_;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_POD_ARENA_H_
