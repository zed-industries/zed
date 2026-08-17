/*
 *  Copyright 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contains interfaces for MediaStream, MediaTrack and MediaSource.
// These interfaces are used for implementing MediaStream and MediaTrack as
// defined in http://dev.w3.org/2011/webrtc/editor/webrtc.html#stream-api. These
// interfaces must be used only with PeerConnection.

#ifndef API_VIDEO_TRACK_SOURCE_CONSTRAINTS_H_
#define API_VIDEO_TRACK_SOURCE_CONSTRAINTS_H_

#include <optional>

namespace webrtc {

// This struct definition describes constraints on the video source that may be
// set with VideoTrackSourceInterface::ProcessConstraints.
struct VideoTrackSourceConstraints {
  std::optional<double> min_fps;
  std::optional<double> max_fps;
};

}  // namespace webrtc

#endif  // API_VIDEO_TRACK_SOURCE_CONSTRAINTS_H_
