#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_UTILS_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_UTILS_H_

#include <webgpu/webgpu_cpp.h>

#include <cstdint>

#include "absl/status/statusor.h"

namespace mediapipe {

absl::StatusOr<uint32_t> WebGpuTextureFormatBytesPerPixel(
    wgpu::TextureFormat format);
absl::StatusOr<uint32_t> WebGpuTextureFormatDepth(wgpu::TextureFormat format);

wgpu::Texture CreateTextureWebGpuTexture2d(const wgpu::Device& device,
                                           uint32_t width, uint32_t height,
                                           wgpu::TextureFormat format,
                                           wgpu::TextureUsage usage);

absl::Status WebGpuTexture2dUploadData(const wgpu::Device& device,
                                       uint32_t width, uint32_t height,
                                       wgpu::TextureFormat format,
                                       const wgpu::Queue& queue,
                                       uint32_t bytes_per_pixel, void* data,
                                       const wgpu::Texture& texture);

absl::StatusOr<wgpu::Texture> CreateWebGpuTexture2dAndUploadData(
    const wgpu::Device& device, uint32_t width, uint32_t height,
    wgpu::TextureFormat format, wgpu::TextureUsage usage,
    const wgpu::Queue& queue, uint32_t bytes_per_pixel, void* data);

#ifdef __EMSCRIPTEN__
absl::Status GetTexture2dData(const wgpu::Device& device,
                              const wgpu::Queue& queue,
                              const wgpu::Texture& texture, uint32_t width,
                              uint32_t height, uint32_t bytes_per_row,
                              uint8_t* dst);
#endif  // __EMSCRIPTEN__

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_UTILS_H_
