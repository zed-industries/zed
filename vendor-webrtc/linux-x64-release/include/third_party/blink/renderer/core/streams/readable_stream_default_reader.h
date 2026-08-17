// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_READER_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/streams/readable_byte_stream_controller.h"
#include "third_party/blink/renderer/core/streams/readable_stream_generic_reader.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class ReadableStreamReadResult;
class ReadRequest;
class ScriptState;

class CORE_EXPORT ReadableStreamDefaultReader
    : public ReadableStreamGenericReader,
      public ActiveScriptWrappable<ReadableStreamDefaultReader>,
      public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ReadableStreamDefaultReader* Create(ScriptState*,
                                             ReadableStream* stream,
                                             ExceptionState&);

  // https://streams.spec.whatwg.org/#default-reader-constructor
  ReadableStreamDefaultReader(ScriptState*,
                              ReadableStream* stream,
                              ExceptionState&);
  ~ReadableStreamDefaultReader() override;

  bool IsDefaultReader() const override { return true; }
  bool IsBYOBReader() const override { return false; }

  // https://streams.spec.whatwg.org/#default-reader-read
  ScriptPromise<ReadableStreamReadResult> read(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#default-reader-release-lock
  void releaseLock(ScriptState*, ExceptionState&);

  static void SetUpDefaultReader(ScriptState*,
                                 ReadableStreamDefaultReader* reader,
                                 ReadableStream* stream,
                                 ExceptionState&);

  //
  // Readable stream reader abstract operations
  //

  // https://streams.spec.whatwg.org/#readable-stream-default-reader-read
  static void Read(ScriptState*,
                   ReadableStreamDefaultReader* reader,
                   ReadRequest*,
                   ExceptionState&);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamdefaultreadererrorreadrequests
  static void ErrorReadRequests(ScriptState*,
                                ReadableStreamDefaultReader* reader,
                                v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamdefaultreaderrelease
  static void Release(ScriptState*, ReadableStreamDefaultReader* reader);

  void Trace(Visitor*) const override;

  bool HasPendingActivity() const final;

 private:
  friend class ByteStreamTeeEngine;
  friend class ReadableByteStreamController;
  friend class ReadableStreamController;
  friend class ReadableStreamDefaultController;
  friend class ReadableStream;

  class DefaultReaderReadRequest;

  HeapDeque<Member<ReadRequest>> read_requests_;
};

template <>
struct DowncastTraits<ReadableStreamDefaultReader> {
  static bool AllowFrom(const ReadableStreamGenericReader& reader) {
    return reader.IsDefaultReader();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_READER_H_
