// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_FRAME_BUFFER_UTIL_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_FRAME_BUFFER_UTIL_H_

#include <cstdint>
#include <memory>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/frame_buffer.h"
#include "mediapipe/framework/formats/tensor.h"

namespace mediapipe {
namespace frame_buffer {

// Creation helpers.
//------------------------------------------------------------------------------

// Default stride value for creating frame buffer from raw buffer. When using
// this default value, the default row stride and pixel stride values will be
// applied. e.g. for an RGB image:
//   row_stride = width * 3
//   pixel_stride = 3.
inline constexpr FrameBuffer::Stride kDefaultStride = {0, 0};

// Creates a FrameBuffer from raw RGBA buffer and passing arguments.
std::shared_ptr<FrameBuffer> CreateFromRgbaRawBuffer(
    uint8_t* input, FrameBuffer::Dimension dimension,
    FrameBuffer::Stride stride = kDefaultStride);

// Creates a FrameBuffer from raw RGB buffer and passing arguments.
std::shared_ptr<FrameBuffer> CreateFromRgbRawBuffer(
    uint8_t* input, FrameBuffer::Dimension dimension,
    FrameBuffer::Stride stride = kDefaultStride);

// Creates a FrameBuffer from raw grayscale buffer and passing arguments.
std::shared_ptr<FrameBuffer> CreateFromGrayRawBuffer(
    uint8_t* input, FrameBuffer::Dimension dimension,
    FrameBuffer::Stride stride = kDefaultStride);

// Creates a FrameBuffer from raw YUV buffer and passing arguments.
absl::StatusOr<std::shared_ptr<FrameBuffer>> CreateFromYuvRawBuffer(
    uint8_t* y_plane, uint8_t* u_plane, uint8_t* v_plane,
    FrameBuffer::Format format, FrameBuffer::Dimension dimension,
    int row_stride_y, int row_stride_uv, int pixel_stride_uv);

// Creates an instance of FrameBuffer from raw buffer and passing arguments.
absl::StatusOr<std::shared_ptr<FrameBuffer>> CreateFromRawBuffer(
    uint8_t* buffer, FrameBuffer::Dimension dimension,
    FrameBuffer::Format target_format);

// Transformations.
//------------------------------------------------------------------------------

// Crops `buffer` to the specified points.
//
// (x0, y0) represents the top-left point of the buffer.
// (x1, y1) represents the bottom-right point of the buffer.
//
// The implementation performs origin moving and resizing operations.
absl::Status Crop(const FrameBuffer& buffer, int x0, int y0, int x1, int y1,
                  FrameBuffer* output_buffer);

// Resizes `buffer` to the size of the given `output_buffer` using bilinear
// interpolation.
absl::Status Resize(const FrameBuffer& buffer, FrameBuffer* output_buffer);

// Rotates `buffer` counter-clockwise by the given `angle_deg` (in degrees).
//
// The given angle must be a multiple of 90 degrees.
absl::Status Rotate(const FrameBuffer& buffer, int angle_deg,
                    FrameBuffer* output_buffer);

// Flips `buffer` horizontally.
absl::Status FlipHorizontally(const FrameBuffer& buffer,
                              FrameBuffer* output_buffer);

// Flips `buffer` vertically.
absl::Status FlipVertically(const FrameBuffer& buffer,
                            FrameBuffer* output_buffer);

// Converts `buffer`'s format to the format of the given `output_buffer`.
//
// Note that grayscale format does not convert to other formats.
// Note the NV21 to RGB/RGBA conversion may downsample by factor of 2 based
// on the buffer and output_buffer dimensions.
absl::Status Convert(const FrameBuffer& buffer, FrameBuffer* output_buffer);

// Converts `buffer` into the provided float Tensor. Each value is converted to
// a float using:
//   output = input * scale + offset
//
// Note that only interleaved single-planar formats support this operation.
absl::Status ToFloatTensor(const FrameBuffer& buffer, float scale, float offset,
                           Tensor& tensor);

// Miscellaneous Methods
// -----------------------------------------------------------------

// Returns the frame buffer size in bytes based on the input format and
// dimensions. GRAY, YV12/YV21 are in the planar formats, NV12/NV21 are in the
// semi-planar formats with the interleaved UV planes. RGB/RGBA are in the
// interleaved format.
int GetFrameBufferByteSize(FrameBuffer::Dimension dimension,
                           FrameBuffer::Format format);

// Returns pixel stride info for kGRAY, kRGB, kRGBA formats.
absl::StatusOr<int> GetPixelStrides(FrameBuffer::Format format);

// Returns the biplanar UV raw buffer for NV12/NV21 frame buffer.
absl::StatusOr<const uint8_t*> GetUvRawBuffer(const FrameBuffer& buffer);

// Returns U or V plane dimension with the given buffer `dimension` and
// `format`. Only supports NV12/NV21/YV12/YV21 formats. Returns
// InvalidArgumentError if 'dimension' is invalid or 'format' is other than the
// supported formats. This method assums the UV plane share the same dimension,
// especially for the YV12 / YV21 formats.
absl::StatusOr<FrameBuffer::Dimension> GetUvPlaneDimension(
    FrameBuffer::Dimension dimension, FrameBuffer::Format format);

// Returns crop dimension based on crop start and end points.
FrameBuffer::Dimension GetCropDimension(int x0, int x1, int y0, int y1);

}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_FRAME_BUFFER_UTIL_H_
