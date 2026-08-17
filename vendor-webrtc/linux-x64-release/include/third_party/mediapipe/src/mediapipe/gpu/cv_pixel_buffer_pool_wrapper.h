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

#ifndef MEDIAPIPE_GPU_CV_PIXEL_BUFFER_POOL_WRAPPER_H_
#define MEDIAPIPE_GPU_CV_PIXEL_BUFFER_POOL_WRAPPER_H_

#include <memory>
#include <string>

#include "CoreFoundation/CFBase.h"
#include "absl/status/statusor.h"
#include "mediapipe/gpu/cv_texture_cache_manager.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/multi_pool.h"
#include "mediapipe/gpu/pixel_buffer_pool_util.h"
#include "mediapipe/objc/CFHolder.h"

namespace mediapipe {

class CvPixelBufferPoolWrapper {
 public:
  CvPixelBufferPoolWrapper(int width, int height, GpuBufferFormat format,
                           CFTimeInterval maxAge,
                           CvTextureCacheManager* texture_caches);

  static std::shared_ptr<CvPixelBufferPoolWrapper> Create(
      const internal::GpuBufferSpec& spec, const MultiPoolOptions& options,
      CvTextureCacheManager* texture_caches = nullptr) {
    return std::make_shared<CvPixelBufferPoolWrapper>(
        spec.width, spec.height, spec.format, options.max_inactive_buffer_age,
        texture_caches);
  }

  absl::StatusOr<CFHolder<CVPixelBufferRef>> GetBuffer();

  int GetBufferCount() const { return count_; }
  std::string GetDebugString() const;

  void Flush();

  static absl::StatusOr<CFHolder<CVPixelBufferRef>> CreateBufferWithoutPool(
      const internal::GpuBufferSpec& spec);

 private:
  int width_;
  int height_;
  GpuBufferFormat format_;
  CFHolder<CVPixelBufferPoolRef> pool_;
  int count_ = 0;
  CvTextureCacheManager* texture_caches_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_CV_PIXEL_BUFFER_POOL_WRAPPER_H_
