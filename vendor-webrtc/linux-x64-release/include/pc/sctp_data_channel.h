/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SCTP_DATA_CHANNEL_H_
#define PC_SCTP_DATA_CHANNEL_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <set>
#include <string>

#include "api/data_channel_interface.h"
#include "api/priority.h"
#include "api/rtc_error.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/pending_task_safety_flag.h"
#include "api/transport/data_channel_transport_interface.h"
#include "pc/data_channel_utils.h"
#include "pc/sctp_utils.h"
#include "rtc_base/containers/flat_set.h"
#include "rtc_base/copy_on_write_buffer.h"
#include "rtc_base/ssl_stream_adapter.h"  // For SSLRole
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread.h"
#include "rtc_base/thread_annotations.h"
#include "rtc_base/weak_ptr.h"

namespace webrtc {

class SctpDataChannel;

// Interface that acts as a bridge from the data channel to the transport.
// All methods in this interface need to be invoked on the network thread.
class SctpDataChannelControllerInterface {
 public:
  // Sends the data to the transport.
  virtual RTCError SendData(StreamId sid,
                            const SendDataParams& params,
                            const CopyOnWriteBuffer& payload) = 0;
  // Adds the data channel SID to the transport for SCTP.
  virtual void AddSctpDataStream(StreamId sid, PriorityValue priority) = 0;
  // Begins the closing procedure by sending an outgoing stream reset. Still
  // need to wait for callbacks to tell when this completes.
  virtual void RemoveSctpDataStream(StreamId sid) = 0;
  // Notifies the controller of state changes.
  virtual void OnChannelStateChanged(SctpDataChannel* data_channel,
                                     DataChannelInterface::DataState state) = 0;
  virtual size_t buffered_amount(StreamId sid) const = 0;
  virtual size_t buffered_amount_low_threshold(StreamId sid) const = 0;
  virtual void SetBufferedAmountLowThreshold(StreamId sid, size_t bytes) = 0;

 protected:
  virtual ~SctpDataChannelControllerInterface() {}
};

struct InternalDataChannelInit : public DataChannelInit {
  enum OpenHandshakeRole { kOpener, kAcker, kNone };
  // The default role is kOpener because the default `negotiated` is false.
  InternalDataChannelInit() : open_handshake_role(kOpener) {}
  explicit InternalDataChannelInit(const DataChannelInit& base);

  // Does basic validation to determine if a data channel instance can be
  // constructed using the configuration.
  bool IsValid() const;

  OpenHandshakeRole open_handshake_role;
  // Optional fallback or backup flag from PC that's used for non-prenegotiated
  // stream ids in situations where we cannot determine the SSL role from the
  // transport for purposes of generating a stream ID.
  // See: https://www.rfc-editor.org/rfc/rfc8832.html#name-protocol-overview
  std::optional<SSLRole> fallback_ssl_role;
};

// Helper class to allocate unique IDs for SCTP DataChannels.
class SctpSidAllocator {
 public:
  SctpSidAllocator() = default;
  // Gets the first unused odd/even id based on the DTLS role. If `role` is
  // SSL_CLIENT, the allocated id starts from 0 and takes even numbers;
  // otherwise, the id starts from 1 and takes odd numbers.
  // If a `StreamId` cannot be allocated, `std::nullopt` is returned.
  std::optional<StreamId> AllocateSid(SSLRole role);

  // Attempts to reserve a specific sid. Returns false if it's unavailable.
  bool ReserveSid(StreamId sid);

  // Indicates that `sid` isn't in use any more, and is thus available again.
  void ReleaseSid(StreamId sid);

