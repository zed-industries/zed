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

#ifndef SRC_PROFILING_MEMORY_UTIL_H_
#define SRC_PROFILING_MEMORY_UTIL_H_

// Make sure the alignment is the same on 32 and 64 bit architectures. This
// is to ensure the structs below are laid out in exactly the same way for
// both of those, at the same build.
// The maximum alignment of every type T is sizeof(T), so we overalign that.
// E.g., the alignment for uint64_t is 4 bytes on 32, and 8 bytes on 64 bit.
#define PERFETTO_CROSS_ABI_ALIGNED(type) alignas(sizeof(type)) type

#endif  // SRC_PROFILING_MEMORY_UTIL_H_
