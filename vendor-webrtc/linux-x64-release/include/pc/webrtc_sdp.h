/*
 *  Copyright 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file contain functions for parsing and serializing SDP messages.
// Related RFC/draft including:
// * RFC 4566 - SDP
// * RFC 5245 - ICE
// * RFC 3388 - Grouping of Media Lines in SDP
// * RFC 4568 - SDP Security Descriptions for Media Streams
// * draft-lennox-mmusic-sdp-source-selection-02 -
//   Mechanisms for Media Source Selection in SDP

#ifndef PC_WEBRTC_SDP_H_
#define PC_WEBRTC_SDP_H_

#include <string>

#include "absl/strings/string_view.h"
#include "api/candidate.h"
#include "api/jsep.h"
#include "api/jsep_ice_candidate.h"
#include "api/jsep_session_description.h"
#include "media/base/codec.h"
#include "rtc_base/strings/string_builder.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {
class IceCandidateInterface;
class JsepIceCandidate;
class JsepSessionDescription;
struct SdpParseError;

// Serializes the passed in JsepSessionDescription.
// Serialize SessionDescription including candidates if
// JsepSessionDescription has candidates.
// jdesc - The JsepSessionDescription object to be serialized.
// return - SDP string serialized from the arguments.
std::string SdpSerialize(const JsepSessionDescription& jdesc);

// Serializes the passed in IceCandidateInterface to a SDP string.
// candidate - The candidate to be serialized.
std::string SdpSerializeCandidate(const IceCandidateInterface& candidate);

// Serializes a cricket Candidate.
// candidate - The candidate to be serialized.
RTC_EXPORT std::string SdpSerializeCandidate(const Candidate& candidate);

// Deserializes the passed in SDP string to a JsepSessionDescription.
// message - SDP string to be Deserialized.
// jdesc - The JsepSessionDescription deserialized from the SDP string.
// error - The detail error information when parsing fails.
// return - true on success, false on failure.
bool SdpDeserialize(absl::string_view message,
                    JsepSessionDescription* jdesc,
                    SdpParseError* error);

// Deserializes the passed in SDP string to one JsepIceCandidate.
// The first line must be a=candidate line and only the first line will be
// parsed.
// message - The SDP string to be Deserialized.
// candidates - The JsepIceCandidate from the SDP string.
// error - The detail error information when parsing fails.
// return - true on success, false on failure.
RTC_EXPORT bool SdpDeserializeCandidate(absl::string_view message,
                                        JsepIceCandidate* candidate,
                                        SdpParseError* error);

// Deserializes the passed in SDP string to a cricket Candidate.
// The first line must be a=candidate line and only the first line will be
// parsed.
// transport_name - The transport name (MID) of the candidate.
// message - The SDP string to be deserialized.
// candidate - The cricket Candidate from the SDP string.
// error - The detail error information when parsing fails.
// return - true on success, false on failure.
RTC_EXPORT bool SdpDeserializeCandidate(absl::string_view transport_name,
                                        absl::string_view message,
                                        Candidate* candidate,
                                        SdpParseError* error);

// Parses `message` according to the grammar defined in RFC 5245, Section 15.1
// and, if successful, stores the result in `candidate` and returns true.
// If unsuccessful, returns false and stores error information in `error` if
// `error` is not null.
// If `is_raw` is false, `message` is expected to be prefixed with "a=".
// If `is_raw` is true, no prefix is expected in `messaage`.
RTC_EXPORT bool ParseCandidate(absl::string_view message,
                               Candidate* candidate,
                               SdpParseError* error,
                               bool is_raw);

// Generates an FMTP line based on `parameters`. Please note that some
// parameters are not considered to be part of the FMTP line, see the function
// IsFmtpParam(). Returns true if the set of FMTP parameters is nonempty, false
// otherwise.
bool WriteFmtpParameters(const webrtc::CodecParameterMap& parameters,
                         StringBuilder* os);

// Parses a string into an FMTP parameter set, in key-value format.
bool ParseFmtpParameterSet(absl::string_view line_params,
                           webrtc::CodecParameterMap& codec_params,
                           SdpParseError* error);

}  // namespace webrtc

#endif  // PC_WEBRTC_SDP_H_
