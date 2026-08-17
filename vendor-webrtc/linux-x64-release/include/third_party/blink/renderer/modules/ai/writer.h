// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_WRITER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_WRITER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/ai/ai_writer.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_writer_create_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_writer_write_options.h"
#include "third_party/blink/renderer/modules/ai/ai_writing_assistance_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// The class that represents a writer object.
class Writer final
    : public ScriptWrappable,
      public AIWritingAssistanceBase<Writer,
                                     mojom::blink::AIWriter,
                                     mojom::blink::AIManagerCreateWriterClient,
                                     WriterCreateCoreOptions,
                                     WriterCreateOptions,
                                     WriterWriteOptions> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Writer(ExecutionContext* execution_context,
         scoped_refptr<base::SequencedTaskRunner> task_runner,
         mojo::PendingRemote<mojom::blink::AIWriter> pending_remote,
         WriterCreateOptions* options);
  void Trace(Visitor* visitor) const override;

  // AIWritingAssistanceBase:
  void remoteExecute(
      const String& input,
      const String& context,
      mojo::PendingRemote<blink::mojom::blink::ModelStreamingResponder>
          responder) override;

  // writer.idl:
  ScriptPromise<IDLString> write(ScriptState* script_state,
                                 const String& writing_task,
                                 const WriterWriteOptions* options,
                                 ExceptionState& exception_state);
  ReadableStream* writeStreaming(ScriptState* script_state,
                                 const String& writing_task,
                                 const WriterWriteOptions* options,
                                 ExceptionState& exception_state);
  ScriptPromise<IDLDouble> measureInputUsage(ScriptState* script_state,
                                             const String& input,
                                             const WriterWriteOptions* options,
                                             ExceptionState& exception_state);
  void destroy(ScriptState* script_state, ExceptionState& exception_state);

  V8WriterTone tone() const { return options_->tone(); }
  V8WriterFormat format() const { return options_->format(); }
  V8WriterLength length() const { return options_->length(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_WRITER_H_
