/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_TESTSUPPORT_FILE_UTILS_OVERRIDE_H_
#define TEST_TESTSUPPORT_FILE_UTILS_OVERRIDE_H_

#include <string>

#include "absl/strings/string_view.h"

namespace webrtc {
namespace test {
namespace internal {

// Returns the absolute path to the output directory where log files and other
// test artifacts should be put. The output directory is generally a directory
// named "out" at the project root. This root is assumed to be two levels above
// where the test binary is located; this is because tests execute in a dir
// out/Whatever relative to the project root. This convention is also followed
// in Chromium.
//
// The exception is Android where we use /sdcard/ instead.
//
// If symbolic links occur in the path they will be resolved and the actual
// directory will be returned.
//
// Returns the path WITH a trailing path delimiter. If the project root is not
// found, the current working directory ("./") is returned as a fallback.
std::string OutputPath();

// Gets the current working directory for the executing program.
// Returns "./" if for some reason it is not possible to find the working
// directory.
std::string WorkingDir();

// Returns a full path to a resource file in the resources_dir dir.
//
// Arguments:
//    name - Name of the resource file. If a plain filename (no directory path)
//           is supplied, the file is assumed to be located in resources/
//           If a directory path is prepended to the filename, a subdirectory
//           hierarchy reflecting that path is assumed to be present.
//    extension - File extension, without the dot, i.e. "bmp" or "yuv".
std::string ResourcePath(absl::string_view name, absl::string_view extension);

}  // namespace internal
}  // namespace test
}  // namespace webrtc

#endif  // TEST_TESTSUPPORT_FILE_UTILS_OVERRIDE_H_
