// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_HANDLER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/notreached.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider_client.h"
#include "third_party/blink/renderer/platform/audio/media_multi_channel_resampler.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"

namespace blink {

class AudioContext;
class HTMLMediaElement;
class MediaElementAudioSourceOptions;

class MediaElementAudioSourceHandler final : public AudioHandler {
 public:
  static scoped_refptr<MediaElementAudioSourceHandler> Create(
      AudioNode&,
      HTMLMediaElement&);
  ~MediaElementAudioSourceHandler() override;

  CrossThreadPersistent<HTMLMediaElement> MediaElement() const;

  // AudioHandler
  void Dispose() override;
  void Process(uint32_t frames_to_process) override;

  // AudioNode
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }

  // Helpers for AudioSourceProviderClient implementation of
  // MediaElementAudioSourceNode.
  void SetFormat(uint32_t number_of_channels, float sample_rate);
  void lock() EXCLUSIVE_LOCK_FUNCTION(GetProcessLock());
  void unlock() UNLOCK_FUNCTION(GetProcessLock());

  // For thread safety analysis only.  Does not actually return mu.
  base::Lock* GetProcessLock() LOCK_RETURNED(process_lock_) { NOTREACHED(); }

  bool RequiresTailProcessing() const final { return false; }

 private:
  MediaElementAudioSourceHandler(AudioNode&, HTMLMediaElement&);
  // As an audio source, we will never propagate silence.
  bool PropagatesSilence() const override { return false; }

  // Returns true if the origin of the media element is tainted so that the
  // audio should be muted when playing through WebAudio.
  bool WouldTaintOrigin();

  // Print warning if CORS restrictions cause MediaElementAudioSource to output
  // zeroes.
  void PrintCorsMessage(const String& message);

  // Provide input to the resampler (if used).
  void ProvideResamplerInput(int resampler_frame_delay, AudioBus* dest);

  // The HTMLMediaElement is held alive by MediaElementAudioSourceNode which is
  // an AudioNode. AudioNode uses pre-finalizers to dispose the handler, so
  // holding a weak reference is ok here and will not interfer with garbage
  // collection.
  //
  // It is accessed by both audio and main thread. TODO: we really should
  // try to minimize or avoid the audio thread touching this element.
  CrossThreadWeakPersistent<HTMLMediaElement> media_element_;
  base::Lock process_lock_;

  unsigned source_number_of_channels_ = 0;
  double source_sample_rate_ = 0;

  std::unique_ptr<MediaMultiChannelResampler> multi_channel_resampler_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // True if the origin would be tainted by the media element.  In this case,
  // this node outputs silence.  This can happen if the media element source is
  // a cross-origin source which we're not allowed to access due to CORS
  // restrictions.
  bool is_origin_tainted_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_MEDIA_ELEMENT_AUDIO_SOURCE_HANDLER_H_
