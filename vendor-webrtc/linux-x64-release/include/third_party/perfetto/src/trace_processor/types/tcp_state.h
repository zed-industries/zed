/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_TYPES_TCP_STATE_H_
#define SRC_TRACE_PROCESSOR_TYPES_TCP_STATE_H_

namespace perfetto {
namespace trace_processor {

// Sock IPV4 Protocol Definition, from include/uapi/linux/in.h.
constexpr int kAfNet = 2;
// Sock IPV6 Protocol Definition, from include/uapi/linux/in.h.
constexpr int kAfNet6 = 10;
// Sock TCP protocol Definition, from include/uapi/linux/in.h.
constexpr int kIpprotoTcp = 6;
// Sock UDP protocol Definition, from include/uapi/linux/in.h.
constexpr int kIpprotoUdp = 17;
// Sock ICMP protocol Definition, from include/uapi/linux/in.h.
constexpr int kIpprotoIcmp = 1;
// Sock ICMPV6 protocol Definition, from include/uapi/linux/in.h.
constexpr int kIpprotoIcmpv6 = 58;
// Skb IPV4 Protocol Definition, from include/uapi/linux/if_ether.h.
constexpr int kEthPIp = 0x800;
// Skb IPV6 Protocol Definition, from include/uapi/linux/if_ether.h.
constexpr int kEthPIp6 = 0x86DD;
// TCP protocol states, from include/net/tcp_states.h.
enum {
  TCP_ESTABLISHED = 1,
  TCP_SYN_SENT,
  TCP_SYN_RECV,
  TCP_FIN_WAIT1,
  TCP_FIN_WAIT2,
  TCP_TIME_WAIT,
  TCP_CLOSE,
  TCP_CLOSE_WAIT,
  TCP_LAST_ACK,
  TCP_LISTEN,
  TCP_CLOSING,
  TCP_NEW_SYN_RECV,
  TCP_MAX_STATES
};
// TCP protocol state to string mapping.
static constexpr const char* const kTcpStateNames[] = {
    "TCP_UNKNOWN",      "TCP_ESTABLISHED", "TCP_SYN_SENT",  "TCP_SYN_RECV",
    "TCP_FIN_WAIT1",    "TCP_FIN_WAIT2",   "TCP_TIME_WAIT", "TCP_CLOSE",
    "TCP_CLOSE_WAIT",   "TCP_LAST_ACK",    "TCP_LISTEN",    "TCP_CLOSING",
    "TCP_NEW_SYN_RECV", "TCP_MAX_STATES"};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_TYPES_TCP_STATE_H_
