// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_GLIB_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_GLIB_H_

#include <glib.h>

#include <memory>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/message_loop/watchable_io_message_pump_posix.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"

namespace base {

// This class implements a base MessagePump needed for TYPE_UI MessageLoops on
// platforms using GLib.
class BASE_EXPORT MessagePumpGlib : public MessagePump,
                                    public WatchableIOMessagePumpPosix {
 public:
  class FdWatchController : public FdWatchControllerInterface {
   public:
    explicit FdWatchController(const Location& from_here);

    FdWatchController(const FdWatchController&) = delete;
    FdWatchController& operator=(const FdWatchController&) = delete;

    ~FdWatchController() override;

    // FdWatchControllerInterface:
    bool StopWatchingFileDescriptor() override;

   private:
    friend class MessagePumpGlib;
    friend class MessagePumpGLibFdWatchTest;

    // FdWatchController instances can be reused (unless fd changes), so we
    // need to keep track of initialization status and taking it into account
    // when setting up a fd watching. Please refer to
    // WatchableIOMessagePumpPosix docs for more details. This is called by
    // WatchFileDescriptor() and sets up a GSource for the input parameters.
    // The source is not attached here, so the events will not be fired until
    // Attach() is called.
    bool InitOrUpdate(int fd, int mode, FdWatcher* watcher);
    // Returns the current initialization status.
    bool IsInitialized() const;

    // Tries to attach the internal GSource instance to the |pump|'s
    // GMainContext, so IO events start to be dispatched. Returns false if
    // |this| is not correctly initialized, otherwise returns true.
    bool Attach(MessagePumpGlib* pump);

    // Forward read and write events to |watcher_|. It is a no-op if watcher_
    // is null, which can happen when controller is suddenly stopped through
    // StopWatchingFileDescriptor().
    void NotifyCanRead();
    void NotifyCanWrite();

    raw_ptr<FdWatcher> watcher_ = nullptr;
    raw_ptr<GSource> source_ = nullptr;
    std::unique_ptr<GPollFD> poll_fd_;
    // If this pointer is non-null, the pointee is set to true in the
    // destructor.
    raw_ptr<bool> was_destroyed_ = nullptr;
  };

  MessagePumpGlib();

  MessagePumpGlib(const MessagePumpGlib&) = delete;
  MessagePumpGlib& operator=(const MessagePumpGlib&) = delete;

  ~MessagePumpGlib() override;

  // Part of WatchableIOMessagePumpPosix interface.
  // Please refer to WatchableIOMessagePumpPosix docs for more details.
  bool WatchFileDescriptor(int fd,
                           bool persistent,
                           int mode,
                           FdWatchController* controller,
                           FdWatcher* delegate);

  // Internal methods used for processing the pump callbacks. They are public
  // for simplicity but should not be used directly. HandlePrepare is called
  // during the prepare step of glib, and returns a timeout that will be passed
  // to the poll. HandleCheck is called after the poll has completed, and
  // returns whether or not HandleDispatch should be called. HandleDispatch is
  // called if HandleCheck returned true.
  int HandlePrepare();
  bool HandleCheck();
  void HandleDispatch();

  // Very similar to the above, with the key difference that these functions are
  // only used to track work items and never indicate work is available, and
  // poll indefinitely.
  void HandleObserverPrepare();
  bool HandleObserverCheck();

  // Overridden from MessagePump:
  void Run(Delegate* delegate) override;
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;

  // Internal methods used for processing the FdWatchSource callbacks. As for
  // main pump callbacks, they are public for simplicity but should not be used
  // directly.
  bool HandleFdWatchCheck(FdWatchController* controller);
  void HandleFdWatchDispatch(FdWatchController* controller);

 private:
  struct GMainContextDeleter {
    inline void operator()(GMainContext* context) const {
      if (context) {
        g_main_context_pop_thread_default(context);
        g_main_context_unref(context);
      }
    }
  };
  struct GSourceDeleter {
    inline void operator()(GSource* source) const {
      if (source) {
        g_source_destroy(source);
        g_source_unref(source);
      }
    }
  };
  bool ShouldQuit() const;

  // We may make recursive calls to Run, so we save state that needs to be
  // separate between them in this structure type.
  struct RunState;

  raw_ptr<RunState> state_;

  // Starts tracking a new work item and stores a `ScopedDoWorkItem` in
  // `state_`.
  void SetScopedWorkItem();
  // Gets rid of the current scoped work item.
  void ClearScopedWorkItem();
  // Ensures there's a ScopedDoWorkItem at the current run-level. This can be
  // useful for contexts where the caller can't tell whether they just woke up
  // or are continuing from native work.
  void EnsureSetScopedWorkItem();
  // Ensures there's no ScopedDoWorkItem at the current run-level. This can be
  // useful in contexts where the caller knows that a sleep is imminent but
  // doesn't know if the current context captures ongoing work (back from
  // native).
  void EnsureClearedScopedWorkItem();

  // Called before entrance to g_main_context_iteration to record context
  // related to nesting depth to track native nested loops which would otherwise
  // be invisible.
  void OnEntryToGlib();
  // Cleans up state set in OnEntryToGlib.
  void OnExitFromGlib();
  // Forces the pump into a nested state by creating two work items back to
  // back.
  void RegisterNested();
  // Removes all of the pump's ScopedDoWorkItems to remove the state of nesting
  // which was forced onto the pump.
  void UnregisterNested();
  // Nest if pump is not already marked as nested.
  void NestIfRequired();
  // Remove the nesting if the pump is nested.
  void UnnestIfRequired();

  std::unique_ptr<GMainContext, GMainContextDeleter> owned_context_;
  // This is a GLib structure that we can add event sources to.  On the main
  // thread, we use the default GLib context, which is the one to which all GTK
  // events are dispatched.
  raw_ptr<GMainContext> context_ = nullptr;

  // The work source.  It is shared by all calls to Run and destroyed when
  // the message pump is destroyed.
  std::unique_ptr<GSource, GSourceDeleter> work_source_;

  // The observer source.  It is shared by all calls to Run and destroyed when
  // the message pump is destroyed.
  std::unique_ptr<GSource, GSourceDeleter> observer_source_;

  // We use a wakeup pipe to make sure we'll get out of the glib polling phase
  // when another thread has scheduled us to do some work.  There is a glib
  // mechanism g_main_context_wakeup, but this won't guarantee that our event's
  // Dispatch() will be called.
  int wakeup_pipe_read_;
  int wakeup_pipe_write_;
  // Use a unique_ptr to avoid needing the definition of GPollFD in the header.
  std::unique_ptr<GPollFD> wakeup_gpollfd_;

  THREAD_CHECKER(watch_fd_caller_checker_);
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_GLIB_H_
