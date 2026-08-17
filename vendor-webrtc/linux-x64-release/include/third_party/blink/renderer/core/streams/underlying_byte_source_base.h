// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_BYTE_SOURCE_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_BYTE_SOURCE_BASE_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ReadableByteStreamController;
class ScriptState;

// Interface to be implemented by C++ code that needs to create a
// ReadableByteStream. Based on the JavaScript [UnderlyingSource
// API](https://streams.spec.whatwg.org/#underlying-source-api). Errors should
// be signalled by exceptions or promise rejections.
class CORE_EXPORT UnderlyingByteSourceBase
    : public GarbageCollected<UnderlyingByteSourceBase> {
 public:
  UnderlyingByteSourceBase() = default;
  UnderlyingByteSourceBase(const UnderlyingByteSourceBase&) = delete;
  UnderlyingByteSourceBase& operator=(const UnderlyingByteSourceBase&) = delete;
  virtual ~UnderlyingByteSourceBase() = default;

  virtual ScriptPromise<IDLUndefined> Pull(
      ReadableByteStreamController* controller,
      ExceptionState&) = 0;

  virtual ScriptPromise<IDLUndefined> Cancel() = 0;
  virtual ScriptPromise<IDLUndefined> Cancel(v8::Local<v8::Value> reason) = 0;

  // Returns the ScriptState associated with this UnderlyingByteSource.
  virtual ScriptState* GetScriptState() = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_UNDERLYING_BYTE_SOURCE_BASE_H_
