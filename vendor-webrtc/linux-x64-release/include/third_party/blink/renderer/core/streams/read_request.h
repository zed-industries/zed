// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_REQUEST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_REQUEST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {
class ExceptionState;
class ScriptState;

// Implementation of the "read request" struct from the standard. ReadRequest
// allows polymorphically overriding the chunk, close, and error steps.
// https://streams.spec.whatwg.org/#read-request
class CORE_EXPORT ReadRequest : public GarbageCollected<ReadRequest> {
 public:
  ReadRequest() = default;
  ReadRequest(const ReadRequest&) = delete;
  ReadRequest& operator=(const ReadRequest&) = delete;
  virtual ~ReadRequest() = default;

  virtual void ChunkSteps(ScriptState*,
                          v8::Local<v8::Value> chunk,
                          ExceptionState&) const = 0;
  virtual void CloseSteps(ScriptState*) const = 0;
  virtual void ErrorSteps(ScriptState*, v8::Local<v8::Value> e) const = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READ_REQUEST_H_
