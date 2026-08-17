// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TRACK_AUDIO_RENDERER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TRACK_AUDIO_RENDERER_H_

#include <stdint.h>

#include <memory>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "base/unguessable_token.h"
#include "media/base/audio_renderer_sink.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream_audio_sink.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_audio_renderer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_component.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"

namespace media {
class AudioBus;
class AudioShifter;
class AudioParameters;
}  // namespace media

namespace blink {

class LocalFrame;

// TrackAudioRenderer is a MediaStreamAudioRenderer for plumbing audio
// data generated from either local or remote (but not
// PeerConnection/WebRTC-sourced) MediaStreamAudioTracks to an audio output
// device, reconciling differences in the rates of production and consumption of
// the audio data.  Note that remote PeerConnection-sourced tracks are NOT
// rendered by this implementation (see MediaStreamRendererFactory).
//
// This class uses AudioDeviceFactory to create media::AudioRendererSink and
// owns/manages their lifecycles.  Output devices are automatically re-created
// in response to audio format changes, or use of the SwitchOutputDevice() API
// by client code.
//
// Audio data is feed-in from the source via calls to OnData().  The
// internally-owned media::AudioOutputDevice calls Render() to pull-out that
// audio data.  However, because of clock differences and other environmental
// factors, the audio will inevitably feed-in at a rate different from the rate
// it is being rendered-out.  media::AudioShifter is used to buffer, stretch
// and skip audio to maintain time synchronization between the producer and
// consumer.
class MODULES_EXPORT TrackAudioRenderer
    : public MediaStreamAudioRenderer,
      public WebMediaStreamAudioSink,
      public media::AudioRendererSink::RenderCallback {
 public:
  // Creates a renderer for the given |audio_track|.  |playout_render_frame|
  // refers to the RenderFrame that owns this instance (e.g., it contains the
  // DOM widget representing the player).  |session_id| and |device_id| are
  // optional, and are used to direct audio output to a pre-selected device;
  // otherwise, audio is output to the default device for the system.
  //
  // Called on the main thread.
  TrackAudioRenderer(MediaStreamComponent* audio_component,
                     LocalFrame& playout_web_frame,
                     const String& device_id,
                     base::RepeatingClosure on_render_error_callback);

  TrackAudioRenderer(const TrackAudioRenderer&) = delete;
  TrackAudioRenderer& operator=(const TrackAudioRenderer&) = delete;

  // MediaStreamAudioRenderer implementation.
  // Called on the main thread.
  void Start() override;
  void Stop() override;
  void Play() override;
  void Pause() override;
  void SetVolume(float volume) override;
  base::TimeDelta GetCurrentRenderTime() override;
  void SwitchOutputDevice(const std::string& device_id,
                          media::OutputDeviceStatusCB callback) override;

  int TotalFramesPushedForTesting() const;
  int FramesInAudioShifterForTesting() const;

 protected:
  ~TrackAudioRenderer() override;

 private:
  struct PendingData {
    PendingData(const media::AudioBus& audio_bus, base::TimeTicks ref_time);

    PendingData(PendingData&& other) = default;
    ~PendingData() = default;

    base::TimeTicks reference_time;
    std::unique_ptr<media::AudioBus> audio;
  };

  using PendingDataQueue = WTF::Deque<PendingData>;

  struct PendingReconfig {
    PendingReconfig(const media::AudioParameters& format, int generation);

    PendingDataQueue data;

    // Used for validation purposes.
    const int reconfig_number;
    const media::AudioParameters format;
  };

  using PendingReconfigQueue = WTF::Deque<PendingReconfig>;

  // WebMediaStreamAudioSink implementation.

  // Called on the AudioInputDevice worker thread.
  void OnData(const media::AudioBus& audio_bus,
              base::TimeTicks reference_time) override;

  // Called on the AudioInputDevice worker thread.
  void OnSetFormat(const media::AudioParameters& params) override;

  // media::AudioRendererSink::RenderCallback implementation.
  // Render() is called on the AudioOutputDevice thread and OnRenderError()
  // on the IO thread.
  int Render(base::TimeDelta delay,
             base::TimeTicks delay_timestamp,
             const media::AudioGlitchInfo& glitch_info,
             media::AudioBus* audio_bus) override;
  void OnRenderError() override;

  void OnRenderErrorCrossThread();

  // Initializes and starts the |sink_| if we have received valid
  // |source_params_| and we are |playing_|.
  void MaybeStartSink(bool reconfiguring = false);

  // Sets new |source_params_| and then re-initializes and restarts |sink_|.
  void ReconfigureSink(const media::AudioParameters new_format,
                       int reconfig_number);

  // Creates a new AudioShifter, destroying the old one (if any).  This is
  // called any time playback is started/stopped, or the sink changes.
  // If we are |reconfiguring|, we will push any PendingData saved in
  // |pending_reconfigs_.front()| into the new |audio_shifter_|.
  void CreateAudioShifter(bool reconfiguring);

  // Called when either the source or sink has changed somehow, or audio has
  // been paused.  Drops the AudioShifter and updates
  // |prior_elapsed_render_time_|.  May be called from either the main thread or
  // the audio thread.  Assumption: |thread_lock_| is already acquired.
  void HaltAudioFlow_Locked();

  // Takes |pending_reconfigs_.front()|, pushing its data into |audio_shifter_|.
  void ConsumePendingReconfigsFront_Locked();

  // Utility function which updates |total_frames_pushed_for_testing_| and calls
  // |audio_shifter_->push()|.
  void PushDataIntoShifter_Locked(std::unique_ptr<media::AudioBus>,
                                  base::TimeTicks);

  // The audio track which provides access to the source data to render.
  //
  // This class is calling WebMediaStreamAudioSink::AddToAudioTrack() and
  // WebMediaStreamAudioSink::RemoveFromAudioTrack() to connect and
  // disconnect with the audio track.
  Persistent<MediaStreamComponent> audio_component_;

  // The LocalFrame in which the audio is rendered into |sink_|.
  WeakPersistent<LocalFrame> playout_frame_;

  // MessageLoop associated with the single thread that performs all control
  // tasks.  Set to the MessageLoop that invoked the ctor.
  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // The sink (destination) for rendered audio.
  scoped_refptr<media::AudioRendererSink> sink_;

  // This does all the synchronization/resampling/smoothing.
  std::unique_ptr<media::AudioShifter> audio_shifter_ GUARDED_BY(thread_lock_);

  // These track the time duration of all the audio rendered so far by this
  // instance.  |prior_elapsed_render_time_| tracks the time duration of all
  // audio rendered before the last format change.  |num_samples_rendered_|
  // tracks the number of audio samples rendered since the last format change.
  base::TimeDelta prior_elapsed_render_time_ GUARDED_BY(thread_lock_);
  int64_t num_samples_rendered_ GUARDED_BY(thread_lock_) = 0;

  // The audio parameters of the track's source.
  // Must only be touched on the main thread.
  media::AudioParameters source_params_;

  base::RepeatingClosure on_render_error_callback_;

  // Set when playing, cleared when paused.
  bool playing_ = false;

  // Protects |audio_shifter_|, |prior_elapsed_render_time_|,
  // |num_samples_rendered_|, and PendingReconfigs.
  mutable base::Lock thread_lock_;

  // The preferred device id of the output device or empty for the default
  // output device.
  String output_device_id_;

  // Cache value for the volume.  Whenever |sink_| is re-created, its volume
  // should be set to this.
  float volume_ = 0.0;

  // Flag to indicate whether |sink_| has been started yet.
  bool sink_started_ = false;

  // Each entry corresponds to a posted ReconfigureSink() call. Entries are
  // queued in OnSetFormat() on the audio capture sequence, and popped in
  // ReconfigureSink() on the main thread. If this queue is not empty, there is
  // still a pending reconfiguration. In that case, we accumulate data (incoming
  // from OnData() on the capture thread) in |pending_reconfigs_.back()|, until
  // the reconfiguration completes on the main thread. Upon completing the
  // reconfiguration, accumulated data is pushed into the new |audio_shifter_|,
  // to be rendered.
  PendingReconfigQueue GUARDED_BY(thread_lock_) pending_reconfigs_;

  // Used to drop incoming ReconfigureSink() calls, by comparing the call's
  // |reconfig_number| against the latest |sink_reconfig_count_|. Incremented
  // on the audio capture sequence, and checked on the main thread.
  int sink_reconfig_count_ GUARDED_BY(thread_lock_) = 0;

  // The last format posted to the main thread, via ReconfigureSink(). Used to
  // avoid calling ReconfigureSink() when consecutive OnSetFormat() calls have
  // compatible formats.
  // Only accessed on the audio capture thread.
  media::AudioParameters last_reconfig_format_;

  int total_frames_pushed_for_testing_ GUARDED_BY(thread_lock_) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_TRACK_AUDIO_RENDERER_H_
