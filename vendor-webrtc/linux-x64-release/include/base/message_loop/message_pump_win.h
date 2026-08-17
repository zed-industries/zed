// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_WIN_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_WIN_H_

#include <atomic>
#include <memory>
#include <optional>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/observer_list.h"
#include "base/synchronization/waitable_event.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/win/message_window.h"
#include "base/win/scoped_handle.h"
#include "base/win/windows_types.h"

namespace base {

// MessagePumpWin serves as the base for specialized versions of the MessagePump
// for Windows. It provides basic functionality like handling of observers and
// controlling the lifetime of the message pump.
class BASE_EXPORT MessagePumpWin : public MessagePump {
 public:
  MessagePumpWin();
  ~MessagePumpWin() override;

  // MessagePump methods:
  void Run(Delegate* delegate) override;
  void Quit() override;

  static void InitializeFeatures();

 protected:
  struct RunState {
    explicit RunState(Delegate* delegate_in) : delegate(delegate_in) {}

    const raw_ptr<Delegate> delegate;

    // Used to flag that the current Run() invocation should return ASAP.
    bool should_quit = false;

    // Set to true if this Run() is nested within another Run().
    bool is_nested = false;
  };

  virtual void DoRunLoop() = 0;

  // True iff:
  //   * MessagePumpForUI: there's a kMsgDoWork message pending in the Windows
  //     Message queue. i.e. when:
  //      a. The pump is about to wakeup from idle and kUIPumpImprovementsWin
  //         is not enabled.
  //      b. The pump is about to enter a nested native loop and a
  //         `ScopedAllowApplicationTasksInNativeNestedLoop` was instantiated to
  //         allow application tasks to execute in that nested loop
  //         (`ScopedAllowApplicationTasksInNativeNestedLoop` invokes
  //         ScheduleWork()).
  //      c. While in a native (nested) loop : HandleWorkMessage() =>
  //         ProcessPumpReplacementMessage() invokes ScheduleWork() before
  //         processing a native message to guarantee this pump will get another
  //         time slice if it goes into native Windows code and enters a native
  //         nested loop. This is different from (b.) because we're not yet
  //         processing an application task at the current run level and
  //         therefore are expected to keep pumping application tasks without
  //         necessitating a `ScopedAllowApplicationTasksInNativeNestedLoop`.
  //
  //   * MessagePumpforIO: there's a dummy IO completion item with `this` as an
  //     lpCompletionKey in the queue which is about to wakeup
  //     WaitForIOCompletion(). MessagePumpForIO doesn't support nesting so
  //     this is simpler than MessagePumpForUI.
  //
  // Note that this should not be used for memory ordering. It is accessed via
  // `memory_order_relaxed` in all cases.
  std::atomic_bool native_msg_scheduled_{false};

  raw_ptr<RunState> run_state_ = nullptr;

  THREAD_CHECKER(bound_thread_);
};

//-----------------------------------------------------------------------------
// MessagePumpForUI extends MessagePumpWin with methods that are particular to a
// MessageLoop instantiated with TYPE_UI.
//
// MessagePumpForUI implements a "traditional" Windows message pump. It contains
// a nearly infinite loop that peeks out messages, and then dispatches them.
// Intermixed with those peeks are callouts to DoWork. When there are no
// events to be serviced, this pump goes into a wait state. In most cases, this
// message pump handles all processing.
//
// However, when a task, or windows event, invokes on the stack a native dialog
// box or such, that window typically provides a bare bones (native?) message
// pump.  That bare-bones message pump generally supports little more than a
// peek of the Windows message queue, followed by a dispatch of the peeked
// message.  MessageLoop extends that bare-bones message pump to also service
// Tasks, at the cost of some complexity.
//
// The basic structure of the extension (referred to as a sub-pump) is that a
// special message, kMsgHaveWork, is repeatedly injected into the Windows
// Message queue.  Each time the kMsgHaveWork message is peeked, checks are made
// for an extended set of events, including the availability of Tasks to run.
//
// After running a task, the special message kMsgHaveWork is again posted to the
// Windows Message queue, ensuring a future time slice for processing a future
// event.  To prevent flooding the Windows Message queue, care is taken to be
// sure that at most one kMsgHaveWork message is EVER pending in the Window's
// Message queue.
//
// There are a few additional complexities in this system where, when there are
// no Tasks to run, this otherwise infinite stream of messages which drives the
// sub-pump is halted.  The pump is automatically re-started when Tasks are
// queued.
//
// A second complexity is that the presence of this stream of posted tasks may
// prevent a bare-bones message pump from ever peeking a WM_PAINT or WM_TIMER.
// Such paint and timer events always give priority to a posted message, such as
// kMsgHaveWork messages.  As a result, care is taken to do some peeking in
// between the posting of each kMsgHaveWork message (i.e., after kMsgHaveWork is
// peeked, and before a replacement kMsgHaveWork is posted).
//
// NOTE: Although it may seem odd that messages are used to start and stop this
// flow (as opposed to signaling objects, etc.), it should be understood that
// the native message pump will *only* respond to messages.  As a result, it is
// an excellent choice.  It is also helpful that the starter messages that are
// placed in the queue when new task arrive also awakens DoRunLoop.
//
class BASE_EXPORT MessagePumpForUI : public MessagePumpWin {
 public:
  MessagePumpForUI();
  ~MessagePumpForUI() override;

