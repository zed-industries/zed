// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_H_

#include "base/types/pass_key.h"
#include "third_party/blink/public/mojom/ai/ai_language_model.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_model_clone_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_model_expected_input.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_model_prompt_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_model_prompt_role.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/streams/readable_stream.h"
#include "third_party/blink/renderer/modules/ai/language_model_factory.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"

namespace blink {

// The class that represents a `LanguageModel` object.
class LanguageModel final : public EventTarget, public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // Get the mojo enum value for the given V8 `role` enum value.
  static mojom::blink::AILanguageModelPromptRole ConvertRoleToMojo(
      V8LanguageModelPromptRole role);

  LanguageModel(
      ExecutionContext* execution_context,
      mojo::PendingRemote<mojom::blink::AILanguageModel> pending_remote,
      scoped_refptr<base::SequencedTaskRunner> task_runner,
      mojom::blink::AILanguageModelInstanceInfoPtr info);
  ~LanguageModel() override = default;

  void Trace(Visitor* visitor) const override;

  // EventTarget implementation
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(quotaoverflow, kQuotaoverflow)

  // language_model.idl implementation.
  static ScriptPromise<LanguageModel> create(
      ScriptState* script_state,
      const LanguageModelCreateOptions* options,
      ExceptionState& exception_state);
  static ScriptPromise<V8Availability> availability(
      ScriptState* script_state,
      const LanguageModelCreateCoreOptions* options,
      ExceptionState& exception_state);
  static ScriptPromise<IDLNullable<LanguageModelParams>> params(
      ScriptState* script_state,
      ExceptionState& exception_state);

  ScriptPromise<IDLString> prompt(ScriptState* script_state,
                                  const V8LanguageModelPromptInput* input,
                                  const LanguageModelPromptOptions* options,
                                  ExceptionState& exception_state);
  ReadableStream* promptStreaming(ScriptState* script_state,
                                  const V8LanguageModelPromptInput* input,
                                  const LanguageModelPromptOptions* options,
                                  ExceptionState& exception_state);
  ScriptPromise<IDLDouble> measureInputUsage(
      ScriptState* script_state,
      const V8LanguageModelPromptInput* input,
      const LanguageModelPromptOptions* options,
      ExceptionState& exception_state);
  double inputQuota() const { return input_quota_; }
  double inputUsage() const { return input_usage_; }

  uint32_t topK() const { return top_k_; }
  float temperature() const { return temperature_; }

  ScriptPromise<LanguageModel> clone(ScriptState* script_state,
                                     const LanguageModelCloneOptions* options,
                                     ExceptionState& exception_state);
  void destroy(ScriptState* script_state, ExceptionState& exception_state);

  HeapMojoRemote<mojom::blink::AILanguageModel>& GetAILanguageModelRemote();
  scoped_refptr<base::SequencedTaskRunner> GetTaskRunner();

 private:
  void OnResponseComplete(
      mojom::blink::ModelExecutionContextInfoPtr context_info);
  void OnQuotaOverflow();

  uint64_t input_usage_;
  uint64_t input_quota_ = 0;
  uint32_t top_k_ = 0;
  float temperature_ = 0.0;
  // Prompt types supported by the language model in this session.
  WTF::HashSet<mojom::blink::AILanguageModelPromptType> input_types_;

  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  HeapMojoRemote<mojom::blink::AILanguageModel> language_model_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_H_
