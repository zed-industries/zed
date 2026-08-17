#ifndef MEDIAPIPE_GPU_WEBGPU_WEBGPU_CHECK_H_
#define MEDIAPIPE_GPU_WEBGPU_WEBGPU_CHECK_H_

// Note: it is safe to include this header on any platform, even if WebGPU is
// not available. IsWebGpuAvailable() will just always return false on such
// platforms.

// Please note that MEDIAPIPE_USE_WEBGPU is misnamed. It's a build-time macro
// that indicates whether we can _build_ WebGPU code. The choice of whether to
// actually use WebGPU is made at runtime by checking IsWebGpuAvailable.

namespace mediapipe {

// Returns true if WebGPU is available to MediaPipe and can be used.
// This requires a WebGPU device to be set up; see auto_setup_webgpu.
bool IsWebGpuAvailable();

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_WEBGPU_WEBGPU_CHECK_H_
