// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_INSPECTOR_WEBSOCKET_EVENTS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_INSPECTOR_WEBSOCKET_EVENTS_H_

#include "third_party/blink/renderer/core/inspector/inspector_trace_events.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"
#include "third_party/perfetto/include/perfetto/tracing/traced_value_forward.h"

namespace blink {

class ExecutionContext;
class KURL;

class InspectorWebSocketCreateEvent {
  STATIC_ONLY(InspectorWebSocketCreateEvent);

 public:
  static void Data(perfetto::TracedValue context,
                   ExecutionContext*,
                   uint64_t identifier,
                   const KURL&,
                   const String& protocol);
};

class InspectorWebSocketEvent {
  STATIC_ONLY(InspectorWebSocketEvent);

 public:
  static void Data(perfetto::TracedValue context,
                   ExecutionContext*,
                   uint64_t identifier);
};

class InspectorWebSocketTransferEvent {
  STATIC_ONLY(InspectorWebSocketTransferEvent);

 public:
  static void Data(perfetto::TracedValue context,
                   ExecutionContext*,
                   uint64_t identifier,
                   uint64_t data_length);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBSOCKETS_INSPECTOR_WEBSOCKET_EVENTS_H_
