/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_SOCKET_DCSCTP_SOCKET_H_
#define NET_DCSCTP_SOCKET_DCSCTP_SOCKET_H_

#include <cstdint>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/chunk/abort_chunk.h"
#include "net/dcsctp/packet/chunk/chunk.h"
#include "net/dcsctp/packet/chunk/cookie_ack_chunk.h"
#include "net/dcsctp/packet/chunk/cookie_echo_chunk.h"
#include "net/dcsctp/packet/chunk/data_chunk.h"
#include "net/dcsctp/packet/chunk/data_common.h"
#include "net/dcsctp/packet/chunk/error_chunk.h"
#include "net/dcsctp/packet/chunk/forward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/forward_tsn_common.h"
#include "net/dcsctp/packet/chunk/heartbeat_ack_chunk.h"
#include "net/dcsctp/packet/chunk/heartbeat_request_chunk.h"
#include "net/dcsctp/packet/chunk/idata_chunk.h"
#include "net/dcsctp/packet/chunk/iforward_tsn_chunk.h"
#include "net/dcsctp/packet/chunk/init_ack_chunk.h"
#include "net/dcsctp/packet/chunk/init_chunk.h"
#include "net/dcsctp/packet/chunk/reconfig_chunk.h"
#include "net/dcsctp/packet/chunk/sack_chunk.h"
#include "net/dcsctp/packet/chunk/shutdown_ack_chunk.h"
#include "net/dcsctp/packet/chunk/shutdown_chunk.h"
#include "net/dcsctp/packet/chunk/shutdown_complete_chunk.h"
#include "net/dcsctp/packet/data.h"
#include "net/dcsctp/packet/sctp_packet.h"
#include "net/dcsctp/public/dcsctp_message.h"
#include "net/dcsctp/public/dcsctp_options.h"
#include "net/dcsctp/public/dcsctp_socket.h"
#include "net/dcsctp/public/packet_observer.h"
#include "net/dcsctp/rx/data_tracker.h"
#include "net/dcsctp/rx/reassembly_queue.h"
#include "net/dcsctp/socket/callback_deferrer.h"
#include "net/dcsctp/socket/packet_sender.h"
#include "net/dcsctp/socket/state_cookie.h"
#include "net/dcsctp/socket/transmission_control_block.h"
#include "net/dcsctp/timer/timer.h"
#include "net/dcsctp/tx/retransmission_error_counter.h"
#include "net/dcsctp/tx/retransmission_queue.h"
#include "net/dcsctp/tx/retransmission_timeout.h"
#include "net/dcsctp/tx/rr_send_queue.h"

namespace dcsctp {

// DcSctpSocket represents a single SCTP socket, to be used over DTLS.
//
// Every dcSCTP is completely isolated from any other socket.
//
// This class manages all packet and chunk dispatching and mainly handles the
// connection sequences (connect, close, shutdown, etc) as well as managing
// the Transmission Control Block (tcb).
//
// This class is thread-compatible.
class DcSctpSocket : public DcSctpSocketInterface {
 public:
  // Instantiates a DcSctpSocket, which interacts with the world through the
  // `callbacks` interface and is configured using `options`.
  //
  // For debugging, `log_prefix` will prefix all debug logs, and a
  // `packet_observer` can be attached to e.g. dump sent and received packets.
  DcSctpSocket(absl::string_view log_prefix,
               DcSctpSocketCallbacks& callbacks,
               std::unique_ptr<PacketObserver> packet_observer,
               const DcSctpOptions& options);

  DcSctpSocket(const DcSctpSocket&) = delete;
  DcSctpSocket& operator=(const DcSctpSocket&) = delete;

