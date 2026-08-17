// Copyright 2022 The gRPC Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_INTERNAL_ERRQUEUE_H
#define GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_INTERNAL_ERRQUEUE_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include "src/core/lib/iomgr/port.h"

#ifdef GRPC_POSIX_SOCKET_TCP

#include <time.h>

#ifdef GRPC_LINUX_ERRQUEUE
#include <linux/errqueue.h>  // IWYU pragma: keep
#include <sys/socket.h>
#endif  // GRPC_LINUX_ERRQUEUE

namespace grpc_event_engine::experimental {

#ifdef GRPC_LINUX_ERRQUEUE

// Redefining scm_timestamping in the same way that <linux/errqueue.h> defines
// it, so that code compiles on systems that don't have it.
struct scm_timestamping {
  struct timespec ts[3];
};
// Also redefine timestamp types
// The timestamp type for when the driver passed skb to NIC, or HW.
constexpr int SCM_TSTAMP_SND = 0;
// The timestamp type for when data entered the packet scheduler.
constexpr int SCM_TSTAMP_SCHED = 1;
// The timestamp type for when data acknowledged by peer.
constexpr int SCM_TSTAMP_ACK = 2;

// Control message type containing OPT_STATS
#ifndef SCM_TIMESTAMPING_OPT_STATS
#define SCM_TIMESTAMPING_OPT_STATS 54
#endif

// Redefine required constants from <linux/net_tstamp.h>
constexpr uint32_t SOF_TIMESTAMPING_TX_SOFTWARE = 1u << 1;
constexpr uint32_t SOF_TIMESTAMPING_SOFTWARE = 1u << 4;
constexpr uint32_t SOF_TIMESTAMPING_OPT_ID = 1u << 7;
constexpr uint32_t SOF_TIMESTAMPING_TX_SCHED = 1u << 8;
constexpr uint32_t SOF_TIMESTAMPING_TX_ACK = 1u << 9;
constexpr uint32_t SOF_TIMESTAMPING_OPT_TSONLY = 1u << 11;
constexpr uint32_t SOF_TIMESTAMPING_OPT_STATS = 1u << 12;

constexpr uint32_t kTimestampingSocketOptions =
    SOF_TIMESTAMPING_SOFTWARE | SOF_TIMESTAMPING_OPT_ID |
    SOF_TIMESTAMPING_OPT_TSONLY | SOF_TIMESTAMPING_OPT_STATS;
constexpr uint32_t kTimestampingRecordingOptions =
    SOF_TIMESTAMPING_TX_SCHED | SOF_TIMESTAMPING_TX_SOFTWARE |
    SOF_TIMESTAMPING_TX_ACK;

// Netlink attribute types used for TCP opt stats.
enum TCPOptStats {
  TCP_NLA_PAD,
  TCP_NLA_BUSY,                   // Time (usec) busy sending data.
  TCP_NLA_RWND_LIMITED,           // Time (usec) limited by receive window.
  TCP_NLA_SNDBUF_LIMITED,         // Time (usec) limited by send buffer.
  TCP_NLA_DATA_SEGS_OUT,          // Data pkts sent including retransmission.
  TCP_NLA_TOTAL_RETRANS,          // Data pkts retransmitted.
  TCP_NLA_PACING_RATE,            // Pacing rate in Bps.
  TCP_NLA_DELIVERY_RATE,          // Delivery rate in Bps.
  TCP_NLA_SND_CWND,               // Sending congestion window.
  TCP_NLA_REORDERING,             // Reordering metric.
  TCP_NLA_MIN_RTT,                // minimum RTT.
  TCP_NLA_RECUR_RETRANS,          // Recurring retransmits for the current pkt.
  TCP_NLA_DELIVERY_RATE_APP_LMT,  // Delivery rate application limited?
  TCP_NLA_SNDQ_SIZE,              // Data (bytes) pending in send queue
  TCP_NLA_CA_STATE,               // ca_state of socket
  TCP_NLA_SND_SSTHRESH,           // Slow start size threshold
  TCP_NLA_DELIVERED,              // Data pkts delivered incl. out-of-order
  TCP_NLA_DELIVERED_CE,           // Like above but only ones w/ CE marks
  TCP_NLA_BYTES_SENT,             // Data bytes sent including retransmission
  TCP_NLA_BYTES_RETRANS,          // Data bytes retransmitted
  TCP_NLA_DSACK_DUPS,             // DSACK blocks received
  TCP_NLA_REORD_SEEN,             // reordering events seen
  TCP_NLA_SRTT,                   // smoothed RTT in usecs
};

// tcp_info from from linux/tcp.h
struct tcp_info {
  uint8_t tcpi_state;
  uint8_t tcpi_ca_state;
  uint8_t tcpi_retransmits;
  uint8_t tcpi_probes;
  uint8_t tcpi_backoff;
  uint8_t tcpi_options;
  uint8_t tcpi_snd_wscale : 4, tcpi_rcv_wscale : 4;
  uint8_t tcpi_delivery_rate_app_limited : 1;
  uint32_t tcpi_rto;
  uint32_t tcpi_ato;
  uint32_t tcpi_snd_mss;
  uint32_t tcpi_rcv_mss;
  uint32_t tcpi_unacked;
  uint32_t tcpi_sacked;
  uint32_t tcpi_lost;
  uint32_t tcpi_retrans;
  uint32_t tcpi_fackets;
  // Times.
  uint32_t tcpi_last_data_sent;
  uint32_t tcpi_last_ack_sent;  // Not remembered, sorry.
  uint32_t tcpi_last_data_recv;
  uint32_t tcpi_last_ack_recv;
  // Metrics.
  uint32_t tcpi_pmtu;
  uint32_t tcpi_rcv_ssthresh;
  uint32_t tcpi_rtt;
  uint32_t tcpi_rttvar;
  uint32_t tcpi_snd_ssthresh;
  uint32_t tcpi_snd_cwnd;
  uint32_t tcpi_advmss;
  uint32_t tcpi_reordering;
  uint32_t tcpi_rcv_rtt;
  uint32_t tcpi_rcv_space;
  uint32_t tcpi_total_retrans;
  uint64_t tcpi_pacing_rate;
  uint64_t tcpi_max_pacing_rate;
  uint64_t tcpi_bytes_acked;     // RFC4898 tcpEStatsAppHCThruOctetsAcked
  uint64_t tcpi_bytes_received;  // RFC4898 tcpEStatsAppHCThruOctetsReceived

