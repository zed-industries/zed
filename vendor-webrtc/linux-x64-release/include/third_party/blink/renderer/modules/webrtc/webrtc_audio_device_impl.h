// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_DEVICE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_DEVICE_IMPL_H_

#include <stdint.h>

#include <list>
#include <memory>
#include <string>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "media/base/audio_glitch_info.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webrtc/webrtc_audio_device_not_impl.h"
#include "third_party/blink/renderer/platform/webrtc/webrtc_source.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

// A WebRtcAudioDeviceImpl instance implements the abstract interface
// webrtc::AudioDeviceModule which makes it possible for a user (e.g. webrtc::
// VoiceEngine) to register this class as an external AudioDeviceModule (ADM).
//
// Implementation notes:
//
//  - This class must be created and destroyed on the main render thread and
//    most methods are called on the same thread. However, some methods are
//    also called on a Libjingle worker thread. RenderData is called on the
//    AudioOutputDevice thread and CaptureData on the AudioInputDevice thread.
//    To summarize: this class lives on four different threads, so it is
//    important to be careful with the order in which locks are acquired in
//    order to avoid potential deadlocks.
//
namespace blink {
class WebRtcAudioRenderer;
}

namespace media {
class AudioBus;
}

namespace blink {

class ProcessedLocalAudioSource;

// Note that this class inherits from webrtc::AudioDeviceModule but due to
// the high number of non-implemented methods, we move the cruft over to the
// WebRtcAudioDeviceNotImpl.
class MODULES_EXPORT WebRtcAudioDeviceImpl
    : public WebRtcAudioDeviceNotImpl,
      public blink::WebRtcAudioRendererSource,
      public blink::WebRtcPlayoutDataSource {
 public:
  // Instances of this object are created on the main render thread.
  WebRtcAudioDeviceImpl();

  WebRtcAudioDeviceImpl(const WebRtcAudioDeviceImpl&) = delete;
  WebRtcAudioDeviceImpl& operator=(const WebRtcAudioDeviceImpl&) = delete;

 protected:
  // Make destructor protected, we should only be deleted by Release().
  ~WebRtcAudioDeviceImpl() override;

 private:
  // webrtc::AudioDeviceModule implementation.
  // All implemented methods are called on the main render thread unless
  // anything else is stated.

  int32_t RegisterAudioCallback(
      webrtc::AudioTransport* audio_callback) override;

  int32_t Init() override;
  int32_t Terminate() override;
  bool Initialized() const override;

  int32_t PlayoutIsAvailable(bool* available) override;
  bool PlayoutIsInitialized() const override;
  int32_t RecordingIsAvailable(bool* available) override;
  bool RecordingIsInitialized() const override;

  // All Start/Stop methods are called on a libJingle worker thread.
  int32_t StartPlayout() override;
  int32_t StopPlayout() override;
  bool Playing() const override;
  int32_t StartRecording() override;
  int32_t StopRecording() override;
  bool Recording() const override;

  int32_t PlayoutDelay(uint16_t* delay_ms) const override;

 public:
  // Sets the |renderer_|, returns false if |renderer_| already exists.
  // Called on the main renderer thread.
  bool SetAudioRenderer(blink::WebRtcAudioRenderer* renderer);

  // Adds/Removes the |capturer| to the ADM.  Does NOT take ownership.
  // Capturers must remain valid until RemoveAudioCapturer() is called.
  // TODO(xians): Remove these two methods once the ADM does not need to pass
  // hardware information up to WebRtc.
  void AddAudioCapturer(ProcessedLocalAudioSource* capturer);
  void RemoveAudioCapturer(ProcessedLocalAudioSource* capturer);

  // Returns the session id of the capture device if it has a paired output
  // device, otherwise 0. The session id is passed on to a webrtc audio renderer
  // (either local or remote), so that audio will be rendered to a matching
  // output device. Note that if there are more than one open capture device the
  // function will not be able to pick an appropriate device and return 0.
  base::UnguessableToken GetAuthorizedDeviceSessionIdForAudioRenderer();

  const scoped_refptr<blink::WebRtcAudioRenderer>& renderer() const {
    return renderer_;
  }

  // blink::WebRtcAudioRendererSource implementation.

  // Called on the AudioOutputDevice worker thread.
  void RenderData(media::AudioBus* audio_bus,
                  int sample_rate,
                  base::TimeDelta audio_delay,
                  base::TimeDelta* current_time,
                  const media::AudioGlitchInfo& glitch_info) override;

  // Called on the main render thread.
  void RemoveAudioRenderer(blink::WebRtcAudioRenderer* renderer) override;
  void AudioRendererThreadStopped() override;
  void SetOutputDeviceForAec(const String& output_device_id) override;

  // blink::WebRtcPlayoutDataSource implementation.
  void AddPlayoutSink(blink::WebRtcPlayoutDataSource::Sink* sink) override;
  void RemovePlayoutSink(blink::WebRtcPlayoutDataSource::Sink* sink) override;

  std::optional<webrtc::AudioDeviceModule::Stats> GetStats() const override;

  const String& GetOutputDeviceForAecForTesting() {
    return output_device_id_for_aec_;
  }

 private:
  using CapturerList =
      std::list<raw_ptr<ProcessedLocalAudioSource, CtnExperimental>>;
  using PlayoutDataSinkList =
      std::list<raw_ptr<blink::WebRtcPlayoutDataSource::Sink, CtnExperimental>>;

  class RenderBuffer;

  // Used to check methods that run on the main render thread.
  THREAD_CHECKER(main_thread_checker_);
  // Used to check methods that are called on libjingle's signaling thread.
  THREAD_CHECKER(signaling_thread_checker_);
  THREAD_CHECKER(worker_thread_checker_);
  THREAD_CHECKER(audio_renderer_thread_checker_);

  // List of captures which provides access to the native audio input layer
  // in the browser process.  The last capturer in this list is considered the
  // "default capturer" by the methods implementing the
  // webrtc::AudioDeviceModule interface.
  CapturerList capturers_;

  // Provides access to the audio renderer in the browser process.
  scoped_refptr<blink::WebRtcAudioRenderer> renderer_ GUARDED_BY(lock_);

  // A list of raw pointer of blink::WebRtcPlayoutDataSource::Sink objects which
  // want to get the playout data, the sink need to call RemovePlayoutSink()
  // before it goes away.
  PlayoutDataSinkList playout_sinks_ GUARDED_BY(lock_);

  // Weak reference to the audio callback.
  // The webrtc client defines |audio_transport_callback_| by calling
  // RegisterAudioCallback().
  raw_ptr<webrtc::AudioTransport, DanglingUntriaged> audio_transport_callback_;

  // Cached value of the current audio delay on the output/renderer side.
  base::TimeDelta output_delay_ GUARDED_BY(lock_);

  // Protects |renderer_|, |playout_sinks_|, |output_delay_|, |playing_|,
  // and |recording_|.
  mutable base::Lock lock_;

  bool initialized_;
  bool playing_ GUARDED_BY(lock_);
  bool recording_ GUARDED_BY(lock_);

  // Buffer used for temporary storage during render callback.
  // It is only accessed by the audio render thread.
  Vector<int16_t> render_buffer_;

  // The output device used for echo cancellation
  String output_device_id_for_aec_;

  // Corresponds to RTCAudioPlayoutStats as defined in
  // https://w3c.github.io/webrtc-stats/#playoutstats-dict*
  media::AudioGlitchInfo cumulative_glitch_info_ GUARDED_BY(lock_);
  base::TimeDelta total_samples_duration_ GUARDED_BY(lock_);
  base::TimeDelta total_playout_delay_ GUARDED_BY(lock_);
  uint64_t total_samples_count_ GUARDED_BY(lock_) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_DEVICE_IMPL_H_
