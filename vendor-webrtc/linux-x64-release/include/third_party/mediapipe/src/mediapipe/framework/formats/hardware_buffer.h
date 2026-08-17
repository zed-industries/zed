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

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_HARDWARE_BUFFER_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_HARDWARE_BUFFER_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"

typedef struct AHardwareBuffer AHardwareBuffer;

namespace mediapipe {

struct HardwareBufferSpec {
  // Buffer pixel formats. See NDK's hardware_buffer.h for descriptions.
  enum AhwbFormat {
    // This must be kept in sync with NDK's hardware_buffer.h
    AHARDWAREBUFFER_FORMAT_R8G8B8A8_UNORM = 0x01,
    AHARDWAREBUFFER_FORMAT_R8G8B8_UNORM = 0x03,
    AHARDWAREBUFFER_FORMAT_R16G16B16A16_FLOAT = 0x16,
    AHARDWAREBUFFER_FORMAT_BLOB = 0x21,
    AHARDWAREBUFFER_FORMAT_R8_UNORM = 0x38,
  };

  // Buffer usage descriptions. See NDK's hardware_buffer.h for descriptions.
  enum AhwbUsage {
    // This must be kept in sync with NDK's hardware_buffer.h
    AHARDWAREBUFFER_USAGE_CPU_READ_NEVER = 0x0UL,
    AHARDWAREBUFFER_USAGE_CPU_READ_RARELY = 0x2UL,
    AHARDWAREBUFFER_USAGE_CPU_READ_OFTEN = 0x3UL,
    AHARDWAREBUFFER_USAGE_CPU_WRITE_NEVER = UINT64_C(0) << 4,
    AHARDWAREBUFFER_USAGE_CPU_WRITE_RARELY = UINT64_C(2) << 4,
    AHARDWAREBUFFER_USAGE_CPU_WRITE_OFTEN = UINT64_C(3) << 4,
    AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE = UINT64_C(1) << 8,
    AHARDWAREBUFFER_USAGE_GPU_FRAMEBUFFER = UINT64_C(1) << 9,
    AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER = UINT64_C(1) << 24,
  };

  // Hashing required to use HardwareBufferSpec as key in buffer pools. See
  // absl::Hash for details.
  template <typename H>
  friend H AbslHashValue(H h, const HardwareBufferSpec& spec) {
    return H::combine(std::move(h), spec.width, spec.height, spec.layers,
                      spec.format, spec.usage);
  }

  uint32_t width = 0;
  uint32_t height = 0;
  uint32_t layers = 0;
  uint32_t format = 0;
  uint64_t usage = 0;
  uint32_t stride = 0;
};

// Equality operators
inline bool operator==(const HardwareBufferSpec& lhs,
                       const HardwareBufferSpec& rhs) {
  return lhs.width == rhs.width && lhs.height == rhs.height &&
         lhs.layers == rhs.layers && lhs.format == rhs.format &&
         lhs.usage == rhs.usage;
}
inline bool operator!=(const HardwareBufferSpec& lhs,
                       const HardwareBufferSpec& rhs) {
  return !operator==(lhs, rhs);
}

// For internal use only. Thinly wraps the Android NDK AHardwareBuffer.
class HardwareBuffer {
 public:
  // Constructs a HardwareBuffer instance from a newly allocated Android NDK
  // AHardwareBuffer.
  static absl::StatusOr<HardwareBuffer> Create(const HardwareBufferSpec& spec);

  // Constructs a HardwareBuffer instance from an existing Android NDK
  // AHardwareBuffer.
  static absl::StatusOr<HardwareBuffer> WrapAndAcquireAHardwareBuffer(
      AHardwareBuffer* ahw_buffer);

  // Destructs the HardwareBuffer, releasing the AHardwareBuffer.
  ~HardwareBuffer();

  // Support HardwareBuffer moves.
  HardwareBuffer(HardwareBuffer&& other);

  // Delete assignment and copy constructors.
  HardwareBuffer(HardwareBuffer& other) = delete;
  HardwareBuffer(const HardwareBuffer& other) = delete;
  HardwareBuffer& operator=(const HardwareBuffer&) = delete;

  // Returns true if AHWB is supported.
  static bool IsSupported();

  // Lock the hardware buffer for the given usage flags. fence_file_descriptor
  // specifies a fence file descriptor on which to wait and close (takes
  // ownership) before locking the buffer. Returns raw memory address if lock is
  // successful, nullptr otherwise.
  ABSL_MUST_USE_RESULT absl::StatusOr<void*> Lock(
      uint64_t usage, std::optional<int> fence_file_descriptor = std::nullopt);

  // Unlocks the hardware buffer synchronously. This method blocks until
  // unlocking is complete.
  absl::Status Unlock();

  // Unlocks the hardware buffer asynchronously. It returns a file_descriptor
  // which can be used as a fence that is signaled once unlocking is complete.
  absl::StatusOr<int> UnlockAsync();

  // Returns the underlying raw AHardwareBuffer pointer to be used directly with
  // AHardwareBuffer APIs.
  AHardwareBuffer* GetAHardwareBuffer() const { return ahw_buffer_; }

  // Returns whether this HardwareBuffer contains a valid AHardwareBuffer.
  bool IsValid() const { return ahw_buffer_ != nullptr; }

  // Returns whether this HardwareBuffer is locked.
  bool IsLocked() const { return is_locked_; }

  // Releases the AHardwareBuffer.
  void Reset();

  // Ahwb's are aligned to an implementation specific cacheline size.
  absl::StatusOr<uint32_t> GetAlignedWidth() const;

  // Returns buffer spec.
  const HardwareBufferSpec& spec() const { return spec_; }

  // Called by ReusablePool when reusing this buffer.
  void Reuse() {}

 private:
  // Allocates an AHardwareBuffer instance;
  static absl::StatusOr<AHardwareBuffer*> AllocateAHardwareBuffer(
      const HardwareBufferSpec& spec);

  // Acquires an existing AHardwareBuffer instance and returns its
  // HardwareBufferSpec;
  static absl::StatusOr<HardwareBufferSpec> AcquireAHardwareBuffer(
      AHardwareBuffer* ahw_buffer);

  // Constructs a HardwareBuffer instance from an already acquired
  // AHardwareBuffer instance and its spec.
  HardwareBuffer(const HardwareBufferSpec& spec, AHardwareBuffer* ahwb);

  // Unlocks the hardware buffer. If fence_file_descriptor_ptr is not nullptr,
  // the function won't block and instead fence_file_descriptor_ptr will be set
  // to a file descriptor to become signaled once unlocking is complete.
  absl::Status UnlockInternal(int* fence_file_descriptor_ptr);

  // Releases ahw_buffer_ AHardwareBuffer instance;
  absl::Status ReleaseAHardwareBuffer();

  // Buffer spec.
  HardwareBufferSpec spec_ = {};

  // Android NDK AHardwareBuffer.
  AHardwareBuffer* ahw_buffer_ = nullptr;

  // Indicates if AHardwareBuffer is locked for reading or writing.
  bool is_locked_ = false;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_AHWB_BUFFER_H_
