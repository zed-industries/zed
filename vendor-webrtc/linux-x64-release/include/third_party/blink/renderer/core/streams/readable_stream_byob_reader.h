// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_BYOB_READER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_BYOB_READER_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/streams/readable_stream_generic_reader.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class ExceptionState;
class ScriptState;
class ReadableStream;
class ReadableStreamReadResult;
class ReadIntoRequest;
class DOMArrayBufferView;

class CORE_EXPORT ReadableStreamBYOBReader
    : public ReadableStreamGenericReader {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ReadableStreamBYOBReader* Create(ScriptState*,
                                          ReadableStream* stream,
                                          ExceptionState&);

  // https://streams.spec.whatwg.org/#byob-reader-constructor
  ReadableStreamBYOBReader(ScriptState*,
                           ReadableStream* stream,
                           ExceptionState&);
  ~ReadableStreamBYOBReader() override;

  bool IsDefaultReader() const override { return false; }
  bool IsBYOBReader() const override { return true; }

  // https://streams.spec.whatwg.org/#byob-reader-read

  ScriptPromise<ReadableStreamReadResult> read(ScriptState*,
                                               NotShared<DOMArrayBufferView>,
                                               ExceptionState&);

  // https://streams.spec.whatwg.org/#byob-reader-release-lock
  void releaseLock(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#set-up-readable-stream-byob-reader
  static void SetUpBYOBReader(ScriptState*,
                              ReadableStreamBYOBReader* reader,
                              ReadableStream* stream,
                              ExceptionState&);

  void Trace(Visitor*) const override;

 private:
  friend class ByteStreamTeeEngine;
  friend class PipeToEngine;
  friend class ReadableByteStreamController;
  friend class ReadableStream;

  class BYOBReaderReadIntoRequest;

  //
  // Readable stream reader abstract operations
  //

  // https://streams.spec.whatwg.org/#readable-stream-byob-reader-read
  static void Read(ScriptState*,
                   ReadableStreamBYOBReader*,
                   NotShared<DOMArrayBufferView> view,
                   ReadIntoRequest*,
                   ExceptionState&);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreambyobreadererrorreadintorequests
  static void ErrorReadIntoRequests(ScriptState*,
                                    ReadableStreamBYOBReader*,
                                    v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreambyobreaderrelease
  static void Release(ScriptState*, ReadableStreamBYOBReader*);

  HeapDeque<Member<ReadIntoRequest>> read_into_requests_;
};

template <>
struct DowncastTraits<ReadableStreamBYOBReader> {
  static bool AllowFrom(const ReadableStreamGenericReader& reader) {
    return reader.IsBYOBReader();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_BYOB_READER_H_
