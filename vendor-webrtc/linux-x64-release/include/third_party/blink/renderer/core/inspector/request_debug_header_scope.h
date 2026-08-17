// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_REQUEST_DEBUG_HEADER_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_REQUEST_DEBUG_HEADER_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "v8/include/v8-inspector.h"

namespace blink {

class ExecutionContext;
class ThreadDebugger;

class CORE_EXPORT RequestDebugHeaderScope {
  STACK_ALLOCATED();

 public:
  static const char kHeaderName[];

  static String CaptureStackIdForCurrentLocation(ExecutionContext*);

  RequestDebugHeaderScope(ExecutionContext*, const String& header);
  ~RequestDebugHeaderScope();

 private:
  ThreadDebugger* debugger_ = nullptr;
  v8_inspector::V8StackTraceId stack_trace_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_REQUEST_DEBUG_HEADER_SCOPE_H_
