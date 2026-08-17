// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_CURRENT_THREAD_H_
#define BASE_TASK_CURRENT_THREAD_H_

#include <ostream>
#include <type_traits>

#include "base/base_export.h"
#include "base/callback_list.h"
#include "base/check.h"
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/message_loop/ios_cronet_buildflags.h"
#include "base/message_loop/message_pump_for_io.h"
#include "base/message_loop/message_pump_for_ui.h"
#include "base/pending_task.h"
#include "base/task/sequence_manager/task_time_observer.h"
#include "base/task/single_thread_task_runner.h"
#include "base/task/task_observer.h"
#include "build/build_config.h"

namespace autofill {
class NextIdleBarrier;
}

namespace content {
class BrowserMainLoop;
}

namespace web {
class WebTaskEnvironment;
}

namespace base {

namespace test {
bool RunUntil(FunctionRef<bool(void)>);
void TestPredicateOrRegisterOnNextIdleCallback(base::FunctionRef<bool(void)>,
                                               CallbackListSubscription*,
                                               OnceClosure);
}  // namespace test

namespace sequence_manager {
namespace internal {
class SequenceManagerImpl;
}
}  // namespace sequence_manager

class IOWatcher;

// CurrentThread is a proxy to a subset of Task related APIs bound to the
// current thread
//
// Current(UI|IO)Thread is available statically through
// Current(UI|IO)Thread::Get() on threads that have registered as CurrentThread
// on this physical thread (e.g. by using SingleThreadTaskExecutor). APIs
// intended for all consumers on the thread should be on Current(UI|IO)Thread,
// while internal APIs might be on multiple internal classes (e.g.
// SequenceManager).
//
// Why: Historically MessageLoop would take care of everything related to event
// processing on a given thread. Nowadays that functionality is split among
// different classes. At that time MessageLoop::current() gave access to the
// full MessageLoop API, preventing both addition of powerful owner-only APIs as
// well as making it harder to remove callers of deprecated APIs (that need to
// stick around for a few owner-only use cases and re-accrue callers after
// cleanup per remaining publicly available).
//
// As such, many methods below are flagged as deprecated and should be removed
// once all static callers have been migrated.
class BASE_EXPORT CurrentThread {
 public:
  // CurrentThread is effectively just a disguised pointer and is fine to
  // copy/move around.
  CurrentThread(const CurrentThread& other) = default;
  CurrentThread(CurrentThread&& other) = default;
  CurrentThread& operator=(const CurrentThread& other) = default;

  friend bool operator==(const CurrentThread&, const CurrentThread&) = default;

  // Returns a proxy object to interact with the Task related APIs for the
  // current thread. It must only be used on the thread it was obtained.
  static CurrentThread Get();

  // Return an empty CurrentThread. No methods should be called on this
  // object.
  static CurrentThread GetNull();

  // Returns true if the current thread is registered to expose CurrentThread
  // API. Prefer this to verifying the boolean value of Get() (so that Get() can
  // ultimately DCHECK it's only invoked when IsSet()).
  static bool IsSet();

  // Allow CurrentThread to be used like a pointer to support the many
  // callsites that used MessageLoop::current() that way when it was a
  // MessageLoop*.
  CurrentThread* operator->() { return this; }
  explicit operator bool() const { return !!current_; }

  // A DestructionObserver is notified when the current task execution
  // environment is being destroyed. These observers are notified prior to
  // CurrentThread::IsSet() being changed to return false. This gives interested
  // parties the chance to do final cleanup.
  //
  // NOTE: Any tasks posted to the current thread during this notification will
  // not be run. Instead, they will be deleted.
  //
  // Deprecation note: Prefer SequenceLocalStorageSlot<std::unique_ptr<Foo>> to
  // DestructionObserver to bind an object's lifetime to the current
  // thread/sequence.
  class BASE_EXPORT DestructionObserver {
   public:
    // TODO(crbug.com/40596446): Rename to
    // WillDestroyCurrentTaskExecutionEnvironment
    virtual void WillDestroyCurrentMessageLoop() = 0;

   protected:
    virtual ~DestructionObserver() = default;
  };

  // Add a DestructionObserver, which will start receiving notifications
  // immediately.
  void AddDestructionObserver(DestructionObserver* destruction_observer);

  // Remove a DestructionObserver.  It is safe to call this method while a
  // DestructionObserver is receiving a notification callback.
  void RemoveDestructionObserver(DestructionObserver* destruction_observer);

