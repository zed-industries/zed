// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SEQUENCE_TOKEN_H_
#define BASE_SEQUENCE_TOKEN_H_

#include "base/base_export.h"

namespace base {
namespace internal {

// A token that identifies a series of sequenced work items (i.e. tasks, native
// message handlers, code blocks running outside or a `RunLoop`, etc. that are
// mutually exclusive).
class BASE_EXPORT SequenceToken {
 public:
  // Instantiates an invalid SequenceToken.
  constexpr SequenceToken() = default;

  // Explicitly allow copy.
  SequenceToken(const SequenceToken& other) = default;
  SequenceToken& operator=(const SequenceToken& other) = default;

  // An invalid SequenceToken is not equal to any other SequenceToken, including
  // other invalid SequenceTokens.
  bool operator==(const SequenceToken& other) const;
  bool operator!=(const SequenceToken& other) const;

  // Returns true if this is a valid SequenceToken.
  bool IsValid() const;

  // Returns the integer uniquely representing this SequenceToken. This method
  // should only be used for tracing and debugging.
  int ToInternalValue() const;

  // Returns a valid SequenceToken which isn't equal to any previously returned
  // SequenceToken.
  static SequenceToken Create();

  // Returns the `SequenceToken` for the work item currently running on this
  // thread. A valid and unique `SequenceToken` is assigned to each thread. It
  // can be overridden in a scope with `TaskScope`.
  static SequenceToken GetForCurrentThread();

 private:
  explicit SequenceToken(int token) : token_(token) {}

  static constexpr int kInvalidSequenceToken = -1;
  int token_ = kInvalidSequenceToken;
};

// A token that identifies a task.
//
// This is used by ThreadCheckerImpl to determine whether calls to
// CalledOnValidThread() come from the same task and hence are deterministically
// single-threaded (vs. calls coming from different sequenced or parallel tasks,
// which may or may not run on the same thread).
class BASE_EXPORT TaskToken {
 public:
  // Instantiates an invalid TaskToken.
  constexpr TaskToken() = default;

  // Explicitly allow copy.
  TaskToken(const TaskToken& other) = default;
  TaskToken& operator=(const TaskToken& other) = default;

  // An invalid TaskToken is not equal to any other TaskToken, including
  // other invalid TaskTokens.
  bool operator==(const TaskToken& other) const;
  bool operator!=(const TaskToken& other) const;

  // Returns true if this is a valid TaskToken.
  bool IsValid() const;

  // In the scope of a `TaskScope`, returns a valid `TaskToken` which isn't
  // equal to any `TaskToken` returned in the scope of a different `TaskScope`.
  // Otherwise, returns an invalid `TaskToken`.
  static TaskToken GetForCurrentThread();

 private:
  friend class TaskScope;

  explicit TaskToken(int token) : token_(token) {}

  // Returns a valid `TaskToken` which isn't equal to any previously returned
  // `TaskToken`. Private as it is only meant to be instantiated by `TaskScope`.
  static TaskToken Create();

  static constexpr int kInvalidTaskToken = -1;
  int token_ = kInvalidTaskToken;
};

// Returns true if a thread checker bound in a different task than the current
// one but on the same sequence and thread may return true from
// `CalledOnValidSequence()`.
bool BASE_EXPORT CurrentTaskIsThreadBound();

// Identifies a scope in which a task runs.
class BASE_EXPORT [[maybe_unused, nodiscard]] TaskScope {
 public:
  // `sequence_token` identifies the series of mutually exclusive work items
  // that this task is part of (may be unique if this task isn't mutually
  // exclusive with any other work item). `is_thread_bound` sets the value
  // returned by `CurrentTaskIsThreadBound()` within the scope.
  // `is_running_synchronously` is true iff this is instantiated for a task run
  // synchronously by `RunOrPostTask()`.
  explicit TaskScope(SequenceToken sequence_token,
                     bool is_thread_bound,
                     bool is_running_synchronously = false);
  TaskScope(const TaskScope&) = delete;
  TaskScope& operator=(const TaskScope&) = delete;
  ~TaskScope();

 private:
  const TaskToken previous_task_token_;
  const SequenceToken previous_sequence_token_;
  const bool previous_task_is_thread_bound_;
  const bool previous_task_is_running_synchronously_;
};

}  // namespace internal

// Returns true if the current task is run synchronously by `RunOrPostTask()`.
bool BASE_EXPORT CurrentTaskIsRunningSynchronously();

}  // namespace base

#endif  // BASE_SEQUENCE_TOKEN_H_
