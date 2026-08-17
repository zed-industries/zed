// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_LANGUAGE_DETECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_LANGUAGE_DETECTOR_H_

#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_availability.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_detection_result.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_detector_create_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_language_detector_detect_options.h"
#include "third_party/blink/renderer/modules/ai/on_device_translation/resolver_with_abort_signal.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/language_detection/language_detection_model.h"

namespace blink {

class LanguageDetector final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ScriptPromise<V8Availability> availability(
      ScriptState* script_state,
      LanguageDetectorCreateCoreOptions* options,
      ExceptionState& exception_state);

  static ScriptPromise<LanguageDetector> create(
      ScriptState* script_state,
      LanguageDetectorCreateOptions* options,
      ExceptionState& exception_state);

  LanguageDetector(ScriptState* script_state,
                   LanguageDetectionModel* language_detection_model,
                   AbortSignal* create_abort_signal,
                   std::optional<Vector<String>> expected_input_languages,
                   scoped_refptr<base::SequencedTaskRunner>& task_runner);
  ~LanguageDetector() override = default;

  void Trace(Visitor* visitor) const override;

  ScriptPromise<IDLSequence<LanguageDetectionResult>> detect(
      ScriptState* script_state,
      const WTF::String& input,
      LanguageDetectorDetectOptions* options,
      ExceptionState& exception_state);
  void destroy(ScriptState* script_state);

  ScriptPromise<IDLDouble> measureInputUsage(
      ScriptState* script_state,
      const WTF::String& input,
      LanguageDetectorDetectOptions* options,
      ExceptionState& exception_state);

  double inputQuota() const;

  const std::optional<Vector<String>>& expectedInputLanguages() const {
    return expected_input_languages_;
  }

  // TODO(crbug.com/349927087): Make the functions below free functions.
  static HeapVector<Member<LanguageDetectionResult>> ConvertResult(
      WTF::Vector<LanguageDetectionModel::LanguagePrediction> predictions);
  static void OnDetectComplete(
      ResolverWithAbortSignal<IDLSequence<LanguageDetectionResult>>* resolver,
      base::expected<WTF::Vector<LanguageDetectionModel::LanguagePrediction>,
                     DetectLanguageError> result);

 private:
  void DestroyImpl();

  void OnCreateAbortSignalAborted(ScriptState* script_state);

  AbortSignal* CreateCompositeSignal(ScriptState* script_state,
                                     LanguageDetectorDetectOptions* options);

  scoped_refptr<base::SequencedTaskRunner> task_runner_;
  Member<LanguageDetectionModel> language_detection_model_;
  Member<AbortController> destruction_abort_controller_;
  Member<AbortSignal> create_abort_signal_;
  Member<AbortSignal::AlgorithmHandle> create_abort_handle_;
  std::optional<Vector<String>> expected_input_languages_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_AI_ON_DEVICE_TRANSLATION_LANGUAGE_DETECTOR_H_
