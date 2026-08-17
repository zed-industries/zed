// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_INTO_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_INTO_REQUEST_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class DOMArrayBufferView;
class ExceptionState;
class ScriptState;

// Implementation of the "read-into request" struct from the standard.
// ReadIntoRequest allows polymorphically overriding the chunk, close, and
// error steps.
// https://streams.spec.whatwg.org/#read-into-request
class ReadIntoRequest : public GarbageCollected<ReadIntoRequest> {
 public:
  ReadIntoRequest() = default;
  ReadIntoRequest(const ReadIntoRequest&) = delete;
  ReadIntoRequest& operator=(const ReadIntoRequest&) = delete;
  virtual ~ReadIntoRequest() = default;

  virtual void ChunkSteps(ScriptState*,
                          DOMArrayBufferView* chunk,
                          ExceptionState&) const = 0;
  virtual void CloseSteps(ScriptState*, DOMArrayBufferView* chunk) const = 0;
  virtual void ErrorSteps(ScriptState*, v8::Local<v8::Value> e) const = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_INTO_REQUEST_H_
