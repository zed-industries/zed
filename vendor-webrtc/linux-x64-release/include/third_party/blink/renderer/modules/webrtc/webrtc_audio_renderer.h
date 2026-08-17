// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_RENDERER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_RENDERER_H_

#include <stdint.h>

#include <atomic>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "media/base/audio_decoder.h"
#include "media/base/audio_glitch_info.h"
#include "media/base/audio_power_monitor.h"
#include "media/base/audio_pull_fifo.h"
#include "media/base/audio_renderer_sink.h"
#include "media/base/channel_layout.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_audio_renderer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/webrtc/webrtc_source.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace media {
class SpeechRecognitionClient;
}  // namespace media

namespace webrtc {
class AudioSourceInterface;
}  // namespace webrtc

namespace blink {

class LocalFrame;
class WebLocalFrame;
class WebRtcAudioRendererSource;

// This renderer handles calls from the pipeline and WebRtc ADM. It is used
// for connecting WebRtc MediaStream with the audio pipeline.
class MODULES_EXPORT WebRtcAudioRenderer
    : public media::AudioRendererSink::RenderCallback,
      public MediaStreamAudioRenderer {
 public:
  // This is a little utility class that holds the configured state of an audio
  // stream.
  // It is used by both WebRtcAudioRenderer and SharedAudioRenderer (see cc
  // file) so a part of why it exists is to avoid code duplication and track
  // the state in the same way in WebRtcAudioRenderer and SharedAudioRenderer.
  class PlayingState {
   public:
    PlayingState() : playing_(false), volume_(1.0f) {}

    ~PlayingState() { DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_); }

    bool playing() const {
      DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
      return playing_;
    }

    void set_playing(bool playing) {
      DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
      playing_ = playing;
    }

    float volume() const {
      DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
      return volume_;
    }

    void set_volume(float volume) {
      DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
      volume_ = volume;
    }

   private:
    bool playing_;
    float volume_;

    SEQUENCE_CHECKER(sequence_checker_);
  };

  enum State {
    kUninitialized,
    kPlaying,
    kPaused,
  };

  WebRtcAudioRenderer() = delete;

  WebRtcAudioRenderer(
      const scoped_refptr<base::SingleThreadTaskRunner>& signaling_thread,
      MediaStreamDescriptor* media_stream_descriptor,
      WebLocalFrame& web_frame,
      const base::UnguessableToken& session_id,
      const String& device_id,
      base::RepeatingCallback<void()> on_render_error_callback);

  WebRtcAudioRenderer(const WebRtcAudioRenderer&) = delete;
  WebRtcAudioRenderer& operator=(const WebRtcAudioRenderer&) = delete;

  // Initialize function called by clients like WebRtcAudioDeviceImpl.
  // Stop() has to be called before |source| is deleted.
  bool Initialize(WebRtcAudioRendererSource* source);

  // When sharing a single instance of WebRtcAudioRenderer between multiple
  // users (e.g. WebMediaPlayerMS), call this method to create a proxy object
  // that maintains the Play and Stop states per caller.
  // The wrapper ensures that Play() won't be called when the caller's state
  // is "playing", Pause() won't be called when the state already is "paused"
  // etc and similarly maintains the same state for Stop().
  // When Stop() is called or when the proxy goes out of scope, the proxy
  // will ensure that Pause() is called followed by a call to Stop(), which
  // is the usage pattern that WebRtcAudioRenderer requires.
  scoped_refptr<MediaStreamAudioRenderer> CreateSharedAudioRendererProxy(
      MediaStreamDescriptor* media_stream_descriptor);

  // Used to DCHECK on the expected state.
  bool IsStarted() const;

  // Accessors to the sink audio parameters.
  int channels() const { return sink_params_.channels(); }
  int sample_rate() const { return sink_params_.sample_rate(); }
  int frames_per_buffer() const { return sink_params_.frames_per_buffer(); }

  // Returns true if called on rendering thread, otherwise false.
  bool CurrentThreadIsRenderingThread();

 private:
  // MediaStreamAudioRenderer implementation.  This is private since
  // we want callers to use proxy objects.
  // TODO(tommi): Make the MediaStreamAudioRenderer implementation a
  // pimpl?
  void Start() override;
  void Play() override;
  void Pause() override;
  void Stop() override;
  void SetVolume(float volume) override;
  base::TimeDelta GetCurrentRenderTime() override;
  void SwitchOutputDevice(const std::string& device_id,
                          media::OutputDeviceStatusCB callback) override;

