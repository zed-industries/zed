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
//
// Defines symbols and methods related to portable portions of the framework.
//

#ifndef MEDIAPIPE_FRAMEWORK_PORT_H_
#define MEDIAPIPE_FRAMEWORK_PORT_H_

// Note: some defines, e.g. MEDIAPIPE_LITE, can only be set in the bazel rules.
// For consistency, we now set MEDIAPIPE_MOBILE there too. However, for the sake
// of projects that may want to build MediaPipe using alternative build systems,
// we also try to set platform-specific defines in this header if missing.
#if !defined(MEDIAPIPE_MOBILE) && \
    (defined(__ANDROID__) || defined(__EMSCRIPTEN__))
#define MEDIAPIPE_MOBILE
#endif

#if !defined(MEDIAPIPE_ANDROID) && defined(__ANDROID__)
#define MEDIAPIPE_ANDROID
#endif

#if defined(__APPLE__)
#include "TargetConditionals.h"  // for TARGET_OS_*
#if !defined(MEDIAPIPE_IOS) && !TARGET_OS_OSX
#define MEDIAPIPE_IOS

#if !defined(MEDIAPIPE_MOBILE) && !TARGET_OS_OSX
#define MEDIAPIPE_MOBILE
#endif

#endif
#if !defined(MEDIAPIPE_OSX) && TARGET_OS_OSX
#define MEDIAPIPE_OSX
#endif
#endif

// These platforms do not support OpenGL ES Compute Shaders (v3.1 and up),
// but may or may not still be able to run other OpenGL code.
#if !defined(MEDIAPIPE_DISABLE_GL_COMPUTE) &&                                  \
    (defined(__APPLE__) || defined(__EMSCRIPTEN__) || MEDIAPIPE_DISABLE_GPU || \
     MEDIAPIPE_USING_LEGACY_SWIFTSHADER)
#define MEDIAPIPE_DISABLE_GL_COMPUTE
#endif

// Compile time target platform definitions.
// Example: #if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_31
#define MEDIAPIPE_OPENGL_ES_20 200
#define MEDIAPIPE_OPENGL_ES_30 300
#define MEDIAPIPE_OPENGL_ES_31 310

// NOTE: MEDIAPIPE_OPENGL_ES_VERSION macro represents the maximum OpenGL ES
// version to build for. Runtime availability is _not_ guaranteed; in
// particular, uses of OpenGL ES 3.1 should be guarded by a runtime check.
// TODO: identify and fix code where macro is used incorrectly.
#if MEDIAPIPE_DISABLE_GPU
#define MEDIAPIPE_OPENGL_ES_VERSION 0
#define MEDIAPIPE_METAL_ENABLED 0
#else
#if defined(MEDIAPIPE_ANDROID)
#if defined(MEDIAPIPE_DISABLE_GL_COMPUTE)
#define MEDIAPIPE_OPENGL_ES_VERSION MEDIAPIPE_OPENGL_ES_30
#else
#define MEDIAPIPE_OPENGL_ES_VERSION MEDIAPIPE_OPENGL_ES_31
#endif
#define MEDIAPIPE_METAL_ENABLED 0
#elif defined(MEDIAPIPE_IOS)
#define MEDIAPIPE_OPENGL_ES_VERSION MEDIAPIPE_OPENGL_ES_30
#define MEDIAPIPE_METAL_ENABLED 1
#elif defined(MEDIAPIPE_OSX)
#define MEDIAPIPE_OPENGL_ES_VERSION 0
#if MEDIAPIPE_USE_WEBGPU
#define MEDIAPIPE_METAL_ENABLED 0
#else
#define MEDIAPIPE_METAL_ENABLED 1
#endif  // MEDIAPIPE_USE_WEBGPU
#elif defined(__EMSCRIPTEN__)
// WebGL config.
#define MEDIAPIPE_OPENGL_ES_VERSION MEDIAPIPE_OPENGL_ES_30
#define MEDIAPIPE_METAL_ENABLED 0
#else
#if MEDIAPIPE_USE_WEBGPU
// TODO: `MEDIAPIPE_USE_WEBGPU` does not necessarily imply
// anything about the GLES versions. But despite that, as a temporary
// measure, setting GLES version to 0 would save a lot of work for the
// current OSS efforts.
#define MEDIAPIPE_OPENGL_ES_VERSION 0
#else
#define MEDIAPIPE_OPENGL_ES_VERSION MEDIAPIPE_OPENGL_ES_31
#endif
#define MEDIAPIPE_METAL_ENABLED 0
#endif
#endif

#ifndef MEDIAPIPE_HAS_RTTI
// Detect if RTTI is disabled in the compiler.
#if defined(__clang__) && defined(__has_feature)
#define MEDIAPIPE_HAS_RTTI __has_feature(cxx_rtti)
#elif defined(__GNUC__) && !defined(__GXX_RTTI)
#define MEDIAPIPE_HAS_RTTI 0
#elif defined(_MSC_VER) && !defined(_CPPRTTI)
#define MEDIAPIPE_HAS_RTTI 0
#else
#define MEDIAPIPE_HAS_RTTI 1
#endif
#endif  // MEDIAPIPE_HAS_RTTI

// AHardware buffers are only available since Android API 26.
#if !defined(MEDIAPIPE_NO_JNI) || defined(MEDIAPIPE_ANDROID_LINK_NATIVE_WINDOW)
#if (__ANDROID_API__ >= 26) || defined(__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__)
#define MEDIAPIPE_GPU_BUFFER_USE_AHWB 1
#endif  // __ANDROID_API__ >= 26 ||
        // defined(__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__)
#endif  // !defined(MEDIAPIPE_NO_JNI) ||
        // defined(MEDIAPIPE_ANDROID_LINK_NATIVE_WINDOW)

// Supported use cases for tensor_ahwb:
// 1. Native code running in Android apps.
// 2. Android vendor processes linked against nativewindow.
#if !defined(MEDIAPIPE_NO_JNI) || defined(MEDIAPIPE_ANDROID_LINK_NATIVE_WINDOW)
#if __ANDROID_API__ >= 26 || defined(__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__)
#define MEDIAPIPE_TENSOR_USE_AHWB 1
#endif  // __ANDROID_API__ >= 26 ||
        // defined(__ANDROID_UNAVAILABLE_SYMBOLS_ARE_WEAK__)
#endif  // !defined(MEDIAPIPE_NO_JNI) ||
        // defined(MEDIAPIPE_ANDROID_LINK_NATIVE_WINDOW)

#endif  // MEDIAPIPE_FRAMEWORK_PORT_H_
