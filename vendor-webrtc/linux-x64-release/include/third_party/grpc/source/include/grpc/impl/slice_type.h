/*
 *
 * Copyright 2015 gRPC authors.
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
 *
 */

#ifndef GRPC_IMPL_SLICE_TYPE_H
#define GRPC_IMPL_SLICE_TYPE_H

// IWYU pragma: private, include <grpc/slice.h>

#include <grpc/support/port_platform.h>
#include <stddef.h>

typedef struct grpc_slice grpc_slice;

/** Slice API

   A slice represents a contiguous reference counted array of bytes.
   It is cheap to take references to a slice, and it is cheap to create a
   slice pointing to a subset of another slice.

   The data-structure for slices is exposed here to allow non-gpr code to
   build slices from whatever data they have available.

   When defining interfaces that handle slices, care should be taken to define
   reference ownership semantics (who should call unref?) and mutability
   constraints (is the callee allowed to modify the slice?) */

/* Inlined half of grpc_slice is allowed to expand the size of the overall type
   by this many bytes */
#define GRPC_SLICE_INLINE_EXTRA_SIZE sizeof(void*)

#define GRPC_SLICE_INLINED_SIZE \
  (sizeof(size_t) + sizeof(uint8_t*) - 1 + GRPC_SLICE_INLINE_EXTRA_SIZE)

struct grpc_slice_refcount;
/** A grpc_slice s, if initialized, represents the byte range
   s.bytes[0..s.length-1].

   It can have an associated ref count which has a destruction routine to be run
   when the ref count reaches zero (see grpc_slice_new() and grp_slice_unref()).
   Multiple grpc_slice values may share a ref count.

   If the slice does not have a refcount, it represents an inlined small piece
   of data that is copied by value.

   As a special case, a slice can be given refcount == uintptr_t(1), meaning
   that the slice represents external data that is not refcounted. */
struct grpc_slice {
  struct grpc_slice_refcount* refcount;
  union grpc_slice_data {
    struct grpc_slice_refcounted {
      size_t length;
      uint8_t* bytes;
    } refcounted;
    struct grpc_slice_inlined {
      uint8_t length;
      uint8_t bytes[GRPC_SLICE_INLINED_SIZE];
    } inlined;
  } data;
};

#define GRPC_SLICE_BUFFER_INLINE_ELEMENTS 6

/** Represents an expandable array of slices, to be interpreted as a
   single item. */
typedef struct grpc_slice_buffer {
  /** This is for internal use only. External users (i.e any code outside grpc
   * core) MUST NOT use this field */
  grpc_slice* base_slices;

  /** slices in the array (Points to the first valid grpc_slice in the array) */
  grpc_slice* slices;
  /** the number of slices in the array */
  size_t count;
  /** the number of slices allocated in the array. External users (i.e any code
   * outside grpc core) MUST NOT use this field */
  size_t capacity;
  /** the combined length of all slices in the array */
  size_t length;
  /** inlined elements to avoid allocations */
  grpc_slice inlined[GRPC_SLICE_BUFFER_INLINE_ELEMENTS];
} grpc_slice_buffer;

#define GRPC_SLICE_START_PTR(slice)                 \
  ((slice).refcount ? (slice).data.refcounted.bytes \
                    : (slice).data.inlined.bytes)
#define GRPC_SLICE_LENGTH(slice)                     \
  ((slice).refcount ? (slice).data.refcounted.length \
                    : (slice).data.inlined.length)
#define GRPC_SLICE_SET_LENGTH(slice, newlen)                              \
  ((slice).refcount ? ((slice).data.refcounted.length = (size_t)(newlen)) \
                    : ((slice).data.inlined.length = (uint8_t)(newlen)))
#define GRPC_SLICE_END_PTR(slice) \
  GRPC_SLICE_START_PTR(slice) + GRPC_SLICE_LENGTH(slice)
#define GRPC_SLICE_IS_EMPTY(slice) (GRPC_SLICE_LENGTH(slice) == 0)

#endif /* GRPC_IMPL_SLICE_TYPE_H */
