/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_ICE_SERVER_PARSING_H_
#define PC_ICE_SERVER_PARSING_H_

#include <vector>

#include "api/peer_connection_interface.h"
#include "api/rtc_error.h"
#include "p2p/base/port.h"
#include "p2p/base/port_allocator.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Parses the URLs for each server in `servers` to build `stun_servers` and
// `turn_servers`. Can return SYNTAX_ERROR if the URL is malformed, or
// INVALID_PARAMETER if a TURN server is missing `username` or `password`.
//
// Intended to be used to convert/validate the servers passed into a
// PeerConnection through RTCConfiguration.
RTC_EXPORT RTCError
ParseIceServersOrError(const PeerConnectionInterface::IceServers& servers,
                       ServerAddresses* stun_servers,
                       std::vector<RelayServerConfig>* turn_servers);

// Calls `ParseIceServersOrError` to extract ice server information from the
// `configuration` and then validates the extracted configuration.
RTC_EXPORT RTCError ParseAndValidateIceServersFromConfiguration(
    const PeerConnectionInterface::RTCConfiguration& configuration,
    ServerAddresses& stun_servers,
    std::vector<RelayServerConfig>& turn_servers);

}  // namespace webrtc

#endif  // PC_ICE_SERVER_PARSING_H_
