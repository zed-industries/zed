// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_SCOPED_HARDWARE_BUFFER_HANDLE_H_
#define BASE_ANDROID_SCOPED_HARDWARE_BUFFER_HANDLE_H_

#include "base/base_export.h"
#include "base/files/scoped_file.h"
#include "base/memory/raw_ptr.h"

extern "C" typedef struct AHardwareBuffer AHardwareBuffer;

namespace base {
namespace android {

// Owns a single reference to an AHardwareBuffer object.
class BASE_EXPORT ScopedHardwareBufferHandle {
 public:
  ScopedHardwareBufferHandle();

  // Takes ownership of |other|'s buffer reference. Does NOT acquire a new one.
  ScopedHardwareBufferHandle(ScopedHardwareBufferHandle&& other);

  ScopedHardwareBufferHandle(const ScopedHardwareBufferHandle&) = delete;
  ScopedHardwareBufferHandle& operator=(const ScopedHardwareBufferHandle&) =
      delete;

  // Releases this handle's reference to the underlying buffer object if still
  // valid.
  ~ScopedHardwareBufferHandle();

  // Assumes ownership of an existing reference to |buffer|. This does NOT
  // acquire a new reference.
  static ScopedHardwareBufferHandle Adopt(AHardwareBuffer* buffer);

  // Adds a reference to |buffer| managed by this handle.
  static ScopedHardwareBufferHandle Create(AHardwareBuffer* buffer);

  // Takes ownership of |other|'s buffer reference. Does NOT acquire a new one.
  ScopedHardwareBufferHandle& operator=(ScopedHardwareBufferHandle&& other);

  bool is_valid() const;

  AHardwareBuffer* get() const;

  // Releases this handle's reference to the underlying buffer object if still
  // valid. Invalidates this handle.
  void reset();

  // Passes implicit ownership of this handle's reference over to the caller,
  // invalidating |this|. Returns the raw buffer handle.
  //
  // The caller is responsible for eventually releasing this reference to the
  // buffer object.
  [[nodiscard]] AHardwareBuffer* Take();

  // Creates a new handle with its own newly acquired reference to the
  // underlying buffer object. |this| must be a valid handle.
  ScopedHardwareBufferHandle Clone() const;

  // Consumes a handle and returns a file descriptor which can be used to
  // transmit the handle over IPC. A subsequent receiver may use
  // |DeserializeFromFileDescriptor()| to recover the buffer handle.
  //
  // NOTE: The returned file descriptor DOES NOT own a reference to the
  // underlying AHardwareBuffer. When using this for IPC, the caller is
  // responsible for retaining at least one reference to the buffer object to
  // keep it alive while the descriptor is in transit.
  ScopedFD SerializeAsFileDescriptor() const;

  // Consumes the supplied single-use file descriptor (which must have been
  // returned by a previous call to |SerializeAsFileDescriptor()|, perhaps in
  // a different process), and recovers an AHardwareBuffer object from it.
  //
  // This acquires a new reference to the AHardwareBuffer, with ownership passed
  // to the caller via the returned ScopedHardwareBufferHandle.
  [[nodiscard]] static ScopedHardwareBufferHandle DeserializeFromFileDescriptor(
      ScopedFD fd);

 private:
  // Assumes ownership of an existing reference to |buffer|. This does NOT
  // acquire a new reference.
  explicit ScopedHardwareBufferHandle(AHardwareBuffer* buffer);

  raw_ptr<AHardwareBuffer> buffer_ = nullptr;
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_SCOPED_HARDWARE_BUFFER_HANDLE_H_
