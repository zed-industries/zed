// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_H_

#include <cstdint>
#include <utility>

#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/formats/image_format.pb.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/gpu/gpu_buffer.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/gpu_buffer_storage_image_frame.h"
#include "mediapipe/gpu/image_frame_view.h"

#if !MEDIAPIPE_DISABLE_GPU

#if defined(__APPLE__)
#include <CoreVideo/CoreVideo.h>

#include "mediapipe/objc/CFHolder.h"
#include "mediapipe/objc/util.h"
#endif  // defined(__APPLE__)

#if !MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER  // OSX, use GL textures.
#include "mediapipe/gpu/gl_texture_buffer.h"
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER

#endif  // !MEDIAPIPE_DISABLE_GPU

namespace mediapipe {

using ImageFrameSharedPtr = std::shared_ptr<ImageFrame>;

// This class wraps ImageFrame(CPU) & GpuBuffer(GPU) data.
// An instance of Image acts as an opaque reference to the underlying
// data objects. Image also maintains backwards compatability with GpuBuffer.
//
// Accessing GPU storage requires a valid OpenGL context active beforehand.
// i.e.: GetGlTextureBufferSharedPtr() & ConvertToGpu() & GetGpuBuffer()
//       should be called inside an active GL context.
//
// Note: 'use_gpu_' flag is used to keep track of where data is (dirty bit).
//
// TODO Refactor Image to use 'Impl' class delegation system.
//
class Image {
 public:
  // Default constructor creates invalid object.
  Image() = default;

  // Copy and move constructors and assignment operators are supported.
  Image(const Image& other) = default;
  Image(Image&& other) = default;
  Image& operator=(const Image& other) = default;
  Image& operator=(Image&& other) = default;

  // Creates an Image representing the same image content as the ImageFrame
  // the input shared pointer points to, and retaining shared ownership.
  explicit Image(ImageFrameSharedPtr image_frame)
      : gpu_buffer_(std::make_shared<GpuBufferStorageImageFrame>(
            std::move(image_frame))) {
    use_gpu_ = false;
  }

  // CPU getters.
  ImageFrameSharedPtr GetImageFrameSharedPtr() const {
    // Write view currently because the return type does not point to const IF.
    return gpu_buffer_.GetWriteView<ImageFrame>();
  }

  // Creates an Image representing the same image content as the input GPU
  // buffer in platform-specific representations.
#if !MEDIAPIPE_DISABLE_GPU
#if MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
  explicit Image(CFHolder<CVPixelBufferRef> pixel_buffer)
      : Image(mediapipe::GpuBuffer(std::move(pixel_buffer))) {}
  explicit Image(CVPixelBufferRef pixel_buffer)
      : Image(mediapipe::GpuBuffer(pixel_buffer)) {}
#else
  explicit Image(mediapipe::GlTextureBufferSharedPtr texture_buffer)
      : Image(mediapipe::GpuBuffer(std::move(texture_buffer))) {}
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
  explicit Image(mediapipe::GpuBuffer gpu_buffer) {
    use_gpu_ = true;
    gpu_buffer_ = gpu_buffer;
  }

  // GPU getters.
#if MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
  CVPixelBufferRef GetCVPixelBufferRef() const {
    if (use_gpu_ == false) ConvertToGpu();
    return mediapipe::GetCVPixelBufferRef(gpu_buffer_);
  }
#else
  mediapipe::GlTextureBufferSharedPtr GetGlTextureBufferSharedPtr() const {
    if (use_gpu_ == false) ConvertToGpu();
    return gpu_buffer_.internal_storage<mediapipe::GlTextureBuffer>();
  }
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
#endif  // !MEDIAPIPE_DISABLE_GPU

  // Provides access to the underlying GpuBuffer storage.
  // Automatically uploads from CPU to GPU if needed and requested through the
  // `upload_to_gpu` argument.
  const mediapipe::GpuBuffer GetGpuBuffer(bool upload_to_gpu = true) const {
    if (!use_gpu_ && upload_to_gpu) ConvertToGpu();
    return gpu_buffer_;
  }

  // Returns image properties.
  int width() const;
  int height() const;
  int channels() const;
  int step() const;  // Row size in bytes.
  bool UsesGpu() const { return use_gpu_; }
  ImageFormat::Format image_format() const;
  mediapipe::GpuBufferFormat format() const;

