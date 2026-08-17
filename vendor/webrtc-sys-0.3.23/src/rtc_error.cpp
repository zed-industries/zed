/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "livekit/rtc_error.h"

#include <iomanip>
#include <sstream>
#include <string>

namespace livekit_ffi {

RtcError to_error(const webrtc::RTCError& error) {
  RtcError lk_error;
  lk_error.error_detail = static_cast<RtcErrorDetailType>(error.error_detail());
  lk_error.error_type = static_cast<RtcErrorType>(error.type());
  lk_error.has_sctp_cause_code = error.sctp_cause_code().has_value();
  lk_error.sctp_cause_code = error.sctp_cause_code().value_or(0);
  lk_error.message = error.message();
  return lk_error;
}

std::string serialize_error(const RtcError& error) {
  std::stringstream ss;
  ss << std::hex << std::setfill('0');
  ss << std::setw(8) << (uint32_t)error.error_type;
  ss << std::setw(8) << (uint32_t)error.error_detail;
  ss << std::setw(2) << (uint16_t)error.has_sctp_cause_code;
  ss << std::setw(4) << (uint16_t)error.sctp_cause_code;
  ss << std::dec << std::setw(1) << std::string(error.message);
  return ss.str();
}

#ifdef LIVEKIT_TEST
rust::String serialize_deserialize() {
  RtcError lk_error;
  lk_error.error_type = RtcErrorType::InternalError;
  lk_error.error_detail = RtcErrorDetailType::DataChannelFailure;
  lk_error.has_sctp_cause_code = true;
  lk_error.sctp_cause_code = 24;
  lk_error.message = "this is not a test, I repeat, this is not a test";
  return serialize_error(lk_error);
}

void throw_error() {
  RtcError lk_error;
  lk_error.error_type = RtcErrorType::InvalidModification;
  lk_error.error_detail = RtcErrorDetailType::None;
  lk_error.has_sctp_cause_code = false;
  lk_error.sctp_cause_code = 0;
  lk_error.message = "exception is thrown!";
  throw std::runtime_error(serialize_error(lk_error));
}
#endif

}  // namespace livekit_ffi
