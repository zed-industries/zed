// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_THREAD_DEBUGGER_COMMON_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_THREAD_DEBUGGER_COMMON_IMPL_H_

#include <memory>

#include "third_party/blink/renderer/platform/bindings/thread_debugger.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExecutionContext;
class SourceLocation;

// Implementation class of "ThreadDebugger" that is an abstract class in
// platform/bindings. CommonImpl means the common implementation among
// MainThreadDebugger and WorkerThreadDebugger.
class ThreadDebuggerCommonImpl : public ThreadDebugger {
 public:
  explicit ThreadDebuggerCommonImpl(v8::Isolate* isolate);
  ThreadDebuggerCommonImpl(const ThreadDebuggerCommonImpl&) = delete;
  ThreadDebuggerCommonImpl& operator=(const ThreadDebuggerCommonImpl&) = delete;
  ~ThreadDebuggerCommonImpl() override;

  void AsyncTaskScheduled(const StringView& task_name,
                          void* task,
                          bool recurring) override;
  void AsyncTaskCanceled(void* task) override;
  void AllAsyncTasksCanceled() override;
  void AsyncTaskStarted(void* task) override;
  void AsyncTaskFinished(void* task) override;
  unsigned PromiseRejected(v8::Local<v8::Context>,
                           const WTF::String& error_message,
                           v8::Local<v8::Value> exception,
                           std::unique_ptr<SourceLocation>) override;
  void PromiseRejectionRevoked(v8::Local<v8::Context>,
                               unsigned promise_rejection_id) override;

  v8_inspector::V8StackTraceId StoreCurrentStackTrace(
      const StringView& description) override;
  void ExternalAsyncTaskStarted(
      const v8_inspector::V8StackTraceId& parent) override;
  void ExternalAsyncTaskFinished(
      const v8_inspector::V8StackTraceId& parent) override;

 protected:
  virtual int ContextGroupId(ExecutionContext*) = 0;
  virtual void ReportConsoleMessage(ExecutionContext*,
                                    mojom::ConsoleMessageSource,
                                    mojom::ConsoleMessageLevel,
                                    const WTF::String& message,
                                    SourceLocation*) = 0;
  void installAdditionalCommandLineAPI(v8::Local<v8::Context>,
                                       v8::Local<v8::Object>) override;
  void CreateFunctionProperty(v8::Local<v8::Context>,
                              v8::Local<v8::Object>,
                              const char* name,
                              v8::FunctionCallback,
                              const char* description,
                              v8::SideEffectType side_effect_type);
  static v8::Maybe<bool> CreateDataPropertyInArray(v8::Local<v8::Context>,
                                                   v8::Local<v8::Array>,
                                                   int index,
                                                   v8::Local<v8::Value>);
  static mojom::ConsoleMessageLevel V8MessageLevelToMessageLevel(
      v8::Isolate::MessageErrorLevel);

  v8::Isolate* isolate_;

 private:
  // V8InspectorClient implementation.
  void beginUserGesture() override;
  std::unique_ptr<v8_inspector::DeepSerializationResult> deepSerialize(
      v8::Local<v8::Value> v8_value,
      int max_depth,
      v8::Local<v8::Object> additional_parameters) override;
  std::unique_ptr<v8_inspector::StringBuffer> valueSubtype(
      v8::Local<v8::Value>) override;
  std::unique_ptr<v8_inspector::StringBuffer> descriptionForValueSubtype(
      v8::Local<v8::Context>,
      v8::Local<v8::Value>) override;
  double currentTimeMS() override;
  bool isInspectableHeapObject(v8::Local<v8::Object>) override;
  void consoleTime(v8::Isolate* isolate, v8::Local<v8::String> label) override;
  void consoleTimeEnd(v8::Isolate* isolate,
                      v8::Local<v8::String> label) override;
  void consoleTimeStamp(v8::Isolate* isolate,
                        v8::Local<v8::String> label) override;

  void consoleTimeStampWithArgs(
      v8::Isolate* isolate,
      v8::Local<v8::String> label,
      const v8::LocalVector<v8::Value>& args) override;

  void startRepeatingTimer(double,
                           v8_inspector::V8InspectorClient::TimerCallback,
                           void* data) override;
  void cancelTimer(void* data) override;
  int64_t generateUniqueId() override;

  void OnTimer(TimerBase*);

  static void SetMonitorEventsCallback(
      const v8::FunctionCallbackInfo<v8::Value>&,
      bool enabled);
  static void MonitorEventsCallback(const v8::FunctionCallbackInfo<v8::Value>&);
  static void UnmonitorEventsCallback(
      const v8::FunctionCallbackInfo<v8::Value>&);
  static void GetAccessibleNameCallback(
      const v8::FunctionCallbackInfo<v8::Value>&);
  static void GetAccessibleRoleCallback(
      const v8::FunctionCallbackInfo<v8::Value>&);

  static void GetEventListenersCallback(
      const v8::FunctionCallbackInfo<v8::Value>&);

  Vector<std::unique_ptr<TaskRunnerTimer<ThreadDebuggerCommonImpl>>> timers_;
  Vector<v8_inspector::V8InspectorClient::TimerCallback> timer_callbacks_;
  Vector<void*> timer_data_;
};

}  // namespace blink

namespace WTF {

template <>
struct CrossThreadCopier<v8_inspector::V8StackTraceId> {
  typedef v8_inspector::V8StackTraceId Type;
  static Type Copy(const Type& id) { return id; }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_THREAD_DEBUGGER_COMMON_IMPL_H_
