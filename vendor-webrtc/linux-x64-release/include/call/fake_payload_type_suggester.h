/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef CALL_FAKE_PAYLOAD_TYPE_SUGGESTER_H_
#define CALL_FAKE_PAYLOAD_TYPE_SUGGESTER_H_

#include <string>

#include "api/rtc_error.h"
#include "call/payload_type.h"
#include "call/payload_type_picker.h"
#include "media/base/codec.h"

namespace webrtc {
// Fake payload type suggester, for use in tests.
// It uses a real PayloadTypePicker in order to do consistent PT
// assignment.
class FakePayloadTypeSuggester : public webrtc::PayloadTypeSuggester {
 public:
  webrtc::RTCErrorOr<webrtc::PayloadType> SuggestPayloadType(
      const std::string& mid,
      Codec codec) override {
    // Ignores mid argument.
    return pt_picker_.SuggestMapping(codec, nullptr);
  }
  webrtc::RTCError AddLocalMapping(const std::string& mid,
                                   webrtc::PayloadType payload_type,
                                   const Codec& codec) override {
    return webrtc::RTCError::OK();
  }

 private:
  webrtc::PayloadTypePicker pt_picker_;
};

}  // namespace webrtc

#endif  // CALL_FAKE_PAYLOAD_TYPE_SUGGESTER_H_
