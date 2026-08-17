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

#ifndef MEDIAPIPE_GPU_GPU_BUFFER_FORMAT_H_
#define MEDIAPIPE_GPU_GPU_BUFFER_FORMAT_H_

#include <cstdint>

#ifdef __APPLE__
#include <CoreVideo/CoreVideo.h>
#if !TARGET_OS_OSX
#define MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER 1
#endif  // TARGET_OS_OSX
#endif  // defined(__APPLE__)

#include "mediapipe/framework/formats/image_format.pb.h"
#if !MEDIAPIPE_DISABLE_GPU
#include "mediapipe/gpu/gl_base.h"
#endif  // !MEDIAPIPE_DISABLE_GPU

// The behavior of multi-char constants is implementation-defined, so out of an
// excess of caution we define them in this portable way.
#define MEDIAPIPE_FOURCC(a, b, c, d) \
  (((a) << 24) + ((b) << 16) + ((c) << 8) + (d))

namespace mediapipe {

using mediapipe::ImageFormat;

enum class GpuBufferFormat : uint32_t {
  kUnknown = 0,
  kBGRA32 = MEDIAPIPE_FOURCC('B', 'G', 'R', 'A'),
  kRGBA32 = MEDIAPIPE_FOURCC('R', 'G', 'B', 'A'),
  kGrayFloat32 = MEDIAPIPE_FOURCC('L', '0', '0', 'f'),
  kGrayHalf16 = MEDIAPIPE_FOURCC('L', '0', '0', 'h'),
  kOneComponent8 = MEDIAPIPE_FOURCC('L', '0', '0', '8'),
  kOneComponent8Alpha = MEDIAPIPE_FOURCC('A', '0', '0', '8'),
  kOneComponent8Red = MEDIAPIPE_FOURCC('R', '0', '0', '8'),
  kTwoComponent8 = MEDIAPIPE_FOURCC('2', 'C', '0', '8'),
  kTwoComponentHalf16 = MEDIAPIPE_FOURCC('2', 'C', '0', 'h'),
  kTwoComponentFloat32 = MEDIAPIPE_FOURCC('2', 'C', '0', 'f'),
  kBiPlanar420YpCbCr8VideoRange = MEDIAPIPE_FOURCC('4', '2', '0', 'v'),
  kBiPlanar420YpCbCr8FullRange = MEDIAPIPE_FOURCC('4', '2', '0', 'f'),
  kRGB24 = 0x00000018,  // Note: prefer BGRA32 whenever possible.
  kRGBAHalf64 = MEDIAPIPE_FOURCC('R', 'G', 'h', 'A'),
  kRGBAFloat128 = MEDIAPIPE_FOURCC('R', 'G', 'f', 'A'),
  // Immutable version of kRGBA32
  kImmutableRGBA32 = MEDIAPIPE_FOURCC('4', 'C', 'I', '8'),
  // Immutable version of kRGBAFloat128
  kImmutableRGBAFloat128 = MEDIAPIPE_FOURCC('4', 'C', 'I', 'f'),
  // 8-bit Y plane + interleaved 8-bit U/V plane with 2x2 subsampling.
  kNV12 = MEDIAPIPE_FOURCC('N', 'V', '1', '2'),
  // 8-bit Y plane + interleaved 8-bit V/U plane with 2x2 subsampling.
  kNV21 = MEDIAPIPE_FOURCC('N', 'V', '2', '1'),
  // 8-bit Y plane + non-interleaved 8-bit U/V planes with 2x2 subsampling.
  kI420 = MEDIAPIPE_FOURCC('I', '4', '2', '0'),
  // 8-bit Y plane + non-interleaved 8-bit V/U planes with 2x2 subsampling.
  kYV12 = MEDIAPIPE_FOURCC('Y', 'V', '1', '2'),
};

#if !MEDIAPIPE_DISABLE_GPU
// TODO: make this more generally applicable.
enum class GlVersion {
  kGL = 1,
  kGLES2 = 2,
  kGLES3 = 3,
};

struct GlTextureInfo {
  GLint gl_internal_format;
  GLenum gl_format;
  GLenum gl_type;
  // For multiplane buffers, this represents how many times smaller than
  // the nominal image size a plane is.
  int downscale;
  // For GLES3.1+ compute shaders, users may explicitly request immutable
  // textures.
  bool immutable = false;
};

const GlTextureInfo& GlTextureInfoForGpuBufferFormat(GpuBufferFormat format,
                                                     int plane,
                                                     GlVersion gl_version);
#endif  // !MEDIAPIPE_DISABLE_GPU

ImageFormat::Format ImageFormatForGpuBufferFormat(GpuBufferFormat format);
GpuBufferFormat GpuBufferFormatForImageFormat(ImageFormat::Format format);

#ifdef __APPLE__

inline OSType CVPixelFormatForGpuBufferFormat(GpuBufferFormat format) {
  switch (format) {
    case GpuBufferFormat::kBGRA32:
      return kCVPixelFormatType_32BGRA;
    case GpuBufferFormat::kRGBA32:
      return kCVPixelFormatType_32RGBA;
    case GpuBufferFormat::kGrayHalf16:
      return kCVPixelFormatType_OneComponent16Half;
    case GpuBufferFormat::kGrayFloat32:
      return kCVPixelFormatType_OneComponent32Float;
    case GpuBufferFormat::kOneComponent8:
      return kCVPixelFormatType_OneComponent8;
    case GpuBufferFormat::kOneComponent8Alpha:
    case GpuBufferFormat::kOneComponent8Red:
      return -1;
    case GpuBufferFormat::kTwoComponent8:
      return kCVPixelFormatType_TwoComponent8;
    case GpuBufferFormat::kTwoComponentHalf16:
      return kCVPixelFormatType_TwoComponent16Half;
    case GpuBufferFormat::kTwoComponentFloat32:
      return kCVPixelFormatType_TwoComponent32Float;
    case GpuBufferFormat::kBiPlanar420YpCbCr8VideoRange:
      return kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange;
    case GpuBufferFormat::kBiPlanar420YpCbCr8FullRange:
      return kCVPixelFormatType_420YpCbCr8BiPlanarFullRange;
    case GpuBufferFormat::kRGB24:
      return kCVPixelFormatType_24RGB;
    case GpuBufferFormat::kRGBAHalf64:
      return kCVPixelFormatType_64RGBAHalf;
    case GpuBufferFormat::kRGBAFloat128:
      return kCVPixelFormatType_128RGBAFloat;
    case GpuBufferFormat::kImmutableRGBA32:
    case GpuBufferFormat::kImmutableRGBAFloat128:
    case GpuBufferFormat::kNV12:
    case GpuBufferFormat::kNV21:
    case GpuBufferFormat::kI420:
    case GpuBufferFormat::kYV12:
    case GpuBufferFormat::kUnknown:
      return -1;
  }
  return -1;
}

inline GpuBufferFormat GpuBufferFormatForCVPixelFormat(OSType format) {
  switch (format) {
    case kCVPixelFormatType_32BGRA:
      return GpuBufferFormat::kBGRA32;
    case kCVPixelFormatType_32RGBA:
      return GpuBufferFormat::kRGBA32;
    case kCVPixelFormatType_DepthFloat32:
      return GpuBufferFormat::kGrayFloat32;
    case kCVPixelFormatType_OneComponent16Half:
      return GpuBufferFormat::kGrayHalf16;
    case kCVPixelFormatType_OneComponent32Float:
      return GpuBufferFormat::kGrayFloat32;
    case kCVPixelFormatType_OneComponent8:
      return GpuBufferFormat::kOneComponent8;
    case kCVPixelFormatType_TwoComponent8:
      return GpuBufferFormat::kTwoComponent8;
    case kCVPixelFormatType_TwoComponent16Half:
      return GpuBufferFormat::kTwoComponentHalf16;
    case kCVPixelFormatType_TwoComponent32Float:
      return GpuBufferFormat::kTwoComponentFloat32;
    case kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange:
      return GpuBufferFormat::kBiPlanar420YpCbCr8VideoRange;
    case kCVPixelFormatType_420YpCbCr8BiPlanarFullRange:
      return GpuBufferFormat::kBiPlanar420YpCbCr8FullRange;
    case kCVPixelFormatType_24RGB:
      return GpuBufferFormat::kRGB24;
    case kCVPixelFormatType_64RGBAHalf:
      return GpuBufferFormat::kRGBAHalf64;
    case kCVPixelFormatType_128RGBAFloat:
      return GpuBufferFormat::kRGBAFloat128;
  }
  return GpuBufferFormat::kUnknown;
}

#endif  // __APPLE__

namespace internal {

struct GpuBufferSpec {
  GpuBufferSpec(int w, int h, GpuBufferFormat f)
      : width(w), height(h), format(f) {}

  template <typename H>
  friend H AbslHashValue(H h, const GpuBufferSpec& spec) {
    return H::combine(std::move(h), spec.width, spec.height,
                      static_cast<uint32_t>(spec.format));
  }

  int width;
  int height;
  GpuBufferFormat format;
};

// BufferSpec equality operators
inline bool operator==(const GpuBufferSpec& lhs, const GpuBufferSpec& rhs) {
  return lhs.width == rhs.width && lhs.height == rhs.height &&
         lhs.format == rhs.format;
}
inline bool operator!=(const GpuBufferSpec& lhs, const GpuBufferSpec& rhs) {
  return !operator==(lhs, rhs);
}

}  // namespace internal

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GPU_BUFFER_FORMAT_H_
