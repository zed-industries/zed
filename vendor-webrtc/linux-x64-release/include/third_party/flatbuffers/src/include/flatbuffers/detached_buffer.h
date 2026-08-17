/*
 * Copyright 2021 Google Inc. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef FLATBUFFERS_DETACHED_BUFFER_H_
#define FLATBUFFERS_DETACHED_BUFFER_H_

#include "flatbuffers/allocator.h"
#include "flatbuffers/base.h"
#include "flatbuffers/default_allocator.h"

namespace flatbuffers {

// DetachedBuffer is a finished flatbuffer memory region, detached from its
// builder. The original memory region and allocator are also stored so that
// the DetachedBuffer can manage the memory lifetime.
class DetachedBuffer {
 public:
  DetachedBuffer()
      : allocator_(nullptr),
        own_allocator_(false),
        buf_(nullptr),
        reserved_(0),
        cur_(nullptr),
        size_(0) {}

  DetachedBuffer(Allocator *allocator, bool own_allocator, uint8_t *buf,
                 size_t reserved, uint8_t *cur, size_t sz)
      : allocator_(allocator),
        own_allocator_(own_allocator),
        buf_(buf),
        reserved_(reserved),
        cur_(cur),
        size_(sz) {}

  DetachedBuffer(DetachedBuffer &&other) noexcept
      : allocator_(other.allocator_),
        own_allocator_(other.own_allocator_),
        buf_(other.buf_),
        reserved_(other.reserved_),
        cur_(other.cur_),
        size_(other.size_) {
    other.reset();
  }

  DetachedBuffer &operator=(DetachedBuffer &&other) noexcept {
    if (this == &other) return *this;

    destroy();

    allocator_ = other.allocator_;
    own_allocator_ = other.own_allocator_;
    buf_ = other.buf_;
    reserved_ = other.reserved_;
    cur_ = other.cur_;
    size_ = other.size_;

    other.reset();

    return *this;
  }

  ~DetachedBuffer() { destroy(); }

  const uint8_t *data() const { return cur_; }

  uint8_t *data() { return cur_; }

  size_t size() const { return size_; }

  uint8_t *begin() { return data(); }
  const uint8_t *begin() const { return data(); }
  uint8_t *end() { return data() + size(); }
  const uint8_t *end() const { return data() + size(); }

  // These may change access mode, leave these at end of public section
  FLATBUFFERS_DELETE_FUNC(DetachedBuffer(const DetachedBuffer &other));
  FLATBUFFERS_DELETE_FUNC(
      DetachedBuffer &operator=(const DetachedBuffer &other));

 protected:
  Allocator *allocator_;
  bool own_allocator_;
  uint8_t *buf_;
  size_t reserved_;
  uint8_t *cur_;
  size_t size_;

  inline void destroy() {
    if (buf_) Deallocate(allocator_, buf_, reserved_);
    if (own_allocator_ && allocator_) { delete allocator_; }
    reset();
  }

  inline void reset() {
    allocator_ = nullptr;
    own_allocator_ = false;
    buf_ = nullptr;
    reserved_ = 0;
    cur_ = nullptr;
    size_ = 0;
  }
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_DETACHED_BUFFER_H_
