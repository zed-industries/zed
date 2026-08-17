// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_TASK_TRAITS_H_
#define BASE_TASK_TASK_TRAITS_H_

#include <stdint.h>

#include <iosfwd>
#include <type_traits>
#include <utility>

#include "base/base_export.h"
#include "base/check.h"
#include "base/check_op.h"
#include "base/traits_bag.h"
#include "build/build_config.h"

namespace base {

// Valid priorities supported by the task scheduling infrastructure.
//
// Note: internal algorithms depend on priorities being expressed as a
// continuous zero-based list from lowest to highest priority. Users of this API
// shouldn't otherwise care about nor use the underlying values.
//
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base.task
enum class TaskPriority : uint8_t {
  // This will always be equal to the lowest priority available.
  LOWEST = 0,
  // Best effort tasks will only start running when machine resources are
  // available. The application may preempt best effort tasks if it expects that
  // resources will soon be needed by work of higher priority. Dependending on
  // the ThreadPolicy, best effort tasks may run on a thread that is likely to
  // be descheduled when higher priority work arrives (in this process or
  // another).
  //
  // Examples:
  // - Reporting metrics.
  // - Persisting data to disk.
  // - Loading data that is required for a potential future user interaction
  //   (Note: Use CreateUpdateableSequencedTaskRunner() to increase the priority
  //   when that user interactions happens).
  BEST_EFFORT = LOWEST,

  // The result of user visible tasks is visible to the user (in the UI or as a
  // side-effect on the system) but it is not an immediate response to a user
  // interaction.
  //
  // Examples:
  // - Updating the UI to reflect progress on a long task.
  // - Downloading a file requested by the user.
  // - Loading an image that is displayed in the UI but is non-critical.
  USER_VISIBLE,

  // User blocking tasks affects UI immediately after a user interaction.
  //
  // Example:
  // - Loading and rendering a web page after the user clicks a link.
  // - Sorting suggestions after the user types a character in the omnibox.
  //
  // This is the default TaskPriority in order for tasks to run in order by
  // default and avoid unintended consequences. The only way to get a task to
  // run at a higher priority than USER_BLOCKING is to coordinate with a
  // higher-level scheduler (contact scheduler-dev@chromium.org for such use
  // cases).
  USER_BLOCKING,

  // This will always be equal to the highest priority available.
  HIGHEST = USER_BLOCKING
};

// Valid shutdown behaviors supported by the thread pool.
enum class TaskShutdownBehavior : uint8_t {
  // Tasks posted with this mode which have not started executing before
  // shutdown is initiated will never run. Tasks with this mode running at
  // shutdown will be ignored (the worker will not be joined).
  //
  // This option provides a nice way to post stuff you don't want blocking
  // shutdown. For example, you might be doing a slow DNS lookup and if it's
  // blocked on the OS, you may not want to stop shutdown, since the result
  // doesn't really matter at that point.
  //
  // However, you need to be very careful what you do in your callback when you
  // use this option. Since the thread will continue to run until the OS
  // terminates the process, the app can be in the process of tearing down when
  // you're running. This means any singletons or global objects you use may
  // suddenly become invalid out from under you. For this reason, it's best to
  // use this only for slow but simple operations like the DNS example.
  CONTINUE_ON_SHUTDOWN,

  // Tasks posted with this mode that have not started executing at
  // shutdown will never run. However, any task that has already begun
  // executing when shutdown is invoked will be allowed to continue and
  // will block shutdown until completion.
  //
  // Note: Because ThreadPoolInstance::Shutdown() may block while these tasks
  // are executing, care must be taken to ensure that they do not block on the
  // thread that called ThreadPoolInstance::Shutdown(), as this may lead to
  // deadlock.
  SKIP_ON_SHUTDOWN,

