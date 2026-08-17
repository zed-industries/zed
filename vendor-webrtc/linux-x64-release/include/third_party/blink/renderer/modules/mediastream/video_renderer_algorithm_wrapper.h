// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_RENDERER_ALGORITHM_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_RENDERER_ALGORITHM_WRAPPER_H_

#include <stddef.h>
#include <stdint.h>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "media/base/media_util.h"
#include "media/base/time_source.h"
#include "media/base/video_frame.h"
#include "media/filters/video_renderer_algorithm.h"
#include "third_party/blink/renderer/modules/mediastream/low_latency_video_renderer_algorithm.h"

namespace blink {

class VideoRendererAlgorithmWrapper {
 public:
  enum RendererAlgorithm { kDefault, kLowLatency };

  VideoRendererAlgorithmWrapper(
      const media::TimeSource::WallClockTimeCB& wall_clock_time_cb,
      media::MediaLog* media_log);

  scoped_refptr<media::VideoFrame> Render(base::TimeTicks deadline_min,
                                          base::TimeTicks deadline_max,
                                          size_t* frames_dropped);
  void EnqueueFrame(scoped_refptr<media::VideoFrame> frame);
  void Reset(media::VideoRendererAlgorithm::ResetFlag reset_flag =
                 media::VideoRendererAlgorithm::ResetFlag::kEverything);

  size_t frames_queued() const {
    return renderer_algorithm_ == RendererAlgorithm::kDefault
               ? default_rendering_frame_buffer_->frames_queued()
               : low_latency_rendering_frame_buffer_->frames_queued();
  }

  base::TimeDelta average_frame_duration() const {
    return renderer_algorithm_ == RendererAlgorithm::kDefault
               ? default_rendering_frame_buffer_->average_frame_duration()
               : low_latency_rendering_frame_buffer_->average_frame_duration();
  }

  bool NeedsReferenceTime() const;

  RendererAlgorithm renderer_algorithm() const { return renderer_algorithm_; }

 private:
  const media::TimeSource::WallClockTimeCB wall_clock_time_cb_;
  raw_ptr<media::MediaLog> media_log_;
  RendererAlgorithm renderer_algorithm_;
  std::unique_ptr<media::VideoRendererAlgorithm>
      default_rendering_frame_buffer_;
  std::unique_ptr<LowLatencyVideoRendererAlgorithm>
      low_latency_rendering_frame_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_VIDEO_RENDERER_ALGORITHM_WRAPPER_H_
