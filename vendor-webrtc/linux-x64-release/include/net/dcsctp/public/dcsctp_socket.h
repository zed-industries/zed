/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PUBLIC_DCSCTP_SOCKET_H_
#define NET_DCSCTP_PUBLIC_DCSCTP_SOCKET_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/task_queue/task_queue_base.h"
#include "api/units/timestamp.h"
#include "net/dcsctp/public/dcsctp_handover_state.h"
#include "net/dcsctp/public/dcsctp_message.h"
#include "net/dcsctp/public/dcsctp_options.h"
#include "net/dcsctp/public/packet_observer.h"
#include "net/dcsctp/public/timeout.h"
#include "net/dcsctp/public/types.h"

namespace dcsctp {

// The socket/association state
enum class SocketState {
  // The socket is closed.
  kClosed,
  // The socket has initiated a connection, which is not yet established. Note
  // that for incoming connections and for reconnections when the socket is
  // already connected, the socket will not transition to this state.
  kConnecting,
  // The socket is connected, and the connection is established.
  kConnected,
  // The socket is shutting down, and the connection is not yet closed.
  kShuttingDown,
};

// Send options for sending messages
struct SendOptions {
  // If the message should be sent with unordered message delivery.
  IsUnordered unordered = IsUnordered(false);

  // If set, will discard messages that haven't been correctly sent and
  // received before the lifetime has expired. This is only available if the
  // peer supports Partial Reliability Extension (RFC3758).
  std::optional<DurationMs> lifetime = std::nullopt;

  // If set, limits the number of retransmissions. This is only available
  // if the peer supports Partial Reliability Extension (RFC3758).
  std::optional<size_t> max_retransmissions = std::nullopt;

