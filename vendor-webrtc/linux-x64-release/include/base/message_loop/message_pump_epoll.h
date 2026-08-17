// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_EPOLL_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_EPOLL_H_

#include <poll.h>
#include <sys/epoll.h>

#include <cstdint>
#include <map>

#include "base/base_export.h"
#include "base/dcheck_is_on.h"
#include "base/feature_list.h"
#include "base/files/scoped_file.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/memory/ref_counted.h"
#include "base/memory/weak_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/message_loop/watchable_io_message_pump_posix.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "third_party/abseil-cpp/absl/container/inlined_vector.h"

#if DCHECK_IS_ON()
#include <deque>
#include <optional>

#include "base/debug/stack_trace.h"
#endif

namespace base {

// Use poll() rather than epoll().
//
// Why? epoll() is supposed to be strictly better. But it has one consequence
// we don't necessarily want: when writing to a AF_UNIX socket, the kernel
// will wake up the waiter with a "sync" wakeup. The concept of a "sync"
// wakeup has various consequences, but on Android it tends to bias the
// scheduler towards a "baton passing" mode, where the current thread yields
// its CPU to the target. This is desirable to lower latency.
//
// However, when using epoll_wait(), the "sync" flag is dropped from the
// wakeup path. This is not the case with poll(). So let's use it to preserve
// this behavior.
//
// Caveat: Since both we and the kernel need to walk the list of all fds at
// every call, don't do it when we have too many FDs.
BASE_FEATURE(kUsePollForMessagePumpEpoll,
             "UsePollForMessagePumpEpoll",
             base::FEATURE_DISABLED_BY_DEFAULT);

// A MessagePump implementation suitable for I/O message loops on Linux-based
// systems with epoll API support.
class BASE_EXPORT MessagePumpEpoll : public MessagePump,
                                     public WatchableIOMessagePumpPosix {
  class Interest;
  struct InterestParams;

 public:
  // Object which FD-watching clients must keep alive to continue watching
  // their FD. See WatchFileDescriptor() below.
  class FdWatchController : public FdWatchControllerInterface {
   public:
    explicit FdWatchController(const Location& from_here);

    FdWatchController(const FdWatchController&) = delete;
    FdWatchController& operator=(const FdWatchController&) = delete;

    // Implicitly calls StopWatchingFileDescriptor.
    ~FdWatchController() override;

    // FdWatchControllerInterface:
    bool StopWatchingFileDescriptor() override;

   private:
    friend class MessagePumpEpoll;
    friend class MessagePumpEpollTest;

    void set_watcher(FdWatcher* watcher) { watcher_ = watcher; }
    void set_pump(WeakPtr<MessagePumpEpoll> pump) { pump_ = std::move(pump); }
    const scoped_refptr<Interest>& interest() const { return interest_; }

    // Creates a new Interest described by `params` and adopts it as this
    // controller's exclusive interest. Any prior interest is dropped by the
    // controller and should be unregistered on the MessagePumpEpoll.
    const scoped_refptr<Interest>& AssignInterest(const InterestParams& params);
    void ClearInterest();

    void OnFdReadable();
    void OnFdWritable();

    raw_ptr<FdWatcher> watcher_ = nullptr;

    // If this pointer is non-null when the FdWatchController is destroyed, the
    // pointee is set to true.
    raw_ptr<bool> was_destroyed_ = nullptr;

    WeakPtr<MessagePumpEpoll> pump_;
    scoped_refptr<Interest> interest_;
  };

  MessagePumpEpoll();
  MessagePumpEpoll(const MessagePumpEpoll&) = delete;
  MessagePumpEpoll& operator=(const MessagePumpEpoll&) = delete;
  ~MessagePumpEpoll() override;

  // Initializes features for this class. See `base::features::Init()`.
  static void InitializeFeatures();

  // Starts watching `fd` for events as prescribed by `mode` (see
  // WatchableIOMessagePumpPosix). When an event occurs, `watcher` is notified.
  //
  // If `persistent` is false, the watch only persists until a matching event
  // is observed, and `watcher` will only see at most one event; otherwise it
  // remains active until explicitly cancelled and `watcher` may see multiple
  // events over time.
  //
  // The watch can be cancelled at any time by destroying the `controller` or
  // explicitly calling StopWatchingFileDescriptor() on it.
  //
  // IMPORTANT: `fd` MUST remain open as long as controller is alive and not
  // stopped. If `fd` is closed while the watch is still active, this will
  // result in memory bugs.
  bool WatchFileDescriptor(int fd,
                           bool persistent,
                           int mode,
                           FdWatchController* controller,
                           FdWatcher* watcher);

  // MessagePump methods:
  void Run(Delegate* delegate) override;
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;

 private:
  friend class MessagePumpEpollTest;

  // The WatchFileDescriptor API supports multiple FdWatchControllers watching
  // the same file descriptor, potentially for different events; but the epoll
  // API only supports a single interest list entry per unique file descriptor.
  //
  // EpollEventEntry tracks all epoll state relevant to a single file
  // descriptor, including references to all active and inactive Interests
  // concerned with that descriptor. This is used to derive a single aggregate
  // interest entry for the descriptor when manipulating epoll.
  struct EpollEventEntry {
    explicit EpollEventEntry(int fd);
    EpollEventEntry(const EpollEventEntry&) = delete;
    EpollEventEntry& operator=(const EpollEventEntry&) = delete;
    ~EpollEventEntry();

    static EpollEventEntry& FromEpollEvent(epoll_event& e) {
      return *static_cast<EpollEventEntry*>(e.data.ptr);
    }

    // Returns the combined set of epoll event flags which should be monitored
    // by the epoll instance for `fd`. This is based on a combination of the
    // parameters of all currently active elements in `interests`. Namely:
    //   - EPOLLIN is set if any active Interest wants to `read`.
    //   - EPOLLOUT is set if any active Interest wants to `write`.
    //   - EPOLLONESHOT is set if all active Interests are one-shot.
    uint32_t ComputeActiveEvents() const;

    // The file descriptor to which this entry pertains.
    const int fd;

    // A cached copy of the last known epoll event bits registered for this
    // descriptor on the epoll instance.
    uint32_t registered_events = 0;

    // A collection of all the interests regarding `fd` on this message pump.
    // The small amount of inline storage avoids heap allocation in virtually
    // all real scenarios, since there's little practical value in having more
    // than two controllers (e.g. one reader and one writer) watch the same
    // descriptor on the same thread.
    absl::InlinedVector<scoped_refptr<Interest>, 2> interests;

    // Temporary pointer to an active epoll_event structure which refers to
    // this entry. This is set immediately upon returning from epoll_wait() and
    // cleared again immediately before dispatching to any registered interests,
    // so long as this entry isn't destroyed in the interim.
    raw_ptr<epoll_event> active_event = nullptr;

    // If the file descriptor is disconnected and no active `interests`, remove
    // it from the epoll interest list to avoid unconditionally epoll_wait
    // return, and prevent any future update on this `EpollEventEntry`.
    bool stopped = false;

#if DCHECK_IS_ON()
    struct EpollHistory {
      base::debug::StackTrace stack_trace;
      std::optional<epoll_event> event;
    };
    static constexpr ssize_t kEpollHistoryWindowSize = 5;
    std::deque<EpollHistory> epoll_history_;

    void PushEpollHistory(std::optional<epoll_event> event) {
      EpollHistory info = {.stack_trace = base::debug::StackTrace(),
                           .event = event};
      epoll_history_.push_back(info);
      if (epoll_history_.size() > kEpollHistoryWindowSize) {
        epoll_history_.pop_front();
      }
    }
#endif
  };

  // State which lives on the stack within Run(), to support nested run loops.
  struct RunState {
    explicit RunState(Delegate* delegate) : delegate(delegate) {}

    // RAW_PTR_EXCLUSION: Performance reasons (based on analysis of sampling
    // profiler data and tab_search:top100:2020).
    RAW_PTR_EXCLUSION Delegate* const delegate;

    // Used to flag that the current Run() invocation should return ASAP.
    bool should_quit = false;
  };

  void AddEpollEvent(EpollEventEntry& entry);
  void UpdateEpollEvent(EpollEventEntry& entry);
  void StopEpollEvent(EpollEventEntry& entry);
  void UnregisterInterest(const scoped_refptr<Interest>& interest);
  bool WaitForEpollEvents(TimeDelta timeout);
  bool GetEventsPoll(int epoll_timeout, std::vector<epoll_event>* epoll_events);
  void OnEpollEvent(EpollEventEntry& entry, uint32_t events);
  void HandleEvent(int fd,
                   bool can_read,
                   bool can_write,
                   FdWatchController* controller);
  void HandleWakeUp();

  void BeginNativeWorkBatch();
  void RecordPeriodicMetrics();

  std::vector<struct pollfd>::iterator FindPollEntry(int fd);
  void RemovePollEntry(int fd);

  // Null if Run() is not currently executing. Otherwise it's a pointer into the
  // stack of the innermost nested Run() invocation.
  raw_ptr<RunState> run_state_ = nullptr;

  // This flag is set when starting to process native work; reset after every
  // `DoWork()` call. See crbug.com/1500295.
  bool native_work_started_ = false;

  // Mapping of all file descriptors currently watched by this message pump.
  // std::map was chosen because (1) the number of elements can vary widely,
  // (2) we don't do frequent lookups, and (3) values need stable addresses
  // across insertion or removal of other elements.
  std::map<int, EpollEventEntry> entries_;

  // pollfd array passed to poll() when not using epoll.
  std::vector<struct pollfd> pollfds_;

  // The epoll instance used by this message pump to monitor file descriptors.
  ScopedFD epoll_;

  // An eventfd object used to wake the pump's thread when scheduling new work.
  ScopedFD wake_event_;

  // Tracks when we should next record periodic metrics.
  base::TimeTicks next_metrics_time_;

  // WatchFileDescriptor() must be called from this thread, and so must
  // FdWatchController::StopWatchingFileDescriptor().
  THREAD_CHECKER(thread_checker_);

  WeakPtrFactory<MessagePumpEpoll> weak_ptr_factory_{this};
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_EPOLL_H_
