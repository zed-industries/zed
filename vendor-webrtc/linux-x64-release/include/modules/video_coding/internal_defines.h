/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_INTERNAL_DEFINES_H_
#define MODULES_VIDEO_CODING_INTERNAL_DEFINES_H_

namespace webrtc {

#define VCM_MAX(a, b) (((a) > (b)) ? (a) : (b))
#define VCM_MIN(a, b) (((a) < (b)) ? (a) : (b))

#define VCM_FLUSH_INDICATOR 4

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_INTERNAL_DEFINES_H_
