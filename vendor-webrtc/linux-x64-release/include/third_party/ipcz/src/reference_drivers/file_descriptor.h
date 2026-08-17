// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef IPCZ_SRC_REFERENCE_DRIVERS_FILE_DESCRIPTOR_H_
#define IPCZ_SRC_REFERENCE_DRIVERS_FILE_DESCRIPTOR_H_

#include <utility>

namespace ipcz::reference_drivers {

// Implements unique ownership of a single POSIX file descriptor.
class FileDescriptor {
 public:
  FileDescriptor();
  explicit FileDescriptor(int fd);

  FileDescriptor(const FileDescriptor&) = delete;
  FileDescriptor& operator=(const FileDescriptor&) = delete;

  FileDescriptor(FileDescriptor&& other);
  FileDescriptor& operator=(FileDescriptor&& other);

  ~FileDescriptor();

  void reset();

  [[nodiscard]] int release() { return std::exchange(fd_, -1); }

  // Duplicates the underlying descriptor, returning a new FileDescriptor object
  // to wrap it. This object must be valid before calling Clone().
  FileDescriptor Clone() const;

  bool is_valid() const { return fd_ != -1; }

  int get() const { return fd_; }

 private:
  int fd_ = -1;
};

}  // namespace ipcz::reference_drivers

#endif  // IPCZ_SRC_REFERENCE_DRIVERS_FILE_DESCRIPTOR_H_
