#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_VIEW_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_VIEW_H_

#include <webgpu/webgpu_cpp.h>

#include <cstdint>
#include <memory>
#include <utility>

#include "mediapipe/gpu/gpu_buffer_storage.h"

namespace mediapipe {

class WebGpuTextureView {
 public:
  WebGpuTextureView() = delete;

  int width() const { return width_; }
  int height() const { return height_; }
  int depth() const { return depth_; }
  const wgpu::Texture& texture() const { return texture_; }

 private:
  friend class WebGpuTextureBuffer;
  friend class WebGpuTextureBuffer3d;

  WebGpuTextureView(const wgpu::Texture& texture, uint32_t width,
                    uint32_t height, uint32_t depth = 1)
      : texture_(texture), width_(width), height_(height), depth_(depth) {}

  const wgpu::Texture& texture_;
  const uint32_t width_;
  const uint32_t height_;
  const uint32_t depth_;  // 1 for normal 2d textures
};

namespace internal {

template <>
class ViewProvider<WebGpuTextureView> {
 public:
  virtual ~ViewProvider() = default;
  virtual WebGpuTextureView GetReadView(types<WebGpuTextureView>) const = 0;
  virtual WebGpuTextureView GetWriteView(types<WebGpuTextureView>) = 0;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_VIEW_H_
