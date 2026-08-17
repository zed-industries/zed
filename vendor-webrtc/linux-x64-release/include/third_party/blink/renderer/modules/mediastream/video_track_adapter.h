// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_TRACK_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_TRACK_ADAPTER_H_

#include <stdint.h>

#include <optional>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/time.h"
#include "media/base/video_frame.h"
#include "third_party/blink/public/platform/modules/mediastream/media_stream_types.h"
#include "third_party/blink/public/web/modules/mediastream/encoded_video_frame.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_video_track.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc_overrides/low_precision_timer.h"

namespace blink {

class VideoTrackAdapterSettings;

// VideoTrackAdapter is a helper class used by MediaStreamVideoSource used for
// adapting the video resolution from a source implementation to the resolution
// a track requires. Different tracks can have different resolution constraints.
// The constraints can be set as max width and height as well as max and min
// aspect ratio.
// Video frames are delivered to a track using a VideoCaptureDeliverFrameCB on
// the video task runner.
// Adaptations is done by wrapping the original media::VideoFrame in a new
// media::VideoFrame with a new visible_rect and natural_size.
class MODULES_EXPORT VideoTrackAdapter
    : public WTF::ThreadSafeRefCounted<VideoTrackAdapter> {
 public:
  using OnMutedCallback = base::RepeatingCallback<void(bool mute_state)>;

  VideoTrackAdapter(
      scoped_refptr<base::SequencedTaskRunner> video_task_runner,
      base::WeakPtr<MediaStreamVideoSource> media_stream_video_source);

  VideoTrackAdapter(const VideoTrackAdapter&) = delete;
  VideoTrackAdapter& operator=(const VideoTrackAdapter&) = delete;

  // Register |track| to receive video frames in and |encoded_frame_callback|
  // and in |frame_callback| with a resolution within the boundaries of the
  // arguments, and settings updates in |settings_callback|. Must be called on
  // the main render thread. |source_frame_rate| is used to calculate a prudent
  // interval to check for passing frames and inform of the result via
  // |on_muted_state_callback|.
  void AddTrack(const MediaStreamVideoTrack* track,
                MediaStreamVideoSourceCallbacks video_stream_fallbacks,
                const VideoTrackAdapterSettings& settings);
  void RemoveTrack(const MediaStreamVideoTrack* track);
  void ReconfigureTrack(const MediaStreamVideoTrack* track,
                        const VideoTrackAdapterSettings& settings);

  // Delivers |frame| to all tracks that have registered a callback.
  // Must be called on the video task runner.
  void DeliverFrameOnVideoTaskRunner(
      scoped_refptr<media::VideoFrame> video_frame,
      base::TimeTicks estimated_capture_time);

  // Delivers |encoded_frame| to all tracks that have registered a callback.
  // Must be called on the video task runner.
  void DeliverEncodedVideoFrameOnVideoTaskRunner(
      scoped_refptr<EncodedVideoFrame> frame,
      base::TimeTicks estimated_capture_time);

  // Called if a frame was dropped prior to delivery, i.e.
  // DeliverFrameOnVideoTaskRunner() will not be called for this frame.
  void OnFrameDroppedOnVideoTaskRunner(
      media::VideoCaptureFrameDropReason reason);

  // Called when it is guaranteed that all subsequent frames delivered
  // over DeliverFrameOnVideoTaskRunner() will have a sub-capture-target version
  // that is equal-to-or-greater-than the given sub-capture-target version.
  void NewSubCaptureTargetVersionOnVideoTaskRunner(
      uint32_t sub_capture_target_version);

  base::SequencedTaskRunner* video_task_runner() const {
    DCHECK_CALLED_ON_VALID_SEQUENCE(sequence_checker_);
    return video_task_runner_.get();
  }

  // Start monitor that frames are delivered to this object. I.E, that
  // |DeliverFrameOnVideoTaskRunner| is called with a frame rate of
  // |source_frame_rate|. |on_muted_callback| is triggered on the main render
  // thread.
  void StartFrameMonitoring(double source_frame_rate,
                            const OnMutedCallback& on_muted_callback);
  void StopFrameMonitoring();

  void SetSourceFrameSize(const gfx::Size& source_frame_size);

  // Exported for testing.
  //
  // Calculates the desired size of a VideoTrack instance, and returns true if
  // |desired_size| is updated successfully, false otherwise.
  // |desired_size| is not updated if |settings| has rescaling disabled and
  // |input_size| is invalid.
  static bool CalculateDesiredSize(bool is_rotated,
                                   const gfx::Size& input_size,
                                   const VideoTrackAdapterSettings& settings,
                                   gfx::Size* desired_size);

  std::optional<gfx::Size> source_frame_size() const {
    return source_frame_size_;
  }

 private:
  virtual ~VideoTrackAdapter();
  friend class WTF::ThreadSafeRefCounted<VideoTrackAdapter>;

  // These aliases mimic the definition of VideoCaptureDeliverFrameCB,
  // VideoTrackSettingsCallback and VideoTrackFormatCallback respectively.
  using VideoCaptureDeliverFrameInternalCallback =
      WTF::CrossThreadFunction<void(
          scoped_refptr<media::VideoFrame> video_frame,
          base::TimeTicks estimated_capture_time)>;
  using VideoCaptureNotifyFrameDroppedInternalCallback =
      WTF::CrossThreadFunction<void(media::VideoCaptureFrameDropReason)>;
  using DeliverEncodedVideoFrameInternalCallback =
      WTF::CrossThreadFunction<void(
          scoped_refptr<EncodedVideoFrame> video_frame,
          base::TimeTicks estimated_capture_time)>;
  using VideoCaptureSubCaptureTargetVersionInternalCallback =
      WTF::CrossThreadFunction<void(uint32_t)>;
  using VideoTrackSettingsInternalCallback = WTF::CrossThreadFunction<void(
      gfx::Size frame_size,
      double frame_rate,
      std::optional<gfx::Size> metadata_source_size,
      std::optional<float> device_scale_factor)>;
  using VideoTrackFormatInternalCallback =
      WTF::CrossThreadFunction<void(const media::VideoCaptureFormat&)>;
  void AddTrackOnVideoTaskRunner(
      const MediaStreamVideoTrack* track,
      VideoCaptureDeliverFrameInternalCallback frame_callback,
      VideoCaptureNotifyFrameDroppedInternalCallback
          notify_frame_dropped_callback,
      DeliverEncodedVideoFrameInternalCallback encoded_frame_callback,
      VideoCaptureSubCaptureTargetVersionInternalCallback
          sub_capture_target_version_callback,
      VideoTrackSettingsInternalCallback settings_callback,
      VideoTrackFormatInternalCallback track_callback,
      const VideoTrackAdapterSettings& settings);

  void RemoveTrackOnVideoTaskRunner(const MediaStreamVideoTrack* track);
  void ReconfigureTrackOnVideoTaskRunner(
      const MediaStreamVideoTrack* track,
      const VideoTrackAdapterSettings& settings);

  using OnMutedInternalCallback =
      WTF::CrossThreadFunction<void(bool mute_state)>;
  void StartFrameMonitoringOnVideoTaskRunner(
      OnMutedInternalCallback on_muted_state_callback,
      double source_frame_rate);
  void StopFrameMonitoringOnVideoTaskRunner();
  void SetSourceFrameSizeOnVideoTaskRunner(const gfx::Size& frame_size);

  // Compare |old_frame_counter_snapshot_| with the current |frame_counter_|,
  // and inform of the situation (muted, not muted) via |on_muted_callback_|.
  void CheckFramesReceivedOnVideoTaskRunner();

  // |thread_checker_| is bound to the main render thread.
  SEQUENCE_CHECKER(sequence_checker_);

  const scoped_refptr<base::SequencedTaskRunner> video_task_runner_;

  base::WeakPtr<MediaStreamVideoSource> media_stream_video_source_;

  // |renderer_task_runner_| is used to ensure that
  // VideoCaptureDeliverFrameCB is released on the main render thread.
  const scoped_refptr<base::SingleThreadTaskRunner> renderer_task_runner_;

  // VideoFrameResolutionAdapter is an inner class that lives on the video task
  // runner. It does the resolution adaptation and delivers frames to all
  // registered tracks.
  class VideoFrameResolutionAdapter;
  using FrameAdapters = WTF::Vector<scoped_refptr<VideoFrameResolutionAdapter>>;
  FrameAdapters adapters_;

  // Is non-null while frame monitoring. It is only accessed on the video task
  // runner.
  std::unique_ptr<LowPrecisionTimer> monitoring_frame_rate_timer_;
  OnMutedInternalCallback on_muted_callback_;

  // Keeps track of it frames have been received. It is only accessed on the
  // video task runner.
  bool muted_state_;

  // Running frame counter, accessed on the video task runner.
  uint64_t frame_counter_;
  uint64_t old_frame_counter_snapshot_;

  // Frame rate configured on the video source, accessed on the video task
  // runner.
  float source_frame_rate_;

  // Resolution configured on the video source, accessed on the video task
  // runner.
  std::optional<gfx::Size> source_frame_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_TRACK_ADAPTER_H_