  // If set, will generate lifecycle events for this message. See e.g.
  // `DcSctpSocketCallbacks::OnLifecycleMessageFullySent`. This value is decided
  // by the client and the library will provide it to all lifecycle callbacks.
  LifecycleId lifecycle_id = LifecycleId::NotSet();
};

enum class ErrorKind {
  // Indicates that no error has occurred. This will never be the case when
  // `OnError` or `OnAborted` is called.
  kNoError,
  // There have been too many retries or timeouts, and the library has given up.
  kTooManyRetries,
  // A command was received that is only possible to execute when the socket is
  // connected, which it is not.
  kNotConnected,
  // Parsing of the command or its parameters failed.
  kParseFailed,
  // Commands are received in the wrong sequence, which indicates a
  // synchronisation mismatch between the peers.
  kWrongSequence,
  // The peer has reported an issue using ERROR or ABORT command.
  kPeerReported,
  // The peer has performed a protocol violation.
  kProtocolViolation,
  // The receive or send buffers have been exhausted.
  kResourceExhaustion,
  // The client has performed an invalid operation.
  kUnsupportedOperation,
};

inline constexpr absl::string_view ToString(ErrorKind error) {
  switch (error) {
    case ErrorKind::kNoError:
      return "NO_ERROR";
    case ErrorKind::kTooManyRetries:
      return "TOO_MANY_RETRIES";
    case ErrorKind::kNotConnected:
      return "NOT_CONNECTED";
    case ErrorKind::kParseFailed:
      return "PARSE_FAILED";
    case ErrorKind::kWrongSequence:
      return "WRONG_SEQUENCE";
    case ErrorKind::kPeerReported:
      return "PEER_REPORTED";
    case ErrorKind::kProtocolViolation:
      return "PROTOCOL_VIOLATION";
    case ErrorKind::kResourceExhaustion:
      return "RESOURCE_EXHAUSTION";
    case ErrorKind::kUnsupportedOperation:
      return "UNSUPPORTED_OPERATION";
  }
}

enum class SendStatus {
  // The message was enqueued successfully. As sending the message is done
  // asynchronously, this is no guarantee that the message has been actually
  // sent.
  kSuccess,
  // The message was rejected as the payload was empty (which is not allowed in
  // SCTP).
  kErrorMessageEmpty,
  // The message was rejected as the payload was larger than what has been set
  // as `DcSctpOptions.max_message_size`.
  kErrorMessageTooLarge,
  // The message could not be enqueued as the socket is out of resources. This
  // mainly indicates that the send queue is full.
  kErrorResourceExhaustion,
  // The message could not be sent as the socket is shutting down.
  kErrorShuttingDown,
};

inline constexpr absl::string_view ToString(SendStatus error) {
  switch (error) {
    case SendStatus::kSuccess:
      return "SUCCESS";
    case SendStatus::kErrorMessageEmpty:
      return "ERROR_MESSAGE_EMPTY";
    case SendStatus::kErrorMessageTooLarge:
      return "ERROR_MESSAGE_TOO_LARGE";
    case SendStatus::kErrorResourceExhaustion:
      return "ERROR_RESOURCE_EXHAUSTION";
    case SendStatus::kErrorShuttingDown:
      return "ERROR_SHUTTING_DOWN";
  }
}

// Return value of ResetStreams.
enum class ResetStreamsStatus {
  // If the connection is not yet established, this will be returned.
  kNotConnected,
  // Indicates that ResetStreams operation has been successfully initiated.
  kPerformed,
  // Indicates that ResetStreams has failed as it's not supported by the peer.
  kNotSupported,
};

inline constexpr absl::string_view ToString(ResetStreamsStatus error) {
  switch (error) {
    case ResetStreamsStatus::kNotConnected:
      return "NOT_CONNECTED";
    case ResetStreamsStatus::kPerformed:
      return "PERFORMED";
    case ResetStreamsStatus::kNotSupported:
      return "NOT_SUPPORTED";
  }
}

// Return value of DcSctpSocketCallbacks::SendPacketWithStatus.
enum class SendPacketStatus {
  // Indicates that the packet was successfully sent. As sending is unreliable,
  // there are no guarantees that the packet was actually delivered.
  kSuccess,
  // The packet was not sent due to a temporary failure, such as the local send
  // buffer becoming exhausted. This return value indicates that the socket will
  // recover and sending that packet can be retried at a later time.
  kTemporaryFailure,
  // The packet was not sent due to other reasons.
  kError,
};

// Represent known SCTP implementations.
enum class SctpImplementation {
  // There is not enough information toto determine any SCTP implementation.
  kUnknown,
  // This implementation.
  kDcsctp,
  // https://github.com/sctplab/usrsctp.
  kUsrSctp,
  // Any other implementation.
  kOther,
};

inline constexpr absl::string_view ToString(SctpImplementation implementation) {
  switch (implementation) {
    case SctpImplementation::kUnknown:
      return "unknown";
    case SctpImplementation::kDcsctp:
      return "dcsctp";
    case SctpImplementation::kUsrSctp:
      return "usrsctp";
    case SctpImplementation::kOther:
      return "other";
  }
}

// Tracked metrics, which is the return value of GetMetrics. Optional members
// will be unset when they are not yet known.
struct Metrics {
  // Transmission stats and metrics.

  // Number of packets sent.
  size_t tx_packets_count = 0;

  // Number of messages requested to be sent.
  size_t tx_messages_count = 0;

  // Number of packets retransmitted. Since SCTP packets can contain both
  // retransmitted DATA chunks and DATA chunks that are transmitted for the
  // first time, this represents an upper bound as it's incremented every time a
  // packet contains a retransmitted DATA chunk.
  size_t rtx_packets_count = 0;

  // Total number of bytes retransmitted. This includes the payload and
  // DATA/I-DATA headers, but not SCTP packet headers.
  uint64_t rtx_bytes_count = 0;

  // The current congestion window (cwnd) in bytes, corresponding to spinfo_cwnd
  // defined in RFC6458.
  size_t cwnd_bytes = 0;

  // Smoothed round trip time, corresponding to spinfo_srtt defined in RFC6458.
  int srtt_ms = 0;

  // Number of data items in the retransmission queue that haven’t been
  // acked/nacked yet and are in-flight. Corresponding to sstat_unackdata
  // defined in RFC6458. This may be an approximation when there are messages in
  // the send queue that haven't been fragmented/packetized yet.
  size_t unack_data_count = 0;

