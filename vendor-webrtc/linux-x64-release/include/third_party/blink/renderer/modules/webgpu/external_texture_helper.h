// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_EXTERNAL_TEXTURE_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_EXTERNAL_TEXTURE_HELPER_H_

#include "base/memory/raw_ptr.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/modules/webgpu/dawn_object.h"
#include "third_party/blink/renderer/platform/graphics/predefined_color_space.h"

namespace gfx {
class ColorSpace;
}  // namespace gfx

namespace media {
class PaintCanvasVideoRenderer;
}  // namespace media

namespace blink {

class ExceptionState;
class HTMLVideoElement;
class VideoFrame;
class WebGPUMailboxTexture;

struct ColorSpaceConversionConstants {
  std::array<float, 9> gamut_conversion_matrix;
  std::array<float, 7> src_transfer_constants;
  std::array<float, 7> dst_transfer_constants;
};

struct ExternalTextureSource {
  scoped_refptr<media::VideoFrame> media_video_frame = nullptr;
  raw_ptr<media::PaintCanvasVideoRenderer> video_renderer = nullptr;
  std::optional<media::VideoFrame::ID> media_video_frame_unique_id =
      std::nullopt;
  bool valid = false;
};

struct ExternalTexture {
  wgpu::ExternalTexture wgpu_external_texture = nullptr;
  scoped_refptr<WebGPUMailboxTexture> mailbox_texture = nullptr;
  bool is_zero_copy = false;
};

std::array<float, 12> GetYUVToRGBMatrix(gfx::ColorSpace color_space,
                                        size_t bit_depth);

ColorSpaceConversionConstants GetColorSpaceConversionConstants(
    gfx::ColorSpace src_color_space,
    gfx::ColorSpace dst_color_space);

bool IsSameGamutAndGamma(gfx::ColorSpace src_color_space,
                         gfx::ColorSpace dst_color_space);

ExternalTextureSource GetExternalTextureSourceFromVideoElement(
    HTMLVideoElement* video,
    ExceptionState& exception_state);

ExternalTextureSource GetExternalTextureSourceFromVideoFrame(
    VideoFrame* frame,
    ExceptionState& exception_state);

ExternalTexture CreateExternalTexture(
    GPUDevice* device,
    PredefinedColorSpace dst_predefined_color_space,
    scoped_refptr<media::VideoFrame> media_video_frame,
    media::PaintCanvasVideoRenderer* video_renderer);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGPU_EXTERNAL_TEXTURE_HELPER_H_
