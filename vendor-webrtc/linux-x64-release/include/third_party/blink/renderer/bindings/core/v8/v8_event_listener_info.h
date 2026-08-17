// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EVENT_LISTENER_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EVENT_LISTENER_INFO_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "v8/include/v8.h"

namespace blink {

struct V8EventListenerInfo {
  DISALLOW_NEW();

 public:
  V8EventListenerInfo(AtomicString event_type,
                      bool use_capture,
                      bool passive,
                      bool once,
                      v8::Local<v8::Object> handler,
                      v8::Local<v8::Function> effective_function,
                      uint64_t backend_node_id)
      : event_type(event_type),
        use_capture(use_capture),
        passive(passive),
        once(once),
        handler(handler),
        effective_function(effective_function),
        backend_node_id(backend_node_id) {}

  AtomicString event_type;
  bool use_capture;
  bool passive;
  bool once;
  v8::Local<v8::Object> handler;
  v8::Local<v8::Function> effective_function;
  uint64_t backend_node_id;
};

using V8EventListenerInfoList = Vector<V8EventListenerInfo>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_BINDINGS_CORE_V8_V8_EVENT_LISTENER_INFO_H_
