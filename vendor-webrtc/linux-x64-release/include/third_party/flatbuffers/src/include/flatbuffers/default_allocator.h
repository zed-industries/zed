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

#ifndef FLATBUFFERS_DEFAULT_ALLOCATOR_H_
#define FLATBUFFERS_DEFAULT_ALLOCATOR_H_

#include "flatbuffers/allocator.h"
#include "flatbuffers/base.h"

namespace flatbuffers {

// DefaultAllocator uses new/delete to allocate memory regions
class DefaultAllocator : public Allocator {
 public:
  uint8_t *allocate(size_t size) FLATBUFFERS_OVERRIDE {
    return new uint8_t[size];
  }

  void deallocate(uint8_t *p, size_t) FLATBUFFERS_OVERRIDE { delete[] p; }

  static void dealloc(void *p, size_t) { delete[] static_cast<uint8_t *>(p); }
};

// These functions allow for a null allocator to mean use the default allocator,
// as used by DetachedBuffer and vector_downward below.
// This is to avoid having a statically or dynamically allocated default
// allocator, or having to move it between the classes that may own it.
inline uint8_t *Allocate(Allocator *allocator, size_t size) {
  return allocator ? allocator->allocate(size)
                   : DefaultAllocator().allocate(size);
}

inline void Deallocate(Allocator *allocator, uint8_t *p, size_t size) {
  if (allocator)
    allocator->deallocate(p, size);
  else
    DefaultAllocator().deallocate(p, size);
}

inline uint8_t *ReallocateDownward(Allocator *allocator, uint8_t *old_p,
                                   size_t old_size, size_t new_size,
                                   size_t in_use_back, size_t in_use_front) {
  return allocator ? allocator->reallocate_downward(old_p, old_size, new_size,
                                                    in_use_back, in_use_front)
                   : DefaultAllocator().reallocate_downward(
                         old_p, old_size, new_size, in_use_back, in_use_front);
}

}  // namespace flatbuffers

#endif  // FLATBUFFERS_DEFAULT_ALLOCATOR_H_
