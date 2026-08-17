// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODER_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODER_BASE_H_

#include <memory>

#include "media/base/encoder_status.h"
#include "media/base/media_log.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_codec_state.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_webcodecs_error_callback.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webcodecs/codec_logger.h"
#include "third_party/blink/renderer/modules/webcodecs/codec_trace_names.h"
#include "third_party/blink/renderer/modules/webcodecs/reclaimable_codec.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class ExceptionState;
enum class DOMExceptionCode;

template <typename Traits>
class MODULES_EXPORT EncoderBase
    : public EventTarget,
      public ActiveScriptWrappable<EncoderBase<Traits>>,
      public ReclaimableCodec {
 public:
  using InitType = typename Traits::Init;
  using ConfigType = typename Traits::Config;
  using InternalConfigType = typename Traits::InternalConfig;
  using InputType = typename Traits::Input;
  using EncodeOptionsType = typename Traits::EncodeOptions;
  using OutputChunkType = typename Traits::OutputChunk;
  using OutputCallbackType = typename Traits::OutputCallback;
  using MediaEncoderType = typename Traits::MediaEncoder;

  static const CodecTraceNames* GetTraceNames();

  EncoderBase(ScriptState*, const InitType*, ExceptionState&);
  ~EncoderBase() override;

  // *_encoder.idl implementation.
  uint32_t encodeQueueSize() { return requested_encodes_; }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(dequeue, kDequeue)

  void configure(const ConfigType*, ExceptionState&);

  void encode(InputType* input,
              const EncodeOptionsType* opts,
              ExceptionState& exception_state);

  ScriptPromise<IDLUndefined> flush(ExceptionState&);

  void reset(ExceptionState&);

  void close(ExceptionState&);

  V8CodecState state() { return state_; }

  // EventTarget override.
  ExecutionContext* GetExecutionContext() const override;

  // ExecutionContextLifecycleObserver override.
  void ContextDestroyed() override;

  // ScriptWrappable override.
  bool HasPendingActivity() const override;

  // GarbageCollected override.
  void Trace(Visitor*) const override;

 protected:
  struct Request final : public GarbageCollected<Request> {
    enum class Type {
      // Configure an encoder from scratch, possibly replacing the existing one.
      kConfigure,
      // Adjust options in the already configured encoder.
      kReconfigure,
      kEncode,
      kFlush,
    };

    void Trace(Visitor*) const;

    // Starts an async trace event.
    void StartTracing();

    // Starts an async encode trace.
    void StartTracingVideoEncode(bool is_keyframe, base::TimeDelta timestamp);

    // Ends the async trace event associated with |this|.
    void EndTracing(bool aborted = false);

    // Get a trace event name from DecoderTemplate::GetTraceNames() and |type|.
    const char* TraceNameFromType();

    Type type;
    // Current value of EncoderBase.reset_count_ when request was created.
    uint32_t reset_count = 0;
    Member<InputType> input;                     // used by kEncode
    Member<const EncodeOptionsType> encodeOpts;  // used by kEncode
    // used by kFlush
    Member<ScriptPromiseResolver<IDLUndefined>> resolver;
    Member<InternalConfigType> config;  // used by kConfigure and kReconfigure

#if DCHECK_IS_ON()
    // Tracks the state of tracing for debug purposes.
    bool is_tracing;
#endif
  };

  void QueueHandleError(DOMException* ex);
  virtual void HandleError(DOMException* ex);
  virtual void EnqueueRequest(Request* request);
  virtual void ProcessRequests();
  virtual bool ReadyToProcessNextRequest();
  virtual void ProcessEncode(Request* request) = 0;
  virtual void ProcessConfigure(Request* request) = 0;
  virtual void ProcessReconfigure(Request* request) = 0;
  virtual void ProcessFlush(Request* request);
  virtual void ResetInternal(DOMException* ex);

  virtual bool CanReconfigure(InternalConfigType& original_config,
                              InternalConfigType& new_config) = 0;
  virtual InternalConfigType* OnNewConfigure(const ConfigType*,
                                             ExceptionState&) = 0;
  virtual bool VerifyCodecSupport(InternalConfigType*,
                                  String* js_error_message) = 0;

  // Called for each new input passed to encode(). If an exception is thrown on
  // `exception_state` the encode() call will be aborted and the exception will
  // be passed back to the caller.
  virtual void OnNewEncode(InputType* input,
                           ExceptionState& exception_state) = 0;

  // ReclaimableCodec implementation.
  void OnCodecReclaimed(DOMException*) override;

  void TraceQueueSizes() const;

  void ScheduleDequeueEvent();
  void DispatchDequeueEvent(Event* event);
  bool dequeue_event_pending_ = false;

  std::unique_ptr<CodecLogger<media::EncoderStatus>> logger_;

  std::unique_ptr<MediaEncoderType> media_encoder_;

  V8CodecState state_;

  Member<InternalConfigType> active_config_;
  Member<ScriptState> script_state_;
  Member<OutputCallbackType> output_callback_;
  Member<V8WebCodecsErrorCallback> error_callback_;
  HeapDeque<Member<Request>> requests_;
  uint32_t requested_encodes_ = 0;

  // How many times reset() was called on the encoder. It's used to decide
  // when a callback needs to be dismissed because reset() was called between
  // an operation and its callback.
  uint32_t reset_count_ = 0;

  // Some kConfigure and kFlush requests can't be executed in parallel with
  // kEncode. Even some kEncode might have synchronous parts like readback.
  //
  // Set this to stop processing of new requests in `requests_` until the
  // current request is finished.
  //
  // During reset(), this member is used to reject any pending promises.
  Member<Request> blocking_request_in_progress_;

  bool first_output_after_configure_ = true;

  // Used to differentiate Encoders' counters during tracing.
  int trace_counter_id_;

  // A runner for callbacks and deleting objects.
  scoped_refptr<base::SingleThreadTaskRunner> callback_runner_;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_ENCODER_BASE_H_
