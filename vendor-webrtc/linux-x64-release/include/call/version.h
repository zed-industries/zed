/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_VERSION_H_
#define CALL_VERSION_H_

// LoadWebRTCVersionInRegistry is a helper function that loads the pointer to
// the WebRTC version string into a register. While this function doesn't do
// anything useful, it is needed in order to avoid that compiler optimizations
// remove the WebRTC version string from the final binary.

namespace webrtc {

void LoadWebRTCVersionInRegister();

}  // namespace webrtc

#endif  // CALL_VERSION_H_