  // Forwards to SequenceManager::SetTaskRunner().
  // DEPRECATED(https://crbug.com/825327): only owners of the SequenceManager
  // instance should replace its TaskRunner.
  void SetTaskRunner(scoped_refptr<SingleThreadTaskRunner> task_runner);

  // Forwards to SequenceManager::(Add|Remove)TaskObserver.
  // DEPRECATED(https://crbug.com/825327): only owners of the SequenceManager
  // instance should add task observers on it.
  void AddTaskObserver(TaskObserver* task_observer);
  void RemoveTaskObserver(TaskObserver* task_observer);

  // When this functionality is enabled, the queue time will be recorded for
  // posted tasks.
  void SetAddQueueTimeToTasks(bool enable);

  // Registers a `OnceClosure` to be called on this thread the next time it goes
  // idle. This is meant for internal usage; callers should use BEST_EFFORT
  // tasks instead of this for generic work that needs to wait until quiescence
  // to run.
  class RegisterOnNextIdleCallbackPasskey {
   private:
    RegisterOnNextIdleCallbackPasskey() = default;

    friend autofill::NextIdleBarrier;
    friend content::BrowserMainLoop;
    friend bool test::RunUntil(FunctionRef<bool(void)>);
    friend void test::TestPredicateOrRegisterOnNextIdleCallback(
        base::FunctionRef<bool(void)>,
        CallbackListSubscription*,
        OnceClosure);
  };
  [[nodiscard]] CallbackListSubscription RegisterOnNextIdleCallback(
      RegisterOnNextIdleCallbackPasskey,
      OnceClosure on_next_idle_callback);

  // Enables nested task processing in scope of an upcoming native message loop.
  // Some unwanted message loops may occur when using common controls or printer
  // functions. Hence, nested task processing is disabled by default to avoid
  // unplanned reentrancy. This re-enables it in cases where the stack is
  // reentrancy safe and processing nestable tasks is explicitly safe.
  //
  // For instance,
  // - The current thread is running a message loop.
  // - It receives a task #1 and executes it.
  // - The task #1 implicitly starts a nested message loop, like a MessageBox in
  //   the unit test. This can also be StartDoc or GetSaveFileName.
  // - The thread receives a task #2 before or while in this second message
  //   loop.
  // - With NestableTasksAllowed set to true, the task #2 will run right away.
  //   Otherwise, it will get executed right after task #1 completes at "thread
  //   message loop level".
  //
  // Use RunLoop::Type::kNestableTasksAllowed when nesting is triggered by the
  // application RunLoop rather than by native code.
  class BASE_EXPORT ScopedAllowApplicationTasksInNativeNestedLoop {
   public:
    ScopedAllowApplicationTasksInNativeNestedLoop();
    ~ScopedAllowApplicationTasksInNativeNestedLoop();

   private:
    const raw_ptr<sequence_manager::internal::SequenceManagerImpl>
        sequence_manager_;
    const bool previous_state_;
  };

  // Returns true if nestable tasks are allowed on the current thread at this
  // time (i.e. if a native nested loop would start from the callee's point in
  // the stack, would it be allowed to run application tasks).
  bool ApplicationTasksAllowedInNativeNestedLoop() const;

  // Returns true if this instance is bound to the current thread.
  bool IsBoundToCurrentThread() const;

  // Returns true if the current thread is idle (ignoring delayed tasks). This
  // is the same condition which triggers DoWork() to return false: i.e. out of
  // tasks which can be processed at the current run-level -- there might be
  // deferred non-nestable tasks remaining if currently in a nested run level.
  bool IsIdleForTesting();

  // Enables ThreadControllerWithMessagePumpImpl's TimeKeeper metrics.
  // `thread_name` will be used as a suffix.
  // Setting `wall_time_based_metrics_enabled_for_testing` adds wall-time
  // based metrics for this thread. This is only for test environments as it
  // disables subsampling.
  void EnableMessagePumpTimeKeeperMetrics(
      const char* thread_name,
      bool wall_time_based_metrics_enabled_for_testing = false);

  // Returns the IOWatcher instance exposed by this thread, if any.
  IOWatcher* GetIOWatcher();

 protected:
  explicit CurrentThread(
      sequence_manager::internal::SequenceManagerImpl* sequence_manager)
      : current_(sequence_manager) {}

  static sequence_manager::internal::SequenceManagerImpl*
  GetCurrentSequenceManagerImpl();

  friend class ScheduleWorkTest;
  friend class Thread;
  friend class sequence_manager::internal::SequenceManagerImpl;
  friend class MessageLoopTaskRunnerTest;
  friend class web::WebTaskEnvironment;

