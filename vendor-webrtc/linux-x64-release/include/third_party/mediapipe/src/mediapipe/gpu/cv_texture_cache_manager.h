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

#ifndef MEDIAPIPE_GPU_CV_TEXTURE_CACHE_MANAGER_H_
#define MEDIAPIPE_GPU_CV_TEXTURE_CACHE_MANAGER_H_

#include <vector>

#include "absl/synchronization/mutex.h"
#include "mediapipe/gpu/pixel_buffer_pool_util.h"
#include "mediapipe/objc/CFHolder.h"

namespace mediapipe {

class CvTextureCacheManager {
 public:
  ~CvTextureCacheManager();

  // TODO: add tests for the texture cache registration.

  // Inform the pool of a cache that should be flushed when it is low on
  // reusable buffers.
  void RegisterTextureCache(CVTextureCacheType cache);

  // Remove a texture cache from the list of caches to be flushed.
  void UnregisterTextureCache(CVTextureCacheType cache);

  void FlushTextureCaches();

 private:
  absl::Mutex mutex_;
  std::vector<CFHolder<CVTextureCacheType>> texture_caches_
      ABSL_GUARDED_BY(mutex_);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_CV_TEXTURE_CACHE_MANAGER_H_