  // Private utility class which keeps track of the state and duration of
  // playing (rendered on an HTML5 audio tag) audio streams. Mainly intended
  // for logging purposes to track down "can't hear" type of issues.
  class AudioStreamTracker {
   public:
    // The internal on-shot timer will use the same |task_runner| as the outer
    // class (OC) who creates this object. |renderer| must outlive the
    // AudioStreamTracker. See comments for |AudioStreamTracker::renderer_| why
    // it is safe to use a raw pointer here. |sample_rate| is the current sample
    // rate used by the audio sink (see WebRtcAudioRenderer::sink_params_).
    explicit AudioStreamTracker(
        scoped_refptr<base::SingleThreadTaskRunner> task_runner,
        WebRtcAudioRenderer* renderer,
        int sample_rate);

    // Note: the destructor takes care of logging of the duration of the stream.
    ~AudioStreamTracker();

    // This function should be called from the audio device callback thread,
    // i.e., the so-called render thread.
    void OnRenderCallbackCalled();

    // Scans the provided audio samples and updates a power measurement. The
    // "average power" is a running average calculated by using a first-order
    // low-pass filter over the square of the samples scanned.
    // Called from the audio render thread and it is safe. See comments in
    // AudioPowerMonitor::Scan() for more details.
    void MeasurePower(const media::AudioBus& buffer, int frames);

   private:
    // Called by the timer when it fires once after a delay of ~5 seconds from
    // start. Reads the state of atomic |render_callbacks_started_|.
    void CheckAlive(TimerBase*);

    void LogAudioPowerLevel();

    // Task runner of outer class (the creating WebRtcAudioRenderer).
    const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

    // Using a raw pointer is safe since the OC instance will outlive this
    // object.
    const raw_ptr<WebRtcAudioRenderer> renderer_;

    // Stores when the timer starts. Used to calculate the stream duration.
    const base::TimeTicks start_time_;

    // Set to true in first render callback by the high-priority audio thread.
    // Use an atomic variable since it can also be read by the outer class (OC)
    // thread once to verify that callbacks started as intended. See comments
    // for CheckAlive().
    std::atomic_bool render_callbacks_started_;

    // One-shot timer that fires ~5 seconds after rendering should start and
    // calls the calls CheckAlive() which checks if |render_callbacks_started_|
    // has been set to true or not. A message is logged to track this state.
    // The timer uses the same task runner as the OC. Hence, the only writer of
    // |render_callbacks_started_| is the render thread and the only reader is
    // the OC thread. DCHECKs are used to confirm this.
    TaskRunnerTimer<AudioStreamTracker> check_alive_timer_;

    // Scans audio samples from Render() as input to compute power levels.
    media::AudioPowerMonitor power_monitor_;

    // Updated each time a power measurement is logged.
    base::TimeTicks last_audio_level_log_time_;

    base::WeakPtr<AudioStreamTracker> weak_this_;
    base::WeakPtrFactory<AudioStreamTracker> weak_factory_{this};
  };

  // Called when an audio renderer, either the main or a proxy, starts playing.
  // Here we maintain a reference count of how many renderers are currently
  // playing so that the shared play state of all the streams can be reflected
  // correctly.
  void EnterPlayState();

  // Called when an audio renderer, either the main or a proxy, is paused.
  // See EnterPlayState for more details.
  void EnterPauseState();

 protected:
  ~WebRtcAudioRenderer() override;

 private:
  // Holds raw pointers to PlaingState objects.  Ownership is managed outside
  // of this type.
  typedef std::vector<raw_ptr<PlayingState, VectorExperimental>> PlayingStates;
  // Maps an audio source to a list of playing states that collectively hold
  // volume information for that source.
  typedef std::map<webrtc::AudioSourceInterface*, PlayingStates>
      SourcePlayingStates ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/1404327");

  // Used to DCHECK that we are called on the correct thread.
  THREAD_CHECKER(thread_checker_);

  // Task runner of the creating thread.
  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Flag to keep track the state of the renderer.
  State state_;

  // media::AudioRendererSink::RenderCallback implementation.
  // These two methods are called on the AudioOutputDevice worker thread.
  int Render(base::TimeDelta delay,
             base::TimeTicks delay_timestamp,
             const media::AudioGlitchInfo& glitch_info,
             media::AudioBus* audio_bus) override;
  void OnRenderError() override;

  void OnRenderErrorCrossThread();

  // Called by AudioPullFifo when more data is necessary.
  // This method is called on the AudioOutputDevice worker thread.
  void SourceCallback(int fifo_frame_delay, media::AudioBus* audio_bus);

