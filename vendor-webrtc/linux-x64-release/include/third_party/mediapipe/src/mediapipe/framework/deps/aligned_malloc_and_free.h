// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_DEPS_ALIGNED_MALLOC_AND_FREE_H_
#define MEDIAPIPE_DEPS_ALIGNED_MALLOC_AND_FREE_H_

#include <stdlib.h>  // for free(), aligned_alloc(),

#if defined(__ANDROID__) || defined(_WIN32)
#include <malloc.h>  // for memalign() on Android, _aligned_alloc() on Windows
#endif

inline void *aligned_malloc(size_t size, int minimum_alignment) {
#if defined(__ANDROID__) || defined(OS_ANDROID)
  return memalign(minimum_alignment, size);
#elif _WIN32
  return _aligned_malloc(size, minimum_alignment);
#else  // !__ANDROID__ && !OS_ANDROID && !_WIN32
  void *ptr = nullptr;
  // posix_memalign requires that the requested alignment be at least
  // sizeof(void*). In this case, fall back on malloc which should return memory
  // aligned to at least the size of a pointer.
  const int required_alignment = sizeof(void *);
  if (minimum_alignment < required_alignment) return malloc(size);
  if (posix_memalign(&ptr, static_cast<size_t>(minimum_alignment), size) != 0)
    return nullptr;
  else
    return ptr;
#endif
}

inline void aligned_free(void *aligned_memory) {
#ifdef _WIN32
  _aligned_free(aligned_memory);
#else
  free(aligned_memory);
#endif  // _WIN32
}

#endif  // MEDIAPIPE_DEPS_ALIGNED_MALLOC_AND_FREE_H_
