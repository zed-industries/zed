// Copyright 2022 The gRPC Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//
// WakeupFd abstracts the concept of a file descriptor for the purpose of
// waking up a thread in select()/poll()/epoll_wait()/etc.

// The poll() family of system calls provide a way for a thread to block until
// there is activity on one (or more) of a set of file descriptors. An
// application may wish to wake up this thread to do non file related work. The
// typical way to do this is to add a pipe to the set of file descriptors, then
// write to the pipe to wake up the thread in poll().
//
// Linux has a lighter weight eventfd specifically designed for this purpose.
// WakeupFd abstracts the difference between the two.
//
// Setup:
// 1. Call CreateWakeupFd() to crete an initialized WakeupFd.
// 2. Add the result of WakeupFd::ReadFd() to the set of monitored file
//    descriptors for the poll() style API you are using. Monitor the file
//    descriptor for readability.
// 3. To tear down, call WakeupFd::Destroy(). This closes the underlying
//    file descriptor.
//
// Usage:
// 1. To wake up a polling thread, call WakeupFd::Wakeup() on a wakeup_fd
//    it is monitoring.
// 2. If the polling thread was awakened by a WakeupFd event, call
//    WakeupFd::Consume() on it.
//
#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_WAKEUP_FD_POSIX_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_WAKEUP_FD_POSIX_H

#include <grpc/support/port_platform.h>

#include "absl/status/status.h"

namespace grpc_event_engine::experimental {

class WakeupFd {
 public:
  virtual absl::Status ConsumeWakeup() = 0;
  virtual absl::Status Wakeup() = 0;
  virtual ~WakeupFd() = default;

  int ReadFd() { return read_fd_; }
  int WriteFd() { return write_fd_; }

 protected:
  WakeupFd() : read_fd_(0), write_fd_(0) {}
  void SetWakeupFds(int read_fd, int write_fd) {
    read_fd_ = read_fd;
    write_fd_ = write_fd;
  }

 private:
  int read_fd_;
  int write_fd_;
};

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_WAKEUP_FD_POSIX_H
