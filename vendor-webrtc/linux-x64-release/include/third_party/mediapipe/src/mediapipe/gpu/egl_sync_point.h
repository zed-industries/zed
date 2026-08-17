#ifndef MEDIAPIPE_GPU_EGL_SYNC_POINT_H_
#define MEDIAPIPE_GPU_EGL_SYNC_POINT_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/gpu/egl_sync.h"
#include "mediapipe/gpu/gl_context.h"

namespace mediapipe {

absl::StatusOr<std::unique_ptr<GlSyncPoint>> CreateEglSyncPoint(
    std::shared_ptr<GlContext> gl_context, EglSync egl_sync);

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_EGL_SYNC_POINT_H_
