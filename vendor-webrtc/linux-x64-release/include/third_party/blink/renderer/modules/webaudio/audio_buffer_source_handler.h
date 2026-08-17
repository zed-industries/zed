// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_HANDLER_H_

#include <atomic>
#include <memory>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/modules/webaudio/audio_buffer.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/modules/webaudio/audio_scheduled_source_node.h"
#include "third_party/blink/renderer/modules/webaudio/panner_node.h"
#include "third_party/blink/renderer/platform/audio/audio_bus.h"
#include "third_party/blink/renderer/platform/wtf/threading.h"

namespace blink {

class AudioBufferSourceOptions;
class BaseAudioContext;

class AudioBufferSourceHandler final : public AudioScheduledSourceHandler {
 public:
  static scoped_refptr<AudioBufferSourceHandler> Create(
      AudioNode&,
      float sample_rate,
      AudioParamHandler& playback_rate,
      AudioParamHandler& detune);
  ~AudioBufferSourceHandler() override;

  // AudioHandler
  void Process(uint32_t frames_to_process) override;

  // setBuffer() is called on the main thread. This is the buffer we use for
  // playback.
  void SetBuffer(AudioBuffer*, ExceptionState&);
  SharedAudioBuffer* Buffer() { return shared_buffer_.get(); }

  // numberOfChannels() returns the number of output channels.  This value
  // equals the number of channels from the buffer.  If a new buffer is set with
  // a different number of channels, then this value will dynamically change.
  unsigned NumberOfChannels();

  // Play-state
  void Start(double when, ExceptionState&);
  void Start(double when, double grain_offset, ExceptionState&);
  void Start(double when,
             double grain_offset,
             double grain_duration,
             ExceptionState&);

  // Note: the attribute was originally exposed as `.looping`, but to be more
  // consistent in naming with <audio> and with how it's described in the
  // specification, the proper attribute name is `.loop`. The old attribute is
  // kept for backwards compatibility.
  bool Loop() const { return is_looping_; }
  void SetLoop(bool looping);

  // Loop times in seconds.
  double LoopStart() const { return loop_start_; }
  double LoopEnd() const { return loop_end_; }
  void SetLoopStart(double loop_start);
  void SetLoopEnd(double loop_end);

  // If we are no longer playing, propagate silence ahead to downstream nodes.
  bool PropagatesSilence() const override;

  void HandleStoppableSourceNode() override;

 private:
  AudioBufferSourceHandler(AudioNode&,
                           float sample_rate,
                           AudioParamHandler& playback_rate,
                           AudioParamHandler& detune);
  void StartSource(double when,
                   double grain_offset,
                   double grain_duration,
                   bool is_duration_given,
                   ExceptionState&);

  // Render audio directly from the buffer to the audio bus. Returns true on
  // success, i.e., audio was written to the output bus because all the internal
  // checks passed.
  //
  //   output_bus -
  //     AudioBus where the rendered audio goes.
  //   destination_frame_offset -
  //     Index into the output bus where the first frame should be written.
  //   number_of_frames -
  //     Maximum number of frames to process; this can be less that a render
  //     quantum.
  //   start_time_offset -
  //     Actual start time relative to the `destination_frame_offset`.  This
  //     should be the `start_time_offset` value returned by
  //     `UpdateSchedulingInfo`.
  bool RenderFromBuffer(AudioBus* output_bus,
                        unsigned destination_frame_offset,
                        uint32_t number_of_frames,
                        double start_time_offset);

  // Render silence starting from "index" frame in AudioBus.
  inline bool RenderSilenceAndFinishIfNotLooping(AudioBus*,
                                                 unsigned index,
                                                 uint32_t frames_to_process);

  // Clamps grain parameters to the duration of the given AudioBuffer.
  void ClampGrainParameters(const SharedAudioBuffer*)
      EXCLUSIVE_LOCKS_REQUIRED(process_lock_);

  base::WeakPtr<AudioScheduledSourceHandler> AsWeakPtr() override;

  // Sample data for the outputs of this node. The shared buffer can safely be
  // accessed from the audio thread.
  std::unique_ptr<SharedAudioBuffer> shared_buffer_;

  // Pointers for the buffer and destination.
  std::unique_ptr<const float*[]> source_channels_;
  std::unique_ptr<float*[]> destination_channels_;

  scoped_refptr<AudioParamHandler> playback_rate_;
  scoped_refptr<AudioParamHandler> detune_;

  bool DidSetLooping() const { return did_set_looping_; }
  void SetDidSetLooping(bool loop) {
    if (loop) {
      did_set_looping_ = true;
    }
  }

  // If `is_looping_` is false, then this node will be done playing and become
  // inactive after it reaches the end of the sample data in the buffer.  If
  // true, it will wrap around to the start of the buffer each time it reaches
  // the end.
  //
  // A process lock must be used to protect access.
  bool is_looping_ = false;

  // True if the source .loop attribute was ever set.
  // A process lock must be used to protect access.
  bool did_set_looping_ = false;

  // A process lock must be used to protect access to both `loop_start_` and
  // `loop_end_`.
  double loop_start_ = 0;
  double loop_end_ = 0;

  // `virtual_read_index_` is a sample-frame index into our buffer representing
  // the current playback position.  Since it's floating-point, it has
  // sub-sample accuracy.
  double virtual_read_index_ = 0;

  // Granular playback
  bool is_grain_ = false;
  double grain_offset_ = 0.0;  // in seconds
  double grain_duration_;      // in seconds
  // True if `grain_duration_` is given explicitly (via 3 arg start method).
  bool is_duration_given_;

  // Compute playback rate (k-rate) by incorporating the sample rate
  // conversion factor, and the value of playbackRate and detune AudioParams.
  double ComputePlaybackRate();

  double GetMinPlaybackRate();

  // The minimum playbackRate value ever used for this source.
  double min_playback_rate_ = 1.0;

  // True if the `buffer` attribute has ever been set to a non-null
  // value.  Defaults to false.
  bool buffer_has_been_set_ = false;

  base::WeakPtrFactory<AudioScheduledSourceHandler> weak_ptr_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_BUFFER_SOURCE_HANDLER_H_
