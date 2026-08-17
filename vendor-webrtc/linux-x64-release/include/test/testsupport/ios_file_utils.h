/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_IOS_FILE_UTILS_H_
#define TEST_TESTSUPPORT_IOS_FILE_UTILS_H_

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {

std::string IOSOutputPath();
std::string IOSRootPath();
std::string IOSResourcePath(absl::string_view name,
                            absl::string_view extension);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_IOS_FILE_UTILS_H_
