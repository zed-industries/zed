// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Author: kenton@google.com (Kenton Varda)
// emulates google3/testing/base/public/googletest.h

#ifndef GOOGLE_PROTOBUF_GOOGLETEST_H__
#define GOOGLE_PROTOBUF_GOOGLETEST_H__

#include <gmock/gmock.h>

#include <string>
#include <vector>

namespace google {
namespace protobuf {

// When running unittests, get the directory containing the source code.
std::string TestSourceDir();

// When running unittests, get a directory where temporary files may be
// placed.
std::string TestTempDir();

// Capture all text written to stdout or stderr.
void CaptureTestStdout();
void CaptureTestStderr();

// Stop capturing stdout or stderr and return the text captured.
std::string GetCapturedTestStdout();
std::string GetCapturedTestStderr();

}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_GOOGLETEST_H__
