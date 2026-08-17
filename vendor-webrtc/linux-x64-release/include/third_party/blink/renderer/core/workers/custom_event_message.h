// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CUSTOM_EVENT_MESSAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CUSTOM_EVENT_MESSAGE_H_

#include "third_party/blink/public/common/messaging/message_port_channel.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"
#include "v8/include/v8-inspector.h"

namespace blink {

struct CORE_EXPORT CustomEventMessage {
  Vector<MessagePortChannel> ports;
  scoped_refptr<blink::SerializedScriptValue> message;
  v8_inspector::V8StackTraceId sender_stack_trace_id;
  uint64_t trace_id;
};

}  // namespace blink

namespace WTF {

template <>
struct CrossThreadCopier<blink::CustomEventMessage> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = blink::CustomEventMessage;
  static Type Copy(Type pointer) {
    return pointer;  // This is in fact a move.
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_WORKERS_CUSTOM_EVENT_MESSAGE_H_
