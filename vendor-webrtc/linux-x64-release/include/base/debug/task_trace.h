// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_TASK_TRACE_H_
#define BASE_DEBUG_TASK_TRACE_H_

#include <iosfwd>
#include <optional>
#include <string>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/debug/stack_trace.h"

namespace base {
namespace debug {

// Provides a snapshot of which places in the code called
// base::TaskRunner::PostTask() that led to the TaskTrace() constructor call.
// Analogous to base::StackTrace, but for posted tasks rather than function
// calls.
//
// Example usage:
//   TaskTrace().Print();
//
// Example output:
//   Task trace:
//   #0 content::ServiceWorkerContextWrapper::DidCheckHasServiceWorker()
//   #1 content::ServiceWorkerStorage::FindForDocumentInDB()
//   #2 content::ServiceWorkerStorage::FindRegistrationForDocument()
//   #3 content::ServiceWorkerContextWrapper::CheckHasServiceWorker()
//   #4 content::ManifestIconDownloader::ScaleIcon()
//   Task trace buffer limit hit, update PendingTask::kTaskBacktraceLength to
//   increase.
class BASE_EXPORT TaskTrace {
 public:
  TaskTrace();

  // Whether there is any trace data.
  bool empty() const;

  // Outputs to stderr via OutputToStream.
  void Print() const;

  // Outputs trace to |os|, may be called when empty() is true.
  void OutputToStream(std::ostream* os) const;

  // Resolves trace to symbols and returns as string.
  std::string ToString() const;

  // Reads the list of addresses currently in the task trace into `addresses`,
  // and returns the maximum length of addresses that could have been read,
  // which may differ from `addresses.size()`.
  size_t GetAddresses(span<const void*> addresses) const;

 private:
  std::optional<StackTrace> stack_trace_;
  bool trace_overflow_ = false;
};

// Forwards to TaskTrace::OutputToStream.
BASE_EXPORT std::ostream& operator<<(std::ostream& os,
                                     const TaskTrace& task_trace);

}  // namespace debug
}  // namespace base

#endif  // BASE_DEBUG_TASK_TRACE_H_
