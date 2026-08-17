// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_PROCESSED_LOCAL_AUDIO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_PROCESSED_LOCAL_AUDIO_SOURCE_H_

#include <string>

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "media/base/audio_capturer_source.h"
#include "media/base/audio_glitch_info.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_level_calculator.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_processor_options.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_source.h"
#include "third_party/webrtc/api/media_stream_interface.h"

namespace media {
class AudioBus;
class AudioProcessorControls;
}  // namespace media

namespace blink {

class AudioServiceAudioProcessorProxy;
class LocalFrame;
class MediaStreamAudioProcessor;
class PeerConnectionDependencyFactory;

// Represents a local source of audio data that is routed through the WebRTC
// audio pipeline for post-processing (e.g., for echo cancellation during a
// video conferencing call). Owns a media::AudioCapturerSource and the
// MediaStreamProcessor that modifies its audio. Modified audio is delivered to
// one or more MediaStreamAudioTracks.
class MODULES_EXPORT ProcessedLocalAudioSource final
    : public MediaStreamAudioSource,
      public media::AudioCapturerSource::CaptureCallback {
 public:
  // |consumer_frame_| references the LocalFrame that will
  // consume the audio data. Audio parameters and (optionally) a pre-existing
  // audio session ID are derived from |device_info|. |factory| must outlive
  // this instance.
  ProcessedLocalAudioSource(
      LocalFrame& frame,
      const MediaStreamDevice& device,
      bool disable_local_echo,
      const AudioProcessingProperties& audio_processing_properties,
      int num_requested_channels,
      ConstraintsRepeatingCallback started_callback,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  ProcessedLocalAudioSource(const ProcessedLocalAudioSource&) = delete;
  ProcessedLocalAudioSource& operator=(const ProcessedLocalAudioSource&) =
      delete;

  ~ProcessedLocalAudioSource() final;

  // MediaStreamAudioSource implementation.
  void ChangeSourceImpl(const MediaStreamDevice& new_device) final;

  // If |source| is an instance of ProcessedLocalAudioSource, return a
  // type-casted pointer to it. Otherwise, return null.
  static ProcessedLocalAudioSource* From(blink::MediaStreamAudioSource* source);

  // Non-browser unit tests cannot provide RenderFrame implementations at
  // run-time. This is used to skip the otherwise mandatory check for a valid
  // render frame ID when the source is started.
  void SetAllowInvalidRenderFrameIdForTesting(bool allowed) {
    allow_invalid_render_frame_id_for_testing_ = allowed;
  }

  const blink::AudioProcessingProperties& audio_processing_properties() const {
    return audio_processing_properties_;
  }

  std::optional<blink::AudioProcessingProperties> GetAudioProcessingProperties()
      const final;

  // Valid after the source is started (when the first track is connected). Will
  // return nullptr if WebRTC stats are no available for the current
  // configuration.
  scoped_refptr<webrtc::AudioProcessorInterface> GetAudioProcessor() const;

  const scoped_refptr<blink::MediaStreamAudioLevelCalculator::Level>&
  audio_level() const {
    return level_calculator_.level();
  }

  void SetOutputDeviceForAec(const std::string& output_device_id);

 protected:
  // MediaStreamAudioSource implementation.
  void* GetClassIdentifier() const final;
  bool EnsureSourceIsStarted() final;
  void EnsureSourceIsStopped() final;

  // AudioCapturerSource::CaptureCallback implementation.
  // Called on the AudioCapturerSource audio thread.
  void OnCaptureStarted() override;
  void Capture(const media::AudioBus* audio_source,
               base::TimeTicks audio_capture_time,
               const media::AudioGlitchInfo& glitch_info,
               double volume) override;
  void OnCaptureError(media::AudioCapturerSource::ErrorCode code,
                      const std::string& message) override;
  void OnCaptureMuted(bool is_muted) override;
  void OnCaptureProcessorCreated(
      media::AudioProcessorControls* controls) override;

 private:
  // Receive and forward processed capture audio. Called on the same thread as
  // Capture().
  void DeliverProcessedAudio(const media::AudioBus& processed_audio,
                             base::TimeTicks audio_capture_time,
                             std::optional<double> new_volume);

  // Update the device (source) mic volume.
  void SetVolume(double volume);

  // Helper method which sends the log |message| to a native WebRTC log and
  // adds the current session ID (from the associated media stream device) to
  // make the log unique.
  void SendLogMessageWithSessionId(const std::string& message) const;

  // If true, processing (controlled via |audio_processor_proxy_|) is done in
  // the audio service (and Chrome-wide echo cancellation is applied if
  // requested; otherwise, |media_stream_audio_processor_| will be applying
  // audio processing locally, and if echo cancellation is requested then only
  // PeerConnection audio from the same context as |this| is cancelled.
  const bool use_remote_apm_;

  // The LocalFrame that will consume the audio data. Used when creating
  // AudioCapturerSources.
  //
  // TODO(crbug.com/704136): Consider moving ProcessedLocalAudioSource to
  // Oilpan and use Member<> here.
  WeakPersistent<LocalFrame> consumer_frame_;
  WeakPersistent<PeerConnectionDependencyFactory> dependency_factory_;

  blink::AudioProcessingProperties audio_processing_properties_;
  int num_requested_channels_;

  // Callback that's called when the audio source has been initialized.
  ConstraintsRepeatingCallback started_callback_;

  // At most one of |audio_processor_| and |audio_processor_proxy_| can be set.

  // Audio processor doing software processing like FIFO, AGC, AEC and NS. Its
  // output data is in a unit of up to 10 ms data chunk.
  scoped_refptr<MediaStreamAudioProcessor> media_stream_audio_processor_;

  // Proxy for the audio processor when it's run in the Audio Service process,
  scoped_refptr<AudioServiceAudioProcessorProxy> audio_processor_proxy_;

  // The device created by the AudioDeviceFactory in EnsureSourceIsStarted().
  scoped_refptr<media::AudioCapturerSource> source_;

  // Used to calculate the signal level that shows in the UI.
  blink::MediaStreamAudioLevelCalculator level_calculator_;

  // Used to signal non-silent mic input to the level calculator, when there is
  // a risk that the audio processor will zero it out.
  // Is only accessed on the audio capture thread.
  bool force_report_nonzero_energy_ = false;

  bool allow_invalid_render_frame_id_for_testing_;

  media::AudioGlitchInfo::Accumulator glitch_info_accumulator_;

  // Provides weak pointers for tasks posted by this instance.
  base::WeakPtrFactory<ProcessedLocalAudioSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_PROCESSED_LOCAL_AUDIO_SOURCE_H_
