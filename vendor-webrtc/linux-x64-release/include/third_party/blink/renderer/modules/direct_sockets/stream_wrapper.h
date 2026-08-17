// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_STREAM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_STREAM_WRAPPER_H_

#include "third_party/blink/renderer/core/streams/readable_byte_stream_controller.h"
#include "third_party/blink/renderer/core/streams/readable_stream_default_controller_with_script_scope.h"
#include "third_party/blink/renderer/core/streams/underlying_byte_source_base.h"
#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/core/streams/underlying_source_base.h"
#include "third_party/blink/renderer/core/streams/writable_stream_default_controller.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;
class ReadableStream;
class WritableStream;
class ScriptValue;
class ScriptState;

class MODULES_EXPORT StreamWrapper : public GarbageCollectedMixin {
 public:
  using CloseOnceCallback =
      base::OnceCallback<void(v8::Local<v8::Value> exception, int net_error)>;

  enum class State { kOpen, kAborted, kClosed, kGracefullyClosing };

  explicit StreamWrapper(ScriptState*);
  virtual ~StreamWrapper();

  State GetState() const { return state_; }
  ScriptState* GetScriptState() const { return script_state_.Get(); }

  // Checks whether associated stream is locked to a reader/writer.
  virtual bool Locked() const = 0;

  virtual void CloseStream() = 0;
  virtual void ErrorStream(int32_t error_code) = 0;

  void Trace(Visitor* visitor) const override;

 protected:
  void SetState(State state) { state_ = state; }

 private:
  const Member<ScriptState> script_state_;
  State state_ = State::kOpen;
};

class ReadableStreamWrapper : public StreamWrapper {
 public:
  ReadableStream* Readable() const { return readable_.Get(); }

  // Checks whether |readable_| is locked to a reader.
  bool Locked() const override;

  // Implements UnderlyingSource::pull(...)
  virtual void Pull() = 0;

  void Trace(Visitor*) const override;

 protected:
  explicit ReadableStreamWrapper(ScriptState*);

  void SetReadable(ReadableStream* readable) { readable_ = readable; }

 private:
  Member<ReadableStream> readable_;
};

class ReadableStreamDefaultWrapper : public ReadableStreamWrapper {
 public:
  using ControllerType = ReadableStreamDefaultControllerWithScriptScope;

  void Trace(Visitor*) const override;

  ControllerType* Controller() const { return controller_.Get(); }
  void SetController(ControllerType* controller) { controller_ = controller; }

 protected:
  explicit ReadableStreamDefaultWrapper(ScriptState*);

  // Creates a specialized underlying source that forwards its method calls to
  // the provided |readable_stream_wrapper|.
  static UnderlyingSourceBase* MakeForwardingUnderlyingSource(
      ReadableStreamDefaultWrapper* readable_stream_wrapper);

  void SetSource(UnderlyingSourceBase* source) { source_ = source; }

 private:
  Member<UnderlyingSourceBase> source_;
  Member<ControllerType> controller_;
};

class ReadableByteStreamWrapper : public ReadableStreamWrapper {
 public:
  using ControllerType = ReadableByteStreamController;

  void Trace(Visitor*) const override;

  ControllerType* Controller() const { return controller_.Get(); }
  void SetController(ControllerType* controller) { controller_ = controller; }

 protected:
  explicit ReadableByteStreamWrapper(ScriptState*);

  // Creates a specialized underlying byte source that forwards its method calls
  // to the provided |readable_stream_wrapper|.
  static UnderlyingByteSourceBase* MakeForwardingUnderlyingByteSource(
      ReadableByteStreamWrapper* readable_stream_wrapper);

  void SetSource(UnderlyingByteSourceBase* source) { source_ = source; }

 private:
  Member<UnderlyingByteSourceBase> source_;
  Member<ControllerType> controller_;
};

class WritableStreamWrapper : public StreamWrapper {
 public:
  using ControllerType = WritableStreamDefaultController;

  WritableStream* Writable() const { return writable_.Get(); }

  // Checks whether |writable_| is locked to a writer.
  bool Locked() const override;

  // Checks whether there's a write in progress.
  virtual bool HasPendingWrite() const { return false; }

  // Intercepts signal from WritableStream::abort(...) and processes it out
  // of order (without waiting for queued writes to complete first).
  // Note that UnderlyingSink::abort(...) will be called right afterwards --
  // therefore normally it's sufficient to reject the pending promise (and the
  // rest will be handled by the controller).
  virtual void OnAbortSignal() = 0;

  // Implements UnderlyingSink::write(...)
  virtual ScriptPromise<IDLUndefined> Write(ScriptValue, ExceptionState&) = 0;

  ControllerType* Controller() const { return controller_.Get(); }
  void SetController(ControllerType* controller) { controller_ = controller; }

  void Trace(Visitor*) const override;

 protected:
  explicit WritableStreamWrapper(ScriptState*);

  // Creates a specialized underlying source that forwards its method calls to
  // the provided |writable_stream_wrapper|.
  static UnderlyingSinkBase* MakeForwardingUnderlyingSink(
      WritableStreamWrapper* writable_stream_wrapper);

  void SetSink(UnderlyingSinkBase* sink) { sink_ = sink; }
  void SetWritable(WritableStream* writable) { writable_ = writable; }

 private:
  Member<UnderlyingSinkBase> sink_;
  Member<WritableStream> writable_;
  Member<ControllerType> controller_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_DIRECT_SOCKETS_STREAM_WRAPPER_H_
