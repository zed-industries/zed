#ifndef MEDIAPIPE_GPU_EGL_ERRORS_H_
#define MEDIAPIPE_GPU_EGL_ERRORS_H_

#include "absl/status/status.h"

namespace mediapipe {

// Returns the error of the last called EGL function in the current thread.
absl::Status GetEglError();

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_EGL_ERRORS_H_
