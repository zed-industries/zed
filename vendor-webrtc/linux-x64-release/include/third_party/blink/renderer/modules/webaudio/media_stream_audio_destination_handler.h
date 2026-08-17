// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_DESTINATION_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_DESTINATION_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_handler.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"

namespace blink {

class AudioNode;
class ExceptionState;
class AudioNodeInput;
class WebAudioDestinationConsumer;

// MediaStreamAudioDestinationHandler is an AudioHandler that manages the audio
// data flow from a Web Audio graph to a MediaStream. It acts as a destination
// node in the Web Audio graph, receiving audio from upstream nodes and
// delivering it to a WebAudioDestinationConsumer.
class MODULES_EXPORT MediaStreamAudioDestinationHandler final
    : public AudioHandler {
 public:
  static scoped_refptr<MediaStreamAudioDestinationHandler> Create(
      AudioNode&,
      uint32_t number_of_channels,
      WebAudioDestinationConsumer*);
  ~MediaStreamAudioDestinationHandler() override;

  // This handler must release its reference to the consumer when the
  // owning MediaStreamSource associated with this handler's node is garbage
  // collected. Returns true when the removal is successful.
  bool RemoveConsumer();

 private:
  friend class MediaStreamAudioDestinationHandlerTest;

  MediaStreamAudioDestinationHandler(
      AudioNode&, uint32_t number_of_channels, WebAudioDestinationConsumer*);

  // AudioHandler
  void Process(uint32_t frames_to_process) override;
  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;
  bool RequiresTailProcessing() const override { return false; }
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }
  bool PropagatesSilence() const override {
    // As an audio source, we will never propagate silence.
    return false;
  }
  void SetChannelCount(unsigned, ExceptionState&) override;
  void PullInputs(uint32_t frames_to_process) override;
  void UpdatePullStatusIfNeeded() override;

  uint32_t MaxChannelCount() const;

  // Sets the WebAudioDestinationConsumer that receives audio data from this
  // handler. The consumer is then responsible for providing this data to the
  // MediaStream infrastructure.
  void SetConsumer(WebAudioDestinationConsumer*,
                   int number_of_channels,
                   float sample_rate);

  // Sets the audio format (number of channels and sample rate) for the
  // associated MediaStreamSource. This method can be called from either the
  // main thread or the WebAudio rendering thread.
  void SetConsumerFormat(int number_of_channels, float sample_rate);

  // Pushes rendered WebAudio data to the WebAudioDestinationConsumer.
  // Must be called on the WebAudio rendering thread.
  void ConsumeAudio(AudioBus* bus, int number_of_frames);

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name, const String& message);

  base::Lock consumer_lock_;
  // `destination_consumer_` is owned by the node's MediaStreamSource.
  raw_ptr<WebAudioDestinationConsumer, DanglingUntriaged>
      destination_consumer_ GUARDED_BY(consumer_lock_);
  Vector<const float*> consumer_bus_wrapper_ GUARDED_BY(consumer_lock_);

  // This synchronizes dynamic changes to the channel count with
  // process() to manage the mix bus.
  mutable base::Lock process_lock_;

  // This internal mix bus is for up/down mixing the input to the actual
  // number of channels in the destination.
  scoped_refptr<AudioBus> mix_bus_;

  // When setting to true, handler will be pulled automatically by
  // BaseAudioContext before the end of each render quantum.
  bool need_automatic_pull_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_STREAM_AUDIO_DESTINATION_HANDLER_H_
