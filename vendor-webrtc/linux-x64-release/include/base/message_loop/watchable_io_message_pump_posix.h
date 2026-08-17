// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_WATCHABLE_IO_MESSAGE_PUMP_POSIX_H_
#define BASE_MESSAGE_LOOP_WATCHABLE_IO_MESSAGE_PUMP_POSIX_H_

#include "base/location.h"

namespace base {

class WatchableIOMessagePumpPosix {
 public:
  // Used with WatchFileDescriptor to asynchronously monitor the I/O readiness
  // of a file descriptor.
  class FdWatcher {
   public:
    virtual void OnFileCanReadWithoutBlocking(int fd) = 0;
    virtual void OnFileCanWriteWithoutBlocking(int fd) = 0;

   protected:
    virtual ~FdWatcher() = default;
  };

  class FdWatchControllerInterface {
   public:
    explicit FdWatchControllerInterface(const Location& from_here);

    FdWatchControllerInterface(const FdWatchControllerInterface&) = delete;
    FdWatchControllerInterface& operator=(const FdWatchControllerInterface&) =
        delete;

    // Subclasses must call StopWatchingFileDescriptor() in their destructor
    // (this parent class cannot generically do it for them as it must usually
    // be invoked before they destroy their state which happens before the
    // parent destructor is invoked).
    virtual ~FdWatchControllerInterface();

    // NOTE: This method isn't called StopWatching() to avoid confusion with the
    // win32 ObjectWatcher class. While this doesn't really need to be virtual
    // as there's only one impl per platform and users don't use pointers to the
    // base class. Having this interface forces implementers to share similar
    // implementations (a problem in the past).

    // Stop watching the FD, always safe to call.  No-op if there's nothing to
    // do.
    virtual bool StopWatchingFileDescriptor() = 0;

    const Location& created_from_location() const {
      return created_from_location_;
    }

   private:
    const Location created_from_location_;
  };

  enum Mode {
    WATCH_READ = 1 << 0,
    WATCH_WRITE = 1 << 1,
    WATCH_READ_WRITE = WATCH_READ | WATCH_WRITE
  };

  // Every subclass of WatchableIOMessagePumpPosix must provide a
  // WatchFileDescriptor() which has the following signature where
  // |FdWatchController| must be the complete type based on
  // FdWatchControllerInterface.

  // Registers |delegate| with the current thread's message loop so that its
  // methods are invoked when file descriptor |fd| becomes ready for reading or
  // writing (or both) without blocking.  |mode| selects ready for reading, for
  // writing, or both.  See "enum Mode" above.  |controller| manages the
  // lifetime of registrations. ("Registrations" are also ambiguously called
  // "events" in many places, for instance in libevent.)  It is an error to use
  // the same |controller| for different file descriptors; however, the same
  // controller can be reused to add registrations with a different |mode|.  If
  // |controller| is already attached to one or more registrations, the new
  // registration is added onto those.  If an error occurs while calling this
  // method, any registration previously attached to |controller| is removed.
  // Returns true on success.  Must be called on the same thread the MessagePump
  // is running on.
  // bool WatchFileDescriptor(int fd,
  //                          bool persistent,
  //                          int mode,
  //                          FdWatchController* controller,
  //                          FdWatcher* delegate) = 0;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_WATCHABLE_IO_MESSAGE_PUMP_POSIX_H_
