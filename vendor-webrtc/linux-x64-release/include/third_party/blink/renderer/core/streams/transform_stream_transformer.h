// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_TRANSFORMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_TRANSFORMER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ScriptState;
class TransformStreamDefaultController;

// Interface to be implemented by C++ code that needs to create a
// TransformStream. Based on the JavaScript [Transformer
// API](https://streams.spec.whatwg.org/#transformer-api). Errors should be
// signalled by exceptions or promise rejections.
class CORE_EXPORT TransformStreamTransformer
    : public GarbageCollected<TransformStreamTransformer> {
 public:
  TransformStreamTransformer() = default;
  TransformStreamTransformer(const TransformStreamTransformer&) = delete;
  TransformStreamTransformer& operator=(const TransformStreamTransformer&) =
      delete;
  virtual ~TransformStreamTransformer() = default;

  virtual ScriptPromise<IDLUndefined> Transform(
      v8::Local<v8::Value> chunk,
      TransformStreamDefaultController*,
      ExceptionState&) = 0;
  virtual ScriptPromise<IDLUndefined> Flush(TransformStreamDefaultController*,
                                            ExceptionState&) = 0;

  // Returns the ScriptState associated with this Transformer.
  virtual ScriptState* GetScriptState() = 0;

  virtual void Trace(Visitor*) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_TRANSFORM_STREAM_TRANSFORMER_H_
