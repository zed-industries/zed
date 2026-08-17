/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_COMMON_INTERNAL_TYPES_H_
#define NET_DCSCTP_COMMON_INTERNAL_TYPES_H_

#include <functional>
#include <utility>

#include "net/dcsctp/public/types.h"
#include "rtc_base/strong_alias.h"

namespace dcsctp {

// Stream Sequence Number (SSN)
using SSN = webrtc::StrongAlias<class SSNTag, uint16_t>;

// Message Identifier (MID)
using MID = webrtc::StrongAlias<class MIDTag, uint32_t>;

// Fragment Sequence Number (FSN)
using FSN = webrtc::StrongAlias<class FSNTag, uint32_t>;

// Transmission Sequence Number (TSN)
using TSN = webrtc::StrongAlias<class TSNTag, uint32_t>;

// Reconfiguration Request Sequence Number
using ReconfigRequestSN =
    webrtc::StrongAlias<class ReconfigRequestSNTag, uint32_t>;

// Verification Tag, used for packet validation.
using VerificationTag = webrtc::StrongAlias<class VerificationTagTag, uint32_t>;

// Tie Tag, used as a nonce when connecting.
using TieTag = webrtc::StrongAlias<class TieTagTag, uint64_t>;

// An ID for every outgoing message, to correlate outgoing data chunks with the
// message it was carved from.
using OutgoingMessageId =
    webrtc::StrongAlias<class OutgoingMessageIdTag, uint32_t>;

}  // namespace dcsctp
#endif  // NET_DCSCTP_COMMON_INTERNAL_TYPES_H_
