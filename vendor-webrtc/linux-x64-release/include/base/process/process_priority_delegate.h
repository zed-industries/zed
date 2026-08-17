// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROCESS_PROCESS_PRIORITY_DELEGATE_H_
#define BASE_PROCESS_PROCESS_PRIORITY_DELEGATE_H_

#include "base/base_export.h"
#include "base/process/process.h"

namespace base {

// A ProcessPriorityDelegate can intercept process priority changes. This can be
// used to adjust process properties via another process (e.g. resourced on
// ChromeOS).
// Methods are thread-safe.
class BASE_EXPORT ProcessPriorityDelegate {
 public:
  ProcessPriorityDelegate() = default;

  ProcessPriorityDelegate(const ProcessPriorityDelegate&) = delete;
  ProcessPriorityDelegate& operator=(const ProcessPriorityDelegate&) = delete;

  virtual ~ProcessPriorityDelegate() = default;

  // Returns true if changing the priority of processes through
  // `base::Process::SetPriority()` is possible.
  virtual bool CanSetProcessPriority() = 0;

  // Initialize internal state for the process priority.
  virtual void InitializeProcessPriority(ProcessId process_id) = 0;

  // Clears internal state for the process priority.
  virtual void ForgetProcessPriority(ProcessId process_id) = 0;

  // Set process priotiry on behalf of base::Process::SetPriority().
  virtual bool SetProcessPriority(ProcessId process_id,
                                  Process::Priority priority) = 0;

  // Returns the process priority on behalf of base::Process::GetPriority().
  virtual Process::Priority GetProcessPriority(ProcessId process_id) = 0;
};

}  // namespace base

#endif  // BASE_PROCESS_PROCESS_PRIORITY_DELEGATE_H_
