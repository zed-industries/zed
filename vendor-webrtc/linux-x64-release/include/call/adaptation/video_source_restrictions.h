/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_ADAPTATION_VIDEO_SOURCE_RESTRICTIONS_H_
#define CALL_ADAPTATION_VIDEO_SOURCE_RESTRICTIONS_H_

#include <cstddef>
#include <optional>
#include <string>

namespace webrtc {

// Describes optional restrictions to the resolution and frame rate of a video
// source.
class VideoSourceRestrictions {
 public:
  // Constructs without any restrictions.
  VideoSourceRestrictions();
  // All values must be positive or nullopt.
  // TODO(hbos): Support expressing "disable this stream"?
  VideoSourceRestrictions(std::optional<size_t> max_pixels_per_frame,
                          std::optional<size_t> target_pixels_per_frame,
                          std::optional<double> max_frame_rate);

  bool operator==(const VideoSourceRestrictions& rhs) const {
    return max_pixels_per_frame_ == rhs.max_pixels_per_frame_ &&
           target_pixels_per_frame_ == rhs.target_pixels_per_frame_ &&
           max_frame_rate_ == rhs.max_frame_rate_;
  }
  bool operator!=(const VideoSourceRestrictions& rhs) const {
    return !(*this == rhs);
  }

  std::string ToString() const;

  // The source must produce a resolution less than or equal to
  // max_pixels_per_frame().
  const std::optional<size_t>& max_pixels_per_frame() const;
  // The source should produce a resolution as close to the
  // target_pixels_per_frame() as possible, provided this does not exceed
  // max_pixels_per_frame().
  // The actual pixel count selected depends on the capabilities of the source.
  // TODO(hbos): Clarify how "target" is used. One possible implementation: open
  // the camera in the smallest resolution that is greater than or equal to the
  // target and scale it down to the target if it is greater. Is this an
  // accurate description of what this does today, or do we do something else?
  const std::optional<size_t>& target_pixels_per_frame() const;
  const std::optional<double>& max_frame_rate() const;

  void set_max_pixels_per_frame(std::optional<size_t> max_pixels_per_frame);
  void set_target_pixels_per_frame(
      std::optional<size_t> target_pixels_per_frame);
  void set_max_frame_rate(std::optional<double> max_frame_rate);

  // Update `this` with min(`this`, `other`).
  void UpdateMin(const VideoSourceRestrictions& other);

 private:
  // These map to VideoSinkWants's `max_pixel_count` and
  // `target_pixel_count`.
  std::optional<size_t> max_pixels_per_frame_;
  std::optional<size_t> target_pixels_per_frame_;
  std::optional<double> max_frame_rate_;
};

bool DidRestrictionsIncrease(VideoSourceRestrictions before,
                             VideoSourceRestrictions after);
bool DidRestrictionsDecrease(VideoSourceRestrictions before,
                             VideoSourceRestrictions after);
bool DidIncreaseResolution(VideoSourceRestrictions restrictions_before,
                           VideoSourceRestrictions restrictions_after);
bool DidDecreaseResolution(VideoSourceRestrictions restrictions_before,
                           VideoSourceRestrictions restrictions_after);
bool DidIncreaseFrameRate(VideoSourceRestrictions restrictions_before,
                          VideoSourceRestrictions restrictions_after);
bool DidDecreaseFrameRate(VideoSourceRestrictions restrictions_before,
                          VideoSourceRestrictions restrictions_after);

}  // namespace webrtc

#endif  // CALL_ADAPTATION_VIDEO_SOURCE_RESTRICTIONS_H_
