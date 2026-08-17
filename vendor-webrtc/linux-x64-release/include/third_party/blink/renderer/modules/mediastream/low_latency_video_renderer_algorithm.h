// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOW_LATENCY_VIDEO_RENDERER_ALGORITHM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOW_LATENCY_VIDEO_RENDERER_ALGORITHM_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace media {
class MediaLog;
}

namespace blink {

class MODULES_EXPORT LowLatencyVideoRendererAlgorithm {
 public:
  explicit LowLatencyVideoRendererAlgorithm(media::MediaLog* media_log);
  LowLatencyVideoRendererAlgorithm(const LowLatencyVideoRendererAlgorithm&) =
      delete;
  LowLatencyVideoRendererAlgorithm& operator=(
      const LowLatencyVideoRendererAlgorithm&) = delete;
  ~LowLatencyVideoRendererAlgorithm();

  // Chooses the best frame for the interval [deadline_min, deadline_max] based
  // on available frames in the queue.
  //
  // If provided, |frames_dropped| will be set to the number of frames which
  // were removed from |frame_queue_|, during this call, which were never
  // returned during a previous Render() call and are no longer suitable for
  // rendering.
  scoped_refptr<media::VideoFrame> Render(base::TimeTicks deadline_min,
                                          base::TimeTicks deadline_max,
                                          size_t* frames_dropped);

  // Adds a frame to |frame_queue_| for consideration by Render(). Frames are
  // rendered in the order they are enqueued. If too many frames are in the
  // queue, the algorithm will enter a drain mode where every second frame will
  // be dropped.
  void EnqueueFrame(scoped_refptr<media::VideoFrame> frame);

  // Removes all frames from |frame_queue_|.
  void Reset();

  // Returns number of frames in the queue. If a frame is currently being
  // rendered it will be included in the count.
  size_t frames_queued() const {
    return frame_queue_.size() + (current_frame_ ? 1 : 0);
  }

  // Returns the average of the duration of a frame. Currently hard coded at
  // 60fps.
  base::TimeDelta average_frame_duration() const {
    // TODO(crbug.com/1138888): Estimate frame duration from content.
    return base::Milliseconds(1000.0 / 60.0);
  }

 private:
  size_t DetermineModeAndNumberOfFramesToRender(
      double fractional_frames_to_render);

  bool ReduceSteadyStateQueue(size_t number_of_frames_to_render);

  void SelectNextAvailableFrameAndUpdateLastDeadline(
      base::TimeTicks deadline_min);

  scoped_refptr<media::VideoFrame> current_frame_;

  // Queue of incoming frames waiting for rendering.
  using VideoFrameQueue = WTF::Deque<scoped_refptr<media::VideoFrame>>;
  VideoFrameQueue frame_queue_;

  // Render deadline min for when the last frame was rendered.
  std::optional<base::TimeTicks> last_render_deadline_min_;

  // Stores the number of fractional frames that were not rendered as of
  // |last_render_deadline_min_|. This is needed in case the display refresh
  // rate is not a multiple of the video stream frame rate.
  double unrendered_fractional_frames_;

  enum class Mode {
    // Render frames at their intended rate.
    kNormal = 0,
    // Render frames at the double rate. This mode is used to drop frames in a
    // controlled manner whenever there's too many frames in the queue.
    kDrain = 1,
    kMaxValue = kDrain
  };
  Mode mode_;

  // The number of consecutive render frames with a post-decode queue back-up
  // (defined as greater than one frame).
  uint16_t consecutive_frames_with_back_up_;

  struct Stats {
    int total_frames;
    int dropped_frames;
    int drained_frames;
    int render_frame;
    int no_new_frame_to_render;
    int accumulated_queue_length;
    int accumulated_queue_length_count;
    int max_queue_length;
    int enter_drain_mode;
    int reduce_steady_state;
    int max_size_drop_queue;
  };
  Stats stats_;
  std::optional<base::TimeTicks> last_deadline_min_stats_recorded_;
  void RecordAndResetStats();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOW_LATENCY_VIDEO_RENDERER_ALGORITHM_H_
