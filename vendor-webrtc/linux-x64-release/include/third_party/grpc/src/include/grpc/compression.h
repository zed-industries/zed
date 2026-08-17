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

#ifndef GRPC_COMPRESSION_H
#define GRPC_COMPRESSION_H

#include <grpc/impl/compression_types.h>  // IWYU pragma: export
#include <grpc/slice.h>
#include <grpc/support/port_platform.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Return if an algorithm is message compression algorithm. */
GRPCAPI int grpc_compression_algorithm_is_message(
    grpc_compression_algorithm algorithm);

/** Return if an algorithm is stream compression algorithm. */
GRPCAPI int grpc_compression_algorithm_is_stream(
    grpc_compression_algorithm algorithm);

/** Parses the \a slice as a grpc_compression_algorithm instance and updating \a
 * algorithm. Returns 1 upon success, 0 otherwise. */
GRPCAPI int grpc_compression_algorithm_parse(
    grpc_slice name, grpc_compression_algorithm* algorithm);

/** Updates \a name with the encoding name corresponding to a valid \a
 * algorithm. Note that \a name is statically allocated and must *not* be freed.
 * Returns 1 upon success, 0 otherwise. */
GRPCAPI int grpc_compression_algorithm_name(
    grpc_compression_algorithm algorithm, const char** name);

/** Returns the compression algorithm corresponding to \a level for the
 * compression algorithms encoded in the \a accepted_encodings bitset.*/
GRPCAPI grpc_compression_algorithm grpc_compression_algorithm_for_level(
    grpc_compression_level level, uint32_t accepted_encodings);

GRPCAPI void grpc_compression_options_init(grpc_compression_options* opts);

/** Mark \a algorithm as enabled in \a opts. */
GRPCAPI void grpc_compression_options_enable_algorithm(
    grpc_compression_options* opts, grpc_compression_algorithm algorithm);

/** Mark \a algorithm as disabled in \a opts. */
GRPCAPI void grpc_compression_options_disable_algorithm(
    grpc_compression_options* opts, grpc_compression_algorithm algorithm);

/** Returns true if \a algorithm is marked as enabled in \a opts. */
GRPCAPI int grpc_compression_options_is_algorithm_enabled(
    const grpc_compression_options* opts, grpc_compression_algorithm algorithm);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_COMPRESSION_H */
