/*
 *  Copyright (c) 2010 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_VIDEO_ADAPTER_H_
#define MEDIA_BASE_VIDEO_ADAPTER_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <utility>

#include "api/video/resolution.h"
#include "api/video/video_source_interface.h"
#include "common_video/framerate_controller.h"
#include "media/base/video_common.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/rtc_export.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

// VideoAdapter adapts an input video frame to an output frame based on the
// specified input and output formats. The adaptation includes dropping frames
// to reduce frame rate and scaling frames.
// VideoAdapter is thread safe.
class RTC_EXPORT VideoAdapter {
 public:
  VideoAdapter();
  // The source requests output frames whose width and height are divisible
  // by `source_resolution_alignment`.
  explicit VideoAdapter(int source_resolution_alignment);
  virtual ~VideoAdapter();

  VideoAdapter(const VideoAdapter&) = delete;
  VideoAdapter& operator=(const VideoAdapter&) = delete;

  // Return the adapted resolution and cropping parameters given the
  // input resolution. The input frame should first be cropped, then
  // scaled to the final output resolution. Returns true if the frame
  // should be adapted, and false if it should be dropped.
  bool AdaptFrameResolution(int in_width,
                            int in_height,
                            int64_t in_timestamp_ns,
                            int* cropped_width,
                            int* cropped_height,
                            int* out_width,
                            int* out_height) RTC_LOCKS_EXCLUDED(mutex_);

  // DEPRECATED. Please use OnOutputFormatRequest below.
  // TODO(asapersson): Remove this once it is no longer used.
  // Requests the output frame size and frame interval from
  // `AdaptFrameResolution` to not be larger than `format`. Also, the input
  // frame size will be cropped to match the requested aspect ratio. The
  // requested aspect ratio is orientation agnostic and will be adjusted to
  // maintain the input orientation, so it doesn't matter if e.g. 1280x720 or
  // 720x1280 is requested.
  // Note: Should be called from the source only.
  void OnOutputFormatRequest(const std::optional<VideoFormat>& format)
      RTC_LOCKS_EXCLUDED(mutex_);

  // Requests output frame size and frame interval from `AdaptFrameResolution`.
  // `target_aspect_ratio`: The input frame size will be cropped to match the
  // requested aspect ratio. The aspect ratio is orientation agnostic and will
  // be adjusted to maintain the input orientation (i.e. it doesn't matter if
  // e.g. <1280,720> or <720,1280> is requested).
  // `max_pixel_count`: The maximum output frame size.
  // `max_fps`: The maximum output framerate.
  // Note: Should be called from the source only.
  void OnOutputFormatRequest(
      const std::optional<std::pair<int, int>>& target_aspect_ratio,
      const std::optional<int>& max_pixel_count,
      const std::optional<int>& max_fps) RTC_LOCKS_EXCLUDED(mutex_);

  // Same as above, but allows setting two different target aspect ratios
  // depending on incoming frame orientation. This gives more fine-grained
  // control and can e.g. be used to force landscape video to be cropped to
  // portrait video.
  void OnOutputFormatRequest(
      const std::optional<std::pair<int, int>>& target_landscape_aspect_ratio,
      const std::optional<int>& max_landscape_pixel_count,
      const std::optional<std::pair<int, int>>& target_portrait_aspect_ratio,
      const std::optional<int>& max_portrait_pixel_count,
      const std::optional<int>& max_fps) RTC_LOCKS_EXCLUDED(mutex_);

  // Requests the output frame size from `AdaptFrameResolution` to have as close
  // as possible to `sink_wants.target_pixel_count` pixels (if set)
  // but no more than `sink_wants.max_pixel_count`.
  // `sink_wants.max_framerate_fps` is essentially analogous to
  // `sink_wants.max_pixel_count`, but for framerate rather than resolution.
  // Set `sink_wants.max_pixel_count` and/or `sink_wants.max_framerate_fps` to
  // std::numeric_limit<int>::max() if no upper limit is desired.
  // The sink resolution alignment requirement is given by
  // `sink_wants.resolution_alignment`.
  // Note: Should be called from the sink only.
  void OnSinkWants(const VideoSinkWants& sink_wants) RTC_LOCKS_EXCLUDED(mutex_);

  // Returns maximum image area, which shouldn't impose any adaptations.
  // Can return `numeric_limits<int>::max()` if no limit is set.
  int GetTargetPixels() const;

  // Returns current frame-rate limit.
  // Can return `numeric_limits<float>::infinity()` if no limit is set.
  float GetMaxFramerate() const;

 private:
  // Determine if frame should be dropped based on input fps and requested fps.
  bool DropFrame(int64_t in_timestamp_ns) RTC_EXCLUSIVE_LOCKS_REQUIRED(mutex_);

  int frames_in_ RTC_GUARDED_BY(mutex_);      // Number of input frames.
  int frames_out_ RTC_GUARDED_BY(mutex_);     // Number of output frames.
  int frames_scaled_ RTC_GUARDED_BY(mutex_);  // Number of frames scaled.
  int adaption_changes_
      RTC_GUARDED_BY(mutex_);  // Number of changes in scale factor.
  int previous_width_ RTC_GUARDED_BY(mutex_);  // Previous adapter output width.
  int previous_height_
      RTC_GUARDED_BY(mutex_);  // Previous adapter output height.

  // The fixed source resolution alignment requirement.
  const int source_resolution_alignment_;
  // The currently applied resolution alignment, as given by the requirements:
  //  - the fixed `source_resolution_alignment_`; and
  //  - the latest `sink_wants.resolution_alignment`.
  int resolution_alignment_ RTC_GUARDED_BY(mutex_);

  // Max number of pixels/fps requested via calls to OnOutputFormatRequest,
  // OnResolutionFramerateRequest respectively.
  // The adapted output format is the minimum of these.
  struct OutputFormatRequest {
    std::optional<std::pair<int, int>> target_landscape_aspect_ratio;
    std::optional<int> max_landscape_pixel_count;
    std::optional<std::pair<int, int>> target_portrait_aspect_ratio;
    std::optional<int> max_portrait_pixel_count;
    std::optional<int> max_fps;

    // For logging.
    std::string ToString() const;
  };

  OutputFormatRequest output_format_request_ RTC_GUARDED_BY(mutex_);
  int resolution_request_target_pixel_count_ RTC_GUARDED_BY(mutex_);
  int resolution_request_max_pixel_count_ RTC_GUARDED_BY(mutex_);
  int max_framerate_request_ RTC_GUARDED_BY(mutex_);
  std::optional<Resolution> scale_resolution_down_to_ RTC_GUARDED_BY(mutex_);

  // Stashed OutputFormatRequest that is used to save value of
  // OnOutputFormatRequest in case all active encoders are using
  // scale_resolution_down_to. I.e when all active encoders are using
  // scale_resolution_down_to, the call to OnOutputFormatRequest is ignored
  // and the value from scale_resolution_down_to is used instead (to scale/crop
  // frame). This allows for an application to only use
  // RtpEncodingParameters::request_resolution and get the same behavior as if
  // it had used VideoAdapter::OnOutputFormatRequest.
  std::optional<OutputFormatRequest> stashed_output_format_request_
      RTC_GUARDED_BY(mutex_);

  FramerateController framerate_controller_ RTC_GUARDED_BY(mutex_);

  // The critical section to protect the above variables.
  mutable Mutex mutex_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::VideoAdapter;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_VIDEO_ADAPTER_H_