  // Receive stats and metrics.

  // Number of packets received.
  size_t rx_packets_count = 0;

  // Number of messages received.
  size_t rx_messages_count = 0;

  // The peer’s last announced receiver window size, corresponding to
  // sstat_rwnd defined in RFC6458.
  uint32_t peer_rwnd_bytes = 0;

  // Returns the detected SCTP implementation of the peer. As this is not
  // explicitly signalled during the connection establishment, heuristics is
  // used to analyze e.g. the state cookie in the INIT-ACK chunk.
  SctpImplementation peer_implementation = SctpImplementation::kUnknown;

  // Indicates if RFC8260 User Message Interleaving has been negotiated by both
  // peers.
  bool uses_message_interleaving = false;

  // Indicates if draft-tuexen-tsvwg-sctp-zero-checksum-00 has been negotiated
  // by both peers.
  bool uses_zero_checksum = false;

  // The number of negotiated incoming and outgoing streams, which is configured
  // locally as `DcSctpOptions::announced_maximum_incoming_streams` and
  // `DcSctpOptions::announced_maximum_outgoing_streams`, and which will be
  // signaled by the peer during connection.
  uint16_t negotiated_maximum_incoming_streams = 0;
  uint16_t negotiated_maximum_outgoing_streams = 0;
};

// Callbacks that the DcSctpSocket will call synchronously to the owning
// client. It is allowed to call back into the library from callbacks that start
// with "On". It has been explicitly documented when it's not allowed to call
// back into this library from within a callback.
//
// Theses callbacks are only synchronously triggered as a result of the client
// calling a public method in `DcSctpSocketInterface`.
class DcSctpSocketCallbacks {
 public:
  virtual ~DcSctpSocketCallbacks() = default;

  // Called when the library wants the packet serialized as `data` to be sent.
  //
  // TODO(bugs.webrtc.org/12943): This method is deprecated, see
  // `SendPacketWithStatus`.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual void SendPacket(webrtc::ArrayView<const uint8_t> /* data */) {}

  // Called when the library wants the packet serialized as `data` to be sent.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual SendPacketStatus SendPacketWithStatus(
      webrtc::ArrayView<const uint8_t> data) {
    SendPacket(data);
    return SendPacketStatus::kSuccess;
  }