  uint32_t tcpi_segs_out;  // RFC4898 tcpEStatsPerfSegsOut
  uint32_t tcpi_segs_in;   // RFC4898 tcpEStatsPerfSegsIn
  uint32_t tcpi_notsent_bytes;
  uint32_t tcpi_min_rtt;

  uint32_t tcpi_data_segs_in;   // RFC4898 tcpEStatsDataSegsIn
  uint32_t tcpi_data_segs_out;  // RFC4898 tcpEStatsDataSegsOut

  uint64_t tcpi_delivery_rate;
  uint64_t tcpi_busy_time;       // Time (usec) busy sending data
  uint64_t tcpi_rwnd_limited;    // Time (usec) limited by receive window
  uint64_t tcpi_sndbuf_limited;  // Time (usec) limited by send buffer

  uint32_t tcpi_delivered;
  uint32_t tcpi_delivered_ce;
  uint64_t tcpi_bytes_sent;     // RFC4898 tcpEStatsPerfHCDataOctetsOut
  uint64_t tcpi_bytes_retrans;  // RFC4898 tcpEStatsPerfOctetsRetrans
  uint32_t tcpi_dsack_dups;     // RFC4898 tcpEStatsStackDSACKDups
  uint32_t tcpi_reord_seen;     // reordering events seen
  socklen_t length;             // Length of struct returned by kernel
};

#ifndef TCP_INFO
#define TCP_INFO 11
#endif

int GetSocketTcpInfo(tcp_info* info, int fd);

#endif  // GRPC_LINUX_ERRQUEUE

// Returns true if kernel is capable of supporting errqueue and timestamping.
// Currently allowing only linux kernels above 4.0.0
bool KernelSupportsErrqueue();

}  // namespace grpc_event_engine::experimental

#endif  // GRPC_POSIX_SOCKET_TCP

#endif  // GRPC_SRC_CORE_LIB_EVENT_ENGINE_POSIX_ENGINE_INTERNAL_ERRQUEUE_H
