/* Copyright 2023 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_GPU_FRAME_BUFFER_VIEW_H_
#define MEDIAPIPE_GPU_FRAME_BUFFER_VIEW_H_

#include "mediapipe/framework/formats/frame_buffer.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"

namespace mediapipe {
namespace internal {

template <>
class ViewProvider<FrameBuffer> {
 public:
  virtual ~ViewProvider() = default;
  virtual std::shared_ptr<const FrameBuffer> GetReadView(
      types<FrameBuffer>) const = 0;
  virtual std::shared_ptr<FrameBuffer> GetWriteView(types<FrameBuffer>) = 0;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_FRAME_BUFFER_VIEW_H_
