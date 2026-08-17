// Copyright 2021 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_UTIL_CHUNKED_VECTOR_H
#define GRPC_SRC_CORE_UTIL_CHUNKED_VECTOR_H

#include <grpc/support/port_platform.h>

#include <cstddef>
#include <iterator>
#include <utility>

#include "absl/log/check.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/util/manual_constructor.h"

namespace grpc_core {

// Arena-friendly vector type.
// This "vector" allocates non-contiguous runs of kChunkSize T's at a time.
// Expectation is that most usage will fit in one chunk, sometimes two will be
// needed, and very rarely three. Appending is constant time, calculating the
// size is O(n_chunks).
template <typename T, size_t kChunkSize>
class ChunkedVector {
 private:
  // One chunk of allocated memory.
  struct Chunk {
    Chunk() = default;
    Chunk* next = nullptr;
    size_t count = 0;
    ManualConstructor<T> data[kChunkSize];
  };

 public:
  explicit ChunkedVector(Arena* arena) : arena_(arena) {}
  template <class Iterator>
  ChunkedVector(Arena* arena, Iterator begin, Iterator end) : arena_(arena) {
    for (; begin != end; ++begin) {
      EmplaceBack(*begin);
    }
  }
  ChunkedVector(const ChunkedVector& other)
      : ChunkedVector(other.arena_, other.begin(), other.end()) {}
  ChunkedVector& operator=(const ChunkedVector& other) {
    ChunkedVector tmp(other);
    Swap(&tmp);
    return *this;
  }
  ChunkedVector(ChunkedVector&& other) noexcept
      : arena_(other.arena_), first_(other.first_), append_(other.append_) {
    other.first_ = nullptr;
    other.append_ = nullptr;
  }
  ChunkedVector& operator=(ChunkedVector&& other) noexcept {
    Swap(&other);
    return *this;
  }
  ~ChunkedVector() { Clear(); }
  void Swap(ChunkedVector* other) {
    std::swap(other->arena_, arena_);
    std::swap(other->first_, first_);
    std::swap(other->append_, append_);
  }

  Arena* arena() const { return arena_; }

  // Append a new element to the end of the vector.
  template <typename... Args>
  T* EmplaceBack(Args&&... args) {
    auto* p = AppendSlot();
    p->Init(std::forward<Args>(args)...);
    return &**p;
  }

  // Remove the last element and return it.
  T PopBack() {
    CHECK_NE(append_, nullptr);
    if (append_->count == 0) {
      CHECK(first_ != append_);
      Chunk* chunk = first_;
      while (chunk->next != append_) {
        chunk = chunk->next;
      }
      append_ = chunk;
    }
    const auto last = append_->count - 1;
    T result = std::move(*append_->data[last]);
    append_->data[last].Destroy();
    append_->count = last;
    return result;
  }

  void Clear() {
    Chunk* chunk = first_;
    while (chunk != nullptr && chunk->count != 0) {
      for (size_t i = 0; i < chunk->count; i++) {
        chunk->data[i].Destroy();
      }
      chunk->count = 0;
      chunk = chunk->next;
    }
    append_ = first_;
  }

  // Forward-only iterator.
  class ForwardIterator {
   public:
    ForwardIterator(Chunk* chunk, size_t n) : chunk_(chunk), n_(n) {}

    using difference_type = std::ptrdiff_t;
    using iterator_category = std::forward_iterator_tag;
    using value_type = T;
    using pointer = T*;
    using reference = T&;

    T& operator*() const { return *chunk_->data[n_]; }
    T* operator->() const { return &*chunk_->data[n_]; }
    ForwardIterator& operator++() {
      ++n_;
      while (chunk_ != nullptr && n_ == chunk_->count) {
        chunk_ = chunk_->next;
        n_ = 0;
      }
      return *this;
    }
    ForwardIterator& operator++(int) {
      ForwardIterator tmp = *this;
      ++*this;
      return tmp;
    }
    bool operator==(const ForwardIterator& other) const {
      return chunk_ == other.chunk_ && n_ == other.n_;
    }
    bool operator!=(const ForwardIterator& other) const {
      return !(*this == other);
    }

   private:
    friend class ChunkedVector;

    Chunk* chunk_;
    size_t n_;
  };

  // Const Forward-only iterator.
  class ConstForwardIterator {
   public:
    ConstForwardIterator(const Chunk* chunk, size_t n) : chunk_(chunk), n_(n) {}

    using iterator_category = std::forward_iterator_tag;

    const T& operator*() const { return *chunk_->data[n_]; }
    const T* operator->() const { return &*chunk_->data[n_]; }
    ConstForwardIterator& operator++() {
      ++n_;
      while (chunk_ != nullptr && n_ == chunk_->count) {
        chunk_ = chunk_->next;
        n_ = 0;
      }
      return *this;
    }
    ConstForwardIterator& operator++(int) {
      ConstForwardIterator tmp = *this;
      ++*this;
      return tmp;
    }
    bool operator==(const ConstForwardIterator& other) const {
      return chunk_ == other.chunk_ && n_ == other.n_;
    }
    bool operator!=(const ConstForwardIterator& other) const {
      return !(*this == other);
    }

   private:
    const Chunk* chunk_;
    size_t n_;
  };

  ForwardIterator begin() {
    if (first_ != nullptr && first_->count == 0) return end();
    return ForwardIterator(first_, 0);
  }
  ForwardIterator end() { return ForwardIterator(nullptr, 0); }

  ConstForwardIterator begin() const {
    if (first_ != nullptr && first_->count == 0) return cend();
    return ConstForwardIterator(first_, 0);
  }
  ConstForwardIterator end() const { return ConstForwardIterator(nullptr, 0); }

  ConstForwardIterator cbegin() const { return begin(); }
  ConstForwardIterator cend() const { return end(); }

  // Count the number of elements in the vector.
  size_t size() const {
    size_t n = 0;
    for (const Chunk* chunk = first_; chunk != nullptr; chunk = chunk->next) {
      n += chunk->count;
    }
    return n;
  }

  // Return true if the vector is empty.
  bool empty() const { return first_ == nullptr || first_->count == 0; }

  void SetEnd(ForwardIterator it) {
    if (it == end()) return;
    Chunk* chunk = it.chunk_;
    for (size_t i = it.n_; i < chunk->count; i++) {
      chunk->data[i].Destroy();
    }
    chunk->count = it.n_;
    append_ = chunk;
    while ((chunk = chunk->next) != nullptr) {
      for (size_t i = 0; i < chunk->count; i++) {
        chunk->data[i].Destroy();
      }
      chunk->count = 0;
    }
  }

 private:
  ManualConstructor<T>* AppendSlot() {
    if (append_ == nullptr) {
      CHECK_EQ(first_, nullptr);
      first_ = arena_->New<Chunk>();
      append_ = first_;
    } else if (append_->count == kChunkSize) {
      if (append_->next == nullptr) {
        append_->next = arena_->New<Chunk>();
      }
      append_ = append_->next;
    }
    return &append_->data[append_->count++];
  }

  Arena* arena_;
  Chunk* first_ = nullptr;
  Chunk* append_ = nullptr;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_CHUNKED_VECTOR_H
