/*
 * Copyright (C) 2018 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_EXT_BASE_EVENT_FD_H_
#define INCLUDE_PERFETTO_EXT_BASE_EVENT_FD_H_

#include "perfetto/base/build_config.h"
#include "perfetto/base/platform_handle.h"
#include "perfetto/ext/base/scoped_file.h"

namespace perfetto {
namespace base {

// A waitable event that can be used with poll/select.
// This is really a wrapper around eventfd_create with a pipe-based fallback
// for other platforms where eventfd is not supported.
class EventFd {
 public:
  EventFd();
  ~EventFd();
  EventFd(EventFd&&) noexcept = default;
  EventFd& operator=(EventFd&&) = default;

  // The non-blocking file descriptor that can be polled to wait for the event.
  PlatformHandle fd() const { return event_handle_.get(); }

  // Can be called from any thread.
  void Notify();

  // Can be called from any thread. If more Notify() are queued a Clear() call
  // can clear all of them (up to 16 per call).
  void Clear();

 private:
  // The eventfd, when eventfd is supported, otherwise this is the read end of
  // the pipe for fallback mode.
  ScopedPlatformHandle event_handle_;

// QNX is specified because it is a non-Linux UNIX platform but it
// still sets the PERFETTO_OS_LINUX flag to be as compatible as possible
// with the Linux build.
#if !PERFETTO_BUILDFLAG(PERFETTO_OS_LINUX_BUT_NOT_QNX) && \
    !PERFETTO_BUILDFLAG(PERFETTO_OS_ANDROID) &&           \
    !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  // On Mac and other non-Linux UNIX platforms a pipe-based fallback is used.
  // The write end of the wakeup pipe.
  ScopedFile write_fd_;
#endif
};

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_EVENT_FD_H_