  // MessagePump methods:
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;
  bool HandleNestedNativeLoopWithApplicationTasks(
      bool application_tasks_desired) override;

  // An observer interface to give the scheduler an opportunity to log
  // information about MSGs before and after they are dispatched.
  class BASE_EXPORT Observer {
   public:
    virtual void WillDispatchMSG(const MSG& msg) = 0;
    virtual void DidDispatchMSG(const MSG& msg) = 0;
  };

  void AddObserver(Observer* observer);
  void RemoveObserver(Observer* obseerver);

 private:
  bool MessageCallback(UINT message,
                       WPARAM wparam,
                       LPARAM lparam,
                       LRESULT* result);
  void DoRunLoop() override;
  NOINLINE NOT_TAIL_CALLED void WaitForWork(
      Delegate::NextWorkInfo next_work_info);
  void HandleWorkMessage();
  void HandleTimerMessage();
  void ScheduleNativeTimer(Delegate::NextWorkInfo next_work_info);
  void KillNativeTimer();
  bool ProcessNextWindowsMessage();
  bool ProcessMessageHelper(const MSG& msg);
  bool ProcessPumpReplacementMessage();

  base::win::MessageWindow message_window_;

  // Non-nullopt if there's currently a native timer installed. If so, it
  // indicates when the timer is set to fire and can be used to avoid setting
  // redundant timers.
  std::optional<TimeTicks> installed_native_timer_;

  // This is used to wake up the pump when the UIPumpImprovementsWin experiment
  // is enabled.
  WaitableEvent event_{WaitableEvent::ResetPolicy::AUTOMATIC};

  // This is set when HandleNestedNativeLoopWithApplicationTasks(true) was
  // called (when a `ScopedAllowApplicationTasksInNativeNestedLoop` is
  // instantiated).
  //
  // When running with `event_`, switches to pumping
  // `kMsgHaveWork` MSGs when there are application tasks to be done during
  // native runloops. In this state, ScheduleDelayedWork() will start a native
  // timer.
  //
  // It is reset when:
  //   - DoRunLoop() gets control back after ProcessNextWindowsMessage().
  //   - HandleNestedNativeLoopWithApplicationTasks(false) is called.
  bool in_nested_native_loop_with_application_tasks_ = false;

  enum class WakeupState {
    kApplicationTask,
    kNative,
    kRunning,
    kInactive,
  };
  // Used to keep track of what the pump knows about the state of its work
  // sources at wakeup for the experiment 'UIPumpImprovementsWin'. Its value is
  // `kInactive` at construction, but set to `kRunning` on entry to DoRunLoop().
  WakeupState wakeup_state_ = WakeupState::kInactive;

  ObserverList<Observer>::Unchecked observers_;
};

//-----------------------------------------------------------------------------
// MessagePumpForIO extends MessagePumpWin with methods that are particular to a
// MessageLoop instantiated with TYPE_IO. This version of MessagePump does not
// deal with Windows mesagges, and instead has a Run loop based on Completion
// Ports so it is better suited for IO operations.
//
class BASE_EXPORT MessagePumpForIO : public MessagePumpWin {
 public:
  class BASE_EXPORT IOContext {
   public:
    IOContext();
    ~IOContext();

    OVERLAPPED* GetOverlapped();

