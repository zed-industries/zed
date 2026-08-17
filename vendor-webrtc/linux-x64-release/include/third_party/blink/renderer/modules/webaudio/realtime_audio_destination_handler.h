// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_HANDLER_H_

#include <atomic>
#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "third_party/blink/public/platform/web_audio_latency_hint.h"
#include "third_party/blink/public/platform/web_audio_sink_descriptor.h"
#include "third_party/blink/renderer/modules/webaudio/audio_context.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_node.h"
#include "third_party/blink/renderer/platform/audio/audio_callback_metric_reporter.h"
#include "third_party/blink/renderer/platform/audio/audio_destination.h"
#include "third_party/blink/renderer/platform/audio/audio_io_callback.h"

namespace blink {

class ExceptionState;
class WebAudioLatencyHint;
class WebAudioSinkDescriptor;

class MODULES_EXPORT RealtimeAudioDestinationHandler final
    : public AudioDestinationHandler,
      public AudioIOCallback {
 public:
  static scoped_refptr<RealtimeAudioDestinationHandler> Create(
      AudioNode&,
      const WebAudioSinkDescriptor&,
      const WebAudioLatencyHint&,
      std::optional<float> sample_rate,
      bool update_echo_cancellation_on_first_start);
  ~RealtimeAudioDestinationHandler() override;

  // For AudioHandler.
  void Dispose() override;
  AudioContext* Context() const override;
  void Initialize() override;
  void Uninitialize() override;
  void SetChannelCount(unsigned, ExceptionState&) override;
  bool RequiresTailProcessing() const override { return false; }
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }

  // For AudioDestinationHandler.
  void StartRendering() override;
  void StopRendering() override;
  void Pause() override;
  void Resume() override;
  void RestartRendering() override;
  void PrepareTaskRunnerForWorklet() override;
  double SampleRate() const override;
  uint32_t MaxChannelCount() const override;

  // For AudioIOCallback. This is invoked by the platform audio destination to
  // get the next render quantum into `destination_bus` and update
  // `output_position`.
  void Render(AudioBus* destination_bus,
              uint32_t number_of_frames,
              const AudioIOPosition& output_position,
              const AudioCallbackMetric& metric,
              base::TimeDelta playout_delay,
              const media::AudioGlitchInfo& glitch_info) override;

  // For AudioIOCallback. This is invoked by AudioDestination to notify when
  // an error has occurred in the audio infra.
  void OnRenderError() override;

  // Returns a hardware callback buffer size from audio infra.
  uint32_t GetCallbackBufferSize() const;

  // Returns a given frames-per-buffer size from audio infra.
  int GetFramesPerBuffer() const;

  base::TimeDelta GetPlatformBufferDuration() const;

  bool IsPullingAudioGraphAllowed() const {
    return allow_pulling_audio_graph_.load(std::memory_order_acquire);
  }

  // Sets the identifier for a new output device. Note that this will recreate
  // a new platform destination with the specified sink device. It also invokes
  // `callback` when the recreation is completed.
  void SetSinkDescriptor(const WebAudioSinkDescriptor& sink_descriptor,
                         media::OutputDeviceStatusCB callback);

  // Testing purposes only.
  void invoke_onrendererror_from_platform_for_testing();
  bool is_silence_detection_active_for_testing() const {
    return is_silence_detection_active_for_testing_;
  }
  bool get_platform_destination_is_playing_for_testing();

 private:
  explicit RealtimeAudioDestinationHandler(
      AudioNode&,
      const WebAudioSinkDescriptor&,
      const WebAudioLatencyHint&,
      std::optional<float> sample_rate,
      bool update_echo_cancellation_on_first_start);

  // Sets the detect silence flag for the platform destination.
  void SetDetectSilence(bool detect_silence);

  void CreatePlatformDestination();
  void StartPlatformDestination();
  void StopPlatformDestination();

  // Checks the current silent detection condition (e.g. the number of
  // automatic pull nodes) and flips the switch if necessary. Called within the
  // Render() method.
  void SetDetectSilenceIfNecessary(bool has_automatic_pull_nodes);

  // Should only be called from StartPlatformDestination.
  void EnablePullingAudioGraph() {
    allow_pulling_audio_graph_.store(true, std::memory_order_release);
  }

  // Should only be called from StopPlatformDestination.
  void DisablePullingAudioGraph() {
    allow_pulling_audio_graph_.store(false, std::memory_order_release);
  }

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name,
                      const String& message) const;

  // Stores a sink descriptor for sink transition.
  WebAudioSinkDescriptor sink_descriptor_;

  const WebAudioLatencyHint latency_hint_;

  // Holds the audio device thread that runs the real time audio context.
  scoped_refptr<AudioDestination> platform_destination_;

  // Stores the user-provided (AudioContextOptions) sample rate. When `nullopt`
  // it is updated with the sample rate of the first platform destination.
  std::optional<float> sample_rate_;

  // If true, the audio graph will be pulled to get new data.  Otherwise, the
  // graph is not pulled, even if the audio thread is still running and
  // requesting data.
  //
  // Must be modified only in StartPlatformDestination (via
  // EnablePullingAudioGraph) or StopPlatformDestination (via
  // DisablePullingAudioGraph). This is modified only by the main thread and
  // the audio thread only reads this.
  std::atomic_bool allow_pulling_audio_graph_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Represents the current condition of silence detection. By default, the
  // silence detection is active. Access only from the audio render thread.
  bool is_detecting_silence_ = true;

  // Represents the silence detection status for integration tests. Access only
  // from the main thread.
  bool is_silence_detection_active_for_testing_ = true;

  // If true, attempt to update the echo cancellation reference the next time
  // the platform destination is started.
  bool update_echo_cancellation_on_next_start_ = false;

  base::WeakPtrFactory<RealtimeAudioDestinationHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_REALTIME_AUDIO_DESTINATION_HANDLER_H_
