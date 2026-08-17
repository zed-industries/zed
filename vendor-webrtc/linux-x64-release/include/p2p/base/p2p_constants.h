/*
 *  Copyright 2004 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef P2P_BASE_P2P_CONSTANTS_H_
#define P2P_BASE_P2P_CONSTANTS_H_

#include <stddef.h>
#include <stdint.h>

#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// CN_ == "content name".  When we initiate a session, we choose the
// name, and when we receive a Gingle session, we provide default
// names (since Gingle has no content names).  But when we receive a
// Jingle call, the content name can be anything, so don't rely on
// these values being the same as the ones received.
extern const char CN_AUDIO[];
extern const char CN_VIDEO[];
extern const char CN_DATA[];
extern const char CN_OTHER[];

// GN stands for group name
extern const char GROUP_TYPE_BUNDLE[];

RTC_EXPORT extern const int ICE_UFRAG_LENGTH;
RTC_EXPORT extern const int ICE_PWD_LENGTH;
extern const size_t ICE_UFRAG_MIN_LENGTH;
extern const size_t ICE_PWD_MIN_LENGTH;
extern const size_t ICE_UFRAG_MAX_LENGTH;
extern const size_t ICE_PWD_MAX_LENGTH;

RTC_EXPORT extern const int ICE_CANDIDATE_COMPONENT_RTP;
RTC_EXPORT extern const int ICE_CANDIDATE_COMPONENT_RTCP;
RTC_EXPORT extern const int ICE_CANDIDATE_COMPONENT_DEFAULT;

// RFC 4145, SDP setup attribute values.
extern const char CONNECTIONROLE_ACTIVE_STR[];
extern const char CONNECTIONROLE_PASSIVE_STR[];
extern const char CONNECTIONROLE_ACTPASS_STR[];
extern const char CONNECTIONROLE_HOLDCONN_STR[];

// RFC 6762, the .local pseudo-top-level domain used for mDNS names.
extern const char LOCAL_TLD[];

// Constants for time intervals are in milliseconds unless otherwise stated.
//
// Most of the following constants are the default values of IceConfig
// paramters. See IceConfig for detailed definition.
//
// Default value of IceConfig.receiving_timeout.
extern const int RECEIVING_TIMEOUT;
// Default value IceConfig.ice_check_min_interval.
extern const int MIN_CHECK_RECEIVING_INTERVAL;
// The next two ping intervals are at the ICE transport level.
//
// STRONG_PING_INTERVAL is applied when the selected connection is both
// writable and receiving.
//
// Default value of IceConfig.ice_check_interval_strong_connectivity.
extern const int STRONG_PING_INTERVAL;
// WEAK_PING_INTERVAL is applied when the selected connection is either
// not writable or not receiving.
//
// Defaul value of IceConfig.ice_check_interval_weak_connectivity.
extern const int WEAK_PING_INTERVAL;
// The next two ping intervals are at the candidate pair level.
//
// Writable candidate pairs are pinged at a slower rate once they are stabilized
// and the channel is strongly connected.
extern const int STRONG_AND_STABLE_WRITABLE_CONNECTION_PING_INTERVAL;
// Writable candidate pairs are pinged at a faster rate while the connections
// are stabilizing or the channel is weak.
extern const int WEAK_OR_STABILIZING_WRITABLE_CONNECTION_PING_INTERVAL;
// Default value of IceConfig.backup_connection_ping_interval
extern const int BACKUP_CONNECTION_PING_INTERVAL;
// Defualt value of IceConfig.receiving_switching_delay.
extern const int RECEIVING_SWITCHING_DELAY;
// Default value of IceConfig.regather_on_failed_networks_interval.
extern const int REGATHER_ON_FAILED_NETWORKS_INTERVAL;
// Default vaule of IceConfig.ice_unwritable_timeout.
extern const int CONNECTION_WRITE_CONNECT_TIMEOUT;
// Default vaule of IceConfig.ice_unwritable_min_checks.
extern const uint32_t CONNECTION_WRITE_CONNECT_FAILURES;
// Default value of IceConfig.ice_inactive_timeout;
extern const int CONNECTION_WRITE_TIMEOUT;
// Default value of IceConfig.stun_keepalive_interval;
extern const int STUN_KEEPALIVE_INTERVAL;

static const int MIN_PINGS_AT_WEAK_PING_INTERVAL = 3;

// The following constants are used at the candidate pair level to determine the
// state of a candidate pair.
//
// The timeout duration when a connection does not receive anything.
extern const int WEAK_CONNECTION_RECEIVE_TIMEOUT;
// A connection will be declared dead if it has not received anything for this
// long.
extern const int DEAD_CONNECTION_RECEIVE_TIMEOUT;
// This is the length of time that we wait for a ping response to come back.
extern const int CONNECTION_RESPONSE_TIMEOUT;
// The minimum time we will wait before destroying a connection after creating
// it.
extern const int MIN_CONNECTION_LIFETIME;

// The type preference MUST be an integer from 0 to 126 inclusive.
// https://datatracker.ietf.org/doc/html/rfc5245#section-4.1.2.1
enum IcePriorityValue : uint8_t {
  ICE_TYPE_PREFERENCE_RELAY_TLS = 0,
  ICE_TYPE_PREFERENCE_RELAY_TCP = 1,
  ICE_TYPE_PREFERENCE_RELAY_UDP = 2,
  ICE_TYPE_PREFERENCE_PRFLX_TCP = 80,
  ICE_TYPE_PREFERENCE_HOST_TCP = 90,
  ICE_TYPE_PREFERENCE_SRFLX = 100,
  ICE_TYPE_PREFERENCE_PRFLX = 110,
  ICE_TYPE_PREFERENCE_HOST = 126
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::BACKUP_CONNECTION_PING_INTERVAL;
using ::webrtc::CN_AUDIO;
using ::webrtc::CN_DATA;
using ::webrtc::CN_OTHER;
using ::webrtc::CN_VIDEO;
using ::webrtc::CONNECTION_RESPONSE_TIMEOUT;
using ::webrtc::CONNECTION_WRITE_CONNECT_FAILURES;
using ::webrtc::CONNECTION_WRITE_CONNECT_TIMEOUT;
using ::webrtc::CONNECTION_WRITE_TIMEOUT;
using ::webrtc::CONNECTIONROLE_ACTIVE_STR;
using ::webrtc::CONNECTIONROLE_ACTPASS_STR;
using ::webrtc::CONNECTIONROLE_HOLDCONN_STR;
using ::webrtc::CONNECTIONROLE_PASSIVE_STR;
using ::webrtc::DEAD_CONNECTION_RECEIVE_TIMEOUT;
using ::webrtc::GROUP_TYPE_BUNDLE;
using ::webrtc::ICE_CANDIDATE_COMPONENT_DEFAULT;
using ::webrtc::ICE_CANDIDATE_COMPONENT_RTCP;
using ::webrtc::ICE_CANDIDATE_COMPONENT_RTP;
using ::webrtc::ICE_PWD_LENGTH;
using ::webrtc::ICE_PWD_MAX_LENGTH;
using ::webrtc::ICE_PWD_MIN_LENGTH;
using ::webrtc::ICE_TYPE_PREFERENCE_HOST;
using ::webrtc::ICE_TYPE_PREFERENCE_HOST_TCP;
using ::webrtc::ICE_TYPE_PREFERENCE_PRFLX;
using ::webrtc::ICE_TYPE_PREFERENCE_PRFLX_TCP;
using ::webrtc::ICE_TYPE_PREFERENCE_RELAY_TCP;
using ::webrtc::ICE_TYPE_PREFERENCE_RELAY_TLS;
using ::webrtc::ICE_TYPE_PREFERENCE_RELAY_UDP;
using ::webrtc::ICE_TYPE_PREFERENCE_SRFLX;
using ::webrtc::ICE_UFRAG_LENGTH;
using ::webrtc::ICE_UFRAG_MAX_LENGTH;
using ::webrtc::ICE_UFRAG_MIN_LENGTH;
using ::webrtc::IcePriorityValue;
using ::webrtc::LOCAL_TLD;
using ::webrtc::MIN_CHECK_RECEIVING_INTERVAL;
using ::webrtc::MIN_CONNECTION_LIFETIME;
using ::webrtc::MIN_PINGS_AT_WEAK_PING_INTERVAL;
using ::webrtc::RECEIVING_SWITCHING_DELAY;
using ::webrtc::RECEIVING_TIMEOUT;
using ::webrtc::REGATHER_ON_FAILED_NETWORKS_INTERVAL;
using ::webrtc::STRONG_AND_STABLE_WRITABLE_CONNECTION_PING_INTERVAL;
using ::webrtc::STRONG_PING_INTERVAL;
using ::webrtc::STUN_KEEPALIVE_INTERVAL;
using ::webrtc::WEAK_CONNECTION_RECEIVE_TIMEOUT;
using ::webrtc::WEAK_OR_STABILIZING_WRITABLE_CONNECTION_PING_INTERVAL;
using ::webrtc::WEAK_PING_INTERVAL;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // P2P_BASE_P2P_CONSTANTS_H_
