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

#ifndef GRPC_SLICE_H
#define GRPC_SLICE_H

#include <grpc/impl/slice_type.h>  // IWYU pragma: export
#include <grpc/support/port_platform.h>
#include <grpc/support/sync.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Increment the refcount of s. Requires slice is initialized.
   Returns s. */
GPRAPI grpc_slice grpc_slice_ref(grpc_slice s);

/** Decrement the ref count of s.  If the ref count of s reaches zero, all
   slices sharing the ref count are destroyed, and considered no longer
   initialized.  If s is ultimately derived from a call to grpc_slice_new(start,
   len, dest) where dest!=NULL , then (*dest)(start) is called, else if s is
   ultimately derived from a call to grpc_slice_new_with_len(start, len, dest)
   where dest!=NULL , then (*dest)(start, len).  Requires s initialized.  */
GPRAPI void grpc_slice_unref(grpc_slice s);

/** Copy slice - create a new slice that contains the same data as s */
GPRAPI grpc_slice grpc_slice_copy(grpc_slice s);

/** Create a slice pointing at some data. Calls malloc to allocate a refcount
   for the object, and arranges that destroy will be called with the pointer
   passed in at destruction. */
GPRAPI grpc_slice grpc_slice_new(void* p, size_t len, void (*destroy)(void*));

/** Equivalent to grpc_slice_new, but with a separate pointer that is
   passed to the destroy function.  This function can be useful when
   the data is part of a larger structure that must be destroyed when
   the data is no longer needed. */
GPRAPI grpc_slice grpc_slice_new_with_user_data(void* p, size_t len,
                                                void (*destroy)(void*),
                                                void* user_data);

/** Equivalent to grpc_slice_new, but with a two argument destroy function that
   also takes the slice length. */
GPRAPI grpc_slice grpc_slice_new_with_len(void* p, size_t len,
                                          void (*destroy)(void*, size_t));

/** Equivalent to grpc_slice_new(malloc(len), len, free), but saves one malloc()
   call.
   Aborts if malloc() fails. */
GPRAPI grpc_slice grpc_slice_malloc(size_t length);
GPRAPI grpc_slice grpc_slice_malloc_large(size_t length);

#define GRPC_SLICE_MALLOC(len) grpc_slice_malloc(len)

/** Create a slice by copying a string.
   Does not preserve null terminators.
   Equivalent to:
     size_t len = strlen(source);
     grpc_slice slice = grpc_slice_malloc(len);
     memcpy(slice->data, source, len); */
GPRAPI grpc_slice grpc_slice_from_copied_string(const char* source);

/** Create a slice by copying a buffer.
   Equivalent to:
     grpc_slice slice = grpc_slice_malloc(len);
     memcpy(slice->data, source, len); */
GPRAPI grpc_slice grpc_slice_from_copied_buffer(const char* source, size_t len);

/** Create a slice pointing to constant memory */
GPRAPI grpc_slice grpc_slice_from_static_string(const char* source);

/** Create a slice pointing to constant memory */
GPRAPI grpc_slice grpc_slice_from_static_buffer(const void* source, size_t len);

/** Return a result slice derived from s, which shares a ref count with \a s,
   where result.data==s.data+begin, and result.length==end-begin. The ref count
   of \a s is increased by one. Do not assign result back to \a s.
   Requires s initialized, begin <= end, begin <= s.length, and
   end <= source->length. */
GPRAPI grpc_slice grpc_slice_sub(grpc_slice s, size_t begin, size_t end);

/** The same as grpc_slice_sub, but without altering the ref count */
GPRAPI grpc_slice grpc_slice_sub_no_ref(grpc_slice s, size_t begin, size_t end);

/** Splits s into two: modifies s to be s[0:split], and returns a new slice,
   sharing a refcount with s, that contains s[split:s.length].
   Requires s initialized, split <= s.length */
GPRAPI grpc_slice grpc_slice_split_tail(grpc_slice* s, size_t split);

typedef enum {
  GRPC_SLICE_REF_TAIL = 1,
  GRPC_SLICE_REF_HEAD = 2,
  GRPC_SLICE_REF_BOTH = 1 + 2
} grpc_slice_ref_whom;

/** The same as grpc_slice_split_tail, but with an option to skip altering
 * refcounts (grpc_slice_split_tail_maybe_ref(..., true) is equivalent to
 * grpc_slice_split_tail(...)) */
GPRAPI grpc_slice grpc_slice_split_tail_maybe_ref(grpc_slice* s, size_t split,
                                                  grpc_slice_ref_whom ref_whom);

/** Splits s into two: modifies s to be s[split:s.length], and returns a new
   slice, sharing a refcount with s, that contains s[0:split].
   Requires s initialized, split <= s.length */
GPRAPI grpc_slice grpc_slice_split_head(grpc_slice* s, size_t split);

GPRAPI grpc_slice grpc_empty_slice(void);

GPRAPI int grpc_slice_eq(grpc_slice a, grpc_slice b);

/** Returns <0 if a < b, ==0 if a == b, >0 if a > b
   The order is arbitrary, and is not guaranteed to be stable across different
   versions of the API. */
GPRAPI int grpc_slice_cmp(grpc_slice a, grpc_slice b);
GPRAPI int grpc_slice_str_cmp(grpc_slice a, const char* b);

/** return non-zero if the first blen bytes of a are equal to b */
GPRAPI int grpc_slice_buf_start_eq(grpc_slice a, const void* b, size_t blen);

/** return the index of the last instance of \a c in \a s, or -1 if not found */
GPRAPI int grpc_slice_rchr(grpc_slice s, char c);
GPRAPI int grpc_slice_chr(grpc_slice s, char c);

/** return the index of the first occurrence of \a needle in \a haystack, or -1
   if it's not found */
GPRAPI int grpc_slice_slice(grpc_slice haystack, grpc_slice needle);

/** Do two slices point at the same memory, with the same length
   If a or b is inlined, actually compares data */
GPRAPI int grpc_slice_is_equivalent(grpc_slice a, grpc_slice b);

/** Return a slice pointing to newly allocated memory that has the same contents
 * as \a s */
GPRAPI grpc_slice grpc_slice_dup(grpc_slice a);

/** Return a copy of slice as a C string. Offers no protection against embedded
   NULL's. Returned string must be freed with gpr_free. */
GPRAPI char* grpc_slice_to_c_string(grpc_slice s);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_SLICE_H */
