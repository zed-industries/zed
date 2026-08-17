// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_DESTINATION_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_DESTINATION_HANDLER_H_

#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/modules/webaudio/audio_buffer.h"
#include "third_party/blink/renderer/modules/webaudio/audio_destination_node.h"
#include "third_party/blink/renderer/modules/webaudio/offline_audio_context.h"
#include "third_party/blink/renderer/platform/scheduler/public/non_main_thread.h"

namespace blink {

class BaseAudioContext;
class AudioBus;
class OfflineAudioContext;

class OfflineAudioDestinationHandler final : public AudioDestinationHandler {
 public:
  static scoped_refptr<OfflineAudioDestinationHandler> Create(
      AudioNode&,
      unsigned number_of_channels,
      uint32_t frames_to_process,
      float sample_rate);
  ~OfflineAudioDestinationHandler() override;

  // AudioHandler
  void Dispose() override;
  void Initialize() override;
  void Uninitialize() override;

  // AudioNode
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }

  OfflineAudioContext* Context() const final;

  // AudioDestinationHandler
  void StartRendering() override;
  void StopRendering() override;
  void Pause() override;
  void Resume() override;
  uint32_t MaxChannelCount() const override;
  void PrepareTaskRunnerForWorklet() override {}

  void RestartRendering() override;

  double SampleRate() const override { return sample_rate_; }

  // This is called when rendering of the offline context is started
  // which will save the rendered audio data in `render_target`.  This
  // allows creation of the AudioBuffer when startRendering is called
  // instead of when the OfflineAudioContext is created.
  void InitializeOfflineRenderThread(AudioBuffer* render_target);

  unsigned NumberOfChannels() const { return number_of_channels_; }

  bool RequiresTailProcessing() const final { return false; }

 private:
  OfflineAudioDestinationHandler(AudioNode&,
                                 unsigned number_of_channels,
                                 uint32_t frames_to_process,
                                 float sample_rate);

  // Set up the rendering and start. After setting the context up, it will
  // eventually call `DoOfflineRendering()`.
  void StartOfflineRendering();

  // Suspend the rendering loop and notify the main thread to resolve the
  // associated promise.
  void SuspendOfflineRendering();

  // Start the rendering loop.
  void DoOfflineRendering();

  // Finish the rendering loop and notify the main thread to resolve the
  // promise with the rendered buffer.
  void FinishOfflineRendering();

  // Suspend/completion callbacks for the main thread.
  void NotifySuspend(size_t);
  void NotifyComplete();

  // The offline version of render() method. If the rendering needs to be
  // suspended after checking, this stops the rendering and returns true.
  // Otherwise, it returns false after rendering one quantum.
  bool RenderIfNotSuspended(AudioBus* source_bus,
                            AudioBus* destination_bus,
                            uint32_t number_of_frames);

  // Prepares a task runner for the rendering based on the operation mode
  // (i.e. non-AudioWorklet or AudioWorklet). This is called when the
  // rendering restarts such as context.resume() after context.suspend().
  // The only possible transition is from the non-AudioWorklet mode to the
  // AudioWorklet mode. Once the AudioWorklet mode is activated, the task runner
  // from AudioWorkletThread will be used until the rendering is finished.
  void PrepareTaskRunnerForRendering();

  // For cross-thread posting, this object uses two different targets.
  // 1. rendering thread -> main thread: WeakPtr
  //    When the main thread starts deleting this object, a task posted with
  //    a WeakPtr from the rendering thread will be cancelled.
  // 2. main thread -> rendering thread: scoped_refptr
  //    `render_thread_` is owned by this object, so it is safe to target with
  //    `WrapRefCounted()` instead of `GetWeakPtr()`.
  base::WeakPtr<OfflineAudioDestinationHandler> GetWeakPtr() {
    return weak_factory_.GetWeakPtr();
  }

  // This AudioHandler renders into this SharedAudioBuffer.
  std::unique_ptr<SharedAudioBuffer> shared_render_target_;
  // Temporary AudioBus for each render quantum.
  scoped_refptr<AudioBus> render_bus_;

  // These variables are for counting the number of frames for the current
  // progress and the remaining frames to be processed.
  size_t frames_processed_ = 0;
  uint32_t frames_to_process_;

  // This flag is necessary to distinguish the state of the context between
  // 'created' and 'suspended'. If this flag is false and the current state
  // is 'suspended', it means the context is created and have not started yet.
  bool is_rendering_started_ = false;

  unsigned number_of_channels_;
  float sample_rate_;

  // The rendering thread for the non-AudioWorklet mode. For the AudioWorklet
  // node, AudioWorkletThread will drive the rendering.
  std::unique_ptr<NonMainThread> render_thread_;

  scoped_refptr<base::SingleThreadTaskRunner> render_thread_task_runner_;
  scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner_;

  base::WeakPtrFactory<OfflineAudioDestinationHandler> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_OFFLINE_AUDIO_DESTINATION_HANDLER_H_
