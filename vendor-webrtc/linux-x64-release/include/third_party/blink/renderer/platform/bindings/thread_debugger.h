// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_THREAD_DEBUGGER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_THREAD_DEBUGGER_H_

#include <memory>

#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "v8/include/v8-forward.h"
#include "v8/include/v8-inspector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class SourceLocation;

class PLATFORM_EXPORT ThreadDebugger : public v8_inspector::V8InspectorClient {
 public:
  explicit ThreadDebugger(v8::Isolate* isolate)
      : v8_inspector_(v8_inspector::V8Inspector::create(isolate, this)) {}
  ThreadDebugger(const ThreadDebugger&) = delete;
  ThreadDebugger& operator=(const ThreadDebugger&) = delete;
  ~ThreadDebugger() override = default;

  static ThreadDebugger* From(v8::Isolate*);
  virtual bool IsWorker() = 0;
  v8_inspector::V8Inspector* GetV8Inspector() const {
    return v8_inspector_.get();
  }

  static void IdleStarted(v8::Isolate*);
  static void IdleFinished(v8::Isolate*);

  virtual void AsyncTaskScheduled(const StringView& task_name,
                                  void* task,
                                  bool recurring) = 0;
  virtual void AsyncTaskCanceled(void* task) = 0;
  virtual void AllAsyncTasksCanceled() = 0;
  virtual void AsyncTaskStarted(void* task) = 0;
  virtual void AsyncTaskFinished(void* task) = 0;
  virtual unsigned PromiseRejected(v8::Local<v8::Context>,
                                   const WTF::String& error_message,
                                   v8::Local<v8::Value> exception,
                                   std::unique_ptr<SourceLocation>) = 0;
  virtual void PromiseRejectionRevoked(v8::Local<v8::Context>,
                                       unsigned promise_rejection_id) = 0;

  virtual v8_inspector::V8StackTraceId StoreCurrentStackTrace(
      const StringView& description) = 0;
  virtual void ExternalAsyncTaskStarted(
      const v8_inspector::V8StackTraceId& parent) = 0;
  virtual void ExternalAsyncTaskFinished(
      const v8_inspector::V8StackTraceId& parent) = 0;

 protected:
  std::unique_ptr<v8_inspector::V8Inspector> v8_inspector_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_THREAD_DEBUGGER_H_
