// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_ASYNC_TASK_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_ASYNC_TASK_CONTEXT_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/ad_script_identifier.h"

namespace v8 {
class Isolate;
}  // namespace v8

namespace WTF {
class StringView;
}  // namespace WTF

namespace blink {
class ExecutionContext;
namespace probe {

// Tracks scheduling and cancelation of a single async task.
// An async task scheduled via `AsyncTaskContext` is guaranteed to be
// canceled.
class CORE_EXPORT AsyncTaskContext {
 public:
  AsyncTaskContext() = default;
  ~AsyncTaskContext();

  // Not copyable or movable. The address of `AsyncTaskContext` is used
  // to identify this task and corresponding runs/invocations via `AsyncTask`.
  AsyncTaskContext(const AsyncTaskContext&) = delete;
  AsyncTaskContext& operator=(const AsyncTaskContext&) = delete;

  // Schedules this async task with the ThreadDebugger. `Schedule` can be called
  // once and only once per AsyncTaskContext instance.
  void Schedule(ExecutionContext* context, const WTF::StringView& name);

  // Explicitly cancel this async task. No `AsyncTasks`s must be created with
  // this context after `Cancel` was called.
  void Cancel();

  // Marks this async task as being created on behalf of ad script. If the ad
  // script has an identifier then pass it in `ad_identifier` else pass
  // `std::nullopt`. `ad_identifier` is for developer debugging purposes and
  // providing an accurate identifier is best effort.
  void SetAdTask(const std::optional<AdScriptIdentifier>& ad_identifier) {
    ad_task_ = true;
    ad_identifier_ = ad_identifier;
  }

  bool IsAdTask() const { return ad_task_; }

  std::optional<AdScriptIdentifier> ad_identifier() const {
    return ad_identifier_;
  }

  // The Id uniquely identifies this task with the V8 debugger. The Id is
  // calculated based on the address of `AsyncTaskContext`.
  void* Id() const;

 private:
  friend class AsyncTask;

  // Whether or not this async task was created by ad script.
  bool ad_task_ = false;

  // If this async task was created by ad-related script, the identifier
  // specifies which ad script it was in many cases, but not always (e.g., not
  // when the entire execution context is considered ad related).
  std::optional<AdScriptIdentifier> ad_identifier_;

  v8::Isolate* isolate_ = nullptr;
};

}  // namespace probe
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PROBE_ASYNC_TASK_CONTEXT_H_