  // Converts to true iff valid.
  explicit operator bool() const { return operator!=(nullptr); }

  bool operator==(const Image& other) const;
  bool operator!=(const Image& other) const { return !operator==(other); }

  // Allow comparison with nullptr.
  bool operator==(std::nullptr_t other) const;
  bool operator!=(std::nullptr_t other) const { return !operator==(other); }

  // Allow assignment from nullptr.
  Image& operator=(std::nullptr_t other);

  // Lock/Unlock pixel data.
  // Should be used exclusively by the PixelLock helper class.
  void LockPixels() const ABSL_EXCLUSIVE_LOCK_FUNCTION();
  void UnlockPixels() const ABSL_UNLOCK_FUNCTION();

  // Helper utility for GPU->CPU data transfer.
  bool ConvertToCpu() const;
  // Helper utility for CPU->GPU data transfer.
  // *Requires a valid OpenGL context to be active before calling!*
  bool ConvertToGpu() const;

 private:
  mutable mediapipe::GpuBuffer gpu_buffer_;
  mutable bool use_gpu_ = false;
};

inline int Image::width() const { return gpu_buffer_.width(); }

inline int Image::height() const { return gpu_buffer_.height(); }

inline ImageFormat::Format Image::image_format() const {
  return mediapipe::ImageFormatForGpuBufferFormat(gpu_buffer_.format());
}

inline mediapipe::GpuBufferFormat Image::format() const {
  return gpu_buffer_.format();
}

inline bool Image::operator==(std::nullptr_t other) const {
  return gpu_buffer_ == other;
}

inline bool Image::operator==(const Image& other) const {
  return gpu_buffer_ == other.gpu_buffer_;
}

inline Image& Image::operator=(std::nullptr_t other) {
  gpu_buffer_ = other;
  return *this;
}

inline int Image::channels() const {
  return ImageFrame::NumberOfChannelsForFormat(image_format());
}

inline int Image::step() const {
  return gpu_buffer_.GetReadView<ImageFrame>()->WidthStep();
}

inline void Image::LockPixels() const {
  ConvertToCpu();  // Download data if necessary.
}

inline void Image::UnlockPixels() const {}

// Helper class for getting access to Image CPU data,
// and handles automatically locking/unlocking CPU data access.
//
// Returns pointer to first pixel, or nullptr if invaild Image is provided
//
// Example use:
//   Image buf = ...
//   {
//     PixelLock lock(&buf);
//     uint8_t* buf_ptr = lock.Pixels();
//     ... use buf_ptr to access pixel data ...
//     ... lock released automatically at end of scope ...
//   }
//
// Note: should be used in separate minimal scope where possible; see example^.
//
class PixelReadLock {
 public:
  explicit PixelReadLock(const Image& image) {
    buffer_ = &image;
    if (buffer_) {
      buffer_->LockPixels();
      frame_ = buffer_->GetImageFrameSharedPtr();
    }
  }
  ~PixelReadLock() {
    if (buffer_) buffer_->UnlockPixels();
  }
  PixelReadLock(const PixelReadLock&) = delete;

  const uint8_t* Pixels() const {
    if (frame_) return frame_->PixelData();
    return nullptr;
  }

  PixelReadLock& operator=(const PixelReadLock&) = delete;

 private:
  const Image* buffer_ = nullptr;
  std::shared_ptr<ImageFrame> frame_;
};

class PixelWriteLock {
 public:
  explicit PixelWriteLock(Image* image) {
    buffer_ = image;
    if (buffer_) {
      buffer_->LockPixels();
      frame_ = buffer_->GetImageFrameSharedPtr();
    }
  }
  ~PixelWriteLock() {
    if (buffer_) buffer_->UnlockPixels();
  }
  PixelWriteLock(const PixelWriteLock&) = delete;

  uint8_t* Pixels() {
    if (frame_) return frame_->MutablePixelData();
    return nullptr;
  }

  PixelWriteLock& operator=(const PixelWriteLock&) = delete;

 private:
  const Image* buffer_ = nullptr;
  std::shared_ptr<ImageFrame> frame_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_H_
