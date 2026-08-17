/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METRICS_HELPER_H_
#define MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METRICS_HELPER_H_

#include <stdint.h>

namespace webrtc {

void RecordCapturerImpl(uint32_t capturer_id);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DESKTOP_CAPTURE_METRICS_HELPER_H_
