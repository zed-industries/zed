// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_VIDEO_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_VIDEO_TIMING_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/media_timing.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// This class specializes MediaTiming for video-specific timing data. Videos
// are always considered animated, and have their first frame time set by the
// HTMLVideoElement when the first frame is presented.
class VideoTiming final : public GarbageCollected<VideoTiming>,
                          public MediaTiming {
 public:
  VideoTiming() = default;

  void Trace(Visitor* visitor) const override { MediaTiming::Trace(visitor); }

  void SetUrl(const KURL& url) { url_ = url; }
  const KURL& Url() const override { return url_; }

  bool IsDataUrl() const override { return Url().ProtocolIsData(); }
  AtomicString MediaType() const override { return AtomicString("video"); }

  void SetIsSufficientContentLoadedForPaint() override { is_loaded_ = true; }
  bool IsSufficientContentLoadedForPaint() const override { return is_loaded_; }

  bool IsAnimatedImage() const override { return true; }
  bool IsPaintedFirstFrame() const override {
    return !first_frame_time_.is_null();
  }
  void SetFirstVideoFrameTime(base::TimeTicks time) override {
    first_frame_time_ = time;
  }
  base::TimeTicks GetFirstVideoFrameTime() const override {
    return first_frame_time_;
  }

  void SetTimingAllowPassed(bool timing_allow_passed) {
    timing_allow_passed_ = timing_allow_passed;
  }
  bool TimingAllowPassed() const override { return timing_allow_passed_; }

  uint64_t ContentSizeForEntropy() const override {
    // We don't do anything clever here to try to isolate the encoded size of
    // just the first frame; if we're calling this, then at least enough data
    // has been received to display that frame. This will result in a more
    // lenient entropy estimate for LCP purposes.
    return content_size_;
  }

  void SetContentSizeForEntropy(size_t length) { content_size_ = length; }

  std::optional<WebURLRequest::Priority> RequestPriority() const override {
    // No priority data are reported for LCP videos as initially we focus on LCP
    // images (crbug.com/1378698).
    // TODO(crbug.com/1379728): Revisit priority reporting also for videos.
    return std::nullopt;
  }

  bool IsBroken() const override { return false; }
  // Video timing does not have information about load start/end time. The
  // functions return 0 Timeticks as placeholders which would not be reported to
  // UKM.
  // TODO(crbug.com/1414077): We should determine what the load start and end
  // time should be for videos.
  base::TimeTicks LoadStart() const override { return base::TimeTicks(); }

  base::TimeTicks LoadEnd() const override { return base::TimeTicks(); }

  base::TimeTicks DiscoveryTime() const override { return base::TimeTicks(); }

 private:
  KURL url_;
  bool is_loaded_ = false;
  base::TimeTicks first_frame_time_;
  bool timing_allow_passed_ = false;
  size_t content_size_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RESOURCE_VIDEO_TIMING_H_
