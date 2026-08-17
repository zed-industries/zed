// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_KQUEUE_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_KQUEUE_H_

#include <mach/mach.h>
#include <stdint.h>
#include <sys/event.h>

#include <vector>

#include "base/apple/scoped_mach_port.h"
#include "base/containers/id_map.h"
#include "base/files/scoped_file.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/message_loop/watchable_io_message_pump_posix.h"

namespace base {

// MessagePumpKqueue is used on macOS to drive an IO MessageLoop that is
// capable of watching both POSIX file descriptors and Mach ports.
class BASE_EXPORT MessagePumpKqueue : public MessagePump,
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

   protected:
    friend class MessagePumpKqueue;

    void Init(WeakPtr<MessagePumpKqueue> pump,
              int fd,
              int mode,
              FdWatcher* watcher);
    void Reset();

    int fd() { return fd_; }
    int mode() { return mode_; }
    FdWatcher* watcher() { return watcher_; }

   private:
    int fd_ = -1;
    int mode_ = 0;
    raw_ptr<FdWatcher> watcher_ = nullptr;
    WeakPtr<MessagePumpKqueue> pump_;
  };

  // Delegate interface that provides notifications of Mach message receive
  // events.
  class MachPortWatcher {
   public:
    virtual ~MachPortWatcher() = default;
    virtual void OnMachMessageReceived(mach_port_t port) = 0;
  };

  // Controller interface that is used to stop receiving events for an
  // installed MachPortWatcher.
  class MachPortWatchController {
   public:
    explicit MachPortWatchController(const Location& from_here);

    MachPortWatchController(const MachPortWatchController&) = delete;
    MachPortWatchController& operator=(const MachPortWatchController&) = delete;

    ~MachPortWatchController();

    bool StopWatchingMachPort();

   protected:
    friend class MessagePumpKqueue;

    void Init(WeakPtr<MessagePumpKqueue> pump,
              mach_port_t port,
              MachPortWatcher* watcher);
    void Reset();

    mach_port_t port() { return port_; }
    MachPortWatcher* watcher() { return watcher_; }

   private:
    mach_port_t port_ = MACH_PORT_NULL;
    raw_ptr<MachPortWatcher> watcher_ = nullptr;
    WeakPtr<MessagePumpKqueue> pump_;
    const Location from_here_;
  };

  MessagePumpKqueue();

  MessagePumpKqueue(const MessagePumpKqueue&) = delete;
  MessagePumpKqueue& operator=(const MessagePumpKqueue&) = delete;

  ~MessagePumpKqueue() override;

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();

  // MessagePump:
  void Run(Delegate* delegate) override;
  // Batched version of the loop used under experiment (crbug.com/1200141)
  void RunBatched(Delegate* delegate);
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;
  TimeTicks AdjustDelayedRunTime(TimeTicks earliest_time,
                                 TimeTicks run_time,
                                 TimeTicks latest_time) override;

  // Begins watching the Mach receive right named by |port|. The |controller|
  // can be used to stop watching for incoming messages, and new message
  // notifications are delivered to the |delegate|. Returns true if the watch
  // was successfully set-up and false on error.
  bool WatchMachReceivePort(mach_port_t port,
                            MachPortWatchController* controller,
                            MachPortWatcher* delegate);

  // WatchableIOMessagePumpPosix:
  bool WatchFileDescriptor(int fd,
                           bool persistent,
                           int mode,
                           FdWatchController* controller,
                           FdWatcher* delegate);

 private:
  // Called by the watch controller implementations to stop watching the
  // respective types of handles.
  bool StopWatchingMachPort(MachPortWatchController* controller);
  bool StopWatchingFileDescriptor(FdWatchController* controller);

  // Checks the |kqueue_| for events. If |next_work_info| is null, then the
  // kqueue will be polled for events. If it is non-null, it will wait for the
  // amount of time specified by the NextWorkInfo or until an event is
  // triggered. Returns whether any events were dispatched, with the events
  // stored in |events_|.
  bool DoInternalWork(Delegate* delegate,
                      Delegate::NextWorkInfo* next_work_info);

  // Called by DoInternalWork() to dispatch the user events stored in |events_|
  // that were triggered. |count| is the number of events to process. Returns
  // true if work was done, or false if no work was done.
  bool ProcessEvents(Delegate* delegate, size_t count);

  // Updates the wakeup timer to |wakeup_time| if it differs from the currently
  // scheduled wakeup. Clears the wakeup timer if |wakeup_time| is
  // base::TimeTicks::Max().
  // Updates |scheduled_wakeup_time_| to follow.
  void MaybeUpdateWakeupTimer(const base::TimeTicks& wakeup_time,
                              base::TimeDelta leeway);

  void SetWakeupTimerEvent(const base::TimeTicks& wakeup_time,
                           base::TimeDelta leeway,
                           kevent64_s* timer_event);

  // Receive right to which an empty Mach message is sent to wake up the pump
  // in response to ScheduleWork().
  apple::ScopedMachReceiveRight wakeup_;
  // Scratch buffer that is used to receive the message sent to |wakeup_|.
  mach_msg_empty_rcv_t wakeup_buffer_;

  // Watch controllers for FDs. IDs are generated by the map and are stored in
  // the kevent64_s::udata field.
  IDMap<FdWatchController*, uint64_t> fd_controllers_;

  // Watch controllers for Mach ports. IDs are the port being watched.
  IDMap<MachPortWatchController*, mach_port_t> port_controllers_;

  // The kqueue that drives the pump.
  ScopedFD kqueue_;

  // Whether the pump has been Quit() or not.
  bool keep_running_ = true;

  // The currently scheduled wakeup, if any. If no wakeup is scheduled,
  // contains base::TimeTicks::Max().
  base::TimeTicks scheduled_wakeup_time_{base::TimeTicks::Max()};

  // The number of events scheduled on the |kqueue_|. There is always at least
  // 1, for the |wakeup_| port.
  size_t event_count_ = 1;
  // Buffer used by DoInternalWork() to be notified of triggered events. This
  // is always at least |event_count_|-sized.
  std::vector<kevent64_s> events_{event_count_};

  WeakPtrFactory<MessagePumpKqueue> weak_factory_;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_KQUEUE_H_
