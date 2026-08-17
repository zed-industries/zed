// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_SCHEDULED_SOURCE_HANDLER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_SCHEDULED_SOURCE_HANDLER_H_

#include <atomic>

#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"

namespace blink {

class BaseAudioContext;
class AudioBus;

class AudioScheduledSourceHandler : public AudioHandler {
 public:
  // These are the possible states an AudioScheduledSourceNode can be in:
  //
  // UNSCHEDULED_STATE - Initial playback state. Created, but not yet scheduled.
  // SCHEDULED_STATE - Scheduled to play (via start()), but not yet playing.
  // PLAYING_STATE - Generating sound.
  // FINISHED_STATE - Finished generating sound.
  //
  // The state can only transition to the next state, except for the
  // FINISHED_STATE which can never be changed.
  enum PlaybackState {
    // These must be defined with the same names and values as in the .idl file.
    UNSCHEDULED_STATE = 0,
    SCHEDULED_STATE = 1,
    PLAYING_STATE = 2,
    FINISHED_STATE = 3
  };

  AudioScheduledSourceHandler(NodeType, AudioNode&, float sample_rate);

  // Scheduling.
  void Start(double when, ExceptionState&);
  void Stop(double when, ExceptionState&);

  // AudioNode
  double TailTime() const override { return 0; }
  double LatencyTime() const override { return 0; }

  PlaybackState GetPlaybackState() const {
    return playback_state_.load(std::memory_order_acquire);
  }

  void SetPlaybackState(PlaybackState new_state) {
    playback_state_.store(new_state, std::memory_order_release);
  }

  bool IsPlayingOrScheduled() const {
    PlaybackState state = GetPlaybackState();
    return state == PLAYING_STATE || state == SCHEDULED_STATE;
  }

  bool HasFinished() const { return GetPlaybackState() == FINISHED_STATE; }

  // Source nodes don't have tail or latency times so no tail
  // processing needed.
  bool RequiresTailProcessing() const final { return false; }

  // Stops a node if the stop time has passed.  Generally used when a source
  // node has been scheduled to start but has disconnected some time before the
  // stop time has arrived.  Nodes that are not reachable from the destination
  // don't progress in time so we need to handle this specially.
  virtual void HandleStoppableSourceNode() = 0;

  // Returns true if the onended event has not been sent (is pending).
  bool IsOnEndedNotificationPending() const {
    return on_ended_notification_pending_;
  }

  void SetOnEndedNotificationPending() {
    on_ended_notification_pending_ = true;
  }

 protected:
  // Get frame information for the current time quantum.
  // We handle the transition into PLAYING_STATE and FINISHED_STATE here,
  // zeroing out portions of the outputBus which are outside the range of
  // startFrame and endFrame.
  //
  // Each frame time is relative to the context's currentSampleFrame().  Three
  // values are returned, in this order:
  //
  // quantumFrameOffset    : Offset frame in this time quantum to start
  //                         rendering.
  // nonSilentFramesToProcess : Number of frames rendering non-silence (will be
  //                            <= quantumFrameSize).
  // startFrameOffset : The fractional frame offset from quantumFrameOffset
  //                    and the actual starting time of the source. This is
  //                    non-zero only when transitioning from the
  //                    SCHEDULED_STATE to the PLAYING_STATE.
  //
  // Callers should check nonSilentFramesToProcess to decide what to
  // do.  If it is zero, the other return values are meaningless.
  std::tuple<size_t, size_t, double> UpdateSchedulingInfo(
      size_t quantum_frame_size,
      AudioBus* output_bus) EXCLUSIVE_LOCKS_REQUIRED(process_lock_);

  // Called when we have no more sound to play or the stop() time has been
  // reached. No onEnded event is called.
  virtual void FinishWithoutOnEnded();

  // Like finishWithoutOnEnded(), but an onEnded (if specified) is called.
  virtual void Finish();

  void NotifyEnded();

  // This synchronizes with process() and any other method that needs to be
  // synchronized like setBuffer for AudioBufferSource.
  mutable base::Lock process_lock_;

  // |start_time_| is the time to start playing based on the context's timeline
  // (0 or a time less than the context's current time means "now").
  double start_time_ GUARDED_BY(process_lock_) = 0;  // in seconds

  // |end_time_| is the time to stop playing based on the context's timeline (0
  // or a time less than the context's current time means "now").  If it hasn't
  // been set explicitly, then the sound will not stop playing (if looping) or
  // will stop when the end of the AudioBuffer has been reached.
  double end_time_ GUARDED_BY(process_lock_);  // in seconds

  // Magic value indicating that the time (start or end) has not yet been set.
  static constexpr double kUnknownTime = -1.0;

  // Number of extra frames to use when determining if a source node can be
  // stopped.  This should be at least one rendering quantum, but we add one
  // more quantum for good measure.  This doesn't need to be extra precise, just
  // more than one rendering quantum.  See `HandleStoppableSourceNode()`.
  static constexpr int kExtraStopFrames = 256;

 private:
  virtual base::WeakPtr<AudioScheduledSourceHandler> AsWeakPtr() = 0;

  // This is accessed by both the main thread and audio thread.  Use the setter
  // and getter to protect the access to this.
  std::atomic<PlaybackState> playback_state_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Onended event is pending.  Becomes true when the node is started,
  // and is false when the onended event is fired.
  bool on_ended_notification_pending_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_SCHEDULED_SOURCE_HANDLER_H_
