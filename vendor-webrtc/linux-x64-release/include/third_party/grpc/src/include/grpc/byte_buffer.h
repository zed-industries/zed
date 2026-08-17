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

#ifndef GRPC_BYTE_BUFFER_H
#define GRPC_BYTE_BUFFER_H

#include <grpc/impl/grpc_types.h>
#include <grpc/slice_buffer.h>
#include <grpc/support/port_platform.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Returns a RAW byte buffer instance over the given slices (up to \a nslices).
 *
 * Increases the reference count for all \a slices processed. The user is
 * responsible for invoking grpc_byte_buffer_destroy on the returned instance.*/
GRPCAPI grpc_byte_buffer* grpc_raw_byte_buffer_create(grpc_slice* slices,
                                                      size_t nslices);

/** Returns a *compressed* RAW byte buffer instance over the given slices (up to
 * \a nslices). The \a compression argument defines the compression algorithm
 * used to generate the data in \a slices.
 *
 * Increases the reference count for all \a slices processed. The user is
 * responsible for invoking grpc_byte_buffer_destroy on the returned instance.*/
GRPCAPI grpc_byte_buffer* grpc_raw_compressed_byte_buffer_create(
    grpc_slice* slices, size_t nslices, grpc_compression_algorithm compression);

/** Copies input byte buffer \a bb.
 *
 * Increases the reference count of all the source slices. The user is
 * responsible for calling grpc_byte_buffer_destroy over the returned copy. */
GRPCAPI grpc_byte_buffer* grpc_byte_buffer_copy(grpc_byte_buffer* bb);

/** Returns the size of the given byte buffer, in bytes. */
GRPCAPI size_t grpc_byte_buffer_length(grpc_byte_buffer* bb);

/** Destroys \a byte_buffer deallocating all its memory. */
GRPCAPI void grpc_byte_buffer_destroy(grpc_byte_buffer* bb);

/** Reader for byte buffers. Iterates over slices in the byte buffer */
struct grpc_byte_buffer_reader;
typedef struct grpc_byte_buffer_reader grpc_byte_buffer_reader;

/** Initialize \a reader to read over \a buffer.
 * Returns 1 upon success, 0 otherwise. */
GRPCAPI int grpc_byte_buffer_reader_init(grpc_byte_buffer_reader* reader,
                                         grpc_byte_buffer* buffer);

/** Cleanup and destroy \a reader */
GRPCAPI void grpc_byte_buffer_reader_destroy(grpc_byte_buffer_reader* reader);

/** Updates \a slice with the next piece of data from from \a reader and returns
 * 1. Returns 0 at the end of the stream. Caller is responsible for calling
 * grpc_slice_unref on the result. */
GRPCAPI int grpc_byte_buffer_reader_next(grpc_byte_buffer_reader* reader,
                                         grpc_slice* slice);

/** EXPERIMENTAL API - This function may be removed and changed, in the future.
 *
 * Updates \a slice with the next piece of data from from \a reader and returns
 * 1. Returns 0 at the end of the stream. Caller is responsible for making sure
 * the slice pointer remains valid when accessed.
 *
 * NOTE: Do not use this function unless the caller can guarantee that the
 *       underlying grpc_byte_buffer outlasts the use of the slice. This is only
 *       safe when the underlying grpc_byte_buffer remains immutable while slice
 *       is being accessed. */
GRPCAPI int grpc_byte_buffer_reader_peek(grpc_byte_buffer_reader* reader,
                                         grpc_slice** slice);

/** Merge all data from \a reader into single slice */
GRPCAPI grpc_slice
grpc_byte_buffer_reader_readall(grpc_byte_buffer_reader* reader);

/** Returns a RAW byte buffer instance from the output of \a reader. */
GRPCAPI grpc_byte_buffer* grpc_raw_byte_buffer_from_reader(
    grpc_byte_buffer_reader* reader);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_BYTE_BUFFER_H */
