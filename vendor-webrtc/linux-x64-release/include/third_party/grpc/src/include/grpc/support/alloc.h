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

#ifndef GRPC_SUPPORT_ALLOC_H
#define GRPC_SUPPORT_ALLOC_H

#include <grpc/support/port_platform.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/** malloc.
 * If size==0, always returns NULL. Otherwise this function never returns NULL.
 * The pointer returned is suitably aligned for any kind of variable it could
 * contain.
 */
GPRAPI void* gpr_malloc(size_t size);
/** like malloc, but zero all bytes before returning them */
GPRAPI void* gpr_zalloc(size_t size);
/** free */
GPRAPI void gpr_free(void* ptr);
/** realloc, never returns NULL */
GPRAPI void* gpr_realloc(void* p, size_t size);
/** aligned malloc, never returns NULL, will align to alignment, which
 * must be a power of 2. */
GPRAPI void* gpr_malloc_aligned(size_t size, size_t alignment);
/** free memory allocated by gpr_malloc_aligned */
GPRAPI void gpr_free_aligned(void* ptr);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_SUPPORT_ALLOC_H */
