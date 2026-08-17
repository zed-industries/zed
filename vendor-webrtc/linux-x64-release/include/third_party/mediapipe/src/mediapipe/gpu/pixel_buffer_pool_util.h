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

#ifndef MEDIAPIPE_GPU_PIXEL_BUFFER_POOL_UTIL_H_
#define MEDIAPIPE_GPU_PIXEL_BUFFER_POOL_UTIL_H_

#include <CoreVideo/CoreVideo.h>
#include <TargetConditionals.h>

#include <functional>

#ifndef __APPLE__
#error gpu_pixel_buffer_pool_util is only for use on Apple platforms.
#endif  // !defined(__APPLE__)

namespace mediapipe {

#if TARGET_OS_OSX
typedef CVOpenGLTextureCacheRef CVTextureCacheType;
#else
typedef CVOpenGLESTextureCacheRef CVTextureCacheType;
#endif  // TARGET_OS_OSX

// Create a CVPixelBufferPool.
CVPixelBufferPoolRef CreateCVPixelBufferPool(int width, int height,
                                             OSType pixel_format,
                                             int keep_count,
                                             CFTimeInterval maxAge);

// Preallocate the given number of pixel buffers.
OSStatus PreallocateCVPixelBufferPoolBuffers(CVPixelBufferPoolRef pool,
                                             int count,
                                             CFDictionaryRef auxAttributes);

// Create a CVPixelBuffer using a pool.
// If the pool is full, will flush the provided texture cache before trying
// again.
CVReturn CreateCVPixelBufferWithPool(CVPixelBufferPoolRef pool,
                                     CFDictionaryRef auxAttributes,
                                     CVTextureCacheType textureCache,
                                     CVPixelBufferRef* outBuffer);

// Create a CVPixelBuffer using a pool.
// If the pool is full, will call the provided function before trying again.
CVReturn CreateCVPixelBufferWithPool(CVPixelBufferPoolRef pool,
                                     CFDictionaryRef auxAttributes,
                                     std::function<void(void)> flush,
                                     CVPixelBufferRef* outBuffer);

// Create an auxiliary attribute dictionary, which can be used with
// CVPixelBufferPool, specifying the given allocation threshold.
CFDictionaryRef CreateCVPixelBufferPoolAuxiliaryAttributesForThreshold(
    int allocationThreshold);

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_PIXEL_BUFFER_POOL_UTIL_H_
