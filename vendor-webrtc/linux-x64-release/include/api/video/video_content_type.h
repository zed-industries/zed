/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_CONTENT_TYPE_H_
#define API_VIDEO_VIDEO_CONTENT_TYPE_H_

#include <stdint.h>

namespace webrtc {

// VideoContentType stored as a single byte, which is sent over the network
// in the rtp-hdrext/video-content-type extension.
// Only the lowest bit is used, per the enum.
enum class VideoContentType : uint8_t {
  UNSPECIFIED = 0,
  SCREENSHARE = 1,
};

namespace videocontenttypehelpers {
bool IsScreenshare(const VideoContentType& content_type);

bool IsValidContentType(uint8_t value);

const char* ToString(const VideoContentType& content_type);
}  // namespace videocontenttypehelpers

}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_CONTENT_TYPE_H_
