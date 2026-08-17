// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_SUMMARIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_SUMMARIZER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/ai/ai_summarizer.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_summarizer_create_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_summarizer_summarize_options.h"
#include "third_party/blink/renderer/modules/ai/ai_writing_assistance_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// The class that represents a summarizer object.
class Summarizer final : public ScriptWrappable,
                         public AIWritingAssistanceBase<
                             Summarizer,
                             mojom::blink::AISummarizer,
                             mojom::blink::AIManagerCreateSummarizerClient,
                             SummarizerCreateCoreOptions,
                             SummarizerCreateOptions,
                             SummarizerSummarizeOptions> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Summarizer(ExecutionContext* execution_context,
             scoped_refptr<base::SequencedTaskRunner> task_runner,
             mojo::PendingRemote<mojom::blink::AISummarizer> pending_remote,
             SummarizerCreateOptions* options);
  void Trace(Visitor* visitor) const override;

  // AIWritingAssistanceBase:
  void remoteExecute(
      const String& input,
      const String& context,
      mojo::PendingRemote<blink::mojom::blink::ModelStreamingResponder>
          responder) override;

  // summarizer.idl:
  ScriptPromise<IDLString> summarize(ScriptState* script_state,
                                     const String& writing_task,
                                     const SummarizerSummarizeOptions* options,
                                     ExceptionState& exception_state);
  ReadableStream* summarizeStreaming(ScriptState* script_state,
                                     const String& writing_task,
                                     const SummarizerSummarizeOptions* options,
                                     ExceptionState& exception_state);
  ScriptPromise<IDLDouble> measureInputUsage(
      ScriptState* script_state,
      const String& input,
      const SummarizerSummarizeOptions* options,
      ExceptionState& exception_state);
  void destroy(ScriptState* script_state, ExceptionState& exception_state);

  V8SummarizerType type() const { return options_->type(); }
  V8SummarizerFormat format() const { return options_->format(); }
  V8SummarizerLength length() const { return options_->length(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_SUMMARIZER_H_
