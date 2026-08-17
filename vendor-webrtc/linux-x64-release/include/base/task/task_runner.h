// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_TASK_RUNNER_H_
#define BASE_TASK_TASK_RUNNER_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/check.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/functional/callback_helpers.h"
#include "base/location.h"
#include "base/memory/ref_counted.h"
#include "base/task/post_task_and_reply_with_result_internal.h"

namespace base {

struct TaskRunnerTraits;
class TimeDelta;

// A TaskRunner is an object that runs posted tasks (in the form of
// OnceClosure objects).  The TaskRunner interface provides a way of
// decoupling task posting from the mechanics of how each task will be
// run.  TaskRunner provides very weak guarantees as to how posted
// tasks are run (or if they're run at all).  In particular, it only
// guarantees:
//
//   - Posting a task will not run it synchronously.  That is, no
//     Post*Task method will call task.Run() directly.
//
//   - Increasing the delay can only delay when the task gets run.
//     That is, increasing the delay may not affect when the task gets
//     run, or it could make it run later than it normally would, but
//     it won't make it run earlier than it normally would.
//
// TaskRunner does not guarantee the order in which posted tasks are
// run, whether tasks overlap, or whether they're run on a particular
// thread.  Also it does not guarantee a memory model for shared data
// between tasks.  (In other words, you should use your own
// synchronization/locking primitives if you need to share data
// between tasks.)
//
// Implementations of TaskRunner should be thread-safe in that all
// methods must be safe to call on any thread.  Ownership semantics
// for TaskRunners are in general not clear, which is why the
// interface itself is RefCountedThreadSafe.
//
// Some theoretical implementations of TaskRunner:
//
//   - A TaskRunner that uses a thread pool to run posted tasks.
//
//   - A TaskRunner that, for each task, spawns a non-joinable thread
//     to run that task and immediately quit.
//
//   - A TaskRunner that stores the list of posted tasks and has a
//     method Run() that runs each runnable task in random order.
class BASE_EXPORT TaskRunner
    : public RefCountedThreadSafe<TaskRunner, TaskRunnerTraits> {
 public:
  // Posts the given task to be run.  Returns true if the task may be
  // run at some point in the future, and false if the task definitely
  // will not be run.
  //
  // Equivalent to PostDelayedTask(from_here, task, 0).
  bool PostTask(const Location& from_here, OnceClosure task);

  // Like PostTask, but tries to run the posted task only after |delay_ms|
  // has passed. Implementations should use a tick clock, rather than wall-
  // clock time, to implement |delay|.
  virtual bool PostDelayedTask(const Location& from_here,
                               OnceClosure task,
                               base::TimeDelta delay) = 0;

  // Posts |task| on the current TaskRunner.  On completion, |reply| is posted
  // to the sequence that called PostTaskAndReply().  On the success case,
  // |task| is destroyed on the target sequence and |reply| is destroyed on the
  // originating sequence immediately after their invocation.  If an error
  // happened on the onward PostTask, both |task| and |reply| are destroyed on
  // the originating sequence, and on an error on the backward PostTask, |reply|
  // is leaked rather than being destroyed on the wrong sequence.  This allows
  // objects that must be deleted on the originating sequence to be bound into
  // the |reply| Closures.  In particular, it can be useful to use WeakPtr<> in
  // the |reply| Closure so that the reply operation can be canceled. See the
  // following pseudo-code:
  //
  // class DataBuffer : public RefCountedThreadSafe<DataBuffer> {
  //  public:
  //   // Called to add data into a buffer.
  //   void AddData(void* buf, size_t length);
  //   ...
  // };
  //
  //
  // class DataLoader {
  //  public:
  //    void GetData() {
  //      scoped_refptr<DataBuffer> buffer = new DataBuffer();
  //      target_thread_.task_runner()->PostTaskAndReply(
  //          FROM_HERE,
  //          base::BindOnce(&DataBuffer::AddData, buffer),
  //          base::BindOnce(&DataLoader::OnDataReceived,
  //                             weak_ptr_factory_.GetWeakPtr(), buffer));
  //    }
  //
  //  private:
  //    void OnDataReceived(scoped_refptr<DataBuffer> buffer) {
  //      // Do something with buffer.
  //    }
  //    base::WeakPtrFactory<DataLoader> weak_ptr_factory_{this};
  // };
  //
  //
  // Things to notice:
  //   * Results of |task| are shared with |reply| by binding a shared argument
  //     (a DataBuffer instance).
  //   * The DataLoader object has no special thread safety.
  //   * The DataLoader object can be deleted while |task| is still running,
  //     and the reply will cancel itself safely because it is bound to a
  //     WeakPtr<>.
  bool PostTaskAndReply(const Location& from_here,
                        OnceClosure task,
                        OnceClosure reply);

  // When you have these methods
  //
  //   R DoWorkAndReturn();
  //   void Callback(const R& result);
  //
  // and want to call them in a PostTaskAndReply kind of fashion where the
  // result of DoWorkAndReturn is passed to the Callback, you can use
  // PostTaskAndReplyWithResult as in this example:
  //
  // PostTaskAndReplyWithResult(
  //     target_thread_.task_runner(),
  //     FROM_HERE,
  //     BindOnce(&DoWorkAndReturn),
  //     BindOnce(&Callback));
  //
  // Templating on the types of `task` and `reply` allows template matching to
  // work for both base::RepeatingCallback and base::OnceCallback in each case.
  template <typename TaskReturnType,
            typename ReplyArgType,
            template <typename>
            class TaskCallbackType,
            template <typename>
            class ReplyCallbackType>
    requires(IsBaseCallback<TaskCallbackType<void()>> &&
             IsBaseCallback<ReplyCallbackType<void()>>)
  bool PostTaskAndReplyWithResult(const Location& from_here,
                                  TaskCallbackType<TaskReturnType()> task,
                                  ReplyCallbackType<void(ReplyArgType)> reply) {
    DCHECK(task);
    DCHECK(reply);
    // std::unique_ptr used to avoid the need of a default constructor.
    auto* result = new std::unique_ptr<TaskReturnType>();
    return PostTaskAndReply(
        from_here,
        BindOnce(&internal::ReturnAsParamAdapter<TaskReturnType>,
                 std::move(task), result),
        BindOnce(&internal::ReplyAdapter<TaskReturnType, ReplyArgType>,
                 std::move(reply), Owned(result)));
  }

 protected:
  friend struct TaskRunnerTraits;

  TaskRunner();
  virtual ~TaskRunner();

  // Called when this object should be destroyed.  By default simply
  // deletes |this|, but can be overridden to do something else, like
  // delete on a certain thread.
  virtual void OnDestruct() const;
};

struct BASE_EXPORT TaskRunnerTraits {
  static void Destruct(const TaskRunner* task_runner);
};

}  // namespace base

#endif  // BASE_TASK_TASK_RUNNER_H_
