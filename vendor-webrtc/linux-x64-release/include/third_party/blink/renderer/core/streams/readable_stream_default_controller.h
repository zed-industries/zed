// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_H_

#include <optional>

#include "third_party/blink/renderer/core/streams/readable_stream_controller.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class QueueWithSizes;
class ReadableStream;
class ReadRequest;
class ScriptState;
class ScriptValue;
class StrategySizeAlgorithm;
class StreamAlgorithm;
class StreamStartAlgorithm;

class ReadableStreamDefaultController : public ReadableStreamController {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ReadableStreamDefaultController(ScriptState*);

  // https://streams.spec.whatwg.org/#rs-default-controller-desired-size
  std::optional<double> desiredSize() const { return GetDesiredSize(); }

  // https://streams.spec.whatwg.org/#rs-default-controller-close
  void close(ScriptState*, ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-default-controller-enqueue
  void enqueue(ScriptState*, ExceptionState&);
  void enqueue(ScriptState*, ScriptValue chunk, ExceptionState&);

  // https://streams.spec.whatwg.org/#rs-default-controller-error
  void error(ScriptState*);
  void error(ScriptState*, ScriptValue e);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-close
  static void Close(ScriptState*, ReadableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-enqueue
  static void Enqueue(ScriptState*,
                      ReadableStreamDefaultController*,
                      v8::Local<v8::Value> chunk,
                      ExceptionState&);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-error
  static void Error(ScriptState*,
                    ReadableStreamDefaultController*,
                    v8::Local<v8::Value> e);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-get-desired-size
  std::optional<double> GetDesiredSize() const;

  //
  // Used by TransformStream
  //
  // https://streams.spec.whatwg.org/#readable-stream-default-controller-can-close-or-enqueue
  static bool CanCloseOrEnqueue(const ReadableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#rs-default-controller-has-backpressure
  static bool HasBackpressure(const ReadableStreamDefaultController*);

  static const char* EnqueueExceptionMessage(
      const ReadableStreamDefaultController*);

  bool IsDefaultController() const override { return true; }
  bool IsByteStreamController() const override { return false; }

  void Trace(Visitor*) const override;

  // https://streams.spec.whatwg.org/#rs-default-controller-private-cancel
  ScriptPromise<IDLUndefined> CancelSteps(ScriptState*,
                                          v8::Local<v8::Value> reason) override;

  // https://streams.spec.whatwg.org/#rs-default-controller-private-pull
  void PullSteps(ScriptState*, ReadRequest*, ExceptionState&) override;

  // https://streams.spec.whatwg.org/#abstract-opdef-readablestreamdefaultcontroller-releasesteps
  void ReleaseSteps() override;

 private:
  friend class ReadableStream;
  friend class ReadableStreamDefaultReader;

  class CallPullIfNeededResolveFunction;
  class CallPullIfNeededRejectFunction;

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-call-pull-if-needed
  static void CallPullIfNeeded(ScriptState*, ReadableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-should-call-pull
  static bool ShouldCallPull(const ReadableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#readable-stream-default-controller-clear-algorithms
  static void ClearAlgorithms(ReadableStreamDefaultController*);

  // https://streams.spec.whatwg.org/#set-up-readable-stream-default-controller
  static void SetUp(ScriptState*,
                    ReadableStream*,
                    ReadableStreamDefaultController*,
                    StreamStartAlgorithm* start_algorithm,
                    StreamAlgorithm* pull_algorithm,
                    StreamAlgorithm* cancel_algorithm,
                    double high_water_mark,
                    StrategySizeAlgorithm* size_algorithm,
                    ExceptionState&);

  // https://streams.spec.whatwg.org/#set-up-readable-stream-default-controller-from-underlying-source
  static void SetUpFromUnderlyingSource(ScriptState*,
                                        ReadableStream*,
                                        v8::Local<v8::Object> underlying_source,
                                        double high_water_mark,
                                        StrategySizeAlgorithm* size_algorithm,
                                        ExceptionState&);

  // Boolean flags are grouped together to reduce object size. Verbs have been
  // added to the names in the standard to match Blink style.
  bool is_close_requested_ = false;
  bool will_pull_again_ = false;
  bool is_pulling_ = false;
  bool is_started_ = false;
  Member<StreamAlgorithm> cancel_algorithm_;
  Member<ReadableStream> controlled_readable_stream_;
  Member<StreamAlgorithm> pull_algorithm_;
  Member<QueueWithSizes> queue_;
  double strategy_high_water_mark_ = 0.0;
  Member<StrategySizeAlgorithm> strategy_size_algorithm_;
  Member<CallPullIfNeededResolveFunction> resolve_function_;
  Member<CallPullIfNeededRejectFunction> reject_function_;
};

template <>
struct DowncastTraits<ReadableStreamDefaultController> {
  static bool AllowFrom(const ReadableStreamController& controller) {
    return controller.IsDefaultController();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_READABLE_STREAM_DEFAULT_CONTROLLER_H_