  // Called when the library wants to create a Timeout. The callback must return
  // an object that implements that interface.
  //
  // Low precision tasks are scheduled more efficiently by using leeway to
  // reduce Idle Wake Ups and is the preferred precision whenever possible. High
  // precision timeouts do not have this leeway, but is still limited by OS
  // timer precision. At the time of writing, kLow's additional leeway may be up
  // to 17 ms, but please see webrtc::TaskQueueBase::DelayPrecision for
  // up-to-date information.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual std::unique_ptr<Timeout> CreateTimeout(
      webrtc::TaskQueueBase::DelayPrecision /* precision */) {
    // TODO(hbos): When dependencies have migrated to this new signature, make
    // this pure virtual and delete the other version.
    return CreateTimeout();
  }
  // TODO(hbos): When dependencies have migrated to the other signature, delete
  // this version.
  virtual std::unique_ptr<Timeout> CreateTimeout() {
    return CreateTimeout(webrtc::TaskQueueBase::DelayPrecision::kLow);
  }

  // Returns the current time in milliseconds (from any epoch).
  //
  // TODO(bugs.webrtc.org/15593): This method is deprecated, see `Now`.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual TimeMs TimeMillis() { return TimeMs(0); }

  // Returns the current time (from any epoch).
  //
  // This callback will eventually replace `TimeMillis()`.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual webrtc::Timestamp Now() {
    return webrtc::Timestamp::Millis(*TimeMillis());
  }

  // Called when the library needs a random number uniformly distributed between
  // `low` (inclusive) and `high` (exclusive). The random numbers used by the
  // library are not used for cryptographic purposes. There are no requirements
  // that the random number generator must be secure.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual uint32_t GetRandomInt(uint32_t low, uint32_t high) = 0;

  // Triggered when the outgoing message buffer is empty, meaning that there are
  // no more queued messages, but there can still be packets in-flight or to be
  // retransmitted. (in contrast to SCTP_SENDER_DRY_EVENT).
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  ABSL_DEPRECATED("Use OnTotalBufferedAmountLow instead")
  virtual void NotifyOutgoingMessageBufferEmpty() {}

  // Called when the library has received an SCTP message in full and delivers
  // it to the upper layer.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnMessageReceived(DcSctpMessage message) = 0;

  // Triggered when an non-fatal error is reported by either this library or
  // from the other peer (by sending an ERROR command). These should be logged,
  // but no other action need to be taken as the association is still viable.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnError(ErrorKind error, absl::string_view message) = 0;

  // Triggered when the socket has aborted - either as decided by this socket
  // due to e.g. too many retransmission attempts, or by the peer when
  // receiving an ABORT command. No other callbacks will be done after this
  // callback, unless reconnecting.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnAborted(ErrorKind error, absl::string_view message) = 0;

  // Called when calling `Connect` succeeds, but also for incoming successful
  // connection attempts.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnConnected() = 0;

  // Called when the socket is closed in a controlled way. No other
  // callbacks will be done after this callback, unless reconnecting.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnClosed() = 0;

  // On connection restarted (by peer). This is just a notification, and the
  // association is expected to work fine after this call, but there could have
  // been packet loss as a result of restarting the association.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnConnectionRestarted() = 0;

  // Indicates that a stream reset request has failed.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnStreamsResetFailed(
      webrtc::ArrayView<const StreamID> outgoing_streams,
      absl::string_view reason) = 0;

  // Indicates that a stream reset request has been performed.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnStreamsResetPerformed(
      webrtc::ArrayView<const StreamID> outgoing_streams) = 0;

  // When a peer has reset some of its outgoing streams, this will be called. An
  // empty list indicates that all streams have been reset.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnIncomingStreamsReset(
      webrtc::ArrayView<const StreamID> incoming_streams) = 0;

  // Will be called when the amount of data buffered to be sent falls to or
  // below the threshold set when calling `SetBufferedAmountLowThreshold`.
  //
  // It is allowed to call into this library from within this callback.
  virtual void OnBufferedAmountLow(StreamID /* stream_id */) {}

  // Will be called when the total amount of data buffered (in the entire send
  // buffer, for all streams) falls to or below the threshold specified in
  // `DcSctpOptions::total_buffered_amount_low_threshold`.
  virtual void OnTotalBufferedAmountLow() {}

  // == Lifecycle Events ==
  //
  // If a `lifecycle_id` is provided as `SendOptions`, lifecycle callbacks will
  // be triggered as the message is processed by the library.
  //
  // The possible transitions are shown in the graph below:
  //
  //        DcSctpSocket::Send  ────────────────────────┐
  //                │                                   │
  //                │                                   │
  //                v                                   v
  //    OnLifecycleMessageFullySent ───────> OnLifecycleMessageExpired
  //                │                                   │
  //                │                                   │
  //                v                                   v
  //    OnLifeCycleMessageDelivered ────────────> OnLifecycleEnd

  // OnLifecycleMessageFullySent will be called when a message has been fully
  // sent, meaning that the last fragment has been produced from the send queue
  // and sent on the network. Note that this will trigger at most once per
  // message even if the message was retransmitted due to packet loss.
  //
  // This is a lifecycle event.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual void OnLifecycleMessageFullySent(LifecycleId /* lifecycle_id */) {}

  // OnLifecycleMessageExpired will be called when a message has expired. If it
  // was expired with data remaining in the send queue that had not been sent
  // ever, `maybe_delivered` will be set to false. If `maybe_delivered` is true,
  // the message has at least once been sent and may have been correctly
  // received by the peer, but it has expired before the receiver managed to
  // acknowledge it. This means that if `maybe_delivered` is true, it's unknown
  // if the message was lost or was delivered, and if `maybe_delivered` is
  // false, it's guaranteed to not be delivered.
  //
  // It's guaranteed that `OnLifecycleMessageDelivered` is not called if this
  // callback has triggered.
  //
  // This is a lifecycle event.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual void OnLifecycleMessageExpired(LifecycleId /* lifecycle_id */,
                                         bool /* maybe_delivered */) {}

  // OnLifecycleMessageDelivered will be called when a non-expired message has
  // been acknowledged by the peer as delivered.
  //
  // Note that this will trigger only when the peer moves its cumulative TSN ack
  // beyond this message, and will not fire for messages acked using
  // gap-ack-blocks as those are renegable. This means that this may fire a bit
  // later than the message was actually first "acked" by the peer, as -
  // according to the protocol - those acks may be unacked later by the client.
  //
  // It's guaranteed that `OnLifecycleMessageExpired` is not called if this
  // callback has triggered.
  //
  // This is a lifecycle event.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual void OnLifecycleMessageDelivered(LifecycleId /* lifecycle_id */) {}

  // OnLifecycleEnd will be called when a lifecycle event has reached its end.
  // It will be called when processing of a message is complete, no matter how
  // it completed. It will be called after all other lifecycle events, if any.
  //
  // Note that it's possible that this callback triggers without any other
  // lifecycle callbacks having been called before in case of errors, such as
  // attempting to send an empty message or failing to enqueue a message if the
  // send queue is full.
  //
  // NOTE: When the socket is deallocated, there will be no `OnLifecycleEnd`
  // callbacks sent for messages that were enqueued. But as long as the socket
  // is alive, `OnLifecycleEnd` callbacks are guaranteed to be sent as messages
  // are either expired or successfully acknowledged.
  //
  // This is a lifecycle event.
  //
  // Note that it's NOT ALLOWED to call into this library from within this
  // callback.
  virtual void OnLifecycleEnd(LifecycleId /* lifecycle_id */) {}
};