  // Tasks posted with this mode before shutdown is complete will block shutdown
  // until they're executed. Generally, this should be used only to save
  // critical user data.
  //
  // Note 1: Delayed tasks cannot block shutdown. Delayed tasks posted as part
  // of a BLOCK_SHUTDOWN sequence will behave like SKIP_ON_SHUTDOWN tasks.
  //
  // Note 2: Background threads will be promoted to normal threads at shutdown
  // (i.e. TaskPriority::BEST_EFFORT + TaskShutdownBehavior::BLOCK_SHUTDOWN will
  // resolve without a priority inversion).
  BLOCK_SHUTDOWN,
};

// Determines at which thread priority a task may run.
//
// ThreadPolicy and priority updates
// ---------------------------------
//
//   If the TaskPriority of an UpdateableSequencedTaskRunner is increased while
//   one of its tasks is running at background thread priority, the task's
//   execution will have to complete at background thread priority (may take a
//   long time) before the next task can be scheduled with the new TaskPriority.
//   If it is important that priority increases take effect quickly,
//   MUST_USE_FOREGROUND should be used to prevent the tasks from running at
//   background thread priority. If it is important to minimize impact on the
//   rest on the system when the TaskPriority is BEST_EFFORT, PREFER_BACKGROUND
//   should be used.
//
// ThreadPolicy and priority inversions
// ------------------------------------
//
//   A priority inversion occurs when a task running at background thread
//   priority is descheduled while holding a resource needed by a thread of
//   higher priority. MUST_USE_FOREGROUND can be combined with BEST_EFFORT to
//   indicate that a task has a low priority, but shouldn't run at background
//   thread priority in order to avoid priority inversions. Please consult with
//   //base/task/OWNERS if you suspect a priority inversion.
enum class ThreadPolicy : uint8_t {
  // The task runs on a background priority thread if:
  // - The TaskPriority is BEST_EFFORT.
  // - Background thread priority is supported by the platform (see
  //   environment_config_unittest.cc).
  // - ThreadPoolInstance::Shutdown() hadn't been called when the task started
  // running.
  //       (Remaining TaskShutdownBehavior::BLOCK_SHUTDOWN tasks use foreground
  //        threads during shutdown regardless of TaskPriority)
  // Otherwise, it runs on a normal priority thread.
  // This is the default.
  PREFER_BACKGROUND,

  // The task runs at normal thread priority, irrespective of its TaskPriority.
  MUST_USE_FOREGROUND
};

// Tasks with this trait may block. This includes but is not limited to tasks
// that wait on synchronous file I/O operations: read or write a file from disk,
// interact with a pipe or a socket, rename or delete a file, enumerate files in
// a directory, etc. This trait isn't required for the mere use of locks. For
// tasks that block on base/ synchronization primitives, see the
// WithBaseSyncPrimitives trait.
struct MayBlock {};

// DEPRECATED. Use base::ScopedAllowBaseSyncPrimitives(ForTesting) instead.
//
// Tasks with this trait will pass base::AssertBaseSyncPrimitivesAllowed(), i.e.
// will be allowed on the following methods :
// - base::WaitableEvent::Wait
// - base::ConditionVariable::Wait
// - base::PlatformThread::Join
// - base::PlatformThread::Sleep
// - base::Process::WaitForExit
// - base::Process::WaitForExitWithTimeout
//
// Tasks should generally not use these methods.
//
// Instead of waiting on a WaitableEvent or a ConditionVariable, put the work
// that should happen after the wait in a callback and post that callback from
// where the WaitableEvent or ConditionVariable would have been signaled. If
// something needs to be scheduled after many tasks have executed, use
// base::BarrierClosure.
//
// On Windows, join processes asynchronously using base::win::ObjectWatcher.
//
// MayBlock() must be specified in conjunction with this trait if and only if
// removing usage of methods listed above in the labeled tasks would still
// result in tasks that may block (per MayBlock()'s definition).
//
// In doubt, consult with //base/task/OWNERS.
struct WithBaseSyncPrimitives {};

// Describes metadata for a single task or a group of tasks.
class BASE_EXPORT TaskTraits {
 public:
  // ValidTrait ensures TaskTraits' constructor only accepts appropriate types.
  struct ValidTrait {
    ValidTrait(TaskPriority);
    ValidTrait(TaskShutdownBehavior);
    ValidTrait(ThreadPolicy);
    ValidTrait(MayBlock);
    ValidTrait(WithBaseSyncPrimitives);
  };

