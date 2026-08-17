// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_RUN_LOOP_H_
#define BASE_RUN_LOOP_H_

#include <stack>
#include <utility>
#include <vector>

#include "base/base_export.h"
#include "base/containers/stack.h"
#include "base/dcheck_is_on.h"
#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/observer_list.h"
#include "base/sequence_checker.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

namespace test {
class ScopedRunLoopTimeout;
class ScopedDisableRunLoopTimeout;
}  // namespace test

#if BUILDFLAG(IS_ANDROID)
class MessagePumpAndroid;
#endif

#if BUILDFLAG(IS_IOS)
class MessagePumpUIApplication;
#endif

class SingleThreadTaskRunner;

// Helper class to run the RunLoop::Delegate associated with the current thread.
// A RunLoop::Delegate must have been bound to this thread (ref.
// RunLoop::RegisterDelegateForCurrentThread()) prior to using any of RunLoop's
// member and static methods unless explicitly indicated otherwise (e.g.
// IsRunning/IsNestedOnCurrentThread()). RunLoop::Run can only be called once
// per RunLoop lifetime. Create a RunLoop on the stack and call Run/Quit to run
// a nested RunLoop but please avoid nested loops in production code!
class BASE_EXPORT RunLoop {
 public:
  // The type of RunLoop: a kDefault RunLoop at the top-level (non-nested) will
  // process system and application tasks assigned to its Delegate. When nested
  // however a kDefault RunLoop will only process system tasks while a
  // kNestableTasksAllowed RunLoop will continue to process application tasks
  // even if nested.
  //
  // This is relevant in the case of recursive RunLoops. Some unwanted run loops
  // may occur when using common controls or printer functions. By default,
  // recursive task processing is disabled.
  //
  // In general, nestable RunLoops are to be avoided. They are dangerous and
  // difficult to get right, so please use with extreme caution.
  //
  // A specific example where this makes a difference is:
  // - The thread is running a RunLoop.
  // - It receives a task #1 and executes it.
  // - The task #1 implicitly starts a RunLoop, like a MessageBox in the unit
  //   test. This can also be StartDoc or GetSaveFileName.
  // - The thread receives a task #2 before or while in this second RunLoop.
  // - With a kNestableTasksAllowed RunLoop, the task #2 will run right away.
  //   Otherwise, it will get executed right after task #1 completes in the main
  //   RunLoop.
  enum class Type {
    kDefault,
    kNestableTasksAllowed,
  };

  explicit RunLoop(Type type = Type::kDefault);
  RunLoop(const RunLoop&) = delete;
  RunLoop& operator=(const RunLoop&) = delete;
  ~RunLoop();

  // Run the current RunLoop::Delegate. This blocks until Quit is called
  // (directly or by running the RunLoop::QuitClosure).
  void Run(const Location& location = Location::Current());

  // Run the current RunLoop::Delegate until it doesn't find any tasks or
  // messages in its queue (it goes idle).
  // WARNING #1: This may run long (flakily timeout) and even never return! Do
  //             not use this when repeating tasks such as animated web pages
  //             are present.
  // WARNING #2: This may return too early! For example, if used to run until an
  //             incoming event has occurred but that event depends on a task in
  //             a different queue -- e.g. another TaskRunner or a system event.
  // Per the warnings above, this tends to lead to flaky tests; prefer
  // QuitClosure()+Run() when at all possible.
  void RunUntilIdle();

