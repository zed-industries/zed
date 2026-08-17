// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_CONTROLLER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "v8/include/v8.h"

namespace blink {
class ExceptionState;
class ReadRequest;
class ScriptState;

class ReadableStreamController : public ScriptWrappable {
 public:
  virtual bool IsDefaultController() const = 0;
  virtual bool IsByteStreamController() const = 0;

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamcontroller-cancelsteps
  virtual ScriptPromise<IDLUndefined> CancelSteps(
      ScriptState*,
      v8::Local<v8::Value> reason) = 0;

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamcontroller-pullsteps
  virtual void PullSteps(ScriptState*, ReadRequest*, ExceptionState&) = 0;

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamcontroller-releasesteps
  virtual void ReleaseSteps() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_CONTROLLER_H_