  // Goes through all renderers for the |source| and applies the proper
  // volume scaling for the source based on the volume(s) of the renderer(s).
  void UpdateSourceVolume(webrtc::AudioSourceInterface* source);

  // Tracks a playing state.  The state must be playing when this method
  // is called.
  // Returns true if the state was added, false if it was already being tracked.
  bool AddPlayingState(webrtc::AudioSourceInterface* source,
                       PlayingState* state);
  // Removes a playing state for an audio source.
  // Returns true if the state was removed from the internal map, false if
  // it had already been removed or if the source isn't being rendered.
  bool RemovePlayingState(webrtc::AudioSourceInterface* source,
                          PlayingState* state);

  // Called whenever the Play/Pause state changes of any of the renderers
  // or if the volume of any of them is changed.
  // Here we update the shared Play state and apply volume scaling to all audio
  // sources associated with the |media_stream_descriptor| based on the
  // collective volume of playing renderers.
  void OnPlayStateChanged(MediaStreamDescriptor* media_stream_descriptor,
                          PlayingState* state);

  // Called when |state| is about to be destructed.
  void OnPlayStateRemoved(PlayingState* state);

  // Updates |sink_params_| and |audio_fifo_| based on |sink_|, and initializes
  // |sink_|.
  void PrepareSink();

  void SendLogMessage(const WTF::String& message);

  // The LocalFrame in which the audio is rendered into |sink_|.
  WeakPersistent<LocalFrame> source_frame_;

  const base::UnguessableToken session_id_;

  const scoped_refptr<base::SingleThreadTaskRunner> signaling_thread_;

  // The sink (destination) for rendered audio.
  scoped_refptr<media::AudioRendererSink> sink_;

  // The media stream that holds the audio tracks that this renderer renders.
  Persistent<MediaStreamDescriptor> media_stream_descriptor_;

  // Contains a copy the unique id of the media stream. By taking a copy at
  // construction, we can convert the id from a WebString to an WTF::string
  // once and that saves resources when |media_stream_descriptor_id_| is added
  // to log messages.
  String media_stream_descriptor_id_;

  // Audio data source from the browser process.
  //
  // TODO(crbug.com/704136): Make it a Member.
  raw_ptr<WebRtcAudioRendererSource> source_;

  // Protects access to |state_|, |source_|, |audio_fifo_|,
  // |audio_delay_milliseconds_|, |fifo_delay_milliseconds_|, |current_time_|,
  // |sink_params_|, |render_callback_count_|, |max_render_time_| and
  // |audio_stream_tracker_|.
  mutable base::Lock lock_;

  // Ref count for the MediaPlayers which are playing audio.
  int play_ref_count_;

  // Ref count for the MediaPlayers which have called Start() but not Stop().
  int start_ref_count_;

  // Used to buffer data between the client and the output device in cases where
  // the client buffer size is not the same as the output device buffer size.
  std::unique_ptr<media::AudioPullFifo> audio_fifo_;

  // Contains the accumulated delay estimate which is provided to the WebRTC
  // AEC.
  base::TimeDelta audio_delay_;

  base::TimeDelta current_time_;

  // Saved volume and playing state of the root renderer.
  PlayingState playing_state_;

  // Audio params used by the sink of the renderer.
  media::AudioParameters sink_params_;

  // The preferred device id of the output device or empty for the default
  // output device. Can change as a result of a SetSinkId() call.
  String output_device_id_;

  // Maps audio sources to a list of active audio renderers.
  // Pointers to PlayingState objects are only kept in this map while the
  // associated renderer is actually playing the stream.  Ownership of the
  // state objects lies with the renderers and they must leave the playing state
  // before being destructed (PlayingState object goes out of scope).
  SourcePlayingStates source_playing_states_;

  // Stores the maximum time spent waiting for render data from the source. Used
  // for logging UMA data. Logged and reset when Stop() is called.
  base::TimeDelta max_render_time_;

  // Used for keeping track of and logging stats for playing audio streams.
  // Created when a stream starts and destroyed when a stream stops.
  // See comments for AudioStreamTracker for more details.
  std::optional<AudioStreamTracker> audio_stream_tracker_;

  base::RepeatingCallback<void()> on_render_error_callback_;

  std::unique_ptr<media::SpeechRecognitionClient> speech_recognition_client_;

  // Accessed only on the rendering thread.
  media::AudioGlitchInfo::Accumulator glitch_info_accumulator_;

  base::WeakPtrFactory<WebRtcAudioRenderer> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBRTC_WEBRTC_AUDIO_RENDERER_H_
