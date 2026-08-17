/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#include <string>

#ifndef RTC_BASE_PROTOBUF_UTILS_H_
#define RTC_BASE_PROTOBUF_UTILS_H_

#if WEBRTC_ENABLE_PROTOBUF

#include "third_party/protobuf/src/google/protobuf/message_lite.h"  // nogncheck
#include "third_party/protobuf/src/google/protobuf/repeated_field.h"  // nogncheck

namespace webrtc {

using google::protobuf::MessageLite;
using google::protobuf::RepeatedPtrField;

}  // namespace webrtc

#endif  // WEBRTC_ENABLE_PROTOBUF

#endif  // RTC_BASE_PROTOBUF_UTILS_H_
