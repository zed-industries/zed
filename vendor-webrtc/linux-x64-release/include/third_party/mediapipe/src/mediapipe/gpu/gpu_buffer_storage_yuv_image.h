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

#include <memory>

#include "mediapipe/framework/formats/frame_buffer.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/formats/yuv_image.h"
#include "mediapipe/gpu/frame_buffer_view.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"
#include "mediapipe/gpu/image_frame_view.h"

#ifndef MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_YUV_IMAGE_H_
#define MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_YUV_IMAGE_H_

namespace mediapipe {

namespace internal {

template <>
class ViewProvider<YUVImage> {
 public:
  virtual ~ViewProvider() = default;
  virtual std::shared_ptr<const YUVImage> GetReadView(
      types<YUVImage>) const = 0;
  virtual std::shared_ptr<YUVImage> GetWriteView(types<YUVImage>) = 0;
};

}  // namespace internal

// TODO: add support for I444.
class GpuBufferStorageYuvImage
    : public internal::GpuBufferStorageImpl<
          GpuBufferStorageYuvImage, internal::ViewProvider<YUVImage>,
          internal::ViewProvider<FrameBuffer>,
          internal::ViewProvider<ImageFrame>> {
 public:
  // Constructor from an existing YUVImage with FOURCC_NV12, FOURCC_NV21,
  // FOURCC_YV12 or FOURCC_I420 format.
  explicit GpuBufferStorageYuvImage(std::shared_ptr<YUVImage> yuv_image);
  // Constructor. Supported formats are kNV12, kNV21, kYV12 and kI420.
  // Stride is set by default so that row boundaries align to 16 bytes.
  GpuBufferStorageYuvImage(int width, int height, GpuBufferFormat format);

  int width() const override { return yuv_image_->width(); }
  int height() const override { return yuv_image_->height(); }
  GpuBufferFormat format() const override;

  std::shared_ptr<const YUVImage> GetReadView(
      internal::types<YUVImage>) const override {
    return yuv_image_;
  }
  std::shared_ptr<YUVImage> GetWriteView(internal::types<YUVImage>) override {
    return yuv_image_;
  }

  std::shared_ptr<const FrameBuffer> GetReadView(
      internal::types<FrameBuffer>) const override;
  std::shared_ptr<FrameBuffer> GetWriteView(
      internal::types<FrameBuffer>) override;
  std::shared_ptr<const ImageFrame> GetReadView(
      internal::types<ImageFrame>) const override;
  std::shared_ptr<ImageFrame> GetWriteView(
      internal::types<ImageFrame>) override;

 private:
  std::shared_ptr<YUVImage> yuv_image_;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_BUFFER_STORAGE_YUV_IMAGE_H_
