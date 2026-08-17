/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_SHARED_MEMORY_H_
#define MODULES_DESKTOP_CAPTURE_SHARED_MEMORY_H_

#include <stddef.h>

#if defined(WEBRTC_WIN)
// Forward declare HANDLE in a windows.h compatible way so that we can avoid
// including windows.h.
typedef void* HANDLE;
#endif

#include <memory>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// SharedMemory is a base class for shared memory. It stores all required
// parameters of the buffer, but doesn't have any logic to allocate or destroy
// the actual buffer. DesktopCapturer consumers that need to use shared memory
// for video frames must extend this class with creation and destruction logic
// specific for the target platform and then call
// DesktopCapturer::SetSharedMemoryFactory().
class RTC_EXPORT SharedMemory {
 public:
#if defined(WEBRTC_WIN)
  typedef HANDLE Handle;
  static const Handle kInvalidHandle;
#else
  typedef int Handle;
  static const Handle kInvalidHandle;
#endif

  void* data() const { return data_; }
  size_t size() const { return size_; }

  // Platform-specific handle of the buffer.
  Handle handle() const { return handle_; }

  // Integer identifier that can be used used by consumers of DesktopCapturer
  // interface to identify shared memory buffers it created.
  int id() const { return id_; }

  virtual ~SharedMemory() {}

  SharedMemory(const SharedMemory&) = delete;
  SharedMemory& operator=(const SharedMemory&) = delete;

 protected:
  SharedMemory(void* data, size_t size, Handle handle, int id);

  void* const data_;
  const size_t size_;
  const Handle handle_;
  const int id_;
};

// Interface used to create SharedMemory instances.
class SharedMemoryFactory {
 public:
  SharedMemoryFactory() {}
  virtual ~SharedMemoryFactory() {}

  SharedMemoryFactory(const SharedMemoryFactory&) = delete;
  SharedMemoryFactory& operator=(const SharedMemoryFactory&) = delete;

  virtual std::unique_ptr<SharedMemory> CreateSharedMemory(size_t size) = 0;
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_SHARED_MEMORY_H_
