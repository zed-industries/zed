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

#ifndef MEDIAPIPE_GPU_GPU_SERVICE_H_
#define MEDIAPIPE_GPU_GPU_SERVICE_H_

#include "absl/base/attributes.h"
#include "mediapipe/framework/graph_service.h"

#if !MEDIAPIPE_DISABLE_GPU
#include "mediapipe/gpu/gpu_shared_data_internal.h"
#endif  // !MEDIAPIPE_DISABLE_GPU

namespace mediapipe {

#if MEDIAPIPE_DISABLE_GPU
class GpuResources {
  GpuResources() = delete;
};
#endif  // MEDIAPIPE_DISABLE_GPU

ABSL_CONST_INIT extern const GraphService<GpuResources> kGpuService;

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_SERVICE_H_