  bool running() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return running_;
  }

  // Quit() transitions this RunLoop to a state where no more tasks will be
  // allowed to run at the run-loop-level of this RunLoop. If invoked from the
  // owning thread, the effect is immediate; otherwise it is thread-safe but
  // asynchronous. When the transition takes effect, the underlying message loop
  // quits this run-loop-level if it is topmost (otherwise the desire to quit
  // this level is saved until run-levels nested above it are quit).
  //
  // QuitWhenIdle() results in this RunLoop returning true from
  // ShouldQuitWhenIdle() at this run-level (the delegate decides when "idle" is
  // reached). This is also thread-safe.
  //
  // There can be other nested RunLoops servicing the same task queue. As
  // mentioned above, quitting one RunLoop has no bearing on the others. Hence,
  // you may never assume that a call to Quit() will terminate the underlying
  // message loop. If a nested RunLoop continues running, the target may NEVER
  // terminate.
  void Quit();
  void QuitWhenIdle();

  // Returns a RepeatingClosure that safely calls Quit() or QuitWhenIdle() (has
  // no effect if the RunLoop instance is gone).
  //
  // The closures must be obtained from the thread owning the RunLoop but may
  // then be invoked from any thread.
  //
  // Returned closures may be safely:
  //   * Passed to other threads.
  //   * Run() from other threads, though this will quit the RunLoop
  //     asynchronously.
  //   * Run() after the RunLoop has stopped or been destroyed, in which case
  //     they are a no-op).
  //   * Run() before RunLoop::Run(), in which case RunLoop::Run() returns
  //     immediately."
  //
  // Example:
  //   RunLoop run_loop;
  //   DoFooAsyncAndNotify(run_loop.QuitClosure());
  //   run_loop.Run();
  //
  // Note that Quit() itself is thread-safe and may be invoked directly if you
  // have access to the RunLoop reference from another thread (e.g. from a
  // capturing lambda or test observer).
  [[nodiscard]] RepeatingClosure QuitClosure() &;
  [[nodiscard]] RepeatingClosure QuitWhenIdleClosure() &;

  // Returns true if Quit() or QuitWhenIdle() was called.
  bool AnyQuitCalled();

  // Returns true if there is an active RunLoop on this thread.
  // Safe to call before RegisterDelegateForCurrentThread().
  static bool IsRunningOnCurrentThread();

  // Returns true if there is an active RunLoop on this thread and it's nested
  // within another active RunLoop.
  // Safe to call before RegisterDelegateForCurrentThread().
  static bool IsNestedOnCurrentThread();

  // A NestingObserver is notified when a nested RunLoop begins and ends.
  class BASE_EXPORT NestingObserver {
   public:
    // Notified before a nested loop starts running work on the current thread.
    virtual void OnBeginNestedRunLoop() = 0;
    // Notified after a nested loop is done running work on the current thread.
    virtual void OnExitNestedRunLoop() {}

   protected:
    virtual ~NestingObserver() = default;
  };

  static void AddNestingObserverOnCurrentThread(NestingObserver* observer);
  static void RemoveNestingObserverOnCurrentThread(NestingObserver* observer);

  // A RunLoop::Delegate is a generic interface that allows RunLoop to be
  // separate from the underlying implementation of the message loop for this
  // thread. It holds private state used by RunLoops on its associated thread.
  // One and only one RunLoop::Delegate must be registered on a given thread
  // via RunLoop::RegisterDelegateForCurrentThread() before RunLoop instances
  // and RunLoop static methods can be used on it.
  class BASE_EXPORT Delegate {
   public:
    Delegate();
    Delegate(const Delegate&) = delete;
    Delegate& operator=(const Delegate&) = delete;
    virtual ~Delegate();

    // Used by RunLoop to inform its Delegate to Run/Quit. Implementations are
    // expected to keep on running synchronously from the Run() call until the
    // eventual matching Quit() call or a delay of |timeout| expires. Upon
    // receiving a Quit() call or timing out it should return from the Run()
    // call as soon as possible without executing remaining tasks/messages.
    // Run() calls can nest in which case each Quit() call should result in the
    // topmost active Run() call returning. The only other trigger for Run()
    // to return is the |should_quit_when_idle_callback_| which the Delegate
    // should probe before sleeping when it becomes idle.
    // |application_tasks_allowed| is true if this is the first Run() call on
    // the stack or it was made from a nested RunLoop of
    // Type::kNestableTasksAllowed (otherwise this Run() level should only
    // process system tasks).
    virtual void Run(bool application_tasks_allowed, TimeDelta timeout) = 0;
    virtual void Quit() = 0;

    // Invoked right before a RunLoop enters a nested Run() call on this
    // Delegate iff this RunLoop is of type kNestableTasksAllowed. The Delegate
    // should ensure that the upcoming Run() call will result in processing
    // application tasks queued ahead of it without further probing. e.g.
    // message pumps on some platforms, like Mac, need an explicit request to
    // process application tasks when nested, otherwise they'll only wait for
    // system messages.
    virtual void EnsureWorkScheduled() = 0;

   protected:
    // Returns the result of this Delegate's |should_quit_when_idle_callback_|.
    // "protected" so it can be invoked only by the Delegate itself. The
    // Delegate is expected to quit Run() if this returns true.
    bool ShouldQuitWhenIdle();

   private:
    // While the state is owned by the Delegate subclass, only RunLoop can use
    // it.
    friend class RunLoop;

    friend class ScopedDisallowRunningRunLoop;

    // A vector-based stack is more memory efficient than the default
    // deque-based stack as the active RunLoop stack isn't expected to ever
    // have more than a few entries.
    using RunLoopStack =
        stack<raw_ptr<RunLoop, VectorExperimental>,
              std::vector<raw_ptr<RunLoop, VectorExperimental>>>;

    RunLoopStack active_run_loops_;
    ObserverList<RunLoop::NestingObserver>::Unchecked nesting_observers_;

#if DCHECK_IS_ON()
    bool allow_running_for_testing_ = true;
#endif

    // True once this Delegate is bound to a thread via
    // RegisterDelegateForCurrentThread().
    bool bound_ = false;

    // Thread-affine per its use of TLS.
    THREAD_CHECKER(bound_thread_checker_);
  };

  // Registers |delegate| on the current thread. Must be called once and only
  // once per thread before using RunLoop methods on it. |delegate| is from then
  // on forever bound to that thread (including its destruction).
  static void RegisterDelegateForCurrentThread(Delegate* new_delegate);

  // Support for //base/test/scoped_run_loop_timeout.h.
  // This must be public for access by the implementation code in run_loop.cc.
  struct BASE_EXPORT RunLoopTimeout {
    RunLoopTimeout();
    ~RunLoopTimeout();
    TimeDelta timeout;
    RepeatingCallback<void(const Location&)> on_timeout;
  };

 private:
  FRIEND_TEST_ALL_PREFIXES(SingleThreadTaskExecutorTypedTest,
                           RunLoopQuitOrderAfter);

