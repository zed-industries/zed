// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_RING_BUFFER_H
#define GRPC_SRC_CORE_UTIL_RING_BUFFER_H

#include <grpc/support/port_platform.h>

#include <array>
#include <cstddef>
#include <iterator>
#include <optional>

namespace grpc_core {

template <typename T, int kCapacity>
class RingBuffer {
 public:
  class RingBufferIterator {
   public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = const T;
    using pointer = void;
    using reference = void;
    using difference_type = std::ptrdiff_t;

    RingBufferIterator& operator++() {
      if (--size_ <= 0) {
        head_ = 0;
        size_ = 0;
        buffer_ = nullptr;
      } else {
        head_ = (head_ + 1) % kCapacity;
      }
      return *this;
    }

    RingBufferIterator operator++(int) {
      RingBufferIterator tmp(*this);
      operator++();
      return tmp;
    }

    bool operator==(const RingBufferIterator& rhs) const {
      return (buffer_ == rhs.buffer_ && head_ == rhs.head_ &&
              size_ == rhs.size_);
    }

    bool operator!=(const RingBufferIterator& rhs) const {
      return !operator==(rhs);
    }

    T operator*() { return buffer_->data_[head_]; }

    RingBufferIterator() : buffer_(nullptr), head_(0), size_(0) {};
    RingBufferIterator(const RingBufferIterator& other) = default;
    explicit RingBufferIterator(const RingBuffer<T, kCapacity>* buffer)
        : buffer_(buffer), head_(buffer->head_), size_(buffer->size_) {
      if (!size_) {
        buffer_ = nullptr;
      }
    }

   private:
    friend class RingBuffer<T, kCapacity>;
    const RingBuffer<T, kCapacity>* buffer_;
    int head_ = 0;
    int size_ = 0;
  };

  RingBuffer() = default;

  void Append(T data) {
    if (size_ < kCapacity) {
      data_[size_] = std::move(data);
      size_++;
    } else {
      data_[head_] = std::move(data);
      head_ = (head_ + 1) % kCapacity;
    }
  }

  // Returns the data of the first element in the buffer and removes it from
  // the buffer. If the buffer is empty, returns std::nullopt.
  std::optional<T> PopIfNotEmpty() {
    if (!size_) return std::nullopt;
    T data = std::move(data_[head_]);
    --size_;
    head_ = (head_ + 1) % kCapacity;
    return data;
  }

  void Clear() {
    head_ = 0;
    size_ = 0;
  }

  RingBufferIterator begin() const { return RingBufferIterator(this); }

  RingBufferIterator end() const { return RingBufferIterator(); }

 private:
  friend class RingBufferIterator;
  std::array<T, kCapacity> data_;
  int head_ = 0;
  int size_ = 0;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_RING_BUFFER_H
