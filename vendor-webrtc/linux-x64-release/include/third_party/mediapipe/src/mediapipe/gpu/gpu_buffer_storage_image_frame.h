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

#ifndef MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_IMAGE_FRAME_H_
#define MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_IMAGE_FRAME_H_

#include <memory>

#include "mediapipe/framework/formats/frame_buffer.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/gpu/frame_buffer_view.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"
#include "mediapipe/gpu/image_frame_view.h"

namespace mediapipe {

// Implements support for ImageFrame as a backing storage of GpuBuffer.
class GpuBufferStorageImageFrame
    : public internal::GpuBufferStorageImpl<
          GpuBufferStorageImageFrame, internal::ViewProvider<ImageFrame>,
          internal::ViewProvider<FrameBuffer>> {
 public:
  explicit GpuBufferStorageImageFrame(std::shared_ptr<ImageFrame> image_frame)
      : image_frame_(image_frame) {}
  GpuBufferStorageImageFrame(int width, int height, GpuBufferFormat format) {
    image_frame_ = std::make_shared<ImageFrame>(
        ImageFormatForGpuBufferFormat(format), width, height);
  }
  int width() const override { return image_frame_->Width(); }
  int height() const override { return image_frame_->Height(); }
  GpuBufferFormat format() const override {
    return GpuBufferFormatForImageFormat(image_frame_->Format());
  }
  std::shared_ptr<const ImageFrame> image_frame() const { return image_frame_; }
  std::shared_ptr<ImageFrame> image_frame() { return image_frame_; }
  std::shared_ptr<const ImageFrame> GetReadView(
      internal::types<ImageFrame>) const override {
    return image_frame_;
  }
  std::shared_ptr<ImageFrame> GetWriteView(
      internal::types<ImageFrame>) override {
    return image_frame_;
  }
  std::shared_ptr<const FrameBuffer> GetReadView(
      internal::types<FrameBuffer>) const override;
  std::shared_ptr<FrameBuffer> GetWriteView(
      internal::types<FrameBuffer>) override;

 private:
  std::shared_ptr<ImageFrame> image_frame_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_IMAGE_FRAME_H_
