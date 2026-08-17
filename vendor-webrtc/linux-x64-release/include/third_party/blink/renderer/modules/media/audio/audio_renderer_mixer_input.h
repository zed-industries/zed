// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// THREAD SAFETY
//
// This class is generally not thread safe. Callers should ensure thread safety.
// For instance, the `sink_lock_` in WebAudioSourceProvider synchronizes access
// to this object across the main thread (for WebAudio APIs) and the
// media thread (for HTMLMediaElement APIs).
//
// The one exception is protection for `volume_` via `volume_lock_`. This lock
// prevents races between SetVolume() (called on any thread) and ProvideInput
// (called on audio device thread). See http://crbug.com/588992.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_INPUT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_INPUT_H_

#include <string>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/unguessable_token.h"
#include "media/base/audio_converter.h"
#include "media/base/audio_latency.h"
#include "media/base/audio_renderer_sink.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class AudioRendererMixerPool;
class AudioRendererMixer;

class BLINK_MODULES_EXPORT AudioRendererMixerInput
    : public media::SwitchableAudioRendererSink,
      public media::AudioConverter::InputCallback {
 public:
  // `mixer_pool` is used to request sinks and mixer instances.
  // `source_frame_token` refers to the local RenderFrame containing the entity
  // producing the audio. It is used to request output sinks.
  // `main_frame_token` refers to the local or remote main frame at the root of
  // the tree containing the RenderFrame referenced by `source_frame_token` and
  // is used for sharing the underlying audio output device.
  // `device_id` is the name of the output device which should be used.
  // `latency` is used to configure buffer size for the output device.
  AudioRendererMixerInput(AudioRendererMixerPool* mixer_pool,
                          const LocalFrameToken& source_frame_token,
                          const FrameToken& main_frame_token,
                          std::string_view device_id,
                          media::AudioLatency::Type latency);

  AudioRendererMixerInput(const AudioRendererMixerInput&) = delete;
  AudioRendererMixerInput& operator=(const AudioRendererMixerInput&) = delete;

  // SwitchableAudioRendererSink implementation.
  void Start() override;
  void Stop() override;
  void Play() override;
  void Pause() override;
  void Flush() override;
  bool SetVolume(double volume) override;
  media::OutputDeviceInfo GetOutputDeviceInfo() override;
  void GetOutputDeviceInfoAsync(OutputDeviceInfoCB info_cb) override;

  bool IsOptimizedForHardwareParameters() override;
  void Initialize(const media::AudioParameters& params,
                  media::AudioRendererSink::RenderCallback* renderer) override;
  void SwitchOutputDevice(const std::string& device_id,
                          media::OutputDeviceStatusCB callback) override;
  // This is expected to be called on the audio rendering thread. The caller
  // must ensure that this input has been added to a mixer before calling the
  // function, and that it is not removed from the mixer before this function
  // returns.
  bool CurrentThreadIsRenderingThread() override;

  // Called by AudioRendererMixer when an error occurs.
  void OnRenderError();

 private:
  ~AudioRendererMixerInput() override;

  friend class AudioRendererMixerInputTest;

  // Pool to obtain mixers from / return them to.
  const raw_ptr<AudioRendererMixerPool> mixer_pool_;

  // Protect `volume_`, accessed by separate threads in ProvideInput() and
  // SetVolume().
  base::Lock volume_lock_;

  bool started_ = false;
  bool playing_ = false;
  double volume_ GUARDED_BY(volume_lock_) = 1.0;

  scoped_refptr<AudioRendererSink> sink_;
  std::optional<media::OutputDeviceInfo> device_info_;

  // AudioConverter::InputCallback implementation.
  double ProvideInput(media::AudioBus* audio_bus,
                      uint32_t frames_delayed,
                      const media::AudioGlitchInfo& glitch_info) override;

  void OnDeviceInfoReceived(OutputDeviceInfoCB info_cb,
                            media::OutputDeviceInfo device_info);

  // Method to help handle device changes. Must be static to ensure we can still
  // execute the `switch_cb` even if the pipeline is destructed. Restarts (if
  // necessary) Start() and Play() state with a new `sink` and `device_info`.
  //
  // `switch_cb` is the callback given to the SwitchOutputDevice() call.
  // `sink` is a fresh sink which should be used if device info is good.
  // `device_info` is the OutputDeviceInfo for `sink` after
  // GetOutputDeviceInfoAsync() completes.
  void OnDeviceSwitchReady(media::OutputDeviceStatusCB switch_cb,
                           scoped_refptr<media::AudioRendererSink> sink,
                           media::OutputDeviceInfo device_info);

  // AudioParameters received during Initialize().
  media::AudioParameters params_;

  // Linearly fades in the input volume during the first ProvideInput() calls,
  // avoiding audible pops.
  int total_fade_in_frames_;
  int remaining_fade_in_frames_ = 0;

  const LocalFrameToken source_frame_token_;
  const FrameToken main_frame_token_;

  std::string device_id_;  // ID of hardware device to use
  const media::AudioLatency::Type latency_;

  // AudioRendererMixer obtained from mixer pool during Initialize(),
  // guaranteed to live (at least) until it is returned to the pool.
  raw_ptr<AudioRendererMixer> mixer_ = nullptr;

  // Source of audio data which is provided to the mixer.
  raw_ptr<AudioRendererSink::RenderCallback> callback_ = nullptr;

  // SwitchOutputDevice() and GetOutputDeviceInfoAsync() must be mutually
  // exclusive when executing; these flags indicate whether one or the other is
  // in progress. Each method will use the other method's to defer its action.
  bool godia_in_progress_ = false;
  bool switch_output_device_in_progress_ = false;

  // Set by GetOutputDeviceInfoAsync() if a SwitchOutputDevice() call is in
  // progress. GetOutputDeviceInfoAsync() will be invoked again with this value
  // once OnDeviceSwitchReady() from the SwitchOutputDevice() call completes.
  OutputDeviceInfoCB pending_device_info_cb_;

  // Set by SwitchOutputDevice() if a GetOutputDeviceInfoAsync() call is in
  // progress. SwitchOutputDevice() will be invoked again with these values once
  // the OnDeviceInfoReceived() from the GODIA() call completes.
  std::string pending_device_id_;
  media::OutputDeviceStatusCB pending_switch_cb_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIA_AUDIO_AUDIO_RENDERER_MIXER_INPUT_H_
