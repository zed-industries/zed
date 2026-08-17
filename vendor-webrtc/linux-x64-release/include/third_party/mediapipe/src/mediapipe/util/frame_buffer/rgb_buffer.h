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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_RGB_BUFFER_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_RGB_BUFFER_H_

#include <memory>

#include "HalideBuffer.h"
#include "HalideRuntime.h"
#include "mediapipe/util/frame_buffer/float_buffer.h"
#include "mediapipe/util/frame_buffer/gray_buffer.h"
#include "mediapipe/util/frame_buffer/yuv_buffer.h"

namespace mediapipe {
namespace frame_buffer {

// RgbBuffer represents a view over an interleaved RGB/RGBA image.
//
// RgbBuffers may be copied and moved efficiently; their backing buffers are
// shared and never deep copied.
//
// RgbBuffer requires a minimum image width depending on the natural vector
// size of the platform, e.g., 16px. This is not validated by RgbBuffer.
class RgbBuffer {
 public:
  // Returns the size (in bytes) of an RGB/RGBA image of the given dimensions
  // without padding.
  static int ByteSize(int width, int height, bool alpha) {
    return width * height * (alpha ? 4 : 3);
  }

  // Builds a RgbBuffer using the given backing buffer and dimensions.
  //
  // Does not take ownership of the backing buffer (provided in 'data').
  RgbBuffer(uint8_t* data, int width, int height, bool alpha);

  // Builds a RgbBuffer using the given backing buffer and dimensions.
  // 'row_stride' must be greater than or equal to 'width'. Padding bytes are at
  // the end of each row, following the image bytes.
  //
  // Does not take ownership of the backing buffer (provided in 'data').
  RgbBuffer(uint8_t* data, int width, int height, int row_stride, bool alpha);

  // Builds a RgbBuffer using the given dimensions.
  //
  // The underlying backing buffer is allocated and owned by this RgbBuffer.
  RgbBuffer(int width, int height, bool alpha);

  // RgbBuffer is copyable. The source retains ownership of its backing buffer.
  RgbBuffer(const RgbBuffer& other);
  // RgbBuffer is moveable. The source loses ownership of any backing buffers.
  RgbBuffer(RgbBuffer&& other);
  // RgbBuffer is assignable.
  RgbBuffer& operator=(const RgbBuffer& other);
  RgbBuffer& operator=(RgbBuffer&& other);

  ~RgbBuffer();

  // Performs an in-place crop. Modifies this buffer so that the new extent
  // matches that of the given crop rectangle -- (x0, y0) becomes (0, 0) and
  // the new width and height are x1 - x0 + 1 and y1 - y0 + 1, respectively.
  bool Crop(int x0, int y0, int x1, int y1);

  // Resize this image to match the dimensions of the given output RgbBuffer
  // and places the result into its backing buffer.
  //
  // Performs a resize with bilinear interpolation (over four source pixels).
  // Resizing with an RGB source buffer and RGBA destination is currently
  // unsupported.
  bool Resize(RgbBuffer* output);

  // Rotate this image into the given buffer by the given angle (90, 180, 270).
  //
  // Rotation is specified in degrees counter-clockwise such that when rotating
  // by 90 degrees, the top-right corner of the source becomes the top-left of
  // the output. The output buffer must have its height and width swapped when
  // rotating by 90 or 270.
  //
  // Any angle values other than (90, 180, 270) are invalid.
  bool Rotate(int angle, RgbBuffer* output);

  // Flip this image horizontally/vertically into the given buffer. Both buffer
  // dimensions and formats must match (this method does not convert RGB-to-RGBA
  // nor RGBA-to-RGB).
  bool FlipHorizontally(RgbBuffer* output);
  bool FlipVertically(RgbBuffer* output);

  // Performs a RGB-to-YUV color format conversion and places the result
  // in the given output YuvBuffer. Both buffer dimensions must match.
  bool Convert(YuvBuffer* output);

  // Performs a RGB to grayscale format conversion.
  bool Convert(GrayBuffer* output);

  // Performs a rgb to rgba / rgba to rgb format conversion.
  bool Convert(RgbBuffer* output);

  // Performs a RGB to float conversion.
  bool ToFloat(float scale, float offset, FloatBuffer* output);

  // Release ownership of the owned backing buffer.
  uint8_t* Release() { return owned_buffer_.release(); }

  // Returns the halide_buffer_t* for the image.
  const halide_buffer_t* buffer() const { return buffer_.raw_buffer(); }
  // Returns the halide_buffer_t* for the image.
  halide_buffer_t* buffer() { return buffer_.raw_buffer(); }

  // Returns the image width.
  const int width() const { return buffer_.dim(0).extent(); }
  // Returns the image height.
  const int height() const { return buffer_.dim(1).extent(); }
  // Returns the number of color channels (3, or 4 if RGBA).
  const int channels() const { return buffer_.dim(2).extent(); }
  // Returns the image row stride.
  const int row_stride() const { return buffer_.dim(1).stride(); }

 private:
  void Initialize(uint8_t* data, int width, int height, bool alpha);

  // Non-NULL iff this RgbBuffer owns its backing buffer.
  std::unique_ptr<uint8_t[]> owned_buffer_;

  // Backing buffer: layout is always width x height x channel (interleaved).
  Halide::Runtime::Buffer<uint8_t> buffer_;
};

}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_RGB_BUFFER_H_
