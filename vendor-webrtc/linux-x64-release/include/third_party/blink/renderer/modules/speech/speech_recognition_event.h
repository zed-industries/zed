/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *  * Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 *  * Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_speech_recognition_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/speech/speech_recognition_result.h"
#include "third_party/blink/renderer/modules/speech/speech_recognition_result_list.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SpeechRecognitionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SpeechRecognitionEvent* Create(const AtomicString&,
                                        const SpeechRecognitionEventInit*);

  SpeechRecognitionEvent(const AtomicString&,
                         const SpeechRecognitionEventInit*);
  SpeechRecognitionEvent(const AtomicString& event_name,
                         uint32_t result_index,
                         SpeechRecognitionResultList* results);
  ~SpeechRecognitionEvent() override;

  static SpeechRecognitionEvent* CreateResult(
      uint32_t result_index,
      const HeapVector<Member<SpeechRecognitionResult>>& results);
  static SpeechRecognitionEvent* CreateNoMatch(SpeechRecognitionResult*);

  uint32_t resultIndex() const { return result_index_; }
  SpeechRecognitionResultList* results() const { return results_.Get(); }

  // Event
  const AtomicString& InterfaceName() const override;

  void Trace(Visitor*) const override;

 private:
  uint32_t result_index_;
  Member<SpeechRecognitionResultList> results_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_EVENT_H_
