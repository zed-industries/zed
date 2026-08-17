// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_CONCURRENT_CLOSURES_H_
#define BASE_FUNCTIONAL_CONCURRENT_CLOSURES_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/memory/raw_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/bind_post_task.h"

namespace base {

// OVERVIEW:
//
// ConcurrentClosures is a OnceClosure version of ConcurrentCallbacks<T> and an
// alternative to BarrierClosure, it dispenses OnceClosures via CreateClosure()
// and invokes the closure passed to Done() after all prior closures have been
// run.
//
// ConcurrentClosures is intended to be used over BarrierClosure in
// cases where the count is unknown prior to requiring a closure to start a
// task, and for cases where the count is manually derived from the code and
// subject to human error.
//
// IMPORTANT NOTES:
//
// - ConcurrentClosures is NOT thread safe.
// - The done closure will NOT be run synchronously, it will be PostTask() to
//   the sequence that Done() was invoked on.
// - ConcurrentClosures cannot be used after Done() is called, a CHECK verifies
//   this.
//
// TYPICAL USAGE:
//
// void DoABC(OnceClosure closure) {
//   base::ConcurrentClosures concurrent;
//
//   DoA(concurrent.CreateClosure());
//   DoB(concurrent.CreateClosure());
//   DoC(concurrent.CreateClosure());
//
//   std::move(concurrent).Done(closure);
// }

class BASE_EXPORT ConcurrentClosures {
 public:
  ConcurrentClosures();
  ~ConcurrentClosures();

  // Create a closure for the done closure to wait for.
  [[nodiscard]] OnceClosure CreateClosure();

  // Finish creating concurrent closures and provide done closure to run once
  // all prior closures have executed.
  // `this` is no longer usable after calling Done(), must be called with
  // std::move().
  void Done(OnceClosure done_closure, const Location& location = FROM_HERE) &&;

 private:
  class Info {
   public:
    Info();
    ~Info();

    void Run();

    size_t pending_ GUARDED_BY_CONTEXT(sequence_checker_) = 0u;
    OnceClosure done_closure_ GUARDED_BY_CONTEXT(sequence_checker_);
    SEQUENCE_CHECKER(sequence_checker_);
  };

  RepeatingClosure info_run_closure_;
  // info_ is owned by info_run_closure_.
  raw_ptr<Info> info_;
};

}  // namespace base

#endif  // BASE_FUNCTIONAL_CONCURRENT_CLOSURES_H_
