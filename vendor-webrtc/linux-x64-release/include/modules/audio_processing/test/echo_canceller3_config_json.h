/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER3_CONFIG_JSON_H_
#define MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER3_CONFIG_JSON_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/audio/echo_canceller3_config.h"

namespace webrtc {
// Parses a JSON-encoded string into an Aec3 config. Fields corresponds to
// substruct names, with the addition that there must be a top-level node
// "aec3". Produces default config values for anything that cannot be parsed
// from the string. If any error was found in the parsing, parsing_successful is
// set to false.
void Aec3ConfigFromJsonString(absl::string_view json_string,
                              EchoCanceller3Config* config,
                              bool* parsing_successful);

// Encodes an Aec3 config in JSON format. Fields corresponds to substruct names,
// with the addition that the top-level node is named "aec3".
std::string Aec3ConfigToJsonString(const EchoCanceller3Config& config);

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER3_CONFIG_JSON_H_
