// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_HANDLER_H_

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_worklet_node_options.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param_map.h"
#include "third_party/blink/renderer/modules/webaudio/audio_worklet_processor_error_state.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

class AudioNodeInput;
class AudioWorkletProcessor;
class BaseAudioContext;
class CrossThreadAudioParamInfo;
class ExceptionState;
class MessagePort;
class ScriptState;

class AudioWorkletHandler final : public AudioHandler {
 public:
  static scoped_refptr<AudioWorkletHandler> Create(
      AudioNode&,
      float sample_rate,
      String name,
      HashMap<String, scoped_refptr<AudioParamHandler>> param_handler_map,
      const AudioWorkletNodeOptions*);

  ~AudioWorkletHandler() override;

  // Called from render thread.
  void Process(uint32_t frames_to_process) override;

  void CheckNumberOfChannelsForInput(AudioNodeInput*) override;
  void UpdatePullStatusIfNeeded() override;

  double TailTime() const override;
  double LatencyTime() const override { return 0; }

  String Name() const { return name_; }

  // Sets `AudioWorkletProcessor` and changes the state of the processor.
  // MUST be called from the render thread.
  void SetProcessorOnRenderThread(AudioWorkletProcessor*);

  // Finish `AudioWorkletProcessor` and set the tail time to zero, when
  // the user-supplied `process()` method returns false.
  void FinishProcessorOnRenderThread();

  void NotifyProcessorError(AudioWorkletProcessorErrorState);

  void MarkProcessorInactiveOnMainThread();
  bool IsProcessorActive() { return is_processor_active_; }

 private:
  AudioWorkletHandler(
      AudioNode&,
      float sample_rate,
      String name,
      HashMap<String, scoped_refptr<AudioParamHandler>> param_handler_map,
      const AudioWorkletNodeOptions*);

  // Used to avoid code duplication when using scoped objects that affect
  // `Process`.
  void ProcessInternal(uint32_t frames_to_process);

  String name_;

  double tail_time_ = std::numeric_limits<double>::infinity();

  // MUST be set/used by render thread.
  CrossThreadPersistent<AudioWorkletProcessor> processor_;

  // Keeps the reference of AudioBus objects from AudioNodeInput and
  // AudioNodeOutput in order to pass them to AudioWorkletProcessor.
  Vector<scoped_refptr<AudioBus>> inputs_;
  Vector<scoped_refptr<AudioBus>> outputs_;

  // For unconnected outputs, the handler needs to provide an AudioBus object
  // to the AudioWorkletProcessor.
  Vector<scoped_refptr<AudioBus>> unconnected_outputs_;

  HashMap<String, scoped_refptr<AudioParamHandler>> param_handler_map_;
  HashMap<String, std::unique_ptr<AudioFloatArray>> param_value_map_;

  // TODO(crbug.com/1447088): The tail time of AudioWorkletNode is decided by
  // the active processing flag. So it doesn't need an automatic tail time
  // management from the renderer.
  bool RequiresTailProcessing() const override { return true; }

  scoped_refptr<base::SingleThreadTaskRunner> main_thread_task_runner_;

  // Used only if number of inputs and outputs are 1.
  bool is_output_channel_count_given_ = false;

  // The active flag of the AudioWorkletProcessor is used to decide the
  // lifecycle of an AudioWorkletNode and its handler. This flag becomes false
  // when a processor stops invoking the user-defined `process()` callback.
  bool is_processor_active_ = true;

  // Cached feature flag value
  const bool allow_denormal_in_processing_;

  base::WeakPtrFactory<AudioWorkletHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_WORKLET_HANDLER_H_
