// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SOURCE_LOCATION_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SOURCE_LOCATION_H_

#include <memory>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"
#include "v8/include/v8-inspector.h"

namespace perfetto::protos::pbzero {
class BlinkSourceLocation;
}  // namespace perfetto::protos::pbzero

namespace blink {

class ExecutionContext;
class TracedValue;

// A location in JS source code.
// CaptureSourceLocation at the bottom of this file captures SourceLocation and
// return it. CaptureSourceLocation that depends on
// third_party/blink/renderer/core/ is in
// third_party/blink/renderer/bindings/core/v8/capture_source_location.h. We can
// capture SourceLocation using ExecutionContext.
class PLATFORM_EXPORT SourceLocation {
  USING_FAST_MALLOC(SourceLocation);

 public:
  // Forces full stack trace.
  static std::unique_ptr<SourceLocation> CaptureWithFullStackTrace();

  // Used only by CaptureSourceLocation.
  static std::unique_ptr<v8_inspector::V8StackTrace> CaptureStackTraceInternal(
      bool full);

  static std::unique_ptr<SourceLocation> CreateFromNonEmptyV8StackTraceInternal(
      std::unique_ptr<v8_inspector::V8StackTrace>);

  SourceLocation(const String& url, int char_position);

  SourceLocation(const String& url,
                 int char_position,
                 unsigned line_number,
                 unsigned column_number);

  SourceLocation(const String& url,
                 const String& function,
                 unsigned line_number,
                 unsigned column_number,
                 std::unique_ptr<v8_inspector::V8StackTrace>,
                 int script_id = 0);
  ~SourceLocation();

  bool IsUnknown() const {
    return url_.IsNull() && !script_id_ && !line_number_;
  }
  const String& Url() const { return url_; }
  const String& Function() const { return function_; }
  unsigned LineNumber() const { return line_number_; }
  unsigned ColumnNumber() const { return column_number_; }
  int CharPosition() const { return char_position_; }
  int ScriptId() const { return script_id_; }
  std::unique_ptr<v8_inspector::V8StackTrace> TakeStackTrace() {
    return std::move(stack_trace_);
  }

  bool HasStackTrace() const {
    return stack_trace_ && !stack_trace_->isEmpty();
  }

  // Safe to pass between threads, drops async chain in stack trace.
  std::unique_ptr<SourceLocation> Clone() const;

  // Write a representation of this object into a trace.
  using Proto = perfetto::protos::pbzero::BlinkSourceLocation;
  void WriteIntoTrace(perfetto::TracedProto<Proto> proto) const;
  // TODO(altimin): Remove TracedValue version.
  void WriteIntoTrace(perfetto::TracedValue context) const;

  // No-op when stack trace is unknown.
  // TODO(altimin): Replace all usages of `ToTracedValue` with
  // `WriteIntoTrace` and remove this method.
  void ToTracedValue(TracedValue*, const char* name) const;

  // Could be null string when stack trace is unknown.
  String ToString() const;

  // Could be null when stack trace is unknown.
  std::unique_ptr<v8_inspector::protocol::Runtime::API::StackTrace>
  BuildInspectorObject() const;

  std::unique_ptr<v8_inspector::protocol::Runtime::API::StackTrace>
  BuildInspectorObject(int max_async_depth) const;

 private:
  String url_;
  String function_;
  unsigned line_number_;
  unsigned column_number_;
  int char_position_;
  std::unique_ptr<v8_inspector::V8StackTrace> stack_trace_;
  int script_id_;
};

// Zero lineNumber and columnNumber mean unknown. Captures current stack
// trace.
PLATFORM_EXPORT std::unique_ptr<SourceLocation> CaptureSourceLocation(
    const String& url,
    unsigned line_number,
    unsigned column_number);

// Returns SourceLocation if non-empty stack trace exists.
// If stack trace doesn't exists or it's empty, returns nullptr.
// This is the same when CaptureSourceLocation(ExecutionContext* = nullptr) in
// bindings/core/v8/capture_source_location.h
PLATFORM_EXPORT std::unique_ptr<SourceLocation> CaptureSourceLocation();

// Captures current stack trace from function.
PLATFORM_EXPORT std::unique_ptr<SourceLocation> CaptureSourceLocation(
    v8::Isolate* isolate,
    v8::Local<v8::Function>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SOURCE_LOCATION_H_
