/*
 * Copyright (C) 2017 The Android Open Source Project
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

#ifndef SRC_TRACING_CORE_ID_ALLOCATOR_H_
#define SRC_TRACING_CORE_ID_ALLOCATOR_H_

#include <stdint.h>

#include <cstddef>
#include <type_traits>
#include <vector>

namespace perfetto {

// Handles assignment of IDs (int types) from a fixed-size pool.
// Zero is not considered a valid ID.
// The base class takes always a uint32_t and the derived class casts and checks
// bounds at compile time. This is to avoid bloating code with different
// instances of the main class for each size.
class IdAllocatorGeneric {
 public:
  // |max_id| is inclusive.
  explicit IdAllocatorGeneric(uint32_t max_id);
  ~IdAllocatorGeneric();

  // Returns an ID in the range [1, max_id] or 0 if no more ids are available.
  uint32_t AllocateGeneric();
  void FreeGeneric(uint32_t);

  bool IsEmpty() const;

 private:
  IdAllocatorGeneric(const IdAllocatorGeneric&) = delete;
  IdAllocatorGeneric& operator=(const IdAllocatorGeneric&) = delete;

  const uint32_t max_id_;
  uint32_t last_id_ = 0;
  std::vector<bool> ids_;
};

template <typename T = uint32_t>
class IdAllocator : public IdAllocatorGeneric {
 public:
  explicit IdAllocator(T end) : IdAllocatorGeneric(end) {
    static_assert(std::is_integral<T>::value && std::is_unsigned<T>::value,
                  "T must be an unsigned integer");
    static_assert(sizeof(T) <= sizeof(uint32_t), "T is too big");
  }

  T Allocate() { return static_cast<T>(AllocateGeneric()); }

  // Tries to allocate `n` IDs. Returns a vector of `n` valid IDs or an empty
  // vector, if not enough IDs are available.
  std::vector<T> AllocateMultiple(size_t n) {
    std::vector<T> res;
    res.reserve(n);
    for (size_t i = 0; i < n; i++) {
      T id = Allocate();
      if (id) {
        res.push_back(id);
      } else {
        for (T free_id : res) {
          Free(free_id);
        }
        return {};
      }
    }
    return res;
  }

  void Free(T id) { FreeGeneric(id); }
};

}  // namespace perfetto

#endif  // SRC_TRACING_CORE_ID_ALLOCATOR_H_
