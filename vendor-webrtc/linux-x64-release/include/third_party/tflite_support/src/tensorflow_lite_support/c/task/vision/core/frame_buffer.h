/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_H_

#include <stdint.h>

// Defines C structs for holding the frame buffer.

#ifdef __cplusplus
extern "C" {
#endif  // __cplusplus

// Colorspace formats.
enum TfLiteFrameBufferFormat {
  kRGBA,
  kRGB,
  kNV12,
  kNV21,
  kYV12,
  kYV21,
  kGRAY,
  kUNKNOWN
};

// FrameBuffer content orientation follows EXIF specification. The name of
// each enum value defines the position of the 0th row and the 0th column of
// the image content. See http://jpegclub.org/exif_orientation.html for
// details.
enum TfLiteFrameBufferOrientation {
  kTopLeft = 1,
  kTopRight = 2,
  kBottomRight = 3,
  kBottomLeft = 4,
  kLeftTop = 5,
  kRightTop = 6,
  kRightBottom = 7,
  kLeftBottom = 8
};

// Dimension information for the whole frame.
struct TfLiteFrameBufferDimension {
  // The width dimension in pixel unit.
  int width;
  // The height dimension in pixel unit.
  int height;
};

// A `FrameBuffer` provides a view into the provided backing buffer (e.g. camera
// frame or still image) with buffer format information. FrameBuffer doesn't
// take ownership of the provided backing buffer. The caller is responsible to
// manage the backing buffer lifecycle for the lifetime of the FrameBuffer.
typedef struct TfLiteFrameBuffer {
  // Colorspace format of the frame buffer.
  enum TfLiteFrameBufferFormat format;
  // Orientation of the frame buffer.
  // If uninitialized or provided with a value outside the range of
  // TfLiteFrameBufferOrientation, it takes the value `kTopLeft`.
  enum TfLiteFrameBufferOrientation orientation;
  // Dimension information for the whole frame.
  struct TfLiteFrameBufferDimension dimension;
  // Holds the backing buffer for the frame buffer. Only single planar images
  // are supported as of now.
  uint8_t* buffer;
} TfLiteFrameBuffer;

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_H_
