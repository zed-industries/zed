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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTROLLER_H_

#include <optional>

#include "media/base/audio_parameters.h"
#include "media/mojo/mojom/speech_recognition_audio_forwarder.mojom-blink.h"
#include "media/mojo/mojom/speech_recognizer.mojom-blink.h"
#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/speech/speech_recognition_phrase_list.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_wrapper_mode.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_or_worker_scheduler.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class LocalDOMWindow;
class SpeechGrammarList;

class MODULES_EXPORT SpeechRecognitionController final
    : public GarbageCollected<SpeechRecognitionController>,
      public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  explicit SpeechRecognitionController(LocalDOMWindow&);
  ~SpeechRecognitionController();

  // Builds the speech recognition request params. If `audio_forwarder` and
  // `audio_parameters` are not defined, speech recognition will use audio from
  // the user's default audio input device instead. `on_device` defines whether
  // on-device speech recognition should be used. `allow_cloud_fallback` defines
  // whether a server-based speech recognition service may be used if on-device
  // speech recognition is not available.
  media::mojom::blink::StartSpeechRecognitionRequestParamsPtr
  BuildStartSpeechRecognitionRequestParams(
      mojo::PendingReceiver<media::mojom::blink::SpeechRecognitionSession>
          session_receiver,
      mojo::PendingRemote<media::mojom::blink::SpeechRecognitionSessionClient>
          session_client,
      const SpeechGrammarList& grammars,
      const SpeechRecognitionPhraseList* phrases,
      const String& lang,
      bool continuous,
      bool interim_results,
      uint32_t max_alternatives,
      bool on_device,
      bool allow_cloud_fallback,
      mojo::PendingReceiver<
          media::mojom::blink::SpeechRecognitionAudioForwarder>
          audio_forwarder = mojo::NullReceiver(),
      std::optional<media::AudioParameters> audio_parameters = std::nullopt);

  // Starts speech recognition with the given params.
  void Start(
      media::mojom::blink::StartSpeechRecognitionRequestParamsPtr params);

  void OnDeviceWebSpeechAvailable(
      const String& language,
      base::OnceCallback<void(media::mojom::blink::AvailabilityStatus)>
          callback);
  void InstallOnDeviceSpeechRecognition(
      const String& language,
      base::OnceCallback<void(bool)> callback);

  static SpeechRecognitionController* From(LocalDOMWindow&);

  void Trace(Visitor* visitor) const override;

 private:
  media::mojom::blink::SpeechRecognizer* GetSpeechRecognizer();
  media::mojom::blink::OnDeviceSpeechRecognition*
  GetOnDeviceSpeechRecognition();

  HeapMojoRemote<media::mojom::blink::SpeechRecognizer> speech_recognizer_;
  HeapMojoRemote<media::mojom::blink::OnDeviceSpeechRecognition>
      on_device_speech_recognition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_SPEECH_SPEECH_RECOGNITION_CONTROLLER_H_