  raw_ptr<sequence_manager::internal::SequenceManagerImpl> current_;
};

#if !BUILDFLAG(IS_NACL)

// UI extension of CurrentThread.
class BASE_EXPORT CurrentUIThread : public CurrentThread {
 public:
  // Returns an interface for the CurrentUIThread of the current thread.
  // Asserts that IsSet().
  static CurrentUIThread Get();

  // Returns true if the current thread is running a CurrentUIThread.
  static bool IsSet();

  CurrentUIThread* operator->() { return this; }

#if BUILDFLAG(IS_OZONE) && !BUILDFLAG(IS_FUCHSIA) && !BUILDFLAG(IS_WIN)
  static_assert(
      std::is_base_of_v<WatchableIOMessagePumpPosix, MessagePumpForUI>,
      "CurrentThreadForUI::WatchFileDescriptor is supported only"
      "by MessagePumpEpoll and MessagePumpGlib implementations.");
  bool WatchFileDescriptor(int fd,
                           bool persistent,
                           MessagePumpForUI::Mode mode,
                           MessagePumpForUI::FdWatchController* controller,
                           MessagePumpForUI::FdWatcher* delegate);
#endif

#if BUILDFLAG(IS_IOS)
  // Forwards to SequenceManager::Attach().
  // TODO(crbug.com/40568517): Plumb the actual SequenceManager* to
  // callers and remove ability to access this method from
  // CurrentUIThread.
  void Attach();
#endif

#if BUILDFLAG(IS_ANDROID)
  // Forwards to MessagePumpAndroid::Abort().
  // TODO(crbug.com/40568517): Plumb the actual MessagePumpForUI* to
  // callers and remove ability to access this method from
  // CurrentUIThread.
  void Abort();
#endif

#if BUILDFLAG(IS_WIN)
  void AddMessagePumpObserver(MessagePumpForUI::Observer* observer);
  void RemoveMessagePumpObserver(MessagePumpForUI::Observer* observer);
#endif

 private:
  explicit CurrentUIThread(
      sequence_manager::internal::SequenceManagerImpl* current)
      : CurrentThread(current) {}

  MessagePumpForUI* GetMessagePumpForUI() const;
};

#endif  // !BUILDFLAG(IS_NACL)

// ForIO extension of CurrentThread.
class BASE_EXPORT CurrentIOThread : public CurrentThread {
 public:
  // Returns an interface for the CurrentIOThread of the current thread.
  // Asserts that IsSet().
  static CurrentIOThread Get();

  // Returns true if the current thread is running a CurrentIOThread.
  static bool IsSet();

  CurrentIOThread* operator->() { return this; }

#if !BUILDFLAG(IS_NACL)

#if BUILDFLAG(IS_WIN)
  // Please see MessagePumpWin for definitions of these methods.
  [[nodiscard]] bool RegisterIOHandler(HANDLE file,
                                       MessagePumpForIO::IOHandler* handler);
  bool RegisterJobObject(HANDLE job, MessagePumpForIO::IOHandler* handler);
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  // Please see WatchableIOMessagePumpPosix for definition.
  // Prefer base::FileDescriptorWatcher for non-critical IO.
  bool WatchFileDescriptor(int fd,
                           bool persistent,
                           MessagePumpForIO::Mode mode,
                           MessagePumpForIO::FdWatchController* controller,
                           MessagePumpForIO::FdWatcher* delegate);
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_MAC) || \
    (BUILDFLAG(IS_IOS) && !BUILDFLAG(CRONET_BUILD) && !BUILDFLAG(IS_IOS_TVOS))
  bool WatchMachReceivePort(
      mach_port_t port,
      MessagePumpForIO::MachPortWatchController* controller,
      MessagePumpForIO::MachPortWatcher* delegate);
#endif

#if BUILDFLAG(IS_FUCHSIA)
  // Additional watch API for native platform resources.
  bool WatchZxHandle(zx_handle_t handle,
                     bool persistent,
                     zx_signals_t signals,
                     MessagePumpForIO::ZxHandleWatchController* controller,
                     MessagePumpForIO::ZxHandleWatcher* delegate);
#endif  // BUILDFLAG(IS_FUCHSIA)

#endif  // !BUILDFLAG(IS_NACL)

 private:
  explicit CurrentIOThread(
      sequence_manager::internal::SequenceManagerImpl* current)
      : CurrentThread(current) {}

  MessagePumpForIO* GetMessagePumpForIO() const;
};

}  // namespace base

#endif  // BASE_TASK_CURRENT_THREAD_H_
