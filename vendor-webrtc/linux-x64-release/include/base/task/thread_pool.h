// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_H_
#define BASE_TASK_THREAD_POOL_H_

#include <memory>
#include <utility>

#include "base/base_export.h"
#include "base/files/file_path.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/location.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/post_task_and_reply_with_result_internal.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/task/single_thread_task_runner_thread_mode.h"
#include "base/task/task_runner.h"
#include "base/task/task_traits.h"
#include "base/task/updateable_sequenced_task_runner.h"
#include "base/time/time.h"
#include "build/build_config.h"

namespace base {

// This is the interface to post tasks to base's thread pool.
//
// To post a simple one-off task with default traits:
//     base::ThreadPool::PostTask(FROM_HERE, base::BindOnce(...));
//
// To post a high priority one-off task to respond to a user interaction:
//     base::ThreadPool::PostTask(
//         FROM_HERE,
//         {base::TaskPriority::USER_BLOCKING},
//         base::BindOnce(...));
//
// To post tasks that must run in sequence with default traits:
//     scoped_refptr<SequencedTaskRunner> task_runner =
//         base::ThreadPool::CreateSequencedTaskRunner();
//     task_runner->PostTask(FROM_HERE, base::BindOnce(...));
//     task_runner->PostTask(FROM_HERE, base::BindOnce(...));
//
// To post tasks that may block, must run in sequence and can be skipped on
// shutdown:
//     scoped_refptr<SequencedTaskRunner> task_runner =
//         base::ThreadPool::CreateSequencedTaskRunner(
//             {MayBlock(), TaskShutdownBehavior::SKIP_ON_SHUTDOWN});
//     task_runner->PostTask(FROM_HERE, base::BindOnce(...));
//     task_runner->PostTask(FROM_HERE, base::BindOnce(...));
//
// The default traits apply to tasks that:
//     (1) don't block (ref. MayBlock() and WithBaseSyncPrimitives()),
//     (2) prefer inheriting the current priority to specifying their own, and
//     (3) can either block shutdown or be skipped on shutdown
//         (implementation is free to choose a fitting default).
// Explicit traits must be specified for tasks for which these loose
// requirements are not sufficient.
//
// Prerequisite: A ThreadPoolInstance must have been registered for the current
// process via ThreadPoolInstance::Set() before the API below can be invoked.
// This is typically done during the initialization phase in each process. If
// your code is not running in that phase, you most likely don't have to worry
// about this. You will encounter DCHECKs or nullptr dereferences if this is
// violated. For tests, use base::test::TaskEnvironment.
class BASE_EXPORT ThreadPool {
 public:
  // base::ThreadPool is a static API. See base::ThreadPoolInstance for the
  // actual instance.
  ThreadPool() = delete;

  // Equivalent to calling PostTask with default TaskTraits.
  static bool PostTask(const Location& from_here, OnceClosure task);
  inline static bool PostTask(OnceClosure task,
                              const Location& from_here = Location::Current()) {
    return PostTask(from_here, std::move(task));
  }

  // Equivalent to calling PostDelayedTask with default TaskTraits.
  //
  // Use PostDelayedTask to specify a BEST_EFFORT priority if the task doesn't
  // have to run as soon as |delay| expires.
  static bool PostDelayedTask(const Location& from_here,
                              OnceClosure task,
                              TimeDelta delay);

  // Equivalent to calling PostTaskAndReply with default TaskTraits.
  static bool PostTaskAndReply(const Location& from_here,
                               OnceClosure task,
                               OnceClosure reply);

  // Equivalent to calling PostTaskAndReplyWithResult with default TaskTraits.
  //
  // Though RepeatingCallback is convertible to OnceCallback, we need a
  // CallbackType template since we can not use template deduction and object
  // conversion at once on the overload resolution.
  // TODO(crbug.com/40516732): Update all callers of the RepeatingCallback
  // version to use OnceCallback and remove the CallbackType template.
  template <template <typename> class CallbackType,
            typename TaskReturnType,
            typename ReplyArgType>
    requires(IsBaseCallback<CallbackType<void()>>)
  static bool PostTaskAndReplyWithResult(
      const Location& from_here,
      CallbackType<TaskReturnType()> task,
      CallbackType<void(ReplyArgType)> reply) {
    return ThreadPool::PostTaskAndReplyWithResult(
        from_here, {}, std::move(task), std::move(reply));
  }

  // Posts |task| with specific |traits|. Returns false if the task definitely
  // won't run because of current shutdown state.
  static bool PostTask(const Location& from_here,
                       const TaskTraits& traits,
                       OnceClosure task);

  // Posts |task| with specific |traits|. |task| will not run before |delay|
  // expires. Returns false if the task definitely won't run because of current
  // shutdown state.
  //
  // Specify a BEST_EFFORT priority via |traits| if the task doesn't have to run
  // as soon as |delay| expires.
  static bool PostDelayedTask(const Location& from_here,
                              const TaskTraits& traits,
                              OnceClosure task,
                              TimeDelta delay);

