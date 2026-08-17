/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_VIDEO_CODEC_CONSTANTS_H_
#define API_VIDEO_VIDEO_CODEC_CONSTANTS_H_

namespace webrtc {

enum : int { kMaxEncoderBuffers = 8 };
enum : int { kMaxSimulcastStreams = 3 };
enum : int { kMaxSpatialLayers = 5 };
enum : int { kMaxTemporalStreams = 4 };
enum : int { kMaxPreferredPixelFormats = 5 };

}  // namespace webrtc

#endif  // API_VIDEO_VIDEO_CODEC_CONSTANTS_H_
