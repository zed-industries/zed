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

// This header includes platform-specific headers for OpenGL.

#ifndef MEDIAPIPE_GPU_GL_BASE_H_
#define MEDIAPIPE_GPU_GL_BASE_H_

#if defined(__EMSCRIPTEN__)
#include <emscripten/html5.h>
#endif  // defined(__EMSCRIPTEN__)

#if defined(__APPLE__)

#include <TargetConditionals.h>

#if TARGET_OS_OSX

#define HAS_NSGL 1

#include <OpenGL/OpenGL.h>

#if CGL_VERSION_1_3
#include <OpenGL/gl3.h>
#include <OpenGL/gl3ext.h>
#else
#include <OpenGL/gl.h>
#include <OpenGL/glext.h>
#endif  // CGL_VERSION_1_3

#else

#define HAS_EAGL 1

#include <OpenGLES/ES2/gl.h>
#include <OpenGLES/ES2/glext.h>
#include <OpenGLES/ES3/gl.h>
#include <OpenGLES/ES3/glext.h>

#endif  // TARGET_OS_OSX

#else

#define HAS_EGL 1

#include <EGL/egl.h>
// TODO: b/377324183 - add <EGL/eglext.h>
#include <GLES2/gl2.h>
#include <GLES2/gl2ext.h>
#if defined(__ANDROID__)
// Weak-link all GL APIs included from this point on.
// TODO: Annotate these with availability attributes for the
// appropriate versions of Android, by including gl{3,31,31}.h and resetting
// GL_APICALL for each.
#undef GL_APICALL
#define GL_APICALL __attribute__((weak_import)) KHRONOS_APICALL
#endif  // defined(__ANDROID__)

#include <GLES3/gl32.h>

// When using the Linux EGL headers, we may end up pulling a
// "#define Status int" from Xlib.h, which interferes with absl::Status.
#undef Status

// More crud from X
#undef None
#undef Bool
#undef Success

// When using Windows, we may end up pulling a #define for GetObject.
#undef GetObject

#endif  // defined(__APPLE__)

namespace mediapipe {

// Doing this as an inline function allows us to avoid unwanted "pointer will
// never be null" errors on certain platforms and compilers.
template <typename T>
inline bool SymbolAvailable(T* symbol) {
  return symbol != nullptr;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_BASE_H_
