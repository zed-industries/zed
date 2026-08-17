/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_SOURCE_INTERFACE_H_
#define API_VIDEO_VIDEO_SOURCE_INTERFACE_H_

#include <limits>
#include <optional>
#include <vector>

#include "api/video/video_sink_interface.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// VideoSinkWants is used for notifying the source of properties a video frame
// should have when it is delivered to a certain sink.
struct RTC_EXPORT VideoSinkWants {
  struct FrameSize {
    FrameSize(int width, int height) : width(width), height(height) {}
    FrameSize(const FrameSize&) = default;
    ~FrameSize() = default;

    int width;
    int height;
  };

  VideoSinkWants();
  VideoSinkWants(const VideoSinkWants&);
  ~VideoSinkWants();

  // Tells the source whether the sink wants frames with rotation applied.
  // By default, any rotation must be applied by the sink.
  bool rotation_applied = false;

  // Tells the source that the sink only wants black frames.
  bool black_frames = false;

  // Tells the source the maximum number of pixels the sink wants.
  int max_pixel_count = std::numeric_limits<int>::max();
  // Tells the source the desired number of pixels the sinks wants. This will
  // typically be used when stepping the resolution up again when conditions
  // have improved after an earlier downgrade. The source should select the
  // closest resolution to this pixel count, but if max_pixel_count is set, it
  // still sets the absolute upper bound.
  std::optional<int> target_pixel_count;
  // Tells the source the maximum framerate the sink wants.
  int max_framerate_fps = std::numeric_limits<int>::max();

  // Tells the source that the sink wants width and height of the video frames
  // to be divisible by `resolution_alignment`.
  // For example: With I420, this value would be a multiple of 2.
  // Note that this field is unrelated to any horizontal or vertical stride
  // requirements the encoder has on the incoming video frame buffers.
  int resolution_alignment = 1;

  // The resolutions that sink is configured to consume. If the sink is an
  // encoder this is what the encoder is configured to encode. In singlecast we
  // only encode one resolution, but in simulcast and SVC this can mean multiple
  // resolutions per frame.
  //
  // The sink is always configured to consume a subset of the
  // webrtc::VideoFrame's resolution. In the case of encoding, we usually encode
  // at webrtc::VideoFrame's resolution but this may not always be the case due
  // to scaleResolutionDownBy or turning off simulcast or SVC layers.
  //
  // For example, we may capture at 720p and due to adaptation (e.g. applying
  // `max_pixel_count` constraints) create webrtc::VideoFrames of size 480p, but
  // if we do scaleResolutionDownBy:2 then the only resolution we end up
  // encoding is 240p. In this case we still need to provide webrtc::VideoFrames
  // of size 480p but we can optimize internal buffers for 240p, avoiding
  // downsampling to 480p if possible.
  //
  // Note that the `resolutions` can change while frames are in flight and
  // should only be used as a hint when constructing the webrtc::VideoFrame.
  std::vector<FrameSize> resolutions;

  // This is the resolution requested by the user using RtpEncodingParameters,
  // which is the maximum `scale_resolution_down_by` value of any encoding.
  std::optional<FrameSize> requested_resolution;

  // `is_active` : Is this VideoSinkWants from an encoder that is encoding any
  // layer. IF YES, it will affect how the VideoAdapter will choose to
  // prioritize the OnOutputFormatRequest vs. requested_resolution. IF NO,
  // VideoAdapter consider this VideoSinkWants as a passive listener (e.g a
  // VideoRenderer or a VideoEncoder that is not currently actively encoding).
  bool is_active = false;

  // This sub-struct contains information computed by VideoBroadcaster
  // that aggregates several VideoSinkWants (and sends them to
  // AdaptedVideoTrackSource).
  struct Aggregates {
    // `any_active_without_requested_resolution` is set by VideoBroadcaster
    // when aggregating sink wants if there exists any sink (encoder) that is
    // active but has not set the `requested_resolution`, i.e is relying on
    // OnOutputFormatRequest to handle encode resolution.
    bool any_active_without_requested_resolution = false;
  };
  std::optional<Aggregates> aggregates;
};

inline bool operator==(const VideoSinkWants::FrameSize& a,
                       const VideoSinkWants::FrameSize& b) {
  return a.width == b.width && a.height == b.height;
}

inline bool operator!=(const VideoSinkWants::FrameSize& a,
                       const VideoSinkWants::FrameSize& b) {
  return !(a == b);
}

template <typename VideoFrameT>
class VideoSourceInterface {
 public:
  virtual ~VideoSourceInterface() = default;

  virtual void AddOrUpdateSink(VideoSinkInterface<VideoFrameT>* sink,
                               const VideoSinkWants& wants) = 0;
  // RemoveSink must guarantee that at the time the method returns,
  // there is no current and no future calls to VideoSinkInterface::OnFrame.
  virtual void RemoveSink(VideoSinkInterface<VideoFrameT>* sink) = 0;

  // Request underlying source to capture a new frame.
  // TODO(crbug/1255737): make pure virtual once downstream projects adapt.
  virtual void RequestRefreshFrame() {}
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::VideoSinkWants;
using ::webrtc::VideoSourceInterface;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES
#endif  // API_VIDEO_VIDEO_SOURCE_INTERFACE_H_