#if BUILDFLAG(IS_ANDROID)
  // Android doesn't support the blocking RunLoop::Run, so it calls
  // BeforeRun and AfterRun directly.
  friend class MessagePumpAndroid;
#endif

#if BUILDFLAG(IS_IOS)
  // iOS doesn't support the blocking RunLoop::Run, so it calls
  // BeforeRun directly.
  friend class MessagePumpUIApplication;
#endif

  // Support for //base/test/scoped_run_loop_timeout.h.
  friend class test::ScopedRunLoopTimeout;
  friend class test::ScopedDisableRunLoopTimeout;

  static void SetTimeoutForCurrentThread(const RunLoopTimeout* timeout);
  static const RunLoopTimeout* GetTimeoutForCurrentThread();

  // Return false to abort the Run.
  bool BeforeRun();
  void AfterRun();

  // A cached reference of RunLoop::Delegate for the thread driven by this
  // RunLoop for quick access without using TLS (also allows access to state
  // from another sequence during Run(), ref. |sequence_checker_| below).
  const raw_ptr<Delegate, DanglingUntriaged> delegate_;

  const Type type_;

#if DCHECK_IS_ON()
  bool run_allowed_ = true;
#endif

  bool quit_called_ = false;
  bool running_ = false;

  // Used to record that QuitWhenIdle() was called on this RunLoop.
  bool quit_when_idle_called_ = false;
  // Whether the Delegate should quit Run() once it becomes idle (it's
  // responsible for probing this state via ShouldQuitWhenIdle()). This state is
  // stored here rather than pushed to Delegate to support nested RunLoops.
  bool quit_when_idle_ = false;

  // RunLoop is not thread-safe. Its state/methods, unless marked as such, may
  // not be accessed from any other sequence than the thread it was constructed
  // on. Exception: RunLoop can be safely accessed from one other sequence (or
  // single parallel task) during Run() -- e.g. to Quit() without having to
  // plumb SingleThreadTaskRunner::GetCurrentDefault() throughout a test to
  // repost QuitClosure to origin thread.
  SEQUENCE_CHECKER(sequence_checker_);

  const scoped_refptr<SingleThreadTaskRunner> origin_task_runner_;

  // WeakPtrFactory for QuitClosure safety.
  WeakPtrFactory<RunLoop> weak_factory_{this};
};

// RunLoop::Run() will DCHECK if called while there's a
// ScopedDisallowRunningRunLoop in scope on its thread. This is useful to add
// safety to some test constructs which allow multiple task runners to share the
// main thread in unit tests. While the main thread can be shared by multiple
// runners to deterministically fake multi threading, there can still only be a
// single RunLoop::Delegate per thread and RunLoop::Run() should only be invoked
// from it (or it would result in incorrectly driving TaskRunner A while in
// TaskRunner B's context).
class BASE_EXPORT [[maybe_unused, nodiscard]] ScopedDisallowRunningRunLoop {
 public:
  ScopedDisallowRunningRunLoop();
  ScopedDisallowRunningRunLoop(const ScopedDisallowRunningRunLoop&) = delete;
  ScopedDisallowRunningRunLoop& operator=(const ScopedDisallowRunningRunLoop&) =
      delete;
  ~ScopedDisallowRunningRunLoop();

 private:
#if DCHECK_IS_ON()
  raw_ptr<RunLoop::Delegate> current_delegate_;
  const bool previous_run_allowance_;
#endif  // DCHECK_IS_ON()
};

}  // namespace base

#endif  // BASE_RUN_LOOP_H_
