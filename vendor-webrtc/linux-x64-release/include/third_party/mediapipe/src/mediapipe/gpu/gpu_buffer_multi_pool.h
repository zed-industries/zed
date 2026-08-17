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

// This class lets calculators allocate GpuBuffers of various sizes, caching
// and reusing them as needed. It does so by automatically creating and using
// platform-specific buffer pools for the requested sizes.
//
// This class is not meant to be used directly by calculators, but is instead
// used by GlCalculatorHelper to allocate buffers.

#ifndef MEDIAPIPE_GPU_GPU_BUFFER_MULTI_POOL_H_
#define MEDIAPIPE_GPU_GPU_BUFFER_MULTI_POOL_H_

#include "absl/status/statusor.h"
#include "mediapipe/gpu/gpu_buffer.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/multi_pool.h"

#if MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
#include "mediapipe/gpu/cv_pixel_buffer_pool_wrapper.h"
#else
#include "mediapipe/gpu/gl_texture_buffer_pool.h"
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER

namespace mediapipe {

class CvPixelBufferPoolWrapper;

class GpuBufferMultiPool : public MultiPool<
#if MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
                               CvPixelBufferPoolWrapper,
#else
                               GlTextureBufferPool,
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
                               internal::GpuBufferSpec, GpuBuffer> {
 public:
  using MultiPool::MultiPool;

  absl::StatusOr<GpuBuffer> GetBuffer(
      int width, int height,
      GpuBufferFormat format = GpuBufferFormat::kBGRA32) {
    return Get(internal::GpuBufferSpec(width, height, format));
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_BUFFER_MULTI_POOL_H_
