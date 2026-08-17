/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_INCLUDE_VIDEO_ERROR_CODES_UTILS_H_
#define MODULES_VIDEO_CODING_INCLUDE_VIDEO_ERROR_CODES_UTILS_H_

#include <stdint.h>

namespace webrtc {

const char* WebRtcVideoCodecErrorToString(int32_t error_code);

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_INCLUDE_VIDEO_ERROR_CODES_UTILS_H_
