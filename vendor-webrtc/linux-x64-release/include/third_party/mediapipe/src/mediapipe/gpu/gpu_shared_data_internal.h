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

// Declares GpuSharedData, a private object that is used to store
// platform-specific resources shared by GPU calculators across a graph.

// Consider this file an implementation detail. None of this is part of the
// public API.

#ifndef MEDIAPIPE_GPU_GPU_SHARED_DATA_INTERNAL_H_
#define MEDIAPIPE_GPU_GPU_SHARED_DATA_INTERNAL_H_

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_node.h"
#include "mediapipe/framework/executor.h"
#include "mediapipe/gpu/gl_context.h"
#include "mediapipe/gpu/gpu_buffer_multi_pool.h"
#include "mediapipe/gpu/multi_pool.h"

#ifdef __APPLE__
#include "mediapipe/gpu/cv_texture_cache_manager.h"
#endif  // defined(__APPLE__)

namespace mediapipe {

#if MEDIAPIPE_METAL_ENABLED
class MetalSharedResources;
#endif  // MEDIAPIPE_METAL_ENABLED

// TODO: rename to GpuService or GpuManager or something.
class GpuResources {
 public:
  using StatusOrGpuResources = absl::StatusOr<std::shared_ptr<GpuResources>>;

  static StatusOrGpuResources Create();
  // Optional gpu_buffer_pool_options argument allows to configure the
  // GpuBufferMultiPool instance.
  static StatusOrGpuResources Create(
      PlatformGlContext external_context,
      const MultiPoolOptions* gpu_buffer_pool_options = nullptr);

  // The destructor must be defined in the implementation file so that on iOS
  // the correct ARC release calls are generated.
  ~GpuResources();

  explicit GpuResources(PlatformGlContext external_context);

  // Shared GL context for calculators.
  // TODO: require passing a context or node identifier.
  const std::shared_ptr<GlContext>& gl_context() { return gl_context(nullptr); }

  const std::shared_ptr<GlContext>& gl_context(CalculatorContext* cc);

  // Shared buffer pool.
  GpuBufferMultiPool& gpu_buffer_pool() { return gpu_buffer_pool_; }

#if MEDIAPIPE_METAL_ENABLED
  MetalSharedResources& metal_shared() { return *metal_shared_; }
#endif  // MEDIAPIPE_METAL_ENABLED

  absl::Status PrepareGpuNode(CalculatorNode* node);

  absl::StatusOr<std::shared_ptr<Executor>> GetDefaultGpuExecutor() const;

  // If the node requires custom GPU executors in the current configuration,
  // returns the executor's names and the executors themselves.
  const std::map<std::string, std::shared_ptr<Executor>>& GetGpuExecutors() {
    return named_executors_;
  }

 private:
  GpuResources() = delete;
  explicit GpuResources(std::shared_ptr<GlContext> gl_context,
                        const MultiPoolOptions* gpu_buffer_pool_options);

  GlContext::StatusOrGlContext GetOrCreateGlContext(const std::string& key);
  const std::string& ContextKey(const std::string& canonical_node_name);

  std::map<std::string, std::string> node_key_;

  using GlContextMapType = std::map<std::string, std::shared_ptr<GlContext>>;
  std::unique_ptr<GlContextMapType, void (*)(GlContextMapType*)>
      gl_key_context_;

#ifdef MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
  std::shared_ptr<CvTextureCacheManager> texture_caches_;
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER

  // The pool must be destructed before the gl_context, but after the
  // ios_gpu_data, so the declaration order is important.
  GpuBufferMultiPool gpu_buffer_pool_;

#if MEDIAPIPE_METAL_ENABLED
  std::unique_ptr<MetalSharedResources> metal_shared_;
#endif  // MEDIAPIPE_METAL_ENABLED

  std::map<std::string, std::shared_ptr<Executor>> named_executors_;
};

// Legacy struct to keep existing client code happy.
// TODO: eliminate!
struct GpuSharedData {
  GpuSharedData();

  explicit GpuSharedData(PlatformGlContext external_context)
      : GpuSharedData(CreateGpuResourcesOrDie(external_context)) {}

  explicit GpuSharedData(std::shared_ptr<GpuResources> gpu_resources)
      : gpu_resources(gpu_resources),
        gl_context(gpu_resources->gl_context()),
        gpu_buffer_pool(gpu_resources->gpu_buffer_pool()) {}

  std::shared_ptr<GpuResources> gpu_resources;

  std::shared_ptr<GlContext> gl_context;

  GpuBufferMultiPool& gpu_buffer_pool;

 private:
  static std::shared_ptr<GpuResources> CreateGpuResourcesOrDie(
      PlatformGlContext external_context) {
    auto status_or_resources = GpuResources::Create(external_context);
    MEDIAPIPE_CHECK_OK(status_or_resources.status())
        << ": could not create GpuResources";
    return std::move(status_or_resources).value();
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_SHARED_DATA_INTERNAL_H_