   private:
    // Hack: This header needs to be pulled in by files that should not
    // `#include <windows.h>`, yet wants to store an `OVERLAPPED` inline.
    // We can't simply define `OVERLAPPED` ourselves, or the compiler will
    // complain about type redefinitions in files that _do_ see the real
    // definition. Instead, define an identical struct, but use it only to
    // align/size storage that we will construct a real `OVERLAPPED` in in the
    // constructor.
    struct Sizer {
      ULONG_PTR Internal;
      ULONG_PTR InternalHigh;
      union {
        struct {
          DWORD Offset;
          DWORD OffsetHigh;
        } DUMMYSTRUCTNAME;
        PVOID Pointer;
      } DUMMYUNIONNAME;
      HANDLE hEvent;
    };

    alignas(Sizer) unsigned char storage_[sizeof(Sizer)];
  };

  // Clients interested in receiving OS notifications when asynchronous IO
  // operations complete should implement this interface and register themselves
  // with the message pump.
  //
  // Typical use #1:
  //   class MyFile : public IOHandler {
  //     MyFile() : IOHandler(FROM_HERE) {
  //       ...
  //       message_pump->RegisterIOHandler(file_, this);
  //     }
  //     // Plus some code to make sure that this destructor is not called
  //     // while there are pending IO operations.
  //     ~MyFile() {
  //     }
  //     virtual void OnIOCompleted(IOContext* context, DWORD bytes_transfered,
  //                                DWORD error) {
  //       ...
  //       delete context;
  //     }
  //     void DoSomeIo() {
  //       ...
  //       IOContext* context = new IOContext;
  //       ReadFile(file_, buffer, num_bytes, &read, &context);
  //     }
  //     HANDLE file_;
  //   };
  //
  // Typical use #2:
  // Same as the previous example, except that in order to deal with the
  // requirement stated for the destructor, the class calls WaitForIOCompletion
  // from the destructor to block until all IO finishes.
  //     ~MyFile() {
  //       while(pending_)
  //         message_pump->WaitForIOCompletion(INFINITE, this);
  //     }
  //
  class BASE_EXPORT IOHandler {
   public:
    explicit IOHandler(const Location& from_here);
    virtual ~IOHandler();

    IOHandler(const IOHandler&) = delete;
    IOHandler& operator=(const IOHandler&) = delete;

    // This will be called once the pending IO operation associated with
    // |context| completes. |error| is the Win32 error code of the IO operation
    // (ERROR_SUCCESS if there was no error). |bytes_transfered| will be zero
    // on error.
    virtual void OnIOCompleted(IOContext* context,
                               DWORD bytes_transfered,
                               DWORD error) = 0;

    const Location& io_handler_location() { return io_handler_location_; }

   private:
    const Location io_handler_location_;
  };

  MessagePumpForIO();
  ~MessagePumpForIO() override;

  // MessagePump methods:
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;

  // Register the handler to be used when asynchronous IO for the given file
  // completes. The registration persists as long as |file_handle| is valid, so
  // |handler| must be valid as long as there is pending IO for the given file.
  // Returns true iff the registration succeeds.
  [[nodiscard]] bool RegisterIOHandler(HANDLE file_handle, IOHandler* handler);

  // Register the handler to be used to process job events. The registration
  // persists as long as the job object is live, so |handler| must be valid
  // until the job object is destroyed. Returns true if the registration
  // succeeded, and false otherwise.
  bool RegisterJobObject(HANDLE job_handle, IOHandler* handler);

 private:
  struct IOItem {
    raw_ptr<IOHandler> handler;
    raw_ptr<IOContext> context;
    DWORD bytes_transfered;
    DWORD error;
  };

  void DoRunLoop() override;
  NOINLINE NOT_TAIL_CALLED void WaitForWork(
      Delegate::NextWorkInfo next_work_info);
  bool GetIOItem(DWORD timeout, IOItem* item);
  bool ProcessInternalIOItem(const IOItem& item);
  // Waits for the next IO completion for up to |timeout| milliseconds.
  // Return true if any IO operation completed, and false if the timeout
  // expired. If the completion port received any messages, the associated
  // handlers will have been invoked before returning from this code.
  bool WaitForIOCompletion(DWORD timeout);

  // The completion port associated with this thread.
  win::ScopedHandle port_;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_WIN_H_
