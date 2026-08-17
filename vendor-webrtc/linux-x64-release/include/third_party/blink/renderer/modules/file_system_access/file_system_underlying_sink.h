// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_UNDERLYING_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_UNDERLYING_SINK_H_

#include "third_party/blink/public/mojom/file_system_access/file_system_access_file_writer.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/streams/underlying_sink_base.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExceptionState;
class V8UnionArrayBufferOrArrayBufferViewOrBlobOrUSVString;
class WriteParams;

class FileSystemUnderlyingSink final : public UnderlyingSinkBase {
 public:
  explicit FileSystemUnderlyingSink(
      ExecutionContext*,
      mojo::PendingRemote<mojom::blink::FileSystemAccessFileWriter>);

  // UnderlyingSinkBase
  ScriptPromise<IDLUndefined> start(ScriptState*,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> write(ScriptState*,
                                    ScriptValue chunk,
                                    WritableStreamDefaultController*,
                                    ExceptionState&) override;
  ScriptPromise<IDLUndefined> close(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> abort(ScriptState*,
                                    ScriptValue reason,
                                    ExceptionState&) override;

  void Trace(Visitor*) const override;

 private:
  // Helpers which ensure `writer_remote_` is reset when there's an error.
  // A WritableStream becomes unusable once there's been an error on the stream.
  // Resetting the remote destroys the corresponding receiver, thereby releasing
  // any locks tied to the writer.
  void ThrowDOMExceptionAndInvalidateSink(ExceptionState& exception_state,
                                          DOMExceptionCode error,
                                          const char* message);
  void ThrowTypeErrorAndInvalidateSink(ExceptionState& exception_state,
                                       const char* message);

  ScriptPromise<IDLUndefined> HandleParams(ScriptState*,
                                           const WriteParams&,
                                           ExceptionState&);
  ScriptPromise<IDLUndefined> WriteData(
      ScriptState*,
      uint64_t position,
      const V8UnionArrayBufferOrArrayBufferViewOrBlobOrUSVString* data,
      ExceptionState&);
  ScriptPromise<IDLUndefined> Truncate(ScriptState*,
                                       uint64_t size,
                                       ExceptionState&);
  ScriptPromise<IDLUndefined> Seek(ScriptState*,
                                   uint64_t offset,
                                   ExceptionState&);
  void WriteComplete(mojom::blink::FileSystemAccessErrorPtr result,
                     uint64_t bytes_written);
  void TruncateComplete(uint64_t to_size,
                        mojom::blink::FileSystemAccessErrorPtr result);
  void CloseComplete(mojom::blink::FileSystemAccessErrorPtr result);

  HeapMojoRemote<mojom::blink::FileSystemAccessFileWriter> writer_remote_;

  uint64_t offset_ = 0;
  Member<ScriptPromiseResolver<IDLUndefined>> pending_operation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_UNDERLYING_SINK_H_
