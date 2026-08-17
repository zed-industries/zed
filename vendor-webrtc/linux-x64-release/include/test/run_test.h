/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_RUN_TEST_H_
#define TEST_RUN_TEST_H_

namespace webrtc {
namespace test {

// Running a test function on a separate thread, if required by the OS.
void RunTest(void (*test)());

}  // namespace test
}  // namespace webrtc

#endif  // TEST_RUN_TEST_H_
