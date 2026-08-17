#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_AHWB_VIEW_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_AHWB_VIEW_H_

#include "mediapipe/framework/port.h"

#ifdef MEDIAPIPE_GPU_BUFFER_USE_AHWB

#include <memory>
#include <utility>

#include "absl/functional/any_invocable.h"
#include "absl/status/status.h"
#include "mediapipe/framework/formats/hardware_buffer.h"
#include "mediapipe/framework/formats/shared_fd.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"

namespace mediapipe {

// Wrapper to facilitate short lived access to Android Hardware Buffer objects.
// Intended use cases:
// - Extracting an AHWB for processing in another library after it's produced by
// MediaPipe.
// - Sending AHWBs to compute devices that are able to map the memory for their
// own usage.
// The AHWB abstractions in GpuBuffer and Tensor are likely more suitable for
// other CPU/GPU uses of AHWBs.

class AhwbView {
 public:
  explicit AhwbView(
      HardwareBuffer* ahwb, int width_step_bytes,
      absl::AnyInvocable<absl::Status(SharedFd)> set_usage_fence_fn)
      : ahwb_(ahwb),
        width_step_bytes_(width_step_bytes),
        set_usage_fence_fn_(std::move(set_usage_fence_fn)) {}
  // Non-copyable
  AhwbView(const AhwbView&) = delete;
  AhwbView& operator=(const AhwbView&) = delete;
  // Non-movable
  AhwbView(AhwbView&&) = delete;

  // Supports only synchronous read usage - all users of GetHandle must finish
  // accessing the buffer before this view object is destroyed to avoid race
  // conditions).
  //
  // Supports async write usage - user must provide a usage fence which is
  // signaled when the write is complete. See more details in `SetUsageFence`.
  // TODO: Support full async usage.
  const AHardwareBuffer* GetHandle() const {
    return ahwb_->GetAHardwareBuffer();
  }

  int GetWidthStepBytes() const { return width_step_bytes_; }

  // Sets usage fence for this AHWB:
  // - fence is not signaled => AHWB is still in use
  // - fence is signaled => AHWB is not in use anymore
  //
  // Example use case:
  // - Calculator gets AhwbView for writing where writing is done asynchronously
  //   and fence is created to indicate write completion. (E.g. TPU/DSP delegate
  //   is used and can provide a completion fence.)
  // - Calculator schedules async write, retrieves the completion fence and sets
  //   it using `SetUsageFence`.
  // - Calculator sends corresponding `GpuBuffer` to a downstream calculator.
  // - The downstream calculator gets `GlBufferView` for reading, `GpuBuffer`
  //   automatically imports and inserts the fence as GL fence sync ensuring
  //   following GL operations wait for write completion.
  //
  // TODO: b/376753887 - replace with a dedicated type (MP's Fence)
  absl::Status SetUsageFence(SharedFd fence) {
    return set_usage_fence_fn_(std::move(fence));
  }

 private:
  const HardwareBuffer* ahwb_;
  const int width_step_bytes_;
  absl::AnyInvocable<absl::Status(SharedFd)> set_usage_fence_fn_;
};

namespace internal {
// Makes this class available as a GpuBuffer view.
template <>
class ViewProvider<AhwbView> {
 public:
  virtual ~ViewProvider() = default;
  virtual const AhwbView GetReadView(types<AhwbView>) const = 0;
  virtual AhwbView GetWriteView(types<AhwbView>) = 0;
};

}  // namespace internal

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_BUFFER_USE_AHWB
#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_AHWB_VIEW_H_
