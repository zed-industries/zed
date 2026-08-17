/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_PROTOBUF_UTILS_H_
#define MODULES_AUDIO_PROCESSING_TEST_PROTOBUF_UTILS_H_

#include <memory>
#include <sstream>  // no-presubmit-check TODO(webrtc:8982)

#include "rtc_base/protobuf_utils.h"

// Generated at build-time by the protobuf compiler.
#include "modules/audio_processing/debug.pb.h"

namespace webrtc {

// Allocates new memory in the unique_ptr to fit the raw message and returns the
// number of bytes read.
size_t ReadMessageBytesFromFile(FILE* file, std::unique_ptr<uint8_t[]>* bytes);

// Returns true on success, false on error or end-of-file.
bool ReadMessageFromFile(FILE* file, MessageLite* msg);

// Returns true on success, false on error or end of string stream.
bool ReadMessageFromString(
    std::stringstream* input,  // no-presubmit-check TODO(webrtc:8982)
    MessageLite* msg);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_PROTOBUF_UTILS_H_
