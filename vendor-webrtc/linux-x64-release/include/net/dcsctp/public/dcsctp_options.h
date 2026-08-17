/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_DCSCTP_OPTIONS_H_
#define NET_DCSCTP_PUBLIC_DCSCTP_OPTIONS_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "net/dcsctp/public/types.h"

namespace dcsctp {
struct DcSctpOptions {
  // The largest safe SCTP packet. Starting from the minimum guaranteed MTU
  // value of 1280 for IPv6 (which may not support fragmentation), take off 85
  // bytes for DTLS/TURN/TCP/IP and ciphertext overhead.
  //
  // Additionally, it's possible that TURN adds an additional 4 bytes of
  // overhead after a channel has been established, so an additional 4 bytes is
  // subtracted
  //
  // 1280 IPV6 MTU
  //  -40 IPV6 header
  //   -8 UDP
  //  -24 GCM Cipher
  //  -13 DTLS record header
  //   -4 TURN ChannelData
  // = 1191 bytes.
  static constexpr size_t kMaxSafeMTUSize = 1191;

  // The local port for which the socket is supposed to be bound to. Incoming
  // packets will be verified that they are sent to this port number and all
  // outgoing packets will have this port number as source port.
  int local_port = 5000;

  // The remote port to send packets to. All outgoing packets will have this
  // port number as destination port.
  int remote_port = 5000;

  // The announced maximum number of incoming streams. Note that this value is
  // constant and can't be currently increased in run-time as "Add Incoming
  // Streams Request" in RFC6525 isn't supported.
  //
  // The socket implementation doesn't have any per-stream fixed costs, which is
  // why the default value is set to be the maximum value.
  uint16_t announced_maximum_incoming_streams = 65535;

  // The announced maximum number of outgoing streams. Note that this value is
  // constant and can't be currently increased in run-time as "Add Outgoing
  // Streams Request" in RFC6525 isn't supported.
  //
  // The socket implementation doesn't have any per-stream fixed costs, which is
  // why the default value is set to be the maximum value.
  uint16_t announced_maximum_outgoing_streams = 65535;

  // Maximum SCTP packet size. The library will limit the size of generated
  // packets to be less than or equal to this number. This does not include any
  // overhead of DTLS, TURN, UDP or IP headers.
  size_t mtu = kMaxSafeMTUSize;

  // The largest allowed message payload to be sent. Messages will be rejected
  // if their payload is larger than this value. Note that this doesn't affect
  // incoming messages, which may larger than this value (but smaller than
  // `max_receiver_window_buffer_size`).
  size_t max_message_size = 256 * 1024;

  // The default stream priority, if not overridden by
  // `SctpSocket::SetStreamPriority`. The default value is selected to be
  // compatible with https://www.w3.org/TR/webrtc-priority/, section 4.2-4.3.
  StreamPriority default_stream_priority = StreamPriority(256);

  // Maximum received window buffer size. This should be a bit larger than the
  // largest sized message you want to be able to receive. This essentially
  // limits the memory usage on the receive side. Note that memory is allocated
  // dynamically, and this represents the maximum amount of buffered data. The
  // actual memory usage of the library will be smaller in normal operation, and
  // will be larger than this due to other allocations and overhead if the
  // buffer is fully utilized.
  size_t max_receiver_window_buffer_size = 5 * 1024 * 1024;

  // Send queue total size limit. It will not be possible to queue more data if
  // the queue size is larger than this number.
  size_t max_send_buffer_size = 2'000'000;

  // Per stream send queue size limit. Similar to `max_send_buffer_size`, but
  // limiting the size of individual streams.
  size_t per_stream_send_queue_limit = 2'000'000;

  // A threshold that, when the amount of data in the send buffer goes below
  // this value, will trigger `DcSctpCallbacks::OnTotalBufferedAmountLow`.
  size_t total_buffered_amount_low_threshold = 1'800'000;

