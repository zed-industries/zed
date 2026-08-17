// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_REWRITER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_REWRITER_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/ai/ai_rewriter.mojom-blink.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rewriter_create_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rewriter_rewrite_options.h"
#include "third_party/blink/renderer/modules/ai/ai_writing_assistance_base.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

// The class that represents a rewriter object.
class Rewriter final : public ScriptWrappable,
                       public AIWritingAssistanceBase<
                           Rewriter,
                           mojom::blink::AIRewriter,
                           mojom::blink::AIManagerCreateRewriterClient,
                           RewriterCreateCoreOptions,
                           RewriterCreateOptions,
                           RewriterRewriteOptions> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Rewriter(ExecutionContext* execution_context,
           scoped_refptr<base::SequencedTaskRunner> task_runner,
           mojo::PendingRemote<mojom::blink::AIRewriter> pending_remote,
           RewriterCreateOptions* options);
  void Trace(Visitor* visitor) const override;

  // AIWritingAssistanceBase:
  void remoteExecute(
      const String& input,
      const String& context,
      mojo::PendingRemote<blink::mojom::blink::ModelStreamingResponder>
          responder) override;

  // rewriter.idl:
  ScriptPromise<IDLString> rewrite(ScriptState* script_state,
                                   const String& input,
                                   const RewriterRewriteOptions* options,
                                   ExceptionState& exception_state);
  ReadableStream* rewriteStreaming(ScriptState* script_state,
                                   const String& input,
                                   const RewriterRewriteOptions* options,
                                   ExceptionState& exception_state);
  ScriptPromise<IDLDouble> measureInputUsage(
      ScriptState* script_state,
      const String& input,
      const RewriterRewriteOptions* options,
      ExceptionState& exception_state);
  void destroy(ScriptState* script_state, ExceptionState& exception_state);

  V8RewriterTone tone() const { return options_->tone(); }
  V8RewriterFormat format() const { return options_->format(); }
  V8RewriterLength length() const { return options_->length(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_REWRITER_H_
