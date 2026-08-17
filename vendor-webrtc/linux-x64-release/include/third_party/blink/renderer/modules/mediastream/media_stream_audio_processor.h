// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_AUDIO_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_AUDIO_PROCESSOR_H_

#include <memory>
#include <optional>

#include "base/files/file.h"
#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "media/base/audio_parameters.h"
#include "media/webrtc/audio_processor.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/mediastream/aec_dump_agent_impl.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_audio_processor_options.h"
#include "third_party/blink/renderer/platform/webrtc/webrtc_source.h"
#include "third_party/webrtc/api/media_stream_interface.h"
#include "third_party/webrtc/modules/audio_processing/include/audio_processing.h"

namespace media {
class AudioBus;
struct AudioProcessingSettings;
}  // namespace media

namespace blink {

class AecDumpAgentImpl;
class WebRtcAudioDeviceImpl;

using webrtc::AudioProcessorInterface;

// This class owns a media::AudioProcessor which processes captured audio.
// MediaStreamAudioProcessor manages subscriptions for echo cancellation
// playout reference audio, diagnostic recording communication with the browser,
// and renderer-specific threading requirements. All processing functionality is
// delegated to the media::AudioProcessor.
class MODULES_EXPORT MediaStreamAudioProcessor
    : public WebRtcPlayoutDataSource::Sink,
      public AudioProcessorInterface,
      public AecDumpAgentImpl::Delegate {
 public:
  // Callback for consuming processed capture audio.
  using DeliverProcessedAudioCallback =
      media::AudioProcessor::DeliverProcessedAudioCallback;

  // |deliver_processed_audio_callback| is used to deliver frames of processed
  // capture audio, from ProcessCapturedAudio(), and has to be valid until
  // Stop() is called. |playout_data_source| is used to register this class as a
  // sink to the WebRtc playout data for processing AEC. If clients do not
  // enable AEC, |playout_data_source| won't be used.
  //
  // Threading note: The constructor assumes it is being run on the main render
  // thread.
  MediaStreamAudioProcessor(
      DeliverProcessedAudioCallback deliver_processed_audio_callback,
      const media::AudioProcessingSettings& settings,
      const media::AudioParameters& capture_data_source_params,
      scoped_refptr<WebRtcAudioDeviceImpl> playout_data_source);

  MediaStreamAudioProcessor(const MediaStreamAudioProcessor&) = delete;
  MediaStreamAudioProcessor& operator=(const MediaStreamAudioProcessor&) =
      delete;

  // Processes and delivers capture audio,
  // See media::AudioProcessor::ProcessCapturedAudio for API details.
  // Must be called on the capture audio thread.
  void ProcessCapturedAudio(const media::AudioBus& audio_source,
                            base::TimeTicks audio_capture_time,
                            int num_preferred_channels,
                            double volume);

  // Stops the audio processor. The caller guarantees that there will be no more
  // calls to ProcessCapturedAudio(). Calling Stop() stops any ongoing aecdump
  // recordings and playout audio analysis.
  void Stop();

  // The format of the processed capture output audio from the processor.
  // Is constant throughout MediaStreamAudioProcessor lifetime.
  const media::AudioParameters& output_format() const {
    return audio_processor_->output_format();
  }

  // Accessor to check if WebRTC audio processing is enabled or not.
  bool has_webrtc_audio_processing() const {
    return audio_processor_->has_webrtc_audio_processing();
  }

  // AecDumpAgentImpl::Delegate implementation.
  // Called on the main render thread.
  void OnStartDump(base::File dump_file) override;
  void OnStopDump() override;

  // Returns true if MediaStreamAudioProcessor would modify the audio signal,
  // based on |properties|. If the audio signal would not be modified, there is
  // no need to instantiate a MediaStreamAudioProcessor and feed audio through
  // it. Doing so would waste a non-trivial amount of memory and CPU resources.
  static bool WouldModifyAudio(const AudioProcessingProperties& properties);

 protected:
  ~MediaStreamAudioProcessor() override;

 private:
  class PlayoutListener;
  friend class MediaStreamAudioProcessorTest;

  // Format of input to ProcessCapturedAudio().
  const media::AudioParameters& GetInputFormatForTesting() const;

  // WebRtcPlayoutDataSource::Sink implementation.
  void OnPlayoutData(media::AudioBus* audio_bus,
                     int sample_rate,
                     base::TimeDelta audio_delay) override;
  void OnPlayoutDataSourceChanged() override;
  void OnRenderThreadChanged() override;

  std::optional<webrtc::AudioProcessing::Config>
  GetAudioProcessingModuleConfigForTesting() const {
    return audio_processor_->GetAudioProcessingModuleConfigForTesting();
  }

  // This method is called on the libjingle thread.
  // TODO(webrtc:5298): |has_remote_tracks| is no longer used, remove it.
  AudioProcessorStatistics GetStats(bool has_remote_tracks) override;

  // Handles audio processing, rebuffering, and input/output formatting.
  const std::unique_ptr<media::AudioProcessor> audio_processor_;

  // Manages subscription to the playout reference audio. Must be outlived by
  // |audio_processor_|.
  std::unique_ptr<PlayoutListener> playout_listener_;

  // Task runner for the main render thread.
  const scoped_refptr<base::SingleThreadTaskRunner> main_thread_runner_;

  // Used to DCHECK that some methods are called on the capture audio thread.
  THREAD_CHECKER(capture_thread_checker_);
  // Used to DCHECK that some methods are called on the render audio thread.
  THREAD_CHECKER(render_thread_checker_);

  // Communication with browser for AEC dump.
  std::unique_ptr<AecDumpAgentImpl> aec_dump_agent_impl_;

  // Flag to avoid executing Stop() more than once.
  bool stopped_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_AUDIO_PROCESSOR_H_
