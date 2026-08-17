// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_GENERIC_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_GENERIC_READER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class ScriptState;

class CORE_EXPORT ReadableStreamGenericReader : public ScriptWrappable {

 public:
  ReadableStreamGenericReader();
  ~ReadableStreamGenericReader() override;

  virtual bool IsDefaultReader() const = 0;
  virtual bool IsBYOBReader() const = 0;

  // https://streams.spec.whatwg.org/#generic-reader-closed
  ScriptPromise<IDLUndefined> closed(ScriptState*) const;

  // https://streams.spec.whatwg.org/#generic-reader-cancel
  ScriptPromise<IDLUndefined> cancel(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> cancel(ScriptState*,
                                     ScriptValue reason,
                                     ExceptionState&);

  //
  // Readable stream reader abstract operations
  //

  // https://streams.spec.whatwg.org/#readable-stream-reader-generic-release
  static void GenericRelease(ScriptState*, ReadableStreamGenericReader*);

  // https://streams.spec.whatwg.org/#readable-stream-reader-generic-cancel
  static ScriptPromise<IDLUndefined> GenericCancel(ScriptState*,
                                                   ReadableStreamGenericReader*,
                                                   v8::Local<v8::Value> reason);

  // https://streams.spec.whatwg.org/#readable-stream-reader-generic-initialize
  static void GenericInitialize(ScriptState*,
                                ReadableStreamGenericReader*,
                                ReadableStream*);

  ScriptPromiseResolver<IDLUndefined>* ClosedResolver() const {
    return closed_resolver_;
  }

  void Trace(Visitor*) const override;

 private:
  friend class PipeToEngine;
  friend class ReadableStreamDefaultController;

  Member<ScriptPromiseResolver<IDLUndefined>> closed_resolver_;

 protected:
  Member<ReadableStream> owner_readable_stream_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_GENERIC_READER_H_
