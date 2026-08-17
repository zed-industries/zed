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

#ifndef GRPC_SLICE_BUFFER_H
#define GRPC_SLICE_BUFFER_H

#include <grpc/slice.h>
#include <grpc/support/port_platform.h>

#ifdef __cplusplus
extern "C" {
#endif

/** initialize a slice buffer */
GPRAPI void grpc_slice_buffer_init(grpc_slice_buffer* sb);
/** destroy a slice buffer - unrefs any held elements */
GPRAPI void grpc_slice_buffer_destroy(grpc_slice_buffer* sb);
/** Add an element to a slice buffer - takes ownership of the slice.
   This function is allowed to concatenate the passed in slice to the end of
   some other slice if desired by the slice buffer. */
GPRAPI void grpc_slice_buffer_add(grpc_slice_buffer* sb, grpc_slice slice);
/** add an element to a slice buffer - takes ownership of the slice and returns
   the index of the slice.
   Guarantees that the slice will not be concatenated at the end of another
   slice (i.e. the data for this slice will begin at the first byte of the
   slice at the returned index in sb->slices)
   The implementation MAY decide to concatenate data at the end of a small
   slice added in this fashion. */
GPRAPI size_t grpc_slice_buffer_add_indexed(grpc_slice_buffer* sb,
                                            grpc_slice slice);
GPRAPI void grpc_slice_buffer_addn(grpc_slice_buffer* sb, grpc_slice* slices,
                                   size_t n);
/** add a very small (less than 8 bytes) amount of data to the end of a slice
   buffer: returns a pointer into which to add the data */
GPRAPI uint8_t* grpc_slice_buffer_tiny_add(grpc_slice_buffer* sb, size_t len);
/** pop the last buffer, but don't unref it */
GPRAPI void grpc_slice_buffer_pop(grpc_slice_buffer* sb);
/** clear a slice buffer, unref all elements */
GPRAPI void grpc_slice_buffer_reset_and_unref(grpc_slice_buffer* sb);
/** swap the contents of two slice buffers */
GPRAPI void grpc_slice_buffer_swap(grpc_slice_buffer* a, grpc_slice_buffer* b);
/** move all of the elements of src into dst */
GPRAPI void grpc_slice_buffer_move_into(grpc_slice_buffer* src,
                                        grpc_slice_buffer* dst);
/** remove n bytes from the end of a slice buffer */
GPRAPI void grpc_slice_buffer_trim_end(grpc_slice_buffer* sb, size_t n,
                                       grpc_slice_buffer* garbage);
/** move the first n bytes of src into dst */
GPRAPI void grpc_slice_buffer_move_first(grpc_slice_buffer* src, size_t n,
                                         grpc_slice_buffer* dst);
/** move the first n bytes of src into dst without adding references */
GPRAPI void grpc_slice_buffer_move_first_no_ref(grpc_slice_buffer* src,
                                                size_t n,
                                                grpc_slice_buffer* dst);
/** move the first n bytes of src into dst (copying them) */
GPRAPI void grpc_slice_buffer_move_first_into_buffer(grpc_slice_buffer* src,
                                                     size_t n, void* dst);
/** take the first slice in the slice buffer */
GPRAPI grpc_slice grpc_slice_buffer_take_first(grpc_slice_buffer* sb);
/** undo the above with (a possibly different) \a slice */
GPRAPI void grpc_slice_buffer_undo_take_first(grpc_slice_buffer* sb,
                                              grpc_slice slice);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_SLICE_BUFFER_H */