  // Max allowed RTT value. When the RTT is measured and it's found to be larger
  // than this value, it will be discarded and not used for e.g. any RTO
  // calculation. The default value is an extreme maximum but can be adapted
  // to better match the environment.
  DurationMs rtt_max = DurationMs(60'000);

  // Initial RTO value.
  DurationMs rto_initial = DurationMs(500);

  // Maximum RTO value.
  DurationMs rto_max = DurationMs(60'000);

  // Minimum RTO value. This must be larger than an expected peer delayed ack
  // timeout.
  DurationMs rto_min = DurationMs(400);

  // T1-init timeout.
  DurationMs t1_init_timeout = DurationMs(1000);

  // T1-cookie timeout.
  DurationMs t1_cookie_timeout = DurationMs(1000);

  // T2-shutdown timeout.
  DurationMs t2_shutdown_timeout = DurationMs(1000);

  // For t1-init, t1-cookie, t2-shutdown, t3-rtx, this value - if set - will be
  // the upper bound on how large the exponentially backed off timeout can
  // become. The lower the duration, the faster the connection can recover on
  // transient network issues. Setting this value may require changing
  // `max_retransmissions` and `max_init_retransmits` to ensure that the
  // connection is not closed too quickly.
  std::optional<DurationMs> max_timer_backoff_duration = std::nullopt;

  // Hearbeat interval (on idle connections only). Set to zero to disable.
  DurationMs heartbeat_interval = DurationMs(30000);

  // The maximum time when a SACK will be sent from the arrival of an
  // unacknowledged packet. Whatever is smallest of RTO/2 and this will be used.
  DurationMs delayed_ack_max_timeout = DurationMs(200);

  // The minimum limit for the measured RTT variance
  //
  // Setting this below the expected delayed ack timeout (+ margin) of the peer
  // might result in unnecessary retransmissions, as the maximum time it takes
  // to ACK a DATA chunk is typically RTT + ATO (delayed ack timeout), and when
  // the SCTP channel is quite idle, and heartbeats dominate the source of RTT
  // measurement, the RTO would converge with the smoothed RTT (SRTT). The
  // default ATO is 200ms in usrsctp, and a 20ms (10%) margin would include the
  // processing time of received packets and the clock granularity when setting
  // the delayed ack timer on the peer.
  //
  // This is defined as "G" in the algorithm for TCP in
  // https://datatracker.ietf.org/doc/html/rfc6298#section-4.
  //
  // Note that this value will be further adjusted by scaling factors, so if you
  // intend to change this, do it incrementally and measure the results.
  DurationMs min_rtt_variance = DurationMs(220);

  // The initial congestion window size, in number of MTUs.
  // See https://tools.ietf.org/html/rfc4960#section-7.2.1 which defaults at ~3
  // and https://research.google/pubs/pub36640/ which argues for at least ten
  // segments.
  size_t cwnd_mtus_initial = 10;

  // The minimum congestion window size, in number of MTUs, upon detection of
  // packet loss by SACK. Note that if the retransmission timer expires, the
  // congestion window will be as small as one MTU. See
  // https://tools.ietf.org/html/rfc4960#section-7.2.3.
  size_t cwnd_mtus_min = 4;

  // When the congestion window is at or above this number of MTUs, the
  // congestion control algorithm will avoid filling the congestion window
  // fully, if that results in fragmenting large messages into quite small
  // packets. When the congestion window is smaller than this option, it will
  // aim to fill the congestion window as much as it can, even if it results in
  // creating small fragmented packets.
  size_t avoid_fragmentation_cwnd_mtus = 6;

  // When the congestion window is below this number of MTUs, sent data chunks
  // will have the "I" (Immediate SACK - RFC7053) bit set. That will prevent the
  // receiver from delaying the SACK, which result in shorter time until the
  // sender can send the next packet as its driven by SACKs. This can reduce
  // latency for low utilized and lossy connections.
  //
  // Default value set to be same as initial congestion window. Set to zero to
  // disable.
  size_t immediate_sack_under_cwnd_mtus = 10;

  // The number of packets that may be sent at once. This is limited to avoid
  // bursts that too quickly fill the send buffer. Typically in a a socket in
  // its "slow start" phase (when it sends as much as it can), it will send
  // up to three packets for every SACK received, so the default limit is set
  // just above that, and then mostly applicable for (but not limited to) fast
  // retransmission scenarios.
  int max_burst = 4;

  // Maximum Data Retransmit Attempts (per DATA chunk). Set to std::nullopt for
  // no limit.
  std::optional<int> max_retransmissions = 10;

  // Max.Init.Retransmits (https://tools.ietf.org/html/rfc4960#section-15). Set
  // to std::nullopt for no limit.
  std::optional<int> max_init_retransmits = 8;

  // RFC3758 Partial Reliability Extension
  bool enable_partial_reliability = true;

  // RFC8260 Stream Schedulers and User Message Interleaving
  bool enable_message_interleaving = false;

  // If RTO should be added to heartbeat_interval
  bool heartbeat_interval_include_rtt = true;

  // Disables SCTP packet crc32 verification. For fuzzers only!
  bool disable_checksum_verification = false;

  // Controls the "zero checksum option" feature, as defined in
  // https://www.ietf.org/archive/id/draft-ietf-tsvwg-sctp-zero-checksum-06.html.
  // To have this feature enabled, both peers must be configured to use the
  // same (defined, not "none") alternate error detection method.
  ZeroChecksumAlternateErrorDetectionMethod
      zero_checksum_alternate_error_detection_method =
          ZeroChecksumAlternateErrorDetectionMethod::None();
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_DCSCTP_OPTIONS_H_
