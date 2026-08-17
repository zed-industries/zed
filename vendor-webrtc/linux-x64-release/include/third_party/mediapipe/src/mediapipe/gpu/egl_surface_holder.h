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

#ifndef MEDIAPIPE_GPU_EGL_SURFACE_HOLDER_H_
#define MEDIAPIPE_GPU_EGL_SURFACE_HOLDER_H_

#include "absl/base/thread_annotations.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/gpu/gl_base.h"

#if HAS_EGL

namespace mediapipe {

// This is used to pass an EGLSurface to a GlSurfaceSinkCalculator.
struct EglSurfaceHolder {
  // Access to the surface needs to be protected by a mutex to ensure that the
  // application does not destroy the surface while MediaPipe is using it.
  // NOTE: Code that needs to grab the GlContext mutex should always do so
  // before grabbing this one. For example, do not call GlContext::Run or
  // GlCalculatorHelper::RunInGlContext while holding this mutex, but instead
  // grab this inside the callable passed to them.
  absl::Mutex mutex;
  EGLSurface surface ABSL_GUARDED_BY(mutex) = EGL_NO_SURFACE;
  // True if MediaPipe created the surface and is responsible for destroying it.
  bool owned ABSL_GUARDED_BY(mutex) = false;
  // Vertical flip of the surface, useful for conversion between coordinate
  // systems with top-left v.s. bottom-left origins.
  bool flip_y = false;
  // If true, update the surface presentation timestamp from the MediaPipe
  // packet on Android. It is set to 1000 times the packet timestamp to convert
  // from microseconds (packet) to nanoseconds (surface).
  //
  // This enables consumers to control the presentation time on a SurfaceView or
  // to recover the timestamp with ImageReader or SurfaceTexture.
  //
  // See
  // https://registry.khronos.org/EGL/extensions/ANDROID/EGL_ANDROID_presentation_time.txt
  // for details about the meaning of the presentation time.
  //
  // See also
  //  * https://developer.android.com/reference/android/media/Image#getTimestamp()
  //  * https://developer.android.com/reference/android/graphics/SurfaceTexture#getTimestamp()
  bool update_presentation_time = false;
};

}  // namespace mediapipe

#endif  // HAS_EGL
#endif  // MEDIAPIPE_GPU_EGL_SURFACE_HOLDER_H_
