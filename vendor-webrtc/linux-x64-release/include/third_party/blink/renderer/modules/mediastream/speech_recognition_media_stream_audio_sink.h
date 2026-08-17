// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SPEECH_RECOGNITION_MEDIA_STREAM_AUDIO_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SPEECH_RECOGNITION_MEDIA_STREAM_AUDIO_SINK_H_

#include <memory>

#include "base/functional/callback.h"
#include "base/sequence_checker.h"
#include "media/base/audio_bus.h"
#include "media/base/audio_bus_pool.h"
#include "media/base/audio_parameters.h"
#include "media/base/channel_mixer.h"
#include "media/mojo/mojom/speech_recognition.mojom-blink.h"
#include "media/mojo/mojom/speech_recognition_audio_forwarder.mojom-blink.h"
#include "media/mojo/mojom/speech_recognizer.mojom-blink.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_handle.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class ExecutionContext;

using StartRecognitionCallback = base::OnceCallback<void(
    std::optional<media::AudioParameters> audio_parameters,
    mojo::PendingReceiver<media::mojom::blink::SpeechRecognitionAudioForwarder>
        audio_forwarder_receiver)>;

// Class used to extract the raw audio from the media stream for use with the
// Web Speech API. Raw audio is extracted from the MediaStreamAudioSink and
// forwarded via the SpeechRecognitionAudioForwarder interface for speech
// recognition. `OnData()` is called on the real-time audio thread, but audio is
// forwarded on the main thread.
class MODULES_EXPORT SpeechRecognitionMediaStreamAudioSink final
    : public GarbageCollected<SpeechRecognitionMediaStreamAudioSink>,
      public WebMediaStreamAudioSink {
 public:
  using SendAudioToSpeechRecognitionServiceCallback =
      base::RepeatingCallback<void(
          media::mojom::blink::AudioDataS16Ptr audio_data)>;

  SpeechRecognitionMediaStreamAudioSink(
      ExecutionContext* context,
      StartRecognitionCallback start_recognition_callback);

  // WebMediaStreamAudioSink implementation. Called on the real-time audio
  // thread.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks estimated_capture_time) override;
  void OnSetFormat(const media::AudioParameters& audio_parameters) override;

  void Trace(Visitor*) const;

 private:
  // Runs the callback to start the speech recognition controller on the main
  // thread.
  void ReconfigureAndMaybeStartRecognitionOnMainThread(
      const media::AudioParameters& audio_parameters,
      std::unique_ptr<media::AudioBusPoolImpl> old_audio_bus_pool);

  void SendAudio(std::unique_ptr<media::AudioBus> audio_data,
                 media::AudioBusPoolImpl* audio_bus_pool);

  media::mojom::blink::AudioDataS16Ptr ConvertToAudioDataS16(
      const media::AudioBus& audio_data,
      int sample_rate,
      media::ChannelLayout channel_layout);

  // Resets the temporary monaural audio bus and the channel mixer used to
  // combine multiple audio channels.
  void ResetChannelMixerIfNeeded(int frame_count,
                                 media::ChannelLayout channel_layout,
                                 int channel_count);

  std::unique_ptr<media::AudioBusPoolImpl> audio_bus_pool_;

  std::unique_ptr<media::ChannelMixer> channel_mixer_
      GUARDED_BY_CONTEXT(main_sequence_checker_);

  // The layout used to instantiate the channel mixer.
  media::ChannelLayout channel_layout_ GUARDED_BY_CONTEXT(
      main_sequence_checker_) = media::ChannelLayout::CHANNEL_LAYOUT_NONE;

  // The number of channels of the audio output.
  int channel_count_ GUARDED_BY_CONTEXT(main_sequence_checker_) = 0;

  // The temporary audio bus used to mix multichannel audio into a single
  // channel.
  std::unique_ptr<media::AudioBus> monaural_audio_bus_
      GUARDED_BY_CONTEXT(main_sequence_checker_);

  HeapMojoRemote<media::mojom::blink::SpeechRecognitionAudioForwarder>
      audio_forwarder_;

  media::AudioParameters audio_parameters_
      GUARDED_BY_CONTEXT(main_sequence_checker_);

  StartRecognitionCallback start_recognition_callback_
      GUARDED_BY_CONTEXT(main_sequence_checker_);

  scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner_;

  SEQUENCE_CHECKER(main_sequence_checker_);

  CrossThreadWeakHandle<SpeechRecognitionMediaStreamAudioSink> weak_handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_SPEECH_RECOGNITION_MEDIA_STREAM_AUDIO_SINK_H_