// The DcSctpSocket implementation implements the following interface.
// This class is thread-compatible.
class DcSctpSocketInterface {
 public:
  virtual ~DcSctpSocketInterface() = default;

  // To be called when an incoming SCTP packet is to be processed.
  virtual void ReceivePacket(webrtc::ArrayView<const uint8_t> data) = 0;

  // To be called when a timeout has expired. The `timeout_id` is provided
  // when the timeout was initiated.
  virtual void HandleTimeout(TimeoutID timeout_id) = 0;

  // Connects the socket. This is an asynchronous operation, and
  // `DcSctpSocketCallbacks::OnConnected` will be called on success.
  virtual void Connect() = 0;

  // Puts this socket to the state in which the original socket was when its
  // `DcSctpSocketHandoverState` was captured by `GetHandoverStateAndClose`.
  // `RestoreFromState` is allowed only on the closed socket.
  // `DcSctpSocketCallbacks::OnConnected` will be called if a connected socket
  // state is restored.
  // `DcSctpSocketCallbacks::OnError` will be called on error.
  virtual void RestoreFromState(const DcSctpSocketHandoverState& state) = 0;

  // Gracefully shutdowns the socket and sends all outstanding data. This is an
  // asynchronous operation and `DcSctpSocketCallbacks::OnClosed` will be called
  // on success.
  virtual void Shutdown() = 0;

  // Closes the connection non-gracefully. Will send ABORT if the connection is
  // not already closed. No callbacks will be made after Close() has returned.
  virtual void Close() = 0;

  // The socket state.
  virtual SocketState state() const = 0;

  // The options it was created with.
  virtual const DcSctpOptions& options() const = 0;

  // Update the options max_message_size.
  virtual void SetMaxMessageSize(size_t max_message_size) = 0;

  // Sets the priority of an outgoing stream. The initial value, when not set,
  // is `DcSctpOptions::default_stream_priority`.
  virtual void SetStreamPriority(StreamID stream_id,
                                 StreamPriority priority) = 0;

  // Returns the currently set priority for an outgoing stream. The initial
  // value, when not set, is `DcSctpOptions::default_stream_priority`.
  virtual StreamPriority GetStreamPriority(StreamID stream_id) const = 0;

