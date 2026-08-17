// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_IO_WATCHER_H_
#define BASE_MESSAGE_LOOP_IO_WATCHER_H_

#include <memory>

#include "base/base_export.h"
#include "base/location.h"
#include "base/message_loop/message_pump_for_io.h"
#include "base/types/pass_key.h"
#include "build/build_config.h"

#if defined(IS_WINDOWS)
#include "base/win/windows_types.h"
#endif

#if BUILDFLAG(IS_MAC) || \
    (BUILDFLAG(IS_IOS) && !BUILDFLAG(CRONET_BUILD) && !BUILDFLAG(IS_IOS_TVOS))
#include <mach/mach.h>
#endif

namespace base {

// An object which can be used to register asynchronous IO handlers to wake the
// calling thread directly on interesting events. This is guaranteed to be
// usable on any MessagePumpType::IO thread, but it may also be usable on other
// thread types if the MessagePump implementation supports it.
class BASE_EXPORT IOWatcher {
 public:
  virtual ~IOWatcher() = default;

  // Returns a valid IOWatcher instance iff it's usable from the calling thread.
  // Returns null otherwise.
  static IOWatcher* Get();

#if !BUILDFLAG(IS_NACL)
#if BUILDFLAG(IS_WIN)
  // Please see MessagePumpWin for definitions of these methods.
  [[nodiscard]] bool RegisterIOHandler(HANDLE file,
                                       MessagePumpForIO::IOHandler* handler);
  bool RegisterJobObject(HANDLE job, MessagePumpForIO::IOHandler* handler);
#elif BUILDFLAG(IS_POSIX)
  class FdWatcher {
   public:
    virtual void OnFdReadable(int fd) = 0;
    virtual void OnFdWritable(int fd) = 0;

   protected:
    virtual ~FdWatcher() = default;
  };

  // Effectively controls the lifetime of a single active FD watch started by
  // WatchFileDescriptor() below.
  class FdWatch {
   public:
    // FdWatch destruction immediately ceases watching the corresponding FD.
    // Must be called on the same thread that made the original call to
    // WatchFileDescriptor().
    virtual ~FdWatch() = default;
  };

  // Asynchronously watches `fd` for IO. If successful, this returns a valid
  // FdWatch object and the FD remains watched until the FdWatch object is
  // destroyed OR a watched event occurs (for a non-persistent watch only);
  // whichever occurs first. While the watch is active, `fd_watcher` will be
  // invoked on the calling thread whenever an interesting IO event happens.
  //
  // The returned FdWatch MUST be destroyed on the calling thread, and
  // `fd_watcher` MUST outlive it.
  enum class FdWatchDuration {
    kOneShot,
    kPersistent,
  };
  enum class FdWatchMode {
    kRead,
    kWrite,
    kReadWrite,
  };
  std::unique_ptr<FdWatch> WatchFileDescriptor(
      int fd,
      FdWatchDuration duration,
      FdWatchMode mode,
      FdWatcher& fd_watcher,
      const Location& location = Location::Current());
#endif

#if BUILDFLAG(IS_MAC) || \
    (BUILDFLAG(IS_IOS) && !BUILDFLAG(CRONET_BUILD) && !BUILDFLAG(IS_IOS_TVOS))
  bool WatchMachReceivePort(
      mach_port_t port,
      MessagePumpForIO::MachPortWatchController* controller,
      MessagePumpForIO::MachPortWatcher* delegate);
#elif BUILDFLAG(IS_FUCHSIA)
  // Additional watch API for native platform resources.
  bool WatchZxHandle(zx_handle_t handle,
                     bool persistent,
                     zx_signals_t signals,
                     MessagePumpForIO::ZxHandleWatchController* controller,
                     MessagePumpForIO::ZxHandleWatcher* delegate);
#endif  // BUILDFLAG(IS_FUCHSIA)
#endif  // !BUILDFLAG(IS_NACL)

 protected:
  IOWatcher();

  // IOWatcher implementations must implement these methods for any applicable
  // platform(s).
#if !BUILDFLAG(IS_NACL)
#if BUILDFLAG(IS_WIN)
  virtual bool RegisterIOHandlerImpl(HANDLE file,
                                     MessagePumpForIO::IOHandler* handler) = 0;
  virtual bool RegisterJobObjectImpl(HANDLE job,
                                     MessagePumpForIO::IOHandler* handler) = 0;
#elif BUILDFLAG(IS_POSIX)
  virtual std::unique_ptr<FdWatch> WatchFileDescriptorImpl(
      int fd,
      FdWatchDuration duration,
      FdWatchMode mode,
      FdWatcher& fd_watcher,
      const Location& location) = 0;
#endif
#if BUILDFLAG(IS_MAC) || \
    (BUILDFLAG(IS_IOS) && !BUILDFLAG(CRONET_BUILD) && !BUILDFLAG(IS_IOS_TVOS))
  virtual bool WatchMachReceivePortImpl(
      mach_port_t port,
      MessagePumpForIO::MachPortWatchController* controller,
      MessagePumpForIO::MachPortWatcher* delegate) = 0;
#elif BUILDFLAG(IS_FUCHSIA)
  virtual bool WatchZxHandleImpl(
      zx_handle_t handle,
      bool persistent,
      zx_signals_t signals,
      MessagePumpForIO::ZxHandleWatchController* controller,
      MessagePumpForIO::ZxHandleWatcher* delegate) = 0;
#endif  // BUILDFLAG(IS_FUCHSIA)
#endif  // !BUILDFLAG(IS_NACL)
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_IO_WATCHER_H_