  // Invoking this constructor without arguments produces default TaskTraits
  // that are appropriate for tasks that
  //     (1) don't block (ref. MayBlock() and WithBaseSyncPrimitives()),
  //     (2) pertain to user-blocking activity,
  //         (explicitly or implicitly by having an ordering dependency with a
  //          component that does)
  //     (3) can either block shutdown or be skipped on shutdown
  //         (the task recipient is free to choose a fitting default).
  //
  // To get TaskTraits for tasks that have more precise traits: provide any
  // combination of ValidTrait's as arguments to this constructor.
  //
  // Note: When posting to well-known threads (e.g. UI/IO), default traits are
  // almost always what you want unless you know for sure the task being posted
  // has no explicit/implicit ordering dependency with anything else running at
  // default (USER_BLOCKING) priority.
  //
  // E.g.
  // constexpr base::TaskTraits default_traits = {};
  // constexpr base::TaskTraits user_visible_traits = {
  //     base::TaskPriority::USER_VISIBLE};
  // constexpr base::TaskTraits user_visible_may_block_traits = {
  //     base::TaskPriority::USER_VISIBLE, base::MayBlock()
  // };
  // constexpr base::TaskTraits other_user_visible_may_block_traits = {
  //     base::MayBlock(), base::TaskPriority::USER_VISIBLE
  // };
  template <class... ArgTypes>
    requires trait_helpers::AreValidTraits<ValidTrait, ArgTypes...>
  // TaskTraits are intended to be implicitly-constructable (eg {}).
  // NOLINTNEXTLINE(google-explicit-constructor)
  constexpr TaskTraits(ArgTypes... args)
      : priority_(
            trait_helpers::GetEnum<TaskPriority, TaskPriority::USER_BLOCKING>(
                args...)),
        shutdown_behavior_(
            static_cast<uint8_t>(
                trait_helpers::GetEnum<TaskShutdownBehavior,
                                       TaskShutdownBehavior::SKIP_ON_SHUTDOWN>(
                    args...)) |
            (trait_helpers::HasTrait<TaskShutdownBehavior, ArgTypes...>()
                 ? kIsExplicitFlag
                 : 0)),
        thread_policy_(
            static_cast<uint8_t>(
                trait_helpers::GetEnum<ThreadPolicy,
                                       ThreadPolicy::PREFER_BACKGROUND>(
                    args...)) |
            (trait_helpers::HasTrait<ThreadPolicy, ArgTypes...>()
                 ? kIsExplicitFlag
                 : 0)),
        may_block_(trait_helpers::HasTrait<MayBlock, ArgTypes...>()),
        with_base_sync_primitives_(
            trait_helpers::HasTrait<WithBaseSyncPrimitives, ArgTypes...>()) {}

  constexpr TaskTraits(const TaskTraits& other) = default;
  TaskTraits& operator=(const TaskTraits& other) = default;

  friend bool operator==(const TaskTraits&, const TaskTraits&) = default;

  // Sets the priority of tasks with these traits to |priority|.
  void UpdatePriority(TaskPriority priority) { priority_ = priority; }

  // Returns the priority of tasks with these traits.
  constexpr TaskPriority priority() const { return priority_; }

  // Returns true if the shutdown behavior was set explicitly.
  constexpr bool shutdown_behavior_set_explicitly() const {
    return shutdown_behavior_ & kIsExplicitFlag;
  }

  // Returns the shutdown behavior of tasks with these traits.
  constexpr TaskShutdownBehavior shutdown_behavior() const {
    return static_cast<TaskShutdownBehavior>(shutdown_behavior_ &
                                             ~kIsExplicitFlag);
  }

  // Returns true if the thread policy was set explicitly.
  constexpr bool thread_policy_set_explicitly() const {
    return thread_policy_ & kIsExplicitFlag;
  }

  // Returns the thread policy of tasks with these traits.
  constexpr ThreadPolicy thread_policy() const {
    return static_cast<ThreadPolicy>(thread_policy_ & ~kIsExplicitFlag);
  }

  // Returns true if tasks with these traits may block.
  constexpr bool may_block() const { return may_block_; }

  // Returns true if tasks with these traits may use base/ sync primitives.
  constexpr bool with_base_sync_primitives() const {
    return with_base_sync_primitives_;
  }

 private:
  // This bit is set in |priority_|, |shutdown_behavior_| and |thread_policy_|
  // when the value was set explicitly.
  static constexpr uint8_t kIsExplicitFlag = 0x80;

  // Ordered for packing.
  TaskPriority priority_;
  uint8_t shutdown_behavior_;
  uint8_t thread_policy_;
  bool may_block_;
  bool with_base_sync_primitives_;
};

// Returns string literals for the enums defined in this file. These methods
// should only be used for tracing and debugging.
BASE_EXPORT const char* TaskPriorityToString(TaskPriority task_priority);
BASE_EXPORT const char* TaskShutdownBehaviorToString(
    TaskShutdownBehavior task_priority);

// Stream operators so that the enums defined in this file can be used in
// DCHECK and EXPECT statements.
BASE_EXPORT std::ostream& operator<<(std::ostream& os,
                                     const TaskPriority& shutdown_behavior);
BASE_EXPORT std::ostream& operator<<(
    std::ostream& os,
    const TaskShutdownBehavior& shutdown_behavior);

}  // namespace base

#endif  // BASE_TASK_TASK_TRAITS_H_
