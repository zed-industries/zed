// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_WRITER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_WRITER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ScriptState;
class ScriptValue;
class WritableStream;
class WritableStream;

// https://streams.spec.whatwg.org/#default-writer-class
class CORE_EXPORT WritableStreamDefaultWriter final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // JavaScript-exposed constructor.
  static WritableStreamDefaultWriter* Create(ScriptState*,
                                             WritableStream* stream,
                                             ExceptionState&);

  // https://streams.spec.whatwg.org/#default-writer-constructor
  WritableStreamDefaultWriter(ScriptState*,
                              WritableStream* stream,
                              ExceptionState&);
  ~WritableStreamDefaultWriter() override;

  // Getters

  // https://streams.spec.whatwg.org/#default-writer-closed
  ScriptPromise<IDLUndefined> closed(ScriptState*) const;

  // https://streams.spec.whatwg.org/#default-writer-desired-size
  ScriptValue desiredSize(ScriptState*, ExceptionState&) const;

  // https://streams.spec.whatwg.org/#default-writer-ready
  ScriptPromise<IDLUndefined> ready(ScriptState*) const;

  // Methods

  // https://streams.spec.whatwg.org/#default-writer-abort
  ScriptPromise<IDLUndefined> abort(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> abort(ScriptState*,
                                    ScriptValue reason,
                                    ExceptionState&);

  // https://streams.spec.whatwg.org/#default-writer-close
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#default-writer-release-lock
  void releaseLock(ScriptState*);

  // https://streams.spec.whatwg.org/#default-writer-write
  ScriptPromise<IDLUndefined> write(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> write(ScriptState*,
                                    ScriptValue chunk,
                                    ExceptionState&);

  //
  // Methods used by WritableStream
  //

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-ensure-ready-promise-rejected
  static void EnsureReadyPromiseRejected(ScriptState*,
                                         WritableStreamDefaultWriter*,
                                         v8::Local<v8::Value> error);

  //
  // Methods used by ReadableStream
  //

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-close-with-error-propagation
  static ScriptPromise<IDLUndefined> CloseWithErrorPropagation(
      ScriptState*,
      WritableStreamDefaultWriter*);

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-release
  static void Release(ScriptState*, WritableStreamDefaultWriter*);

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-write
  static ScriptPromise<IDLUndefined> Write(ScriptState*,
                                           WritableStreamDefaultWriter*,
                                           v8::Local<v8::Value> chunk,
                                           ExceptionState&);

  //
  // Accessors used by ReadableStream and WritableStream. These do
  // not appear in the standard.
  //
  ScriptPromiseResolver<IDLUndefined>* ClosedResolver() {
    return closed_resolver_.Get();
  }
  ScriptPromiseResolver<IDLUndefined>* ReadyResolver() {
    return ready_resolver_.Get();
  }
  WritableStream* OwnerWritableStream() { return owner_writable_stream_.Get(); }

  // This is a variant of GetDesiredSize() that doesn't create an intermediate
  // JavaScript object. Instead it returns std::nullopt where the JavaScript
  // version would return null.
  std::optional<double> GetDesiredSizeInternal() const;

  void ResetReadyPromise(ScriptState*);

  void Trace(Visitor*) const override;

 private:
  // https://streams.spec.whatwg.org/#writable-stream-default-writer-abort
  static ScriptPromise<IDLUndefined> Abort(ScriptState*,
                                           WritableStreamDefaultWriter*,
                                           v8::Local<v8::Value> reason);

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-close
  static ScriptPromise<IDLUndefined> Close(ScriptState*,
                                           WritableStreamDefaultWriter*);

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-ensure-closed-promise-rejected
  static void EnsureClosedPromiseRejected(ScriptState*,
                                          WritableStreamDefaultWriter*,
                                          v8::Local<v8::Value> error);

  // https://streams.spec.whatwg.org/#writable-stream-default-writer-get-desired-size
  static v8::Local<v8::Value> GetDesiredSize(
      v8::Isolate* isolate,
      const WritableStreamDefaultWriter*);

  Member<WritableStream> owner_writable_stream_;

  // |closed_promise_| and |ready_promise_| are implemented as resolvers. The
  // names come from the slots [[closedPromise]] and [[readyPromise]] in the
  // standard.
  Member<ScriptPromiseResolver<IDLUndefined>> closed_resolver_;
  Member<ScriptPromiseResolver<IDLUndefined>> ready_resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_DEFAULT_WRITER_H_