 private:
  flat_set<StreamId> used_sids_ RTC_GUARDED_BY(&sequence_checker_);
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_{
      SequenceChecker::kDetached};
};

// SctpDataChannel is an implementation of the DataChannelInterface based on
// SctpTransport. It provides an implementation of unreliable or
// reliable data channels.

// DataChannel states:
// kConnecting: The channel has been created the transport might not yet be
//              ready.
// kOpen: The open handshake has been performed (if relevant) and the data
//        channel is able to send messages.
// kClosing: DataChannelInterface::Close has been called, or the remote side
//           initiated the closing procedure, but the closing procedure has not
//           yet finished.
// kClosed: The closing handshake is finished (possibly initiated from this,
//          side, possibly from the peer).
//
// How the closing procedure works for SCTP:
// 1. Alice calls Close(), state changes to kClosing.
// 2. Alice finishes sending any queued data.
// 3. Alice calls RemoveSctpDataStream, sends outgoing stream reset.
// 4. Bob receives incoming stream reset; OnClosingProcedureStartedRemotely
//    called.
// 5. Bob sends outgoing stream reset.
// 6. Alice receives incoming reset, Bob receives acknowledgement. Both receive
//    OnClosingProcedureComplete callback and transition to kClosed.
class SctpDataChannel : public DataChannelInterface {
 public:
  static scoped_refptr<SctpDataChannel> Create(
      WeakPtr<SctpDataChannelControllerInterface> controller,
      const std::string& label,
      bool connected_to_transport,
      const InternalDataChannelInit& config,
      Thread* signaling_thread,
      Thread* network_thread);

  // Instantiates an API proxy for a SctpDataChannel instance that will be
  // handed out to external callers.
  // The `signaling_safety` flag is used for the ObserverAdapter callback proxy
  // which delivers callbacks on the signaling thread but must not deliver such
  // callbacks after the peerconnection has been closed. The data controller
  // will update the flag when closed, which will cancel any pending event
  // notifications.
  static scoped_refptr<DataChannelInterface> CreateProxy(
      scoped_refptr<SctpDataChannel> channel,
      scoped_refptr<PendingTaskSafetyFlag> signaling_safety);

  void RegisterObserver(DataChannelObserver* observer) override;
  void UnregisterObserver() override;

  std::string label() const override;
  bool reliable() const override;
  bool ordered() const override;

  std::optional<int> maxPacketLifeTime() const override;
  std::optional<int> maxRetransmitsOpt() const override;
  std::string protocol() const override;
  bool negotiated() const override;
  int id() const override;
  PriorityValue priority() const override;

  uint64_t buffered_amount() const override;
  void Close() override;
  DataState state() const override;
  RTCError error() const override;
  uint32_t messages_sent() const override;
  uint64_t bytes_sent() const override;
  uint32_t messages_received() const override;
  uint64_t bytes_received() const override;
  bool Send(const DataBuffer& buffer) override;
  void SendAsync(DataBuffer buffer,
                 absl::AnyInvocable<void(RTCError) &&> on_complete) override;

  // Close immediately, ignoring any queued data or closing procedure.
  // This is called when the underlying SctpTransport is being destroyed.
  // It is also called by the PeerConnection if SCTP ID assignment fails.
  void CloseAbruptlyWithError(RTCError error);
  // Specializations of CloseAbruptlyWithError
  void CloseAbruptlyWithDataChannelFailure(const std::string& message);

  // Called when the SctpTransport's ready to use. That can happen when we've
  // finished negotiation, or if the channel was created after negotiation has
  // already finished.
  void OnTransportReady();

  void OnDataReceived(DataMessageType type, const CopyOnWriteBuffer& payload);

  // Sets the SCTP sid and adds to transport layer if not set yet. Should only
  // be called once.
  void SetSctpSid_n(StreamId sid);

  // The remote side started the closing procedure by resetting its outgoing
  // stream (our incoming stream). Sets state to kClosing.
  void OnClosingProcedureStartedRemotely();
  // The closing procedure is complete; both incoming and outgoing stream
  // resets are done and the channel can transition to kClosed. Called
  // asynchronously after RemoveSctpDataStream.
  void OnClosingProcedureComplete();
  // Called when the transport channel is created.
  void OnTransportChannelCreated();
  // Called when the transport channel is unusable.
  // This method makes sure the DataChannel is disconnected and changes state
  // to kClosed.
  void OnTransportChannelClosed(RTCError error);
  // Called when the amount of data buffered to be sent falls to or below the
  // threshold set when calling `SetBufferedAmountLowThreshold`.
  void OnBufferedAmountLow();

