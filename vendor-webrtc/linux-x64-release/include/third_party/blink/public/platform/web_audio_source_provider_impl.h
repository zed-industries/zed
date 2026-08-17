// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_SOURCE_PROVIDER_IMPL_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_SOURCE_PROVIDER_IMPL_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <vector>

#include "base/functional/callback.h"
#include "base/functional/callback_forward.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "media/base/audio_renderer_sink.h"
#include "third_party/blink/public/platform/web_audio_source_provider.h"
#include "third_party/blink/public/platform/web_common.h"

namespace media {
class MediaLog;
}

namespace blink {

class WebAudioSourceProviderClient;

// WebAudioSourceProviderImpl is either one of two things (but not both):
// - a connection between a RestartableAudioRendererSink (the |sink_|) passed in
//   constructor and an AudioRendererSink::RenderCallback passed on Initialize()
//   by means of an internal AudioRendererSink::RenderCallback.
// - a connection between the said AudioRendererSink::RenderCallback and a
//   WebAudioSourceProviderClient passed via setClient() (the |client_|),
//   again using the internal AudioRendererSink::RenderCallback. Blink calls
//   provideInput() periodically to fetch the appropriate data.
//
// In either case, the internal RenderCallback allows for delivering a copy of
// the data if a listener is configured. WASPImpl is also a
// RestartableAudioRendererSink itself in order to be controlled (Play(),
// Pause() etc).
//
// All calls are protected by a lock.
class BLINK_PLATFORM_EXPORT WebAudioSourceProviderImpl
    : public WebAudioSourceProvider,
      public media::SwitchableAudioRendererSink {
 public:
  using CopyAudioCB =
      base::RepeatingCallback<void(std::unique_ptr<media::AudioBus>,
                                   uint32_t frames_delayed,
                                   int sample_rate)>;

  // Optionally provide a callback to be run the first time a
  // WebAudioSourceProviderClient is attached via SetClient(). Note that the
  // callback will be run once at most, however SetClient may be called any
  // number of times.
  WebAudioSourceProviderImpl(
      scoped_refptr<media::SwitchableAudioRendererSink> sink,
      media::MediaLog* media_log,
      base::OnceClosure on_set_client_callback = base::OnceClosure());

  WebAudioSourceProviderImpl(const WebAudioSourceProviderImpl&) = delete;
  WebAudioSourceProviderImpl& operator=(const WebAudioSourceProviderImpl&) =
      delete;

  // WebAudioSourceProvider implementation.
  void SetClient(WebAudioSourceProviderClient* client) override;
  void ProvideInput(const std::vector<float*>& audio_data,
                    int number_of_frames) override;

  // RestartableAudioRendererSink implementation.
  void Initialize(const media::AudioParameters& params,
                  RenderCallback* renderer) override;
  void Start() override;
  void Stop() override;
  void Play() override;
  void Pause() override;
  void Flush() override;
  bool SetVolume(double volume) override;
  media::OutputDeviceInfo GetOutputDeviceInfo() override;
  void GetOutputDeviceInfoAsync(OutputDeviceInfoCB info_cb) override;
  bool IsOptimizedForHardwareParameters() override;
  bool CurrentThreadIsRenderingThread() override;
  void SwitchOutputDevice(const std::string& device_id,
                          media::OutputDeviceStatusCB callback) override;
  void TaintOrigin();

  // These methods allow a client to get a copy of the rendered audio.
  void SetCopyAudioCallback(CopyAudioCB callback);
  void ClearCopyAudioCallback();

  int RenderForTesting(media::AudioBus* audio_bus);

  bool IsAudioBeingCaptured() const;

  void ConnectToDestinationReady();

 private:
  ~WebAudioSourceProviderImpl() override;

  friend class WebAudioSourceProviderImplTest;

  // Calls setFormat() on |client_| from the Blink renderer thread.
  void OnSetFormat();

  // Used to keep the volume across reconfigurations.
  double volume_ = 1.0;

  // Tracks the current playback state.
  enum PlaybackState { kStopped, kStarted, kPlaying };
  PlaybackState state_ = kStopped;

  // Closure that calls OnSetFormat() on |client_| on the renderer thread.
  base::RepeatingClosure set_format_cb_;

  // When set via setClient() it overrides |sink_| for consuming audio.
  raw_ptr<WebAudioSourceProviderClient> client_ = nullptr;

  // Where audio ends up unless overridden by |client_|.
  base::Lock sink_lock_;
  scoped_refptr<media::SwitchableAudioRendererSink> sink_
      GUARDED_BY(sink_lock_);
  std::unique_ptr<media::AudioBus> bus_wrapper_;

  // An inner class acting as a T filter where actual data can be tapped.
  class TeeFilter;
  const std::unique_ptr<TeeFilter> tee_filter_;

  // This dangling raw_ptr occurred in:
  // webkit_unit_tests: WebMediaPlayerImplTest.MediaPositionState_Playing
  // https://ci.chromium.org/ui/p/chromium/builders/try/win-rel/237451/test-results?q=ExactID%3Aninja%3A%2F%2Fthird_party%2Fblink%2Frenderer%2Fcontroller%3Ablink_unittests%2FWebMediaPlayerImplTest.MediaPositionState_Playing+VHash%3Abfecf1e29c759a1c
  const raw_ptr<media::MediaLog, FlakyDanglingUntriaged> media_log_;

  base::OnceClosure on_set_client_callback_;

  bool has_copy_audio_callback_ = false;

  // NOTE: Weak pointers must be invalidated before all other member variables.
  base::WeakPtrFactory<WebAudioSourceProviderImpl> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_AUDIO_SOURCE_PROVIDER_IMPL_H_
