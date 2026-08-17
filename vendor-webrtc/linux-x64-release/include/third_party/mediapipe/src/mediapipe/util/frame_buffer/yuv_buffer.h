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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_YUV_BUFFER_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_YUV_BUFFER_H_

#include <memory>

#include "HalideBuffer.h"
#include "HalideRuntime.h"

namespace mediapipe {
namespace frame_buffer {
class RgbBuffer;

// YuvBuffer represents a view over a YUV 4:2:0 image.
//
// YuvBuffers may be copied and moved efficiently; their backing buffers are
// shared and never deep copied.
//
// YuvBuffer requires a minimum image width depending on the natural vector
// size of the platform, e.g., 16px. This is not validated by YuvBuffer.
class YuvBuffer {
 public:
  // YUV formats. Rather than supporting every possible format, we prioritize
  // formats with broad hardware/platform support.
  //
  // Enum values are FourCC codes; see http://fourcc.org/yuv.php for more.
  enum Format {
    NV21 = 0x3132564E,  // YUV420SP (VU interleaved)
    YV12 = 0x32315659,  // YUV420P (VU planar)
  };

  // Returns the size (in bytes) of a YUV image of the given dimensions.
  static int ByteSize(int width, int height) {
    // 1 byte per pixel in the Y plane, 2 bytes per 2x2 block in the UV plane.
    // Dimensions with odd sizes are rounded up.
    const int y_size = width * height;
    const int uv_size = ((width + 1) / 2) * ((height + 1) / 2) * 2;
    return y_size + uv_size;
  }

  // Builds a generic YUV420 YuvBuffer with the given backing buffers,
  // dimensions and strides. Supports both interleaved or planar UV with
  // custom strides.
  //
  // Does not take ownership of any backing buffers, which must be large
  // enough to fit their contents.
  YuvBuffer(uint8_t* y_plane, uint8_t* u_plane, uint8_t* v_plane, int width,
            int height, int row_stride_y, int row_stride_uv,
            int pixel_stride_uv);

  // Builds a YuvBuffer using the given backing buffer, dimensions, and format.
  // Expects an NV21- or YV12-format image only.
  //
  // Does not take ownership of the backing buffer (provided in 'data'), which
  // must be sized to hold at least the amount indicated by ByteSize().
  YuvBuffer(uint8_t* data, int width, int height, Format format);

  // Builds a YuvBuffer using the given dimensions and format. Expects
  // an NV21- or YV12-format image only.
  //
  // The underlying backing buffer is allocated and owned by this YuvBuffer.
  YuvBuffer(int width, int height, Format format);

  // YuvBuffer is copyable. The source retains ownership of its backing buffers.
  YuvBuffer(const YuvBuffer& other);
  // YuvBuffer is moveable. The source loses ownership of any backing buffers.
  YuvBuffer(YuvBuffer&& other);
  // YuvBuffer is assignable.
  YuvBuffer& operator=(const YuvBuffer& other);
  YuvBuffer& operator=(YuvBuffer&& other);

  ~YuvBuffer();

  // Performs an in-place crop. Modifies this buffer so that the new extent
  // matches that of the given crop rectangle -- (x0, y0) becomes (0, 0) and
  // the new width and height are x1 - x0 + 1 and y1 - y0 + 1, respectively.
  //
  // Note that the top-left corner (x0, y0) coordinates must be even to
  // maintain alignment between the Y and UV grids.
  bool Crop(int x0, int y0, int x1, int y1);

  // Resize this image to match the dimensions of the given output YuvBuffer
  // and places the result into its backing buffer.
  //
  // Performs a resize with bilinear interpolation (over four source pixels).
  bool Resize(YuvBuffer* output);

  // Rotate this image into the given buffer by the given angle (90, 180, 270).
  //
  // Rotation is specified in degrees counter-clockwise such that when rotating
  // by 90 degrees, the top-right corner of the source becomes the top-left of
  // the output. The output buffer must have its height and width swapped when
  // rotating by 90 or 270.
  //
  // Any angle values other than (90, 180, 270) are invalid.
  bool Rotate(int angle, YuvBuffer* output);

  // Flip this image horizontally/vertically into the given buffer. Both buffer
  // dimensions must match.
  bool FlipHorizontally(YuvBuffer* output);
  bool FlipVertically(YuvBuffer* output);

  // Performs a YUV-to-RGB color format conversion and places the result
  // in the given output RgbBuffer. Both buffer dimensions must match.
  //
  // When halve is true, the converted output is downsampled by a factor of
  // two by discarding three of four luminance values in every 2x2 block.
  bool Convert(bool halve, RgbBuffer* output);

  // Release ownership of the owned backing buffer.
  uint8_t* Release() { return owned_buffer_.release(); }

  // Returns the halide_buffer_t* for the Y plane.
  const halide_buffer_t* y_buffer() const { return y_buffer_.raw_buffer(); }
  // Returns the halide_buffer_t* for the UV plane(s).
  const halide_buffer_t* uv_buffer() const { return uv_buffer_.raw_buffer(); }
  // Returns the halide_buffer_t* for the Y plane.
  halide_buffer_t* y_buffer() { return y_buffer_.raw_buffer(); }
  // Returns the halide_buffer_t* for the UV plane(s).
  halide_buffer_t* uv_buffer() { return uv_buffer_.raw_buffer(); }

  // Returns the image width.
  const int width() const { return y_buffer_.dim(0).extent(); }
  // Returns the image height.
  const int height() const { return y_buffer_.dim(1).extent(); }

 private:
  void Initialize(uint8_t* data, int width, int height, Format format);

  // Non-NULL iff this YuvBuffer owns its buffer.
  std::unique_ptr<uint8_t[]> owned_buffer_;

  // Y (luminance) backing buffer: layout is always width x height.
  Halide::Runtime::Buffer<uint8_t> y_buffer_;

  // UV (chrominance) backing buffer; width/2 x height/2 x 2 (channel).
  // May be interleaved or planar.
  //
  // Note that the planes are in the reverse of the usual order: channel 0 is V
  // and channel 1 is U.
  Halide::Runtime::Buffer<uint8_t> uv_buffer_;
};

}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_YUV_BUFFER_H_
