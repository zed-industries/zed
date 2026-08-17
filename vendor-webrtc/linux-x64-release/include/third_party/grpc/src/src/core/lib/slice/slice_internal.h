//
//
// Copyright 2016 gRPC authors.
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
//
//

#ifndef GRPC_SRC_CORE_LIB_SLICE_SLICE_INTERNAL_H
#define GRPC_SRC_CORE_LIB_SLICE_SLICE_INTERNAL_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <cstddef>
#include <string>

#include "absl/hash/hash.h"
#include "absl/log/check.h"
#include "absl/strings/string_view.h"
#include "src/core/util/memory.h"

// Returns a pointer to the first slice in the slice buffer without giving
// ownership to or a reference count on that slice.
inline grpc_slice* grpc_slice_buffer_peek_first(grpc_slice_buffer* sb) {
  DCHECK_GT(sb->count, 0u);
  return &sb->slices[0];
}

// Removes the first slice from the slice buffer.
void grpc_slice_buffer_remove_first(grpc_slice_buffer* sb);

// Calls grpc_slice_sub with the given parameters on the first slice.
void grpc_slice_buffer_sub_first(grpc_slice_buffer* sb, size_t begin,
                                 size_t end);

// if slice matches a static slice, returns the static slice
// otherwise returns the passed in slice (without reffing it)
// used for surface boundaries where we might receive an un-interned static
// string
grpc_slice grpc_slice_maybe_static_intern(grpc_slice slice,
                                          bool* returned_slice_is_different);
uint32_t grpc_static_slice_hash(grpc_slice s);
int grpc_static_slice_eq(grpc_slice a, grpc_slice b);

grpc_slice grpc_slice_from_moved_buffer(grpc_core::UniquePtr<char> p,
                                        size_t len);
grpc_slice grpc_slice_from_moved_string(grpc_core::UniquePtr<char> p);
grpc_slice grpc_slice_from_cpp_string(std::string str);

// Returns the memory used by this slice, not counting the slice structure
// itself. This means that inlined and slices from static strings will return
// 0. All other slices will return the size of the allocated chars.
size_t grpc_slice_memory_usage(grpc_slice s);

grpc_slice grpc_slice_split_tail_maybe_ref_no_inline(
    grpc_slice* source, size_t split, grpc_slice_ref_whom ref_whom);

grpc_slice grpc_slice_split_tail_no_inline(grpc_slice* source, size_t split);

grpc_slice grpc_slice_split_head_no_inline(grpc_slice* source, size_t split);

namespace grpc_core {

// Converts grpc_slice to absl::string_view.
inline absl::string_view StringViewFromSlice(const grpc_slice& slice) {
  return absl::string_view(
      reinterpret_cast<const char*>(GRPC_SLICE_START_PTR(slice)),
      GRPC_SLICE_LENGTH(slice));
}

}  // namespace grpc_core

inline uint32_t grpc_slice_hash(const grpc_slice& s) {
  return absl::HashOf(grpc_core::StringViewFromSlice(s));
}

namespace grpc_core {
struct SliceHash {
  std::size_t operator()(const grpc_slice& slice) const {
    return grpc_slice_hash(slice);
  }
};
}  // namespace grpc_core

inline bool operator==(const grpc_slice& s1, const grpc_slice& s2) {
  return grpc_slice_eq(s1, s2);
}

#endif  // GRPC_SRC_CORE_LIB_SLICE_SLICE_INTERNAL_H
