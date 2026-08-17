#ifndef MEDIAPIPE_GPU_EGL_SYNC_H_
#define MEDIAPIPE_GPU_EGL_SYNC_H_

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/shared_fd.h"
#include "mediapipe/framework/formats/unique_fd.h"
#include "mediapipe/gpu/egl_base.h"

namespace mediapipe {

// RAII wrapper for EGL sync object.
class EglSync {
 public:
  // Creates a fence in OpenGL command stream. This sync is enqueued and *not*
  // flushed.
  static absl::StatusOr<EglSync> Create(EGLDisplay display);

  // Creates a native fence in OpenGL command stream. This sync is enqueued and
  // *not* flushed.
  static absl::StatusOr<EglSync> CreateNative(EGLDisplay display);

  // Creates a native fence in OpenGL command stream based on a native fence FD.
  static absl::StatusOr<EglSync> CreateNative(EGLDisplay display,
                                              const UniqueFd& native_fence_fd);

  // Creates a native fence in OpenGL command stream based on a native fence FD.
  static absl::StatusOr<EglSync> CreateNative(EGLDisplay display,
                                              const SharedFd& native_fence_fd);

  static bool IsSupported(EGLDisplay display);
  static bool IsNativeSupported(EGLDisplay display);

  // Move-only
  EglSync(EglSync&& sync);
  EglSync& operator=(EglSync&& sync);

  EglSync(const EglSync&) = delete;
  EglSync& operator=(const EglSync&) = delete;

  ~EglSync() { Invalidate(); }

  // Causes GPU to block and wait until this sync has been signaled.
  // This call does not block and returns immediately.
  absl::Status WaitOnGpu();

  // Causes CPU to block and wait until this sync has been signaled.
  absl::Status Wait();

  // Returns the EGLDisplay on which this instance was created.
  EGLDisplay display() const { return display_; }

  // Returns the EGLSyncKHR wrapped by this instance.
  EGLSyncKHR sync() const { return sync_; }

  // Returns true if this EGL sync is signaled.
  absl::StatusOr<bool> IsSignaled();

  // Duplicates the file descriptor stored in native EGL fence sync.
  absl::StatusOr<UniqueFd> DupNativeFd();

 private:
  EglSync(EGLDisplay display, EGLSyncKHR sync)
      : display_(display), sync_(sync) {}

  // `native_fence_fd` - valid native fence FD.
  // NOTE: this function duplicates `native_fence_fd` (doesn't take ownership or
  // modifies it)
  static absl::StatusOr<EglSync> CreateNative(EGLDisplay display,
                                              int native_fence_fd);

  void Invalidate();

  EGLDisplay display_;
  EGLSyncKHR sync_ = EGL_NO_SYNC_KHR;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_EGL_SYNC_H_
