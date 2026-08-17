// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_BYTE_STREAM_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_BYTE_STREAM_CONTROLLER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/streams/readable_stream_controller.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "v8/include/v8.h"

namespace blink {

class DOMArrayBuffer;
class DOMArrayBufferView;
class ExceptionState;
class ReadableStream;
class ReadableStreamBYOBRequest;
class ReadIntoRequest;
class ReadRequest;
class ScriptState;
class StreamAlgorithm;
class StreamStartAlgorithm;
class UnderlyingSource;

class CORE_EXPORT ReadableByteStreamController
    : public ReadableStreamController {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ReadableByteStreamController();

  // https://streams.spec.whatwg.org/#rbs-controller-byob-request
  ReadableStreamBYOBRequest* byobRequest();

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontrollergetbyobrequest
  static ReadableStreamBYOBRequest* GetBYOBRequest(
      ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#rbs-controller-desired-size
  std::optional<double> desiredSize();

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-get-desired-size
  static std::optional<double> GetDesiredSize(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#rbs-controller-close
  void close(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#rbs-controller-enqueue
  void enqueue(ScriptState*,
               NotShared<DOMArrayBufferView> chunk,
               ExceptionState&);

  // https://streams.spec.whatwg.org/#rbs-controller-error
  void error(ScriptState*);
  void error(ScriptState*, const ScriptValue& e);

  bool IsByteStreamController() const override { return true; }
  bool IsDefaultController() const override { return false; }

  void Trace(Visitor*) const override;

 private:
  friend class BodyStreamBuffer;
  friend class BodyStreamBufferUnderlyingByteSource;
  friend class ByteStreamTeeEngine;
  friend class ReadableStream;
  friend class ReadableStreamBYOBReader;
  friend class ReadableStreamBYOBRequest;

  // https://streams.spec.whatwg.org/#readable-byte-stream-queue-entry
  struct QueueEntry final : public GarbageCollected<QueueEntry> {
    explicit QueueEntry(DOMArrayBuffer* buffer,
                        size_t byte_offset,
                        size_t byte_length);

    const Member<DOMArrayBuffer> buffer;
    size_t byte_offset;
    size_t byte_length;

    void Trace(Visitor*) const;
  };

  enum class ReaderType { kDefault, kBYOB, kNone };

  // https://streams.spec.whatwg.org/#pull-into-descriptor
  struct PullIntoDescriptor final
      : public GarbageCollected<PullIntoDescriptor> {
    // A function pointer is used to represent the view constructor
    // to accommodate for different array types as specified by the
    // ArrayBufferViewConstructorAdaptor.
    using ViewConstructorType = DOMArrayBufferView* (*)(DOMArrayBuffer*,
                                                        size_t,
                                                        size_t);

    explicit PullIntoDescriptor(DOMArrayBuffer* buffer,
                                size_t buffer_byte_length,
                                size_t byte_offset,
                                size_t byte_length,
                                size_t bytes_filled,
                                size_t element_size,
                                ViewConstructorType view_constructor,
                                ReaderType reader_type);

    Member<DOMArrayBuffer> buffer;
    const size_t buffer_byte_length;
    size_t byte_offset;
    const size_t byte_length;
    size_t bytes_filled;
    const size_t element_size;
    const ViewConstructorType view_constructor;
    ReaderType reader_type;

    void Trace(Visitor*) const;
  };

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-close
  void Close(ScriptState*, ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-error
  static void Error(ScriptState*,
                    ReadableByteStreamController*,
                    v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-enqueue
  static void Enqueue(ScriptState*,
                      ReadableByteStreamController*,
                      NotShared<DOMArrayBufferView> chunk,
                      ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-enqueue-chunk-to-queue
  static void EnqueueChunkToQueue(ReadableByteStreamController*,
                                  DOMArrayBuffer*,
                                  size_t byte_offset,
                                  size_t byte_length);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontrollerenqueueclonedchunktoqueue
  static void EnqueueClonedChunkToQueue(ReadableByteStreamController*,
                                        DOMArrayBuffer*,
                                        size_t byte_offset,
                                        size_t byte_length);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontrollerenqueuedetachedpullintotoqueue
  static void EnqueueDetachedPullIntoToQueue(ReadableByteStreamController*,
                                             PullIntoDescriptor*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-process-pull-into-descriptors-using-queue
  static void ProcessPullIntoDescriptorsUsingQueue(
      ScriptState*,
      ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontrollerprocessreadrequestsusingqueue
  static void ProcessReadRequestsUsingQueue(ScriptState*,
                                            ReadableByteStreamController*,
                                            ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-call-pull-if-needed
  static void CallPullIfNeeded(ScriptState*, ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-shift-pending-pull-into
  static PullIntoDescriptor* ShiftPendingPullInto(
      ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-should-call-pull
  static bool ShouldCallPull(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-commit-pull-into-descriptor
  static void CommitPullIntoDescriptor(ScriptState*,
                                       ReadableStream*,
                                       PullIntoDescriptor*,
                                       ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-convert-pull-into-descriptor
  static DOMArrayBufferView* ConvertPullIntoDescriptor(ScriptState*,
                                                       PullIntoDescriptor*,
                                                       ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-clear-pending-pull-intos
  static void ClearPendingPullIntos(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-clear-algorithms
  static void ClearAlgorithms(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-invalidate-byob-request
  static void InvalidateBYOBRequest(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#set-up-readable-byte-stream-controller
  static void SetUp(ScriptState*,
                    ReadableStream*,
                    ReadableByteStreamController*,
                    StreamStartAlgorithm* start_algorithm,
                    StreamAlgorithm* pull_algorithm,
                    StreamAlgorithm* cancel_algorithm,
                    double high_water_mark,
                    size_t auto_allocate_chunk_size,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#set-up-readable-byte-stream-controller-from-underlying-source
  static void SetUpFromUnderlyingSource(
      ScriptState*,
      ReadableStream*,
      v8::Local<v8::Object> underlying_source,
      UnderlyingSource* underlying_source_dict,
      double high_water_mark,
      ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-fill-head-pull-into-descriptor
  static void FillHeadPullIntoDescriptor(ReadableByteStreamController*,
                                         size_t size,
                                         PullIntoDescriptor*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-fill-pull-into-descriptor-from-queue
  static bool FillPullIntoDescriptorFromQueue(ReadableByteStreamController*,
                                              PullIntoDescriptor*,
                                              ExceptionState&);

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontrollerfillreadrequestfromqueue
  static void FillReadRequestFromQueue(ScriptState*,
                                       ReadableByteStreamController*,
                                       ReadRequest* read_request,
                                       ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-pull-into
  static void PullInto(ScriptState*,
                       ReadableByteStreamController*,
                       NotShared<DOMArrayBufferView> view,
                       ReadIntoRequest*,
                       ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-handle-queue-drain
  static void HandleQueueDrain(ScriptState*, ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#reset-queue
  static void ResetQueue(ReadableByteStreamController*);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-respond
  static void Respond(ScriptState*,
                      ReadableByteStreamController*,
                      size_t bytes_written,
                      ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-respond-in-closed-state
  static void RespondInClosedState(ScriptState*,
                                   ReadableByteStreamController*,
                                   PullIntoDescriptor* first_descriptor,
                                   ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-respond-in-readable-state
  static void RespondInReadableState(ScriptState*,
                                     ReadableByteStreamController*,
                                     size_t bytes_written,
                                     PullIntoDescriptor*,
                                     ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-respond-internal
  static void RespondInternal(ScriptState*,
                              ReadableByteStreamController*,
                              size_t bytes_written,
                              ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-byte-stream-controller-respond-with-new-view
  static void RespondWithNewView(ScriptState*,
                                 ReadableByteStreamController*,
                                 NotShared<DOMArrayBufferView> view,
                                 ExceptionState&);

  // https://streams.spec.whatwg.org/#can-transfer-array-buffer
  static bool CanTransferArrayBuffer(DOMArrayBuffer* buffer);

  // https://streams.spec.whatwg.org/#transfer-array-buffer
  static DOMArrayBuffer* TransferArrayBuffer(ScriptState*,
                                             DOMArrayBuffer* buffer,
                                             ExceptionState&);

  // https://streams.spec.whatwg.org/#rbs-controller-private-cancel
  ScriptPromise<IDLUndefined> CancelSteps(ScriptState*,
                                          v8::Local<v8::Value> reason) override;

  // https://streams.spec.whatwg.org/#rbs-controller-private-pull
  void PullSteps(ScriptState*, ReadRequest*, ExceptionState&) override;

  // https://streams.spec.whatwg.org/#abstract-opdef-readablebytestreamcontroller-releasesteps
  void ReleaseSteps() override;

  // autoAllocateChunkSize is encoded as 0 when it is undefined
  size_t auto_allocate_chunk_size_ = 0u;
  Member<ReadableStreamBYOBRequest> byob_request_;
  Member<StreamAlgorithm> cancel_algorithm_;
  bool close_requested_ = false;
  bool pull_again_ = false;
  Member<StreamAlgorithm> pull_algorithm_;
  bool pulling_ = false;
  HeapDeque<Member<PullIntoDescriptor>> pending_pull_intos_;
  HeapDeque<Member<QueueEntry>> queue_;
  double queue_total_size_;
  bool started_ = false;
  double strategy_high_water_mark_ = 0.0;
  Member<ReadableStream> controlled_readable_stream_;
};

template <>
struct DowncastTraits<ReadableByteStreamController> {
  static bool AllowFrom(const ReadableStreamController& controller) {
    return controller.IsByteStreamController();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_BYTE_STREAM_CONTROLLER_H_
