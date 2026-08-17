#ifndef MEDIAPIPE_GPU_EGL_BASE_H_
#define MEDIAPIPE_GPU_EGL_BASE_H_

#include "mediapipe/gpu/gl_base.h"

#if defined(HAS_EGL)

// TODO: b/377324183 - merge into gl_base.h

#include <EGL/egl.h>
#include <EGL/eglext.h>

#endif  // defined(HAS_EGL)

#endif  // MEDIAPIPE_GPU_EGL_BASE_H_