  // Posts |task| with specific |traits| and posts |reply| on the caller's
  // execution context (i.e. same sequence or thread and same TaskTraits if
  // applicable) when |task| completes. Returns false if the task definitely
  // won't run because of current shutdown state. Can only be called when
  // SequencedTaskRunner::HasCurrentDefault().
  static bool PostTaskAndReply(const Location& from_here,
                               const TaskTraits& traits,
                               OnceClosure task,
                               OnceClosure reply);

  // Posts |task| with specific |traits| and posts |reply| with the return value
  // of |task| as argument on the caller's execution context (i.e. same sequence
  // or thread and same TaskTraits if applicable) when |task| completes. Returns
  // false if the task definitely won't run because of current shutdown state.
  // Can only be called when SequencedTaskRunner::HasCurrentDefault().
  //
  // Though RepeatingCallback is convertible to OnceCallback, we need a
  // CallbackType template since we can not use template deduction and object
  // conversion at once on the overload resolution.
  // TODO(crbug.com/40516732): Update all callers of the RepeatingCallback
  // version to use OnceCallback and remove the CallbackType template.
  template <template <typename> class CallbackType,
            typename TaskReturnType,
            typename ReplyArgType>
    requires(IsBaseCallback<CallbackType<void()>>)
  static bool PostTaskAndReplyWithResult(
      const Location& from_here,
      const TaskTraits& traits,
      CallbackType<TaskReturnType()> task,
      CallbackType<void(ReplyArgType)> reply) {
    auto* result = new std::unique_ptr<TaskReturnType>();
    return PostTaskAndReply(
        from_here, traits,
        BindOnce(&internal::ReturnAsParamAdapter<TaskReturnType>,
                 std::move(task), result),
        BindOnce(&internal::ReplyAdapter<TaskReturnType, ReplyArgType>,
                 std::move(reply), Owned(result)));
  }

  // Returns a TaskRunner whose PostTask invocations result in scheduling tasks
  // using |traits|. Tasks may run in any order and in parallel.
  static scoped_refptr<TaskRunner> CreateTaskRunner(const TaskTraits& traits);

  // Returns a SequencedTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits|. Tasks run one at a time in posting order.
  static scoped_refptr<SequencedTaskRunner> CreateSequencedTaskRunner(
      const TaskTraits& traits);

  // Returns a task runner whose PostTask invocations result in scheduling tasks
  // using |traits|. The priority in |traits| can be updated at any time via
  // UpdateableSequencedTaskRunner::UpdatePriority(). An update affects all
  // tasks posted to the task runner that aren't running yet. Tasks run one at a
  // time in posting order.
  //
  // |traits| requirements:
  // - base::ThreadPolicy must be specified if the priority of the task runner
  //   will ever be increased from BEST_EFFORT.
  static scoped_refptr<UpdateableSequencedTaskRunner>
  CreateUpdateableSequencedTaskRunner(const TaskTraits& traits);

  // Returns a SingleThreadTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits| on a thread determined by |thread_mode|.
  // See base/task/single_thread_task_runner_thread_mode.h for |thread_mode|
  // details. If |traits| identifies an existing thread,
  // SingleThreadTaskRunnerThreadMode::SHARED must be used. Tasks run on a
  // single thread in posting order.
  //
  // If all you need is to make sure that tasks don't run concurrently (e.g.
  // because they access a data structure which is not thread-safe), use
  // CreateSequencedTaskRunner(). Only use this if you rely on a thread-affine
  // API (it might be safer to assume thread-affinity when dealing with
  // under-documented third-party APIs, e.g. other OS') or share data across
  // tasks using thread-local storage.
  static scoped_refptr<SingleThreadTaskRunner> CreateSingleThreadTaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode =
          SingleThreadTaskRunnerThreadMode::SHARED);

#if BUILDFLAG(IS_WIN)
  // Returns a SingleThreadTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits| in a COM Single-Threaded Apartment on a
  // thread determined by |thread_mode|. See
  // base/task/single_thread_task_runner_thread_mode.h for |thread_mode|
  // details. If |traits| identifies an existing thread,
  // SingleThreadTaskRunnerThreadMode::SHARED must be used. Tasks run in the
  // same Single-Threaded Apartment in posting order for the returned
  // SingleThreadTaskRunner. There is not necessarily a one-to-one
  // correspondence between SingleThreadTaskRunners and Single-Threaded
  // Apartments. The implementation is free to share apartments or create new
  // apartments as necessary. In either case, care should be taken to make sure
  // COM pointers are not smuggled across apartments.
  static scoped_refptr<SingleThreadTaskRunner> CreateCOMSTATaskRunner(
      const TaskTraits& traits,
      SingleThreadTaskRunnerThreadMode thread_mode =
          SingleThreadTaskRunnerThreadMode::SHARED);
#endif  // BUILDFLAG(IS_WIN)

  // Returns a SequencedTaskRunner whose PostTask invocations result in
  // scheduling tasks using |traits|. Tasks run one at a time in posting order.
  // Returns the existing `SequenceTaskRunner` for 'path', or creates it.
  // Ensures tasks accessing the same `path` are sequenced, even if posted from
  // `SequencedTaskRunner`s obtained in different contexts. The same `traits`
  // must be provided to all calls with the same `path`.
  static scoped_refptr<SequencedTaskRunner>
  CreateSequencedTaskRunnerForResource(const TaskTraits& traits,
                                       const base::FilePath& path);
};

}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_H_
