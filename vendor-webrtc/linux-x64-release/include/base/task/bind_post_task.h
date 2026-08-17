// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_BIND_POST_TASK_H_
#define BASE_TASK_BIND_POST_TASK_H_

#include <memory>
#include <type_traits>
#include <utility>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/task/bind_post_task_internal.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/task_runner.h"

// BindPostTask() is a helper function for binding a OnceCallback or
// RepeatingCallback to a task runner. BindPostTask(task_runner, callback)
// returns a task runner bound callback with an identical type to |callback|.
// The returned callback will take the same arguments as the input |callback|.
// Invoking Run() on the returned callback will post a task to run |callback| on
// target |task_runner| with the provided arguments.
//
// This is typically used when a callback must be invoked on a specific task
// runner but is provided as a result callback to a function that runs
// asynchronously on a different task runner.
//
// Example:
//    // |result_cb| can only be safely run on |my_task_runner|.
//    auto result_cb = BindOnce(&Foo::ReceiveReply, foo);
//    // Note that even if |returned_cb| is never run |result_cb| will attempt
//    // to be destroyed on |my_task_runner|.
//    auto returned_cb = BindPostTask(my_task_runner, std::move(result_cb));
//    // RunAsyncTask() will run the provided callback upon completion.
//    other_task_runner->PostTask(FROM_HERE,
//                                BindOnce(&RunAsyncTask,
//                                         std::move(returned_cb)));
//
// If the example provided |result_cb| to RunAsyncTask() then |result_cb| would
// run unsafely on |other_task_runner|. Instead RunAsyncTask() will run
// |returned_cb| which will post a task to |current_task_runner| before running
// |result_cb| safely.
//
// An alternative approach in the example above is to change RunAsyncTask() to
// also take a task runner as an argument and have RunAsyncTask() post the task.
// For cases where that isn't desirable BindPostTask() provides a convenient
// alternative.
//
// The input |callback| will always attempt to be destroyed on the target task
// runner. Even if the returned callback is never invoked, a task will be posted
// to destroy the input |callback|. However, if the target task runner has
// shutdown this is no longer possible. PostTask() will return false and the
// callback will be destroyed immediately on the current thread.
//
// The input |callback| must have a void return type to be compatible with
// PostTask(). If you want to drop the callback return value then use
// base::IgnoreResult(&Func) when creating the input |callback|.

namespace base {

// Creates a OnceCallback that will run |callback| on |task_runner|. If the
// returned callback is destroyed without being run then |callback| will be
// be destroyed on |task_runner|.
template <typename ReturnType, typename... Args>
  requires std::is_void_v<ReturnType>
OnceCallback<void(Args...)> BindPostTask(
    scoped_refptr<TaskRunner> task_runner,
    OnceCallback<ReturnType(Args...)> callback,
    const Location& location = FROM_HERE) {
  using Helper = internal::BindPostTaskTrampoline<OnceCallback<void(Args...)>>;

  return base::BindOnce(
      &Helper::template Run<Args...>,
      std::make_unique<Helper>(std::move(task_runner), location,
                               std::move(callback)));
}

// Creates a RepeatingCallback that will run |callback| on |task_runner|. When
// the returned callback is destroyed a task will be posted to destroy the input
// |callback| on |task_runner|.
template <typename ReturnType, typename... Args>
  requires std::is_void_v<ReturnType>
RepeatingCallback<void(Args...)> BindPostTask(
    scoped_refptr<TaskRunner> task_runner,
    RepeatingCallback<ReturnType(Args...)> callback,
    const Location& location = FROM_HERE) {
  using Helper =
      internal::BindPostTaskTrampoline<RepeatingCallback<void(Args...)>>;

  return base::BindRepeating(
      &Helper::template Run<Args...>,
      std::make_unique<Helper>(std::move(task_runner), location,
                               std::move(callback)));
}

// Creates a OnceCallback or RepeatingCallback that will run the `callback` on
// the default SequencedTaskRunner for the current sequence, i.e.
// `SequencedTaskRunner::GetCurrentDefault()`.
// Notes:
// - Please prefer using `base::SequenceBound<T>` if applicable.
// - Please consider using `base::PostTaskAndReplyWithResult()` instead where
// appropriate.
// - Please consider using an explicit task runner.
// - Only use this helper as a last resort if none of the above apply.

template <typename ReturnType, typename... Args>
  requires std::is_void_v<ReturnType>
OnceCallback<void(Args...)> BindPostTaskToCurrentDefault(
    OnceCallback<ReturnType(Args...)> callback,
    const Location& location = FROM_HERE) {
  return BindPostTask(SequencedTaskRunner::GetCurrentDefault(),
                      std::move(callback), location);
}

template <typename ReturnType, typename... Args>
  requires std::is_void_v<ReturnType>
RepeatingCallback<void(Args...)> BindPostTaskToCurrentDefault(
    RepeatingCallback<ReturnType(Args...)> callback,
    const Location& location = FROM_HERE) {
  return BindPostTask(SequencedTaskRunner::GetCurrentDefault(),
                      std::move(callback), location);
}

}  // namespace base

#endif  // BASE_TASK_BIND_POST_TASK_H_
