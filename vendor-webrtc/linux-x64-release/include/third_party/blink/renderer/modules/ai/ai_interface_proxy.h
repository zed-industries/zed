// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_INTERFACE_PROXY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_INTERFACE_PROXY_H_

#include "components/language_detection/content/common/language_detection.mojom-blink.h"
#include "third_party/blink/public/mojom/ai/ai_manager.mojom-blink.h"
#include "third_party/blink/public/mojom/on_device_translation/translation_manager.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/language_detection/language_detection_model.h"

namespace blink {

// Provides static getters to browser interfaces for the built-in AI APIs.
class AIInterfaceProxy final : public GarbageCollected<AIInterfaceProxy>,
                               public Supplement<ExecutionContext> {
 public:
  static const char kSupplementName[];

  using GetLanguageDetectionModelStatusCallback = base::OnceCallback<void(
      language_detection::mojom::blink::LanguageDetectionModelStatus)>;

  using GetLanguageDetectionModelCallback = base::OnceCallback<void(
      base::expected<LanguageDetectionModel*, DetectLanguageError>)>;

  explicit AIInterfaceProxy(ExecutionContext* execution_context);
  ~AIInterfaceProxy();

  // Not copyable or movable
  AIInterfaceProxy(const AIInterfaceProxy&) = delete;
  AIInterfaceProxy& operator=(const AIInterfaceProxy&) = delete;

  void Trace(Visitor* visitor) const override;

  static scoped_refptr<base::SequencedTaskRunner> GetTaskRunner(
      ExecutionContext* execution_context);

  static HeapMojoRemote<mojom::blink::TranslationManager>&
  GetTranslationManagerRemote(ExecutionContext* execution_context);

  static void GetLanguageDetectionModelStatus(
      ExecutionContext* execution_context,
      GetLanguageDetectionModelStatusCallback callback);

  static void GetLanguageDetectionModel(
      ExecutionContext* execution_context,
      GetLanguageDetectionModelCallback callback);

  static HeapMojoRemote<mojom::blink::AIManager>& GetAIManagerRemote(
      ExecutionContext* execution_context);

 private:
  static AIInterfaceProxy* From(ExecutionContext* execution_context);

  HeapMojoRemote<mojom::blink::TranslationManager>&
  GetTranslationManagerRemoteImpl(ExecutionContext* execution_context);

  HeapMojoRemote<
      language_detection::mojom::blink::ContentLanguageDetectionDriver>&
  GetLanguageDetectionDriverRemote(ExecutionContext* execution_context);

  void GetLanguageDetectionModelImpl(
      ExecutionContext* execution_context,
      GetLanguageDetectionModelCallback callback);

  HeapMojoRemote<mojom::blink::AIManager>& GetAIManagerRemoteImpl(
      ExecutionContext* execution_context);

  scoped_refptr<base::SequencedTaskRunner> task_runner_;

  HeapMojoRemote<mojom::blink::TranslationManager> translation_manager_remote_{
      nullptr};

  HeapMojoRemote<
      language_detection::mojom::blink::ContentLanguageDetectionDriver>
      language_detection_driver_{nullptr};

  // TODO(crbug.com/406770758): Consider updating ownership of
  // `language_detection_model_` to the `LanguageDetectorCreate` class.
  Member<LanguageDetectionModel> language_detection_model_;

  HeapMojoRemote<mojom::blink::AIManager> ai_manager_remote_{nullptr};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_AI_INTERFACE_PROXY_H_