  // Sends the message `message` using the provided send options.
  // Sending a message is an asynchronous operation, and the `OnError` callback
  // may be invoked to indicate any errors in sending the message.
  //
  // The association does not have to be established before calling this method.
  // If it's called before there is an established association, the message will
  // be queued.
  virtual SendStatus Send(DcSctpMessage message,
                          const SendOptions& send_options) = 0;

  // Sends the messages `messages` using the provided send options.
  // Sending a message is an asynchronous operation, and the `OnError` callback
  // may be invoked to indicate any errors in sending the message.
  //
  // This has identical semantics to Send, except that it may coalesce many
  // messages into a single SCTP packet if they would fit.
  virtual std::vector<SendStatus> SendMany(
      webrtc::ArrayView<DcSctpMessage> messages,
      const SendOptions& send_options) = 0;

  // Resetting streams is an asynchronous operation and the results will
  // be notified using `DcSctpSocketCallbacks::OnStreamsResetDone()` on success
  // and `DcSctpSocketCallbacks::OnStreamsResetFailed()` on failure. Note that
  // only outgoing streams can be reset.
  //
  // When it's known that the peer has reset its own outgoing streams,
  // `DcSctpSocketCallbacks::OnIncomingStreamReset` is called.
  //
  // Note that resetting a stream will also remove all queued messages on those
  // streams, but will ensure that the currently sent message (if any) is fully
  // sent before closing the stream.
  //
  // Resetting streams can only be done on an established association that
  // supports stream resetting. Calling this method on e.g. a closed association
  // or streams that don't support resetting will not perform any operation.
  virtual ResetStreamsStatus ResetStreams(
      webrtc::ArrayView<const StreamID> outgoing_streams) = 0;

  // Returns the number of bytes of data currently queued to be sent on a given
  // stream.
  virtual size_t buffered_amount(StreamID stream_id) const = 0;

  // Returns the number of buffered outgoing bytes that is considered "low" for
  // a given stream. See `SetBufferedAmountLowThreshold`.
  virtual size_t buffered_amount_low_threshold(StreamID stream_id) const = 0;

  // Used to specify the number of bytes of buffered outgoing data that is
  // considered "low" for a given stream, which will trigger an
  // OnBufferedAmountLow event. The default value is zero (0).
  virtual void SetBufferedAmountLowThreshold(StreamID stream_id,
                                             size_t bytes) = 0;

  // Retrieves the latest metrics. If the socket is not fully connected,
  // `std::nullopt` will be returned. Note that metrics are not guaranteed to
  // be carried over if this socket is handed over by calling
  // `GetHandoverStateAndClose`.
  virtual std::optional<Metrics> GetMetrics() const = 0;

  // Returns empty bitmask if the socket is in the state in which a snapshot of
  // the state can be made by `GetHandoverStateAndClose()`. Return value is
  // invalidated by a call to any non-const method.
  virtual HandoverReadinessStatus GetHandoverReadiness() const = 0;

  // Collects a snapshot of the socket state that can be used to reconstruct
  // this socket in another process. On success this socket object is closed
  // synchronously and no callbacks will be made after the method has returned.
  // The method fails if the socket is not in a state ready for handover.
  // nullopt indicates the failure. `DcSctpSocketCallbacks::OnClosed` will be
  // called on success.
  virtual std::optional<DcSctpSocketHandoverState>
  GetHandoverStateAndClose() = 0;

  // Returns the detected SCTP implementation of the peer. As this is not
  // explicitly signalled during the connection establishment, heuristics is
  // used to analyze e.g. the state cookie in the INIT-ACK chunk.
  //
  // If this method is called too early (before
  // `DcSctpSocketCallbacks::OnConnected` has triggered), this will likely
  // return `SctpImplementation::kUnknown`.
  ABSL_DEPRECATED("See Metrics::peer_implementation instead")
  virtual SctpImplementation peer_implementation() const {
    return SctpImplementation::kUnknown;
  }
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_PUBLIC_DCSCTP_SOCKET_H_
