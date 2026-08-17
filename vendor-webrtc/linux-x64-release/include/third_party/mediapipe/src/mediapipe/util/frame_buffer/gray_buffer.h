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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_GRAY_BUFFER_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_GRAY_BUFFER_H_

#include <memory>

#include "HalideBuffer.h"
#include "HalideRuntime.h"

namespace mediapipe {
namespace frame_buffer {

// GrayBuffer represents a view over a grayscale (i.e. luminance,
// or Y-only) buffer.
// GrayBuffer may be copied and moved efficiently; their backing buffers are
// shared and never deep copied.
// GrayBuffer requires a minimum image width depending on the natural vector
// size of the platform, e.g., 16px. This is not validated by GrayBuffer.
class GrayBuffer {
 public:
  // Returns the size (in bytes) of a grayscale image of the given
  // dimensions. The given dimensions contain padding.
  static int ByteSize(int buffer_width, int buffer_height) {
    const int size = buffer_width * buffer_height;
    return size;
  }

  // Builds a grayscale buffer with size as width * height. The buffer should
  // be in row-major order with no padding.
  //
  // Does not take ownership of any backing buffers, which must be large
  // enough to fit their contents.
  GrayBuffer(uint8_t* buffer, int width, int height);

  // Builds a grayscale buffer with size as width * height.
  //
  // The underlying backing buffer is allocated and owned by this
  // GrayBuffer.
  GrayBuffer(int width, int height);

  // GrayBuffer is copyable. The source retains ownership of its backing
  // buffers.
  //
  // Since the source retains ownership of its backing buffer, the source needs
  // to outlive this instance's lifetime when the backing buffer is owned by
  // the source. Otherwise, the passing in backing buffer should outlive this
  // instance.
  GrayBuffer(const GrayBuffer& other);
  // GrayBuffer is moveable. The source loses ownership of any backing buffers.
  // Specifically, if the source owns its backing buffer, after the move,
  // Release() will return nullptr.
  GrayBuffer(GrayBuffer&& other);

  // GrayBuffer is assignable. The source retains ownership of its backing
  // buffers.
  //
  // Since the source retains ownership of its backing buffer, the source needs
  // to outlive this instance's lifetime when the backing buffer is owned by the
  // source. Otherwise, the passing in backing buffer should outlive this
  // instance.
  GrayBuffer& operator=(const GrayBuffer& other);
  GrayBuffer& operator=(GrayBuffer&& other);

  ~GrayBuffer();

  // Performs an in-place crop. Modifies this buffer so that the new extent
  // matches that of the given crop rectangle -- (x0, y0) becomes (0, 0) and
  // the new width and height are x1 - x0 + 1 and y1 - y0 + 1, respectively.
  bool Crop(int x0, int y0, int x1, int y1);

  // Resizes this image to match the dimensions of the given output GrayBuffer
  // and places the result into output's backing buffer.
  //
  // Note, if the output backing buffer is shared with multiple instances, by
  // calling this method, all the instances' backing buffers will change.
  bool Resize(GrayBuffer* output);

  // Rotates this image into the given buffer by the given angle (90, 180, 270).
  //
  // Rotation is specified in degrees counter-clockwise such that when rotating
  // by 90 degrees, the top-right corner of the source becomes the top-left of
  // the output. The output buffer must have its height and width swapped when
  // rotating by 90 or 270.
  //
  // Any angle values other than (90, 180, 270) are invalid.
  //
  // Note, if the output backing buffer is shared with multiple instances, by
  // calling this method, all the instances' backing buffers will change.
  bool Rotate(int angle, GrayBuffer* output);

  // Flips this image horizontally/vertically into the given buffer. Both buffer
  // dimensions must match.
  //
  // Note, if the output backing buffer is shared with multiple instances, by
  // calling this method, all the instances' backing buffers will change.
  bool FlipHorizontally(GrayBuffer* output);
  bool FlipVertically(GrayBuffer* output);

  // Releases ownership of the owned backing buffer.
  uint8_t* Release() { return owned_buffer_.release(); }

  // Returns the halide_buffer_t* for the image.
  halide_buffer_t* buffer() { return buffer_.raw_buffer(); }

  // Returns the image width.
  const int width() const { return buffer_.dim(0).extent(); }
  // Returns the image height.
  const int height() const { return buffer_.dim(1).extent(); }

 private:
  void Initialize(uint8_t* data, int width, int height);

  // Non-NULL iff this GrayBuffer owns its buffer.
  std::unique_ptr<uint8_t[]> owned_buffer_;

  // Backing buffer: layout is always width x height. The backing buffer binds
  // to either "owned_buffer_" or an external buffer.
  Halide::Runtime::Buffer<uint8_t> buffer_;
};

}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_GRAY_BUFFER_H_
