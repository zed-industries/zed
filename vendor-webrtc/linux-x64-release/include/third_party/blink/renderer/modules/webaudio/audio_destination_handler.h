// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_HANDLER_H_

#include <atomic>

#include "base/notreached.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

// The AudioDestinationHandler (ADH) is a base class for the rendering backend
// for AudioDestinatioNode. It contains common information required for the
// rendering such as current sample frame, sample rate and maximum channel count
// of the context.
class AudioDestinationHandler : public AudioHandler {
 public:
  explicit AudioDestinationHandler(AudioNode&);
  ~AudioDestinationHandler() override;

  // The method MUST NOT be invoked when rendering a graph because the
  // destination node is a sink. Instead, this node gets pulled by the
  // underlying renderer (audio hardware or worker thread).
  void Process(uint32_t) final { NOTREACHED(); }

  virtual void StartRendering() = 0;
  virtual void StopRendering() = 0;
  virtual void Pause() = 0;
  virtual void Resume() = 0;

  // The render thread needs to be changed after Worklet JS code is loaded by
  // AudioWorklet. This method ensures the switching of render thread and the
  // restart of the context.
  virtual void RestartRendering() = 0;

  // The worklet thread change can happen when a context/destination is
  // suspended. In that case, we prepare the worklet operation but do not start
  // running.
  virtual void PrepareTaskRunnerForWorklet() = 0;

  size_t CurrentSampleFrame() const {
    return current_sample_frame_.load(std::memory_order_acquire);
  }

  double CurrentTime() const { return CurrentSampleFrame() / SampleRate(); }

  virtual double SampleRate() const = 0;
  virtual uint32_t MaxChannelCount() const = 0;

  void ContextDestroyed() { is_execution_context_destroyed_ = true; }
  bool IsExecutionContextDestroyed() const {
    return is_execution_context_destroyed_;
  }

 protected:
  void AdvanceCurrentSampleFrame(size_t number_of_frames) {
    current_sample_frame_.fetch_add(number_of_frames,
                                    std::memory_order_release);
  }

 private:
  // The number of sample frames processed by the destination so far.
  std::atomic_size_t current_sample_frame_{0};

  // True if the execution context is being destroyed.  If this is true, the
  // destination node must avoid checking for or accessing the execution
  // context.
  bool is_execution_context_destroyed_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_DESTINATION_HANDLER_H_
