// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines a common helper encapsulating the tricky bits of
// implementing PostTaskAndReply(). These tricky bits are specifically around
// handling of the reply callback and ensuring it is deleted on the correct
// sequence.

#ifndef BASE_THREADING_POST_TASK_AND_REPLY_IMPL_H_
#define BASE_THREADING_POST_TASK_AND_REPLY_IMPL_H_

#include <utility>

#include "base/base_export.h"
#include "base/check_op.h"
#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/sequenced_task_runner.h"

namespace base::internal {

class BASE_EXPORT PostTaskAndReplyRelay {
 public:
  PostTaskAndReplyRelay(const Location& from_here,
                        OnceClosure task,
                        OnceClosure reply,
                        scoped_refptr<SequencedTaskRunner> reply_task_runner);

  PostTaskAndReplyRelay(PostTaskAndReplyRelay&&);
  // No assignment operator because of const member.
  PostTaskAndReplyRelay& operator=(PostTaskAndReplyRelay&&) = delete;

  PostTaskAndReplyRelay(const PostTaskAndReplyRelay&) = delete;
  PostTaskAndReplyRelay& operator=(const PostTaskAndReplyRelay&) = delete;

  ~PostTaskAndReplyRelay();

  // Static function is used because it is not possible to bind a method call to
  // a non-pointer type.
  static void RunTaskAndPostReply(PostTaskAndReplyRelay relay) {
    DCHECK(relay.task_);
    std::move(relay.task_).Run();

    // Keep a pointer to the reply TaskRunner for the PostTask() call before
    // |relay| is moved into a callback.
    SequencedTaskRunner* reply_task_runner_raw = relay.reply_task_runner_.get();

    const Location from_here = relay.from_here_;
    reply_task_runner_raw->PostTask(
        from_here,
        BindOnce(&PostTaskAndReplyRelay::RunReply, std::move(relay)));
  }

 private:
  // Static function is used because it is not possible to bind a method call to
  // a non-pointer type.
  static void RunReply(PostTaskAndReplyRelay relay) {
    DCHECK(!relay.task_);
    DCHECK(relay.reply_);
    std::move(relay.reply_).Run();
  }

  const Location from_here_;
  OnceClosure task_;
  OnceClosure reply_;
  // Not const to allow moving.
  scoped_refptr<SequencedTaskRunner> reply_task_runner_;
};

// Precondition: `SequencedTaskRunner::HasCurrentDefault()` must be true.
//
// Posts `task` by calling `task_poster`. On completion, posts `reply` to the
// origin sequence. Each callback is deleted synchronously after running, or
// scheduled for asynchronous deletion on the origin sequence if it can't run
// (e.g. if a TaskRunner skips it on shutdown). In this case, the callback may
// leak: see `SequencedTaskRunner::DeleteSoon()` for details about when objects
// scheduled for asynchronous deletion can be leaked.
//
// Note: All //base task posting APIs require callbacks to support deletion on
// the posting sequence if they can't be scheduled.
template <typename TaskPoster>
bool PostTaskAndReplyImpl(TaskPoster task_poster,
                          const Location& from_here,
                          OnceClosure task,
                          OnceClosure reply) {
  DCHECK(task) << from_here.ToString();
  DCHECK(reply) << from_here.ToString();

  const bool has_sequenced_context = SequencedTaskRunner::HasCurrentDefault();

  const bool post_task_success = task_poster(
      from_here, BindOnce(&PostTaskAndReplyRelay::RunTaskAndPostReply,
                          PostTaskAndReplyRelay(
                              from_here, std::move(task), std::move(reply),
                              has_sequenced_context
                                  ? SequencedTaskRunner::GetCurrentDefault()
                                  : nullptr)));

  // PostTaskAndReply() requires a SequencedTaskRunner::CurrentDefaultHandle to
  // post the reply.  Having no SequencedTaskRunner::CurrentDefaultHandle is
  // allowed when posting the task fails, to simplify calls during shutdown
  // (https://crbug.com/922938).
  CHECK(has_sequenced_context || !post_task_success);

  return post_task_success;
}

}  // namespace base::internal

#endif  // BASE_THREADING_POST_TASK_AND_REPLY_IMPL_H_
