/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_VIDEO_LOOPBACK_H_
#define VIDEO_VIDEO_LOOPBACK_H_

namespace webrtc {
// Expose the main test method.
int RunLoopbackTest(int argc, char* argv[]);
}  // namespace webrtc

#endif  // VIDEO_VIDEO_LOOPBACK_H_
