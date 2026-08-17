// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_WRAPPED_FILE_DESCRIPTOR_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_WRAPPED_FILE_DESCRIPTOR_H_

#include "reference_drivers/file_descriptor.h"
#include "reference_drivers/object.h"

namespace ipcz::reference_drivers {

// Wraps a FileDescriptor as a driver object. The Linux multiprocess reference
// driver uses this to facilitate serialization of more complex objects into
// these readily transmissible objects.
class WrappedFileDescriptor
    : public ObjectImpl<WrappedFileDescriptor, Object::kFileDescriptor> {
 public:
  explicit WrappedFileDescriptor(FileDescriptor fd);

  const FileDescriptor& handle() const { return fd_; }
  FileDescriptor TakeDescriptor() { return std::move(fd_); }

  static IpczDriverHandle Create(FileDescriptor fd) {
    return ReleaseAsHandle(
        MakeRefCounted<WrappedFileDescriptor>(std::move(fd)));
  }

  static FileDescriptor UnwrapHandle(IpczDriverHandle handle) {
    return TakeFromHandle(handle)->TakeDescriptor();
  }

 private:
  ~WrappedFileDescriptor() override;

  FileDescriptor fd_;
};

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_WRAPPED_FILE_DESCRIPTOR_H_
