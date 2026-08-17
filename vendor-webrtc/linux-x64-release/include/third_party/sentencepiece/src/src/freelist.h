// Copyright 2018 Google Inc.
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
// limitations under the License.!

#ifndef FREELIST_H_
#define FREELIST_H_

#include <string.h>

#include <vector>

namespace sentencepiece {
namespace model {

// Simple FreeList that allocates a chunk of T at once.
template <class T>
class FreeList {
 public:
  FreeList() = delete;
  explicit FreeList(size_t chunk_size) : chunk_size_(chunk_size) {}
  virtual ~FreeList() {
    for (auto& chunk : freelist_) delete[] chunk;
  }

  // `Free` doesn't free the object but reuse the allocated memory chunks.
  void Free() {
    const int size = std::min<int>(chunk_index_ + 1, freelist_.size());
    for (int i = 0; i < size; ++i) {
      T* chunk = freelist_[i];
      memset(static_cast<void*>(chunk), 0, sizeof(*chunk) * chunk_size_);
    }
    chunk_index_ = 0;
    element_index_ = 0;
  }

  // Returns the number of allocated elements.
  size_t size() const { return chunk_size_ * chunk_index_ + element_index_; }

  void swap(FreeList<T>& other) {
    std::swap(freelist_, other.freelist_);
    std::swap(element_index_, other.element_index_);
    std::swap(chunk_index_, other.chunk_index_);
    std::swap(chunk_size_, other.chunk_size_);
  }

  // Returns the element as an array.
  T* operator[](size_t index) const {
    return freelist_[index / chunk_size_] + index % chunk_size_;
  }

  // Allocates new element.
  T* Allocate() {
    if (element_index_ >= chunk_size_) {
      ++chunk_index_;
      element_index_ = 0;
    }

    if (chunk_index_ == freelist_.size()) {
      T* chunk = new T[chunk_size_];
      memset(static_cast<void*>(chunk), 0, sizeof(*chunk) * chunk_size_);
      freelist_.push_back(chunk);
    }

    T* result = freelist_[chunk_index_] + element_index_;
    ++element_index_;

    return result;
  }

 private:
  std::vector<T*> freelist_;

  // The last element is stored at freelist_[chunk_index_][element_index_]
  size_t element_index_ = 0;
  size_t chunk_index_ = 0;
  size_t chunk_size_ = 0;  // Do not modify except in swap()
};
}  // namespace model
}  // namespace sentencepiece
#endif  // FREELIST_H_
