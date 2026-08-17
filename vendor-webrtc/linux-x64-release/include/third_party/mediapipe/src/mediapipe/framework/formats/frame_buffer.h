/* Copyright 2023 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_FRAME_BUFFER_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_FRAME_BUFFER_H_

#include <cstdint>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/status/statusor.h"

namespace mediapipe {

// A `FrameBuffer` provides a view into the provided backing buffer (e.g. camera
// frame or still image) with buffer format information. FrameBuffer doesn't
// take ownership of the provided backing buffer. The caller is responsible to
// manage the backing buffer lifecycle for the lifetime of the FrameBuffer.
//
// Examples:
//
// // Create an metadata instance with no backing buffer.
// FrameBuffer buffer{/*planes=*/{}, dimension, kRGBA};
//
// // Create an RGBA instance with backing buffer on single plane.
// FrameBuffer::Plane plane{rgba_buffer, /*stride=*/{dimension.width * 4, 4}};
// FrameBuffer buffer{{plane}, dimension, kRGBA, kTopLeft)};
//
// // Create an YUV instance with planar backing buffer.
// FrameBuffer::Plane y_plane{y_buffer, /*stride=*/{dimension.width , 1}};
// FrameBuffer::Plane uv_plane{u_buffer, /*stride=*/{dimension.width, 2}};
// FrameBuffer buffer{{y_plane, uv_plane}, dimension, kNV21};
class FrameBuffer {
 public:
  // Colorspace formats.
  enum class Format {
    kRGBA,
    kRGB,
    kNV12,
    kNV21,
    kYV12,
    kYV21,
    kGRAY,
    kUNKNOWN
  };

  // Stride information.
  struct Stride {
    // The row stride in bytes. This is the distance between the start pixels of
    // two consecutive rows in the image.
    int row_stride_bytes;
    // This is the distance between two consecutive pixel values in a row of
    // pixels in bytes. It may be larger than the size of a single pixel to
    // account for interleaved image data or padded formats.
    int pixel_stride_bytes;

    bool operator==(const Stride& other) const {
      return row_stride_bytes == other.row_stride_bytes &&
             pixel_stride_bytes == other.pixel_stride_bytes;
    }

    bool operator!=(const Stride& other) const { return !operator==(other); }
  };

  // Plane encapsulates buffer and stride information.
  struct Plane {
    Plane(uint8_t* buffer, Stride stride) : buffer_(buffer), stride_(stride) {}
    const uint8_t* buffer() const { return buffer_; }
    uint8_t* mutable_buffer() { return buffer_; }
    Stride stride() const { return stride_; }

   private:
    uint8_t* buffer_;
    Stride stride_;
  };

  // Dimension information for the whole frame or a cropped portion of it.
  struct Dimension {
    // The width dimension in pixel unit.
    int width;
    // The height dimension in pixel unit.
    int height;

    bool operator==(const Dimension& other) const {
      return width == other.width && height == other.height;
    }

    bool operator!=(const Dimension& other) const {
      return width != other.width || height != other.height;
    }

    bool operator>=(const Dimension& other) const {
      return width >= other.width && height >= other.height;
    }

    bool operator<=(const Dimension& other) const {
      return width <= other.width && height <= other.height;
    }

    // Swaps width and height.
    void Swap() {
      using std::swap;
      swap(width, height);
    }

    // Returns area represented by width * height.
    int Size() const { return width * height; }
  };

  // YUV data structure.
  struct YuvData {
    const uint8_t* y_buffer;
    const uint8_t* u_buffer;
    const uint8_t* v_buffer;
    // Y buffer row stride in bytes.
    int y_row_stride;
    // U/V buffer row stride in bytes.
    int uv_row_stride;
    // U/V pixel stride in bytes. This is the distance between two consecutive
    // u/v pixel values in a row.
    int uv_pixel_stride;
  };

  // Builds a FrameBuffer object from a row-major backing buffer.
  //
  // The FrameBuffer does not take ownership of the backing buffer. The caller
  // is responsible for maintaining the backing buffer lifecycle for the
  // lifetime of FrameBuffer.
  FrameBuffer(const std::vector<Plane>& planes, Dimension dimension,
              Format format)
      : planes_(planes), dimension_(dimension), format_(format) {}

  // Returns number of planes.
  int plane_count() const { return planes_.size(); }

  // Returns plane indexed by the input `index`.
  const Plane& plane(int index) const {
    ABSL_CHECK_GE(index, 0);
    ABSL_CHECK_LT(static_cast<size_t>(index), planes_.size());
    return planes_[index];
  }

  // Returns mutable plane indexed by the input `index`.
  Plane mutable_plane(int index) {
    ABSL_CHECK_GE(index, 0);
    ABSL_CHECK_LT(static_cast<size_t>(index), planes_.size());
    return planes_[index];
  }

  // Returns FrameBuffer dimension.
  Dimension dimension() const { return dimension_; }

  // Returns FrameBuffer format.
  Format format() const { return format_; }

  // Returns YuvData which contains the Y, U, and V buffer and their
  // stride info from the input `source` FrameBuffer which is in the YUV family
  // formats (e.g NV12, NV21, YV12, and YV21).
  static absl::StatusOr<YuvData> GetYuvDataFromFrameBuffer(
      const FrameBuffer& source);

 private:
  std::vector<Plane> planes_;
  Dimension dimension_;
  Format format_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_FRAME_BUFFER_H_
