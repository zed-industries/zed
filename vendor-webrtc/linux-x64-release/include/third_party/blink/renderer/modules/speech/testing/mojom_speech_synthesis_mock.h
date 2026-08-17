/*
 * Copyright (C) 2013 Apple Computer, Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_TESTING_MOJOM_SPEECH_SYNTHESIS_MOCK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_TESTING_MOJOM_SPEECH_SYNTHESIS_MOCK_H_

#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/speech/speech_synthesis.mojom-blink.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class ExecutionContext;

class MojomSpeechSynthesisMock final : public mojom::blink::SpeechSynthesis {
 public:
  static mojo::PendingRemote<mojom::blink::SpeechSynthesis> Create(
      ExecutionContext*);

  // mojom::blink::SpeechSynthesis
  void AddVoiceListObserver(
      mojo::PendingRemote<mojom::blink::SpeechSynthesisVoiceListObserver>
          pending_observer) override;
  void Speak(mojom::blink::SpeechSynthesisUtterancePtr utterance,
             mojo::PendingRemote<mojom::blink::SpeechSynthesisClient>
                 pending_client) override;
  void Pause() override;
  void Resume() override;
  void Cancel() override;

 private:
  explicit MojomSpeechSynthesisMock(ExecutionContext*);
  ~MojomSpeechSynthesisMock() override;

  void SpeakNext();

  void SpeakingErrorOccurred(TimerBase*);
  void SpeakingFinished(TimerBase*);

  struct SpeechRequest {
    mojom::blink::SpeechSynthesisUtterancePtr utterance;
    mojo::PendingRemote<mojom::blink::SpeechSynthesisClient> pending_client;
  };

  TaskRunnerTimer<MojomSpeechSynthesisMock> speaking_error_occurred_timer_;
  TaskRunnerTimer<MojomSpeechSynthesisMock> speaking_finished_timer_;
  Vector<mojo::Remote<mojom::blink::SpeechSynthesisVoiceListObserver>>
      voice_list_observers_;
  mojom::blink::SpeechSynthesisUtterancePtr current_utterance_;
  mojo::Remote<mojom::blink::SpeechSynthesisClient> current_client_;
  Deque<SpeechRequest> queued_requests_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_TESTING_MOJOM_SPEECH_SYNTHESIS_MOCK_H_
