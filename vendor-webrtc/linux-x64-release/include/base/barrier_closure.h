// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_BARRIER_CLOSURE_H_
#define BASE_BARRIER_CLOSURE_H_

#include "base/base_export.h"
#include "base/functional/callback.h"

namespace base {

// BarrierClosure executes |done_closure| after it has been invoked
// |num_closures| times.
//
// If |num_closures| is 0, |done_closure| is executed immediately.
//
// BarrierClosure is thread-safe - the count of remaining closures is
// maintained as a base::AtomicRefCount. |done_closure| will be run on
// the thread that calls the final Run() on the returned closures.
//
// |done_closure| is also cleared on the final calling thread.
BASE_EXPORT RepeatingClosure BarrierClosure(size_t num_closures,
                                            OnceClosure done_closure);

}  // namespace base

#endif  // BASE_BARRIER_CLOSURE_H_
