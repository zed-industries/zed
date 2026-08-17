// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_SCOPED_HARDWARE_BUFFER_FENCE_SYNC_H_
#define BASE_ANDROID_SCOPED_HARDWARE_BUFFER_FENCE_SYNC_H_

#include "base/android/scoped_hardware_buffer_handle.h"
#include "base/base_export.h"
#include "base/files/scoped_file.h"

namespace base {
namespace android {

// This class provides a ScopedHardwareBufferHandle and may include a fence
// which will be signaled when all pending work for the buffer has been finished
// and it can be safely read from.
class BASE_EXPORT ScopedHardwareBufferFenceSync {
 public:
  ScopedHardwareBufferFenceSync(
      base::android::ScopedHardwareBufferHandle handle,
      base::ScopedFD fence_fd,
      base::ScopedFD available_fence_fd);
  virtual ~ScopedHardwareBufferFenceSync();

  AHardwareBuffer* buffer() const { return handle_.get(); }
  ScopedHardwareBufferHandle TakeBuffer();
  ScopedFD TakeFence();
  ScopedFD TakeAvailableFence();

  // Provides fence which is signaled when the reads for this buffer are done
  // and it can be reused. Must only be called once.
  virtual void SetReadFence(base::ScopedFD fence_fd) = 0;

 private:
  ScopedHardwareBufferHandle handle_;
  ScopedFD fence_fd_;
  ScopedFD available_fence_fd_;
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_SCOPED_HARDWARE_BUFFER_FENCE_SYNC_H_
