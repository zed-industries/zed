// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_H_

#include <stdint.h>
#include <memory>

#include "third_party/blink/renderer/bindings/core/v8/async_iterable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_async_iterator_readable_stream.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_readable_stream_iterator_options.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/streams/readable_stream_default_reader.h"
#include "third_party/blink/renderer/core/streams/transferable_streams.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/bindings/trace_wrapper_v8_reference.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class MessagePort;
class PipeOptions;
class ReadableByteStreamController;
class ReadableStreamBYOBReader;
class ReadableStreamController;
class ReadableStreamDefaultController;
class ReadableStreamGetReaderOptions;
class ReadableStreamTransferringOptimizer;
class ReadableWritablePair;
class ReadIntoRequest;
class ReadRequest;
class ScriptState;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamPipeOptions;
class StreamStartAlgorithm;
class UnderlyingByteSourceBase;
class UnderlyingSourceBase;
class WritableStream;

// C++ implementation of ReadableStream.
// See https://streams.spec.whatwg.org/#rs-model for background.
class CORE_EXPORT ReadableStream
    : public ScriptWrappable,
      public ValueAsyncIterable<ReadableStream,
                                ReadableStreamIteratorOptions*> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum State : uint8_t { kReadable, kClosed, kErrored };

  // Zero-argument form of the constructor called from JavaScript.
  static ReadableStream* Create(ScriptState*, ExceptionState&);

  // One-argument constructor called from JavaScript.
  static ReadableStream* Create(ScriptState*,
                                ScriptValue underlying_source,
                                ExceptionState&);

  // Two-argument constructor called from JavaScript.
  static ReadableStream* Create(ScriptState* script_state,
                                ScriptValue underlying_source,
                                ScriptValue strategy,
                                ExceptionState& exception_state);

  // Entry point to create a ReadableStream from other C++ APIs.
  static ReadableStream* CreateWithCountQueueingStrategy(
      ScriptState* script_state,
      UnderlyingSourceBase* underlying_source,
      size_t high_water_mark);
  // Specifying true for `allow_per_chunk_transferring` implies the following:
  //  1. Each chunk has never been exposed to scripts.
  //  2. Each chunk is transferable.
  static ReadableStream* CreateWithCountQueueingStrategy(
      ScriptState* script_state,
      UnderlyingSourceBase* underlying_source,
      size_t high_water_mark,
      AllowPerChunkTransferring allow_per_chunk_transferring,
      std::unique_ptr<ReadableStreamTransferringOptimizer> optimizer);

  // CreateReadableStream():
  // https://streams.spec.whatwg.org/#create-readable-stream
  static ReadableStream* Create(ScriptState*,
                                StreamStartAlgorithm* start_algorithm,
                                StreamAlgorithm* pull_algorithm,
                                StreamAlgorithm* cancel_algorithm,
                                double high_water_mark,
                                StrategySizeAlgorithm* size_algorithm,
                                ExceptionState&);

  // https://streams.spec.whatwg.org/#abstract-opdef-createreadablebytestream
  static ReadableStream* CreateByteStream(ScriptState*,
                                          StreamStartAlgorithm* start_algorithm,
                                          StreamAlgorithm* pull_algorithm,
                                          StreamAlgorithm* cancel_algorithm,
                                          ExceptionState&);

  // Entry point to create a ReadableByteStream from other C++ APIs.
  // CreateReadableByteStream():
  // https://streams.spec.whatwg.org/#abstract-opdef-createreadablebytestream
  static ReadableStream* CreateByteStream(
      ScriptState*,
      UnderlyingByteSourceBase* underlying_byte_source);

  static void InitByteStream(ScriptState*,
                             ReadableStream*,
                             UnderlyingByteSourceBase* underlying_byte_source,
                             ExceptionState&);
  static void InitByteStream(ScriptState*,
                             ReadableStream*,
                             ReadableByteStreamController*,
                             StreamStartAlgorithm* start_algorithm,
                             StreamAlgorithm* pull_algorithm,
                             StreamAlgorithm* cancel_algorithm,
                             ExceptionState&);

  ReadableStream();

  ~ReadableStream() override;

  // See CreateWithCountQueueingStrategy() comment above for how to use
  // `allow_per_chunk_transferring`.
  void InitWithCountQueueingStrategy(
      ScriptState*,
      UnderlyingSourceBase*,
      size_t high_water_mark,
      AllowPerChunkTransferring allow_per_chunk_transferring,
      std::unique_ptr<ReadableStreamTransferringOptimizer>,
      ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-constructor
  bool locked() const;

  ScriptPromise<IDLUndefined> cancel(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-cancel
  ScriptPromise<IDLUndefined> cancel(ScriptState*,
                                     ScriptValue reason,
                                     ExceptionState&);

  V8ReadableStreamReader* getReader(ScriptState* script_state,
                                    ExceptionState& exception_state);

  // https://streams.spec.whatwg.org/#rs-get-reader
  V8ReadableStreamReader* getReader(
      ScriptState* script_state,
      const ReadableStreamGetReaderOptions* options,
      ExceptionState& exception_state);

  ReadableStreamDefaultReader* GetDefaultReaderForTesting(ScriptState*,
                                                          ExceptionState&);

  ReadableStreamBYOBReader* GetBYOBReaderForTesting(ScriptState*,
                                                    ExceptionState&);

  ReadableStream* pipeThrough(ScriptState*,
                              ReadableWritablePair* transform,
                              ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-pipe-through
  ReadableStream* pipeThrough(ScriptState*,
                              ReadableWritablePair* transform,
                              const StreamPipeOptions* options,
                              ExceptionState&);

  ScriptPromise<IDLUndefined> pipeTo(ScriptState*,
                                     WritableStream* destination,
                                     ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-pipe-to
  ScriptPromise<IDLUndefined> pipeTo(ScriptState*,
                                     WritableStream* destination,
                                     const StreamPipeOptions* options,
                                     ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-tee
  HeapVector<Member<ReadableStream>> tee(ScriptState*, ExceptionState&);

  void Tee(ScriptState*,
           ReadableStream** branch1,
           ReadableStream** branch2,
           bool clone_for_branch2,
           ExceptionState&);

  void ByteStreamTee(ScriptState*,
                     ReadableStream** branch1,
                     ReadableStream** branch2,
                     ExceptionState&);

  bool IsLocked() const { return IsLocked(this); }

  bool IsDisturbed() const { return IsDisturbed(this); }

  bool IsReadable() const { return IsReadable(this); }

  bool IsClosed() const { return IsClosed(this); }

  bool IsErrored() const { return IsErrored(this); }

  void LockAndDisturb(ScriptState*);

  // https://streams.spec.whatwg.org/#readablestream-close
  void CloseStream(ScriptState*, ExceptionState&);

  void Serialize(ScriptState*, MessagePort* port, ExceptionState&);

  static ReadableStream* Deserialize(
      ScriptState*,
      MessagePort* port,
      std::unique_ptr<ReadableStreamTransferringOptimizer> optimizer,
      ExceptionState&);

  //
  // Readable stream abstract operations
  //

  // https://streams.spec.whatwg.org/#is-readable-stream-disturbed
  static bool IsDisturbed(const ReadableStream* stream) {
    return stream->is_disturbed_;
  }

  // https://streams.spec.whatwg.org/#is-readable-stream-locked
  static bool IsLocked(const ReadableStream* stream) {
    return stream->reader_ != nullptr;
  }

  // https://streams.spec.whatwg.org/#readable-stream-pipe-to
  static ScriptPromise<IDLUndefined> PipeTo(ScriptState*,
                                            ReadableStream*,
                                            WritableStream*,
                                            PipeOptions*,
                                            ExceptionState&);

  // https://streams.spec.whatwg.org/#acquire-readable-stream-reader
  static ReadableStreamDefaultReader* AcquireDefaultReader(ScriptState*,
                                                           ReadableStream*,
                                                           ExceptionState&);

  // https://streams.spec.whatwg.org/#acquire-readable-stream-byob-reader
  static ReadableStreamBYOBReader* AcquireBYOBReader(ScriptState*,
                                                     ReadableStream*,
                                                     ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-stream-cancel
  static ScriptPromise<IDLUndefined> Cancel(ScriptState*,
                                            ReadableStream*,
                                            v8::Local<v8::Value> reason);

  //
  // Functions exported for use by TransformStream. Not part of the standard.
  //

  static bool IsReadable(const ReadableStream* stream) {
    return stream->state_ == kReadable;
  }

  static bool IsClosed(const ReadableStream* stream) {
    return stream->state_ == kClosed;
  }

  static bool IsErrored(const ReadableStream* stream) {
    return stream->state_ == kErrored;
  }

  ReadableStreamController* GetController() {
    return readable_stream_controller_.Get();
  }

  v8::Local<v8::Value> GetStoredError(v8::Isolate*) const;

  std::unique_ptr<ReadableStreamTransferringOptimizer>
  TakeTransferringOptimizer();

  void SetAllowPerChunkTransferringForTesting(AllowPerChunkTransferring value) {
    allow_per_chunk_transferring_ = value;
  }

  void Trace(Visitor*) const override;

 private:
  friend class ByteStreamTeeEngine;
  friend class PipeToEngine;
  friend class ReadableByteStreamController;
  friend class ReadableStreamBYOBReader;
  friend class ReadableStreamDefaultController;
  friend class ReadableStreamDefaultReader;
  friend class ReadableStreamGenericReader;
  friend class TeeEngine;

  class PullAlgorithm;
  class CancelAlgorithm;
  class ReadHandleImpl;
  class IterationSource;
  class IterationReadRequest;

  // https://streams.spec.whatwg.org/#rs-constructor
  void InitInternal(ScriptState*,
                    ScriptValue raw_underlying_source,
                    ScriptValue raw_strategy,
                    bool created_by_ua,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#initialize-readable-stream
  static void Initialize(ReadableStream*);

  static void AddReadIntoRequest(ScriptState*,
                                 ReadableStream*,
                                 ReadIntoRequest*);

  // https://streams.spec.whatwg.org/#readable-stream-add-read-request
  static void AddReadRequest(ScriptState*, ReadableStream*, ReadRequest*);

  // https://streams.spec.whatwg.org/#readable-stream-close
  static void Close(ScriptState*, ReadableStream*);

  // https://streams.spec.whatwg.org/#readable-stream-error
  static void Error(ScriptState*, ReadableStream*, v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#readable-stream-fulfill-read-into-request
  static void FulfillReadIntoRequest(ScriptState*,
                                     ReadableStream*,
                                     DOMArrayBufferView* chunk,
                                     bool done,
                                     ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-stream-fulfill-read-request
  static void FulfillReadRequest(ScriptState*,
                                 ReadableStream*,
                                 v8::Local<v8::Value> chunk,
                                 bool done,
                                 ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-stream-get-num-read-into-requests
  static int GetNumReadIntoRequests(const ReadableStream*);

  // https://streams.spec.whatwg.org/#readable-stream-get-num-read-requests
  static int GetNumReadRequests(const ReadableStream*);

  // https://streams.spec.whatwg.org/#readable-stream-has-byob-reader
  static bool HasBYOBReader(const ReadableStream*);

  // https://streams.spec.whatwg.org/#readable-stream-has-default-reader
  static bool HasDefaultReader(const ReadableStream*);

  // Calls Tee() on |readable|, converts the two branches to a JavaScript array
  // and returns them.
  static HeapVector<Member<ReadableStream>> CallTeeAndReturnBranchArray(
      ScriptState* script_state,
      ReadableStream* readable,
      bool clone_for_branch2,
      ExceptionState& exception_state);

  bool is_disturbed_ = false;
  // When set to true, each chunk can be transferred instead of cloned on
  // transferring the stream.
  AllowPerChunkTransferring allow_per_chunk_transferring_{false};
  State state_ = kReadable;
  Member<ReadableStreamController> readable_stream_controller_;
  Member<ReadableStreamGenericReader> reader_;
  TraceWrapperV8Reference<v8::Value> stored_error_;
  std::unique_ptr<ReadableStreamTransferringOptimizer> transferring_optimizer_;

  // ValueAsyncIterable<ReadableStream> overrides:
  using IterationSourceBase =
      ValueAsyncIterable<ReadableStream,
                         ReadableStreamIteratorOptions*>::IterationSource;
  IterationSourceBase* CreateIterationSource(
      ScriptState* script_state,
      IterationSourceBase::Kind kind,
      ReadableStreamIteratorOptions* options,
      ExceptionState& exception_state) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_H_
