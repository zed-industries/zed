/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_PROFILING_MEMORY_WRAP_ALLOCATORS_H_
#define SRC_PROFILING_MEMORY_WRAP_ALLOCATORS_H_

#include <stdlib.h>

#include <cinttypes>

namespace perfetto {
namespace profiling {

void* wrap_malloc(uint32_t heap_id, void* (*fn)(size_t), size_t size);
void* wrap_calloc(uint32_t heap_id,
                  void* (*fn)(size_t, size_t),
                  size_t nmemb,
                  size_t size);
void* wrap_memalign(uint32_t heap_id,
                    void* (*fn)(size_t, size_t),
                    size_t alignment,
                    size_t size);
int wrap_posix_memalign(uint32_t heap_id,
                        int (*fn)(void**, size_t, size_t),
                        void** memptr,
                        size_t alignment,
                        size_t size);
void wrap_free(uint32_t heap_id, void (*fn)(void*), void* pointer);
void* wrap_realloc(uint32_t heap_id,
                   void* (*fn)(void*, size_t),
                   void* pointer,
                   size_t size);
void* wrap_pvalloc(uint32_t heap_id, void* (*fn)(size_t), size_t size);
void* wrap_valloc(uint32_t heap_id, void* (*fn)(size_t), size_t size);

void* wrap_reallocarray(uint32_t heap_id,
                        void* (*fn)(void*, size_t, size_t),
                        void* pointer,
                        size_t nmemb,
                        size_t size);

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_MEMORY_WRAP_ALLOCATORS_H_
