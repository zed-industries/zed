#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_BUFFER_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_BUFFER_H_

#include <webgpu/webgpu_cpp.h>

#include <cstdint>
#include <memory>
#include <utility>

#include "absl/status/statusor.h"
#include "mediapipe/gpu/gpu_buffer.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"
#include "mediapipe/gpu/multi_pool.h"
#include "mediapipe/gpu/reusable_pool.h"
#include "mediapipe/gpu/webgpu/webgpu_service.h"
#include "mediapipe/gpu/webgpu/webgpu_texture_view.h"

namespace mediapipe {

class WebGpuTextureBuffer
    : public internal::GpuBufferStorageImpl<
          WebGpuTextureBuffer, internal::ViewProvider<WebGpuTextureView>> {
 public:
  static std::unique_ptr<WebGpuTextureBuffer> Create(
      const wgpu::Device& device, uint32_t width, uint32_t height,
      GpuBufferFormat format = GpuBufferFormat::kRGBA32);

  static std::unique_ptr<WebGpuTextureBuffer> Create(uint32_t width,
                                                     uint32_t height,
                                                     GpuBufferFormat format);

  WebGpuTextureBuffer(wgpu::Texture texture, uint32_t width, uint32_t height,
                      GpuBufferFormat format)
      : texture_(std::move(texture)),
        width_(width),
        height_(height),
        format_(format) {}

  ~WebGpuTextureBuffer() override { texture_.Destroy(); }

  int width() const override { return width_; }
  int height() const override { return height_; }

  WebGpuTextureView GetReadView(
      internal::types<WebGpuTextureView>) const override;

  WebGpuTextureView GetWriteView(internal::types<WebGpuTextureView>) override;

  GpuBufferFormat format() const override { return format_; }

  // TODO: implement.
  void Reuse() {}

 private:
  wgpu::Texture texture_;
  const uint32_t width_ = 0;
  const uint32_t height_ = 0;
  const GpuBufferFormat format_;
};

class Canvas;
Canvas& RenderToWebGpuCanvas(const std::shared_ptr<WebGpuTextureBuffer>& input,
                             int canvas_width, int canvas_height);

class WebGpuTextureBufferPool : public ReusablePool<WebGpuTextureBuffer> {
 public:
  static std::shared_ptr<WebGpuTextureBufferPool> Create(
      const wgpu::Device& device, const internal::GpuBufferSpec& spec,
      const MultiPoolOptions& options) {
    return std::make_shared<WebGpuTextureBufferPool>(device, spec, options);
  }

  static absl::StatusOr<std::shared_ptr<WebGpuTextureBuffer>>
  CreateBufferWithoutPool(const internal::GpuBufferSpec& spec);

  WebGpuTextureBufferPool(const wgpu::Device& device,
                          const internal::GpuBufferSpec& spec,
                          const MultiPoolOptions& options)
      : ReusablePool<WebGpuTextureBuffer>(
            [this] {
              return WebGpuTextureBuffer::Create(device_, spec_.width,
                                                 spec_.height, spec_.format);
            },
            options),
        device_(device),
        spec_(spec) {}

 private:
  const wgpu::Device device_;
  const internal::GpuBufferSpec& spec_;
};

class WebGpuTextureBufferMultiPool
    : public MultiPool<WebGpuTextureBufferPool, internal::GpuBufferSpec,
                       std::shared_ptr<WebGpuTextureBuffer>> {
 public:
  explicit WebGpuTextureBufferMultiPool(
      const wgpu::Device& device,
      const MultiPoolOptions& options = kDefaultMultiPoolOptions)
      : MultiPool(
            [this](const internal::GpuBufferSpec& spec,
                   const MultiPoolOptions& options) {
              return WebGpuTextureBufferPool::Create(device_, spec, options);
            },
            options),
        device_(device) {}

  // TODO: This and RenderToWebGpuCanvas use kBGRA32 as defaults, while
  // the rest of our code defaults to kRGBA32 instead. Investigate whether this
  // is intentional or typos leading to accidental over-sharding of pools.
  absl::StatusOr<std::shared_ptr<WebGpuTextureBuffer>> GetBuffer(
      int width, int height,
      GpuBufferFormat format = GpuBufferFormat::kBGRA32) {
    return Get(internal::GpuBufferSpec(width, height, format));
  }

 private:
  const wgpu::Device device_;
};

inline const WebGpuDeviceAttachment<WebGpuTextureBufferMultiPool>
    kWebGpuTexturePool{[](wgpu::Device& device) {
      return WebGpuDeviceAttachment<WebGpuTextureBufferMultiPool>::MakePtr(
          device);
    }};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_TEXTURE_BUFFER_H_
