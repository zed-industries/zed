/*
 * Copyright (C) 2013 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_EVENT_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_speech_synthesis_event_init.h"
#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/modules/speech/speech_synthesis_utterance.h"

namespace blink {

class SpeechSynthesisEvent : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static SpeechSynthesisEvent* Create(const AtomicString& type,
                                      const SpeechSynthesisEventInit* init);

  SpeechSynthesisEvent(const AtomicString& type,
                       SpeechSynthesisUtterance*,
                       unsigned char_index,
                       unsigned char_length,
                       float elapsed_time,
                       const String& name);

  SpeechSynthesisUtterance* utterance() const { return utterance_.Get(); }
  unsigned charIndex() const { return char_index_; }
  unsigned charLength() const { return char_length_; }
  float elapsedTime() const { return elapsed_time_; }
  const String& name() const { return name_; }

  const AtomicString& InterfaceName() const override {
    return event_interface_names::kSpeechSynthesisEvent;
  }

  void Trace(Visitor*) const override;

 private:
  Member<SpeechSynthesisUtterance> utterance_;
  unsigned char_index_;
  unsigned char_length_;
  float elapsed_time_;
  String name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_EVENT_H_
