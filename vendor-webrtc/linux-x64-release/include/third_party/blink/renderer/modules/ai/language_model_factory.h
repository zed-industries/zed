// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_FACTORY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_FACTORY_H_

#include "base/task/sequenced_task_runner.h"
#include "base/types/expected.h"
#include "third_party/blink/public/mojom/ai/ai_language_model.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/ai/ai_language_model.mojom-blink.h"
#include "third_party/blink/public/mojom/ai/ai_manager.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_availability.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_model_create_options.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/ai/language_model_params.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class AI;
class LanguageModel;

// This class is responsible for creating LanguageModel instances.
class LanguageModelFactory final : public ScriptWrappable,
                                   public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit LanguageModelFactory(AI* ai);
  ~LanguageModelFactory() override = default;

  void Trace(Visitor* visitor) const override;

  // language_model_factory.idl implementation.
  ScriptPromise<LanguageModel> create(ScriptState* script_state,
                                      const LanguageModelCreateOptions* options,
                                      ExceptionState& exception_state);
  ScriptPromise<V8Availability> availability(
      ScriptState* script_state,
      const LanguageModelCreateCoreOptions* options,
      ExceptionState& exception_state);
  ScriptPromise<IDLNullable<LanguageModelParams>> params(
      ScriptState* script_state,
      ExceptionState& exception_state);

 private:
  void OnCanCreateLanguageModelComplete(
      ScriptPromiseResolver<V8Availability>* resolver,
      mojom::blink::ModelAvailabilityCheckResult check_result);
  void OnGetLanguageModelParamsComplete(
      ScriptPromiseResolver<IDLNullable<LanguageModelParams>>* resolver,
      mojom::blink::AILanguageModelParamsPtr language_model_params);

  Member<AI> ai_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_LANGUAGE_MODEL_FACTORY_H_
