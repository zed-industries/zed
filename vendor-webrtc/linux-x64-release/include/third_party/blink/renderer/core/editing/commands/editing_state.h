// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_STATE_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// EditingState represents current editing command running state for propagating
// DOM tree mutation operation failure to callers.
//
// Example usage:
//  EditingState editingState;
//  ...
//  functionMutatesDOMTree(..., &editingState);
//  if (editingState.isAborted())
//      return;
//
class CORE_EXPORT EditingState final {
  STACK_ALLOCATED();

 public:
  EditingState();
  EditingState(const EditingState&) = delete;
  EditingState& operator=(const EditingState&) = delete;

  void Abort();
  bool IsAborted() const { return is_aborted_; }

 private:
  bool is_aborted_ = false;
};

// TODO(yosin): Once all commands aware |EditingState|, we get rid of
// |IgnorableEditingAbortState | class
class IgnorableEditingAbortState final {
  STACK_ALLOCATED();

 public:
  IgnorableEditingAbortState();
  IgnorableEditingAbortState(const IgnorableEditingAbortState&) = delete;
  IgnorableEditingAbortState& operator=(const IgnorableEditingAbortState&) =
      delete;
  ~IgnorableEditingAbortState();

  EditingState* GetEditingState() { return &editing_state_; }

 private:
  EditingState editing_state_;
};

// Abort the editing command if the specified expression is true.
#define ABORT_EDITING_COMMAND_IF(expr) \
  do {                                 \
    if (expr) {                        \
      editing_state->Abort();          \
      return;                          \
    }                                  \
  } while (false)

#if DCHECK_IS_ON()
// This class is inspired by |NoExceptionStateAssertionChecker|.
class NoEditingAbortChecker final {
  STACK_ALLOCATED();

 public:
  explicit NoEditingAbortChecker(
      const base::Location& location = base::Location::Current());
  NoEditingAbortChecker(const NoEditingAbortChecker&) = delete;
  NoEditingAbortChecker& operator=(const NoEditingAbortChecker&) = delete;
  ~NoEditingAbortChecker();

  EditingState* GetEditingState() { return &editing_state_; }

 private:
  EditingState editing_state_;
  const base::Location location_;
};

// If a function with EditingState* argument should not be aborted,
// ASSERT_NO_EDITING_ABORT should be specified.
//    fooFunc(...., ASSERT_NO_EDITING_ABORT);
// It causes an assertion failure If DCHECK_IS_ON() and the function was aborted
// unexpectedly.
#define ASSERT_NO_EDITING_ABORT (NoEditingAbortChecker().GetEditingState())
#else
#define ASSERT_NO_EDITING_ABORT (IgnorableEditingAbortState().GetEditingState())
#endif

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_EDITING_STATE_H_
