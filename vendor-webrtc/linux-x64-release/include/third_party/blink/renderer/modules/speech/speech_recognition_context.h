// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTEXT_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/speech/speech_recognition_phrase_list.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class MODULES_EXPORT SpeechRecognitionContext final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SpeechRecognitionContext* Create(SpeechRecognitionPhraseList* phrases);

  explicit SpeechRecognitionContext(SpeechRecognitionPhraseList* phrases);
  ~SpeechRecognitionContext() override = default;

  // ScriptWrappable:
  void Trace(Visitor* visitor) const override;

  // SpeechRecognitionContext:
  SpeechRecognitionPhraseList* phrases() const { return phrases_.Get(); }
  void setPhrases(SpeechRecognitionPhraseList* phrases) { phrases_ = phrases; }

 private:
  Member<SpeechRecognitionPhraseList> phrases_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTEXT_H_
