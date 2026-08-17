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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_FLOAT_BUFFER_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_FLOAT_BUFFER_H_

#include "HalideBuffer.h"
#include "HalideRuntime.h"

namespace mediapipe {
namespace frame_buffer {

// FloatBuffer represents a view over an interleaved floating-point image.
//
// FloatBuffers may be copied and moved efficiently; their backing buffers are
// shared and never deep copied.
//
// FloatBuffer requires a minimum image width depending on the natural vector
// size of the platform, e.g., 16px. This is not validated by FloatBuffer.
class FloatBuffer {
 public:
  // Returns the size (in number of float) of a FloatBuffer given dimensions.
  static int FloatSize(int width, int height, int channels) {
    return width * height * channels;
  }

  // Builds a FloatBuffer using the given backing buffer and dimensions.
  FloatBuffer(float* data, int width, int height, int channels);

  // Builds a FloatBuffer using the given dimensions.
  //
  // The underlying backing buffer if allocated and owned by this FloatBuffer.
  FloatBuffer(int width, int height, int channels);

  // FloatBuffer is copyable. The source retains ownership of its backing
  // buffer.
  FloatBuffer(const FloatBuffer& other);
  // FloatBuffer is moveable. The source loses ownership of any backing buffers.
  FloatBuffer(FloatBuffer&& other);
  // FloatBuffer is assignable.
  FloatBuffer& operator=(const FloatBuffer& other);
  FloatBuffer& operator=(FloatBuffer&& other);

  ~FloatBuffer();

  // Release ownership of the owned backing buffer.
  float* Release() { return owned_buffer_.release(); }

  // Returns the halide_buffer_t* for the image.
  const halide_buffer_t* buffer() const { return buffer_.raw_buffer(); }
  // Returns the halide_buffer_t* for the image.
  halide_buffer_t* buffer() { return buffer_.raw_buffer(); }

  // Returns the image width.
  int width() const { return buffer_.dim(0).extent(); }
  // Returns the image height.
  int height() const { return buffer_.dim(1).extent(); }
  // Returns the number of channels.
  int channels() const { return buffer_.dim(2).extent(); }

 private:
  void Initialize(float* data, int width, int height, int channels);

  // Non-NULL iff this FloatBuffer owns its backing buffer.
  std::unique_ptr<float[]> owned_buffer_;

  // Backing buffer: layout is always width x height x channel (interleaved).
  Halide::Runtime::Buffer<float> buffer_;
};

}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_FLOAT_BUFFER_H_
