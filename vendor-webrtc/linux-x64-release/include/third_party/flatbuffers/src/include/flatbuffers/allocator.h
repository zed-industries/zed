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

#ifndef FLATBUFFERS_ALLOCATOR_H_
#define FLATBUFFERS_ALLOCATOR_H_

#include "flatbuffers/base.h"

namespace flatbuffers {

// Allocator interface. This is flatbuffers-specific and meant only for
// `vector_downward` usage.
class Allocator {
 public:
  virtual ~Allocator() {}

  // Allocate `size` bytes of memory.
  virtual uint8_t *allocate(size_t size) = 0;

  // Deallocate `size` bytes of memory at `p` allocated by this allocator.
  virtual void deallocate(uint8_t *p, size_t size) = 0;

  // Reallocate `new_size` bytes of memory, replacing the old region of size
  // `old_size` at `p`. In contrast to a normal realloc, this grows downwards,
  // and is intended specifcally for `vector_downward` use.
  // `in_use_back` and `in_use_front` indicate how much of `old_size` is
  // actually in use at each end, and needs to be copied.
  virtual uint8_t *reallocate_downward(uint8_t *old_p, size_t old_size,
                                       size_t new_size, size_t in_use_back,
                                       size_t in_use_front) {
    FLATBUFFERS_ASSERT(new_size > old_size);  // vector_downward only grows
    uint8_t *new_p = allocate(new_size);
    memcpy_downward(old_p, old_size, new_p, new_size, in_use_back,
                    in_use_front);
    deallocate(old_p, old_size);
    return new_p;
  }

 protected:
  // Called by `reallocate_downward` to copy memory from `old_p` of `old_size`
  // to `new_p` of `new_size`. Only memory of size `in_use_front` and
  // `in_use_back` will be copied from the front and back of the old memory
  // allocation.
  void memcpy_downward(uint8_t *old_p, size_t old_size, uint8_t *new_p,
                       size_t new_size, size_t in_use_back,
                       size_t in_use_front) {
    memcpy(new_p + new_size - in_use_back, old_p + old_size - in_use_back,
           in_use_back);
    memcpy(new_p, old_p, in_use_front);
  }
};

}  // namespace flatbuffers

#endif  // FLATBUFFERS_ALLOCATOR_H_
