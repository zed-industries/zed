/*
 *  Copyright 2018 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef EXAMPLES_TURNSERVER_READ_AUTH_FILE_H_
#define EXAMPLES_TURNSERVER_READ_AUTH_FILE_H_

#include <istream>
#include <map>
#include <string>

namespace webrtc_examples {

std::map<std::string, std::string> ReadAuthFile(std::istream* s);

}  // namespace webrtc_examples

#endif  // EXAMPLES_TURNSERVER_READ_AUTH_FILE_H_
