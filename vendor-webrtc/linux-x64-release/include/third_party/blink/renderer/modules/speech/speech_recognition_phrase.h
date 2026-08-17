// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_PHRASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_PHRASE_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MODULES_EXPORT SpeechRecognitionPhrase final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SpeechRecognitionPhrase* Create(
      const WTF::String& phrase,
      float boost = 1.0,
      ExceptionState& exception_state = ASSERT_NO_EXCEPTION);

  explicit SpeechRecognitionPhrase(const WTF::String& phrase,
                                   float boost = 1.0);
  ~SpeechRecognitionPhrase() override = default;

  WTF::String& phrase() { return phrase_; }
  float boost() { return boost_; }

 private:
  WTF::String phrase_;
  float boost_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_PHRASE_H_