  DataChannelStats GetStats() const;

  // Returns a unique identifier that's guaranteed to always be available,
  // doesn't change throughout SctpDataChannel's lifetime and is used for
  // stats purposes (see also `GetStats()`).
  int internal_id() const { return internal_id_; }

  std::optional<StreamId> sid_n() const {
    RTC_DCHECK_RUN_ON(network_thread_);
    return id_n_;
  }

  // Reset the allocator for internal ID values for testing, so that
  // the internal IDs generated are predictable. Test only.
  static void ResetInternalIdAllocatorForTesting(int new_value);

 protected:
  SctpDataChannel(const InternalDataChannelInit& config,
                  WeakPtr<SctpDataChannelControllerInterface> controller,
                  const std::string& label,
                  bool connected_to_transport,
                  Thread* signaling_thread,
                  Thread* network_thread);
  ~SctpDataChannel() override;

 private:
  class ObserverAdapter;

  // The OPEN(_ACK) signaling state.
  enum HandshakeState {
    kHandshakeInit,
    kHandshakeShouldSendOpen,
    kHandshakeShouldSendAck,
    kHandshakeWaitingForAck,
    kHandshakeReady
  };

  RTCError SendImpl(DataBuffer buffer) RTC_RUN_ON(network_thread_);
  void UpdateState() RTC_RUN_ON(network_thread_);
  void SetState(DataState state) RTC_RUN_ON(network_thread_);

  void DeliverQueuedReceivedData() RTC_RUN_ON(network_thread_);

  RTCError SendDataMessage(const DataBuffer& buffer, bool queue_if_blocked)
      RTC_RUN_ON(network_thread_);

  bool SendControlMessage(const CopyOnWriteBuffer& buffer)
      RTC_RUN_ON(network_thread_);

  bool connected_to_transport() const RTC_RUN_ON(network_thread_) {
    return network_safety_->alive();
  }
  void MaybeSendOnBufferedAmountChanged() RTC_RUN_ON(network_thread_);

  Thread* const signaling_thread_;
  Thread* const network_thread_;
  std::optional<StreamId> id_n_ RTC_GUARDED_BY(network_thread_) = std::nullopt;
  const int internal_id_;
  const std::string label_;
  const std::string protocol_;
  const std::optional<int> max_retransmit_time_;
  const std::optional<int> max_retransmits_;
  const std::optional<PriorityValue> priority_;
  const bool negotiated_;
  const bool ordered_;
  // See the body of `MaybeSendOnBufferedAmountChanged`.
  size_t expected_buffer_amount_ = 0;

  DataChannelObserver* observer_ RTC_GUARDED_BY(network_thread_) = nullptr;
  std::unique_ptr<ObserverAdapter> observer_adapter_;
  DataState state_ RTC_GUARDED_BY(network_thread_) = kConnecting;
  RTCError error_ RTC_GUARDED_BY(network_thread_);
  uint32_t messages_sent_ RTC_GUARDED_BY(network_thread_) = 0;
  uint64_t bytes_sent_ RTC_GUARDED_BY(network_thread_) = 0;
  uint32_t messages_received_ RTC_GUARDED_BY(network_thread_) = 0;
  uint64_t bytes_received_ RTC_GUARDED_BY(network_thread_) = 0;
  WeakPtr<SctpDataChannelControllerInterface> controller_
      RTC_GUARDED_BY(network_thread_);
  HandshakeState handshake_state_ RTC_GUARDED_BY(network_thread_) =
      kHandshakeInit;
  // Did we already start the graceful SCTP closing procedure?
  bool started_closing_procedure_ RTC_GUARDED_BY(network_thread_) = false;
  PacketQueue queued_received_data_ RTC_GUARDED_BY(network_thread_);
  scoped_refptr<PendingTaskSafetyFlag> network_safety_ =
      PendingTaskSafetyFlag::CreateDetachedInactive();
};

}  // namespace webrtc

#endif  // PC_SCTP_DATA_CHANNEL_H_
