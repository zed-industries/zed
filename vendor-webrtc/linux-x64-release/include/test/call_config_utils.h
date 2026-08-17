/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_CALL_CONFIG_UTILS_H_
#define TEST_CALL_CONFIG_UTILS_H_

#include "call/video_receive_stream.h"
#include "rtc_base/strings/json.h"

namespace webrtc {
namespace test {

// Deserializes a JSON representation of the VideoReceiveStreamInterface::Config
// back into a valid object. This will not initialize the decoders or the
// renderer.
VideoReceiveStreamInterface::Config ParseVideoReceiveStreamJsonConfig(
    webrtc::Transport* transport,
    const Json::Value& json);

// Serialize a VideoReceiveStreamInterface::Config into a Json object.
Json::Value GenerateVideoReceiveStreamJsonConfig(
    const VideoReceiveStreamInterface::Config& config);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_CALL_CONFIG_UTILS_H_
