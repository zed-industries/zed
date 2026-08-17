/*
 * Copyright (C) 2021 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_SMALL_VECTOR_H_
#define INCLUDE_PERFETTO_EXT_BASE_SMALL_VECTOR_H_

#include <algorithm>
#include <type_traits>
#include <utility>

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/utils.h"

namespace perfetto {
namespace base {

// Uses inline storage first, switches to dynamic storage when it overflows.
template <typename T, size_t kSize>
class SmallVector {
 public:
  static constexpr size_t kInlineSize = kSize;

  explicit SmallVector() = default;

  ~SmallVector() {
    clear();
    if (PERFETTO_UNLIKELY(is_using_heap()))
      AlignedFree(begin_);
    begin_ = end_ = end_of_storage_ = nullptr;
  }

  // Move operators.
  SmallVector(SmallVector&& other) noexcept(
      std::is_nothrow_move_constructible<T>::value) {
    if (other.is_using_heap()) {
      // Move the heap content, no need to move the individual objects as their
      // location won't change.
      begin_ = other.begin_;
      end_ = other.end_;
      end_of_storage_ = other.end_of_storage_;
    } else {
      const size_t other_size = other.size();
      PERFETTO_DCHECK(other_size <= capacity());
      for (size_t i = 0; i < other_size; i++) {
        // Move the entries and destroy the ones in the moved-from object.
        new (&begin_[i]) T(std::move(other.begin_[i]));
        other.begin_[i].~T();
      }
      end_ = begin_ + other_size;
    }
    auto* const other_inline_storage = other.inline_storage_begin();
    other.end_ = other.begin_ = other_inline_storage;
    other.end_of_storage_ = other_inline_storage + kInlineSize;
  }

  SmallVector& operator=(SmallVector&& other) noexcept(
      std::is_nothrow_move_constructible<T>::value) {
    this->~SmallVector();
    new (this) SmallVector<T, kSize>(std::move(other));
    return *this;
  }

  // Copy operators.
  SmallVector(const SmallVector& other) {
    const size_t other_size = other.size();
    if (other_size > capacity())
      Grow(other_size);
    // Copy-construct the elements.
    for (size_t i = 0; i < other_size; ++i)
      new (&begin_[i]) T(other.begin_[i]);
    end_ = begin_ + other_size;
  }

  SmallVector& operator=(const SmallVector& other) {
    if (PERFETTO_UNLIKELY(this == &other))
      return *this;
    this->~SmallVector();
    new (this) SmallVector<T, kSize>(other);
    return *this;
  }

  T* data() { return begin_; }
  const T* data() const { return begin_; }

  T* begin() { return begin_; }
  const T* begin() const { return begin_; }

  T* end() { return end_; }
  const T* end() const { return end_; }

  size_t size() const { return static_cast<size_t>(end_ - begin_); }

  bool empty() const { return end_ == begin_; }

  size_t capacity() const {
    return static_cast<size_t>(end_of_storage_ - begin_);
  }

  T& front() {
    PERFETTO_DCHECK(!empty());
    return begin_[0];
  }
  const T& front() const {
    PERFETTO_DCHECK(!empty());
    return begin_[0];
  }

  T& back() {
    PERFETTO_DCHECK(!empty());
    return end_[-1];
  }
  const T& back() const {
    PERFETTO_DCHECK(!empty());
    return end_[-1];
  }

  T& operator[](size_t index) {
    PERFETTO_DCHECK(index < size());
    return begin_[index];
  }

  const T& operator[](size_t index) const {
    PERFETTO_DCHECK(index < size());
    return begin_[index];
  }

  template <typename... Args>
  void emplace_back(Args&&... args) {
    T* end = end_;
    if (PERFETTO_UNLIKELY(end == end_of_storage_))
      end = Grow();
    new (end) T(std::forward<Args>(args)...);
    end_ = end + 1;
  }

  void pop_back() {
    PERFETTO_DCHECK(!empty());
    back().~T();
    --end_;
  }

  // Clear without reverting back to inline storage.
  void clear() {
    while (!empty())
      pop_back();
  }

 private:
  PERFETTO_NO_INLINE T* Grow(size_t desired_capacity = 0) {
    size_t cur_size = size();
    size_t new_capacity = desired_capacity;
    if (desired_capacity <= cur_size)
      new_capacity = std::max(capacity() * 2, size_t(128));
    T* new_storage =
        static_cast<T*>(AlignedAlloc(alignof(T), new_capacity * sizeof(T)));
    for (size_t i = 0; i < cur_size; ++i) {
      // Move the elements into the new heap buffer and destroy the old ones.
      new (&new_storage[i]) T(std::move(begin_[i]));
      begin_[i].~T();
    }
    if (is_using_heap())
      AlignedFree(begin_);
    begin_ = new_storage;
    end_ = new_storage + cur_size;
    end_of_storage_ = new_storage + new_capacity;
    return end_;
  }

  T* inline_storage_begin() { return reinterpret_cast<T*>(inline_storage_); }
  bool is_using_heap() { return begin_ != inline_storage_begin(); }

  T* begin_ = inline_storage_begin();
  T* end_ = begin_;
  T* end_of_storage_ = begin_ + kInlineSize;

  alignas(T) char inline_storage_[sizeof(T) * kInlineSize];
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_SMALL_VECTOR_H_
