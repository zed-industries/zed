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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_H_

#include "third_party/blink/public/mojom/speech/speech_synthesis.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/speech/speech_synthesis_base.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/speech/speech_synthesis_utterance.h"
#include "third_party/blink/renderer/modules/speech/speech_synthesis_voice.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;

class MODULES_EXPORT SpeechSynthesis final
    : public EventTarget,
      public SpeechSynthesisBase,
      public Supplement<LocalDOMWindow>,
      public mojom::blink::SpeechSynthesisVoiceListObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];

  static SpeechSynthesisBase* Create(LocalDOMWindow&);
  static SpeechSynthesis* speechSynthesis(LocalDOMWindow&);
  static void CreateForTesting(
      LocalDOMWindow&,
      mojo::PendingRemote<mojom::blink::SpeechSynthesis>);

  explicit SpeechSynthesis(LocalDOMWindow&);

  bool pending() const;
  bool speaking() const { return Speaking(); }
  bool paused() const;

  // SpeechSynthesisBase
  void Speak(const String&, const String&) override;
  void Cancel() override;
  void Pause() override;
  void Resume() override;
  bool Speaking() const override;

  void speak(ScriptState*, SpeechSynthesisUtterance*);
  void cancel() { Cancel(); }
  void pause() { Pause(); }
  void resume() { Resume(); }

  const HeapVector<Member<SpeechSynthesisVoice>>& getVoices();

  DEFINE_ATTRIBUTE_EVENT_LISTENER(voiceschanged, kVoiceschanged)

  ExecutionContext* GetExecutionContext() const override;

  // GarbageCollected
  void Trace(Visitor*) const override;

  // mojom::blink::SpeechSynthesisVoiceListObserver
  void OnSetVoiceList(
      Vector<mojom::blink::SpeechSynthesisVoicePtr> voices) override;

  // These methods are called by SpeechSynthesisUtterance:
  void DidStartSpeaking(SpeechSynthesisUtterance*);
  void DidPauseSpeaking(SpeechSynthesisUtterance*);
  void DidResumeSpeaking(SpeechSynthesisUtterance*);
  void DidFinishSpeaking(SpeechSynthesisUtterance*,
                         mojom::blink::SpeechSynthesisErrorCode);
  void SpeakingErrorOccurred(SpeechSynthesisUtterance*);
  void WordBoundaryEventOccurred(SpeechSynthesisUtterance*,
                                 unsigned char_index,
                                 unsigned char_length);
  void SentenceBoundaryEventOccurred(SpeechSynthesisUtterance*,
                                     unsigned char_index,
                                     unsigned char_length);

  mojom::blink::SpeechSynthesis* MojomSynthesis() {
    return mojom_synthesis_.get();
  }

 private:
  void VoicesDidChange();
  void StartSpeakingImmediately();
  void HandleSpeakingCompleted(
      SpeechSynthesisUtterance*,
      mojom::blink::SpeechSynthesisErrorCode error_code);
  void FireEvent(const AtomicString& type,
                 SpeechSynthesisUtterance*,
                 uint32_t char_index,
                 uint32_t char_length,
                 const String& name);

  void FireErrorEvent(SpeechSynthesisUtterance*,
                      uint32_t char_index,
                      const String& error);

  // Returns the utterance at the front of the queue.
  SpeechSynthesisUtterance* CurrentSpeechUtterance() const;

  // Gets a timestamp in millis that is safe to expose to the web.
  // Returns false if it cannot get a timestamp.
  bool GetElapsedTimeMillis(double* millis);

  bool IsAllowedToStartByAutoplay() const;

  void RecordVoicesForIdentifiability() const;

  void SetMojomSynthesisForTesting(
      mojo::PendingRemote<mojom::blink::SpeechSynthesis>);
  mojom::blink::SpeechSynthesis* TryEnsureMojomSynthesis();

  HeapMojoReceiver<mojom::blink::SpeechSynthesisVoiceListObserver,
                   SpeechSynthesis>
      receiver_;
  HeapMojoRemote<mojom::blink::SpeechSynthesis> mojom_synthesis_;
  HeapVector<Member<SpeechSynthesisVoice>> voice_list_;
  HeapDeque<Member<SpeechSynthesisUtterance>> utterance_queue_;
  bool is_paused_ = false;

  // EventTarget
  const AtomicString& InterfaceName() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_SYNTHESIS_H_