  // Implementation of `DcSctpSocketInterface`.
  void ReceivePacket(webrtc::ArrayView<const uint8_t> data) override;
  void HandleTimeout(TimeoutID timeout_id) override;
  void Connect() override;
  void RestoreFromState(const DcSctpSocketHandoverState& state) override;
  void Shutdown() override;
  void Close() override;
  SendStatus Send(DcSctpMessage message,
                  const SendOptions& send_options) override;
  std::vector<SendStatus> SendMany(webrtc::ArrayView<DcSctpMessage> messages,
                                   const SendOptions& send_options) override;
  ResetStreamsStatus ResetStreams(
      webrtc::ArrayView<const StreamID> outgoing_streams) override;
  SocketState state() const override;
  const DcSctpOptions& options() const override { return options_; }
  void SetMaxMessageSize(size_t max_message_size) override;
  void SetStreamPriority(StreamID stream_id, StreamPriority priority) override;
  StreamPriority GetStreamPriority(StreamID stream_id) const override;
  size_t buffered_amount(StreamID stream_id) const override;
  size_t buffered_amount_low_threshold(StreamID stream_id) const override;
  void SetBufferedAmountLowThreshold(StreamID stream_id, size_t bytes) override;
  std::optional<Metrics> GetMetrics() const override;
  HandoverReadinessStatus GetHandoverReadiness() const override;
  std::optional<DcSctpSocketHandoverState> GetHandoverStateAndClose() override;
  SctpImplementation peer_implementation() const override {
    return metrics_.peer_implementation;
  }
  // Returns this socket's verification tag, or zero if not yet connected.
  VerificationTag verification_tag() const {
    return tcb_ != nullptr ? tcb_->my_verification_tag() : VerificationTag(0);
  }

 private:
  // Parameter proposals valid during the connect phase.
  struct ConnectParameters {
    TSN initial_tsn = TSN(0);
    VerificationTag verification_tag = VerificationTag(0);
  };

  // Detailed state (separate from SocketState, which is the public state).
  enum class State {
    kClosed,
    kCookieWait,
    // TCB valid in these:
    kCookieEchoed,
    kEstablished,
    kShutdownPending,
    kShutdownSent,
    kShutdownReceived,
    kShutdownAckSent,
  };

  // Returns the log prefix used for debug logging.
  std::string log_prefix() const;

  bool IsConsistent() const;
  static constexpr absl::string_view ToString(DcSctpSocket::State state);

  void CreateTransmissionControlBlock(const Capabilities& capabilities,
                                      VerificationTag my_verification_tag,
                                      TSN my_initial_tsn,
                                      VerificationTag peer_verification_tag,
                                      TSN peer_initial_tsn,
                                      size_t a_rwnd,
                                      TieTag tie_tag);

  // Changes the socket state, given a `reason` (for debugging/logging).
  void SetState(State state, absl::string_view reason);
  // Closes the association. Note that the TCB will not be valid past this call.
  void InternalClose(ErrorKind error, absl::string_view message);
  // Closes the association, because of too many retransmission errors.
  void CloseConnectionBecauseOfTooManyTransmissionErrors();
  // Timer expiration handlers
  webrtc::TimeDelta OnInitTimerExpiry();
  webrtc::TimeDelta OnCookieTimerExpiry();
  webrtc::TimeDelta OnShutdownTimerExpiry();
  void OnSentPacket(webrtc::ArrayView<const uint8_t> packet,
                    SendPacketStatus status);
  // Sends SHUTDOWN or SHUTDOWN-ACK if the socket is shutting down and if all
  // outstanding data has been acknowledged.
  void MaybeSendShutdownOrAck();
  // If the socket is shutting down, responds SHUTDOWN to any incoming DATA.
  void MaybeSendShutdownOnPacketReceived(const SctpPacket& packet);
  // If there are streams pending to be reset, send a request to reset them.
  void MaybeSendResetStreamsRequest();
  // Performs internal processing shared between Send and SendMany.
  SendStatus InternalSend(const DcSctpMessage& message,
                          const SendOptions& send_options);
  // Sends a INIT chunk.
  void SendInit();
  // Sends a SHUTDOWN chunk.
  void SendShutdown();
  // Sends a SHUTDOWN-ACK chunk.
  void SendShutdownAck();
  // Validates the SCTP packet, as a whole - not the validity of individual
  // chunks within it, as that's done in the different chunk handlers.
  bool ValidatePacket(const SctpPacket& packet);
  // Parses `payload`, which is a serialized packet that is just going to be
  // sent and prints all chunks.
  void DebugPrintOutgoing(webrtc::ArrayView<const uint8_t> payload);
  // Called whenever data has been received, or the cumulative acknowledgment
  // TSN has moved, that may result in delivering messages.
  void MaybeDeliverMessages();
  // Returns true if there is a TCB, and false otherwise (and reports an error).
  bool ValidateHasTCB();

