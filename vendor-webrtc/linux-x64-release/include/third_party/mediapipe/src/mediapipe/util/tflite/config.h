// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_TFLITE_CONFIG_H_
#define MEDIAPIPE_UTIL_TFLITE_CONFIG_H_

#include "mediapipe/framework/calculator_framework.h"

// MediaPipe code should use the following defines to determine whether TFLite
// GPU support is available, and whether GL or Metal inference is available.

#ifdef MEDIAPIPE_DISABLE_GL_COMPUTE
#define MEDIAPIPE_TFLITE_GL_INFERENCE 0
#else
#define MEDIAPIPE_TFLITE_GL_INFERENCE 1
#endif  // MEDIAPIPE_DISABLE_GL_COMPUTE

#ifdef MEDIAPIPE_IOS
#define MEDIAPIPE_TFLITE_METAL_INFERENCE 1
#else
#define MEDIAPIPE_TFLITE_METAL_INFERENCE 0
#endif  // MEDIAPIPE_IOS

#if TARGET_OS_OSX && !MEDIAPIPE_DISABLE_GPU
#define MEDIAPIPE_TFLITE_METAL_INFERENCE 1
#endif  // TARGET_OS_OSX && !MEDIAPIPE_DISABLE_GPU

#define MEDIAPIPE_TFLITE_GPU_SUPPORTED \
  ((MEDIAPIPE_TFLITE_GL_INFERENCE) || (MEDIAPIPE_TFLITE_METAL_INFERENCE))

#if MEDIAPIPE_TFLITE_GL_INFERENCE
#include "tensorflow/lite/delegates/gpu/gl/gl_buffer.h"
#endif  // MEDIAPIPE_TFLITE_GL_INFERENCE

#if MEDIAPIPE_TFLITE_METAL_INFERENCE
#import <Metal/Metal.h>
#endif  // MEDIAPIPE_TFLITE_METAL_INFERENCE

namespace mediapipe {

#if MEDIAPIPE_TFLITE_GL_INFERENCE
typedef ::tflite::gpu::gl::GlBuffer GpuTensor;
#elif MEDIAPIPE_TFLITE_METAL_INFERENCE
typedef id<MTLBuffer> GpuTensor;
#else
struct DummyGpuTensor {};
typedef DummyGpuTensor GpuTensor;  // Dummy define for less #ifdefs
#endif  // MEDIAPIPE_TFLITE_GL_INFERENCE

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TFLITE_CONFIG_H_
