// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_CONCURRENT_CALLBACKS_H_
#define BASE_FUNCTIONAL_CONCURRENT_CALLBACKS_H_

#include <memory>
#include <type_traits>
#include <vector>

#include "base/functional/bind.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/bind_post_task.h"
#include "base/task/sequenced_task_runner.h"

// OVERVIEW:
//
// ConcurrentCallbacks<T> is an alternative to BarrierCallback<T>, it dispenses
// OnceCallbacks via CreateCallback() and invokes the callback passed to Done()
// after all prior callbacks have been run.
//
// ConcurrentCallbacks<T> is intended to be used over BarrierCallback<T> in
// cases where the count is unknown prior to requiring a callback to start a
// task, and for cases where the count is manually derived from the code and
// subject to human error.
//
// IMPORTANT NOTES:
//
// - ConcurrentCallbacks<T> is NOT thread safe.
// - The done callback will NOT be run synchronously, it will be PostTask() to
//   the sequence that Done() was invoked on.
// - ConcurrentCallbacks<T> cannot be used after Done() is called, a CHECK
//   verifies this.
//
// TYPICAL USAGE:
//
// class Example {
//   void OnRequestsReceived(std::vector<Request> requests) {
//     base::ConcurrentCallbacks<Result> concurrent;
//
//     for (Request& request : requests) {
//       if (IsValidRequest(request)) {
//         StartRequest(std::move(request), concurrent.CreateCallback());
//       }
//     }
//
//     std::move(concurrent).Done(
//         base::BindOnce(&Example::OnRequestsComplete, GetWeakPtr()));
//   }
//
//   void StartRequest(Request request,
//                     base::OnceCallback<void(Result)> callback) {
//     // Process the request asynchronously and call callback with a Result.
//   }
//
//   void OnRequestsComplete(std::vector<Result> results) {
//     // Invoked after all requests are completed and receives the results of
//     // all of them.
//   }
// };

namespace base {

template <typename T>
class ConcurrentCallbacks {
 public:
  using Results = std::vector<std::remove_cvref_t<T>>;

  ConcurrentCallbacks() {
    auto info_owner = std::make_unique<Info>();
    info_ = info_owner.get();
    info_run_callback_ = BindRepeating(&Info::Run, std::move(info_owner));
  }

  // Create a callback for the done callback to wait for.
  [[nodiscard]] OnceCallback<void(T)> CreateCallback() {
    CHECK(info_);
    DCHECK_CALLED_ON_VALID_SEQUENCE(info_->sequence_checker_);
    ++info_->pending_;
    return info_run_callback_;
  }

  // Finish creating concurrent callbacks and provide done callback to run once
  // all prior callbacks have executed.
  // `this` is no longer usable after calling Done(), must be called with
  // std::move().
  void Done(OnceCallback<void(Results)> done_callback,
            const Location& location = FROM_HERE) && {
    CHECK(info_);
    DCHECK_CALLED_ON_VALID_SEQUENCE(info_->sequence_checker_);
    info_->done_callback_ =
        BindPostTask(SequencedTaskRunner::GetCurrentDefault(),
                     std::move(done_callback), location);
    if (info_->pending_ == 0u) {
      std::move(info_->done_callback_).Run(std::move(info_->results_));
    }
    info_ = nullptr;
  }

 private:
  class Info {
   public:
    Info() = default;

    void Run(T value) {
      DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
      CHECK_GT(pending_, 0u);
      --pending_;
      results_.push_back(std::move(value));
      if (done_callback_ && pending_ == 0u) {
        std::move(done_callback_).Run(std::move(results_));
      }
    }

    size_t pending_ GUARDED_BY_CONTEXT(sequence_checker_) = 0u;
    Results results_ GUARDED_BY_CONTEXT(sequence_checker_);
    OnceCallback<void(Results)> done_callback_
        GUARDED_BY_CONTEXT(sequence_checker_);
    SEQUENCE_CHECKER(sequence_checker_);
  };

  RepeatingCallback<void(T)> info_run_callback_;
  // info_ is owned by info_run_callback_.
  raw_ptr<Info> info_;
};

}  // namespace base

#endif  // BASE_FUNCTIONAL_CONCURRENT_CALLBACKS_H_
