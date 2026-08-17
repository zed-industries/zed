// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_DETECTION_LANGUAGE_DETECTION_MODEL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_DETECTION_LANGUAGE_DETECTION_MODEL_H_

#include "base/functional/callback_forward.h"
#include "base/memory/raw_ref.h"
#include "base/types/expected.h"
#include "components/language_detection/content/common/language_detection.mojom-blink.h"
#include "components/language_detection/core/language_detection_model.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

enum class DetectLanguageError {
  // 0 intentionally skipped.
  // The model was not available for use.
  kUnavailable = 1,
};

// Manages getting and holding a language detection model.
//
// Use this by calling `Create` and waiting for the callback to be called.
// If a model is returned, at that point it can be used to detect language.
class PLATFORM_EXPORT LanguageDetectionModel
    : public GarbageCollected<LanguageDetectionModel> {
 public:
  using MaybeModel =
      base::expected<LanguageDetectionModel*, DetectLanguageError>;
  using CreateLanguageDetectionModelCallback =
      base::OnceCallback<void(MaybeModel)>;

  LanguageDetectionModel() = default;

  // Loads the model file and passes this to `callback` when the model has been
  // loaded (or we know that it will fail to load). `interface_broker` can be
  // used to communicate with the browser to find the model.
  void LoadModelFile(base::File model_file,
                     CreateLanguageDetectionModelCallback callback);

  void Trace(Visitor* visitor) const;

  using LanguagePrediction = language_detection::Prediction;
  using DetectLanguageCallback = base::OnceCallback<void(
      base::expected<WTF::Vector<LanguagePrediction>, DetectLanguageError>)>;

  // Detects the language(s) used in the string using the TFLite model on the
  // whole string.
  //
  // This is an asynchronous operation. The result will be passed to the
  // `on_complete` callback.
  void DetectLanguage(scoped_refptr<base::SequencedTaskRunner>& task_runner,
                      const WTF::String& text,
                      DetectLanguageCallback on_complete);

  int64_t GetModelSize() const;

 private:
  void DetectLanguageImpl(const WTF::String& text,
                          DetectLanguageCallback on_complete);
  // This model is shared across all execution contexts in the process.
  language_detection::LanguageDetectionModel language_detection_model_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LANGUAGE_DETECTION_LANGUAGE_DETECTION_MODEL_H_
