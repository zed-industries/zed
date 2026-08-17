// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TASK_THREAD_POOL_WORKER_THREAD_SET_H_
#define BASE_TASK_THREAD_POOL_WORKER_THREAD_SET_H_

#include <stddef.h>

#include <set>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"

namespace base {
namespace internal {

class WorkerThread;

// An ordered set of WorkerThreads which has custom logic to treat the worker at
// the front of the set as being "in-use" (so its time in that position doesn't
// count towards being inactive / reclaimable). Supports removal of arbitrary
// WorkerThreads. DCHECKs when a WorkerThread is inserted multiple times.
// WorkerThreads are not owned by the set. All operations are amortized
// O(log(n)). This class is NOT thread-safe.
class BASE_EXPORT WorkerThreadSet {
  struct Compare {
    bool operator()(const WorkerThread* a, const WorkerThread* b) const;
  };

 public:
  WorkerThreadSet();
  WorkerThreadSet(const WorkerThreadSet&) = delete;
  WorkerThreadSet& operator=(const WorkerThreadSet&) = delete;
  ~WorkerThreadSet();

  // Inserts |worker| in the set. |worker| must not already be on the set. Flags
  // the WorkerThread previously at the front of the set, if it
  // changed, or |worker| as unused.
  void Insert(WorkerThread* worker);

  // Removes the front WorkerThread from the set and returns it. Returns nullptr
  // if the set is empty. Flags the WorkerThread now at the front of the set, if
  // any, as being in-use.
  WorkerThread* Take();

  // Returns the front WorkerThread from the set, nullptr if empty.
  WorkerThread* Peek() const;

  // Returns true if |worker| is already in the set.
  bool Contains(const WorkerThread* worker) const;

  // Removes |worker| from the set. Must not be invoked for the first worker
  // on the set.
  void Remove(const WorkerThread* worker);

  // Returns the number of WorkerThreads on the set.
  size_t Size() const { return set_.size(); }

  // Returns true if the set is empty.
  bool IsEmpty() const { return set_.empty(); }

 private:
  std::set<raw_ptr<WorkerThread, SetExperimental>, Compare> set_;
};

}  // namespace internal
}  // namespace base

#endif  // BASE_TASK_THREAD_POOL_WORKER_THREAD_SET_H_
