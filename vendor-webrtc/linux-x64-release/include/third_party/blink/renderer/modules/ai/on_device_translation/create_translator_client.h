// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_CREATE_TRANSLATOR_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_CREATE_TRANSLATOR_CLIENT_H_

#include "third_party/blink/public/mojom/on_device_translation/translation_manager.mojom-blink.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_translator_create_options.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/ai/ai_context_observer.h"
#include "third_party/blink/renderer/modules/ai/on_device_translation/translator.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"

namespace blink {

class CreateMonitor;

class CreateTranslatorClient
    : public GarbageCollected<CreateTranslatorClient>,
      public mojom::blink::TranslationManagerCreateTranslatorClient,
      public ExecutionContextClient,
      public AIContextObserver<Translator> {
 public:
  CreateTranslatorClient(ScriptState* script_state,
                         TranslatorCreateOptions* options,
                         ScriptPromiseResolver<Translator>* resolver);
  ~CreateTranslatorClient() override;

  CreateTranslatorClient(const CreateTranslatorClient&) = delete;
  CreateTranslatorClient& operator=(const CreateTranslatorClient&) = delete;

  void Trace(Visitor* visitor) const override;

  void OnResult(mojom::blink::CreateTranslatorResultPtr result) override;

  void OnGotAvailability(mojom::blink::CanCreateTranslatorResult result);

  void ResetReceiver() override;

 private:
  Member<CreateMonitor> monitor_;
  String source_language_;
  String target_language_;

  HeapMojoReceiver<mojom::blink::TranslationManagerCreateTranslatorClient,
                   CreateTranslatorClient>
      receiver_;
  scoped_refptr<base::SequencedTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_CREATE_TRANSLATOR_CLIENT_H_
