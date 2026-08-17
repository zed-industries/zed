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

#ifndef INCLUDE_PERFETTO_TRACING_INTERNAL_FNV1A_H_
#define INCLUDE_PERFETTO_TRACING_INTERNAL_FNV1A_H_

#include <cstddef>
#include <cstdint>

namespace perfetto {
namespace internal {

// Constexpr functions to compute a 64-bit hash of the input data. The algorithm
// used is FNV-1a as it is fast and easy to implement and has relatively few
// collisions.
//
// WARNING: This hash function should not be used for any cryptographic purpose.

static constexpr uint64_t kFnv1a64OffsetBasis = 0xcbf29ce484222325;
static constexpr uint64_t kFnv1a64Prime = 0x100000001b3;

static constexpr inline uint64_t Fnv1a(const char* s) {
  uint64_t ret = kFnv1a64OffsetBasis;
  for (; *s; s++) {
    ret = ret ^ static_cast<uint8_t>(*s);
    ret *= kFnv1a64Prime;
  }
  return ret;
}

static constexpr inline uint64_t Fnv1a(const void* data, size_t size) {
  uint64_t ret = kFnv1a64OffsetBasis;
  const uint8_t* s = static_cast<const uint8_t*>(data);
  for (size_t i = 0; i < size; i++) {
    ret = ret ^ s[i];
    ret *= kFnv1a64Prime;
  }
  return ret;
}

}  // namespace internal
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_INTERNAL_FNV1A_H_