  // Returns true if the parsing of a chunk of type `T` succeeded. If it didn't,
  // it reports an error and returns false.
  template <class T>
  bool ValidateParseSuccess(const std::optional<T>& c) {
    if (c.has_value()) {
      return true;
    }

    ReportFailedToParseChunk(T::kType);
    return false;
  }

  // Reports failing to have parsed a chunk with the provided `chunk_type`.
  void ReportFailedToParseChunk(int chunk_type);
  // Called when unknown chunks are received. May report an error.
  bool HandleUnrecognizedChunk(const SctpPacket::ChunkDescriptor& descriptor);

  // Will dispatch more specific chunk handlers.
  bool Dispatch(const CommonHeader& header,
                const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming DATA chunks.
  void HandleData(const CommonHeader& header,
                  const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming I-DATA chunks.
  void HandleIData(const CommonHeader& header,
                   const SctpPacket::ChunkDescriptor& descriptor);
  // Common handler for DATA and I-DATA chunks.
  void HandleDataCommon(AnyDataChunk& chunk);
  // Handles incoming INIT chunks.
  void HandleInit(const CommonHeader& header,
                  const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming INIT-ACK chunks.
  void HandleInitAck(const CommonHeader& header,
                     const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming SACK chunks.
  void HandleSack(const CommonHeader& header,
                  const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming HEARTBEAT chunks.
  void HandleHeartbeatRequest(const CommonHeader& header,
                              const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming HEARTBEAT-ACK chunks.
  void HandleHeartbeatAck(const CommonHeader& header,
                          const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming ABORT chunks.
  void HandleAbort(const CommonHeader& header,
                   const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming ERROR chunks.
  void HandleError(const CommonHeader& header,
                   const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming COOKIE-ECHO chunks.
  void HandleCookieEcho(const CommonHeader& header,
                        const SctpPacket::ChunkDescriptor& descriptor);
  // Handles receiving COOKIE-ECHO when there already is a TCB. The return value
  // indicates if the processing should continue.
  bool HandleCookieEchoWithTCB(const CommonHeader& header,
                               const StateCookie& cookie);
  // Handles incoming COOKIE-ACK chunks.
  void HandleCookieAck(const CommonHeader& header,
                       const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming SHUTDOWN chunks.
  void HandleShutdown(const CommonHeader& header,
                      const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming SHUTDOWN-ACK chunks.
  void HandleShutdownAck(const CommonHeader& header,
                         const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming FORWARD-TSN chunks.
  void HandleForwardTsn(const CommonHeader& header,
                        const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming I-FORWARD-TSN chunks.
  void HandleIForwardTsn(const CommonHeader& header,
                         const SctpPacket::ChunkDescriptor& descriptor);
  // Handles incoming RE-CONFIG chunks.
  void HandleReconfig(const CommonHeader& header,
                      const SctpPacket::ChunkDescriptor& descriptor);
  // Common handled for FORWARD-TSN/I-FORWARD-TSN.
  void HandleForwardTsnCommon(const AnyForwardTsnChunk& chunk);
  // Handles incoming SHUTDOWN-COMPLETE chunks
  void HandleShutdownComplete(const CommonHeader& header,
                              const SctpPacket::ChunkDescriptor& descriptor);

  const std::string log_prefix_;
  const std::unique_ptr<PacketObserver> packet_observer_;
  Metrics metrics_;
  DcSctpOptions options_;

  // Enqueues callbacks and dispatches them just before returning to the caller.
  CallbackDeferrer callbacks_;

  TimerManager timer_manager_;
  const std::unique_ptr<Timer> t1_init_;
  const std::unique_ptr<Timer> t1_cookie_;
  const std::unique_ptr<Timer> t2_shutdown_;

  // Packets that failed to be sent, but should be retried.
  PacketSender packet_sender_;

  // The actual SendQueue implementation. As data can be sent on a socket before
  // the connection is established, this component is not in the TCB.
  RRSendQueue send_queue_;

  // Contains verification tag and initial TSN between having sent the INIT
  // until the connection is established (there is no TCB at this point).
  ConnectParameters connect_params_;
  // The socket state.
  State state_ = State::kClosed;
  // If the connection is established, contains a transmission control block.
  std::unique_ptr<TransmissionControlBlock> tcb_;
};
}  // namespace dcsctp

#endif  // NET_DCSCTP_SOCKET_DCSCTP_SOCKET_H_
