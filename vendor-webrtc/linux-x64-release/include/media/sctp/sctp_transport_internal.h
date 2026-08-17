/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_SCTP_SCTP_TRANSPORT_INTERNAL_H_
#define MEDIA_SCTP_SCTP_TRANSPORT_INTERNAL_H_

// TODO(deadbeef): Move SCTP code out of media/, and make it not depend on
// anything in media/.

#include <cstddef>
#include <cstdint>
#include <functional>
#include <optional>

#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/sctp_transport_interface.h"
#include "api/transport/data_channel_transport_interface.h"
#include "p2p/dtls/dtls_transport_internal.h"
#include "rtc_base/copy_on_write_buffer.h"

namespace webrtc {

// Constants that are important to API users

// The number of outgoing streams that we'll negotiate. Since stream IDs (SIDs)
// are 0-based, the highest usable SID is 1023.
//
// It's recommended to use the maximum of 65535 in:
// https://tools.ietf.org/html/draft-ietf-rtcweb-data-channel-13#section-6.2
// However, we use 1024 in order to save memory. usrsctp allocates 104 bytes
// for each pair of incoming/outgoing streams (on a 64-bit system), so 65535
// streams would waste ~6MB.
//
// Note: "max" and "min" here are inclusive.
constexpr uint16_t kMaxSctpStreams = 1024;
constexpr uint16_t kMaxSctpSid = kMaxSctpStreams - 1;
constexpr uint16_t kMinSctpSid = 0;
// The maximum number of streams that can be negotiated according to spec.
constexpr uint16_t kSpecMaxSctpSid = 65535;

// This is the default SCTP port to use. It is passed along the wire and the
// connectee and connector must be using the same port. It is not related to the
// ports at the IP level. (Corresponds to: sockaddr_conn.sconn_port in
// usrsctp.h)
const int kSctpDefaultPort = 5000;

// Error cause codes defined at
// https://www.iana.org/assignments/sctp-parameters/sctp-parameters.xhtml#sctp-parameters-24
enum class SctpErrorCauseCode : uint16_t {
  kInvalidStreamIdentifier = 1,
  kMissingMandatoryParameter = 2,
  kStaleCookieError = 3,
  kOutOfResource = 4,
  kUnresolvableAddress = 5,
  kUnrecognizedChunkType = 6,
  kInvalidMandatoryParameter = 7,
  kUnrecognizedParameters = 8,
  kNoUserData = 9,
  kCookieReceivedWhileShuttingDown = 10,
  kRestartWithNewAddresses = 11,
  kUserInitiatedAbort = 12,
  kProtocolViolation = 13,
};

// Abstract SctpTransport interface for use internally (by PeerConnection etc.).
// Exists to allow mock/fake SctpTransports to be created.
class SctpTransportInternal {
 public:
  virtual ~SctpTransportInternal() {}

  virtual void SetOnConnectedCallback(std::function<void()> callback) = 0;
  virtual void SetDataChannelSink(DataChannelSink* sink) = 0;

  // Changes what underlying DTLS transport is uses. Used when switching which
  // bundled transport the SctpTransport uses.
  virtual void SetDtlsTransport(DtlsTransportInternal* transport) = 0;

  // When Start is called, connects as soon as possible; this can be called
  // before DTLS completes, in which case the connection will begin when DTLS
  // completes. This method can be called multiple times, though not if either
  // of the ports are changed.
  //
  virtual bool Start(const SctpOptions& options) = 0;
  // TODO(deadbeef): Support calling Start with different local/remote ports
  // and create a new association? Not clear if this is something we need to
  // support though. See: https://github.com/w3c/webrtc-pc/issues/979
  [[deprecated("Call with SctpOptions")]]
  virtual bool Start(int local_sctp_port,
                     int remote_sctp_port,
                     int max_message_size) {
    return Start({
        .local_port = local_sctp_port,
        .remote_port = remote_sctp_port,
        .max_message_size = max_message_size,
    });
  }

  // NOTE: Initially there was a "Stop" method here, but it was never used, so
  // it was removed.

  // Informs SctpTransport that `sid` will start being used. Returns false if
  // it is impossible to use `sid`, or if it's already in use.
  // Until calling this, can't send data using `sid`.
  // TODO(deadbeef): Actually implement the "returns false if `sid` can't be
  // used" part. See:
  // https://bugs.chromium.org/p/chromium/issues/detail?id=619849
  virtual bool OpenStream(int sid, PriorityValue priority) = 0;
  // The inverse of OpenStream. Begins the closing procedure, which will
  // eventually result in SignalClosingProcedureComplete on the side that
  // initiates it, and both SignalClosingProcedureStartedRemotely and
  // SignalClosingProcedureComplete on the other side.
  virtual bool ResetStream(int sid) = 0;
  // Send data down this channel.
  // Returns RTCError::OK() if successful an error otherwise. Notably
  // RTCErrorType::RESOURCE_EXHAUSTED for blocked operations.
  virtual RTCError SendData(int sid,
                            const SendDataParams& params,
                            const CopyOnWriteBuffer& payload) = 0;

  // Indicates when the SCTP socket is created and not blocked by congestion
  // control. This changes to false when SDR_BLOCK is returned from SendData,
  // and
  // changes to true when SignalReadyToSendData is fired. The underlying DTLS/
  // ICE channels may be unwritable while ReadyToSendData is true, because data
  // can still be queued in usrsctp.
  virtual bool ReadyToSendData() = 0;
  // Returns the current max message size, set with Start().
  virtual int max_message_size() const = 0;
  // Returns the current negotiated max # of outbound streams.
  // Will return std::nullopt if negotiation is incomplete.
  virtual std::optional<int> max_outbound_streams() const = 0;
  // Returns the current negotiated max # of inbound streams.
  virtual std::optional<int> max_inbound_streams() const = 0;
  // Returns the amount of buffered data in the send queue for a stream.
  virtual size_t buffered_amount(int sid) const = 0;
  virtual size_t buffered_amount_low_threshold(int sid) const = 0;
  virtual void SetBufferedAmountLowThreshold(int sid, size_t bytes) = 0;

  // Helper for debugging.
  virtual void set_debug_name_for_testing(const char* debug_name) = 0;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::kMaxSctpSid;
using ::webrtc::kMaxSctpStreams;
using ::webrtc::kMinSctpSid;
using ::webrtc::kSctpDefaultPort;
using ::webrtc::kSpecMaxSctpSid;
using ::webrtc::SctpErrorCauseCode;
using ::webrtc::SctpTransportInternal;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_SCTP_SCTP_TRANSPORT_INTERNAL_H_
